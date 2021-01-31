use crate::metric::*;
use crate::multi::*;
use crate::rom::*;

// Plan to progress CheckForUserInterruption inputs
pub struct SkipTextsPlan {
  // instance state
  text_plan: TextPlan<NullMetric>,
  text_scroll_wait_plan: TextScrollWaitPlan<NullMetric>,
  num_texts_remaining: u32,
  at_wait: bool,

  // config state
  initial_num_texts: u32,
}
impl SkipTextsPlan {
  pub fn new(num_texts: u32) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      text_plan: TextPlan::new(),
      text_scroll_wait_plan: TextScrollWaitPlan::new(),
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
impl<R: MultiRom + TextAddresses> Plan<R> for SkipTextsPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    let inner_plan = if self.at_wait { Plan::<R>::save(&self.text_scroll_wait_plan) } else { Plan::<R>::save(&self.text_plan) };
    PlanState::SkipTextsState { num_texts_remaining: self.num_texts_remaining, at_wait: self.at_wait, inner_plan: std::rc::Rc::new(inner_plan) }
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::SkipTextsState { num_texts_remaining, at_wait, inner_plan } = state {
      self.num_texts_remaining = *num_texts_remaining;
      self.at_wait = *at_wait;
      if self.at_wait { Plan::<R>::restore(&mut self.text_scroll_wait_plan, inner_plan) } else { Plan::<R>::restore(&mut self.text_plan, inner_plan) }
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.num_texts_remaining = self.initial_num_texts;
    self.at_wait = false;
    self.text_plan.initialize(gb, state);
  }
  fn is_safe(&self) -> bool { if self.at_wait { Plan::<R>::is_safe(&self.text_scroll_wait_plan) } else { Plan::<R>::is_safe(&self.text_plan) } }
  fn get_blockable_inputs(&self) -> Input { if self.at_wait { Plan::<R>::get_blockable_inputs(&self.text_scroll_wait_plan) } else { Plan::<R>::get_blockable_inputs(&self.text_plan) } }

  fn canonicalize_input(&self, input: Input) -> Option<Input> { if self.at_wait { Plan::<R>::canonicalize_input(&self.text_scroll_wait_plan, input) } else { Plan::<R>::canonicalize_input(&self.text_plan, input) } }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    if self.at_wait {
      if let Some((new_state, value)) = self.text_scroll_wait_plan.execute_input(gb, state, input) {
        if value.is_some() {
          if self.num_texts_remaining <= 1 {
            return Some((new_state, Some(())));
          }
          self.num_texts_remaining -= 1;
          self.at_wait = false;
          self.text_plan.initialize(gb, &new_state);
        }
        Some((new_state, None))
      } else { None }
    } else {
      if let Some((new_state, value)) = self.text_plan.execute_input(gb, state, input) {
        if value.is_some() {
          self.at_wait = true;
          self.text_scroll_wait_plan.initialize(gb, &new_state);
        }
        Some((new_state, None))
      } else { None }
    }
  }
}
