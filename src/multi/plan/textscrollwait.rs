use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::metric::*;
use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextScrollWaitPlanState  {
  hjoy5_state: HJoy5State,
}
impl PartialOrd for TextScrollWaitPlanState {
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    Some(Ordering::Equal)
  }
}
impl PartialEq for TextScrollWaitPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress WaitForTextScrollButtonPress inputs or ShowPokedexDataInternal inputs
pub struct TextScrollWaitPlan<M> {
  // instance state
  hjoy5_state: HJoy5State,

  // config state
  metric: M,
}
impl TextScrollWaitPlan<NullMetric> {
  pub fn new() -> Self { Self::with_metric(NullMetric) }
}
impl<M> TextScrollWaitPlan<M> {
  pub fn with_metric(metric: M) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      hjoy5_state: HJoy5State::unknown(),

      // Default config state.
      metric,
    }
  }
}
impl<R: MultiRom, M: Metric<R>> Plan<R> for TextScrollWaitPlan<M> {
  type Value = M::ValueType;

  fn save(&self) -> PlanState {
    PlanState::TextScrollWaitState(TextScrollWaitPlanState { hjoy5_state: self.hjoy5_state.clone() })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::TextScrollWaitState(TextScrollWaitPlanState { hjoy5_state, }) = state {
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
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)> {
    gb.restore(s);
    if self.hjoy5_state.get_hjoy5(input).intersects(A | B) {
      gb.input(input);
      if let Some(metric_value) = self.metric.evaluate(gb) {
        if !gb.is_at_input() { gb.step(); }
        Some((gb.save(), Some(metric_value)))
      } else { None }
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
