use crate::rom::*;
use crate::segment::*;
use crate::segment::overworld::gen2::*;
use crate::statebuffer::StateBuffer;

pub struct EndTrainerBattleSegment {
  num_defeat_texts: usize,
  level_up: bool,
  buffer_size: usize,
}
impl EndTrainerBattleSegment {
  #[allow(dead_code)]
  pub fn with_defeat_texts(num_defeat_texts: usize) -> Self {
    Self {
      num_defeat_texts,
      level_up: false,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn with_level_up(self) -> Self { Self { level_up: true, ..self } }
}
impl WithOutputBufferSize for EndTrainerBattleSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + TextAddresses + Gen2MapEventsAddresses> Segment<R> for EndTrainerBattleSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let sb = SkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // enemy // mon // fainted
    let mut sb = SkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // gained // num XP
    if self.level_up {
      sb = SkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // grew to level // X
    }
    let sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // ??? was defeated
    let sb = SkipTextsSegment::new(self.num_defeat_texts).with_buffer_size(self.buffer_size).execute(gbe, sb); // defeat texts
    let sb = SkipTextsSegment::new(1).with_skip_ends(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // player got // X for winning
    let sb = SkipScriptSegment::new().with_buffer_size(self.buffer_size).execute(gbe, sb); // After trainer map scripts

    Some(((), sb)).into_iter().collect()
  }
}
