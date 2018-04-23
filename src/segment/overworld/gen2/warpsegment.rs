use gambatte::Input;
use gb::*;
use rom::*;
use segment::WithDebugOutput;
use statebuffer::StateBuffer;
use std::marker::PhantomData;
use super::{OverworldInteractionResult,PlayerEventScript};

#[allow(dead_code)]
pub struct WarpSegment<T> {
  input: Input,
  debug_output: bool,
  _rom: PhantomData<T>,
}
impl <T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> WarpSegment<T> {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self {
      input: Input::empty(),
      debug_output: false,
      _rom: PhantomData,
    }
  }
  #[allow(dead_code)]
  pub fn with_input(mut self, input: Input) -> Self { self.input = input; self }
}
impl<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> WithDebugOutput for WarpSegment<T> {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}

impl<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> ::segment::Segment for WarpSegment<T> {
  type Rom = T;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    let sb = ::segment::MoveSegment::with_check(self.input, |gb| {
      let result = super::get_overworld_interaction_result(gb);
      if result != OverworldInteractionResult::Warped {
        println!("WarpSegment warping failed: {:?}", result); false
      } else { true }
    }).with_debug_output(self.debug_output).execute(gb, iter);
    let sb = ::segment::MoveLoopSegment::new(|gb| super::get_overworld_interaction_result(gb) == OverworldInteractionResult::ScriptRunning(PlayerEventScript::Warp)).with_debug_output(self.debug_output).execute(gb, sb);
    ::segment::MoveLoopSegment::new(|gb| super::get_overworld_interaction_result(gb) == OverworldInteractionResult::ForcedMovement).with_debug_output(self.debug_output).execute(gb, sb)
  }
}
