use gambatte::Input;
use gb::*;
use rom::*;
use segment::*;
use statebuffer::StateBuffer;
use std::collections::BTreeMap;

pub struct TextSegment {
  skip_input: Input,
  debug_output: bool,
  buffer_size: usize,
  expect_conflicting_inputs: bool,
  ignore_conflicting_inputs: bool,
}
impl WithDebugOutput for TextSegment {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}
impl WithOutputBufferSize for TextSegment {
  fn with_buffer_size(mut self, buffer_size: usize) -> Self { self.buffer_size = buffer_size; self }
}
impl TextSegment {
  pub fn new(skip_input: Input) -> Self {
    TextSegment {
      skip_input,
      debug_output: false,
      buffer_size: ::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      expect_conflicting_inputs: false,
      ignore_conflicting_inputs: false,
    }
  }
  // conflicting future inputs are expected, and the default behavior of dropping these states is employed without warning.
  pub fn expect_conflicting_inputs(mut self) -> Self { self.expect_conflicting_inputs = true; self }
  // conflicting future inputs can be safely ignored. Setting this may cause the segment to run over into the next input, pressing no buttons once.
  pub fn ignore_conflicting_inputs(mut self) -> Self { self.ignore_conflicting_inputs = true; self }

  fn is_print_letter_delay_frame<R: JoypadAddresses + RngAddresses + TextAddresses>(gb: &mut Gb<R>) -> bool {
    gb.input(Input::empty());
    super::is_correct_input_use(gb, R::TEXT_BEFORE_JOYPAD_ADDRESS, R::TEXT_JOYPAD_ADDRESS, R::TEXT_AFTER_JOYPAD_ADDRESS)
  }
  fn progress_print_letter_delay_frame<R: JoypadAddresses + RngAddresses + TextAddresses>(&self, gb: &mut Gb<R>, s: State) -> Vec<(State, u32)> {
    let mut result = vec![];
    let mut num_done = 0;

    gb.restore(&s);
    gb.input(self.skip_input);
    if gb.step_until(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS, R::TEXT_JOYPAD_ADDRESS]) == R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS {
      num_done += 1;
      gb.step_until(&[R::TEXT_JOYPAD_ADDRESS]); // This is guaranteed to hit by is_print_letter_delay_frame
    }
    let delay = gb.gb.read_memory(R::TEXT_DELAY_FRAME_COUNTER_MEM_ADDRESS);
    assert!(gb.step_until_or_any_vblank(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS]) == 0); // this should never leave the function
    result.push((gb.save(), num_done));
    if delay >= 1 { // there's no way this input has an affect beyond PrintLetterDelay, there will always be a VBlank before leaving.
      gb.restore(&s);
      gb.input(Input::empty());
      gb.step_until(&[R::TEXT_JOYPAD_ADDRESS]); // This is guaranteed to hit by is_print_letter_delay_frame
      assert!(gb.step_until_or_any_vblank(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS]) == 0); // this should never leave the function
      result.push((gb.save(), num_done));
    } else if delay == 0 { // not pressing anything may conflict with future inputs, check whether it does
      gb.restore(&s);
      gb.input(Input::empty());
      gb.step_until(&[R::TEXT_JOYPAD_ADDRESS]); // This is guaranteed to hit by is_print_letter_delay_frame
      let (stayed, num_cycles) = Self::check_stays_within_print_letter_delay(gb);
      if !stayed && self.ignore_conflicting_inputs { gb.step(); /* finish overrunning */ }
      if stayed || self.ignore_conflicting_inputs { result.push((gb.save(), num_done + num_cycles)); }
      else if !self.expect_conflicting_inputs { println!("WARNING: TextSegment found state with conflicting inputs between PrintLetterDelay and future inputs [{}].", gb.get_stack_trace_string()); }
    }
    result
  }
  fn check_stays_within_print_letter_delay<R: JoypadAddresses + RngAddresses + TextAddresses>(gb: &mut Gb<R>) -> (bool, u32) {
    let mut num_cycles = 0;
    loop {
      if gb.step_until(&[R::TEXT_AFTER_JOYPAD_ADDRESS]) == 0 { return (true, num_cycles); }
      let mut hit = gb.step_until(&[&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS, R::TEXT_BEFORE_JOYPAD_ADDRESS], R::TEXT_SAFE_CONFLICTING_INPUT_ADDRESSES, R::JOYPAD_USE_ADDRESSES].concat());
      if hit == R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS {
        num_cycles += 1;
        hit = gb.step_until(&[&[R::TEXT_BEFORE_JOYPAD_ADDRESS], R::TEXT_SAFE_CONFLICTING_INPUT_ADDRESSES, R::JOYPAD_USE_ADDRESSES].concat());
      }
      if hit == 0 { return (true, num_cycles); }
      if hit != R::TEXT_BEFORE_JOYPAD_ADDRESS {
        if R::TEXT_SAFE_CONFLICTING_INPUT_ADDRESSES.contains(&hit) {
          gb.step(); /* finish overrunning into safe conflicting input */
          return (true, num_cycles);
        }
        return (false, num_cycles);
      }
      if gb.step_until(&[R::TEXT_JOYPAD_ADDRESS]) == 0 { return (true, num_cycles); }
      if gb.gb.read_memory(R::TEXT_DELAY_FRAME_COUNTER_MEM_ADDRESS) > 0 {
        assert!(gb.step_until_or_any_vblank(&[R::TEXT_PRINT_LETTER_DELAY_DONE_ADDRESS]) == 0); // this should never leave the function
        return (true, num_cycles);
      }
    }
  }
}
impl<R: JoypadAddresses + RngAddresses + TextAddresses> Segment<R> for TextSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer {
    // intermediate buffers are larger by default so the goal buffer ends up with enough (varied) states.
    let intermediate_buffer_size = self.buffer_size; // << 2;

    let mut goal_buffer = StateBuffer::with_max_size(self.buffer_size);
    let mut active_states: BTreeMap<u32, StateBuffer> = BTreeMap::new();
    for s in iter.into_iter() {
      gb.restore(&s);
      if !Self::is_print_letter_delay_frame(gb) {
        println!("WARNING: found State not at PrintLetterDelay initially, maybe there's another input before. Dropping state.");
      } else {
        active_states.entry(0).or_insert_with(|| StateBuffer::with_max_size(intermediate_buffer_size)).add_state(s);
      }
    }
    while !active_states.is_empty() {
      let min_cycles: u32 = *active_states.keys().next().unwrap();
      let max_cycles: u32 = *active_states.keys().next_back().unwrap();
      let sb = active_states.remove(&min_cycles).unwrap();
      if self.debug_output { println!("TextSegment loop cycles {}-{}, min cycle size {}, goal_buffer size {}", min_cycles, max_cycles, sb.len(), goal_buffer.len()); }
      for s in sb.into_iter() {
        for (s, num_cycles) in self.progress_print_letter_delay_frame(gb, s) {
          gb.restore(&s);
          if Self::is_print_letter_delay_frame(gb) {
            active_states.entry(min_cycles + num_cycles).or_insert_with(|| StateBuffer::with_max_size(intermediate_buffer_size)).add_state(s);
          } else {
            goal_buffer.add_state(s);
          }
        }
      }
    }
    goal_buffer
  }
}
