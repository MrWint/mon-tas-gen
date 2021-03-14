use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

pub struct OverrideMovePlan;

impl OverrideMovePlan {
  pub fn choose<R: MultiRom + TextAddresses + HandleMenuInputAddresses>(index: u8) -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(1))); // mon // trying to learn
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(1))); // move // .
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // but // mon // can't learn more
    plans.push(Box::new(SkipTextsPlan::new(1))); // than 4 moves
    plans.push(Box::new(SkipTextsPlan::new(1))); // delete move to make room
    plans.push(Box::new(TextPlan::new().with_skip_ends(2))); // for // move // ?
    plans.push(Box::new(TwoOptionMenuPlan::yes()));
    plans.push(Box::new(TextPlan::new())); // which move should be forgotten?
    plans.push(Box::new(OverrideMoveMenuPlan::choose(index))); // Override Horn Attack
    plans.push(Box::new(SeqPlan::new(TextPlan::new(), TextCommandPausePlan::new()))); // 1, 2, and
    plans.push(Box::new(SeqPlan::new(TextPlan::new(), SeqPlan::new(TextCommandPausePlan::new(), TextScrollWaitPlan::new())))); // poof
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(3))); // mon // forgot // move // .
    plans.push(Box::new(SkipTextsPlan::new(1))); // and
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(3))); // mon // learned // move // !
    ListPlan::new(plans)
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverrideMoveMenuPlanState {
  handle_menu_input_state: HandleMenuInputState,
  goal_dist: u8
}
impl PartialOrd for OverrideMoveMenuPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.goal_dist.partial_cmp(&self.goal_dist)
  }
}
impl PartialEq for OverrideMoveMenuPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress HandleMenuInput_ inputs, selecting an move to override
pub struct OverrideMoveMenuPlan {
  // instance state
  handle_menu_input_state: HandleMenuInputState,

  // config state
  choose_option: u8,
}
impl OverrideMoveMenuPlan {
  pub fn choose(choose_option: u8) -> Self {
    Self {
      handle_menu_input_state: HandleMenuInputState::unknown(),
      choose_option,
    }
  }
  pub fn quit() -> Self { Self::choose(4) }
  fn goal_dist(&self, current_item: u8) -> u8 {
    if self.choose_option == 4 { 0 } else if current_item > self.choose_option { current_item - self.choose_option } else { self.choose_option - current_item }
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses + HandleMenuInputAddresses + InputIdentificationAddresses> Plan<R> for OverrideMoveMenuPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::OverrideMoveMenuState(OverrideMoveMenuPlanState { handle_menu_input_state: self.handle_menu_input_state.clone(), goal_dist: self.goal_dist(self.handle_menu_input_state.current_item) })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::OverrideMoveMenuState(OverrideMoveMenuPlanState { handle_menu_input_state, goal_dist: _ }) = state {
      self.handle_menu_input_state = handle_menu_input_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.handle_menu_input_state = HandleMenuInputState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { if self.choose_option < 4 { A } else { B } }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    match self.handle_menu_input_state.get_result(input) {
      HandleMenuInputResult::DoNothing => Some(Input::empty()),
      HandleMenuInputResult::ScrollTo { current_item } => {
        if self.goal_dist(current_item) > self.goal_dist(self.handle_menu_input_state.current_item) { return None; } // Never scroll away from goal item
        Some(input & (U|D))
      },
      HandleMenuInputResult::Exit { current_item, input: exit_input } => {
        if !exit_input.intersects(B) {
          if current_item == self.choose_option { Some(A) } else { None }
        } else {
          if self.choose_option == 4 { Some(B) } else { None }
        }
      },
    }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)> {
    match self.handle_menu_input_state.get_result(input) {
      HandleMenuInputResult::DoNothing => {
        gb.restore(s);
        while gb.get_input_frame_lo() == s.get_input_frame_lo() && gb.get_input_frame_hi() == s.get_input_frame_hi() {
          gb.input(input);
          gb.delay_step();
        }
        let new_state = gb.save();
        self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
        Some((new_state, None))
      },
      HandleMenuInputResult::ScrollTo { current_item } => {
        if self.goal_dist(current_item) > self.goal_dist(self.handle_menu_input_state.current_item) { return None; } // Never scroll away from goal item
        gb.restore(s);
        gb.input(input);
        if self.goal_dist(current_item) < self.goal_dist(self.handle_menu_input_state.current_item) { gb.step(); } else { gb.delay_step(); }
        
        let new_state = gb.save();
        self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
        Some((new_state, None))
      },
      HandleMenuInputResult::Exit { current_item, input: exit_input } => {
        if !exit_input.intersects(B) {
          if current_item != self.choose_option { return None; }
        } else {
          if self.choose_option != 4 { return None; }
        }
        gb.restore(s);
        gb.input(input);
        gb.step();
        Some((gb.save(), Some(())))
      },
    }
  }
}
