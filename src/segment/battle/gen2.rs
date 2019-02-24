use crate::constants::*;
use crate::gb::*;
use crate::rom::*;
use crate::segment::Metric;

pub struct Gen2AIChooseMoveMetric {}
impl<R: JoypadAddresses + Gen2AIChooseMoveAddresses> Metric<R> for Gen2AIChooseMoveMetric {
  type ValueType = Move;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(&[R::AFTER_AI_CHOOSE_MOVE_ADDRESS]) == 0 { return None; }
    Some(Move::from_index(gb.gb.read_memory(R::CUR_ENEMY_MOVE_MEM_ADDRESS)).unwrap())
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum FightTurnResult {
  OutOfPP,
  HitMissed,
  Hit { damage: u16, },
  CriticalHit { damage: u16, },
  Failed,
  StatUpDown,
}
pub struct Gen2NormalHitMetric {
  expected_max_damage: u16,
  expected_max_crit_damage: u16,
}
impl Gen2NormalHitMetric {
  pub fn with_expected_max_damage(expected_max_damage: u16, expected_max_crit_damage: u16) -> Gen2NormalHitMetric {
    Gen2NormalHitMetric {
      expected_max_damage,
      expected_max_crit_damage
    }
  }
}
impl<R: JoypadAddresses + Gen2FightTurnAddresses> Metric<R> for Gen2NormalHitMetric {
  type ValueType = FightTurnResult;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    // Enter BattleCommand_DoTurn
    if gb.run_until_or_next_input_use(&[R::BATTLE_COMMAND_DOTURN_ADDRESS]) == 0 { return None; }

    // Check if user is out of PP
    let hit = gb.run_until_or_next_input_use(&[R::NEXT_BATTLE_COMMAND_ADDRESS, R::OUT_OF_PP_ADDRESS]);
    if hit == 0 { return None; }
    if hit == R::OUT_OF_PP_ADDRESS { return Some(FightTurnResult::OutOfPP); }

    // Enter BattleCommand_DamageVariation
    if gb.run_until_or_next_input_use(&[R::BATTLE_COMMAND_DAMAGEVARIATION_ADDRESS]) == 0 { return None; }
    let max_damage = gb.gb.read_memory_word_be(R::CUR_DAMAGE_MEM_ADDRESS);

    // Enter BattleCommand_LowerSub (used in moveanim)
    if gb.run_until_or_next_input_use(&[R::BATTLE_COMMAND_LOWERSUB_ADDRESS]) == 0 { return None; }
    if gb.gb.read_memory(R::ATTACK_MISSED_MEM_ADDRESS) != 0 { return Some(FightTurnResult::HitMissed); }
    let damage = gb.gb.read_memory_word_be(R::CUR_DAMAGE_MEM_ADDRESS);
    let critical_hit = gb.gb.read_memory_word_be(R::CRITICAL_HIT_MEM_ADDRESS) != 0;

    assert!(max_damage == if critical_hit { self.expected_max_crit_damage } else  { self.expected_max_damage });

    if critical_hit { Some(FightTurnResult::CriticalHit { damage, }) }
    else { Some(FightTurnResult::Hit { damage, }) }
  }
}

pub struct Gen2StatUpDownMetric {}
impl<R: JoypadAddresses + Gen2FightTurnAddresses> Metric<R> for Gen2StatUpDownMetric {
  type ValueType = FightTurnResult;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    // Enter BattleCommand_DoTurn
    if gb.run_until_or_next_input_use(&[R::BATTLE_COMMAND_DOTURN_ADDRESS]) == 0 { return None; }

    // Check if user is out of PP
    let hit = gb.run_until_or_next_input_use(&[R::NEXT_BATTLE_COMMAND_ADDRESS, R::OUT_OF_PP_ADDRESS]);
    if hit == 0 { return None; }
    if hit == R::OUT_OF_PP_ADDRESS { return Some(FightTurnResult::OutOfPP); }

    // Enter BattleCommand_LowerSub
    if gb.run_until_or_next_input_use(&[R::BATTLE_COMMAND_LOWERSUB_ADDRESS]) == 0 { return None; }
    if gb.gb.read_memory(R::ATTACK_MISSED_MEM_ADDRESS) != 0 { return Some(FightTurnResult::Failed); }
    Some(FightTurnResult::StatUpDown)
  }
}
