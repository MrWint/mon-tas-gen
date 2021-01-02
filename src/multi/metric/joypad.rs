use super::*;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PressedInputState {
  last_input: Input, // hJoyLast
}
impl PressedInputState {
  pub fn unknown() -> PressedInputState {
    PressedInputState {
      last_input: Input::empty(),
    }
  }
  pub fn from_gb_state<R: Rom + JoypadLowSensitivityAddresses>(gb: &mut Gb<R>, state: &GbState) -> PressedInputState {
    gb.restore(state);
    Self::from_gb(gb)
  }
  pub fn from_gb<R: Rom + JoypadLowSensitivityAddresses>(gb: &mut Gb<R>) -> PressedInputState {
    PressedInputState {
      last_input: Input::from_bits_truncate(gb.gb.read_memory(R::JOYPAD_LAST_MEM_ADDRESS)),
    }
  }
  pub fn get_pressed_input(&self, input: Input) -> Input {
    input - self.last_input
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct HJoy5State {
  last_input: Input, // hJoyLast
  hjoy7: bool,
  frame_counter: bool,
  hjoy6: bool,
}
impl HJoy5State {
  pub fn unknown() -> HJoy5State {
    HJoy5State {
      last_input: Input::empty(),
      hjoy7: false,
      frame_counter: false,
      hjoy6: true,
    }
  }
  pub fn from_gb_state<R: Rom + JoypadLowSensitivityAddresses>(gb: &mut Gb<R>, state: &GbState) -> HJoy5State {
    gb.restore(state);
    Self::from_gb(gb)
  }
  pub fn from_gb<R: Rom + JoypadLowSensitivityAddresses>(gb: &mut Gb<R>) -> HJoy5State {
    gb.input(Input::empty());
    let last_input = Input::from_bits_truncate(gb.gb.read_memory(R::JOYPAD_LAST_MEM_ADDRESS));
    let hjoy7 = gb.gb.read_memory(R::JOYPAD_HJOY7_MEM_ADDRESS) > 0;
    let hit = gb.step_until(&[R::JOYPAD_FRAME_COUNTER_CHECK_ADDRESS]);
    assert!(hit == R::JOYPAD_FRAME_COUNTER_CHECK_ADDRESS, "Not a JoypadLowSensitivity input");
    let frame_counter = gb.gb.read_memory(R::JOYPAD_FRAME_COUNTER_MEM_ADDRESS) > 0;
    let hjoy6 = gb.gb.read_memory(R::JOYPAD_HJOY6_MEM_ADDRESS) > 0;
  
    let res = HJoy5State {
      last_input,
      hjoy7,
      frame_counter,
      hjoy6,
    };
    res
  }
  pub fn get_pressed_input(&self, input: Input) -> Input {
    input - self.last_input
  }
  pub fn get_hjoy5(&self, input: Input) -> Input {
    let pressed_input = input - self.last_input;
    if !self.hjoy7 { return pressed_input; }
    if !pressed_input.is_empty() { return input; }
    if self.frame_counter { return Input::empty(); }
    if !input.contains(Input::A | Input::B) { return input; }
    if self.hjoy6 { input } else { Input::empty() }
  }
}
