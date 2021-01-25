use crate::constants::*;
use crate::metric::battle::*;
use crate::rom::*;
use crate::segment::*;
use crate::segment::battle::gen1::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct NextTrainerMonSegment {
  level_up: bool,
  learn_move: bool,
  skip_learning_move: bool,
  override_move: Option<Move>,
  buffer_size: usize,
}
impl NextTrainerMonSegment {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self {
      level_up: false,
      learn_move: false,
      skip_learning_move: false,
      override_move: None,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn with_level_up(self) -> Self { Self { level_up: true, ..self } }
  pub fn with_learn_move(self) -> Self { Self { learn_move: true, ..self } }
  pub fn with_skip_learning_move(self) -> Self { Self { skip_learning_move: true, ..self } }
  pub fn with_override_move(self, mov: Move) -> Self { Self { override_move: Some(mov), ..self } }
}
impl WithOutputBufferSize for NextTrainerMonSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + TextAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + InputIdentificationAddresses> Segment<R> for NextTrainerMonSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let sb = SkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // enemy // mon // fainted
    let mut sb = SkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // gained // num XP
    if self.level_up {
      sb = SkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // grew to level // X
    }
    if self.learn_move {
      sb = SkipTextsSegment::new(1).with_skip_ends(3).with_confirm_input(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // learns // move // .
    } else if let Some(mov) = self.override_move {
      let move_infos = gbe.execute_state(&sb, MoveInfosFn::new(Who::Player)).get_value_assert_all_equal();
      let move_index = move_infos.iter().position(|move_info| move_info.mov == mov).expect("move not found");

      sb = OverrideMoveSegment::new(move_index).with_buffer_size(self.buffer_size).execute(gbe, sb);
    } else if self.skip_learning_move {
      sb = OverrideMoveSegment::dont_learn().with_buffer_size(self.buffer_size).execute(gbe, sb);
    }
    let sb = TextSegment::new().with_skip_ends(3).with_buffer_size(self.buffer_size).with_allowed_end_inputs(Input::B).execute(gbe, sb); // trainer // sent out // mon // go // mon // !

    Some(((), sb)).into_iter().collect()
  }
}
