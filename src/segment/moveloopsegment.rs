use crate::metric::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;
use log::debug;

pub struct MoveLoopSegment<M> {
  input: Input,
  metric: M,
  buffer_size: usize,
}
impl <M> MoveLoopSegment<M> {
  pub fn new(metric: M) -> Self {
    Self {
      input: Input::empty(),
      metric,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  #[allow(dead_code)]
  pub fn with_input(self, input: Input) -> Self { Self { input, ..self } }
}
impl<M> WithOutputBufferSize for MoveLoopSegment<M> {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom, M: Metric<R>> Segment<R> for MoveLoopSegment<M> {
  type Key = M::ValueType;

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    gbe.execute(sb, move |gb, mut s, tx| {
      gb.restore(&s);
      let mut skips = 0;
      loop {
        gb.input(self.input);
        if let Some(value) = self.metric.evaluate(gb) {
          tx.send(s.replace_value(value)).unwrap();
          debug!("MoveLoopSegment left after {} skips", skips);
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
