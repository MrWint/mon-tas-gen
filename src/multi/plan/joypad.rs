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
      last_input: if gb.is_pressed_always_cleared() { Input::all() } else { Input::from_bits_truncate(gb.gb.read_memory(R::JOYPAD_LAST_MEM_ADDRESS)) },
    }
  }
  pub fn get_pressed_input(&self, input: Input) -> Input {
    input - self.last_input
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct HJoy5State {
  pressed: PressedInputState,
  hjoy7: bool,
  frame_counter: bool,
  hjoy6: bool,
}
impl HJoy5State {
  pub fn unknown() -> HJoy5State {
    HJoy5State {
      pressed: PressedInputState::unknown(),
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
    let pressed = PressedInputState::from_gb(gb);
    let hjoy7 = gb.gb.read_memory(R::JOYPAD_HJOY7_MEM_ADDRESS) > 0;
    let hit = gb.step_until(&[R::JOYPAD_FRAME_COUNTER_CHECK_ADDRESS]);
    assert!(hit == R::JOYPAD_FRAME_COUNTER_CHECK_ADDRESS, "Not a JoypadLowSensitivity input");
    let frame_counter = gb.gb.read_memory(R::JOYPAD_FRAME_COUNTER_MEM_ADDRESS) > 0;
    let hjoy6 = gb.gb.read_memory(R::JOYPAD_HJOY6_MEM_ADDRESS) > 0;
  
    HJoy5State {
      pressed,
      hjoy7,
      frame_counter,
      hjoy6,
    }
  }
  pub fn get_pressed_input(&self, input: Input) -> Input {
    self.pressed.get_pressed_input(input)
  }
  pub fn get_hjoy5(&self, input: Input) -> Input {
    let pressed_input = self.pressed.get_pressed_input(input);
    if !self.hjoy7 { return pressed_input; }
    if !pressed_input.is_empty() { return input; }
    if self.frame_counter { return Input::empty(); }
    if !input.intersects(Input::A | Input::B) { return input; }
    if self.hjoy6 { input } else { Input::empty() }
  }
}


#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct HandleMenuInputState {
  hjoy5: HJoy5State,
  pub current_item: u8,
  pub max_item: u8,
  watched_keys: Input,
  wrapping_enabled: bool,
  watch_moving_oob: bool,
  poll_count_instant_exit: bool,
  a_button_priority: bool,
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
      a_button_priority: false,
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
    let a_button_priority = R::MENU_A_BUTTON_PRIORITY;
  
    let res = HandleMenuInputState {
      hjoy5,
      current_item,
      max_item,
      watched_keys,
      wrapping_enabled,
      watch_moving_oob,
      poll_count_instant_exit,
      a_button_priority,
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
    if self.a_button_priority && hjoy5.contains(Input::A) {
      // Skip directional key checks and go to watched keys check directly.
    } else if hjoy5.contains(Input::UP) {
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

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct JoypadOverworldState {
  pressed: PressedInputState,
  forced_downwards: bool, // wFlags_D733 bit 3 or wCurMap
  simulating_button_presses: bool, // wd730 bit 7
  simulating_button_presses_override_mask: Input, // wOverrideSimulatedJoypadStatesMask
  simulated_button_press: Option<Input>, // done simulating if None
}

impl JoypadOverworldState {
  pub fn unknown() -> Self {
    Self {
      pressed: PressedInputState::unknown(),
      forced_downwards: false,
      simulating_button_presses: false,
      simulating_button_presses_override_mask: Input::empty(),
      simulated_button_press: None,
    }
  }
  pub fn from_gb_state<R: MultiRom + JoypadOverworldAddresses>(gb: &mut Gb<R>, state: &GbState) -> Self {
    gb.restore(state);
    Self::from_gb(gb)
  }
  pub fn from_gb<R: MultiRom + JoypadOverworldAddresses>(gb: &mut Gb<R>) -> Self {
    let pressed = PressedInputState::from_gb(gb);
    let forced_downwards = (gb.gb.read_memory(R::FLAGS_D733_MEM_ADDRESS) & 0b1000) == 0 && gb.gb.read_memory(R::CUR_MAP_MEM_ADDRESS) == 28;
    let simulating_button_presses = (gb.gb.read_memory(R::FLAGS_D730_MEM_ADDRESS) & 0b1000_0000) != 0;
    let simulating_button_presses_override_mask = Input::from_bits_truncate(gb.gb.read_memory(R::SIMULATED_JOYPAD_OVERRIDE_MASK_MEM_ADDRESS));
    let simulated_button_index = gb.gb.read_memory(R::SIMULATED_JOYPAD_STATES_INDEX_MEM_ADDRESS);
    let simulated_button_press = if simulated_button_index == 0 { None } else {
      Some(Input::from_bits_truncate(gb.gb.read_memory(R::SIMULATED_JOYPAD_STATES_END_MEM_ADDRESS + u16::from(simulated_button_index - 1))))
    };
  
    let res = Self {
      pressed,
      forced_downwards,
      simulating_button_presses,
      simulating_button_presses_override_mask,
      simulated_button_press,
    };
    log::trace!("JoypadOverworld state: {:?}", res);
    res
  }
  pub fn get_input(&self, mut input: Input) -> (Input, Input) {
    let pressed = self.pressed.get_pressed_input(input);
    if self.forced_downwards && !input.intersects(Input::DOWN | Input::UP | Input::LEFT | Input::RIGHT | Input::B | Input::A) {
      input = Input::DOWN;
    }
    if !self.simulating_button_presses || input.intersects(self.simulating_button_presses_override_mask) { return (input, pressed); }
    if let Some(simulated_input) = self.simulated_button_press {
      if simulated_input.is_empty() {
        (Input::empty(), Input::empty())
      } else {
        (simulated_input, pressed)
      }
    } else {
      (Input::empty(), pressed)
    }
  }
}
