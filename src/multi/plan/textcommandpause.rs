use serde_derive::{Serialize, Deserialize};

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TextCommandPausePlanState;

// Plan to progress TextCommandPause inputs
pub struct TextCommandPausePlan;
impl TextCommandPausePlan {
  pub fn new() -> Self { Self }
}
impl<R: MultiRom> Plan<R> for TextCommandPausePlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::TextCommandPauseState(TextCommandPausePlanState)
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::TextCommandPauseState(TextCommandPausePlanState) = state {
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, _gb: &mut Gb<R>, _state: &GbState) {}
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() }

  fn canonicalize_input(&self, input: Input) -> Option<Input> { if input.intersects(A | B) { Some(A) } else { None } }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    if !input.intersects(A | B) { return None; }
    gb.restore(s);
    gb.input(input);
    gb.step();
    Some((gb.save(), Some(())))
  }
}
