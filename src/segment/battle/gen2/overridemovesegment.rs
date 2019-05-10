use crate::rom::*;
use crate::segment::*;
use crate::segment::battle::gen2::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct OverrideMoveSegment {
  expected_next_mon: Option<(Pokemon, u8)>,
  move_index: Option<usize>,
  buffer_size: usize,
}
impl OverrideMoveSegment {
  #[allow(dead_code)]
  pub fn new(move_index: usize) -> Self {
    Self {
      expected_next_mon: None,
      move_index: Some(move_index),
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn dont_learn() -> Self {
    Self {
      expected_next_mon: None,
      move_index: None,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }

  pub fn with_expected_next_mon(self, expected_pokemon: Pokemon, expected_level: u8) -> Self { Self { expected_next_mon: Some((expected_pokemon, expected_level)), ..self } }
}
impl WithOutputBufferSize for OverrideMoveSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + TextAddresses + Gen2BattleSwitchMonAddresses> Segment<R> for OverrideMoveSegment {
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
        sb = MoveSegment::new(Input::UP).with_buffer_size(self.buffer_size).execute(gbe, sb);
      } else if move_index == 2 {
        sb = MoveSegment::new(Input::DOWN).with_buffer_size(self.buffer_size).execute(gbe, sb);
        sb = MoveSegment::new(Input::empty()).with_buffer_size(self.buffer_size).execute(gbe, sb);
        sb = MoveSegment::new(Input::DOWN).with_buffer_size(self.buffer_size).execute(gbe, sb);
      } else if move_index == 1 {
        sb = MoveSegment::new(Input::DOWN).with_buffer_size(self.buffer_size).execute(gbe, sb);
      }
      sb = MoveSegment::new(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb);

      sb = SkipTextsSegment::new(2).with_confirm_input(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); // 1, 2, poof!
      sb = MoveSegment::new(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb); // confirm text line
      sb = SkipTextsSegment::new(1).with_skip_ends(3).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // forgot // move // .
      sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // and
      sb = TextSegment::new().with_skip_ends(3).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // learned // move // !
    } else {
      sb = SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); // for // move // ?
      sb = SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb); // don't learn?
      sb = SkipTextsSegment::new(1).with_skip_ends(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // don't learn
      sb = TextSegment::new().with_skip_ends(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // move // .
    }

    if let Some((expected_pokemon, expected_level)) = self.expected_next_mon {
      sb = DelaySegment::new(MoveSegment::with_metric(Input::A | Input::B, Gen2SwitchMonMetric {}.debug_print().expect((expected_pokemon, expected_level)))).with_buffer_size(self.buffer_size).execute(gbe, sb);
    } else {
      sb = MoveSegment::new(Input::A | Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb);
    }

    Some(((), sb)).into_iter().collect()
  }
}
