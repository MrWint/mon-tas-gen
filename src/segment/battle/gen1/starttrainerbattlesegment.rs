use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct StartTrainerBattleSegment {
  num_pre_battle_texts: usize,
  buffer_size: usize,
}
impl StartTrainerBattleSegment {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self {
      num_pre_battle_texts: 0,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn with_pre_battle_texts(self, num_pre_battle_texts: usize) -> Self { Self { num_pre_battle_texts, ..self } }
}
impl WithOutputBufferSize for StartTrainerBattleSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + TextAddresses> Segment<R> for StartTrainerBattleSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, mut sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    if self.num_pre_battle_texts > 0 {
      sb = SkipTextsSegment::new(self.num_pre_battle_texts).with_buffer_size(self.buffer_size).with_confirm_input(Input::B).execute(gbe, sb); // pre-battle texts
    }
    let sb = SkipTextsSegment::new(1).with_skip_ends(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // trainer // wants to fight
    let sb = TextSegment::new().with_skip_ends(7).with_buffer_size(self.buffer_size).with_allowed_end_inputs(Input::B).execute(gbe, sb); // trainer // sent out // mon // go // mon // !

    Some(((), sb)).into_iter().collect()
  }
}
