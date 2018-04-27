use gambatte::Input;
use gb::*;
use rom::*;
use statebuffer::StateBuffer;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::marker::PhantomData;

// Represents a transition from one decision point to another decision point.
pub trait Segment {
  type Rom: JoypadAddresses + RngAddresses;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<Self::Rom>, iter: I) -> StateBuffer;
}
pub trait WithDebugOutput {
  fn with_debug_output(self, debug_output: bool) -> Self;
}
pub trait WithOutputBufferSize {
  fn with_buffer_size(self, buffer_size: usize) -> Self;
  fn with_unbounded_buffer(self) -> Self where Self: Sized {
    self.with_buffer_size(::statebuffer::STATE_BUFFER_UNBOUNDED_MAX_SIZE)
  }
}
pub trait SplitSegment {
  type KeyType: Eq + Hash;
  type Rom: JoypadAddresses + RngAddresses;

  fn execute_split<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<Self::Rom>, iter: I) -> HashMap<Self::KeyType, StateBuffer>;
}


pub trait StateBufferHashMap {
  fn to_state_buffer(self) -> StateBuffer;
  fn to_sized_state_buffer(self, size: usize) -> StateBuffer;
  fn to_unbounded_state_buffer(self) -> StateBuffer;
}
impl<K: Eq + Hash> StateBufferHashMap for HashMap<K, StateBuffer> {
  fn to_state_buffer(self) -> StateBuffer {
    StateBuffer::from_iter(self.into_iter().flat_map(|(_, v)| v))
  }
  fn to_sized_state_buffer(self, size: usize) -> StateBuffer {
    StateBuffer::from_iter_sized(self.into_iter().flat_map(|(_, v)| v), size)
  }
  fn to_unbounded_state_buffer(self) -> StateBuffer {
    StateBuffer::from_iter_unbounded(self.into_iter().flat_map(|(_, v)| v))
  }
}
pub trait Metric {
  type Rom;
  type ValueType: Eq + Hash;

  fn evaluate(&self, gb: &mut Gb<Self::Rom>) -> Option<Self::ValueType>;

  fn split_states<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<Self::Rom>, iter: I) -> HashMap<Self::ValueType, StateBuffer> 
      where Self::Rom: JoypadAddresses + RngAddresses {
    let mut result: HashMap<Self::ValueType, StateBuffer> = HashMap::new();
    for s in iter {
      gb.restore(&s);
      if let Some(value) = self.evaluate(gb) {
        gb.restore(&s);
        gb.step();
        result.entry(value).or_insert(StateBuffer::new()).add_state(gb.save());
      }
    }
    result
  }
}
pub struct NullMetric<T> {
  _rom: PhantomData<T>,
}
impl<T> NullMetric<T> {
  pub fn new() -> Self {
    Self {
      _rom: PhantomData,
    }
  }
}
impl<T> Metric for NullMetric<T> {
  type Rom = T;
  type ValueType = ();

  fn evaluate(&self, _gb: &mut Gb<Self::Rom>) -> Option<Self::ValueType> {
    Some(())
  }
}







// Checks whether an vblank input that was just made uses the input in the expected way. pre_address and post_address identify the expected before/after state around the use can should be closer than one video frame to each other.
fn is_correct_input_use<R: JoypadAddresses>(gb: &mut Gb<R>, pre_address: i32, use_address: i32, post_address: i32) -> bool {
  assert!(!gb.is_at_input);
  let hit = gb.step_until(&[&[pre_address, use_address], R::JOYPAD_USE_ADDRESSES].concat());
  if hit == pre_address {
    true // will always continue to use_address this frame since this is an input frame and no other input uses came before this point
  } else if hit == use_address {
    // didn't hit pre_address (meaning that the VBlank happened inbetween), so there has to be enough time left this frame to hit post_address.
    // check for pre_address to make sure it's not a different joypad use which just rolls into pre_address later.
    gb.step_until(&[post_address, pre_address]) == post_address
  } else {
    false // hit a different use of the joypad first
  }
}

pub mod overworld;

mod delaysegment;
pub use self::delaysegment::DelaySegment;
mod identifyinputsegment;
pub use self::identifyinputsegment::IdentifyInputSegment;
mod moveloopsegment;
pub use self::moveloopsegment::MoveLoopSegment;
mod movesegment;
pub use self::movesegment::MoveSegment;
mod skiptextssegment;
pub use self::skiptextssegment::SkipTextsSegment;
mod textsegment;
pub use self::textsegment::TextSegment;
