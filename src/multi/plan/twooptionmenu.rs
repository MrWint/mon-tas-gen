use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TwoOptionMenuPlanState {
  handle_menu_input_state: HandleMenuInputState,
}
impl PartialOrd for TwoOptionMenuPlanState {
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    Some(Ordering::Equal)
  }
}
impl PartialEq for TwoOptionMenuPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress HandleMenuInput_ inputs, selecting an option in a two-option menu (DisplayTwoOptionMenu)
pub struct TwoOptionMenuPlan {
  // instance state
  handle_menu_input_state: HandleMenuInputState,

  // config state
  choose_primary: bool,
}
impl TwoOptionMenuPlan {
  pub fn yes() -> Self { Self::with_choose_primary(true) }
  pub fn no() -> Self { Self::with_choose_primary(false) }
  fn with_choose_primary(choose_primary: bool) -> Self {
    Self {
      handle_menu_input_state: HandleMenuInputState::unknown(),
      choose_primary,
    }
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses + HandleMenuInputAddresses + InputIdentificationAddresses> Plan<R> for TwoOptionMenuPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::TwoOptionMenuState(TwoOptionMenuPlanState { handle_menu_input_state: self.handle_menu_input_state.clone(), })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::TwoOptionMenuState(TwoOptionMenuPlanState { handle_menu_input_state, }) = state {
      self.handle_menu_input_state = handle_menu_input_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.handle_menu_input_state = HandleMenuInputState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { if self.choose_primary { A } else { B } }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    match self.handle_menu_input_state.get_result(input) {
      HandleMenuInputResult::DoNothing => Some(Input::empty()),
      HandleMenuInputResult::ScrollTo { current_item } => {
        if self.choose_primary && current_item > 0 { return None; } // Don't scroll away from primary item
        Some(Input::DOWN)
      },
      HandleMenuInputResult::Exit { current_item, input: exit_input } => {
        if exit_input.intersects(B) {
          if self.choose_primary { None } else { Some(B) }
        } else {
          if (current_item == 0) == self.choose_primary { Some(A) } else { None }
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
        if self.choose_primary && current_item > 0 { return None; } // Don't scroll away from primary item
        gb.restore(s);
        gb.input(input);
        gb.delay_step();
        let new_state = gb.save();
        self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
        Some((new_state, None))
      },
      HandleMenuInputResult::Exit { current_item, input: exit_input } => {
        if exit_input.intersects(B) {
          if self.choose_primary { return None; }
        } else {
          if (current_item == 0) != self.choose_primary { return None; }
        }
        gb.restore(s);
        gb.input(input);
        gb.step();
        Some((gb.save(), Some(())))
    },
    }
  }
}
