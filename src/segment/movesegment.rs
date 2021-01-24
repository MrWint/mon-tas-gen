use crate::metric::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;
use log::trace;
use std::collections::HashMap;
use std::marker::PhantomData;



pub struct MoveSegment<R, M> {
  input: Input,
  metric: M,
  max_skips: u32,
  buffer_size: usize,
  _rom: PhantomData<R>,
}
impl<R: JoypadAddresses + RngAddresses, F: Metric<R>, V> MoveSegment<R, F> where F: Fn(&mut dyn GbI<R>) -> Option<V> {
  pub fn with_metric_fn(input: Input, f: F) -> Self { Self::with_metric(input, f) }
}
impl<R: JoypadAddresses + RngAddresses, M: Metric<R>> MoveSegment<R, M> {
  pub fn with_metric(input: Input, metric: M) -> Self {
    Self {
      input,
      metric,
      max_skips: 0,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      _rom: PhantomData,
    }
  }
  pub fn with_max_skips(self, max_skips: u32) -> Self { Self { max_skips, ..self } }
}
impl<R> MoveSegment<R, NullMetric> {
  pub fn new(input: Input) -> Self {
    Self {
      input,
      metric: NullMetric,
      max_skips: 0,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      _rom: PhantomData,
    }
  }
}
impl<R, M> WithOutputBufferSize for MoveSegment<R, M> {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom, M: Metric<R>> Segment<R> for MoveSegment<R, M> {
  type Key = M::ValueType;

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    gbe.execute(sb, move |gb, mut s, tx| {
      gb.restore(&s);
      let mut skips = 0;
      loop {
        gb.input(self.input);
        let hit = gb.step_until(R::JOYPAD_USE_ADDRESSES); // don't check for the metric until after the joypad input has been used.
        assert!(hit != 0); // This is a decision point, there has to be a joypad use.
        trace!("MoveSegment use at pc {:04x} {}", hit, gb.get_stack_trace_string());
        if let Some(value) = self.metric.evaluate(gb) {
          if gb.skipped_relevant_inputs { // restore state if metric overran next input
            gb.restore(&s);
            gb.input(self.input);
          }
          if !gb.is_at_input { gb.step(); }
          tx.send(gb.save_with_value(value)).unwrap();
        }
        if skips >= self.max_skips { break; }
        gb.restore(&s);
        gb.input(Input::empty());
        gb.step();
        s = gb.save();
        skips += 1;
      }
    }).into_state_buffer_map(self.buffer_size)
  }
}
