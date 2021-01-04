use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

// Plan to progress WaitForTextScrollButtonPress inputs
pub struct TextScrollWaitPlan {
  // instance state
  hjoy5_state: HJoy5State,

  // config state
}
impl TextScrollWaitPlan {
  pub fn new() -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      hjoy5_state: HJoy5State::unknown(),

      // Default config state.
    }
  }
}
impl<R: MultiRom> Plan<R> for TextScrollWaitPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::TextScrollWaitState { hjoy5_state: self.hjoy5_state.clone() }
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::TextScrollWaitState { hjoy5_state, } = state {
      self.hjoy5_state = hjoy5_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.hjoy5_state = HJoy5State::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { A | B }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    Some(self.hjoy5_state.get_hjoy5(input) & (A | B))
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    gb.restore(s);
    if self.hjoy5_state.get_hjoy5(input).intersects(A | B) {
      gb.input(input);
      gb.step();
      Some((gb.save(), Some(())))
    } else {
      while gb.get_input_frame_lo() == s.get_input_frame_lo() && gb.get_input_frame_hi() == s.get_input_frame_hi() {
        gb.input(input);
        gb.delay_step();
      }
      let new_state = gb.save();
      self.hjoy5_state = HJoy5State::from_gb(gb);
      Some((new_state, None))
    }
  }
}
