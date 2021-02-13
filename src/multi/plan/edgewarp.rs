use serde_derive::{Serialize, Deserialize};

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;


#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EdgeWarpPlanState;

// Plan to progress CheckWarpsNoCollisionLoop inputs
pub struct EdgeWarpPlan;
impl EdgeWarpPlan {
  pub fn new() -> Self { Self }
}
impl<R: MultiRom> Plan<R> for EdgeWarpPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::EdgeWarpState(EdgeWarpPlanState)
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::EdgeWarpState(EdgeWarpPlanState) = state {
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, _gb: &mut Gb<R>, _state: &GbState) {}
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    if input.intersects(U | D | L | R) { Some(U) } else { None }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    if input.intersects(U | D | L | R) {
      gb.restore(s);
      gb.input(input);
      gb.step();
      Some((gb.save(), Some(())))
    } else { None }
  }
}
