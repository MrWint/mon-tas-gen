use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

// Plan to progress HoldTextDisplayOpen inputs
pub struct HoldTextDisplayOpenPlan;
impl HoldTextDisplayOpenPlan {
  pub fn new() -> Self { Self }
}
impl<R: MultiRom> Plan<R> for HoldTextDisplayOpenPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::HoldTextDisplayOpenState
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::HoldTextDisplayOpenState = state {
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, _gb: &mut Gb<R>, _state: &GbState) {}
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() }

  fn canonicalize_input(&self, input: Input) -> Option<Input> { Some(input & A) }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    gb.restore(s);
    gb.input(input);
    if input.intersects(A) {
      gb.delay_step();
      Some((gb.save(), None))
    } else {
      gb.step();
      Some((gb.save(), Some(())))
    }
  }
}
