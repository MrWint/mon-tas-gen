use crate::constants::*;
use crate::metric::*;
use crate::metric::battle::*;
use crate::metric::battle::gen1::*;
use crate::rom::*;
use crate::segment::*;
use crate::segment::battle::gen1::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct OHKOSegment {
  mov: Move,
  buffer_size: usize,
}
impl OHKOSegment {
  #[allow(dead_code)]
  pub fn new(mov: Move) -> Self {
    Self {
      mov,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
}
impl WithOutputBufferSize for OHKOSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + Gen1FightTurnAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + TextAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + JoypadLowSensitivityAddresses> Segment<R> for OHKOSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let sb = SelectMoveSegment::new(self.mov).with_buffer_size(self.buffer_size).execute(gbe, sb); // Select desired move

    log::debug!("Player: {:?}", gbe.execute_state(&sb, MoveInfosFn::new(Who::Player)).get_value_assert_all_equal());
    log::debug!("Player: {:?}", gbe.execute_state(&sb, BattleMonInfoFn::new(Who::Player)).get_value_assert_all_equal());
    log::debug!("Enemy: {:?}", gbe.execute_state(&sb, MoveInfosFn::new(Who::Enemy)).get_value_assert_all_equal());
    log::debug!("Enemy: {:?}", gbe.execute_state(&sb, BattleMonInfoFn::new(Who::Enemy)).get_value_assert_all_equal());

    let move_infos = gbe.execute_state(&sb, MoveInfosFn::new(Who::Player)).get_value_assert_all_equal();
    let move_info = move_infos.into_iter().find(|move_info| move_info.mov == self.mov).expect("move not found");
    let player_mon_info = gbe.execute_state(&sb, BattleMonInfoFn::new(Who::Player)).get_value_assert_all_equal();
    let enemy_mon_info = gbe.execute_state(&sb, BattleMonInfoFn::new(Who::Enemy)).get_value_assert_all_equal();
    let enemy_mon_hp = enemy_mon_info.hp;
    let expect_crit = move_info.max_damage < enemy_mon_hp;
    let expected_max_damage = if expect_crit { move_info.max_crit_damage } else { move_info.max_damage };

    assert!(expected_max_damage >= enemy_mon_hp, "Move {:?} won't OHKO enemy {:?}", move_info, enemy_mon_info);
    assert!(player_mon_info.stats.spd >= enemy_mon_info.stats.spd, "Player {:?} slower than enemy {:?}", player_mon_info, enemy_mon_info);
    let mut sb = DelaySegment::new(
        MoveSegment::with_metric(Input::A,
            BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}).debug_print().expect(BattleObedience::Obey)).with_buffer_size(self.buffer_size)
        .seq(TextSegment::with_metric(
            Gen1NormalHitMetric::with_expected_max_damage(move_info.max_damage, move_info.max_crit_damage).debug_print().filter(|v| match v {
              FightTurnResult::Hit { damage } => !expect_crit && *damage >= enemy_mon_hp,
              FightTurnResult::CriticalHit { damage } =>  expect_crit && *damage >= enemy_mon_hp,
              _ => false,
            }).into_unit()).with_skip_ends(4).with_unbounded_buffer())
        ).with_buffer_size(self.buffer_size).execute(gbe, sb); // mon // used // move // !
    if expect_crit {
      sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // critical hit
    }
    if move_info.is_effective {
      sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb); // (not) very effective
    }

    Some(((), sb)).into_iter().collect()
  }
}
