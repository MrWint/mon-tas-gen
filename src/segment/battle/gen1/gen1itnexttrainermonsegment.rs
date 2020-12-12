use crate::rom::*;
use crate::segment::*;
use crate::segment::battle::gen1::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct Gen1ITNextTrainerMonSegment {
  level_up: bool,
  learn_move: bool,
  // skip_learning_move: bool,
  // override_move: Option<Move>,
  buffer_size: usize,
}
impl Gen1ITNextTrainerMonSegment {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self {
      level_up: false,
      learn_move: false,
      // skip_learning_move: false,
      // override_move: None,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn with_level_up(self) -> Self { Self { level_up: true, ..self } }
  pub fn with_learn_move(self) -> Self { Self { learn_move: true, ..self } }
  // pub fn with_skip_learning_move(self) -> Self { Self { skip_learning_move: true, ..self } }
  // pub fn with_override_move(self, mov: Move) -> Self { Self { override_move: Some(mov), ..self } }
}
impl WithOutputBufferSize for Gen1ITNextTrainerMonSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + JoypadLowSensitivityAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses> Segment<R> for Gen1ITNextTrainerMonSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let sb = Gen1ITSkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // enemy // mon // fainted
    let mut sb = Gen1ITSkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // gained // num XP
    if self.level_up {
      sb = Gen1ITSkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // grew to level // X
    }
    if self.learn_move {
      sb = Gen1ITSkipTextsSegment::new(1).with_skip_ends(3).with_confirm_input(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // learns // move // .
    }

    Some(((), sb)).into_iter().collect()
  }
}
