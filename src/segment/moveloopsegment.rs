use gambatte::Input;
use gb::*;
use rom::*;
use statebuffer::StateBuffer;
use std::marker::PhantomData;

pub struct MoveLoopSegment<F> {
  input: Input,
  check_func: F,
  debug_output: bool,
}
impl <F> MoveLoopSegment<F> {
  pub fn new(check_func: F) -> Self {
    Self {
      input: Input::empty(),
      check_func: check_func,
      debug_output: false,
    }
  }
  #[allow(dead_code)]
  pub fn with_input(mut self, input: Input) -> Self { self.input = input; self }
}
impl<F> super::WithDebugOutput for MoveLoopSegment<F> {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}

impl<F, T: JoypadAddresses + RngAddresses> super::Segment<T> for MoveLoopSegment<F> where F: Fn(&mut Gb<T>) -> bool {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    iter.into_iter().map(|mut s| {
      gb.restore(&s);
      let mut skips = 0;
      loop {
        gb.input(self.input);
        if !(self.check_func)(gb) {
          if self.debug_output { println!("MoveLoopSegment left after {} skips", skips); }
          return s;
        }
        gb.restore(&s);
        gb.input(self.input);
        gb.step();
        s = gb.save();
        skips += 1;
      }
    }).collect()
  }
}
