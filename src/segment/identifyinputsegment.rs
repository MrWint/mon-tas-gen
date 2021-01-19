use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct IdentifyInputSegment {
  buffer_size: usize,
}
impl WithOutputBufferSize for IdentifyInputSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl Default for IdentifyInputSegment {
  fn default() -> Self {
    IdentifyInputSegment {
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
}

pub fn get_input_identification<R: JoypadAddresses + RngAddresses + InputIdentificationAddresses>(gb: &mut Gb<R>, s: &State) -> Option<&'static str> {
  for &(pre, input, post, name) in R::II_ADDRESSES {
    gb.restore(s);
    gb.input(Input::empty());
    if super::is_correct_input_use(gb, pre, input, post) {
      return Some(name);
    }
  }
  return None;
}

impl IdentifyInputSegment {
  pub fn new() -> Self { Default::default() }

  fn print_identification<R: JoypadAddresses + RngAddresses + InputIdentificationAddresses>(gb: &mut Gb<R>, s: &State) {
    if let Some(name) = get_input_identification(gb, s) {
      log::info!("IdentifyInputSegment: Identified input as {}", name);
    } else {
      log::info!("IdentifyInputSegment: Input not identified");
    }
  }
}

impl<R: Rom + InputIdentificationAddresses> Segment<R> for IdentifyInputSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    gbe.execute(sb, move |gb, s, tx| {
      Self::print_identification(gb, &s);
      tx.send(s).unwrap();
    }).into_state_buffer_map(self.buffer_size)
  }
}
