use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct VerifyInputSegment {
  buffer_size: usize,
  expected_input_str: &'static str,
  input: Input,
}
impl WithOutputBufferSize for VerifyInputSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl VerifyInputSegment {
  pub fn new(expected_input_str: &'static str) -> Self {
    VerifyInputSegment {
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      expected_input_str,
      input: Input::empty(),
    }
  }
  #[allow(dead_code)]
  pub fn with_input(self, input: Input) -> Self { Self { input, ..self } }
}

impl<R: Rom + InputIdentificationAddresses> Segment<R> for VerifyInputSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let &(pre, input, post, _) = R::II_ADDRESSES.iter().find(|(_, _, _, name)| name == &self.expected_input_str).unwrap();
    gbe.execute(sb, move |gb, s, tx| {
      gb.restore(&s);
      gb.input(self.input);
      if !super::is_correct_input_use(gb, pre, input, post) {
        log::error!("VerifyInputSegment: Input is not {} as expected", self.expected_input_str);
      } else {
        gb.step();
        tx.send(gb.save()).unwrap();
      }
    }).into_state_buffer_map(self.buffer_size)
  }
}
