use crate::rom::*;
use gambatte::*;
use serde_derive::{Serialize, Deserialize};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize)]
pub struct GbState {
  /// Saved internal Gambatte state.
  gb_state: SaveState,
  // Current input frame number
  cur_input_frame: [u32; 2], // lo, hi
  input_use_address: i32,
  pressed_cleared: bool,
  ignored_inputs: Input, // inputs ignored by current input use

  // derived state for StateBuffer decisions
  pub rng_state: u32,
  pub blocked_inputs: Input,

  // additional semantic information and stats
  pub num_delays: u32,
}
impl GbState {
  /// Returns the D-Sum of this state.
  pub fn get_d_sum(&self) -> u8 {
    assert!(self.is_at_input(), "Can't determine D-Sum of state which was not at a decision point.");
    (self.rng_state + (self.rng_state >> 8)) as u8
  }
  /// Returns the Div state of this state [0-3fff].
  pub fn get_div_state(&self) -> u16 {
    assert!(self.is_at_input(), "Can't determine D-Sum of state which was not at a decision point.");
    (self.rng_state >> 16) as u16
  }
  #[inline]
  pub fn is_at_input(&self) -> bool {
    self.input_use_address != 0
  }
  #[inline]
  pub fn is_pressed_always_cleared(&self) -> bool {
    self.pressed_cleared
  }
  #[inline]
  pub fn get_input_frame_lo(&self) -> u32 {
    self.cur_input_frame[0]
  }
  #[inline]
  pub fn get_input_frame_hi(&self) -> u32 {
    self.cur_input_frame[1]
  }
  /// Returns an input that will perform identical to the given input, potentially with some ignored inputs removed.
  #[inline]
  pub fn remove_ignored_inputs(&self, input: Input) -> Input {
    assert!(self.is_at_input());
    input - self.ignored_inputs
  }

  pub fn num_delays(&self) -> u32 {
    self.num_delays
  }
}

/// Object encapsulating an ongoing game execution.
/// It abstracts away the frame timing of Gambatte/BizHawk and operates on when inputs are read and used.
/// The game is executed by alternatingly using ```step``` to advance to the next decision point, and ```input``` to perform an input at this decision point.
/// A decision point is a point in the execution where a relevant input (i.e. an input which is later actually used to potentially change the execution flow) is read.
pub struct Gb<R> {
  /// Gambatte instance used for the emulation.
  pub gb: Gambatte,
  _rom: PhantomData<R>,

  // Savable state variables
  /// Frame (according to Gambatte's used timing) at which the most recent joypad nybble read occurred.
  cur_input_frame: [u32; 2], // lo, hi
  /// Address of the current input use the execution is stopped at, or 0 if not at an input use.
  input_use_address: i32,
  pressed_cleared: bool, // Whether hJoyPressed will always be empty at the current input
  ignored_inputs: Input, // inputs ignored by current input use

  pub num_delays: u32,
}

