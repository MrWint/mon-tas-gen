use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct Gen1ITStartTrainerBattleSegment {
  num_pre_battle_texts: usize,
  buffer_size: usize,
}
impl Gen1ITStartTrainerBattleSegment {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self {
      num_pre_battle_texts: 0,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn with_pre_battle_texts(self, num_pre_battle_texts: usize) -> Self { Self { num_pre_battle_texts, ..self } }
}
impl WithOutputBufferSize for Gen1ITStartTrainerBattleSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + JoypadLowSensitivityAddresses> Segment<R> for Gen1ITStartTrainerBattleSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, mut sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    if self.num_pre_battle_texts > 0 {
      sb = Gen1ITSkipTextsSegment::new(self.num_pre_battle_texts).with_buffer_size(self.buffer_size).with_confirm_input(Input::B).execute(gbe, sb); // pre-battle texts (end on b to prevent hold open dialog)
    }
    let sb = Gen1ITSkipTextsSegment::new(1).with_skip_ends(1).with_buffer_size(self.buffer_size).with_confirm_input(Input::B).execute(gbe, sb); // trainer // wants to fight

    Some(((), sb)).into_iter().collect()
  }
}
