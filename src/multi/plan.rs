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

  fn save(&self) -> PlanState { PlanState::EmptyState }
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

#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub enum PlanState {
  EmptyState,
  ListState { cur_item: usize, sub_plan: Option<Rc<PlanState>> },
  SkipIntroState { inputs_until_auto_pass: u32, hjoy5_state: HJoy5State, },
  MainMenuState { handle_menu_input_state: HandleMenuInputState, },
  TextState { printed_characters: u32, ends_to_be_skipped: u32, },
  TextScrollWaitState { hjoy5_state: HJoy5State, },
  SkipTextsState { num_texts_remaining: u32, at_wait: bool, inner_plan: Rc<PlanState> },
}
impl PartialEq for PlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

impl PartialOrd for PlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self {
      PlanState::EmptyState => {
        if let PlanState::EmptyState = other {
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
      PlanState::SkipIntroState { inputs_until_auto_pass, hjoy5_state: _ } => {
        if let PlanState::SkipIntroState { inputs_until_auto_pass: other_inputs_until_auto_pass, hjoy5_state: _ } = other {
          other_inputs_until_auto_pass.partial_cmp(inputs_until_auto_pass)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::MainMenuState { handle_menu_input_state: _ } => {
        if let PlanState::MainMenuState { handle_menu_input_state: _ } = other {
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
    }
  }
}

mod identifyinput;
pub use identifyinput::*;
mod mainmenu;
pub use mainmenu::*;
mod skipintro;
pub use skipintro::*;
mod skiptexts;
pub use skiptexts::*;
mod text;
pub use text::*;
mod textscrollwait;
pub use textscrollwait::*;
