use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct MoveLoopSegment<M> {
  input: Input,
  metric: M,
  debug_output: bool,
  buffer_size: usize,
}
impl <M> MoveLoopSegment<M> {
  pub fn new(metric: M) -> Self {
    Self {
      input: Input::empty(),
      metric,
      debug_output: false,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  #[allow(dead_code)]
  pub fn with_input(mut self, input: Input) -> Self { self.input = input; self }
}
impl<M> WithDebugOutput for MoveLoopSegment<M> {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}
impl<M> WithOutputBufferSize for MoveLoopSegment<M> {
  fn with_buffer_size(mut self, buffer_size: usize) -> Self { self.buffer_size = buffer_size; self }
}

impl<R: Rom, M: Metric<R>> Segment<R> for MoveLoopSegment<M> {
  type Key = M::ValueType;

  fn execute_split<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    gbe.execute(iter, move |gb, mut s, tx| {
      gb.restore(&s);
      let mut skips = 0;
      loop {
        gb.input(self.input);
        if let Some(value) = self.metric.evaluate(gb) {
          tx.send((value, s)).unwrap();
          if self.debug_output { println!("MoveLoopSegment left after {} skips", skips); }
          break;
        }
        if gb.skipped_relevant_inputs { // restore state if metric overran next input
          gb.restore(&s);
          gb.input(self.input);
        }
        if !gb.is_at_input { gb.step(); }
        s = gb.save();
        skips += 1;
      }
    }).into_state_buffer_map(self.buffer_size)
  }
}
