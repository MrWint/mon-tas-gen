use gambatte::Input;
use gb::*;
use rom::*;
use statebuffer::StateBuffer;
use std::marker::PhantomData;

pub struct IdentifyInputSegment<R: JoypadAddresses + RngAddresses + InputIdentificationAddresses> {
  debug_output: bool,
  _rom: PhantomData<R>,
}
impl<R: JoypadAddresses + RngAddresses + InputIdentificationAddresses> super::WithDebugOutput for IdentifyInputSegment<R> {
  fn with_debug_output(mut self) -> Self { self.debug_output = true; self }
}
impl<R: JoypadAddresses + RngAddresses + InputIdentificationAddresses> IdentifyInputSegment<R> {
  pub fn new() -> Self {
    IdentifyInputSegment {
      debug_output: false,
      _rom: PhantomData,
    }
  }

  fn print_identification(gb: &mut Gb<R>, s: &State) {
    for &(pre, input, post, name) in R::II_ADDRESSES {
      gb.restore(s);
      gb.input(Input::empty());
      if super::is_correct_input_use(gb, pre, input, post) {
        println!("Identified input as {}", name);
        return;
      }
    }
    println!("Input not identified");
  }
}
impl<R: JoypadAddresses + RngAddresses + InputIdentificationAddresses> super::Segment for IdentifyInputSegment<R> {
  type Rom = R;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer {
    let mut goal_buffer = StateBuffer::new();
    for s in iter.into_iter() {
      Self::print_identification(gb, &s);
      goal_buffer.add_state(s);
    }
    goal_buffer
  }
}
