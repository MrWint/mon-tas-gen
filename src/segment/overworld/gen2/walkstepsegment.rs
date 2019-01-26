use crate::gb::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;
use super::OverworldInteractionResult;

pub struct WalkStepSegment {
  input: Input,
  into_result: OverworldInteractionResult,
  max_skips: u32,
  buffer_size: usize,
  debug_output: bool,
}
impl WalkStepSegment {
  #[allow(dead_code)]
  pub fn new(input: Input) -> Self {
    Self {
      input,
      into_result: OverworldInteractionResult::NoEvents,
      max_skips: 0,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      debug_output: false,
    }
  }
  #[allow(dead_code)]
  pub fn into(mut self, into_result: OverworldInteractionResult) -> Self { self.into_result = into_result; self }
  #[allow(dead_code)]
  pub fn with_max_skips(mut self, max_skips: u32) -> Self { self.max_skips = max_skips; self }
}
impl WithOutputBufferSize for WalkStepSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl WithDebugOutput for WalkStepSegment {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}

impl<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> crate::segment::Segment<T> for WalkStepSegment {
  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<T>, iter: I) -> StateBuffer {
    iter.into_iter().flat_map(|mut s| {
      let mut result = vec![];
      let mut skips = 0;
      loop {
        gb.restore(&s);
        let facing_dir = match gb.gb.read_memory(T::PLAYER_DIRECTION_MEM_ADDRESS) & 0b1100 {
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
          result.push(gb.save());
        }
        if skips >= self.max_skips || facing_dir != self.input { break result; }
        gb.restore(&s);
        gb.input(Input::empty());
        gb.step();
        s = gb.save();
        skips += 1;
      }
    }).collect()
  }
}

impl<R: Rom + Gen2MapEventsAddresses> crate::segment::ParallelSegment<R> for WalkStepSegment {
  type Key = ();

  fn execute_parallel<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<R>>(&self, gbe: &mut E, iter: I) -> HashMap<Self::Key, StateBuffer> {
    gbe.execute(iter, move |gb, mut s, tx| {
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
          tx.send(((), gb.save())).unwrap();
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
  let result = super::get_overworld_interaction_result(gb);
  if let OverworldInteractionResult::Walked(_, _) = result {
    gb.step();
    gb.input(Input::empty());
    let result = super::get_overworld_interaction_result(gb);
    if result != *into_result {
      println!("WalkStepSegment into_result failed: {:?}", result); false
    } else {
      true
    }
  } else {
    println!("WalkStepSegment walking failed: {:?}", result); false
  }
}
