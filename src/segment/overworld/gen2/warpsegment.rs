use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;
use super::{OverworldInteractionResult,PlayerEventScript};

#[allow(dead_code)]
pub struct WarpSegment {
  input: Input,
  buffer_size: usize,
}
impl Default for WarpSegment {
    fn default() -> Self {
      Self {
        input: Input::empty(),
        buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      }
    }
}
impl WarpSegment {
  #[allow(dead_code)]
  pub fn new() -> Self { Default::default() }
  #[allow(dead_code)]
  pub fn with_input(self, input: Input) -> Self { Self { input, ..self } }
}
impl WithOutputBufferSize for WarpSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + Gen2MapEventsAddresses> Segment<R> for WarpSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let sb = MoveSegment::with_metric(self.input, WarpMetric {}).execute(gbe, sb);
    let sb = MoveLoopSegment::new(super::OverworldInteractionMetric {}.filter(|v| v != &OverworldInteractionResult::ScriptRunning(PlayerEventScript::Warp)).into_unit()).execute(gbe, sb);
    MoveLoopSegment::new(super::OverworldInteractionMetric {}.filter(|v| v != &OverworldInteractionResult::ForcedMovement).into_unit()).execute_split(gbe, sb)
  }
}

struct WarpMetric {}
impl<R: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> Metric<R> for WarpMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    let result = super::get_overworld_interaction_result(gb);
    if result != OverworldInteractionResult::Warped {
      log::warn!("WarpSegment warping failed: {:?}", result); None
    } else { Some(()) }
  }
}
