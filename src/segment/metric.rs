use crate::gb::*;
use crate::gbexecutor::StateKey;
use crate::rom::*;
use std::marker::PhantomData;

pub trait Metric<R>: Sync {
  type ValueType: StateKey;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType>;

  fn filter<F>(self, f: F) -> Filter<R, Self, F> where Self: Sized, F: Fn(&Self::ValueType) -> bool {
    Filter { metric: self, f, _rom: PhantomData, }
  }
  fn map<F, K: StateKey>(self, f: F) -> Map<R, Self, F> where Self: Sized, F: Fn(Self::ValueType) -> K {
    Map { metric: self, f, _rom: PhantomData, }
  }
  fn into_unit(self) -> IntoUnit<R, Self> where Self: Sized {
    IntoUnit {metric: self, _rom: PhantomData, }
  }
}

pub struct Filter<R, M, F> {
  metric: M,
  f: F,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>, F: Sync> Metric<R> for Filter<R, M, F> where F: Fn(&M::ValueType) -> bool {
  type ValueType = M::ValueType;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).filter(&self.f)
  }
}
pub struct Map<R, M, F> {
  metric: M,
  f: F,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>, K: StateKey, F: Sync> Metric<R> for Map<R, M, F> where F: Fn(M::ValueType) -> K {
  type ValueType = K;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).map(&self.f)
  }
}
pub struct IntoUnit<R, M> {
  metric: M,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>> Metric<R> for IntoUnit<R, M> {
  type ValueType = ();

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).map(|_| ())
  }
}


pub struct FnMetric<F> {
  f: F,
}
impl<F> FnMetric<F> {
  pub fn new<R, V>(f: F) -> FnMetric<F> where F: Fn(&mut Gb<R>) -> Option<V> {
    FnMetric { f, }
  }
}
impl<R, F: Sync, V: StateKey> Metric<R> for FnMetric<F> where F: Fn(&mut Gb<R>) -> Option<V> {
  type ValueType = V;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    (self.f)(gb)
  }
}
impl<R, F: Sync, V: StateKey> Metric<R> for F where F: Fn(&mut Gb<R>) -> Option<V> {
  type ValueType = V;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self(gb)
  }
}


pub struct NullMetric {}
impl<R> Metric<R> for NullMetric {
  type ValueType = ();

  fn evaluate(&self, _gb: &mut Gb<R>) -> Option<Self::ValueType> {
    Some(())
  }
}


#[derive(Debug, Eq, Hash, PartialEq)]
pub struct DVs {
  pub atk: u8,
  pub def: u8,
  pub spd: u8,
  pub spc: u8,
  pub div_state: u16,
  pub cycle_count: u64,
}
#[allow(dead_code)]
pub struct Gen1DVMetric {}
impl<R: JoypadAddresses + Gen1DVAddresses> Metric<R> for Gen1DVMetric {
  type ValueType = DVs;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(R::AFTER_DV_GENERATION_ADDRESSES) == 0 { return None; }
    let registers = gb.gb.read_registers();

    let atk = ((registers.a >> 4) & 0xF) as u8;
    let def = ((registers.a) & 0xF) as u8;
    let spd = ((registers.b >> 4) & 0xF) as u8;
    let spc = ((registers.b) & 0xF) as u8;

    Some(DVs {
      atk,
      def,
      spd,
      spc,
      div_state: gb.gb.read_div_state(),
      cycle_count: gb.gb.get_cycle_count(),
    })
  }
}
#[allow(dead_code)]
pub struct Gen2DVMetric {}
impl<R: JoypadAddresses + Gen2DVAddresses> Metric<R> for Gen2DVMetric {
  type ValueType = DVs;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(&[R::AFTER_DV_GENERATION_ADDRESS]) == 0 { return None; }
    let registers = gb.gb.read_registers();

    let atk = ((registers.b >> 4) & 0xF) as u8;
    let def = ((registers.b) & 0xF) as u8;
    let spd = ((registers.c >> 4) & 0xF) as u8;
    let spc = ((registers.c) & 0xF) as u8;

    Some(DVs {
      atk,
      def,
      spd,
      spc,
      div_state: gb.gb.read_div_state(),
      cycle_count: gb.gb.get_cycle_count(),
    })
  }
}
#[allow(dead_code)]
pub struct TrainerIDMetric {}
impl<R: JoypadAddresses + TrainerIDAddresses> Metric<R> for TrainerIDMetric {
  type ValueType = u16;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(&[R::TRAINER_ID_AFTER_GENERATION_ADDRESS]) == 0 { return None; }
    Some(gb.gb.read_memory_word_be(R::TRAINER_ID_MEM_ADDRESS))
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum MoveOrder {
  PlayerFirst,
  EnemyFirst,
}
#[allow(dead_code)]
pub struct Gen2MoveOrderMetric {}
impl<R: JoypadAddresses + Gen2DetermineMoveOrderAddresses> Metric<R> for Gen2MoveOrderMetric {
  type ValueType = MoveOrder;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    let hit = gb.run_until_or_next_input_use(&[R::MOVE_ORDER_PLAYER_FIRST_ADDRESS, R::MOVE_ORDER_ENEMY_FIRST_ADDRESS]);
    if hit == R::MOVE_ORDER_PLAYER_FIRST_ADDRESS { return Some(MoveOrder::PlayerFirst); }
    if hit == R::MOVE_ORDER_ENEMY_FIRST_ADDRESS { return Some(MoveOrder::EnemyFirst); }
    None
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct AIChosenMove(u8);
#[allow(dead_code)]
pub struct Gen2AIChooseMoveMetric {}
impl<R: JoypadAddresses + Gen2AIChooseMoveAddresses> Metric<R> for Gen2AIChooseMoveMetric {
  type ValueType = AIChosenMove;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(&[R::AFTER_AI_CHOOSE_MOVE_ADDRESS]) == 0 { return None; }
    Some(AIChosenMove(gb.gb.read_memory(R::CUR_ENEMY_MOVE_MEM_ADDRESS)))
  }
}
