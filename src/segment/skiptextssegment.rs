use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct SkipTextsSegment {
  num_texts: u32,
  confirm_input: Input,
  buffer_size: usize,
  ends_to_be_skipped: u32,
}
impl WithOutputBufferSize for SkipTextsSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl SkipTextsSegment {
  pub fn new(num_texts: u32, confirm_input: Input) -> Self {
    assert!(num_texts > 0);
    assert!(!confirm_input.contains(Input::A) || !confirm_input.contains(Input::B));
    SkipTextsSegment {
      num_texts,
      confirm_input,
      ends_to_be_skipped: 0,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  // How often is an "end" of the text expected (can happen when special characters are printed).
  pub fn with_skip_ends(self, ends_to_be_skipped: u32) -> Self { Self { ends_to_be_skipped, ..self } }
}

impl<R: Rom + TextAddresses> Segment<R> for SkipTextsSegment {
  type Key = ();

  fn execute_split<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    let skip_input = if self.confirm_input.contains(Input::A) { Input::B } else { Input::A };
    let text_segment = TextSegment::new(skip_input).with_skip_ends(self.ends_to_be_skipped).with_buffer_size(self.buffer_size);
    let confirm_segment = MoveSegment::new(self.confirm_input).with_buffer_size(self.buffer_size);
    let mut sb = text_segment.execute(gbe, iter);
    for _ in 1..self.num_texts {
      sb = confirm_segment.execute(gbe, sb);
      sb = text_segment.execute(gbe, sb);
    }
    confirm_segment.execute_split(gbe, sb)
  }
}
