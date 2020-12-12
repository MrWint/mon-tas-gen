use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::*;

pub struct Gen1ITEndTrainerBattleSegment {
  num_defeat_texts: usize,
  name_in_defeat_texts: bool,
  level_up: bool,
  learn_move: bool,
  buffer_size: usize,
}
impl Gen1ITEndTrainerBattleSegment {
  #[allow(dead_code)]
  pub fn with_defeat_texts(num_defeat_texts: usize) -> Self {
    Self {
      num_defeat_texts,
      name_in_defeat_texts: false,
      level_up: false,
      learn_move: false,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn with_level_up(self) -> Self { Self { level_up: true, ..self } }
  pub fn with_name_in_defeat_texts(self) -> Self { Self { name_in_defeat_texts: true, ..self } }
  pub fn with_learn_move(self) -> Self { Self { learn_move: true, ..self } }
}
impl WithOutputBufferSize for Gen1ITEndTrainerBattleSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + JoypadLowSensitivityAddresses> Segment<R> for Gen1ITEndTrainerBattleSegment {
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
    let sb = Gen1ITSkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // I defeated U
    let sb = if self.name_in_defeat_texts && self.num_defeat_texts == 1 {
      Gen1ITSkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb) // defeat texts
    } else if self.name_in_defeat_texts {
      let sb = Gen1ITSkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // defeat texts
      Gen1ITSkipTextsSegment::new(self.num_defeat_texts - 1).with_buffer_size(self.buffer_size).execute(gbe, sb) // defeat texts
    } else { 
      Gen1ITSkipTextsSegment::new(self.num_defeat_texts).with_buffer_size(self.buffer_size).execute(gbe, sb) // defeat texts
    };
    let sb = Gen1ITSkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // player got // X for winning

    Some(((), sb)).into_iter().collect()
  }
}
