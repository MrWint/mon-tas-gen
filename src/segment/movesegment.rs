use gambatte::Input;
use gb::*;
use rom::*;
use statebuffer::StateBuffer;
use std::marker::PhantomData;

pub struct MoveSegment<F, T> {
  input: Input,
  check_func: F,
  max_skips: u32,
  debug_output: bool,
  _rom: PhantomData<T>,
}
impl <F, T: JoypadAddresses + RngAddresses> MoveSegment<F, T> where F: Fn(&mut Gb<T>) -> bool {
  pub fn with_check(input: Input, check_func: F) -> Self {
    Self {
      input: input,
      check_func: check_func,
      max_skips: 0,
      debug_output: false,
      _rom: PhantomData,
    }
  }
  pub fn with_max_skips(mut self, max_skips: u32) -> Self {
    self.max_skips = max_skips;
    self
  }
}
impl<F, T: JoypadAddresses + RngAddresses> super::WithDebugOutput for MoveSegment<F, T> {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}
impl <T: JoypadAddresses + RngAddresses> MoveSegment<fn(&mut Gb<T>) -> bool, T> {
  pub fn new(input: Input) -> Self {
    Self {
      input: input,
      check_func: Self::always_true,
      max_skips: 0,
      debug_output: false,
      _rom: PhantomData,
    }
  }
  fn always_true(_: &mut Gb<T>) -> bool { true }
}

impl<F, T: JoypadAddresses + RngAddresses> super::Segment for MoveSegment<F, T> where F: Fn(&mut Gb<T>) -> bool {
  type Rom = T;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    iter.into_iter().flat_map(|mut s| {
      let mut result = vec![];
      let mut skips = 0;
      loop {
        if self.debug_output && skips == 0 {
          gb.restore(&s);
          gb.input(self.input);
          let hit = gb.step_until(T::JOYPAD_USE_ADDRESSES);
          println!("MoveSegment use at pc {:04x} {}", hit, gb.get_stack_trace_string());
        }
        gb.restore(&s);
        gb.input(self.input);
        if (self.check_func)(gb) {
          gb.restore(&s);
          gb.input(self.input);
          gb.step();
          result.push(gb.save());
        }
        if skips >= self.max_skips { break; }
        gb.restore(&s);
        gb.input(Input::empty());
        gb.step();
        s = gb.save();
        skips += 1;
      }
      result
    }).collect()
  }
}