impl <R: BasicRomInfo + JoypadAddresses> Gb<R> {
  /// Creates a new game execution using the given Gambatte instance.
  pub fn create<S: ScreenUpdateCallback + 'static>(equal_length_frames: bool, rtc_divisor_offset: i32, screen_update_callback: S) -> Self {
    let gambatte = Gambatte::create("roms/gbc_bios.bin", R::ROM_FILE_NAME, equal_length_frames, rtc_divisor_offset, screen_update_callback);
    let mut pgb = Gb {
      gb: gambatte,
      _rom: PhantomData,

      cur_input_frame: [0, 0],
      input_use_address: 0,
      pressed_cleared: false,
      ignored_inputs: Input::empty(),
      num_delays: 0,
    };
    pgb.step(); // move to first decision point
    pgb
  }
}
impl <R: RngAddresses + JoypadLowSensitivityAddresses> Gb<R> {
  /// Saves the current execution state to a State object.
  pub fn save(&self) -> GbState {
    GbState {
      // save inherent state
      gb_state: self.gb.save_state(),
      cur_input_frame: self.cur_input_frame,
      input_use_address: self.input_use_address,
      pressed_cleared: self.pressed_cleared,
      ignored_inputs: self.ignored_inputs,
      // save derived state
      blocked_inputs: if self.is_at_input() { Input::from_bits_truncate(self.gb.read_memory(R::JOYPAD_LAST_MEM_ADDRESS)) } else { Input::empty() },
      rng_state: if self.is_at_input() { self.get_rng_state() } else { 0 },
      num_delays: self.num_delays,
    }
  }
  /// Determines the RNG state at the current point of the execution, represented as a number in [0x0, 0x3fffffff].
  fn get_rng_state(&self) -> u32 {
    (u32::from(self.gb.read_div_state()) << 16) + u32::from(self.gb.read_memory_word_be(R::RNG_MEM_ADDRESS))
  }
}
impl <R> Gb<R> {
  /// Generates a stack trace of the current point in the game's execution, returning at most 40 values from the stack.
  /// Values may be return addresses or registers stored on the stack.
  pub fn get_stack_trace_string(&self) -> String {
    let cur_sp = self.gb.read_registers().sp;
    let mut stack_values = vec![];
    let mut sp = cur_sp;
    for _ in 0..40 {
      if sp & 0x1fff == 0x1fff { break; }
      stack_values.push(self.gb.read_memory_word_le(sp as u16));
      if sp & 0x1fff == 0x1ffe { break; }
      sp += 2;
    }
    format!("sp {:04x}, stack ({})", cur_sp, stack_values.into_iter().map(|v| format!("{:04X}", v)).collect::<Vec<String>>().join(" "))
  }
  #[inline]
  pub fn is_at_input(&self) -> bool {
    self.input_use_address != 0
  }
  #[inline]
  pub fn is_pressed_always_cleared(&self) -> bool {
    self.pressed_cleared
  }
  #[inline]
  pub fn get_input_frame_lo(&self) -> u32 {
    self.cur_input_frame[0]
  }
  #[inline]
  pub fn get_input_frame_hi(&self) -> u32 {
    self.cur_input_frame[1]
  }
}
impl <R: JoypadAddresses> Gb<R> {
  /// Performs an input at a decision point. Returns actual input pressed, in case some were ignored.
  pub fn input(&mut self, mut input: Input) {
    assert!(self.is_at_input());
    if input.intersects(self.ignored_inputs) {
      log::warn!("Part of inputs {:?} are ignored by mask {:?}, resulting in input {:?}", input, self.ignored_inputs, input - self.ignored_inputs);
      input -= self.ignored_inputs;
    }
    // Write input directly to HRAM. This ignores Joypad interrupts and only works when they are not enabled. Will cause desync otherwise.
    self.gb.write_memory(R::JOYPAD_INPUT_MEM_ADDRESS, input.bits());
    while self.gb.run_until(&[self.input_use_address + 2, R::JOYPAD_READ_FIRST_ADDRESS]) == R::JOYPAD_READ_FIRST_ADDRESS { self.handle_vblank(); } // skip single LDH instruction.
    self.input_use_address = 0;
    self.pressed_cleared = false;
  }

  fn handle_vblank(&mut self) {
    // println!("VBlank at {}", self.gb.frame() - 1);
    // In VBlank, update the most recent input frame and retry.
    self.cur_input_frame[if R::JOYPAD_READ_FIRST_ADDRESS == R::JOYPAD_READ_LO_ADDRESS { 0 } else { 1 }] = self.gb.frame() - 1;
    self.gb.run_until(&[R::JOYPAD_READ_LAST_ADDRESS]);
    self.cur_input_frame[if R::JOYPAD_READ_FIRST_ADDRESS == R::JOYPAD_READ_LO_ADDRESS { 1 } else { 0 }] = self.gb.frame() - 1;
  }

