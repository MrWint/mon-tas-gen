use crate::constants::*;
use crate::metric::*;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum FightTurnResult {
  OutOfPP,
  Hit { damage: u16, },
  CriticalHit { damage: u16, },
  HitWithoutEffect { damage: u16, },
  CriticalHitWithoutEffect { damage: u16, },
  Failed,
  Succeeded,
  DeductedPP { amount: u8, }
}
pub struct Gen2NormalHitMetric {
  expected_max_damage: u16,
  expected_max_crit_damage: u16,
  has_effect: bool,
}
impl Gen2NormalHitMetric {
  pub fn with_expected_max_damage(expected_max_damage: u16, expected_max_crit_damage: u16) -> Gen2NormalHitMetric {
    Gen2NormalHitMetric {
      expected_max_damage,
      expected_max_crit_damage,
      has_effect: false,
    }
  }
  pub fn with_effect(self) -> Gen2NormalHitMetric { Gen2NormalHitMetric { has_effect: true, ..self } }
}
impl<R: JoypadAddresses + Gen2FightTurnAddresses> Metric<R> for Gen2NormalHitMetric {
  type ValueType = FightTurnResult;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    // Commented out: Rollout does doturn before usedmovetext.
    // // Enter BattleCommand_DoTurn
    // if gb.step_until(&[R::BATTLE_COMMAND_DOTURN_ADDRESS]) == 0 { return None; }

    // // Check if user is out of PP
    // let hit = gb.step_until(&[R::NEXT_BATTLE_COMMAND_ADDRESS, R::OUT_OF_PP_ADDRESS]);
    // if hit == 0 { return None; }
    // if hit == R::OUT_OF_PP_ADDRESS { return Some(FightTurnResult::OutOfPP); }

    // Enter BattleCommand_DamageVariation
    if gb.step_until(&[R::BATTLE_COMMAND_DAMAGEVARIATION_ADDRESS]) == 0 { return None; }
    let max_damage = gb.gb().read_memory_word_be(R::CUR_DAMAGE_MEM_ADDRESS);

    // Enter BattleCommand_MoveAnimNoSub (used in moveanim)
    if gb.step_until(&[R::BATTLE_COMMAND_MOVEANIMNOSUB_ADDRESS]) == 0 { return None; }
    if gb.gb().read_memory(R::ATTACK_MISSED_MEM_ADDRESS) != 0 { return Some(FightTurnResult::Failed); }
    let damage = gb.gb().read_memory_word_be(R::CUR_DAMAGE_MEM_ADDRESS);
    let critical_hit = gb.gb().read_memory_word_be(R::CRITICAL_HIT_MEM_ADDRESS) != 0;
    let effect_failed = self.has_effect && gb.gb().read_memory(R::EFFECT_FAILED_MEM_ADDRESS) != 0;

    let expected_max_damage = if critical_hit { self.expected_max_crit_damage } else  { self.expected_max_damage };
    assert!(max_damage == expected_max_damage, "max damage {} doesn't match expected value {}", max_damage, expected_max_damage);

    if critical_hit && effect_failed { Some(FightTurnResult::CriticalHitWithoutEffect { damage, }) }
    else if critical_hit { Some(FightTurnResult::CriticalHit { damage, }) }
    else if effect_failed { Some(FightTurnResult::HitWithoutEffect { damage, }) }
    else { Some(FightTurnResult::Hit { damage, }) }
  }
}

pub struct Gen2AdditionalMultiHitMetric {}
impl Gen2AdditionalMultiHitMetric {
  pub fn new() -> Gen2AdditionalMultiHitMetric { Gen2AdditionalMultiHitMetric {} }
}
impl<R: JoypadAddresses + Gen2FightTurnAddresses> Metric<R> for Gen2AdditionalMultiHitMetric {
  type ValueType = FightTurnResult;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    // Enter BattleCommand_MoveAnimNoSub (used in moveanim)
    if gb.step_until(&[R::BATTLE_COMMAND_MOVEANIMNOSUB_ADDRESS]) == 0 { return None; }
    let damage = gb.gb().read_memory_word_be(R::CUR_DAMAGE_MEM_ADDRESS);
    let critical_hit = gb.gb().read_memory_word_be(R::CRITICAL_HIT_MEM_ADDRESS) != 0;

