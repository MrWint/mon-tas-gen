use gambatte::Gambatte;

use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use crate::rom::*;

/// Interface used by metrics to interact with Gb instances.
pub trait GbI<R> {
  // Step until address or next input use.
  fn step_until(&mut self, addresses: &[i32]) -> i32;
  /// interact with immutable underlying Gambatte for memory inspection.
  fn gb(&self) -> &Gambatte;
}

pub trait MetricValueType: Eq + Hash + Debug + Send + 'static {}
impl<T: Eq + Hash + Debug + Send + 'static> MetricValueType for T {}

pub trait Metric<R>: Sync {
  type ValueType: MetricValueType;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType>;

  fn filter<F>(self, f: F) -> Filter<R, Self, F> where Self: Sized, F: Fn(&Self::ValueType) -> bool {
    Filter { metric: self, f, _rom: PhantomData, }
  }
  fn assert<F>(self, f: F) -> Assert<R, Self, F> where Self: Sized, F: Fn(&Self::ValueType) -> bool {
    Assert { metric: self, f, _rom: PhantomData, }
  }
  fn expect(self, expected_value: Self::ValueType) -> Expect<R, Self> where Self: Sized {
    Expect { metric: self, expected_value, _rom: PhantomData, }
  }
  fn assert_eq(self, expected_value: Self::ValueType) -> AssertEq<R, Self> where Self: Sized {
    AssertEq { metric: self, expected_value, _rom: PhantomData, }
  }
  fn map<F, K: MetricValueType>(self, f: F) -> Map<R, Self, F> where Self: Sized, F: Fn(Self::ValueType) -> K {
    Map { metric: self, f, _rom: PhantomData, }
  }
  fn and_then<M: Metric<R>>(self, then_metric: M) -> AndThen<R, Self, M> where Self: Sized {
    AndThen { metric: self, then_metric, _rom: PhantomData, }
  }
  fn and_then_split<M: Metric<R, ValueType=()>>(self, then_metric: M) -> AndThenSplit<R, Self, M> where Self: Sized {
    AndThenSplit { metric: self, then_metric, _rom: PhantomData, }
  }
  fn into_unit(self) -> IntoUnit<R, Self> where Self: Sized {
    IntoUnit {metric: self, _rom: PhantomData, }
  }
  fn debug_print(self) -> DebugPrint<R, Self> where Self: Sized {
    DebugPrint {metric: self, _rom: PhantomData, }
  }
}

pub struct Filter<R, M, F> {
  metric: M,
  f: F,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>, F: Sync> Metric<R> for Filter<R, M, F> where F: Fn(&M::ValueType) -> bool {
  type ValueType = M::ValueType;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).filter(&self.f)
  }
}
pub struct Assert<R, M, F> {
  metric: M,
  f: F,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>, F: Sync> Metric<R> for Assert<R, M, F> where F: Fn(&M::ValueType) -> bool {
  type ValueType = M::ValueType;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).map(|v| { assert!((self.f)(&v)); v })
  }
}
pub struct Map<R, M, F> {
  metric: M,
  f: F,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>, K: MetricValueType, F: Sync> Metric<R> for Map<R, M, F> where F: Fn(M::ValueType) -> K {
  type ValueType = K;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).map(&self.f)
  }
}
pub struct AndThen<R, M, TM> {
  metric: M,
  then_metric: TM,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>, TM: Metric<R>> Metric<R> for AndThen<R, M, TM> {
  type ValueType = TM::ValueType;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).and_then(|_| self.then_metric.evaluate(gb))
  }
}
pub struct AndThenSplit<R, M, TM> {
  metric: M,
  then_metric: TM,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>, TM: Metric<R, ValueType=()>> Metric<R> for AndThenSplit<R, M, TM> {
  type ValueType = M::ValueType;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).and_then(|v| self.then_metric.evaluate(gb).map(|_| v))
  }
}
pub struct IntoUnit<R, M> {
  metric: M,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>> Metric<R> for IntoUnit<R, M> {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).map(|_| ())
  }
}
pub struct Expect<R, M: Metric<R>> {
  metric: M,
  expected_value: M::ValueType,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>> Metric<R> for Expect<R, M> where M::ValueType: Sync {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).filter(|v| v == &self.expected_value).map(|_| ())
  }
}
pub struct AssertEq<R, M: Metric<R>> {
  metric: M,
  expected_value: M::ValueType,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>> Metric<R> for AssertEq<R, M> where M::ValueType: Sync {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).map(|v| assert_eq!(v, self.expected_value))
  }
}
pub struct DebugPrint<R, M> {
  metric: M,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>> Metric<R> for DebugPrint<R, M> {
  type ValueType = M::ValueType;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    let value = self.metric.evaluate(gb);
    log::debug!("{:?}", value);
    value
  }
}


