use gambatte::Input;
use gb::*;
use rom::*;
use segment::*;
use statebuffer::StateBuffer;
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
impl<R, M> MoveSegment<R, M> {
  pub fn with_metric(input: Input, metric: M) -> Self {
    Self {
      input: input,
      metric: metric,
      max_skips: 0,
      debug_output: false,
      buffer_size: ::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      _rom: PhantomData,
    }
  }
  pub fn with_max_skips(mut self, max_skips: u32) -> Self { self.max_skips = max_skips; self }
}
impl<R> MoveSegment<R, NullMetric> {
  pub fn new(input: Input) -> Self {
    Self {
      input: input,
      metric: NullMetric::new(),
      max_skips: 0,
      debug_output: false,
      buffer_size: ::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
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
    self.execute_split(gb, iter).to_sized_state_buffer(self.buffer_size)
  }
}
impl<R: JoypadAddresses + RngAddresses, M: Metric<R>> SplitSegment<R> for MoveSegment<R, M> {
  type KeyType = M::ValueType;

  fn execute_split<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> HashMap<Self::KeyType, StateBuffer> {
    let mut result: HashMap<Self::KeyType, StateBuffer> = HashMap::new();
    for mut s in iter {
      let mut skips = 0;
      loop {
        if self.debug_output && skips == 0 {
          gb.restore(&s);
          gb.input(self.input);
          let hit = gb.step_until(R::JOYPAD_USE_ADDRESSES);
          println!("MoveSegment use at pc {:04x} {}", hit, gb.get_stack_trace_string());
        }
        gb.restore(&s);
        gb.input(self.input);
        if let Some(value) = self.metric.evaluate(gb) {
          if gb.skipped_relevant_inputs { // restore state if metric overran next input
            gb.restore(&s);
            gb.input(self.input);
          }
          if !gb.is_at_input { gb.step(); }
          result.entry(value).or_insert(StateBuffer::with_max_size(self.buffer_size)).add_state(gb.save());
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
