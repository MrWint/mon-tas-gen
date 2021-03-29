use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BikeShopMenuPlanState {
  handle_menu_input_state: HandleMenuInputState,
}
impl PartialOrd for BikeShopMenuPlanState {
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    Some(Ordering::Equal)
  }
}
impl PartialEq for BikeShopMenuPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress HandleMenuInput_ inputs, selecting an option in the DisplayBikeShopMenu
pub struct BikeShopMenuPlan {
  // instance state
  handle_menu_input_state: HandleMenuInputState,
}
impl BikeShopMenuPlan {
  pub fn trigger_instant_text() -> Self {
    Self {
      handle_menu_input_state: HandleMenuInputState::unknown(),
    }
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses + HandleMenuInputAddresses + InputIdentificationAddresses> Plan<R> for BikeShopMenuPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::BikeShopMenuState(BikeShopMenuPlanState { handle_menu_input_state: self.handle_menu_input_state.clone() })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::BikeShopMenuState(BikeShopMenuPlanState { handle_menu_input_state }) = state {
      self.handle_menu_input_state = handle_menu_input_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.handle_menu_input_state = HandleMenuInputState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { B }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    match self.handle_menu_input_state.get_result(input) {
      HandleMenuInputResult::DoNothing => Some(Input::empty()),
      HandleMenuInputResult::ScrollTo { current_item: _ } => Some(Input::empty()),
      HandleMenuInputResult::Exit { current_item: _, input: exit_input } => {
        if !exit_input.intersects(B) { None } else { Some(B) }
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
      HandleMenuInputResult::ScrollTo { current_item: _ } => {
        gb.restore(s);
        gb.input(input);
        gb.delay_step();
        let new_state = gb.save();
        self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
        Some((new_state, None))
      },
      HandleMenuInputResult::Exit { current_item: _, input: exit_input } => {
        if !exit_input.intersects(B) {return None; }
        gb.restore(s);
        gb.input(input);
        gb.step();
        Some((gb.save(), Some(())))
      },
    }
  }
}
