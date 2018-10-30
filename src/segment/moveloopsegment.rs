use gambatte::Input;
use gb::*;
use rom::*;
use segment::*;
use statebuffer::StateBuffer;

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
      metric: metric,
      debug_output: false,
      buffer_size: ::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
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

impl<R: JoypadAddresses + RngAddresses, M: Metric<R>> Segment<R> for MoveLoopSegment<M> {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer {
    self.execute_split(gb, iter).to_sized_state_buffer(self.buffer_size)
  }
}
impl<R: JoypadAddresses + RngAddresses, M: Metric<R>> SplitSegment<R> for MoveLoopSegment<M> {
  type KeyType = M::ValueType;

  fn execute_split<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> HashMap<Self::KeyType, StateBuffer> {
    let mut result: HashMap<Self::KeyType, StateBuffer> = HashMap::new();
    for mut s in iter {
      gb.restore(&s);
      let mut skips = 0;
      loop {
        gb.input(self.input);
        if let Some(value) = self.metric.evaluate(gb) {
          result.entry(value).or_insert(StateBuffer::with_max_size(self.buffer_size)).add_state(s);
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
    }
    result
  }
}
