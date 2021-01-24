use serde_derive::{Serialize, Deserialize};

use crate::rom::*;
use super::*;

use std::cmp::Ordering;
use std::rc::Rc;

pub trait Plan<R: Rom> {
  /// Type of the resulting values of this plan.
  type Value;

  /// Saves current plan state
  fn save(&self) -> PlanState;
  /// Restores saved plan state
  fn restore(&mut self, state: &PlanState);
  /// Checks whether the current state is considered safe, i.e. there is guaranteed to be a sequence of inputs which completes it.
  fn is_safe(&self) -> bool;
  /// Returns the set of buttons which can be blocked from being pressed by the previous input.
  fn get_blockable_inputs(&self) -> Input;
  /// Analyzed the initial state and initialize the internal plan state to allow completion.
  /// Called before any canonicalize_input or execute_input calls are made.
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState);
  /// Normalizes the input to one that has the same effect, used to deduplicate inputs with the same outcome.
  fn canonicalize_input(&self, input: Input) -> Option<Input>;
  /// Executes the given input on the given state. If successful, returns a new state and updates the internal state, potentially finishing and returning a value.
  /// If the plan returns, the internal state is no longer guaranteed to be consistent.
  /// The execution step may skip multiple input uses, as long as all of them happen on the same input frame and use the same given input.
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)>;
}
pub struct NullPlan;
impl<R: MultiRom> Plan<R> for NullPlan {
  type Value = ();

  fn save(&self) -> PlanState { PlanState::NullState }
  fn restore(&mut self, _state: &PlanState) { }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() }
  fn initialize(&mut self, _gb: &mut Gb<R>, _state: &GbState) { }
  fn canonicalize_input(&self, _input: Input) -> Option<Input> { Some(Input::empty()) }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    gb.restore(state);
    gb.input(input);
    gb.step();
    Some((gb.save(), None))
  }
}

pub struct ListPlan<R: Rom> {
  plans: Vec<Box<dyn Plan<R, Value=()>>>,
  cur_item: usize,
  cur_item_is_initialized: bool,
}
impl<R: Rom> ListPlan<R> {
  pub fn new(plans: Vec<Box<dyn Plan<R, Value=()>>>) -> Self {
    assert!(!plans.is_empty());
    Self {
      plans,
      cur_item: 0,
      cur_item_is_initialized: false,
    }
  }
  fn initialize_cur_item(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.plans[self.cur_item].initialize(gb, state);
    self.cur_item_is_initialized = true;
  }
  pub fn ensure_cur_item_initialized(&mut self, gb: &mut Gb<R>, state: &GbState) {
    if !self.cur_item_is_initialized {
      self.plans[self.cur_item].initialize(gb, state);
      self.cur_item_is_initialized = true;
    }
  }
}
impl<R: Rom> Plan<R> for ListPlan<R> {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::ListState {
      cur_item: self.cur_item,
      sub_plan: if self.cur_item >= self.plans.len() { None } else { Some(Rc::new(self.plans[self.cur_item].save())) },
    }
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::ListState { cur_item, sub_plan } = state {
      assert!(*cur_item < self.plans.len());
      self.cur_item = *cur_item;
      if let Some(sub_plan) = sub_plan {
        self.plans[*cur_item].restore(sub_plan);
        self.cur_item_is_initialized = true;
      } else {
        self.cur_item_is_initialized = false;
      }
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.cur_item = 0;
    self.initialize_cur_item(gb, state);
  }
  fn is_safe(&self) -> bool {
    self.cur_item >= self.plans.len() || self.plans[self.cur_item].is_safe()
  }
  fn get_blockable_inputs(&self) -> Input {
    if self.cur_item < self.plans.len() { self.plans[self.cur_item].get_blockable_inputs() } else { Input::all() }
  }
  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    assert!(self.cur_item_is_initialized);
    self.plans[self.cur_item].canonicalize_input(input)
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    assert!(self.cur_item < self.plans.len());
    self.ensure_cur_item_initialized(gb, state);
    if let Some((new_state, result)) = self.plans[self.cur_item].execute_input(gb, state, input) {
      if result.is_some() {
        self.cur_item += 1;
        if self.cur_item >= self.plans.len() {
          self.cur_item_is_initialized = false;
          return Some((new_state, Some(())));
        }
        self.initialize_cur_item(gb, &new_state);
      }
      Some((new_state, None))
    } else { None }
  }
}

