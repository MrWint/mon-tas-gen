use serde_derive::{Serialize, Deserialize};
use std::{cmp::Ordering, rc::Rc};

use crate::metric::*;
use crate::multi::*;
use crate::rom::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkipTextsPlanState {
  num_texts_remaining: u32,
  at_wait: bool,
  inner_plan: Rc<PlanState>,
}
impl PartialOrd for SkipTextsPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.num_texts_remaining != other.num_texts_remaining {
      other.num_texts_remaining.partial_cmp(&self.num_texts_remaining)
    } else if self.at_wait != other.at_wait {
      self.at_wait.partial_cmp(&other.at_wait)
    } else {
      self.inner_plan.partial_cmp(&other.inner_plan)
    }
  }
}
impl PartialEq for SkipTextsPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress CheckForUserInterruption inputs
pub struct SkipTextsPlan<M> {
  // instance state
  text_plan: TextPlan<NullMetric>,
  text_scroll_wait_plan: TextScrollWaitPlan<NullMetric>,
  last_text_scroll_wait_plan: TextScrollWaitPlan<M>,
  num_texts_remaining: u32,
  at_wait: bool,

  // config state
  initial_num_texts: u32,
}
impl SkipTextsPlan<NullMetric> {
  pub fn new(num_texts: u32) -> Self {
    Self::with_metric(num_texts, NullMetric)
  }
}
impl<M> SkipTextsPlan<M> {
  pub fn with_metric(num_texts: u32, metric: M) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      text_plan: TextPlan::new(),
      text_scroll_wait_plan: TextScrollWaitPlan::new(),
      last_text_scroll_wait_plan: TextScrollWaitPlan::with_metric(metric),
      num_texts_remaining: num_texts,
      at_wait: false,

      // Default config state.
      initial_num_texts: num_texts,
    }
  }
  /// How often is an "end" of the text expected (can happen when special characters are printed). This avoid inputs conflicting with the next text's inputs.
  pub fn with_skip_ends(self, ends_to_be_skipped: u32) -> Self {
    Self { text_plan: self.text_plan.with_skip_ends(ends_to_be_skipped), ..self }
  }
}
impl<R: MultiRom + TextAddresses, M: Metric<R>> Plan<R> for SkipTextsPlan<M> {
  type Value = M::ValueType;

  fn save(&self) -> PlanState {
    let inner_plan = if self.at_wait {
      if self.num_texts_remaining <= 1 { Plan::<R>::save(&self.last_text_scroll_wait_plan) } else { Plan::<R>::save(&self.text_scroll_wait_plan) }
    } else { Plan::<R>::save(&self.text_plan) };
    PlanState::SkipTextsState(SkipTextsPlanState { num_texts_remaining: self.num_texts_remaining, at_wait: self.at_wait, inner_plan: Rc::new(inner_plan) })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::SkipTextsState(SkipTextsPlanState { num_texts_remaining, at_wait, inner_plan }) = state {
      self.num_texts_remaining = *num_texts_remaining;
      self.at_wait = *at_wait;
      if self.at_wait {
        if self.num_texts_remaining <= 1 { Plan::<R>::restore(&mut self.last_text_scroll_wait_plan, inner_plan) } else { Plan::<R>::restore(&mut self.text_scroll_wait_plan, inner_plan) }
      } else { Plan::<R>::restore(&mut self.text_plan, inner_plan) }
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.num_texts_remaining = self.initial_num_texts;
    self.at_wait = false;
    self.text_plan.initialize(gb, state);
  }
  fn is_safe(&self) -> bool { if self.at_wait {
    if self.num_texts_remaining <= 1 { Plan::<R>::is_safe(&self.last_text_scroll_wait_plan) } else { Plan::<R>::is_safe(&self.text_scroll_wait_plan) }
  } else { Plan::<R>::is_safe(&self.text_plan) } }
  fn get_blockable_inputs(&self) -> Input { if self.at_wait {
    if self.num_texts_remaining <= 1 { Plan::<R>::get_blockable_inputs(&self.last_text_scroll_wait_plan) } else { Plan::<R>::get_blockable_inputs(&self.text_scroll_wait_plan) }
  } else { Plan::<R>::get_blockable_inputs(&self.text_plan) } }

  fn canonicalize_input(&self, input: Input) -> Option<Input> { if self.at_wait {
    if self.num_texts_remaining <= 1 { Plan::<R>::canonicalize_input(&self.last_text_scroll_wait_plan, input) } else { Plan::<R>::canonicalize_input(&self.text_scroll_wait_plan, input) }
  } else { Plan::<R>::canonicalize_input(&self.text_plan, input) } }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<M::ValueType>)> {
    if self.at_wait {
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
            self.at_wait = false;
            self.text_plan.initialize(gb, &new_state);
          }
          Some((new_state, None))
        } else { None }
      }
    } else {
      if let Some((new_state, value)) = self.text_plan.execute_input(gb, state, input) {
        if value.is_some() {
          self.at_wait = true;
          if self.num_texts_remaining <= 1 { self.last_text_scroll_wait_plan.initialize(gb, &new_state) } else { self.text_scroll_wait_plan.initialize(gb, &new_state) };
        }
        Some((new_state, None))
      } else { None }
    }
  }
}
