use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct OverrideMoveSegment {
  move_index: Option<usize>,
  confirm_input: Input,
  buffer_size: usize,
}
impl OverrideMoveSegment {
  #[allow(dead_code)]
  pub fn new(move_index: usize) -> Self {
    Self {
      move_index: Some(move_index),
      confirm_input: Input::A | Input::B,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn dont_learn() -> Self {
    Self {
      move_index: None,
      confirm_input: Input::A | Input::B,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  /// Which button is used to confirm after the end of each TextSegment.
  pub fn with_confirm_input(self, confirm_input: Input) -> Self { Self { confirm_input, ..self } }
}
impl WithOutputBufferSize for OverrideMoveSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + TextAddresses + InputIdentificationAddresses> Segment<R> for OverrideMoveSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, mut sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    sb = SkipTextsSegment::new(1).with_skip_ends(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // trying to learn
    sb = SkipTextsSegment::new(1).with_skip_ends(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // move // .
    sb = SkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // but // mon // can't learn more
    sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // than four moves
    sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // delete to make room
    if let Some(move_index) = self.move_index {
      sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).with_skip_ends(2).with_confirm_input(Input::A).execute(gbe, sb); // for // move // ?
      sb = TextSegment::new().with_allowed_end_inputs(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); // which shoiuld be forgotten?
      if move_index == 3 {
        sb = MoveSegment::new(Input::DOWN).with_buffer_size(self.buffer_size).execute(gbe, sb);
        sb = MoveSegment::new(Input::empty()).with_buffer_size(self.buffer_size).execute(gbe, sb);
        sb = MoveSegment::new(Input::DOWN).with_buffer_size(self.buffer_size).execute(gbe, sb);
        sb = MoveSegment::new(Input::empty()).with_buffer_size(self.buffer_size).execute(gbe, sb);
        sb = MoveSegment::new(Input::DOWN | Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb);
      } else if move_index == 2 {
        sb = MoveSegment::new(Input::DOWN).with_buffer_size(self.buffer_size).execute(gbe, sb);
        sb = MoveSegment::new(Input::empty()).with_buffer_size(self.buffer_size).execute(gbe, sb);
        sb = MoveSegment::new(Input::DOWN | Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb);
      } else if move_index == 1 {
        sb = MoveSegment::new(Input::DOWN | Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb);
      } else {
        sb = MoveSegment::new(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb);
      }

      sb = TextSegment::new().expect_conflicting_inputs().with_buffer_size(self.buffer_size).execute(gbe, sb); // 1, 2 and
      sb = VerifyInputSegment::new("TextCommand0A").with_input(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); // skip delay
      sb = TextSegment::new().expect_conflicting_inputs().with_buffer_size(self.buffer_size).execute(gbe, sb); // 1, 2 and
      sb = VerifyInputSegment::new("TextCommand0A").with_input(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); // skip delay
      sb = MoveSegment::new(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb); // Cofirm text box

      sb = SkipTextsSegment::new(1).with_skip_ends(3).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // forgot // move // .
      sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // and
      sb = SkipTextsSegment::new(1).with_skip_ends(3).with_confirm_input(self.confirm_input).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // learned // move // !
    } else {
      sb = SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); // for // move // ?
      sb = SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb); // don't learn?
      sb = SkipTextsSegment::new(1).with_skip_ends(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // don't learn
      sb = SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(self.confirm_input).with_buffer_size(self.buffer_size).execute(gbe, sb); // move // .
    }

    Some(((), sb)).into_iter().collect()
  }
}
