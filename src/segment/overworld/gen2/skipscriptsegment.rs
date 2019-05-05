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
    MoveLoopSegment::new(super::OverworldInteractionMetric {}.filter(|v|
        v != &super::OverworldInteractionResult::CountStepEvent &&
        v != &super::OverworldInteractionResult::MapCoordEvent &&
        v != &super::OverworldInteractionResult::SceneScript &&
        v != &super::OverworldInteractionResult::ScriptRunning(super::PlayerEventScript::MapScript) &&
        v != &super::OverworldInteractionResult::ScriptRunning(super::PlayerEventScript::TalkToTrainer) &&
        v != &super::OverworldInteractionResult::ScriptRunning(super::PlayerEventScript::Fall) &&
        v != &super::OverworldInteractionResult::SeenByTrainer &&
        v != &super::OverworldInteractionResult::ScriptRunning(super::PlayerEventScript::SeenByTrainer)
    ).into_unit()).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
  }
}
