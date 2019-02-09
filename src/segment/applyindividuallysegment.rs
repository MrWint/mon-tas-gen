use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use std::collections::HashMap;
use std::marker::PhantomData;

#[allow(dead_code)]
pub struct ApplyIndividuallySegment<R, S> {
  segment: S,
  buffer_size: usize,
  _rom: PhantomData<R>,
}
impl<R, S> ApplyIndividuallySegment<R, S> {
  #[allow(dead_code)]
  pub fn new(segment: S) -> Self {
    Self {
      segment,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      _rom: PhantomData,
    }
  }
}
impl<R, S> WithOutputBufferSize for ApplyIndividuallySegment<R, S> {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom, S: Segment<R>> Segment<R> for ApplyIndividuallySegment<R, S> {
  type Key = S::Key;

  fn execute_split<SR: StateRef, I: IntoIterator<Item=SR>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    let mut result: HashMap<S::Key, StateBuffer> = HashMap::new();
    for s in iter {
      for (value, states) in self.segment.execute_split(gbe, vec![s]).into_iter() {
        result.entry(value).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_all(states);
      }
    }
    result
  }
}
