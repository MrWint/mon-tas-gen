use gb::*;
use rom::*;
use statebuffer::StateBuffer;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FromIterator;


// Represents a transition from one decision point to another decision point.
pub trait Segment<R> {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer;
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
pub trait SplitSegment<R> {
  type KeyType: Eq + Hash + Debug;

  fn execute_split<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> HashMap<Self::KeyType, StateBuffer>;
}


pub trait StateBufferHashMap {
  fn to_state_buffer(self) -> StateBuffer;
  fn to_sized_state_buffer(self, size: usize) -> StateBuffer;
  fn to_unbounded_state_buffer(self) -> StateBuffer;
  fn is_all_full(&self) -> bool;
  fn to_string(&self) -> String;
}
impl<K: Eq + Hash + Debug> StateBufferHashMap for HashMap<K, StateBuffer> {
  fn to_state_buffer(self) -> StateBuffer {
    StateBuffer::from_iter(self.into_iter().flat_map(|(_, v)| v))
  }
  fn to_sized_state_buffer(self, size: usize) -> StateBuffer {
    StateBuffer::from_iter_sized(self.into_iter().flat_map(|(_, v)| v), size)
  }
  fn to_unbounded_state_buffer(self) -> StateBuffer {
    StateBuffer::from_iter_unbounded(self.into_iter().flat_map(|(_, v)| v))
  }
  fn is_all_full(&self) -> bool {
    !self.is_empty() && self.values().all(|sb| sb.is_full())
  }
  fn to_string(&self) -> String {
    format!("{:?}", self.iter().map(|(k, v)| (k, format!("{}", v))).collect::<HashMap<_,_>>())
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

mod metric;
pub use self::metric::Metric;
pub use self::metric::FnMetric;
pub use self::metric::NullMetric;
pub use self::metric::Gen2DVMetric;
pub use self::metric::DVs;

mod applyindividuallysegment;
pub use self::applyindividuallysegment::ApplyIndividuallySegment;
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
