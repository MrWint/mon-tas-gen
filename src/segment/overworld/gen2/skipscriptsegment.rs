use crate::metric::*;
use crate::metric::overworld::gen2::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;

pub struct SkipScriptSegment {
  buffer_size: usize,
}
impl Default for SkipScriptSegment {
  fn default() -> Self {
    Self {
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
}
impl SkipScriptSegment {
  #[allow(dead_code)]
  pub fn new() -> Self { Default::default() }
}
impl WithOutputBufferSize for SkipScriptSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + Gen2MapEventsAddresses> Segment<R> for SkipScriptSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    MoveLoopSegment::new(OverworldInteractionMetric {}.filter(|v|
        v != &OverworldInteractionResult::CountStepEvent &&
        v != &OverworldInteractionResult::MapCoordEvent &&
        v != &OverworldInteractionResult::SceneScript &&
        v != &OverworldInteractionResult::ScriptRunning(PlayerEventScript::MapScript) &&
        v != &OverworldInteractionResult::ScriptRunning(PlayerEventScript::TalkToTrainer) &&
        v != &OverworldInteractionResult::ScriptRunning(PlayerEventScript::Fall) &&
        v != &OverworldInteractionResult::SeenByTrainer &&
        v != &OverworldInteractionResult::ScriptRunning(PlayerEventScript::SeenByTrainer)
    ).into_unit()).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
  }
}
