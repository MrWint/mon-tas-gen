mod joypad;
pub use joypad::*;

use std::{fmt::Debug, hash::Hash};

use super::*;

pub trait MetricValueType: Eq + Hash + Debug + 'static {}
impl<T: Eq + Hash + Debug + 'static> MetricValueType for T {}

pub trait Metric<R: Rom>: Sync {
  type ValueType: MetricValueType;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType>;

  fn filter<F>(self, f: F) -> Filter<Self, F> where Self: Sized, F: Fn(&Self::ValueType) -> bool {
    Filter { metric: self, f, }
  }
  fn assert<F>(self, f: F) -> Assert<Self, F> where Self: Sized, F: Fn(&Self::ValueType) -> bool {
    Assert { metric: self, f, }
  }
  fn expect(self, expected_value: Self::ValueType) -> Expect<R, Self> where Self: Sized {
    Expect { metric: self, expected_value, }
  }
  fn assert_eq(self, expected_value: Self::ValueType) -> AssertEq<R, Self> where Self: Sized {
    AssertEq { metric: self, expected_value, }
  }
  fn map<F, K>(self, f: F) -> Map<Self, F> where Self: Sized, F: Fn(Self::ValueType) -> K {
    Map { metric: self, f, }
  }
  fn and_then<M: Metric<R>>(self, then_metric: M) -> AndThen<Self, M> where Self: Sized {
    AndThen { metric: self, then_metric, }
  }
  fn and_then_split<M: Metric<R, ValueType=()>>(self, then_metric: M) -> AndThenSplit<Self, M> where Self: Sized {
    AndThenSplit { metric: self, then_metric, }
  }
  fn into_unit(self) -> IntoUnit<Self> where Self: Sized {
    IntoUnit {metric: self, }
  }
  fn debug_print(self) -> DebugPrint<Self> where Self: Sized {
    DebugPrint {metric: self, }
  }
}

pub struct Filter<M, F> {
  metric: M,
  f: F,
}
impl<R: Rom, M: Metric<R>, F: Sync> Metric<R> for Filter<M, F> where F: Fn(&M::ValueType) -> bool {
  type ValueType = M::ValueType;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).filter(&self.f)
  }
}
pub struct Assert<M, F> {
  metric: M,
  f: F,
}
impl<R: Rom, M: Metric<R>, F: Sync> Metric<R> for Assert<M, F> where F: Fn(&M::ValueType) -> bool {
  type ValueType = M::ValueType;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).map(|v| { assert!((self.f)(&v)); v })
  }
}
pub struct Map<M, F> {
  metric: M,
  f: F,
}
impl<R: Rom, M: Metric<R>, K: MetricValueType, F: Sync> Metric<R> for Map<M, F> where F: Fn(M::ValueType) -> K {
  type ValueType = K;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).map(&self.f)
  }
}
pub struct AndThen<M, TM> {
  metric: M,
  then_metric: TM,
}
impl<R: Rom, M: Metric<R>, TM: Metric<R>> Metric<R> for AndThen<M, TM> {
  type ValueType = TM::ValueType;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).and_then(|_| self.then_metric.evaluate(gb))
  }
}
pub struct AndThenSplit<M, TM> {
  metric: M,
  then_metric: TM,
}
impl<R: Rom, M: Metric<R>, TM: Metric<R, ValueType=()>> Metric<R> for AndThenSplit<M, TM> {
  type ValueType = M::ValueType;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).and_then(|v| self.then_metric.evaluate(gb).map(|_| v))
  }
}
pub struct IntoUnit<M> {
  metric: M,
}
impl<R: Rom, M: Metric<R>> Metric<R> for IntoUnit<M> {
  type ValueType = ();

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).map(|_| ())
  }
}
pub struct Expect<R: Rom, M: Metric<R>> {
  metric: M,
  expected_value: M::ValueType,
}
impl<R: Rom, M: Metric<R>> Metric<R> for Expect<R, M> where M::ValueType: Sync {
  type ValueType = ();

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).filter(|v| v == &self.expected_value).map(|_| ())
  }
}
pub struct AssertEq<R: Rom, M: Metric<R>> {
  metric: M,
  expected_value: M::ValueType,
}
impl<R: Rom, M: Metric<R>> Metric<R> for AssertEq<R, M> where M::ValueType: Sync {
  type ValueType = ();

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).map(|v| assert_eq!(v, self.expected_value))
  }
}
pub struct DebugPrint<M> {
  metric: M,
}
impl<R: Rom, M: Metric<R>> Metric<R> for DebugPrint<M> {
  type ValueType = M::ValueType;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    let value = self.metric.evaluate(gb);
    log::debug!("{:?}", value);
    value
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
impl<R: Rom, F: Sync, V: MetricValueType> Metric<R> for FnMetric<F> where F: Fn(&mut Gb<R>) -> Option<V> {
  type ValueType = V;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    (self.f)(gb)
  }
}
impl<R: Rom, F: Sync, V: MetricValueType> Metric<R> for F where F: Fn(&mut Gb<R>) -> Option<V> {
  type ValueType = V;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self(gb)
  }
}


pub struct NullMetric;
impl<R: Rom> Metric<R> for NullMetric {
  type ValueType = ();

  fn evaluate(&self, _gb: &mut Gb<R>) -> Option<Self::ValueType> {
    Some(())
  }
}

#[allow(dead_code)]
pub struct TrainerIDMetric {}
impl<R: Rom + TrainerIDAddresses> Metric<R> for TrainerIDMetric {
  type ValueType = u16;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.step_until(&[R::TRAINER_ID_AFTER_GENERATION_ADDRESS]) == 0 { return None; }
    Some(gb.gb.read_memory_word_be(R::TRAINER_ID_MEM_ADDRESS))
  }
}
