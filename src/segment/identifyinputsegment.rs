use gambatte::Input;
use gb::*;
use rom::*;
use statebuffer::StateBuffer;
use std::marker::PhantomData;

pub struct IdentifyInputSegment {
  debug_output: bool,
}
impl super::WithDebugOutput for IdentifyInputSegment {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}
impl IdentifyInputSegment {
  pub fn new() -> Self {
    IdentifyInputSegment {
      debug_output: false,
    }
  }

  fn print_identification<R: JoypadAddresses + RngAddresses + InputIdentificationAddresses>(gb: &mut Gb<R>, s: &State) {
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
impl<R: JoypadAddresses + RngAddresses + InputIdentificationAddresses> super::Segment<R> for IdentifyInputSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer {
    let mut goal_buffer = StateBuffer::new();
    for s in iter.into_iter() {
      Self::print_identification(gb, &s);
      goal_buffer.add_state(s);
    }
    goal_buffer
  }
}
