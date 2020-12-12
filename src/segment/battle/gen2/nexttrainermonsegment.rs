use crate::rom::*;
use crate::segment::*;
use crate::segment::battle::*;
use crate::segment::battle::gen2::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct NextTrainerMonSegment {
  expected_pokemon: Pokemon,
  expected_level: u8,
  expected_move: Option<Move>,
  level_up: bool,
  learn_move: bool,
  skip_learning_move: bool,
  override_move: Option<Move>,
  buffer_size: usize,
}
impl NextTrainerMonSegment {
  #[allow(dead_code)]
  pub fn new(expected_pokemon: Pokemon, expected_level: u8) -> Self {
    Self {
      expected_pokemon,
      expected_level,
      expected_move: None,
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
  pub fn with_expected_move(self, mov: Move) -> Self { Self { expected_move: Some(mov), ..self } }
}
impl WithOutputBufferSize for NextTrainerMonSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + TextAddresses + Gen2BattleSwitchMonAddresses + AIChooseMoveAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses> Segment<R> for NextTrainerMonSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let sb = SkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // enemy // mon // fainted
    let mut sb = TextSegment::new().with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // gained // num XP
    if self.level_up {
      sb = MoveSegment::new(Input::A | Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); // confirm
      sb = TextSegment::new().with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // grew to level // X
    }
    if self.learn_move {
      sb = MoveSegment::new(Input::A | Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); // confirm
      sb = TextSegment::new().with_skip_ends(3).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // learns // move // .
    } else if let Some(mov) = self.override_move {
      sb = MoveSegment::new(Input::A | Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); // confirm

      let move_infos = gbe.execute_state(&sb, MoveInfosFn::new(Who::Player)).get_value_assert_all_equal();
      let move_index = move_infos.iter().position(|move_info| move_info.mov == mov).expect("move not found");

      sb = OverrideMoveSegment::new(move_index).with_expected_next_mon(self.expected_pokemon, self.expected_level).with_buffer_size(self.buffer_size).execute(gbe, sb);
    } else if self.skip_learning_move {
      sb = MoveSegment::new(Input::A | Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); // confirm
      sb = OverrideMoveSegment::dont_learn().with_expected_next_mon(self.expected_pokemon, self.expected_level).with_buffer_size(self.buffer_size).execute(gbe, sb);
    } else {
      sb = DelaySegment::new(MoveSegment::with_metric(Input::A | Input::B, Gen2SwitchMonMetric {}.debug_print().expect((self.expected_pokemon, self.expected_level)))).with_buffer_size(self.buffer_size).execute(gbe, sb);
    }
    let sb = TextSegment::new().with_buffer_size(self.buffer_size).execute(gbe, sb); // sent out
    let sb = DelaySegment::new(
        MoveSegment::new(Input::A | Input::B).with_buffer_size(self.buffer_size).seq(
        TextSegment::with_metric(ExpectedAIChooseMoveMetric { expected_move: self.expected_move }).with_allowed_end_inputs(Input::B).with_skip_ends(1).with_unbounded_buffer()) // mon
      ).with_buffer_size(self.buffer_size).execute(gbe, sb);

    Some(((), sb)).into_iter().collect()
  }
}