pub struct FnMetric<F> {
  f: F,
}
impl<F> FnMetric<F> {
  pub fn new<R, V>(f: F) -> FnMetric<F> where F: Fn(&mut dyn GbI<R>) -> Option<V> {
    FnMetric { f, }
  }
}
impl<R, F: Sync, V: MetricValueType> Metric<R> for FnMetric<F> where F: Fn(&mut dyn GbI<R>) -> Option<V> {
  type ValueType = V;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    (self.f)(gb)
  }
}
impl<R, F: Sync, V: MetricValueType> Metric<R> for F where F: Fn(&mut dyn GbI<R>) -> Option<V> {
  type ValueType = V;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    self(gb)
  }
}


pub struct NullMetric;
impl<R> Metric<R> for NullMetric {
  type ValueType = ();

  fn evaluate(&self, _gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    Some(())
  }
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DVs {
  pub atk: u8,
  pub def: u8,
  pub spd: u8,
  pub spc: u8,
}
impl DVs {
  pub fn from_u16_be(dvs: u16) -> DVs {
    DVs {
      atk: ((dvs >> 12) & 0xF) as u8,
      def: ((dvs >> 8) & 0xF) as u8,
      spd: ((dvs >> 4) & 0xF) as u8,
      spc: ((dvs >> 0) & 0xF) as u8,
    }
  }
  pub fn hp(&self) -> u8 {
    (self.atk & 1) << 3 | (self.def & 1) << 2 | (self.spd & 1) << 1 | (self.spc & 1)
  }
}
#[allow(dead_code)]
pub struct Gen1DVMetric {}
impl<R: JoypadAddresses + Gen1DVAddresses> Metric<R> for Gen1DVMetric {
  type ValueType = DVs;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if gb.step_until(R::AFTER_DV_GENERATION_ADDRESSES) == 0 { return None; }
    let registers = gb.gb().read_registers();

    Some(DVs::from_u16_be((registers.a as u16) << 8 | (registers.b as u16)))
  }
}
#[allow(dead_code)]
pub struct Gen2DVMetric {}
impl<R: JoypadAddresses + Gen2DVAddresses> Metric<R> for Gen2DVMetric {
  type ValueType = DVs;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if gb.step_until(&[R::AFTER_DV_GENERATION_ADDRESS, R::AFTER_WILD_DV_GENERATION_ADDRESS]) == 0 { return None; }
    let registers = gb.gb().read_registers();

    Some(DVs::from_u16_be((registers.b as u16) << 8 | (registers.c as u16)))
  }
}
#[allow(dead_code)]
pub struct VermilionFirstTrashCanMetric {}
impl<R: JoypadAddresses + VermilionTrashCanAddresses> Metric<R> for VermilionFirstTrashCanMetric {
  type ValueType = u8;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if gb.step_until(&[R::AFTER_FIRST_TRASH_CAN_ADDRESS]) == 0 { return None; }
    Some(gb.gb().read_memory(R::FIRST_TRASH_CAN_MEM_ADDRESS))
  }
}
#[allow(dead_code)]
pub struct VermilionSecondTrashCanMetric {}
impl<R: JoypadAddresses + VermilionTrashCanAddresses> Metric<R> for VermilionSecondTrashCanMetric {
  type ValueType = u8;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if gb.step_until(&[R::AFTER_SECOND_TRASH_CAN_ADDRESS]) == 0 { return None; }
    Some(gb.gb().read_memory(R::SECOND_TRASH_CAN_MEM_ADDRESS))
  }
}
#[allow(dead_code)]
pub struct TrainerIDMetric {}
impl<R: JoypadAddresses + TrainerIDAddresses> Metric<R> for TrainerIDMetric {
  type ValueType = u16;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if gb.step_until(&[R::TRAINER_ID_AFTER_GENERATION_ADDRESS]) == 0 { return None; }
    Some(gb.gb().read_memory_word_be(R::TRAINER_ID_MEM_ADDRESS))
  }
}




pub trait StateFn<R> {
  type OV;

  fn invoke(&self, gb: &dyn GbI<R>) -> Self::OV;
}
impl<R, OV, F: Fn(&dyn GbI<R>) -> OV> StateFn<R> for F {
  type OV = OV;

  fn invoke(&self, gb: &dyn GbI<R>) -> OV { self(gb) }
}


pub mod battle;
pub mod overworld;
