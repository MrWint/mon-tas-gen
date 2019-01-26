use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;

pub struct SkipScriptSegment {
  debug_output: bool,
  buffer_size: usize,
}
impl Default for SkipScriptSegment {
  fn default() -> Self {
    Self {
      debug_output: false,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
}
impl SkipScriptSegment {
  #[allow(dead_code)]
  pub fn new() -> Self { Default::default() }
}
impl WithDebugOutput for SkipScriptSegment {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}
impl WithOutputBufferSize for SkipScriptSegment {
  fn with_buffer_size(mut self, buffer_size: usize) -> Self { self.buffer_size = buffer_size; self }
}

impl<R: Rom + Gen2MapEventsAddresses> Segment<R> for SkipScriptSegment {
  type Key = super::OverworldInteractionResult;

  fn execute_split<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    MoveLoopSegment::new(super::OverworldInteractionMetric {}.filter(|v|
        v != &super::OverworldInteractionResult::MapCoordEvent &&
        v != &super::OverworldInteractionResult::SceneScript &&
        v != &super::OverworldInteractionResult::ScriptRunning(super::PlayerEventScript::MapScript)
    )).with_buffer_size(self.buffer_size).with_debug_output(self.debug_output).execute_split(gbe, iter)
  }
}
