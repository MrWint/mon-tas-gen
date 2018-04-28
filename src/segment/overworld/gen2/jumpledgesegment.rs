use gambatte::Input;
use gb::*;
use rom::*;
use segment::WithDebugOutput;
use statebuffer::StateBuffer;
use super::OverworldInteractionResult;

pub struct JumpLedgeSegment {
  input: Input,
  debug_output: bool,
}
impl JumpLedgeSegment {
  #[allow(dead_code)]
  pub fn new(input: Input) -> Self {
    Self {
      input: input,
      debug_output: false,
    }
  }
}
impl WithDebugOutput for JumpLedgeSegment {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}

impl<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> ::segment::Segment<T> for JumpLedgeSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    ::segment::MoveSegment::with_check(self.input, |gb: &mut Gb<T>| {
      let result = super::get_overworld_interaction_result(gb);
      if result != OverworldInteractionResult::JumpedLedge {
        println!("JumpLedgeSegment jumping failed: {:?}", result); false
      } else { true }
    }).with_debug_output(self.debug_output).execute(gb, iter)
  }
}
