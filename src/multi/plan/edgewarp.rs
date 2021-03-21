use serde_derive::{Serialize, Deserialize};

use crate::metric::*;
use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;


#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EdgeWarpPlanState;

// Plan to progress CheckWarpsNoCollisionLoop inputs
pub struct EdgeWarpPlan<M> {
  metric: M,
}
impl EdgeWarpPlan<NullMetric> {
  pub fn new() -> Self { Self::with_metric(NullMetric) }
}
impl<M> EdgeWarpPlan<M> {
  pub fn with_metric(metric: M) -> Self { Self { metric } }
}
impl<R: MultiRom, M: Metric<R>> Plan<R> for EdgeWarpPlan<M> {
  type Value = M::ValueType;

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
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)> {
    if input.intersects(U | D | L | R) {
      gb.restore(s);
      gb.input(input);
      if let Some(metric_value) = self.metric.evaluate(gb) {
        if !gb.is_at_input() { gb.step(); }
        Some((gb.save(), Some(metric_value)))
      } else { None }
    } else { None }
  }
}
