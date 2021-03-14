use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemUseTossMenuPlanState {
  handle_menu_input_state: HandleMenuInputState,
  goal_dist: u8
}
impl PartialOrd for ItemUseTossMenuPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.goal_dist.partial_cmp(&self.goal_dist)
  }
}
impl PartialEq for ItemUseTossMenuPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress HandleMenuInput_ inputs, selecting an option in the DisplayItemUseTossMenu
pub struct ItemUseTossMenuPlan {
  // instance state
  handle_menu_input_state: HandleMenuInputState,

  // config state
  choose_option: u8,
}
impl ItemUseTossMenuPlan {
  pub fn use_() -> Self { Self::with_choose_option(0) }
  pub fn toss() -> Self { Self::with_choose_option(1) }
  pub fn quit() -> Self { Self::with_choose_option(2) }
  fn with_choose_option(choose_option: u8) -> Self {
    Self {
      handle_menu_input_state: HandleMenuInputState::unknown(),
      choose_option,
    }
  }
  fn goal_dist(&self, current_item: u8) -> u8 {
    if self.choose_option == 2 { 0 } else if current_item > self.choose_option { current_item - self.choose_option } else { self.choose_option - current_item }
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses + HandleMenuInputAddresses + InputIdentificationAddresses> Plan<R> for ItemUseTossMenuPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::ItemUseTossMenuState(ItemUseTossMenuPlanState { handle_menu_input_state: self.handle_menu_input_state.clone(), goal_dist: self.goal_dist(self.handle_menu_input_state.current_item) })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::ItemUseTossMenuState(ItemUseTossMenuPlanState { handle_menu_input_state, goal_dist: _ }) = state {
      self.handle_menu_input_state = handle_menu_input_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.handle_menu_input_state = HandleMenuInputState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { if self.choose_option < 2 { A } else { B } }

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
          if self.choose_option == 2 { Some(B) } else { None }
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
          if self.choose_option != 2 { return None; }
        }
        gb.restore(s);
        gb.input(input);
        gb.step();
        Some((gb.save(), Some(())))
      },
    }
  }
}
