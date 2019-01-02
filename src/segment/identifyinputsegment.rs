use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct IdentifyInputSegment {
  debug_output: bool,
  buffer_size: usize,
}
impl WithDebugOutput for IdentifyInputSegment {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}
impl WithOutputBufferSize for IdentifyInputSegment {
  fn with_buffer_size(mut self, buffer_size: usize) -> Self { self.buffer_size = buffer_size; self }
}
impl Default for IdentifyInputSegment {
  fn default() -> Self {
    IdentifyInputSegment {
      debug_output: false,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
}
impl IdentifyInputSegment {
  pub fn new() -> Self { Default::default() }

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
impl<R: JoypadAddresses + RngAddresses + InputIdentificationAddresses> Segment<R> for IdentifyInputSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer {
    let mut goal_buffer = StateBuffer::with_max_size(self.buffer_size);
    for s in iter.into_iter() {
      Self::print_identification(gb, &s);
      goal_buffer.add_state(s);
    }
    goal_buffer
  }
}

impl<R: Rom + InputIdentificationAddresses> ParallelSegment<R> for IdentifyInputSegment {
  type Key = ();

  fn execute_parallel<I: IntoIterator<Item=State>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    gbe.execute(iter, move |gb, s, tx| {
      Self::print_identification(gb, &s);
      tx.send(((), s)).unwrap();
    }).into_state_buffer_map(self.buffer_size)
  }
}
