use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;
use super::OverworldInteractionResult;

pub struct JumpLedgeSegment {
  input: Input,
  buffer_size: usize,
  debug_output: bool,
}
impl JumpLedgeSegment {
  #[allow(dead_code)]
  pub fn new(input: Input) -> Self {
    Self {
      input,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      debug_output: false,
    }
  }
}
impl WithOutputBufferSize for JumpLedgeSegment {
  fn with_buffer_size(mut self, buffer_size: usize) -> Self { self.buffer_size = buffer_size; self }
}
impl WithDebugOutput for JumpLedgeSegment {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}

impl<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> Segment<T> for JumpLedgeSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    MoveSegment::with_metric(self.input, JumpLedgeMetric {}).with_debug_output(self.debug_output).execute(gb, iter)
  }
}
impl<R: Rom + Gen2MapEventsAddresses> ParallelSegment<R> for JumpLedgeSegment {
  type Key = ();

  fn execute_parallel<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    MoveSegment::with_metric(self.input, JumpLedgeMetric {}).with_buffer_size(self.buffer_size).with_debug_output(self.debug_output).execute_parallel(gbe, iter)
  }
}

struct JumpLedgeMetric {}
impl<R: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> Metric<R> for JumpLedgeMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    let result = super::get_overworld_interaction_result(gb);
    if result != OverworldInteractionResult::JumpedLedge {
      println!("JumpLedgeSegment jumping failed: {:?}", result); None
    } else { Some(()) }
  }
}
