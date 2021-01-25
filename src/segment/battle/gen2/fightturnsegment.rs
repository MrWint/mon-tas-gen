use crate::constants::*;
use crate::metric::*;
use crate::metric::battle::*;
use crate::metric::battle::gen2::*;
use crate::rom::*;
use crate::segment::*;
use crate::segment::battle::gen2::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;


pub struct FightTurnSegment {
  player_move: Move,
  expect_crit: bool,
  has_effect: bool,
  berry_recovery: bool,
  burn_damage: bool,
  min_acceptable_damage: u16,
  damage_slack: u16,
  enemy_attack: EnemyAttack,
  expected_enemy_followup_move: Option<Move>,
  buffer_size: usize,
}
impl FightTurnSegment {
  #[allow(dead_code)]
  pub fn new(player_move: Move, expect_crit: bool, min_acceptable_damage: u16, enemy_attack: EnemyAttack) -> Self {
    Self {
      player_move,
      expect_crit,
      has_effect: false,
      berry_recovery: false,
      burn_damage: false,
      min_acceptable_damage,
      damage_slack: 0,
      enemy_attack,
      expected_enemy_followup_move: None,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn with_has_effect(self, has_effect: bool) -> Self { Self { has_effect, ..self } }
  pub fn with_expected_enemy_followup_move(self, expected_enemy_followup_move: Option<Move>) -> Self { Self { expected_enemy_followup_move, ..self } }
  pub fn with_berry_recovery(self, berry_recovery: bool) -> Self { Self { berry_recovery, ..self } }
  pub fn with_burn_damage(self, burn_damage: bool) -> Self { Self { burn_damage, ..self } }
}
impl WithOutputBufferSize for FightTurnSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + Gen2FightTurnAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + TextAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + AIChooseMoveAddresses + Gen2BattleSpiteAddresses> Segment<R> for FightTurnSegment {
  type Key = u16;

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let move_infos = gbe.execute_state(&sb, MoveInfosFn::new(Who::Player)).get_value_assert_all_equal();
    let move_info = move_infos.into_iter().find(|move_info| move_info.mov == self.player_move).expect("move not found");
    assert!(move_info.max_damage > 0, "selected move does not do any damage");
    assert!(move_info.max_crit_damage >= move_info.max_damage);
    let max_damage = if self.expect_crit { move_info.max_crit_damage } else { move_info.max_damage };
    assert!(max_damage >= self.min_acceptable_damage, "min damage {} higher than max damage {}", self.min_acceptable_damage, max_damage);

    let enemy_move_infos = gbe.execute_state(&sb, MoveInfosFn::new(Who::Enemy)).get_value_assert_all_equal();
    let enemy_move_info = enemy_move_infos.into_iter().find(|move_info| move_info.mov == self.enemy_attack.mov).expect("enemy move not found");

    let player_mon_info = gbe.execute_state(&sb, BattleMonInfoFn::new(Who::Player)).get_value_assert_all_equal();
    let enemy_mon_info = gbe.execute_state(&sb, BattleMonInfoFn::new(Who::Enemy)).get_value_assert_all_equal();

    let burn_damage = if self.burn_damage { (enemy_mon_info.max_hp + 4 ) / 8} else { 0 };
    let is_ko = max_damage >= enemy_mon_info.hp;
    let is_burn_ko = max_damage + burn_damage >= enemy_mon_info.hp;
    assert!(!is_ko || self.min_acceptable_damage == enemy_mon_info.hp);
    let primary_damage = if is_ko { enemy_mon_info.hp } else { max_damage - self.damage_slack };

    let player_moves_first = player_mon_info.stats.spd >= enemy_mon_info.stats.spd;

    let player_after_attack_texts_segment = if !self.expect_crit && !move_info.is_effective && (player_moves_first || !self.berry_recovery) { None } else {
      Some(PlayerAfterAttackTextsSegment {
        crit: self.expect_crit,
        effective: move_info.is_effective,
        buffer_size: self.buffer_size,
      })};
    let player_has_after_attack_texts = player_after_attack_texts_segment.is_some();
    let enemy_after_attack_texts_segment = self.enemy_attack.attack_type.get_after_attack_texts_segment(enemy_move_info.is_effective, self.burn_damage, self.buffer_size);
    let enemy_has_after_attack_texts = enemy_after_attack_texts_segment.is_some();
    let enemy_expected_next_move_metric = EnemyExpectedNextMoveMetric { is_ko: is_burn_ko, expected_move: self.expected_enemy_followup_move };

    let mut player_attack_metric = Gen2NormalHitMetric::with_expected_max_damage(move_info.max_damage, move_info.max_crit_damage);
    if self.has_effect { player_attack_metric = player_attack_metric.with_effect(); }
    let player_attack_metric = player_attack_metric
      .map(|v| match v {
        FightTurnResult::Hit { damage }                      => if !self.has_effect && !self.expect_crit { damage } else { 0 },
        FightTurnResult::HitWithoutEffect { damage }         => if  self.has_effect && !self.expect_crit { damage } else { 0 },
        FightTurnResult::CriticalHit { damage }              => if !self.has_effect &&  self.expect_crit { damage } else { 0 },
        FightTurnResult::CriticalHitWithoutEffect { damage } => if  self.has_effect &&  self.expect_crit { damage } else { 0 },
        _ => 0 }) // filter unwanted result types
      .map(|damage| std::cmp::min(damage, enemy_mon_info.hp)) // cap damage at enemy hp
      .filter(|&damage| damage >= self.min_acceptable_damage) // filter damage that's too low
      .and_then_split(FnMetric::new(|gb| if !player_moves_first && !player_has_after_attack_texts { enemy_expected_next_move_metric.evaluate(gb) } else { Some(()) })); // check next enemy move if necessary
    let player_attack_segment = TextSegment::with_metric(player_attack_metric).with_skip_ends(3).with_unbounded_buffer(); // mon used move
    let enemy_attack_metric = EnemyAttackMetric::new(self.enemy_attack.attack_type, enemy_move_info.max_damage, enemy_move_info.max_crit_damage)
      .and_then_split(FnMetric::new(|gb| if player_moves_first && !enemy_has_after_attack_texts && !self.berry_recovery { enemy_expected_next_move_metric.evaluate(gb) } else { Some(()) })); // check next enemy move if necessary
    let enemy_attack_segment = TextSegment::with_metric(enemy_attack_metric).with_skip_ends(3).with_unbounded_buffer(); // enemy mon used move

    let sb = SelectMoveSegment::new(self.player_move).with_buffer_size(self.buffer_size).execute(gbe, sb); // Select desired move

    if player_moves_first && is_ko {
      log::info!("FightTurnSegment: player moves first, ko");
      let mut sb_map = DelaySegment::new(
        MoveSegment::with_metric(Input::A,
          BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey))).with_buffer_size(self.buffer_size)
        .seq(player_attack_segment)
      ).with_primary_key(primary_damage).with_buffer_size(self.buffer_size).execute_split(gbe, sb);
      if let Some(player_after_attack_texts_segment) = player_after_attack_texts_segment {
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, player_after_attack_texts_segment.execute(gbe, sb))).collect();
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, MoveSegment::new(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
      }
      sb_map
    } else if player_moves_first {
      let mut sb_map = if let Some(player_after_attack_texts_segment) = player_after_attack_texts_segment {
        log::info!("FightTurnSegment: player moves first, separate moves");
        DelaySegment::new(
          MoveSegment::with_metric(Input::A,
            BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey))).with_buffer_size(self.buffer_size)
          .seq(player_attack_segment)
        ).with_primary_key(primary_damage).with_buffer_size(self.buffer_size)
        .seq_split(player_after_attack_texts_segment)
        .seq_split(DelaySegment::new(
          MoveSegment::with_metric(Input::A, BattleObedienceMetric {}.expect(BattleObedience::Obey)).with_buffer_size(self.buffer_size)
          .seq(enemy_attack_segment)
        ).with_buffer_size(self.buffer_size)).execute_split(gbe, sb)
      } else {
        log::info!("FightTurnSegment: player moves first, combined moves");
        DelaySegment::new(
          MoveSegment::with_metric(Input::A,
            BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey))).with_buffer_size(self.buffer_size)
          .seq(player_attack_segment)
          .seq_split(enemy_attack_segment.with_allowed_end_inputs(Input::B))
        ).with_primary_key(primary_damage).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
      };
      if let Some(enemy_after_attack_texts_segment) = enemy_after_attack_texts_segment {
        let enemy_after_attack_texts_segment = if self.berry_recovery { enemy_after_attack_texts_segment } else { enemy_after_attack_texts_segment.with_unbounded_buffer() };
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, enemy_after_attack_texts_segment.execute(gbe, sb))).collect();
        if self.berry_recovery {
          sb_map = sb_map.into_iter().map(|(k, sb)| (k, MoveSegment::new(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
          sb_map = sb_map.into_iter().map(|(k, sb)| (k, SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
          sb_map = sb_map.into_iter().map(|(k, sb)| (k, TextSegment::new().with_skip_ends(1).with_allowed_end_inputs(Input::A).with_unbounded_buffer().execute(gbe, sb))).collect();
        }
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, DelaySegment::new(MoveSegment::with_metric(Input::B, EnemyExpectedNextMoveMetric { is_ko, expected_move: self.expected_enemy_followup_move }).with_buffer_size(self.buffer_size)).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
      } else if self.berry_recovery {
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, TextSegment::new().with_skip_ends(1).with_allowed_end_inputs(Input::A).with_unbounded_buffer().execute(gbe, sb))).collect();
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, DelaySegment::new(MoveSegment::with_metric(Input::B, EnemyExpectedNextMoveMetric { is_ko, expected_move: self.expected_enemy_followup_move }).with_buffer_size(self.buffer_size)).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
      }
      sb_map
    } else if burn_damage >= enemy_mon_info.hp {
      log::info!("FightTurnSegment: enemy moves first, burn ko");
      let mut sb_map: HashMap<u16, StateBuffer> = DelaySegment::new(
        MoveSegment::with_metric(Input::A,
          BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey))).with_buffer_size(self.buffer_size)
        .seq(enemy_attack_segment)
      ).with_buffer_size(self.buffer_size).execute_split(gbe, sb).into_iter().map(|(_, sb)| (0, sb)).collect();
      if let Some(enemy_after_attack_texts_segment) = enemy_after_attack_texts_segment {
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, enemy_after_attack_texts_segment.execute(gbe, sb))).collect();
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, MoveSegment::new(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
      }
      sb_map
    } else {
      let mut sb_map = if let Some(enemy_after_attack_texts_segment) = enemy_after_attack_texts_segment {
        log::info!("FightTurnSegment: enemy moves first, separate moves");
        DelaySegment::new(
          MoveSegment::with_metric(Input::A,
            BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey))).with_buffer_size(self.buffer_size)
          .seq(enemy_attack_segment)
        ).with_buffer_size(self.buffer_size)
        .seq(enemy_after_attack_texts_segment)
        .seq(DelaySegment::new(
          MoveSegment::with_metric(Input::A, BattleObedienceMetric {}.expect(BattleObedience::Obey)).with_buffer_size(self.buffer_size)
          .seq(player_attack_segment)
        ).with_primary_key(primary_damage).with_buffer_size(self.buffer_size)).execute_split(gbe, sb)
      } else {
        log::info!("FightTurnSegment: enemy moves first, combined moves");
        DelaySegment::new(
          MoveSegment::with_metric(Input::A,
            BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey))).with_buffer_size(self.buffer_size)
          .seq(enemy_attack_segment)
          .seq(player_attack_segment.with_allowed_end_inputs(Input::B))
        ).with_primary_key(primary_damage).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
      };
      if let Some(player_after_attack_texts_segment) = player_after_attack_texts_segment {
        let player_after_attack_texts_segment = if self.berry_recovery { player_after_attack_texts_segment } else { player_after_attack_texts_segment.with_unbounded_buffer() };
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, player_after_attack_texts_segment.execute(gbe, sb))).collect();
        if self.berry_recovery {
          sb_map = sb_map.into_iter().map(|(k, sb)| (k, MoveSegment::new(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
          sb_map = sb_map.into_iter().map(|(k, sb)| (k, SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
          sb_map = sb_map.into_iter().map(|(k, sb)| (k, TextSegment::new().with_skip_ends(1).with_allowed_end_inputs(Input::A).with_unbounded_buffer().execute(gbe, sb))).collect();
        }
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, DelaySegment::new(MoveSegment::with_metric(Input::B, EnemyExpectedNextMoveMetric { is_ko, expected_move: self.expected_enemy_followup_move }).with_buffer_size(self.buffer_size)).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
      } else if self.berry_recovery {
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, TextSegment::new().with_skip_ends(1).with_allowed_end_inputs(Input::A).with_unbounded_buffer().execute(gbe, sb))).collect();
        sb_map = sb_map.into_iter().map(|(k, sb)| (k, DelaySegment::new(MoveSegment::with_metric(Input::B, EnemyExpectedNextMoveMetric { is_ko, expected_move: self.expected_enemy_followup_move }).with_buffer_size(self.buffer_size)).with_buffer_size(self.buffer_size).execute(gbe, sb))).collect();
      }
      sb_map
    }
  }
}



