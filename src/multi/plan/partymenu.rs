use serde_derive::{Serialize, Deserialize};
use std::cmp::{Ordering, min};

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartyMenuPlanState {
  handle_menu_input_state: HandleMenuInputState,
  goal_dist: u8
}
impl PartialOrd for PartyMenuPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.goal_dist.partial_cmp(&self.goal_dist)
  }
}
impl PartialEq for PartyMenuPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress HandleMenuInput_ inputs, selecting a Pokemon from the party menu
pub struct PartyMenuPlan {
  // instance state
  handle_menu_input_state: HandleMenuInputState,

  // config state
  goal_index: u8,
  quit: bool,
}
impl PartyMenuPlan {
  pub fn quit() -> Self {
    Self {
      handle_menu_input_state: HandleMenuInputState::unknown(),
      goal_index: 0,
      quit: true,
    }
  }
  pub fn choose(goal_index: u8) -> Self {
    Self {
      handle_menu_input_state: HandleMenuInputState::unknown(),
      goal_index,
      quit: false,
    }
  }
  fn goal_dist(&self, current_quantity: u8) -> u8 {
    if self.quit { 0 } else { min(self.goal_dist_up(current_quantity), self.goal_dist_down(current_quantity)) }
  }
  fn goal_dist_up(&self, current_index: u8) -> u8 {
    let max_index = self.handle_menu_input_state.max_item;
    if current_index <= self.goal_index {
      self.goal_index - current_index
    } else {
      self.goal_index + 1 + max_index - current_index
    }
  }
  fn goal_dist_down(&self, current_index: u8) -> u8 {
    let max_index = self.handle_menu_input_state.max_item;
    if current_index >= self.goal_index {
      current_index - self.goal_index
    } else {
      current_index + 1 + max_index - self.goal_index
    }
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses + HandleMenuInputAddresses + InputIdentificationAddresses> Plan<R> for PartyMenuPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::PartyMenuState(PartyMenuPlanState { handle_menu_input_state: self.handle_menu_input_state.clone(), goal_dist: self.goal_dist(self.handle_menu_input_state.current_item) })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::PartyMenuState(PartyMenuPlanState { handle_menu_input_state, goal_dist: _ }) = state {
      self.handle_menu_input_state = handle_menu_input_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.handle_menu_input_state = HandleMenuInputState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { if self.quit { B } else { A } }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    match self.handle_menu_input_state.get_result(input) {
      HandleMenuInputResult::DoNothing => Some(Input::empty()),
      HandleMenuInputResult::ScrollTo { current_item } => {
        let new_dist = self.goal_dist(current_item);
        let prev_dist = self.goal_dist(self.handle_menu_input_state.current_item);
        if new_dist > prev_dist { None } // Never scroll away from goal item
        else if new_dist < prev_dist { Some(input & (U|D)) }
        else { Some(Input::empty()) }
      },
      HandleMenuInputResult::Exit { current_item, input: exit_input } => {
        if exit_input.intersects(B) {
          if !self.quit { None } else { Some(B) }
        } else {
          if self.quit || current_item != self.goal_index { None } else { Some(A) }
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
        let new_dist = self.goal_dist(current_item);
        let prev_dist = self.goal_dist(self.handle_menu_input_state.current_item);
        if new_dist > prev_dist { return None; } // Never scroll away from goal item
        gb.restore(s);
        gb.input(input);
        if new_dist < prev_dist { gb.step(); } else { gb.delay_step(); }
        
        let new_state = gb.save();
        self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
        Some((new_state, None))
      },
      HandleMenuInputResult::Exit { current_item, input: exit_input } => {
        if exit_input.intersects(B) {
          if !self.quit { return None; }
        } else {
          if self.quit || current_item != self.goal_index { return None; }
        }
        gb.restore(s);
        gb.input(input);
        gb.step();
        Some((gb.save(), Some(())))
      },
    }
  }
}
