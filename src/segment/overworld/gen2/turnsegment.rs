use gambatte::Input;
use gb::*;
use rom::*;
use segment::WithDebugOutput;
use statebuffer::StateBuffer;
use super::{OverworldInteractionResult,PlayerEventScript};

pub struct TurnSegment {
  input: Input,
  debug_output: bool,
}
impl TurnSegment {
  #[allow(dead_code)]
  pub fn new(input: Input) -> Self {
    Self {
      input: input,
      debug_output: false,
    }
  }
}
impl WithDebugOutput for TurnSegment {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}

impl<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> ::segment::Segment<T> for TurnSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    let sb = ::segment::MoveSegment::with_check(self.input, |gb: &mut Gb<T>| {
      let result = super::get_overworld_interaction_result(gb);
      if result != OverworldInteractionResult::Turned {
        println!("TurnSegment turning failed: {:?}", result); false
      } else { true }
    }).with_debug_output(self.debug_output).execute(gb, iter);
    ::segment::MoveLoopSegment::new(|gb: &mut Gb<T>| super::get_overworld_interaction_result(gb) == OverworldInteractionResult::ScriptRunning(PlayerEventScript::JoyChangeFacing)).with_debug_output(self.debug_output).execute(gb, sb)
  }
}
