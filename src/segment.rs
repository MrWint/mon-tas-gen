use crate::gb::*;
use crate::gbexecutor::*;
use crate::rom::*;
use crate::statebuffer::StateBuffer;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::marker::PhantomData;


/// Represents a transition from one decision point to another decision point.
pub trait Segment<R: Rom> {
  type Key: StateKey;

  fn execute_split(&self, gb: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer>;
  fn execute(&self, gb: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> StateBuffer<Self::Key> where Self::Key: Clone {
    self.execute_split(gb, sb).into_state_buffer()
  }
  fn seq<S2: Segment<R>>(self, s2: S2) -> SeqSegment<R, Self, S2> where Self: Sized + Segment<R, Key=()> {
    SeqSegment {
      s1: self,
      s2,
      _rom: PhantomData,
    }
  }
  fn seq_split<S2: Segment<R, Key=()>>(self, s2: S2) -> SeqSplitSegment<R, Self, S2> where Self: Sized {
    SeqSplitSegment {
      s1: self,
      s2,
      _rom: PhantomData,
    }
  }
}
pub struct SeqSegment<R, S1, S2> {
  s1: S1,
  s2: S2,
  _rom: PhantomData<R>,
}
impl <R: Rom, S1: Segment<R, Key=()>, S2: Segment<R>> Segment<R> for SeqSegment<R, S1, S2> {
  type Key = S2::Key;
  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let sb = self.s1.execute(gbe, sb);
    if sb.is_empty() { return HashMap::new() }
    self.s2.execute_split(gbe, sb)
  }
}

pub struct SeqSplitSegment<R, S1, S2> {
  s1: S1,
  s2: S2,
  _rom: PhantomData<R>,
}
impl <R: Rom, S1: Segment<R>, S2: Segment<R, Key=()>> Segment<R> for SeqSplitSegment<R, S1, S2> {
  type Key = S1::Key;
  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let sb_map = self.s1.execute_split(gbe, sb);
    sb_map.into_iter().map(|(k, sb)| (k, self.s2.execute(gbe, sb))).filter(|(_, sb)| !sb.is_empty()).collect()
  }
}

pub trait WithOutputBufferSize {
  fn with_buffer_size(self, buffer_size: usize) -> Self;
  fn with_unbounded_buffer(self) -> Self where Self: Sized {
    self.with_buffer_size(crate::statebuffer::STATE_BUFFER_UNBOUNDED_MAX_SIZE)
  }
}


pub trait StateBufferHashMap<V> {
  fn merge_state_buffers(self) -> StateBuffer<V>;
  fn merge_state_buffers_sized(self, size: usize) -> StateBuffer<V>;
  fn merge_state_buffers_unbounded(self) -> StateBuffer<V>;
  fn is_all_full(&self) -> bool;
  fn to_string(&self) -> String;
}
impl<V, K: StateKey, S: std::hash::BuildHasher> StateBufferHashMap<V> for HashMap<K, StateBuffer<V>, S> {
  fn merge_state_buffers(self) -> StateBuffer<V> {
    StateBuffer::from_iter(self.into_iter().flat_map(|(_, v)| v))
  }
  fn merge_state_buffers_sized(self, size: usize) -> StateBuffer<V> {
    StateBuffer::from_iter_sized(self.into_iter().flat_map(|(_, v)| v), size)
  }
  fn merge_state_buffers_unbounded(self) -> StateBuffer<V> {
    StateBuffer::from_iter_unbounded(self.into_iter().flat_map(|(_, v)| v))
  }
  fn is_all_full(&self) -> bool {
    !self.is_empty() && self.values().all(|sb| sb.is_full())
  }
  fn to_string(&self) -> String {
    format!("{:?}", self.iter().map(|(k, v)| (k, format!("{:?}", v))).collect::<HashMap<_,_>>())
  }
}
pub trait SingleStateBuffer<K> {
  fn into_state_buffer(self) -> StateBuffer<K>;
}
impl<K: StateKey + Clone, S: std::hash::BuildHasher> SingleStateBuffer<K> for HashMap<K, StateBuffer, S> {
  fn into_state_buffer(self) -> StateBuffer<K> {
    let max_size = self.iter().map(|(_, v)| v.get_max_size()).max().unwrap_or(0);
    StateBuffer::from_iter_sized(self.into_iter().flat_map(|(k, v)| v.into_iter().map(move |s| s.replace_value(k.clone()))), max_size)
  }
}



/// Assumes the execution is stopped right after an input was performed on a decision point.
/// Checks whether a vblank input that was just made uses the input in the expected way.
/// ```pre_address``` and ```post_address``` identify the expected before/after state around the use and should be closer than one frame to each other.
fn is_correct_input_use<R: JoypadAddresses>(gb: &mut Gb<R>, pre_address: i32, use_address: i32, post_address: i32) -> bool {
  assert!(!gb.is_at_input);
  let hit = gb.step_until(&[&[pre_address, use_address], R::JOYPAD_USE_ADDRESSES].concat());
  if hit == pre_address {
    true // will always continue to use_address since this is a decision point and no other input uses came before this point.
  } else if hit == use_address {
    // didn't hit pre_address (meaning that the VBlank happened inbetween), so there has to be enough time left this frame to hit post_address.
    // check for pre_address to make sure it's not a different joypad use which just rolls into post_address later.
    gb.step_until(&[post_address, pre_address]) == post_address
  } else {
    false // hit a different use of the joypad first
  }
}

pub mod battle;
pub mod overworld;

mod metric;
pub use self::metric::*;
pub use self::metric::FnMetric;
pub use self::metric::NullMetric;
pub use self::metric::Gen1DVMetric;
pub use self::metric::Gen2DVMetric;
pub use self::metric::DVs;
pub use self::metric::TrainerIDMetric;

mod applyindividuallysegment;
pub use self::applyindividuallysegment::ApplyIndividuallySegment;
mod delaysegment;
pub use self::delaysegment::DelaySegment;
mod graphsegment;
mod gen1itskiptextssegment;
pub use self::gen1itskiptextssegment::Gen1ITSkipTextsSegment;
mod identifyinputsegment;
pub use self::identifyinputsegment::IdentifyInputSegment;
mod joypadlowsensitivitysegment;
pub use self::joypadlowsensitivitysegment::JoypadLowSensitivitySegment;
mod moveloopsegment;
pub use self::moveloopsegment::MoveLoopSegment;
mod movesegment;
pub use self::movesegment::MoveSegment;
mod noopsegment;
pub use self::noopsegment::NoopSegment;
mod skiptextssegment;
pub use self::skiptextssegment::SkipTextsSegment;
mod textsegment;
pub use self::textsegment::TextSegment;
mod verifyinputsegment;
pub use self::verifyinputsegment::VerifyInputSegment;
