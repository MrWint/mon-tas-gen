use gb::*;
use rom::*;
use segment::*;
use statebuffer::StateBuffer;
use std::collections::HashMap;
use std::marker::PhantomData;
use util::*;

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
      buffer_size: ::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
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

impl<R: JoypadAddresses + RngAddresses, _S> DelaySegment<R, _S> {
  fn execute_internal<S: SplitSegment<R>, I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I, segment: &S) -> HashMap<S::Key, StateBuffer> {
    let mut result: HashMap<S::Key, StateBuffer> = HashMap::new();

    let mut active_states: Vec<State> = iter.into_iter().collect();
    let mut skips = 0;
    let mut full_cycle_count = ::std::u64::MAX >> 1;
    let mut nonempty_cycle_count = ::std::u64::MAX >> 1;
    while !active_states.is_empty() {
      if self.debug_output { println!("DelaySegment processing {} active states at {} skips", active_states.len(), skips); }
      let mut next_states = vec![];
      let mut cur_min_cycle_count =  ::std::u64::MAX >> 1;
      for s in active_states.iter() {
        cur_min_cycle_count = ::std::cmp::min(cur_min_cycle_count, s.cycle_count);

        if skips > self.max_skips {
          // if self.debug_output { println!("DelaySegment interrupting search (maxDelay)"); }
          continue;
        }
        if s.cycle_count > full_cycle_count + DELAY_SEGMENT_FULL_CUTOFF_DELAY {
          // if self.debug_output { println!("DelaySegment interrupting search (fullFrame)"); }
          continue;
        }
        if s.cycle_count > nonempty_cycle_count + DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY {
          // if self.debug_output { println!("DelaySegment interrupting search (nonemptyFrame)"); }
          continue;
        }

        gb.restore(s);
        gb.input(::gambatte::Input::empty());
        gb.step();
        next_states.push(gb.save());
      }
      for (value, states) in segment.execute_split(gb, active_states).into_iter() {
        result.entry(value).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_all(states);
      }
      if !result.is_empty() && cur_min_cycle_count < nonempty_cycle_count {
        nonempty_cycle_count = cur_min_cycle_count;
        if self.debug_output { println!("DelaySegment set nonempty_cycle_count to {}", to_human_readable_time(nonempty_cycle_count)); }
      }
      if result.is_all_full() && cur_min_cycle_count < full_cycle_count {
        full_cycle_count = cur_min_cycle_count;
        if self.debug_output { println!("DelaySegment set full_cycle_count to {}", to_human_readable_time(full_cycle_count)); }
      }

      active_states = next_states;
      skips += 1;
    }

    if self.debug_output { println!("DelaySegment result {}", result.to_string()); }
    result
  }
}


impl<R: JoypadAddresses + RngAddresses, S: Segment<R>> Segment<R> for DelaySegment<R, S> {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer {
    self.execute_internal(gb, iter, &WrapperSplitSegment { segment: &self.segment, _rom: PhantomData }).into_state_buffer()
  }
}
impl<R: JoypadAddresses + RngAddresses, S: SplitSegment<R>> SplitSegment<R> for DelaySegment<R, S> {
  type Key = S::Key;

  fn execute_split<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> HashMap<Self::Key, StateBuffer> {
    self.execute_internal(gb, iter, &self.segment)
  }
}

struct WrapperSplitSegment<'a, R, S: 'a> {
  segment: &'a S,
  _rom: PhantomData<R>,
}
impl<'a, R, S: Segment<R>> SplitSegment<R> for WrapperSplitSegment<'a, R, S> {
  type Key = ();

  fn execute_split<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> HashMap<Self::Key, StateBuffer> {
    let sb = self.segment.execute(gb, iter);
    let mut result: HashMap<Self::Key, StateBuffer> = HashMap::new();
    if !sb.is_empty() {
      result.insert((), sb);
    }
    result
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum DelayKey {
  ThisRound,
  NextRound,
}

impl<R: Rom, S: ParallelSegment<R>> ParallelSegment<R> for DelaySegment<R, S> {
  type Key = S::Key;

  fn execute_parallel<I: IntoIterator<Item=State>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    let mut result: HashMap<S::Key, StateBuffer> = HashMap::new();

    let mut active_states: Vec<State> = iter.into_iter().collect();
    let mut skips = 0;
    let mut full_cycle_count = ::std::u64::MAX >> 1;
    let mut nonempty_cycle_count = ::std::u64::MAX >> 1;
    while !active_states.is_empty() {
      if self.debug_output { println!("DelaySegment processing {} active states at {} skips", active_states.len(), skips); }
      let cur_min_cycle_count = active_states.iter().map(|s| s.cycle_count).min().unwrap();
      let max_skips = self.max_skips;
      let debug_output = self.debug_output;
      let mut ss = gbe.execute(active_states.drain(..), move |gb, s, tx| {
        let mut proceed_to_next_round = true;
        if skips > max_skips {
          if debug_output { println!("DelaySegment interrupting search (maxDelay)"); }
          proceed_to_next_round = false;
        } else if s.cycle_count > full_cycle_count + DELAY_SEGMENT_FULL_CUTOFF_DELAY {
          if debug_output { println!("DelaySegment interrupting search (fullFrame)"); }
          proceed_to_next_round = false;
        } else if s.cycle_count > nonempty_cycle_count + DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY {
          if debug_output { println!("DelaySegment interrupting search (nonemptyFrame)"); }
          proceed_to_next_round = false;
        }
        if proceed_to_next_round {
          gb.restore(&s);
          gb.input(::gambatte::Input::empty());
          gb.step();
          tx.send((DelayKey::NextRound, gb.save())).unwrap();
        }
        tx.send((DelayKey::ThisRound, s)).unwrap();
      }).into_state_buffer_map(self.buffer_size);
      if let Some(this_round) = ss.remove(&DelayKey::ThisRound) {
        for (value, states) in self.segment.execute_parallel(gbe, this_round).into_iter() {
          result.entry(value).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_all(states);
        }
      }
      if !result.is_empty() && cur_min_cycle_count < nonempty_cycle_count {
        nonempty_cycle_count = cur_min_cycle_count;
        if self.debug_output { println!("DelaySegment set nonempty_cycle_count to {}", to_human_readable_time(nonempty_cycle_count)); }
      }
      if result.is_all_full() && cur_min_cycle_count < full_cycle_count {
        full_cycle_count = cur_min_cycle_count;
        if self.debug_output { println!("DelaySegment set full_cycle_count to {}", to_human_readable_time(full_cycle_count)); }
      }

      if let Some(next_round) = ss.remove(&DelayKey::NextRound) {
        active_states.extend(next_round);
      }

      skips += 1;
    }

    if self.debug_output { println!("DelaySegment result {}", result.to_string()); }
    result
  }
}
