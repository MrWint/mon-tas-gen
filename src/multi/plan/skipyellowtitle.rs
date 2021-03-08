use serde_derive::{Serialize, Deserialize};

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SkipYellowTitlePlanState;

// Plan to progress DisplayTitleScreen inputs
pub struct SkipYellowTitlePlan;
impl SkipYellowTitlePlan {
  pub fn new() -> Self { Self }
}
impl<R: Rom + JoypadLowSensitivityAddresses> Plan<R> for SkipYellowTitlePlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::SkipYellowTitleState(SkipYellowTitlePlanState)
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::SkipYellowTitleState(SkipYellowTitlePlanState) = state {
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, _gb: &mut Gb<R>, _state: &GbState) {}
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    if input.contains(U | SELECT | B) {
      // U|SELECT|B is not allowed, pressing it would enter the clear save dialog.
      return None;
    }
    if input.intersects(A | START) {
      return Some(A | START);
    }
    Some(Input::empty())
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    if input.contains(U | SELECT | B) {
      // U|SELECT|B is not allowed, pressing it would enter the clear save dialog.
      return None;
    }
    gb.restore(s);
    gb.input(input);
    if input.intersects(A | START) {
      gb.step();
      Some((gb.save(), Some(())))
    } else {
      // Stay in title screen, update instance state.
      gb.delay_step();
      Some((gb.save(), None))
    }
  }
}
