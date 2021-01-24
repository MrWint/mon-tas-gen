use crate::metric::*;
use crate::metric::overworld::gen2::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct TurnSegment {
  input: Input,
  buffer_size: usize,
}
impl TurnSegment {
  #[allow(dead_code)]
  pub fn new(input: Input) -> Self {
    Self {
      input,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
}
impl WithOutputBufferSize for TurnSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + Gen2MapEventsAddresses> Segment<R> for TurnSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let sb = MoveSegment::with_metric(self.input, TurnMetric {}).with_buffer_size(self.buffer_size).execute(gbe, sb);
    MoveLoopSegment::new(OverworldInteractionMetric {}.filter(|v| v != &OverworldInteractionResult::ScriptRunning(PlayerEventScript::JoyChangeFacing)).into_unit()).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
  }
}

struct TurnMetric {}
impl<R: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> Metric<R> for TurnMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    let result = get_overworld_interaction_result(gb);
    if result != OverworldInteractionResult::Turned {
      log::warn!("TurnSegment turning failed: {:?}", result); None
    } else { Some(()) }
  }
}
