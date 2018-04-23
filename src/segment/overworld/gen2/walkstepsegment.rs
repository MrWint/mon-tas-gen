use gambatte::Input;
use gb::*;
use rom::*;
use segment::WithDebugOutput;
use statebuffer::StateBuffer;
use std::marker::PhantomData;
use super::OverworldInteractionResult;

pub struct WalkStepSegment<T> {
  input: Input,
  into_result: OverworldInteractionResult,
  max_skips: u32,
  debug_output: bool,
  _rom: PhantomData<T>,
}
impl <T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> WalkStepSegment<T> {
  #[allow(dead_code)]
  pub fn new(input: Input) -> Self {
    Self {
      input: input,
      into_result: OverworldInteractionResult::NoEvents,
      max_skips: 0,
      debug_output: false,
      _rom: PhantomData,
    }
  }
  #[allow(dead_code)]
  pub fn into(mut self, into_result: OverworldInteractionResult) -> Self { self.into_result = into_result; self }
  pub fn with_max_skips(mut self, max_skips: u32) -> Self { self.max_skips = max_skips; self }
}
impl<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> WithDebugOutput for WalkStepSegment<T> {
  fn with_debug_output(mut self, debug_output: bool) -> Self { self.debug_output = debug_output; self }
}

impl<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses> ::segment::Segment for WalkStepSegment<T> {
  type Rom = T;

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
        if skips >= self.max_skips || facing_dir != self.input { break; }
        gb.restore(&s);
        gb.input(Input::empty());
        gb.step();
        s = gb.save();
        skips += 1;
      }
      result
    }).collect()
  }
}

pub fn walk_step_check<T: JoypadAddresses + RngAddresses + Gen2MapEventsAddresses>(gb: &mut Gb<T>, into_result: &OverworldInteractionResult) -> bool {
  let result = super::get_overworld_interaction_result(gb);
  if let OverworldInteractionResult::Walked(_) = result {
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
