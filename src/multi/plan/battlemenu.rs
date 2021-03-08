use serde_derive::{Serialize, Deserialize};

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;
use std::cmp::Ordering;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BattleMenuPlanState {
  handle_menu_input_state: HandleMenuInputState,
  correct_side: bool,
  correct_index: bool,
}
impl PartialOrd for BattleMenuPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.correct_side != other.correct_side {
      self.correct_side.partial_cmp(&other.correct_side)
    } else {
      self.correct_index.partial_cmp(&other.correct_index)
    }
  }
}
impl PartialEq for BattleMenuPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress HandleMenuInput_ inputs, selecting a chosen battle action
pub struct BattleMenuPlan {
  // instance state
  handle_menu_input_state: HandleMenuInputState,
  right_side: bool,

  // config state
  goal_index: u8,
}
impl BattleMenuPlan {
  pub fn fight() -> Self { Self::with_index(0) }
  pub fn items() -> Self { Self::with_index(1) }
  pub fn mon() -> Self { Self::with_index(2) }
  pub fn run() -> Self { Self::with_index(3) }
  fn with_index(index: u8) -> Self {
    Self {
      handle_menu_input_state: HandleMenuInputState::unknown(),
      right_side: false,

      goal_index: index,
    }
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses + HandleMenuInputAddresses + InputIdentificationAddresses> Plan<R> for BattleMenuPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::BattleMenuState(BattleMenuPlanState { handle_menu_input_state: self.handle_menu_input_state.clone(), correct_side: self.right_side == (self.goal_index >= 2), correct_index: self.handle_menu_input_state.current_item & 1 == self.goal_index & 1 })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::BattleMenuState(BattleMenuPlanState { handle_menu_input_state, correct_side, correct_index: _ }) = state {
      self.handle_menu_input_state = handle_menu_input_state.clone();
      self.right_side = *correct_side == (self.goal_index >= 2);
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.handle_menu_input_state = HandleMenuInputState::from_gb_state(gb, state);
    self.right_side = gb.gb.read_memory(R::BATTLE_START_SAVED_MENU_ITEM) >= 2;
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input {
    let mut blocked_inputs = Input::empty();
    if self.right_side && self.goal_index < 2 {
      blocked_inputs |= L;
    } else if !self.right_side && self.goal_index >= 2 {
      blocked_inputs |= R;
    } else {
      blocked_inputs |= A;
    }
    if self.handle_menu_input_state.current_item == 0 && (self.goal_index & 1) == 1 {
      blocked_inputs |= D;
    } else if self.handle_menu_input_state.current_item == 1 && (self.goal_index & 1) == 0 {
      blocked_inputs |= U;
    }
    blocked_inputs
  }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    match self.handle_menu_input_state.get_result(input) {
      HandleMenuInputResult::DoNothing => Some(Input::empty()),
      HandleMenuInputResult::ScrollTo { current_item } => {
        if self.handle_menu_input_state.current_item & 1 != self.goal_index & 1 && current_item & 1 == self.goal_index & 1 {
          Some(D)
        } else if self.handle_menu_input_state.current_item & 1 != self.goal_index & 1 || current_item & 1 == self.goal_index & 1 {
          Some(Input::empty())
        } else { None }
      },
      HandleMenuInputResult::Exit { current_item, input: exit_input } => {
        if self.right_side {
          if exit_input.intersects(L) {
            if self.goal_index < 2 { Some(L) } else { None }
          } else {
            if self.goal_index == current_item + 2 { Some(A) } else { None }
          }
        } else {
          if exit_input.intersects(R) {
            if self.goal_index >= 2 { Some(R) } else { None }
          } else {
            if self.goal_index == current_item { Some(A) } else { None }
          }
        }
      },
    }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
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
        // In Red/Blue, scrolling is effectively waiting since you can always reach the goal index within one frame
        gb.restore(s);
        gb.input(input);
        if self.handle_menu_input_state.current_item & 1 != self.goal_index & 1 && current_item & 1 == self.goal_index & 1 {
          gb.step();
        } else {
          gb.delay_step();
        }
        let new_state = gb.save();
        self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
        Some((new_state, None))
      },
      HandleMenuInputResult::Exit { current_item, input: exit_input } => {
        if self.right_side {
          if exit_input.intersects(L) {
            if self.goal_index < 2 {
              gb.restore(s);
              gb.input(input);
              gb.step();
              let new_state = gb.save();
              self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
              self.right_side = false;
              Some((new_state, None))
            } else { None }
          } else {
            if self.goal_index == current_item + 2 {
              gb.restore(s);
              gb.input(input);
              gb.step();
              Some((gb.save(), Some(())))
            } else { None }
          }
        } else {
          if exit_input.intersects(R) {
            if self.goal_index >= 2 {
              gb.restore(s);
              gb.input(input);
              gb.step();
              let new_state = gb.save();
              self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
              self.right_side = true;
              Some((new_state, None))
            } else { None }
          } else {
            if self.goal_index == current_item {
              gb.restore(s);
              gb.input(input);
              gb.step();
              Some((gb.save(), Some(())))
            } else { None }
          }
        }
      },
    }
  }
}
