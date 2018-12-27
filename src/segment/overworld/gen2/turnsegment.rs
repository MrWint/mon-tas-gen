use gambatte::Input;
use gb::*;
use rom::*;
use segment::*;
use statebuffer::StateBuffer;
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
      buffer_size: ::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
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

impl<T: Rom + Gen2MapEventsAddresses> Segment<T> for TurnSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    let sb = MoveSegment::with_metric(self.input, TurnMetric {}).with_debug_output(self.debug_output).execute(gb, iter);
    MoveLoopSegment::new(super::OverworldInteractionMetric {}.filter(|v| v != &OverworldInteractionResult::ScriptRunning(PlayerEventScript::JoyChangeFacing))).with_debug_output(self.debug_output).execute(gb, sb)
  }
}

impl<R: Rom + Gen2MapEventsAddresses> ParallelSegment<R> for TurnSegment {
  type Key = super::OverworldInteractionResult;

  fn execute_parallel<I: IntoIterator<Item=State>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    let sb = MoveSegment::with_metric(self.input, TurnMetric {}).with_buffer_size(self.buffer_size).with_debug_output(self.debug_output).execute_parallel_single(gbe, iter);
    MoveLoopSegment::new(super::OverworldInteractionMetric {}.filter(|v| v != &OverworldInteractionResult::ScriptRunning(PlayerEventScript::JoyChangeFacing))).with_buffer_size(self.buffer_size).with_debug_output(self.debug_output).execute_parallel(gbe, sb)
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
