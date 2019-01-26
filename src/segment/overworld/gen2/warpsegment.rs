use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;
use super::{OverworldInteractionResult,PlayerEventScript};

#[allow(dead_code)]
pub struct WarpSegment {
  input: Input,
  debug_output: bool,
}
impl Default for WarpSegment {
    fn default() -> Self {
      Self {
        input: Input::empty(),
        debug_output: false,
      }
    }
}
impl WarpSegment {
  #[allow(dead_code)]
  pub fn new() -> Self { Default::default() }
  #[allow(dead_code)]
  pub fn with_input(mut self, input: Input) -> Self { self.input = input; self }
}
impl WithDebugOutput for WarpSegment {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}

impl<T: Rom + Gen2MapEventsAddresses> Segment<T> for WarpSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    let sb = MoveSegment::with_metric(self.input, WarpMetric {}).with_debug_output(self.debug_output).execute(gb, iter);
    let sb = MoveLoopSegment::new(super::OverworldInteractionMetric {}.filter(|v| v != &OverworldInteractionResult::ScriptRunning(PlayerEventScript::Warp))).with_debug_output(self.debug_output).execute(gb, sb);
    MoveLoopSegment::new(super::OverworldInteractionMetric {}.filter(|v| v != &OverworldInteractionResult::ForcedMovement)).with_debug_output(self.debug_output).execute(gb, sb)
  }
}

impl<R: Rom + Gen2MapEventsAddresses> ParallelSegment<R> for WarpSegment {
  type Key = OverworldInteractionResult;

  fn execute_parallel<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    let sb = MoveSegment::with_metric(self.input, WarpMetric {}).with_debug_output(self.debug_output).execute_parallel_single(gbe, iter);
    let sb = MoveLoopSegment::new(super::OverworldInteractionMetric {}.filter(|v| v != &OverworldInteractionResult::ScriptRunning(PlayerEventScript::Warp)).into_unit()).with_debug_output(self.debug_output).execute_parallel_single(gbe, sb);
    MoveLoopSegment::new(super::OverworldInteractionMetric {}.filter(|v| v != &OverworldInteractionResult::ForcedMovement)).with_debug_output(self.debug_output).execute_parallel(gbe, sb)
  }
}

struct WarpMetric {}
impl<R: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> Metric<R> for WarpMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    let result = super::get_overworld_interaction_result(gb);
    if result != OverworldInteractionResult::Warped {
      println!("WarpSegment warping failed: {:?}", result); None
    } else { Some(()) }
  }
}