  /// Runs until any of the given addresses is hit, or the next decision point is reached.
  pub fn step_until(&mut self, addresses: &[i32]) -> i32 {
    assert!(!self.is_at_input());
    'check_for_input_uses: loop {
      let mut pressed_cleared = false;
      let hit = loop {
        let hit = self.gb.run_until(&[&[R::JOYPAD_READ_FIRST_ADDRESS], R::JOYPAD_USE_ADDRESSES, addresses].concat());
        if hit != R::JOYPAD_READ_FIRST_ADDRESS { break hit; }
        self.handle_vblank();
      };
      if addresses.contains(&hit) {
        // Hit one of the given addresses, so we're done.
        return hit;
      } else { // R::JOYPAD_USE_ADDRESSES
        // Found a potential use of the last read input. Check whether the use actually can change the execution flow.
        for &(use_add, ignore_mask_mem_add, skip_add) in R::JOYPAD_USE_IGNORE_MASK_MEM_ADDRESSES {
          if hit == use_add {
            self.ignored_inputs = Input::from_bits_truncate(self.gb.read_memory(ignore_mask_mem_add));
            if self.ignored_inputs.bits() == 0xff { // discard if all inputs ignored
              while self.gb.run_until(&[skip_add, R::JOYPAD_READ_FIRST_ADDRESS]) == R::JOYPAD_READ_FIRST_ADDRESS { self.handle_vblank(); }
              continue 'check_for_input_uses;
            }
            break;
          }
        }
        if let Some((ignore_input_counter_mem_add, ignore_flag_mem_add, ignore_flag_bit, calc_joy_pressed_add, check_ignore_flag_add, discard_add)) = R::JOYPAD_USE_DISCARD_ADDRESSES {
          let ignore_input_counter = self.gb.read_memory(ignore_input_counter_mem_add);
          let inputs_ignored = (self.gb.read_memory(ignore_flag_mem_add) >> ignore_flag_bit) & 1 != 0;
          if inputs_ignored {
            if ignore_input_counter != 1 { // no risk of VBlank interfering during Joypad calculation
              while self.gb.run_until(&[discard_add, R::JOYPAD_READ_FIRST_ADDRESS]) == R::JOYPAD_READ_FIRST_ADDRESS { self.handle_vblank(); }
              continue 'check_for_input_uses;
            }
            // VBlank might interfere and disable ingoring inputs before the check is made.
            let s = self.gb.save_state();
            if self.gb.run_until(&[calc_joy_pressed_add, R::JOYPAD_READ_FIRST_ADDRESS]) == calc_joy_pressed_add {
              if self.gb.run_until(&[check_ignore_flag_add, R::JOYPAD_READ_FIRST_ADDRESS]) == check_ignore_flag_add {
                // This input use is still full ignored
                while self.gb.run_until(&[discard_add, R::JOYPAD_READ_FIRST_ADDRESS]) == R::JOYPAD_READ_FIRST_ADDRESS { self.handle_vblank(); }
                continue 'check_for_input_uses;
              }
              // ignoring inputs is disabled before after hJoyPressed calculation was made, this input is usable but hJoyPressed will be cleared
              pressed_cleared = true;
            } // else: ignoring inputs is disabled before any calculations are made, this input is fully usable
            self.gb.load_state(&s);
          }
        }
        self.input_use_address = hit;
        self.pressed_cleared = pressed_cleared;
        return 0;
      }
    }
  }
  /// Resumes the execution until the next decision point is reached.
  pub fn step(&mut self) {
    assert!(!self.is_at_input());
    self.step_until(&[]);
  }
  // proceeds to the next decision point and count the previous input as delay
  pub fn delay_step(&mut self) {
    let previous_input_frame = *self.cur_input_frame[..].iter().max().unwrap();
    self.step();
    self.num_delays += *self.cur_input_frame[..].iter().max().unwrap() - previous_input_frame;
  }

  // Restores a saved execution state object.
  pub fn restore(&mut self, s: &GbState) {
    // load inherent state
    self.gb.load_state(&s.gb_state);
    self.cur_input_frame.clone_from(&s.cur_input_frame);
    self.input_use_address = s.input_use_address;
    self.pressed_cleared = s.pressed_cleared;
    self.ignored_inputs = s.ignored_inputs;
    self.num_delays = s.num_delays;
  }
}

impl<R: JoypadAddresses> crate::metric::GbI<R> for Gb<R> {
  fn step_until(&mut self, addresses: &[i32]) -> i32 {
    self.step_until(addresses)
  }

  fn gb(&self) -> &Gambatte {
    &self.gb
  }
}
