use crate::rom::*;
use gambatte::*;
use serde_derive::{Serialize, Deserialize};
use std::marker::PhantomData;

pub trait StateRef<V = ()> {
  fn to_state(self) -> State<V>;
  fn as_ref(&self) -> &State<V>;
}
impl<V> StateRef<V> for State<V> {
  fn to_state(self) -> State<V> { self }
  fn as_ref(&self) -> &State<V> { self }
}
impl<'a, V: Clone, S: StateRef<V>> StateRef<V> for &'a S {
  fn to_state(self) -> State<V> { let s: &State<V> = self.as_ref(); let s: State<V> = s.clone(); s }
  fn as_ref(&self) -> &State<V> { (*self).as_ref() }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct State<V = ()> {
  raw_state: std::sync::Arc<RawState>,
  // Additional associated value
  pub value: V,
}
impl<V> std::ops::Deref for State<V> {
    type Target = RawState;

    fn deref(&self) -> &RawState {
        &self.raw_state
    }
}
impl<V> State<V> {
  pub fn replace_value<NV>(self, value: NV) -> State<NV> {
    State {
      raw_state: self.raw_state,
      value,
    }
  }
  pub fn split_state_and_value(self) -> (State, V) {
    (State {
      raw_state: self.raw_state,
      value: (),
    }, self.value)
  }
}
/// Represents a saved state of a game execution.
#[derive(Serialize, Deserialize)]
pub struct RawState {
  /// Saved internal Gambatte state.
  gb_state: SaveState,
  /// List of all inputs performed so far.
  #[serde(skip_deserializing)]
  #[serde(skip_serializing)] 
  inputs_old: Vec<Input>,
  /// List of all non-null inputs with corresponding frame
  // #[serde(skip_deserializing)]
  pub inputs: Vec<(u32, Input)>,
  last_input_frame: [u32; 2], // first, last
  pub is_at_input: bool,
  ignored_inputs: Input, // inputs ignored by next input use

  // derived state for StateBuffer decisions
  pub rng_state: u32,
  pub cycle_count: u64,

  // additional semantic information and stats
  pub num_delays: u32,
}
impl RawState {
  /// Returns the D-Sum of this state.
  pub fn get_d_sum(&self) -> u8 {
    assert!(self.is_at_input, "Can't determine D-Sum of state which was not at a decision point.");
    (self.rng_state + (self.rng_state >> 8)) as u8
  }
  /// Returns the Div state of this state [0-3fff].
  pub fn get_div_state(&self) -> u16 {
    assert!(self.is_at_input, "Can't determine D-Sum of state which was not at a decision point.");
    (self.rng_state >> 16) as u16
  }
}

/// Object encapsulating an ongoing game execution.
/// It abstracts away the frame timing of Gambatte/BizHawk and operates on when inputs are read and used.
/// The game is executed by alternatingly using ```step``` to advance to the next decision point, and ```input``` to perform an input at this decision point.
/// A decision point is a point in the execution where a relevant input (i.e. an input which is later actually used to potentially change the execution flow) is read.
pub struct Gb<R> {
  /// Gambatte instance used for the emulation.
  pub gb: Gambatte,
  /// Saved initial internal Gambatte state, before any execution.
  initial_gambatte_state: SaveState,
  /// Whether relevant inputs were skipped over, making this inherently unsavable in any way.
  pub skipped_relevant_inputs: bool,
  _rom: PhantomData<R>,

