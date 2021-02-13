use crate::metric::*;
use crate::metric::overworld::gen1::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct OverworldMoveSegment {
  input: Input,
  result: OverworldInteractionResult,
  buffer_size: usize,
}
impl OverworldMoveSegment {
  pub fn auto_walk(input: Input) -> Self {
    Self {
      input: Input::empty(),
      result: OverworldInteractionResult::Walked { direction: input },
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn walk(input: Input) -> Self {
    Self {
      input,
      result: OverworldInteractionResult::Walked { direction: input },
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn turn(input: Input) -> Self {
    Self {
      input,
      result: OverworldInteractionResult::Turned { direction: input },
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn collide(input: Input) -> Self {
    Self {
      input,
      result: OverworldInteractionResult::Collision,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn jump_ledge(input: Input) -> Self {
    Self {
      input,
      result: OverworldInteractionResult::JumpLedge { direction: input },
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn wait() -> Self {
    Self {
      input: Input::empty(),
      result: OverworldInteractionResult::NoAction,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn warp() -> Self {
    Self {
      input: Input::empty(),
      result: OverworldInteractionResult::FlyWarpOrDungeonWarp,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn open_main_menu() -> Self {
    Self {
      input: Input::START,
      result: OverworldInteractionResult::DisplayText { id: 0 },
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn interact_with(id: u8) -> Self {
    Self {
      input: Input::A,
      result: OverworldInteractionResult::DisplayText { id },
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
}
impl WithOutputBufferSize for OverworldMoveSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + Gen1OverworldAddresses + Gen1DVAddresses> Segment<R> for OverworldMoveSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    MoveSegment::with_metric(self.input, OverworldInteractionMetric {}.assert_eq(self.result)).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
  }
}
