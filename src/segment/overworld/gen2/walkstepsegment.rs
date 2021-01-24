use crate::gb::*;
use crate::metric::overworld::gen2::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;
use log::debug;

pub struct WalkStepSegment {
  input: Input,
  into_result: OverworldInteractionResult,
  max_skips: u32,
  buffer_size: usize,
}
impl WalkStepSegment {
  #[allow(dead_code)]
  pub fn new(input: Input) -> Self {
    Self {
      input,
      into_result: OverworldInteractionResult::NoEvents,
      max_skips: 0,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  #[allow(dead_code)]
  pub fn into(self, into_result: OverworldInteractionResult) -> Self { Self { into_result, ..self } }
  #[allow(dead_code)]
  pub fn with_max_skips(self, max_skips: u32) -> Self { Self { max_skips, ..self } }
}
impl WithOutputBufferSize for WalkStepSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + Gen2MapEventsAddresses> crate::segment::Segment<R> for WalkStepSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    gbe.execute(sb, move |gb, mut s, tx| {
      let mut skips = 0;
      loop {
        gb.restore(&s);
        let facing_dir = match gb.gb.read_memory(R::PLAYER_DIRECTION_MEM_ADDRESS) & 0b1100 {
          0x0 => Input::DOWN,
          0x4 => Input::UP,
          0x8 => Input::LEFT,
          0xc => Input::RIGHT,
          _ => panic!("got invalid direction"),
        };
        gb.input(self.input);
        if walk_step_check(gb, &self.into_result) {
          gb.restore(&s);
          gb.input(self.input);
          gb.step();
          tx.send(gb.save()).unwrap();
        }
        if skips >= self.max_skips || facing_dir != self.input { break; }
        gb.restore(&s);
        gb.input(Input::empty());
        gb.step();
        s = gb.save();
        skips += 1;
      }
    }).into_state_buffer_map(self.buffer_size)
  }
}

pub fn walk_step_check<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses>(gb: &mut Gb<T>, into_result: &OverworldInteractionResult) -> bool {
  let result = get_overworld_interaction_result(gb);
  if let OverworldInteractionResult::Walked(_, _) = result {
    gb.step();
    gb.input(Input::empty());
    let result = get_overworld_interaction_result(gb);
    if result != *into_result {
      debug!("WalkStepSegment into_result failed: {:?}", result);
      false
    } else {
      true
    }
  } else {
    debug!("WalkStepSegment walking failed: {:?}", result);
    false
  }
}