struct PlayerAfterAttackTextsSegment {
  crit: bool,
  effective: bool,
  buffer_size: usize,
}
impl WithOutputBufferSize for PlayerAfterAttackTextsSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl<R: Rom + TextAddresses> Segment<R> for PlayerAfterAttackTextsSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, mut sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {

    assert!(self.crit || self.effective);
    if self.crit && self.effective {
      sb = SkipTextsSegment::new(1).with_buffer_size(self.buffer_size).execute(gbe, sb);
    }
    TextSegment::new().with_allowed_end_inputs(Input::A).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnemyAttack {
  pub mov: Move,
  pub attack_type: EnemyAttackType,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnemyAttackType {
  Hit,
  HitWithoutEffect,
  Failed,
  HitFailed,
  StatUpDown,
  StatWayUpDown,
  Poisoned,
  Spite,
}
impl EnemyAttackType {
  fn get_after_attack_texts_segment(&self, is_effective: bool, burn_damage: bool, buffer_size: usize) -> Option<EnemyAfterAttackTextsSegment> {
    match self {
      EnemyAttackType::Hit => if is_effective || burn_damage { Some(EnemyAfterAttackTextsSegment { attack_type: *self, is_effective, burn_damage, buffer_size, }) } else { None },
      EnemyAttackType::HitWithoutEffect => if is_effective || burn_damage { Some(EnemyAfterAttackTextsSegment { attack_type: *self, is_effective, burn_damage, buffer_size, }) } else { None },
      EnemyAttackType::Failed => Some(EnemyAfterAttackTextsSegment { attack_type: *self, is_effective, burn_damage, buffer_size, }),
      EnemyAttackType::HitFailed => Some(EnemyAfterAttackTextsSegment { attack_type: *self, is_effective, burn_damage, buffer_size, }),
      EnemyAttackType::StatUpDown => Some(EnemyAfterAttackTextsSegment { attack_type: *self, is_effective, burn_damage, buffer_size, }),
      EnemyAttackType::StatWayUpDown => Some(EnemyAfterAttackTextsSegment { attack_type: *self, is_effective, burn_damage, buffer_size, }),
      EnemyAttackType::Poisoned => Some(EnemyAfterAttackTextsSegment { attack_type: *self, is_effective, burn_damage, buffer_size, }),
      EnemyAttackType::Spite => Some(EnemyAfterAttackTextsSegment { attack_type: *self, is_effective, burn_damage, buffer_size, }),
    }
  }
}
struct EnemyAttackMetric {
  enemy_attack_type: EnemyAttackType,
  expected_max_damage: u16,
  expected_max_crit_damage: u16,
}
impl EnemyAttackMetric {
  fn new(enemy_attack_type: EnemyAttackType, expected_max_damage: u16, expected_max_crit_damage: u16) -> Self {
    EnemyAttackMetric {
      enemy_attack_type,
      expected_max_damage,
      expected_max_crit_damage,
    }
  }
}
impl<R: Rom + Gen2FightTurnAddresses + Gen2BattleSpiteAddresses> Metric<R> for EnemyAttackMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<()> {
    match self.enemy_attack_type {
      EnemyAttackType::Hit => Gen2NormalHitMetric::with_expected_max_damage(self.expected_max_damage, self.expected_max_crit_damage).expect(FightTurnResult::Hit { damage: (u32::from(self.expected_max_damage) * 217 / 255) as u16, }).evaluate(gb),
      EnemyAttackType::HitWithoutEffect => Gen2NormalHitMetric::with_expected_max_damage(self.expected_max_damage, self.expected_max_crit_damage).with_effect().expect(FightTurnResult::HitWithoutEffect { damage: (u32::from(self.expected_max_damage) * 217 / 255) as u16, }).evaluate(gb),
      EnemyAttackType::Failed => Gen2MoveSuccessMetric {}.expect(FightTurnResult::Failed).evaluate(gb),
      EnemyAttackType::HitFailed => Gen2NormalHitMetric::with_expected_max_damage(self.expected_max_damage, self.expected_max_crit_damage).expect(FightTurnResult::Failed).evaluate(gb),
      EnemyAttackType::StatUpDown => Gen2MoveSuccessMetric {}.expect(FightTurnResult::Succeeded).evaluate(gb),
      EnemyAttackType::StatWayUpDown => Gen2MoveSuccessMetric {}.expect(FightTurnResult::Succeeded).evaluate(gb),
      EnemyAttackType::Poisoned => Gen2MoveSuccessMetric {}.expect(FightTurnResult::Succeeded).evaluate(gb),
      EnemyAttackType::Spite => Gen2SpiteMetric {}.expect(FightTurnResult::DeductedPP { amount: 2 }).evaluate(gb),
    }
  }
}
struct EnemyAfterAttackTextsSegment {
  attack_type: EnemyAttackType,
  is_effective: bool,
  burn_damage: bool,
  buffer_size: usize,
}
impl WithOutputBufferSize for EnemyAfterAttackTextsSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl<R: Rom + TextAddresses> Segment<R> for EnemyAfterAttackTextsSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, mut sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let executed_text = match self.attack_type {
      EnemyAttackType::Hit | EnemyAttackType::HitWithoutEffect => {
        if self.is_effective {
          sb = TextSegment::new().with_allowed_end_inputs(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb);
          true
        } else { false }
      },
      EnemyAttackType::Failed | EnemyAttackType::HitFailed | EnemyAttackType::Poisoned => {
        sb = TextSegment::new().with_allowed_end_inputs(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb); true
      },
      EnemyAttackType::StatUpDown => {
        sb = TextSegment::new().with_allowed_end_inputs(Input::A).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb); true
      },
      EnemyAttackType::StatWayUpDown => {
        sb = SkipTextsSegment::new(1).with_skip_ends(1).with_buffer_size(self.buffer_size).execute(gbe, sb);
        sb = TextSegment::new().with_allowed_end_inputs(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb); true
      },
      EnemyAttackType::Spite => {
        sb = SkipTextsSegment::new(1).with_skip_ends(2).with_buffer_size(self.buffer_size).execute(gbe, sb);
        sb = TextSegment::new().with_skip_ends(1).with_allowed_end_inputs(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb); true
      },
    };
    if self.burn_damage {
      if executed_text { sb = MoveSegment::new(Input::B).with_buffer_size(self.buffer_size).execute(gbe, sb); } // confirm
      sb = TextSegment::new().with_allowed_end_inputs(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb); // hurt by burn
    }
    Some(((), sb)).into_iter().collect()
  }
}
struct EnemyExpectedNextMoveMetric {
  is_ko: bool,
  expected_move: Option<Move>,
}
impl<R: Rom + AIChooseMoveAddresses> Metric<R> for EnemyExpectedNextMoveMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<()> {
    if self.is_ko { Some(()) } else { ExpectedAIChooseMoveMetric { expected_move: self.expected_move }.evaluate(gb) }
  }
}
