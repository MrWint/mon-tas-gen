use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

// Plan to progress HandleMenuInput_ inputs
pub struct MainMenuPlan {
  // instance state
  handle_menu_input_state: HandleMenuInputState,

  // config state
}
impl MainMenuPlan {
  pub fn new() -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      handle_menu_input_state: HandleMenuInputState::unknown(),

      // Default config state.
    }
  }
}
impl<R: MultiRom + HandleMenuInputAddresses> Plan<R> for MainMenuPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::MainMenuState { handle_menu_input_state: self.handle_menu_input_state.clone() }
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::MainMenuState { handle_menu_input_state, } = state {
      self.handle_menu_input_state = handle_menu_input_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.handle_menu_input_state = HandleMenuInputState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { A | START } // B is also blockable but that would only be beneficial

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
        let input_frame_lo = gb.get_input_frame_lo();
        let input_frame_hi = gb.get_input_frame_hi();
        while gb.get_input_frame_lo() == input_frame_lo && gb.get_input_frame_hi() == input_frame_hi {
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
      HandleMenuInputResult::Exit { current_item, input } => if current_item != 0 || input.contains(B) { None } else {
        gb.restore(s);
        gb.input(input);
        gb.step();
        Some((gb.save(), Some(())))
      },
    };
  }
}
