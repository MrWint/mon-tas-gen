use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use crate::util::*;
use std::collections::HashMap;
use std::marker::PhantomData;

const DELAY_SEGMENT_DEFAULT_MAX_SKIPS: u32 = 1000;
const DELAY_SEGMENT_FULL_CUTOFF_DELAY: u64 = 3 * 35112;
const DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY: u64 = 10 * 35112;

pub struct DelaySegment<R, S> {
  segment: S,
  debug_output: bool,
  buffer_size: usize,
  max_skips: u32,
  _rom: PhantomData<R>,
}
impl<R, S> DelaySegment<R, S> {
  pub fn new(segment: S) -> Self {
    Self {
      segment,
      debug_output: false,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      max_skips: DELAY_SEGMENT_DEFAULT_MAX_SKIPS,
      _rom: PhantomData,
    }
  }
}
impl<R, S> WithDebugOutput for DelaySegment<R, S> {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}
impl<R, S> WithOutputBufferSize for DelaySegment<R, S> {
  fn with_buffer_size(mut self, buffer_size: usize) -> Self { self.buffer_size = buffer_size; self }
}

impl<R: Rom, S: Segment<R>> Segment<R> for DelaySegment<R, S> {
  type Key = S::Key;

  fn execute_split<BS: StateRef, I: IntoIterator<Item=BS>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    let mut result: HashMap<S::Key, StateBuffer> = HashMap::new();

    let mut active_states: Vec<State> = iter.into_iter().map(|s| s.to_state()).collect();
    let mut skips = 0;
    let mut full_cycle_count = ::std::u64::MAX >> 1;
    let mut nonempty_cycle_count = ::std::u64::MAX >> 1;
    while !active_states.is_empty() {
      if self.debug_output { println!("DelaySegment processing {} active states at {} skips", active_states.len(), skips); }
      // Try segment on current active states.
      for (value, states) in self.segment.execute_split(gbe, &active_states).into_iter() {
        result.entry(value).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_all(states);
      }

      // Update loop exit conditions.
      let cur_min_cycle_count = active_states.iter().map(|s| s.cycle_count).min().unwrap();
      if !result.is_empty() && cur_min_cycle_count < nonempty_cycle_count {
        nonempty_cycle_count = cur_min_cycle_count;
        if self.debug_output { println!("DelaySegment set nonempty_cycle_count to {}", to_human_readable_time(nonempty_cycle_count)); }
      }
      if result.is_all_full() && cur_min_cycle_count < full_cycle_count {
        full_cycle_count = cur_min_cycle_count;
        if self.debug_output { println!("DelaySegment set full_cycle_count to {}", to_human_readable_time(full_cycle_count)); }
      }

      // Update active states for next loop iteration.
      active_states = gbe.execute(active_states.into_iter().filter(|s| {
        if skips > self.max_skips {
          if self.debug_output { println!("DelaySegment interrupting search (maxDelay)"); }
          false
        } else if s.cycle_count > full_cycle_count + DELAY_SEGMENT_FULL_CUTOFF_DELAY {
          if self.debug_output { println!("DelaySegment interrupting search (fullFrame)"); }
          false
        } else if s.cycle_count > nonempty_cycle_count + DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY {
          if self.debug_output { println!("DelaySegment interrupting search (nonemptyFrame)"); }
          false
        } else { true }
      }), move |gb, s, tx| {
        gb.restore(&s);
        gb.input(::gambatte::Input::empty());
        gb.step();
        tx.send(((), gb.save())).unwrap();
      }).into_iter().collect();

      skips += 1;
    }

    if self.debug_output { println!("DelaySegment result {}", result.to_string()); }
    result
  }
}