    if critical_hit { Some(FightTurnResult::CriticalHit { damage, }) }
    else { Some(FightTurnResult::Hit { damage, }) }
  }
}

pub struct Gen2SpiteMetric {}
impl Gen2SpiteMetric {
  pub fn new() -> Gen2SpiteMetric { Gen2SpiteMetric {} }
}
impl<R: JoypadAddresses + Gen2BattleSpiteAddresses> Metric<R> for Gen2SpiteMetric {
  type ValueType = FightTurnResult;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    let hit = gb.step_until(&[R::SPITE_SUCCESS_ADDRESS, R::SPITE_FAIL_ADDRESS]);
    if hit == 0 { None }
    else if hit == R::SPITE_FAIL_ADDRESS { Some(FightTurnResult::Failed) }
    else { Some(FightTurnResult::DeductedPP {amount: gb.gb().read_registers().b as u8 }) }
  }
}


pub struct Gen2MultiHitCountMetric {}
impl Gen2MultiHitCountMetric {
  pub fn new() -> Gen2MultiHitCountMetric { Gen2MultiHitCountMetric {} }
}
impl<R: JoypadAddresses + Gen2BattleMultiHitAddresses> Metric<R> for Gen2MultiHitCountMetric {
  type ValueType = u8;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if gb.step_until(&[R::MULTI_HIT_ADDRESS]) == 0 { return None; }
    Some(gb.gb().read_registers().a as u8 + 1)
  }
}

pub struct Gen2MoveSuccessMetric {}
impl<R: JoypadAddresses + Gen2FightTurnAddresses> Metric<R> for Gen2MoveSuccessMetric {
  type ValueType = FightTurnResult;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    // Commented out: Rollout does doturn before usedmovetext.
    // // Enter BattleCommand_DoTurn
    // if gb.step_until(&[R::BATTLE_COMMAND_DOTURN_ADDRESS]) == 0 { return None; }

    // // Check if user is out of PP
    // let hit = gb.step_until(&[R::NEXT_BATTLE_COMMAND_ADDRESS, R::OUT_OF_PP_ADDRESS]);
    // if hit == 0 { return None; }
    // if hit == R::OUT_OF_PP_ADDRESS { return Some(FightTurnResult::OutOfPP); }

    // Enter BattleCommand_LowerSub
    if gb.step_until(&[R::BATTLE_COMMAND_LOWERSUB_ADDRESS]) == 0 { return None; }
    if gb.gb().read_memory(R::ATTACK_MISSED_MEM_ADDRESS) != 0 { return Some(FightTurnResult::Failed); }
    Some(FightTurnResult::Succeeded)
  }
}

pub struct Gen2SwitchMonMetric {}
impl<R: Rom + Gen2BattleSwitchMonAddresses> Metric<R> for Gen2SwitchMonMetric {
  type ValueType = (Pokemon, u8);

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    // Run until switch is decided
    if gb.step_until(&[R::SWITCH_DECIDED_ADDRESS]) == 0 { return None; }

    let mon = Pokemon::from_index::<R>(gb.gb().read_memory(R::SWITCH_SPECIES_MEM_ADDRESS)).unwrap();
    let lvl = gb.gb().read_memory(R::SWITCH_LEVEL_MEM_ADDRESS);
    Some((mon, lvl))
  }
}

pub struct Gen2FullyParalyzedMetric {}
impl<R: Rom + Gen2FightTurnAddresses> Metric<R> for Gen2FullyParalyzedMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if gb.step_until(&[R::FULLY_PARALYZED_ADDRESS]) == 0 { return None; }
    Some(())
  }
}
