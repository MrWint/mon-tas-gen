use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntroNameMenuPlanState {
  handle_menu_input_state: HandleMenuInputState,
}
impl PartialOrd for IntroNameMenuPlanState {
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    Some(Ordering::Equal)
  }
}
impl PartialEq for IntroNameMenuPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress HandleMenuInput_ inputs
pub struct IntroNameMenuPlan {
  // instance state
  handle_menu_input_state: HandleMenuInputState,

  // config state
}
impl IntroNameMenuPlan {
  pub fn choose_custom_name() -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      handle_menu_input_state: HandleMenuInputState::unknown(),

      // Default config state.
    }
  }
}
impl<R: MultiRom + HandleMenuInputAddresses> Plan<R> for IntroNameMenuPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::IntroNameMenuState(IntroNameMenuPlanState { handle_menu_input_state: self.handle_menu_input_state.clone() })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::IntroNameMenuState(IntroNameMenuPlanState { handle_menu_input_state, }) = state {
      self.handle_menu_input_state = handle_menu_input_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.handle_menu_input_state = HandleMenuInputState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { A }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    return match self.handle_menu_input_state.get_result(input) {
      HandleMenuInputResult::DoNothing => Some(Input::empty()),
      HandleMenuInputResult::ScrollTo { current_item } => if current_item != 0 { None } else { Some(Input::empty()) },
      HandleMenuInputResult::Exit { current_item, input } => if current_item != 0 || input.contains(B) { None } else { Some(input & (A | START)) },
    };
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    return match self.handle_menu_input_state.get_result(input) {
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
      HandleMenuInputResult::ScrollTo { current_item } => if current_item != 0 { None } else {
        gb.restore(s);
        gb.input(input);
        gb.delay_step();
        let new_state = gb.save();
        self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
        Some((new_state, None))
      },
      HandleMenuInputResult::Exit { current_item, input } => if current_item != 0 { None } else {
        gb.restore(s);
        gb.input(input);
        gb.step();
        Some((gb.save(), Some(())))
      },
    };
  }
}
