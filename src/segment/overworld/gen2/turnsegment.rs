use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;
use super::{OverworldInteractionResult,PlayerEventScript};

pub struct TurnSegment {
  input: Input,
  buffer_size: usize,
  debug_output: bool,
}
impl TurnSegment {
  #[allow(dead_code)]
  pub fn new(input: Input) -> Self {
    Self {
      input,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      debug_output: false,
    }
  }
}
impl WithOutputBufferSize for TurnSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl WithDebugOutput for TurnSegment {
  fn with_debug_output(self, debug_output: bool) -> Self { Self { debug_output, ..self } }
}

impl<R: Rom + Gen2MapEventsAddresses> Segment<R> for TurnSegment {
  type Key = super::OverworldInteractionResult;

  fn execute_split<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    let sb = MoveSegment::with_metric(self.input, TurnMetric {}).with_buffer_size(self.buffer_size).with_debug_output(self.debug_output).execute(gbe, iter);
    MoveLoopSegment::new(super::OverworldInteractionMetric {}.filter(|v| v != &OverworldInteractionResult::ScriptRunning(PlayerEventScript::JoyChangeFacing))).with_buffer_size(self.buffer_size).with_debug_output(self.debug_output).execute_split(gbe, sb)
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
