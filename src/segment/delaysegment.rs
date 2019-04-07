use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use crate::util::*;
use std::collections::HashMap;
use std::marker::PhantomData;
use log::{debug,info};

const DELAY_SEGMENT_DEFAULT_MAX_SKIPS: u32 = 1000;
const DELAY_SEGMENT_FULL_CUTOFF_DELAY: u64 = 1 * 35112;
const DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY: u64 = 5 * 35112;

pub struct DelaySegment<R, S> {
  segment: S,
  buffer_size: usize,
  max_skips: u32,
  _rom: PhantomData<R>,
}
impl<R, S> DelaySegment<R, S> {
  pub fn new(segment: S) -> Self {
    Self {
      segment,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      max_skips: DELAY_SEGMENT_DEFAULT_MAX_SKIPS,
      _rom: PhantomData,
    }
  }
}
impl<R, S> WithOutputBufferSize for DelaySegment<R, S> {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom, S: Segment<R>> Segment<R> for DelaySegment<R, S> {
  type Key = S::Key;

  fn execute_split<SR: StateRef, I: IntoIterator<Item=SR>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    let mut result: HashMap<S::Key, StateBuffer> = HashMap::new();

    let mut active_states: Vec<State> = iter.into_iter().map(|s| s.to_state()).collect();
    let mut skips = 0;
    let mut cycle_count_cutoff = std::u64::MAX >> 1;
    while !active_states.is_empty() {
      info!("DelaySegment processing {} active states at {} skips", active_states.len(), skips);
      // Try segment on current active states.
      for (value, states) in self.segment.execute_split(gbe, &active_states).into_iter() {
        result.entry(value).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_all(states);
      }

      // Update loop exit conditions.
      let cur_max_cycle_count = active_states.iter().map(|s| s.cycle_count).max().unwrap();
      if result.is_all_full() && cur_max_cycle_count + DELAY_SEGMENT_FULL_CUTOFF_DELAY < cycle_count_cutoff {
        cycle_count_cutoff = cur_max_cycle_count + DELAY_SEGMENT_FULL_CUTOFF_DELAY;
        info!("DelaySegment set cycle_count_cutoff (full) to {}", to_human_readable_time(cycle_count_cutoff));
      }
      if !result.is_empty() && cur_max_cycle_count + DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY < cycle_count_cutoff {
        cycle_count_cutoff = cur_max_cycle_count + DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY;
        info!("DelaySegment set cycle_count_cutoff (nonempty) to {}", to_human_readable_time(cycle_count_cutoff));
      }

      // Update active states for next loop iteration.
      active_states = gbe.execute(active_states.into_iter().filter(|s| {
        if skips > self.max_skips {
          debug!("DelaySegment interrupting search (max_skips)");
          false
        } else if s.cycle_count > cycle_count_cutoff {
          debug!("DelaySegment interrupting search (cycle_count_cutoff)");
          false
        } else { true }
      }), move |gb, s, tx| {
        gb.restore(&s);
        gb.input(::gambatte::Input::empty());
        gb.step();
        tx.send(gb.save()).unwrap();
      }).into_iter().collect();

      skips += 1;
    }

    debug!("DelaySegment result {}", result.to_string());
    result
  }
}
