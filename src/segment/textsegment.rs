use crate::gb::*;
use crate::metric::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;
use log::debug;
use std::collections::BTreeMap;
use std::marker::PhantomData;

/// Segment for skipping past text which is written out character by character.
/// Metrics are evaluated after the text finishes.
pub struct TextSegment<R: Rom + TextAddresses, M> {
  /// Input can be pressed on the last decision point before the end of the segment. Useful so inputs can be newly pressed immediately.
  allowed_end_inputs: Input,
  /// Metric evaluated at the end of this segment.
  metric: M,
  buffer_size: usize,
  expect_conflicting_inputs: bool,
  ignore_conflicting_inputs: bool,
  ends_to_be_skipped: u32,
  _rom: PhantomData<R>,
}
impl<R: Rom + TextAddresses, M: Metric<R>> WithOutputBufferSize for TextSegment<R, M> {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl<R: Rom + TextAddresses> TextSegment<R, NullMetric> {
  pub fn new() -> Self {
    TextSegment {
      allowed_end_inputs: Input::all(),
      metric: NullMetric,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      expect_conflicting_inputs: false,
      ignore_conflicting_inputs: false,
      ends_to_be_skipped: 0,
      _rom: PhantomData,
    }
  }
}
impl<R: Rom + TextAddresses, M: Metric<R>> TextSegment<R, M> {
  pub fn with_metric(metric: M) -> Self {
    TextSegment {
      allowed_end_inputs: Input::all(),
      metric,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      expect_conflicting_inputs: false,
      ignore_conflicting_inputs: false,
      ends_to_be_skipped: 0,
      _rom: PhantomData,
    }
  }
  /// conflicting future inputs are expected, and the default behavior of dropping these states is employed without warning.
  pub fn expect_conflicting_inputs(self) -> Self { Self { expect_conflicting_inputs: true, ..self } }
  /// conflicting future inputs can be safely ignored. Setting this may cause the segment to run over into the next input, pressing no buttons once.
  pub fn ignore_conflicting_inputs(self) -> Self { Self { ignore_conflicting_inputs: true, ..self } }
  /// How often is an "end" of the text expected (can happen when special characters are printed). This avoid inputs conflicting with the next text's inputs.
  pub fn with_skip_ends(self, ends_to_be_skipped: u32) -> Self { Self { ends_to_be_skipped, ..self } }
  /// Input can be pressed on the last decision point before the end of the segment. Useful so inputs can be newly pressed immediately.
  pub fn with_allowed_end_inputs(self, allowed_end_inputs: Input) -> Self { Self { allowed_end_inputs, ..self } }

