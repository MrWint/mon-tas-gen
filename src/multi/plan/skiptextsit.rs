use serde_derive::{Serialize, Deserialize};
use std::{cmp::Ordering, rc::Rc};

use crate::metric::*;
use crate::multi::*;
use crate::rom::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkipTextsITPlanState {
  num_texts_remaining: u32,
  inner_plan: Rc<PlanState>,
}
impl PartialOrd for SkipTextsITPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.num_texts_remaining != other.num_texts_remaining {
      other.num_texts_remaining.partial_cmp(&self.num_texts_remaining)
    } else {
      self.inner_plan.partial_cmp(&other.inner_plan)
    }
  }
}
impl PartialEq for SkipTextsITPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress CheckForUserInterruption inputs
pub struct SkipTextsITPlan<M> {
  // instance state
  text_scroll_wait_plan: TextScrollWaitPlan<NullMetric>,
  last_text_scroll_wait_plan: TextScrollWaitPlan<M>,
  num_texts_remaining: u32,

  // config state
  initial_num_texts: u32,
}
impl SkipTextsITPlan<NullMetric> {
  pub fn new(num_texts: u32) -> Self {
    Self::with_metric(num_texts, NullMetric)
  }
}
impl<M> SkipTextsITPlan<M> {
  pub fn with_metric(num_texts: u32, metric: M) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      text_scroll_wait_plan: TextScrollWaitPlan::new(),
      last_text_scroll_wait_plan: TextScrollWaitPlan::with_metric(metric),
      num_texts_remaining: num_texts,

      // Default config state.
      initial_num_texts: num_texts,
    }
  }
}
impl<R: MultiRom, M: Metric<R>> Plan<R> for SkipTextsITPlan<M> {
  type Value = M::ValueType;

  fn save(&self) -> PlanState {
    let inner_plan = if self.num_texts_remaining <= 1 { Plan::<R>::save(&self.last_text_scroll_wait_plan) } else { Plan::<R>::save(&self.text_scroll_wait_plan) };
    PlanState::SkipTextsITState(SkipTextsITPlanState { num_texts_remaining: self.num_texts_remaining, inner_plan: Rc::new(inner_plan) })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::SkipTextsITState(SkipTextsITPlanState { num_texts_remaining, inner_plan }) = state {
      self.num_texts_remaining = *num_texts_remaining;
      if self.num_texts_remaining <= 1 { Plan::<R>::restore(&mut self.last_text_scroll_wait_plan, inner_plan) } else { Plan::<R>::restore(&mut self.text_scroll_wait_plan, inner_plan) }
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.num_texts_remaining = self.initial_num_texts;
    if self.num_texts_remaining <= 1 { self.last_text_scroll_wait_plan.initialize(gb, &state) } else { self.text_scroll_wait_plan.initialize(gb, &state) };
  }
  fn is_safe(&self) -> bool {
    if self.num_texts_remaining <= 1 { Plan::<R>::is_safe(&self.last_text_scroll_wait_plan) } else { Plan::<R>::is_safe(&self.text_scroll_wait_plan) }
  }
  fn get_blockable_inputs(&self) -> Input {
    if self.num_texts_remaining <= 1 { Plan::<R>::get_blockable_inputs(&self.last_text_scroll_wait_plan) } else { Plan::<R>::get_blockable_inputs(&self.text_scroll_wait_plan) }
  }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    if self.num_texts_remaining <= 1 { Plan::<R>::canonicalize_input(&self.last_text_scroll_wait_plan, input) } else { Plan::<R>::canonicalize_input(&self.text_scroll_wait_plan, input) }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<M::ValueType>)> {
    if self.num_texts_remaining <= 1 {
      if let Some((new_state, value)) = self.last_text_scroll_wait_plan.execute_input(gb, state, input) {
        if value.is_some() {
          return Some((new_state, value));
        }
        Some((new_state, None))
      } else { None }
    } else {
      if let Some((new_state, value)) = self.text_scroll_wait_plan.execute_input(gb, state, input) {
        if value.is_some() {
          self.num_texts_remaining -= 1;
          if self.num_texts_remaining <= 1 { self.last_text_scroll_wait_plan.initialize(gb, &new_state) } else { self.text_scroll_wait_plan.initialize(gb, &new_state) };
        }
        Some((new_state, None))
      } else { None }
    }
  }
}
