use gb::*;
use rom::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

pub trait Metric<R> {
  type ValueType: Eq + Hash + Debug;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType>;

  fn filter<F>(self, f: F) -> Filter<R, Self, F> where Self: Sized, F: Fn(&Self::ValueType) -> bool {
    Filter { metric: self, f: f, _rom: PhantomData, }
  }
}

pub struct Filter<R, M, F> {
  metric: M,
  f: F,
  _rom: PhantomData<R>,
}
impl<R, M: Metric<R>, F> Metric<R> for Filter<R, M, F> where F: Fn(&M::ValueType) -> bool {
  type ValueType = M::ValueType;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).filter(&self.f)
  }
}


pub struct FnMetric<F> {
  f: F,
}
impl<F> FnMetric<F> {
  pub fn new<R, V>(f: F) -> FnMetric<F> where F: Fn(&mut Gb<R>) -> Option<V> {
    FnMetric { f: f, }
  }
}
impl<R, F, V: Eq + Hash + Debug> Metric<R> for FnMetric<F> where F: Fn(&mut Gb<R>) -> Option<V> {
  type ValueType = V;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    (self.f)(gb)
  }
}


pub struct NullMetric {}
impl NullMetric {
  pub fn new() -> Self {
    Self {}
  }
}
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
      atk: atk,
      def: def,
      spd: spd,
      spc: spc,
    })
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
    return None;
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
