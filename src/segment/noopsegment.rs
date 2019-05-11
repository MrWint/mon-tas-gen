use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;

pub struct NoopSegment {
  buffer_size: usize,
}
impl WithOutputBufferSize for NoopSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl Default for NoopSegment {
  fn default() -> Self {
    NoopSegment {
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
}
impl NoopSegment {
  pub fn new() -> Self { Default::default() }
}

impl<R: Rom> Segment<R> for NoopSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    gbe.execute(sb, move |gb, s, tx| {
      gb.restore(&s);
      tx.send(gb.save()).unwrap();
    }).into_state_buffer_map(self.buffer_size)
  }
}
