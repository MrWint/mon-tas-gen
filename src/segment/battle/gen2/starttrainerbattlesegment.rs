use crate::constants::*;
use crate::metric::battle::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct StartTrainerBattleSegment {
  num_pre_battle_texts: usize,
  expected_move: Option<Move>,
  buffer_size: usize,
}
impl StartTrainerBattleSegment {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self {
      num_pre_battle_texts: 0,
      expected_move: None,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn with_pre_battle_texts(self, num_pre_battle_texts: usize) -> Self { Self { num_pre_battle_texts, ..self } }
  pub fn with_expected_move(self, mov: Move) -> Self { Self { expected_move: Some(mov), ..self } }
}
impl WithOutputBufferSize for StartTrainerBattleSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + TextAddresses + AIChooseMoveAddresses> Segment<R> for StartTrainerBattleSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, mut sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    if self.num_pre_battle_texts > 0 {
      sb = SkipTextsSegment::new(self.num_pre_battle_texts).with_buffer_size(self.buffer_size).execute(gbe, sb); // pre-battle texts
    }
    let sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // trainer wants to battle
    let sb = TextSegment::new().with_buffer_size(self.buffer_size).execute(gbe, sb); // trainer sent out
    let sb = DelaySegment::new(
        MoveSegment::new(Input::A | Input::B).with_buffer_size(self.buffer_size).seq(
        TextSegment::with_metric(ExpectedAIChooseMoveMetric { expected_move: self.expected_move }).with_unbounded_buffer().with_allowed_end_inputs(Input::B).with_skip_ends(5))
      ).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // ! // Go // mon // !

    Some(((), sb)).into_iter().collect()
  }
}
