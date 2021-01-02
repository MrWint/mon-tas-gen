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
  
    HJoy5State {
      last_input,
      hjoy7,
      frame_counter,
      hjoy6,
    }
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


#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct HandleMenuInputState {
  hjoy5: HJoy5State,
  current_item: u8,
  max_item: u8,
  watched_keys: Input,
  wrapping_enabled: bool,
  watch_moving_oob: bool,
  poll_count_instant_exit: bool,
}
impl HandleMenuInputState {
  pub fn unknown() -> HandleMenuInputState {
    HandleMenuInputState {
      hjoy5: HJoy5State::unknown(),
      current_item: 0,
      max_item: 0,
      watched_keys: Input::empty(),
      wrapping_enabled: false,
      watch_moving_oob: false,
      poll_count_instant_exit: false,
    }
  }
  pub fn from_gb_state<R: MultiRom + HandleMenuInputAddresses>(gb: &mut Gb<R>, state: &GbState) -> HandleMenuInputState {
    gb.restore(state);
    Self::from_gb(gb)
  }
  pub fn from_gb<R: MultiRom + HandleMenuInputAddresses>(gb: &mut Gb<R>) -> HandleMenuInputState {
    let hjoy5 = HJoy5State::from_gb(gb);
    let current_item = gb.gb.read_memory(R::CURRENT_MENU_ITEM_MEM_ADDRESS);
    let max_item = gb.gb.read_memory(R::MAX_MENU_ITEM_MEM_ADDRESS);
    let watched_keys = Input::from_bits_truncate(gb.gb.read_memory(R::MENU_WATCHED_KEYS_MEM_ADDRESS));
    let wrapping_enabled = gb.gb.read_memory(R::MENU_WRAPPING_ENABLED_MEM_ADDRESS) > 0;
    let watch_moving_oob = gb.gb.read_memory(R::MENU_WATCH_MOVING_OOB_MEM_ADDRESS) > 0;
    let poll_count_instant_exit = gb.gb.read_memory(R::MENU_JOYPAD_POLL_COUNT_MEM_ADDRESS) == 1;
  
    let res = HandleMenuInputState {
      hjoy5,
      current_item,
      max_item,
      watched_keys,
      wrapping_enabled,
      watch_moving_oob,
      poll_count_instant_exit,
    };
    log::trace!("HandleMenuInput state: {:?}", res);
    res
  }
  pub fn get_result(&self, input: Input) -> HandleMenuInputResult {
    let hjoy5 = self.hjoy5.get_hjoy5(input);
    let mut current_item = self.current_item;
    if hjoy5.is_empty() {
      if self.poll_count_instant_exit {
        return HandleMenuInputResult::Exit { current_item, input: Input::empty() };
      }
      return HandleMenuInputResult::DoNothing;
    }
    if hjoy5.contains(Input::UP) {
      if current_item > 0 {
        current_item -= 1;
      } else if self.wrapping_enabled {
        current_item = self.max_item;
      } else if self.watch_moving_oob {
        return HandleMenuInputResult::Exit { current_item, input: hjoy5 }
      }
    } else if hjoy5.contains(Input::DOWN) {
      if current_item < self.max_item {
        current_item += 1;
      } else if self.wrapping_enabled {
        current_item = 0;
      } else if self.watch_moving_oob {
        return HandleMenuInputResult::Exit { current_item, input: hjoy5 }
      }
    }
    if hjoy5.intersects(self.watched_keys) {
      return HandleMenuInputResult::Exit { current_item, input: hjoy5, }
    }
    return HandleMenuInputResult::ScrollTo { current_item, }
  }
}
pub enum HandleMenuInputResult {
  Exit { current_item: u8, input: Input, },
  ScrollTo { current_item: u8, },
  DoNothing,
}