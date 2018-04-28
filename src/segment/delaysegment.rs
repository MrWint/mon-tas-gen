use gb::*;
use rom::*;
use statebuffer::StateBuffer;
use std::marker::PhantomData;

const DELAY_SEGMENT_DEFAULT_MAX_SKIPS: u32 = 100;
const DELAY_SEGMENT_FULL_CUTOFF_DELAY: u32 = 3;
const DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY: u32 = 10;

pub struct DelaySegment<T, S: super::Segment<T>> {
  segment: S,
  debug_output: bool,
  max_skips: u32,
  _rom: PhantomData<T>,
}
impl<T, S: super::Segment<T>> DelaySegment<T, S> {
  pub fn new(segment: S) -> Self {
    Self {
      segment: segment,
      debug_output: false,
      max_skips: DELAY_SEGMENT_DEFAULT_MAX_SKIPS,
      _rom: PhantomData,
    }
  }
}
impl<R, S: super::Segment<R>> super::WithDebugOutput for DelaySegment<R, S> {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}
impl<R: JoypadAddresses + RngAddresses, S: super::Segment<R>> super::Segment<R> for DelaySegment<R, S> {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer {
    let mut result = StateBuffer::new();

    let mut active_states: Vec<State> = iter.into_iter().collect();
    let mut skips = 0;
    let mut full_frame = ::std::u32::MAX >> 1;
    let mut nonempty_frame = ::std::u32::MAX >> 1;
    while !active_states.is_empty() {
      if self.debug_output { println!("DelaySegment processing {} active states at {} skips", active_states.len(), skips); }
      let mut next_states = vec![];
      for s in active_states {
        gb.restore(&s);

        if skips > self.max_skips {
          if self.debug_output { println!("DelaySegment interrupting search (maxDelay)"); }
          continue;
        }
        let cur_frame = s.frame;
        if cur_frame > full_frame + DELAY_SEGMENT_FULL_CUTOFF_DELAY {
          if self.debug_output { println!("DelaySegment interrupting search (fullFrame)"); }
          continue;
        }
        if cur_frame > nonempty_frame + DELAY_SEGMENT_NONEMPTY_CUTOFF_DELAY {
          if self.debug_output { println!("DelaySegment interrupting search (nonemptyFrame)"); }
          continue;
        }

        gb.input(::gambatte::Input::empty());
        gb.step();
        next_states.push(gb.save());
        result.add_all(self.segment.execute(gb, vec![s]));
        if !result.is_empty() && cur_frame < nonempty_frame {
          nonempty_frame = cur_frame;
          if self.debug_output { println!("DelaySegment set nonempty_frame to {}", nonempty_frame); }
        }
        if result.is_full() && cur_frame < full_frame {
          full_frame = cur_frame;
          if self.debug_output { println!("DelaySegment set full_frame to {}", full_frame); }
        }
      }

      active_states = next_states;
      skips += 1;
    }

    if self.debug_output { println!("DelaySegment result {}", result); }
    result
  }
}
