use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;
use std::collections::HashMap;
use std::marker::PhantomData;



pub struct MoveSegment<R, M> {
  input: Input,
  metric: M,
  max_skips: u32,
  debug_output: bool,
  buffer_size: usize,
  _rom: PhantomData<R>,
}
impl<R: JoypadAddresses + RngAddresses, F: Metric<R>, V> MoveSegment<R, F> where F: Fn(&mut Gb<R>) -> Option<V> {
  pub fn with_metric_fn(input: Input, f: F) -> Self { Self::with_metric(input, f) }
}
impl<R: JoypadAddresses + RngAddresses, M: Metric<R>> MoveSegment<R, M> {
  pub fn with_metric(input: Input, metric: M) -> Self {
    Self {
      input,
      metric,
      max_skips: 0,
      debug_output: false,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      _rom: PhantomData,
    }
  }
  pub fn with_max_skips(mut self, max_skips: u32) -> Self { self.max_skips = max_skips; self }
}
impl<R> MoveSegment<R, NullMetric> {
  pub fn new(input: Input) -> Self {
    Self {
      input,
      metric: NullMetric {},
      max_skips: 0,
      debug_output: false,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      _rom: PhantomData,
    }
  }
}
impl<R, M> WithDebugOutput for MoveSegment<R, M> {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}
impl<R, M> WithOutputBufferSize for MoveSegment<R, M> {
  fn with_buffer_size(mut self, buffer_size: usize) -> Self { self.buffer_size = buffer_size; self }
}

impl<R: JoypadAddresses + RngAddresses, M: Metric<R>> Segment<R> for MoveSegment<R, M> {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer {
    self.execute_split(gb, iter).merge_state_buffers_sized(self.buffer_size)
  }
}
impl<R: JoypadAddresses + RngAddresses, M: Metric<R>> SplitSegment<R> for MoveSegment<R, M> {
  type Key = M::ValueType;

  fn execute_split<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> HashMap<Self::Key, StateBuffer> {
    let mut result: HashMap<Self::Key, StateBuffer> = HashMap::new();
    for mut s in iter {
      gb.restore(&s);
      let mut skips = 0;
      loop {
        if self.debug_output && skips == 0 {
          gb.input(self.input);
          let hit = gb.step_until(R::JOYPAD_USE_ADDRESSES);
          println!("MoveSegment use at pc {:04x} {}", hit, gb.get_stack_trace_string());
          gb.restore(&s);
        }
        gb.input(self.input);
        if let Some(value) = self.metric.evaluate(gb) {
          if gb.skipped_relevant_inputs { // restore state if metric overran next input
            gb.restore(&s);
            gb.input(self.input);
          }
          if !gb.is_at_input { gb.step(); }
          result.entry(value).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_state(gb.save());
        }
        if skips >= self.max_skips { break; }
        gb.restore(&s);
        gb.input(Input::empty());
        gb.step();
        s = gb.save();
        skips += 1;
      }
    }
    result
  }
}






impl<R: Rom, M: Metric<R>> ParallelSegment<R> for MoveSegment<R, M> {
  type Key = M::ValueType;

  fn execute_parallel<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    gbe.execute(iter, move |gb, mut s, tx| {
      gb.restore(&s);
      let mut skips = 0;
      loop {
        if self.debug_output && skips == 0 {
          gb.input(self.input);
          let hit = gb.step_until(R::JOYPAD_USE_ADDRESSES);
          println!("MoveSegment use at pc {:04x} {}", hit, gb.get_stack_trace_string());
          gb.restore(&s);
        }
        gb.input(self.input);
        if let Some(value) = self.metric.evaluate(gb) {
          if gb.skipped_relevant_inputs { // restore state if metric overran next input
            gb.restore(&s);
            gb.input(self.input);
          }
          if !gb.is_at_input { gb.step(); }
          tx.send((value, gb.save())).unwrap();
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
