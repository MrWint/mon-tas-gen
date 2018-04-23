use gambatte::Input;
use gb::*;
use rom::*;
use segment::WithDebugOutput;
use statebuffer::StateBuffer;
use std::marker::PhantomData;
use super::OverworldInteractionResult;

pub struct JumpLedgeSegment<T> {
  input: Input,
  debug_output: bool,
  _rom: PhantomData<T>,
}
impl <T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> JumpLedgeSegment<T> {
  #[allow(dead_code)]
  pub fn new(input: Input) -> Self {
    Self {
      input: input,
      debug_output: false,
      _rom: PhantomData,
    }
  }
}
impl<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> WithDebugOutput for JumpLedgeSegment<T> {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}

impl<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> ::segment::Segment for JumpLedgeSegment<T> {
  type Rom = T;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    ::segment::MoveSegment::with_check(self.input, |gb| {
      let result = super::get_overworld_interaction_result(gb);
      if result != OverworldInteractionResult::JumpedLedge {
        println!("JumpLedgeSegment jumping failed: {:?}", result); false
      } else { true }
    }).with_debug_output(self.debug_output).execute(gb, iter)
  }
}
