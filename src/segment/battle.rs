pub mod gen2;

use crate::gb::*;
use crate::rom::*;
use crate::segment::Metric;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum MoveOrder {
  PlayerFirst,
  EnemyFirst,
}
#[allow(dead_code)]
pub struct BattleMoveOrderMetric {}
impl<R: JoypadAddresses + BattleDetermineMoveOrderAddresses> Metric<R> for BattleMoveOrderMetric {
  type ValueType = MoveOrder;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    let hit = gb.run_until_or_next_input_use(&[R::MOVE_ORDER_PLAYER_FIRST_ADDRESS, R::MOVE_ORDER_ENEMY_FIRST_ADDRESS]);
    if hit == R::MOVE_ORDER_PLAYER_FIRST_ADDRESS { return Some(MoveOrder::PlayerFirst); }
    if hit == R::MOVE_ORDER_ENEMY_FIRST_ADDRESS { return Some(MoveOrder::EnemyFirst); }
    None
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum BattleObedience {
  Obey,
  Disobey,
}
#[allow(dead_code)]
pub struct BattleObedienceMetric {}
impl<R: JoypadAddresses + BattleObedienceAddresses> Metric<R> for BattleObedienceMetric {
  type ValueType = BattleObedience;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(&[R::CHECK_OBEDIENCE_START_ADDRESS]) == 0 { return None; }
    let hit = gb.run_until_or_next_input_use(&[R::CHECK_OBEDIENCE_OBEY_ADDRESS, R::CHECK_OBEDIENCE_DISOBEY_ADDRESS]);
    if hit == R::CHECK_OBEDIENCE_OBEY_ADDRESS { Some(BattleObedience::Obey) }
    else if hit == R::CHECK_OBEDIENCE_DISOBEY_ADDRESS { Some(BattleObedience::Disobey) }
    else { None }
  }
}
