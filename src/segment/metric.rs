use crate::gb::*;
use crate::gbexecutor::StateKey;
use crate::rom::*;
use std::marker::PhantomData;

pub trait Metric<R>: Sync {
  type ValueType: StateKey;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType>;
  // evaluates the metric and if it returns Some(_), finish the current step.
  fn evaluate_and_step<V>(&self, gb: &mut Gb<R>, s: State<V>, input: gambatte::Input) -> Option<Self::ValueType> where R: JoypadAddresses {
    if let Some(value) = self.evaluate(gb) {
      if gb.skipped_relevant_inputs { // restore state if metric overran next input
        gb.restore(&s);
        gb.input(input);
      }
      if !gb.is_at_input { gb.step(); }
      Some(value)
    } else { None }
  }

  fn filter<F>(self, f: F) -> Filter<R, Self, F> where Self: Sized, F: Fn(&Self::ValueType) -> bool {
    Filter { metric: self, f, _rom: PhantomData, }
  }
  fn expect(self, expected_value: Self::ValueType) -> Expect<R, Self> where Self: Sized {
    Expect { metric: self, expected_value, _rom: PhantomData, }
  }
  fn map<F, K: StateKey>(self, f: F) -> Map<R, Self, F> where Self: Sized, F: Fn(Self::ValueType) -> K {
    Map { metric: self, f, _rom: PhantomData, }
  }
  fn and_then<M: Metric<R>>(self, then_metric: M) -> AndThen<R, Self, M> where Self: Sized {
    AndThen { metric: self, then_metric, _rom: PhantomData, }
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
pub struct AndThen<R, M, TM> {
  metric: M,
  then_metric: TM,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>, TM: Metric<R>> Metric<R> for AndThen<R, M, TM> {
  type ValueType = TM::ValueType;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).and_then(|_| self.then_metric.evaluate(gb))
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
pub struct Expect<R, M: Metric<R>> {
  metric: M,
  expected_value: M::ValueType,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>> Metric<R> for Expect<R, M> where M::ValueType: Sync {
  type ValueType = ();

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    self.metric.evaluate(gb).filter(|v| v == &self.expected_value).map(|_| ())
  }
}
pub struct DebugPrint<R, M> {
  metric: M,
  _rom: PhantomData<R>,
}
impl<R: Sync, M: Metric<R>> Metric<R> for DebugPrint<R, M> {
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
}
#[allow(dead_code)]
pub struct Gen1DVMetric {}
impl<R: JoypadAddresses + Gen1DVAddresses> Metric<R> for Gen1DVMetric {
  type ValueType = DVs;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(R::AFTER_DV_GENERATION_ADDRESSES) == 0 { return None; }
    let registers = gb.gb.read_registers();

    Some(DVs::from_u16_be((registers.a as u16) << 8 | (registers.b as u16)))
  }
}
#[allow(dead_code)]
pub struct Gen2DVMetric {}
impl<R: JoypadAddresses + Gen2DVAddresses> Metric<R> for Gen2DVMetric {
  type ValueType = DVs;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(&[R::AFTER_DV_GENERATION_ADDRESS]) == 0 { return None; }
    let registers = gb.gb.read_registers();

    Some(DVs::from_u16_be((registers.b as u16) << 8 | (registers.c as u16)))
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
