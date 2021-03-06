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

pub struct DelaySegment<R: Rom, S: Segment<R>> {
  segment: S,
  buffer_size: usize,
  max_skips: u32,
  primary_key: Option<S::Key>,
  _rom: PhantomData<R>,
}
impl<R: Rom, S: Segment<R>> DelaySegment<R, S> {
  pub fn new(segment: S) -> Self {
    Self {
      segment,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      max_skips: DELAY_SEGMENT_DEFAULT_MAX_SKIPS,
      primary_key: None,
      _rom: PhantomData,
    }
  }
  pub fn with_primary_key(self, primary_key: S::Key) -> Self { Self { primary_key: Some(primary_key), ..self } }
}
impl<R: Rom, S: Segment<R>> WithOutputBufferSize for DelaySegment<R, S> {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom, S: Segment<R>> Segment<R> for DelaySegment<R, S> {
  type Key = S::Key;

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let mut result: HashMap<S::Key, StateBuffer> = HashMap::new();

    let active_buffer_size = sb.get_max_size();
    let mut active_states = sb;
    let mut skips = 0;
    let mut cycle_count_cutoff = std::u64::MAX >> 1;
    while !active_states.is_empty() {
      info!("DelaySegment processing {} active states at {} skips", active_states.len(), skips);
      // Try segment on current active states.
      for (value, states) in self.segment.execute_split(gbe, active_states.clone()).into_iter() {
        result.entry(value).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_all(states);
      }

      // Update loop exit conditions.
      let cur_max_cycle_count = (&active_states).into_iter().map(|s| s.cycle_count).max().unwrap();
      if let Some(primary_key) = &self.primary_key {
        if let Some(sb) = result.get(primary_key) {
          if sb.is_full() && cur_max_cycle_count + DELAY_SEGMENT_FULL_CUTOFF_DELAY < cycle_count_cutoff {
            cycle_count_cutoff = cur_max_cycle_count + DELAY_SEGMENT_FULL_CUTOFF_DELAY;
            info!("DelaySegment set cycle_count_cutoff (primary full) to {}", to_human_readable_time(cycle_count_cutoff));
          }
          if !sb.is_empty() && cur_max_cycle_count + DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY < cycle_count_cutoff {
            cycle_count_cutoff = cur_max_cycle_count + DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY;
            info!("DelaySegment set cycle_count_cutoff (primary nonempty) to {}", to_human_readable_time(cycle_count_cutoff));
          }
        }
      } else {
        if result.is_all_full() && cur_max_cycle_count + DELAY_SEGMENT_FULL_CUTOFF_DELAY < cycle_count_cutoff {
          cycle_count_cutoff = cur_max_cycle_count + DELAY_SEGMENT_FULL_CUTOFF_DELAY;
          info!("DelaySegment set cycle_count_cutoff (full) to {}", to_human_readable_time(cycle_count_cutoff));
        }
        if !result.is_empty() && cur_max_cycle_count + DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY < cycle_count_cutoff {
          cycle_count_cutoff = cur_max_cycle_count + DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY;
          info!("DelaySegment set cycle_count_cutoff (nonempty) to {}", to_human_readable_time(cycle_count_cutoff));
        }
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
        gb.delay();
        tx.send(gb.save()).unwrap();
      }).into_state_buffer(active_buffer_size);

      skips += 1;
    }

    debug!("DelaySegment result {}", result.to_string());
    result
  }
}