  // Savable state variables
  /// List of all non-null inputs with corresponding frame
  inputs: Vec<(u32, Input)>,
  /// Frame (according to Gambatte's used timing) at which the last decision point occurred.
  /// Used to discard future inputs in the frame due to BizHawk's limit of only allowing one input per frame in its input format.
  last_input_frame: [u32; 2], // first, last
  /// Whether the execution is currently stopped at a decision point.
  pub is_at_input: bool,
  ignored_inputs: Input, // inputs ignored by next input use
  pub num_delays: u32,
}

impl <R: BasicRomInfo + JoypadAddresses> Gb<R> {
  /// Creates a new game execution using the given Gambatte instance.
  pub fn create<S: ScreenUpdateCallback + 'static>(equal_length_frames: bool, screen_update_callback: S) -> Self {
    let gambatte = Gambatte::create("roms/gbc_bios.bin", R::ROM_FILE_NAME, equal_length_frames, screen_update_callback);
    let initial_gambatte_state = gambatte.save_state();
    let mut pgb = Gb {
      gb: gambatte,
      initial_gambatte_state,
      skipped_relevant_inputs: false,
      _rom: PhantomData,

      inputs: vec![],
      last_input_frame: [0, 0],
      is_at_input: false,
      ignored_inputs: Input::empty(),
      num_delays: 0,
    };
    pgb.step(); // move to first decision point
    pgb
  }
}
impl <R: RngAddresses> Gb<R> {
  /// Saves the current execution state to a State object.
  pub fn save(&mut self) -> State { self.save_with_value(()) }
  pub fn save_with_value<V>(&mut self, value: V) -> State<V> {
    assert!(!self.skipped_relevant_inputs);
    State {
      raw_state: std::sync::Arc::new(RawState {
        // save inherent state
        gb_state: self.gb.save_state(),
        inputs_old: vec![],
        inputs: self.inputs.clone(),
        last_input_frame: self.last_input_frame,
        is_at_input: self.is_at_input,
        ignored_inputs: self.ignored_inputs,
        // save derived state
        cycle_count: self.gb.get_cycle_count(),
        rng_state: if self.is_at_input { self.get_rng_state() } else { 0 },
        num_delays: self.num_delays,
      }),
      // save associated value
      value,
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
}
impl <R: JoypadAddresses> Gb<R> {
  /// Resets the current execution to the initial state.
  pub fn restore_initial_state(&mut self) {
    self.gb.load_state(&self.initial_gambatte_state);
    self.skipped_relevant_inputs = false;
    self.inputs.clear();
    self.last_input_frame = [0, 0];
    self.is_at_input = false;
    self.ignored_inputs = Input::empty();
    self.num_delays = 0;
    self.step(); // move to first decision point
  }

  pub fn delay(&mut self) {
    self.num_delays += 1;
    self.input(Input::empty());
    self.step();
  }

  /// Performs an input at a decision point.
  pub fn input(&mut self, input: Input) {
    assert!(self.is_at_input);
    if input.intersects(self.ignored_inputs) {
      log::warn!("Part of inputs {:?} are ignored by mask {:?}", input, self.ignored_inputs);
    }
    self.gb.set_input(input);
    self.gb.run_until(&[R::JOYPAD_READ_LOCKED_ADDRESS]);
    Self::record_input(&mut self.inputs, self.last_input_frame[0] - 1, input & if R::JOYPAD_READ_FIRST_ADDRESS == R::JOYPAD_READ_HI_ADDRESS { inputs::HI_INPUTS } else { inputs::LO_INPUTS });
    Self::record_input(&mut self.inputs, self.last_input_frame[1] - 1, input & if R::JOYPAD_READ_LAST_ADDRESS == R::JOYPAD_READ_HI_ADDRESS { inputs::HI_INPUTS } else { inputs::LO_INPUTS });
    self.is_at_input = false;
  }
  fn record_input(inputs: &mut Vec<(u32, Input)>, frame: u32, input: Input) {
    if input.is_empty() { return; }
    if let Some((last_frame, last_input)) = inputs.last_mut() {
      assert!(*last_frame <= frame);
      if *last_frame == frame {
        *last_input |= input;
        return;
      }
    }
    inputs.push((frame, input));
  }

  /// Executes until any of the given addresses are about to be executed, or until the execution is about to read its next usable input.
  /// Returns the address which was hit (and stops the execution at this address), or a saved internal Gambatte state at the next usable input
  /// and the frames at which the two halves of the input were read (the running execution is stopped at reading the second half of the input).
  fn run_to_next_vblank_until(&mut self, addresses: &[i32]) -> RunToNextVBlankResult {
    loop {
      let hit = self.gb.run_until(&[&[R::JOYPAD_READ_FIRST_ADDRESS], addresses].concat());
      if addresses.contains(&hit) { return RunToNextVBlankResult::HitAddress(hit); }
      let input_first_address_frame = self.gb.frame();
      if input_first_address_frame == self.last_input_frame[0] {
          log::warn!("found multiple first inputs in frame {}, skipping possible input", self.last_input_frame[0]);
          self.gb.run_until(&[R::JOYPAD_READ_LOCKED_ADDRESS]);
          continue;
      }
      let state_at_input = self.gb.save_state();
      self.gb.run_until(&[R::JOYPAD_READ_LAST_ADDRESS]);
      let input_last_address_frame = self.gb.frame();
      if input_last_address_frame == self.last_input_frame[1] {
          log::warn!("found multiple last inputs in frame {}, skipping possible input", self.last_input_frame[1]);
          self.gb.run_until(&[R::JOYPAD_READ_LOCKED_ADDRESS]);
          continue;
      }
      return RunToNextVBlankResult::AtNextVBlank(state_at_input, [input_first_address_frame, input_last_address_frame]);
    }
  }
  /// Assumes the current execution is stopped at a usable input by ```run_to_next_vblank_until```,
  /// checks whether the input is actually used and skips over inputs that don't affect the execution path.
  /// Executes until any of the given addresses are about to be executed, returning the hit address, or until the next decision point is reached, returning 0.
  fn skip_irrelevant_vblanks_until(&mut self, addresses: &[i32], mut state_at_input: SaveState, mut input_frame: [u32; 2], allow_hit_after_relevant_input_read: bool) -> i32 {
    loop {
      self.gb.run_until(&[R::JOYPAD_READ_LOCKED_ADDRESS]);

      let mut hit_any_addresses = false;
      let mut hit;
      'check_for_input_uses_until_next_input: loop {
        self.ignored_inputs = Input::empty();
        hit = if hit_any_addresses {
          self.gb.run_until(&[&[R::JOYPAD_READ_FIRST_ADDRESS], R::JOYPAD_USE_ADDRESSES].concat())
        } else {
          self.gb.run_until(&[&[R::JOYPAD_READ_FIRST_ADDRESS], R::JOYPAD_USE_ADDRESSES, addresses].concat())
        };
        if !hit_any_addresses && addresses.contains(&hit) {
          // Hit an address between input read and use, but still need to check whether the read input is relevant, since it came first.
          hit_any_addresses = true;
        } else if hit == R::JOYPAD_READ_FIRST_ADDRESS {
          // Didn't find any use of the last read input.
          break;
        } else { // R::JOYPAD_USE_ADDRESSES
          // Found a potential use of the last read input. Check whether the use actually can change the execution flow.
          for &(use_add, ignore_mask_mem_add, skip_add) in R::JOYPAD_USE_IGNORE_MASK_MEM_ADDRESSES {
            if hit == use_add {
              self.ignored_inputs = Input::from_bits_truncate(self.gb.read_memory(ignore_mask_mem_add));
              if self.ignored_inputs.bits() == 0xff { // discard if all inputs ignored
                self.gb.run_until(&[skip_add]);
                continue 'check_for_input_uses_until_next_input;
              }
              break;
            }
          }
          for &(use_add, keep_add, discard_add) in R::JOYPAD_USE_DISCARD_ADDRESSES {
            if hit == use_add {
              if self.gb.run_until(&[keep_add, discard_add]) == discard_add { continue 'check_for_input_uses_until_next_input; }
              break;
            }
          }
          // Found use can potentially change the execution flow, so the last read input is a decision point.
          break;
        }
      }

      if hit_any_addresses && (allow_hit_after_relevant_input_read || hit == R::JOYPAD_READ_FIRST_ADDRESS) {
        self.skipped_relevant_inputs |= hit != R::JOYPAD_READ_FIRST_ADDRESS; // If the hit happened after reading a relevant input (but before using it), this execution is now unsavable.
        self.gb.load_state(&state_at_input);
        return self.gb.run_until(addresses);
      } else if hit != R::JOYPAD_READ_FIRST_ADDRESS {
        // Found a decision point.
        self.gb.load_state(&state_at_input);
        self.last_input_frame = input_frame;
        self.is_at_input = true;
        return 0;
      }
      // Didn't hit any address and the last read input turned out to be irrelevant.
      state_at_input = self.gb.save_state();
      input_frame[0] = self.gb.frame();
      self.gb.run_until(&[R::JOYPAD_READ_LAST_ADDRESS]);
      input_frame[1] = self.gb.frame();
    }
  }

  /// Resumes the execution until the next decision point is reached.
  pub fn step(&mut self) {
    assert!(!self.is_at_input);
    match self.run_to_next_vblank_until(&[]) {
      RunToNextVBlankResult::HitAddress(hit) => hit,
      RunToNextVBlankResult::AtNextVBlank(s, input_frame) => self.skip_irrelevant_vblanks_until(&[], s, input_frame, /* allow_hit_after_relevant_input_read = */ false),
    };
  }
  /// Runs until any of the given addresses is hit, or the next decision point is reached.
  pub fn step_until(&mut self, addresses: &[i32]) -> i32 {
    assert!(!self.is_at_input);
    match self.run_to_next_vblank_until(addresses) {
      RunToNextVBlankResult::HitAddress(hit) => hit,
      RunToNextVBlankResult::AtNextVBlank(s, input_frame) => self.skip_irrelevant_vblanks_until(addresses, s, input_frame, /* allow_hit_after_relevant_input_read = */ false),
    }
  }
  /// Runs until any of the given addresses is hit, or the next relevant input is being used.
  /// This operation can render the execution unsavable if the input use happens between the reading and the use of the next input.
  /// This operation is useful to evalutate metrics as the result of pressing an input, which happen before the next input is used.
  pub fn run_until_or_next_input_use(&mut self, addresses: &[i32]) -> i32 {
    assert!(!self.is_at_input);
    match self.run_to_next_vblank_until(addresses) {
      RunToNextVBlankResult::HitAddress(hit) => hit,
      RunToNextVBlankResult::AtNextVBlank(s, input_frame) => self.skip_irrelevant_vblanks_until(addresses, s, input_frame, /* allow_hit_after_relevant_input_read = */ true),
    }
  }

  /// Issues soft reset inputs (A+B+START+SELECT) at the the next possible vblank (not necessarily a decision point).
  pub fn soft_reset(&mut self) {
    if !self.is_at_input {
      match self.run_to_next_vblank_until(&[]) {
        RunToNextVBlankResult::HitAddress(hit) => panic!("unexpected hit of address {:x}", hit),
        RunToNextVBlankResult::AtNextVBlank(s, input_frame) => {
          self.gb.load_state(&s);
          self.last_input_frame = input_frame;
          self.is_at_input = true;
        },
      };
    }
    self.input(inputs::LO_INPUTS);
  }

  // Restores a saved execution state object.
  pub fn restore<V>(&mut self, s: &State<V>) {
    // load or reconstruct inputs
    if s.inputs.is_empty() && !s.inputs_old.iter().all(Input::is_empty) {
      log::info!("legacy state loaded, reconstructing inputs...");
      self.inputs = self.create_inputs_from_ftii(&s.inputs_old);
      log::info!("successfully reconstructed {} inputs", self.inputs.len());
    } else {
      self.inputs.clone_from(&s.inputs);
    }

    // load inherent state
    self.gb.load_state(&s.gb_state);
    self.skipped_relevant_inputs = false;
    self.last_input_frame.clone_from(&s.last_input_frame);
    self.is_at_input = s.is_at_input;
    self.ignored_inputs = s.ignored_inputs;
    self.num_delays = s.num_delays;
  }

  #[allow(dead_code)]
  pub fn create_inputs(&mut self) -> Vec<Input> {
    assert!(!self.skipped_relevant_inputs);

    let result = self.get_inputs();
    #[cfg(feature = "gambatte-track-inputs")] {
      let gb_result = self.gb.get_inputs();
      if gb_result != result {
        log::error!("Input diff detected! Likely desync. Possibly caused by manual memory writes");
      }
    }
    log::info!("creating inputs done: #inputs: {}", result.len());
    result
  }
  pub fn get_inputs(&self) -> Vec<Input> {
    let mut result = vec![];
    for &(frame, input) in self.inputs.iter() {
      assert!(result.len() <= frame as usize);
      result.resize((frame + 1) as usize, Input::empty());
      result[frame as usize] = input;
    }
    result
  }

  #[allow(dead_code)]
  pub fn create_inputs_from_ftii(&mut self, inputs: &[Input]) -> Vec<(u32, Input)> {
    self.restore_initial_state();

    let mut result: Vec<(u32, Input)> = vec![];
    for &input in inputs.iter() {
      self.gb.set_input(input);
      Self::record_input(&mut result, self.gb.frame() - 1, input & if R::JOYPAD_READ_FIRST_ADDRESS == R::JOYPAD_READ_HI_ADDRESS { inputs::HI_INPUTS } else { inputs::LO_INPUTS });
      self.gb.run_until(&[R::JOYPAD_READ_LAST_ADDRESS]);
      Self::record_input(&mut result, self.gb.frame() - 1, input & if R::JOYPAD_READ_LAST_ADDRESS == R::JOYPAD_READ_HI_ADDRESS { inputs::HI_INPUTS } else { inputs::LO_INPUTS });
      self.gb.run_until(&[R::JOYPAD_READ_LOCKED_ADDRESS]);
      self.is_at_input = false;
      self.step();
    }
    result
  }
}

enum RunToNextVBlankResult {
  HitAddress(i32),
  AtNextVBlank(SaveState, [u32; 2]),
}
