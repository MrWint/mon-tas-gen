use gb::*;
use rom::*;
use segment::*;
use statebuffer::StateBuffer;
use std::collections::HashMap;
use std::marker::PhantomData;

#[allow(dead_code)]
pub struct ApplyIndividuallySegment<R, S> {
  segment: S,
  debug_output: bool,
  buffer_size: usize,
  _rom: PhantomData<R>,
}
impl<R, S> ApplyIndividuallySegment<R, S> {
  #[allow(dead_code)]
  pub fn new(segment: S) -> Self {
    Self {
      segment: segment,
      debug_output: false,
      buffer_size: ::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      _rom: PhantomData,
    }
  }
}
impl<R, S> WithDebugOutput for ApplyIndividuallySegment<R, S> {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}
impl<R, S> WithOutputBufferSize for ApplyIndividuallySegment<R, S> {
  fn with_buffer_size(mut self, buffer_size: usize) -> Self { self.buffer_size = buffer_size; self }
}

impl<R: JoypadAddresses + RngAddresses, S: Segment<R>> Segment<R> for ApplyIndividuallySegment<R, S> {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer {
    StateBuffer::from_iter_sized(iter.into_iter().flat_map(|s| self.segment.execute(gb, vec![s])), self.buffer_size)
  }
}
impl<R: JoypadAddresses + RngAddresses, S: SplitSegment<R>> SplitSegment<R> for ApplyIndividuallySegment<R, S> {
  type KeyType = S::KeyType;

  fn execute_split<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> HashMap<Self::KeyType, StateBuffer> {
    let mut result: HashMap<S::KeyType, StateBuffer> = HashMap::new();
    for s in iter {
      for (value, states) in self.segment.execute_split(gb, vec![s]).into_iter() {
        result.entry(value).or_insert(StateBuffer::with_max_size(self.buffer_size)).add_all(states);
      }
    }
    result
  }
}