  /// Checks whether the current decision point is a PrintLetterDelay input.
  /// Expected to be called when at a decision point.
  fn is_print_letter_delay_frame(gb: &mut Gb<R>) -> bool {
    gb.input(Input::empty());
    super::is_correct_input_use(gb, R::TEXT_BEFORE_JOYPAD_ADDRESS, R::TEXT_JOYPAD_ADDRESS, R::TEXT_AFTER_JOYPAD_ADDRESS)
  }
  /// Converts the hit address to the TextSegmentEnd it represents, if any.
  fn hit_address_to_text_segment_end(hit: i32) -> Option<TextSegmentEnd> {
    if R::TEXT_END_NOINPUT_ADDRESSES.contains(&hit) { Some(TextSegmentEnd::NoInput) }
    else if R::TEXT_END_WITHINPUT_ADDRESSES.contains(&hit) { Some(TextSegmentEnd::WithInput) }
    else { None }
  }
  /// Tracks the progress of the execution until the next decision point is reached, assuming that there can't be any input uses after the end of this segment which could possibly collide wit it.
  /// Expects to start after the input has been processed, but before TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS.
  fn progress_print_letter_delay_end_no_conflicts(&self, gb: &mut Gb<R>, s: State<PrintLetterState>, input: Input, mut printed_characters: u32, mut ends_to_be_skipped: u32) -> Option<State<PrintLetterProgressResult<M::ValueType>>> {
    loop {
      assert!(gb.step_until(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS, R::JOYPAD_READ_FIRST_ADDRESS]) == R::JOYPAD_READ_FIRST_ADDRESS); // this should never leave the function before the next vblank
      if gb.step_until(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS]) == 0 {
        return Some(gb.save_with_value(PrintLetterProgressResult::ContinueAtLetter { printed_characters, ends_to_be_skipped, last_input: input }));
      }
      printed_characters += 1;
      let hit = gb.step_until(&[R::TEXT_END_NOINPUT_ADDRESSES, R::TEXT_END_WITHINPUT_ADDRESSES].concat());
      if hit == 0 {
        return Some(gb.save_with_value(PrintLetterProgressResult::ContinueAtLetter { printed_characters, ends_to_be_skipped, last_input: input }));
      }
      let end = Self::hit_address_to_text_segment_end(hit).unwrap();
      if ends_to_be_skipped == 0 {
        if !self.allowed_end_inputs.contains(input) { return None; }
        if let Some(metric_value) = self.metric.evaluate(gb) {
          if gb.skipped_relevant_inputs { // restore state if metric overran next input
            gb.restore(&s);
            gb.input(input);
          }
          if !gb.is_at_input { gb.step(); }
          return Some(gb.save_with_value(PrintLetterProgressResult::Finished(TextSegmentResult { printed_characters, end, metric_value })));
        } else { return None; }
      }
      assert!(end != TextSegmentEnd::WithInput); // skipping over ends which require a button press is pointless.
      ends_to_be_skipped -= 1;
    }
  }
  /// Progress the execution from a decision point until the input use within PrintLetterDelay has been reached (if it exists).
  fn progress_print_letter_delay_run_until_input_processed(&self, gb: &mut Gb<R>, s: &State<PrintLetterState>, input: Input) -> Option<PrintLetterProgressResult<M::ValueType>> {
    gb.restore(s);
    gb.input(input);
    match s.value {
      PrintLetterState::BeforeFirstInputUse => {
        assert!(gb.step_until(&[R::TEXT_JOYPAD_ADDRESS]) != 0); // This is guaranteed to hit by is_print_letter_delay_frame
        Some(PrintLetterProgressResult::ContinueAtLetter { printed_characters: 0, ends_to_be_skipped: self.ends_to_be_skipped, last_input: Input::all() })
      },
      PrintLetterState::InProgress { mut printed_characters, mut ends_to_be_skipped, last_input } => {
        let mut hit = gb.step_until(&[&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS], R::TEXT_END_NOINPUT_ADDRESSES, R::TEXT_END_WITHINPUT_ADDRESSES, &[R::TEXT_BEFORE_JOYPAD_ADDRESS, R::TEXT_JOYPAD_ADDRESS], R::JOYPAD_USE_ADDRESSES].concat());
        if hit == R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS {
          printed_characters += 1;
          hit = gb.step_until(&[R::TEXT_END_NOINPUT_ADDRESSES, R::TEXT_END_WITHINPUT_ADDRESSES, &[R::TEXT_BEFORE_JOYPAD_ADDRESS, R::TEXT_JOYPAD_ADDRESS], R::JOYPAD_USE_ADDRESSES].concat());
        } 
        if let Some(end) = Self::hit_address_to_text_segment_end(hit) {
          if ends_to_be_skipped == 0 {
            if !self.allowed_end_inputs.contains(last_input) { return None; }
            if let Some(metric_value) = self.metric.evaluate(gb) {
              return Some(PrintLetterProgressResult::Finished(TextSegmentResult { printed_characters, end, metric_value }));
            } else { return None; }
          }
          assert!(end != TextSegmentEnd::WithInput); // skipping over ends which require a button press is pointless.
          ends_to_be_skipped -= 1;
          hit = gb.step_until(&[&[R::TEXT_BEFORE_JOYPAD_ADDRESS, R::TEXT_JOYPAD_ADDRESS], R::JOYPAD_USE_ADDRESSES].concat());
        }
        assert!(hit != 0, "found no input use in supposed decision point, stack [{}]", gb.get_stack_trace_string());
        if hit == R::TEXT_BEFORE_JOYPAD_ADDRESS {
          assert!(gb.step_until(&[R::TEXT_JOYPAD_ADDRESS]) != 0); // This is a decision point, and no other input use was hit so far
        } else {
          assert!(hit == R::TEXT_JOYPAD_ADDRESS, "hit unexpected decision point[1] at {:x}, stack [{}]", hit, gb.get_stack_trace_string());
          let hit = gb.step_until(&[R::TEXT_AFTER_JOYPAD_ADDRESS, R::TEXT_BEFORE_JOYPAD_ADDRESS]);
          assert!(hit == R::TEXT_AFTER_JOYPAD_ADDRESS, "hit unexpected decision point[2] at {:x}, stack [{}]", hit, gb.get_stack_trace_string());
        }
        Some(PrintLetterProgressResult::ContinueAtLetter { printed_characters, ends_to_be_skipped, last_input })
      },
    }
  }
  fn progress_print_letter_delay_letter_advance_input(&self, gb: &mut Gb<R>, s: State<PrintLetterState>, input: Input) -> Option<State<PrintLetterProgressResult<M::ValueType>>> {
    let (printed_characters, ends_to_be_skipped) = match self.progress_print_letter_delay_run_until_input_processed(gb, &s, input) {
      Some(PrintLetterProgressResult::Finished(_)) => return None, // already submitted by the no_input case
      Some(PrintLetterProgressResult::ContinueAtLetter { printed_characters, ends_to_be_skipped, last_input: _ }) => (printed_characters, ends_to_be_skipped),
      None => return None,
    };
    self.progress_print_letter_delay_end_no_conflicts(gb, s, input, printed_characters, ends_to_be_skipped)
  }
  fn progress_print_letter_delay_no_input(&self, gb: &mut Gb<R>, s: State<PrintLetterState>) -> Option<State<PrintLetterProgressResult<M::ValueType>>> {
    let (mut printed_characters, mut ends_to_be_skipped) = match self.progress_print_letter_delay_run_until_input_processed(gb, &s, Input::empty()) {
      Some(PrintLetterProgressResult::Finished(result)) => return Some(s.replace_value(PrintLetterProgressResult::Finished(result))),
      Some(PrintLetterProgressResult::ContinueAtLetter { printed_characters, ends_to_be_skipped, last_input: _ }) => (printed_characters, ends_to_be_skipped),
      None => return None,
    };
    loop {
      if gb.gb.read_memory(R::TEXT_DELAY_FRAME_COUNTER_MEM_ADDRESS) > 0 {
        return self.progress_print_letter_delay_end_no_conflicts(gb, s, Input::empty(), printed_characters, ends_to_be_skipped);
      }
      if gb.step_until(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS]) == 0 {
        return Some(gb.save_with_value(PrintLetterProgressResult::ContinueAtLetter { printed_characters, ends_to_be_skipped, last_input: Input::empty() }));
      }
      printed_characters += 1;
      let mut hit = gb.step_until(&[R::TEXT_END_NOINPUT_ADDRESSES, R::TEXT_END_WITHINPUT_ADDRESSES, &[R::TEXT_BEFORE_JOYPAD_ADDRESS], R::TEXT_SAFE_CONFLICTING_INPUT_ADDRESSES, R::JOYPAD_USE_ADDRESSES].concat());
      if let Some(end) = Self::hit_address_to_text_segment_end(hit) {
        if ends_to_be_skipped == 0 {
          if let Some(metric_value) = self.metric.evaluate(gb) {
            if gb.skipped_relevant_inputs { // metric overran next input -> no input conflict
              gb.restore(&s);
              gb.input(Input::empty());
            }
            if gb.is_at_input { return Some(gb.save_with_value(PrintLetterProgressResult::Finished(TextSegmentResult { printed_characters, end, metric_value }))); }
            let hit = gb.step_until(&[R::TEXT_SAFE_CONFLICTING_INPUT_ADDRESSES, R::JOYPAD_USE_ADDRESSES].concat()); // no R::TEXT_BEFORE_JOYPAD_ADDRESS, as this is the end
            if hit == 0 {
              return Some(gb.save_with_value(PrintLetterProgressResult::Finished(TextSegmentResult { printed_characters, end, metric_value })));
            }
            if R::TEXT_SAFE_CONFLICTING_INPUT_ADDRESSES.contains(&hit) || self.ignore_conflicting_inputs {
              gb.step(); /* finish overrunning into safe conflicting input */
              return Some(gb.save_with_value(PrintLetterProgressResult::Finished(TextSegmentResult { printed_characters, end, metric_value })));
            } else if !self.expect_conflicting_inputs { log::warn!("TextSegment found state with conflicting inputs between PrintLetterDelay and future inputs [{}].", gb.get_stack_trace_string()); }
            return None;
          } else { return None; }
        }
        assert!(end != TextSegmentEnd::WithInput); // skipping over ends which require a button press is pointless.
        ends_to_be_skipped -= 1;
        hit = gb.step_until(&[&[R::TEXT_BEFORE_JOYPAD_ADDRESS], R::TEXT_SAFE_CONFLICTING_INPUT_ADDRESSES, R::JOYPAD_USE_ADDRESSES].concat());
      }
      if hit == 0 {
        return Some(gb.save_with_value(PrintLetterProgressResult::ContinueAtLetter { printed_characters, ends_to_be_skipped, last_input: Input::empty() }));
      }
      if hit != R::TEXT_BEFORE_JOYPAD_ADDRESS {
        panic!("unexpected conflicting input detected at {:x} stack [{}]", hit, gb.get_stack_trace_string());
      }
      if gb.step_until(&[R::TEXT_JOYPAD_ADDRESS]) == 0 {
        return Some(gb.save_with_value(PrintLetterProgressResult::ContinueAtLetter { printed_characters, ends_to_be_skipped, last_input: Input::empty() }));
      }
    }
  }
}