pub struct SeqPlan<P, Q> {
  p: P,
  q: Q,
  p_done: bool,
}
impl<P, Q> SeqPlan<P, Q> {
  pub fn new(p: P, q: Q) -> Self {
    Self { p, q, p_done: false, }
  }
}
impl<R: Rom,P: Plan<R>, Q: Plan<R>> Plan<R> for SeqPlan<P, Q> {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::SeqState {
      p_done: self.p_done,
      sub_plan: Rc::new(if self.p_done { self.q.save() } else { self.p.save() }),
    }
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::SeqState { p_done, sub_plan } = state {
      self.p_done = *p_done;
      if self.p_done { self.q.restore(sub_plan) } else { self.p.restore(sub_plan) };
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.p_done = false;
    self.p.initialize(gb, state);
  }
  fn is_safe(&self) -> bool {
    if self.p_done { self.q.is_safe() } else { self.p.is_safe() }
  }
  fn get_blockable_inputs(&self) -> Input {
    if self.p_done { self.q.get_blockable_inputs() } else { self.p.get_blockable_inputs() }
  }
  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    if self.p_done { self.q.canonicalize_input(input) } else { self.p.canonicalize_input(input) }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    if self.p_done {
      if let Some((new_state, result)) = self.q.execute_input(gb, state, input) {
        if result.is_some() {
          return Some((new_state, Some(())));
        }
        Some((new_state, None))
      } else { None }
    } else {
      if let Some((new_state, result)) = self.p.execute_input(gb, state, input) {
        if result.is_some() {
          self.p_done = true;
          self.q.initialize(gb, &new_state);
        }
        Some((new_state, None))
      } else { None }
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlanState {
  NullState,
  ChangeOptionsState { progress: ChangeOptionsProgress, hjoy5_state: HJoy5State, },
  EdgeWarpState,
  IdentifyInputState,
  HoldTextDisplayOpenState,
  ListState { cur_item: usize, sub_plan: Option<Rc<PlanState>> },
  IntroNameMenuState { handle_menu_input_state: HandleMenuInputState, },
  MainMenuState { handle_menu_input_state: HandleMenuInputState, },
  NamingScreenState { letter_selected: bool, delta: (i8, i8), pressed_input_state: PressedInputState, },
  OverworldInteractState { joypad_overworld_state: JoypadOverworldState, },
  OverworldOpenStartMenuState { joypad_overworld_state: JoypadOverworldState, },
  OverworldTurnState { joypad_overworld_state: JoypadOverworldState, },
  OverworldWaitState,
  SeqState { p_done: bool, sub_plan: Rc<PlanState> },
  SkipIntroState { inputs_until_auto_pass: u32, hjoy5_state: HJoy5State, },
  StartMenuState { handle_menu_input_state: HandleMenuInputState, distance_to_goal: u8, },
  StartMenuCloseState { pressed_input_state: PressedInputState, },
  TextState { printed_characters: u32, ends_to_be_skipped: u32, },
  TextScrollWaitState { hjoy5_state: HJoy5State, },
  SkipTextsState { num_texts_remaining: u32, at_wait: bool, inner_plan: Rc<PlanState> },
  TwoOptionMenuState { handle_menu_input_state: HandleMenuInputState, },
  WalkToState { pos: (usize, usize), turnframe_direction: Option<u8>, map: Rc<MapState>, joypad_overworld_state: JoypadOverworldState, dist_to_goal: i32, requires_turn: bool, },
}
impl PartialEq for PlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

impl PartialOrd for PlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self {
      PlanState::NullState => {
        if let PlanState::NullState = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::ChangeOptionsState { progress, hjoy5_state: _ } => {
        if let PlanState::ChangeOptionsState { progress: other_progress, hjoy5_state: _ } = other {
          progress.partial_cmp(other_progress)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::EdgeWarpState => {
        if let PlanState::EdgeWarpState = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::HoldTextDisplayOpenState => {
        if let PlanState::HoldTextDisplayOpenState = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::IdentifyInputState => {
        if let PlanState::IdentifyInputState = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::ListState { cur_item, sub_plan } => {
        if let PlanState::ListState { cur_item: other_item, sub_plan: other_plan } = other {
          if cur_item != other_item {
            cur_item.partial_cmp(other_item)
          } else {
            sub_plan.partial_cmp(other_plan)
          }
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::IntroNameMenuState { handle_menu_input_state: _ } => {
        if let PlanState::IntroNameMenuState { handle_menu_input_state: _ } = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::MainMenuState { handle_menu_input_state: _ } => {
        if let PlanState::MainMenuState { handle_menu_input_state: _ } = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::NamingScreenState { letter_selected, delta: (dx, dy), pressed_input_state: _ } => {
        if let PlanState::NamingScreenState { letter_selected: other_letter_selected, delta: (odx, ody), pressed_input_state: _ } = other {
          if letter_selected != other_letter_selected {
            letter_selected.partial_cmp(other_letter_selected)
          } else {
            (odx.abs() + ody.abs()).partial_cmp(&(dx.abs() + dy.abs()))
          }
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::OverworldInteractState { joypad_overworld_state: _ } => {
        if let PlanState::OverworldInteractState { joypad_overworld_state: _ } = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::OverworldOpenStartMenuState { joypad_overworld_state: _ } => {
        if let PlanState::OverworldOpenStartMenuState { joypad_overworld_state: _ } = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::OverworldTurnState { joypad_overworld_state: _ } => {
        if let PlanState::OverworldTurnState { joypad_overworld_state: _ } = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::OverworldWaitState => {
        if let PlanState::OverworldWaitState = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::SeqState { p_done, sub_plan } => {
        if let PlanState::SeqState { p_done: other_p_done, sub_plan: other_plan } = other {
          if p_done != other_p_done {
            p_done.partial_cmp(other_p_done)
          } else {
            sub_plan.partial_cmp(other_plan)
          }
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::SkipIntroState { inputs_until_auto_pass, hjoy5_state: _ } => {
        if let PlanState::SkipIntroState { inputs_until_auto_pass: other_inputs_until_auto_pass, hjoy5_state: _ } = other {
          other_inputs_until_auto_pass.partial_cmp(inputs_until_auto_pass)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::StartMenuState { handle_menu_input_state: _, distance_to_goal } => {
        if let PlanState::StartMenuState { handle_menu_input_state: _, distance_to_goal: other_distance_to_goal } = other {
          other_distance_to_goal.partial_cmp(distance_to_goal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::StartMenuCloseState { pressed_input_state: _ } => {
        if let PlanState::StartMenuCloseState { pressed_input_state: _ } = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::TextState { printed_characters, ends_to_be_skipped } => {
        if let PlanState::TextState { printed_characters: other_printed_characters, ends_to_be_skipped: other_ends_to_be_skipped } = other {
          if ends_to_be_skipped != other_ends_to_be_skipped {
            other_ends_to_be_skipped.partial_cmp(ends_to_be_skipped)
          } else {
            printed_characters.partial_cmp(other_printed_characters)
          }
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::TextScrollWaitState { hjoy5_state: _ } => {
        if let PlanState::TextScrollWaitState { hjoy5_state: _ } = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::SkipTextsState { num_texts_remaining, at_wait, inner_plan } => {
        if let PlanState::SkipTextsState { num_texts_remaining: other_num_texts_remaining, at_wait: other_at_wait, inner_plan: other_inner_plan } = other {
          if num_texts_remaining != other_num_texts_remaining {
            other_num_texts_remaining.partial_cmp(num_texts_remaining)
          } else if at_wait != other_at_wait {
            at_wait.partial_cmp(other_at_wait)
          } else {
            inner_plan.partial_cmp(other_inner_plan)
          }
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::TwoOptionMenuState { handle_menu_input_state: _, } => {
        if let PlanState::TwoOptionMenuState { handle_menu_input_state: _, } = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::WalkToState { pos: _, turnframe_direction: _, map: _, joypad_overworld_state: _, dist_to_goal, requires_turn, } => {
        if let PlanState::WalkToState { pos: _, turnframe_direction: _, map: _, joypad_overworld_state: _, dist_to_goal: other_dist_to_goal, requires_turn: other_requires_turn, } = other {
          if dist_to_goal != other_dist_to_goal {
            other_dist_to_goal.partial_cmp(dist_to_goal)
          } else {
            other_requires_turn.partial_cmp(requires_turn)
          }
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
    }
  }
}

mod changeoptions;
pub use changeoptions::*;
mod edgewarp;
pub use edgewarp::*;
mod holdtextdisplayopen;
pub use holdtextdisplayopen::*;
mod identifyinput;
pub use identifyinput::*;
mod intronamemenu;
pub use intronamemenu::*;
mod joypad;
pub use joypad::*;
mod mainmenu;
pub use mainmenu::*;
mod namingscreen;
pub use namingscreen::*;
mod overworldinteract;
pub use overworldinteract::*;
mod overworldopenstartmenu;
pub use overworldopenstartmenu::*;
mod overworldturn;
pub use overworldturn::*;
mod overworldwait;
pub use overworldwait::*;
mod skipintro;
pub use skipintro::*;
mod skiptexts;
pub use skiptexts::*;
mod startmenu;
pub use startmenu::*;
mod text;
pub use text::*;
mod textscrollwait;
pub use textscrollwait::*;
mod twooptionmenu;
pub use twooptionmenu::*;
mod walkto;
pub use walkto::*;
