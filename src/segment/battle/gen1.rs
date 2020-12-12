use crate::rom::*;
use crate::segment::*;

mod endtrainerbattlesegment;
pub use self::endtrainerbattlesegment::EndTrainerBattleSegment;
mod fightturnsegment;
pub use self::fightturnsegment::EnemyAttack;
pub use self::fightturnsegment::EnemyAttackType;
pub use self::fightturnsegment::FightTurnSegment;
mod gen1itendtrainerbattlesegment;
pub use self::gen1itendtrainerbattlesegment::Gen1ITEndTrainerBattleSegment;
mod gen1itfightturnsegment;
pub use self::gen1itfightturnsegment::Gen1ITFightTurnSegment;
mod gen1itkosegment;
pub use self::gen1itkosegment::Gen1ITKOSegment;
mod gen1itnexttrainermonsegment;
pub use self::gen1itnexttrainermonsegment::Gen1ITNextTrainerMonSegment;
mod gen1itohkosegment;
pub use self::gen1itohkosegment::Gen1ITOHKOSegment;
mod gen1itstarttrainerbattlesegment;
pub use self::gen1itstarttrainerbattlesegment::Gen1ITStartTrainerBattleSegment;
mod kosegment;
pub use self::kosegment::KOSegment;
mod nexttrainermonsegment;
pub use self::nexttrainermonsegment::NextTrainerMonSegment;
mod ohkosegment;
pub use self::ohkosegment::OHKOSegment;
mod overridemovesegment;
pub use self::overridemovesegment::OverrideMoveSegment;
mod selectmovesegment;
pub use self::selectmovesegment::SelectMoveSegment;
mod starttrainerbattlesegment;
pub use self::starttrainerbattlesegment::StartTrainerBattleSegment;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum TrainerAIAction {
  NoAction,
  FullHeal,
  FullRestore,
  GuardSpec,
  Potion,
  Switch,
  XItem,
}
pub struct TrainerAIMetric {}
impl<R: JoypadAddresses + Gen1TrainerAIAddresses> Metric<R> for TrainerAIMetric {
  type ValueType = TrainerAIAction;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(&[R::TRAINER_AI_START_ADDRESS]) == 0 { return None; }
    let hit = gb.run_until_or_next_input_use(&[
      R::TRAINER_AI_NO_ACTION_ADDRESS,
      R::TRAINER_AI_FULL_HEAL_ADDRESS,
      R::TRAINER_AI_FULL_RESTORE_ADDRESS,
      R::TRAINER_AI_GUARD_SPEC_ADDRESS,
      R::TRAINER_AI_POTION_ADDRESS,
      R::TRAINER_AI_SWITCH_ADDRESS,
      R::TRAINER_AI_XITEM_ADDRESS]);
    if hit == R::TRAINER_AI_NO_ACTION_ADDRESS { Some(TrainerAIAction::NoAction) }
    else if hit == R::TRAINER_AI_FULL_HEAL_ADDRESS { Some(TrainerAIAction::FullHeal) }
    else if hit == R::TRAINER_AI_FULL_RESTORE_ADDRESS { Some(TrainerAIAction::FullRestore) }
    else if hit == R::TRAINER_AI_GUARD_SPEC_ADDRESS { Some(TrainerAIAction::GuardSpec) }
    else if hit == R::TRAINER_AI_POTION_ADDRESS { Some(TrainerAIAction::Potion) }
    else if hit == R::TRAINER_AI_SWITCH_ADDRESS { Some(TrainerAIAction::Switch) }
    else if hit == R::TRAINER_AI_XITEM_ADDRESS { Some(TrainerAIAction::XItem) }
    else { None }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveEffectResult {
  NoEffect,
  Sleep { turns: u8 },
  NothingHappened,
  Bide { turns: u8 },
  Thrash { turns: u8 },
  MultiHit { hits: u8 },
  Trapping { turns: u8 },
  Confused { turns: u8 },
  Success,
  Failed,
  Unknown,
}

pub struct MoveEffectMetric {}
impl<R: JoypadAddresses + Gen1MoveEffectAddresses> Metric<R> for MoveEffectMetric {
  type ValueType = MoveEffectResult;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(&[R::MOVE_EFFECT_START_ADDRESS]) == 0 { log::debug!("MOVE_EFFECT_START_ADDRESS not found"); return None; }
    let hit = gb.run_until_or_next_input_use(&[
      R::MOVE_EFFECT_NO_EFFECT_ADDRESS,
      R::MOVE_EFFECT_SLEEP_SUCCESS_ADDRESS,
      R::MOVE_EFFECT_SLEEP_FAILED_ADDRESS,
      R::MOVE_EFFECT_POISON_SUCCESS_ADDRESS,
      R::MOVE_EFFECT_POISON_FAILED_ADDRESS,
      R::MOVE_EFFECT_FREEZE_BURN_PARALYZE_PLAYER_SUCCESS_ADDRESS,
      R::MOVE_EFFECT_FREEZE_BURN_PARALYZE_ENEMY_SUCCESS_ADDRESS,
      R::MOVE_EFFECT_DEFROSTED_SUCCESS_ADDRESS,
      R::MOVE_EFFECT_STAT_UP_SUCCESS_ADDRESS,
      R::MOVE_EFFECT_STAT_UP_NOTHING_HAPPENED_ADDRESS,
      R::MOVE_EFFECT_STAT_DOWN_SUCCESS_ADDRESS,
      R::MOVE_EFFECT_STAT_DOWN_FAILED_ADDRESS,
      R::MOVE_EFFECT_STAT_DOWN_NOTHING_HAPPENED_ADDRESS,
      R::MOVE_EFFECT_BIDE_ADDRESS,
      R::MOVE_EFFECT_THRASH_PETALDANCE_ADDRESS,
      R::MOVE_EFFECT_MULTI_HIT_ADDRESS,
      R::MOVE_EFFECT_FLINCHED_ADDRESS,
      R::MOVE_EFFECT_TRAPPING_ADDRESS,
      R::MOVE_EFFECT_CONFUSION_SUCCESS_ADDRESS,
      R::MOVE_EFFECT_CONFUSION_FAILED_ADDRESS,
      R::MOVE_EFFECT_PARALYZE_SUCCESS_ADDRESS,
      R::MOVE_EFFECT_PARALYZE_FAILED_ADDRESS,
      R::MOVE_EFFECT_PARALYZE_NO_EFFECT_ADDRESS,
      R::MOVE_EFFECT_MIMIC_SUCCESS_ADDRESS,
      R::MOVE_EFFECT_MIMIC_FAILED_ADDRESS,
      R::MOVE_EFFECT_DISABLE_SUCCESS_ADDRESS,
      R::MOVE_EFFECT_DISABLE_FAILED_ADDRESS]);
    if hit == R::MOVE_EFFECT_NO_EFFECT_ADDRESS { Some(MoveEffectResult::NoEffect) }
    else if hit == R::MOVE_EFFECT_SLEEP_SUCCESS_ADDRESS { Some(MoveEffectResult::Sleep { turns: gb.gb.read_registers().a as u8 }) }
    else if hit == R::MOVE_EFFECT_SLEEP_FAILED_ADDRESS { Some(MoveEffectResult::Failed) }
    else if hit == R::MOVE_EFFECT_POISON_SUCCESS_ADDRESS { Some(MoveEffectResult::Success) }
    else if hit == R::MOVE_EFFECT_POISON_FAILED_ADDRESS { Some(MoveEffectResult::Failed) }
    else if hit == R::MOVE_EFFECT_FREEZE_BURN_PARALYZE_PLAYER_SUCCESS_ADDRESS { Some(MoveEffectResult::Success) }
    else if hit == R::MOVE_EFFECT_FREEZE_BURN_PARALYZE_ENEMY_SUCCESS_ADDRESS { Some(MoveEffectResult::Success) }
    else if hit == R::MOVE_EFFECT_DEFROSTED_SUCCESS_ADDRESS { Some(MoveEffectResult::Success) }
    else if hit == R::MOVE_EFFECT_STAT_UP_SUCCESS_ADDRESS { Some(MoveEffectResult::Success) }
    else if hit == R::MOVE_EFFECT_STAT_UP_NOTHING_HAPPENED_ADDRESS { Some(MoveEffectResult::NothingHappened) }
    else if hit == R::MOVE_EFFECT_STAT_DOWN_SUCCESS_ADDRESS { Some(MoveEffectResult::Success) }
    else if hit == R::MOVE_EFFECT_STAT_DOWN_FAILED_ADDRESS { Some(MoveEffectResult::Failed) }
    else if hit == R::MOVE_EFFECT_STAT_DOWN_NOTHING_HAPPENED_ADDRESS { Some(MoveEffectResult::NothingHappened) }
    else if hit == R::MOVE_EFFECT_BIDE_ADDRESS { Some(MoveEffectResult::Bide { turns: gb.gb.read_registers().a as u8 }) }
    else if hit == R::MOVE_EFFECT_THRASH_PETALDANCE_ADDRESS { Some(MoveEffectResult::Thrash { turns: gb.gb.read_registers().a as u8 }) }
    else if hit == R::MOVE_EFFECT_MULTI_HIT_ADDRESS { Some(MoveEffectResult::MultiHit { hits: gb.gb.read_registers().a as u8 }) }
    else if hit == R::MOVE_EFFECT_FLINCHED_ADDRESS { Some(MoveEffectResult::Success) }
    else if hit == R::MOVE_EFFECT_TRAPPING_ADDRESS { Some(MoveEffectResult::Trapping { turns: gb.gb.read_registers().a as u8 }) }
    else if hit == R::MOVE_EFFECT_CONFUSION_SUCCESS_ADDRESS { Some(MoveEffectResult::Confused { turns: gb.gb.read_registers().a as u8 }) }
    else if hit == R::MOVE_EFFECT_CONFUSION_FAILED_ADDRESS { Some(MoveEffectResult::Failed) }
    else if hit == R::MOVE_EFFECT_PARALYZE_SUCCESS_ADDRESS { Some(MoveEffectResult::Success) }
    else if hit == R::MOVE_EFFECT_PARALYZE_FAILED_ADDRESS { Some(MoveEffectResult::Failed) }
    else if hit == R::MOVE_EFFECT_PARALYZE_NO_EFFECT_ADDRESS { Some(MoveEffectResult::NothingHappened) }
    else if hit == R::MOVE_EFFECT_MIMIC_SUCCESS_ADDRESS { Some(MoveEffectResult::Success) }
    else if hit == R::MOVE_EFFECT_MIMIC_FAILED_ADDRESS { Some(MoveEffectResult::Failed) }
    else if hit == R::MOVE_EFFECT_DISABLE_SUCCESS_ADDRESS { Some(MoveEffectResult::Success) }
    else if hit == R::MOVE_EFFECT_DISABLE_FAILED_ADDRESS { Some(MoveEffectResult::Failed) }
    else { Some(MoveEffectResult::Unknown) }
  }
}


