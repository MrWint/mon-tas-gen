use gambatte::Input;
use gb::*;
use rom::*;
use segment::*;
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
      input,
      debug_output: false,
    }
  }
}
impl WithDebugOutput for TurnSegment {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}

impl<T: Rom + Gen2MapEventsAddresses> Segment<T> for TurnSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    let sb = MoveSegment::with_metric(self.input, TurnMetric {}).with_debug_output(self.debug_output).execute(gb, iter);
    MoveLoopSegment::new(super::OverworldInteractionMetric {}.filter(|v| v != &OverworldInteractionResult::ScriptRunning(PlayerEventScript::JoyChangeFacing))).with_debug_output(self.debug_output).execute(gb, sb)
  }
}

struct TurnMetric {}
impl<R: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> Metric<R> for TurnMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    let result = super::get_overworld_interaction_result(gb);
    if result != OverworldInteractionResult::Turned {
      println!("TurnSegment turning failed: {:?}", result); None
    } else { Some(()) }
  }
}
