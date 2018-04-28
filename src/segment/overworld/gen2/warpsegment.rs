use gambatte::Input;
use gb::*;
use rom::*;
use segment::WithDebugOutput;
use statebuffer::StateBuffer;
use super::{OverworldInteractionResult,PlayerEventScript};

#[allow(dead_code)]
pub struct WarpSegment {
  input: Input,
  debug_output: bool,
}
impl WarpSegment {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self {
      input: Input::empty(),
      debug_output: false,
    }
  }
  #[allow(dead_code)]
  pub fn with_input(mut self, input: Input) -> Self { self.input = input; self }
}
impl WithDebugOutput for WarpSegment {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}

impl<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> ::segment::Segment<T> for WarpSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    let sb = ::segment::MoveSegment::with_check(self.input, |gb: &mut Gb<T>| {
      let result = super::get_overworld_interaction_result(gb);
      if result != OverworldInteractionResult::Warped {
        println!("WarpSegment warping failed: {:?}", result); false
      } else { true }
    }).with_debug_output(self.debug_output).execute(gb, iter);
    let sb = ::segment::MoveLoopSegment::new(|gb: &mut Gb<T>| super::get_overworld_interaction_result(gb) == OverworldInteractionResult::ScriptRunning(PlayerEventScript::Warp)).with_debug_output(self.debug_output).execute(gb, sb);
    ::segment::MoveLoopSegment::new(|gb: &mut Gb<T>| super::get_overworld_interaction_result(gb) == OverworldInteractionResult::ForcedMovement).with_debug_output(self.debug_output).execute(gb, sb)
  }
}