#[derive(Debug, Eq, Hash, PartialEq)]
pub enum FightTurnResult {
  Hit { damage: u16, },
  CriticalHit { damage: u16, },
  Failed,
  Succeeded,
}
pub struct Gen1NormalHitMetric {
  pub expected_max_damage: u16,
  pub expected_max_crit_damage: u16,
}
impl Gen1NormalHitMetric {
  pub fn with_expected_max_damage(expected_max_damage: u16, expected_max_crit_damage: u16) -> Gen1NormalHitMetric {
    Gen1NormalHitMetric {
      expected_max_damage,
      expected_max_crit_damage,
    }
  }
}
impl<R: JoypadAddresses + Gen1FightTurnAddresses> Metric<R> for Gen1NormalHitMetric {
  type ValueType = FightTurnResult;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(&[R::AFTER_MAX_DAMAGE_CALC_ADDRESS]) == 0 { log::debug!("AFTER_MAX_DAMAGE_CALC_ADDRESS not found"); return None; }
    let max_damage = gb.gb.read_memory_word_be(R::CUR_DAMAGE_MEM_ADDRESS);

    if gb.run_until_or_next_input_use(&[R::AFTER_PLAYER_HIT_CHECK_ADDRESS, R::AFTER_ENEMY_HIT_CHECK_ADDRESS]) == 0 { log::debug!("HIT_CHECK_ADDRESS not found"); return None; }
    if gb.gb.read_memory(R::ATTACK_MISSED_MEM_ADDRESS) != 0 { return Some(FightTurnResult::Failed); }
    let damage = gb.gb.read_memory_word_be(R::CUR_DAMAGE_MEM_ADDRESS);
    let critical_hit = gb.gb.read_memory_word_be(R::CRITICAL_HIT_MEM_ADDRESS) != 0;

    let expected_max_damage = if critical_hit { self.expected_max_crit_damage } else  { self.expected_max_damage };
    assert!(max_damage == expected_max_damage, "max damage {} doesn't match expected value {}", max_damage, expected_max_damage);

    if critical_hit { Some(FightTurnResult::CriticalHit { damage, }) }
    else { Some(FightTurnResult::Hit { damage, }) }
  }
}

pub struct Gen1MoveSuccessMetric {}
impl<R: JoypadAddresses + Gen1FightTurnAddresses> Metric<R> for Gen1MoveSuccessMetric {
  type ValueType = FightTurnResult;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(&[R::AFTER_PLAYER_HIT_CHECK_ADDRESS, R::AFTER_ENEMY_HIT_CHECK_ADDRESS]) == 0 { return None; }
    if gb.gb.read_memory(R::ATTACK_MISSED_MEM_ADDRESS) != 0 { return Some(FightTurnResult::Failed); }
    Some(FightTurnResult::Succeeded)
  }
}
