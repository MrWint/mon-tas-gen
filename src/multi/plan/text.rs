use crate::multi::*;
use crate::rom::*;

// Plan to progress PrintLetterDelay inputs
pub struct TextPlan<M> {
  // instance state
  printed_characters: u32,
  ends_to_be_skipped: u32,

  // config state
  initial_ends_to_be_skipped: u32,
  metric: M,
}
impl TextPlan<NullMetric> {
  pub fn new() -> Self { Self::with_metric(NullMetric) }
}
impl<M> TextPlan<M> {
  pub fn with_metric(metric: M) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      printed_characters: 0,
      ends_to_be_skipped: 0,

      // Default config state.
      initial_ends_to_be_skipped: 0,
      metric,
    }
  }
  /// How often is an "end" of the text expected (can happen when special characters are printed). This avoid inputs conflicting with the next text's inputs.
  pub fn with_skip_ends(self, initial_ends_to_be_skipped: u32) -> Self { Self { initial_ends_to_be_skipped, ..self } }
}
impl<R: MultiRom + TextAddresses, M: Metric<R>> Plan<R> for TextPlan<M> {
  type Value = M::ValueType;

  fn save(&self) -> PlanState {
    PlanState::TextState { printed_characters: self.printed_characters, ends_to_be_skipped: self.ends_to_be_skipped }
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::TextState { printed_characters, ends_to_be_skipped, } = state {
      self.printed_characters = *printed_characters;
      self.ends_to_be_skipped = *ends_to_be_skipped;
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, _gb: &mut Gb<R>, _state: &GbState) {
    self.printed_characters = 0;
    self.ends_to_be_skipped = self.initial_ends_to_be_skipped;
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    Some(input & (Input::A | Input::B))
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)> {
    gb.restore(s);
    gb.input(input);
    if !input.intersects(Input::A | Input::B) {
      let input_frame_lo = s.get_input_frame_lo();
      let input_frame_hi = s.get_input_frame_hi();
      while gb.step_until(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS]) == 0 {
        if gb.get_input_frame_lo() != input_frame_lo || gb.get_input_frame_hi() != input_frame_hi {
          return Some((gb.save(), None));
        }
        gb.input(input);
      }
    } else {
      assert!(gb.step_until(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS]) != 0);
    }
    // PrintLetterDelay is done.
    self.printed_characters += 1;
    let hit = gb.step_until(&[R::TEXT_END_NOINPUT_ADDRESSES, R::TEXT_END_WITHINPUT_ADDRESSES, &[R::TEXT_BEFORE_JOYPAD_ADDRESS]].concat());
    assert!(hit != 0, "Failed to track continuation after PrintLetterDelay, stack [{}]", gb.get_stack_trace_string());
    if hit == R::TEXT_BEFORE_JOYPAD_ADDRESS {
      gb.step(); // Finish to next PrintLetterDelay.
      return Some((gb.save(), None));
    }
    // At some end.
    if self.ends_to_be_skipped > 0 {
      assert!(R::TEXT_END_NOINPUT_ADDRESSES.contains(&hit)); // skipping over ends which require a button press is pointless.
      self.ends_to_be_skipped -= 1;
      assert!(gb.step_until(&[R::TEXT_BEFORE_JOYPAD_ADDRESS]) != 0); // After a skipped input we assume another PrintLetterDelay input.
      gb.step(); // Finish to next PrintLetterDelay.
      return Some((gb.save(), None));
    }
    // Final end has been reached.
    if let Some(metric_value) = self.metric.evaluate(gb) {
      if !gb.is_at_input() { gb.step(); }
      log::trace!("Text finished with {} characters printed", self.printed_characters);
      Some((gb.save(), Some(metric_value)))
    } else { None }
  }
}
