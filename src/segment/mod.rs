use gb::*;
use rom::*;
use statebuffer::StateBuffer;

// Represents a transition from one decision point to another decision point.
pub trait Segment {
  type Rom: JoypadAddresses + RngAddresses;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<Self::Rom>, iter: I) -> StateBuffer;
}
pub trait WithDebugOutput {
  fn with_debug_output(self) -> Self;
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
mod movesegment;
pub use self::movesegment::MoveSegment;
mod skiptextssegment;
pub use self::skiptextssegment::SkipTextsSegment;
mod textsegment;
pub use self::textsegment::TextSegment;