impl<R: Rom + TextAddresses, M: Metric<R>> Segment<R> for TextSegment<R, M> {
  type Key = M::ValueType;

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    // Collect initial states.
    let mut active_states: BTreeMap<IntermediateBufferKey, StateBuffer<PrintLetterState>> = BTreeMap::new();
    let initial_state_buffer = gbe.execute(sb, move |gb, s, tx| {
      gb.restore(&s);
      if !Self::is_print_letter_delay_frame(gb) {
        log::warn!("found State not at PrintLetterDelay initially, maybe there's another input before. Dropping state.");
      } else {
        tx.send(s.replace_value(PrintLetterState::BeforeFirstInputUse)).unwrap();
      }
    }).into_state_buffer(self.buffer_size);
    assert!(!initial_state_buffer.is_empty());
    active_states.insert(IntermediateBufferKey { printed_characters: 0, ends_to_be_skipped: self.ends_to_be_skipped, last_input: Input::all() }, initial_state_buffer);

    let mut goal_buffer = HashMap::<Self::Key, StateBuffer>::new();
    while !active_states.is_empty() {
      let min_cycles: IntermediateBufferKey = active_states.keys().next().unwrap().clone();
      let max_cycles: IntermediateBufferKey = active_states.keys().next_back().unwrap().clone();
      let sb = active_states.remove(&min_cycles).unwrap();
      debug!("TextSegment loop cycles {}-{}, min cycle size {}, goal_buffer size {}, min cycle {:?}", min_cycles.printed_characters, max_cycles.printed_characters, sb.len(), goal_buffer.len(), min_cycles);

      for s in gbe.execute(sb, move |gb, s, tx| {
        if let Some(result) = self.progress_print_letter_delay_no_input(gb, s.clone()) {
          tx.send(result).unwrap();
        }
        if let Some(result) = self.progress_print_letter_delay_letter_advance_input(gb, s.clone(), Input::B) {
          tx.send(result).unwrap();
        }
        if let Some(result) = self.progress_print_letter_delay_letter_advance_input(gb, s.clone(), Input::A) {
          tx.send(result).unwrap();
        }
      }) {
        let (s, value) = s.split_state_and_value();
        match value {
          PrintLetterProgressResult::Finished(result) => {
            debug!("Add Goal state with result {:?}", result);
            goal_buffer.entry(result.metric_value).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_state(s)
          },
          PrintLetterProgressResult::ContinueAtLetter { printed_characters, ends_to_be_skipped, last_input } => {
            active_states.entry(IntermediateBufferKey { printed_characters, ends_to_be_skipped, last_input }).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_state(s.replace_value(PrintLetterState::InProgress { printed_characters, ends_to_be_skipped, last_input }))
          },
        }
      }
    }
    goal_buffer
  }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct IntermediateBufferKey {
  // order of members is important as it defines sorting order
  printed_characters: u32,
  ends_to_be_skipped: u32,
  last_input: Input,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum PrintLetterState {
  BeforeFirstInputUse,
  InProgress { printed_characters: u32, ends_to_be_skipped: u32, last_input: Input },
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct TextSegmentResult<V> {
  printed_characters: u32,
  end: TextSegmentEnd,
  metric_value: V,
}
#[derive(Debug, Eq, Hash, PartialEq)]
enum TextSegmentEnd {
  NoInput,       // $57, others
  WithInput,     // $4b ($55), $51, $58
}


#[derive(Debug, Eq, Hash, PartialEq)]
enum PrintLetterProgressResult<V> {
  Finished(TextSegmentResult<V>),
  ContinueAtLetter { printed_characters: u32, ends_to_be_skipped: u32, last_input: Input },
}
