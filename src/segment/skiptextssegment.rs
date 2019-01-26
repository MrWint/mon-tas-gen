use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct SkipTextsSegment {
  num_texts: u32,
  confirm_input: Input,
  buffer_size: usize,
}
impl WithOutputBufferSize for SkipTextsSegment {
  fn with_buffer_size(mut self, buffer_size: usize) -> Self { self.buffer_size = buffer_size; self }
}
impl SkipTextsSegment {
  pub fn new(num_texts: u32, confirm_input: Input) -> Self {
    assert!(num_texts > 0);
    assert!(!confirm_input.contains(Input::A) || !confirm_input.contains(Input::B));
    SkipTextsSegment {
      num_texts,
      confirm_input,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
}
impl<R: JoypadAddresses + RngAddresses + TextAddresses> Segment<R> for SkipTextsSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer {
    let skip_input = if self.confirm_input.contains(Input::A) { Input::B } else { Input::A };
    let text_segment = TextSegment::new(skip_input).with_buffer_size(self.buffer_size);
    let confirm_segment = MoveSegment::new(self.confirm_input).with_buffer_size(self.buffer_size);
    let mut sb = text_segment.execute(gb, iter);
    for _ in 1..self.num_texts {
      sb = confirm_segment.execute(gb, sb);
      sb = text_segment.execute(gb, sb);
    }
    confirm_segment.execute(gb, sb)
  }
}

impl<R: Rom + TextAddresses> ParallelSegment<R> for SkipTextsSegment {
  type Key = ();

  fn execute_parallel<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    let skip_input = if self.confirm_input.contains(Input::A) { Input::B } else { Input::A };
    let text_segment = TextSegment::new(skip_input).with_buffer_size(self.buffer_size);
    let confirm_segment = MoveSegment::new(self.confirm_input).with_buffer_size(self.buffer_size);
    let mut sb = text_segment.execute_parallel_single(gbe, iter);
    for _ in 1..self.num_texts {
      sb = confirm_segment.execute_parallel_single(gbe, sb);
      sb = text_segment.execute_parallel_single(gbe, sb);
    }
    confirm_segment.execute_parallel(gbe, sb)
  }
}
