use serde_derive::{Serialize, Deserialize};

use crate::metric::*;
use crate::metric::overworld::gen1::*;
use crate::multi::*;
use crate::rom::*;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct OverworldWaitPlanState;

// Plan to progress JoypadOverworld inputs, waiting one input (with result NoAction)
pub struct OverworldWaitPlan<M> {
  // config state
  metric: M,
}
impl<M> OverworldWaitPlan<M> {
  pub fn with_metric(metric: M) -> Self {
    Self {
      metric,
    }
  }
}
impl<R: JoypadAddresses + Gen1OverworldAddresses + Gen1DVAddresses> OverworldWaitPlan<Expect<R, OverworldInteractionMetric>> {
  pub fn new() -> Self {
    Self::with_metric(OverworldInteractionMetric.expect(OverworldInteractionResult::NoAction))
  }
  pub fn auto_walk(direction: Input) -> Self {
    Self::with_metric(OverworldInteractionMetric.expect(OverworldInteractionResult::Walked { direction }))
  }
  pub fn trainer_battle(id: u8) -> Self {
    Self::with_metric(OverworldInteractionMetric.expect(OverworldInteractionResult::TrainerBattle { species: id }))
  }
}
impl<R: MultiRom, M: Metric<R>> Plan<R> for OverworldWaitPlan<M> {
  type Value = M::ValueType;

  fn save(&self) -> PlanState {
    PlanState::OverworldWaitState(OverworldWaitPlanState)
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::OverworldWaitState(OverworldWaitPlanState) = state {
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, _gb: &mut Gb<R>, _state: &GbState) {}
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() }

  fn canonicalize_input(&self, _input: Input) -> Option<Input> { Some(Input::empty()) }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<M::ValueType>)> {
    gb.restore(s);
    gb.input(input);
    if let Some(metric_value) = self.metric.evaluate(gb) {
      if !gb.is_at_input() { gb.step(); }
      Some((gb.save(), Some(metric_value)))
    } else { None }
  }
}
