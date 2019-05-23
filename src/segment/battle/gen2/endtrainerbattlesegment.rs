use crate::rom::*;
use crate::segment::*;
use crate::segment::battle::gen2::*;
use crate::segment::overworld::gen2::*;
use crate::statebuffer::StateBuffer;

pub struct EndTrainerBattleSegment {
  num_defeat_texts: usize,
  level_up: bool,
  learn_move: bool,
  skip_learning_move: bool,
  override_move_index: Option<usize>,
  buffer_size: usize,
}
impl EndTrainerBattleSegment {
  #[allow(dead_code)]
  pub fn with_defeat_texts(num_defeat_texts: usize) -> Self {
    Self {
      num_defeat_texts,
      level_up: false,
      learn_move: false,
      skip_learning_move: false,
      override_move_index: None,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn with_level_up(self) -> Self { Self { level_up: true, ..self } }
  pub fn with_learn_move(self) -> Self { Self { learn_move: true, ..self } }
  pub fn with_skip_learning_move(self) -> Self { Self { skip_learning_move: true, ..self } }
  pub fn with_override_move_index(self, move_index: usize) -> Self { Self { override_move_index: Some(move_index), ..self } }
}
impl WithOutputBufferSize for EndTrainerBattleSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + TextAddresses + Gen2MapEventsAddresses + Gen2BattleSwitchMonAddresses> Segment<R> for EndTrainerBattleSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let sb = SkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // enemy // mon // fainted
    let mut sb = SkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // gained // num XP
    if self.level_up {
      sb = SkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // grew to level // X
    }
    if self.learn_move {
      sb = SkipTextsSegment::new(1).with_skip_ends(3).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // learns // move // .
    } else if let Some(override_move_index) = self.override_move_index {
      sb = OverrideMoveSegment::new(override_move_index).with_buffer_size(self.buffer_size).execute(gbe, sb);
    } else if self.skip_learning_move {
      sb = OverrideMoveSegment::dont_learn().with_buffer_size(self.buffer_size).execute(gbe, sb);
    }
    let sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // ??? was defeated
    let sb = SkipTextsSegment::new(self.num_defeat_texts).with_buffer_size(self.buffer_size).execute(gbe, sb); // defeat texts
    let sb = SkipTextsSegment::new(1).with_skip_ends(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // player got // X for winning
    let sb = SkipScriptSegment::new().with_buffer_size(self.buffer_size).execute(gbe, sb); // After trainer map scripts

    Some(((), sb)).into_iter().collect()
  }
}
