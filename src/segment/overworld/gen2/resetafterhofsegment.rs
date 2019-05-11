use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct ResetAfterHofSegment {
  input: Input,
  buffer_size: usize,
}
impl ResetAfterHofSegment {
  #[allow(dead_code)]
  pub fn new(input: Input) -> Self {
    Self {
      input,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
}
impl WithOutputBufferSize for ResetAfterHofSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + HallOfFameAddresses> Segment<R> for ResetAfterHofSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    gbe.execute(sb, move |gb, s, tx| {
      gb.restore(&s);
      gb.input(self.input);
      if gb.step_until(&[R::HALL_OF_FAME_AFTER_SAVING_ADDRESS]) == 0 {
        log::warn!("ResetAfterHofSegment: didn't find HOF saving address, dropping state!");
        return;
      }
      gb.soft_reset();
      gb.step();
      tx.send(gb.save()).unwrap();
    }).into_state_buffer_map(self.buffer_size)
  }
}
