use gambatte::*;
use rom::*;
use std::marker::PhantomData;

/// Represents a saved state of a game execution.
#[derive(Serialize, Deserialize)]
pub struct State {
  /// Saved internal Gambatte state.
  gb_state: Vec<u8>,
  /// List of all inputs performed so far.
  inputs: Vec<Input>,
  last_input_frame: [u32; 2], // first, last
  pub is_at_input: bool,
  ignored_inputs: Input, // inputs ignored by next input use

  // derived state for StateBuffer decisions
  pub rng_state: u32,
  pub cycle_count: u64,
}
impl State {
  /// Returns the D-Sum of this state.
  pub fn get_d_sum(&self) -> u8 {
    assert!(self.is_at_input, "Can't determine D-Sum of state which was not at a decision point.");
    (self.rng_state + (self.rng_state >> 8)) as u8
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
  initial_gambatte_state: Vec<u8>,
  /// Whether relevant inputs were skipped over, making this inherently unsavable in any way.
  pub skipped_relevant_inputs: bool,
  _rom: PhantomData<R>,

  // Savable state variables
  /// List of all inputs performed so far at past decision points.
  inputs: Vec<Input>,
  /// Frame (according to Gambatte's used timing) at which the last decision point occurred.
  /// Used to discard future inputs in the frame due to BizHawk's limit of only allowing one input per frame in its input format.
  last_input_frame: [u32; 2], // first, last
  /// Whether the execution is currently stopped at a decision point.
  pub is_at_input: bool,
  ignored_inputs: Input, // inputs ignored by next input use
}

impl <R: BasicRomInfo + JoypadAddresses> Gb<R> {
  /// Creates a new game execution using the given Gambatte instance.
  pub fn create(mut gambatte: Gambatte) -> Self {
    gambatte.load_gbc_bios("roms/gbc_bios.bin");
    gambatte.load_rom(R::ROM_NAME);
    let initial_gambatte_state = gambatte.save_state();
    let mut pgb = Gb {
      gb: gambatte,
      initial_gambatte_state: initial_gambatte_state,
      skipped_relevant_inputs: false,
      _rom: PhantomData,

      inputs: vec![],
      last_input_frame: [0, 0],
      is_at_input: false,
      ignored_inputs: Input::empty(),
    };
    pgb.step(); // move to first decision point
    pgb
  }
}
impl <R: RngAddresses> Gb<R> {
  /// Saves the current execution state to a State object.
  pub fn save(&self) -> State {
    assert!(!self.skipped_relevant_inputs);
    State {
      // save inherent state
      gb_state: self.gb.save_state(),
      inputs: self.inputs.clone(),
      last_input_frame: self.last_input_frame.clone(),
      is_at_input: self.is_at_input,
      ignored_inputs: self.ignored_inputs,
      // save derived state
      cycle_count: self.gb.get_cycle_count(),
      rng_state: if self.is_at_input { self.get_rng_state() } else { 0 },
    }
  }
  /// Determines the RNG state at the current point of the execution, represented as a number in [0x0, 0x3fffffff].
  fn get_rng_state(&self) -> u32 {
    ((self.gb.read_div_state() as u32) << 16) + self.gb.read_memory_word_be(R::RNG_MEM_ADDRESS) as u32
  }
}
impl <R> Gb<R> {
  // Restores a saved execution state object.
  pub fn restore(&mut self, s: &State) {
    // load inherent state
    self.gb.load_state(&s.gb_state);
    self.skipped_relevant_inputs = false;
    self.inputs.clone_from(&s.inputs);
    self.last_input_frame.clone_from(&s.last_input_frame);
    self.is_at_input = s.is_at_input;
    self.ignored_inputs = s.ignored_inputs;
  }
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
    self.step(); // move to first decision point
  }

  /// Performs an input at a decision point.
  pub fn input(&mut self, input: Input) {
    assert!(self.is_at_input);
    if input.intersects(self.ignored_inputs) {
      println!("WARNING: part of inputs {:?} are ignored by mask {:?}", input, self.ignored_inputs);
    }
    self.gb.set_input(input);
    self.gb.run_until(&[R::JOYPAD_READ_LOCKED_ADDRESS]);
    self.inputs.push(input);
    self.is_at_input = false;
  }

  /// Resumes the execution until the next decision point is reached.
  pub fn step(&mut self) {
    self.step_until_or_any_vblank(&[]); // slightly more performant than step_until
  }
  /// Executes until any of the given addresses are about to be executed, or until the execution is about to read its next usable input.
  /// Returns the address which was hit (and stops the execution at this address), or a saved internal Gambatte state at the next usable input
  /// and the frames at which the two halves of the input were read (the running execution is stopped at reading the second half of the input).
  fn run_to_next_vblank_until(&mut self, addresses: &[i32]) -> (i32, Vec<u8>, [u32; 2]) {
    loop {
      let hit = self.gb.run_until(&[&[R::JOYPAD_READ_FIRST_ADDRESS], addresses].concat());
      if hit != R::JOYPAD_READ_FIRST_ADDRESS { return (hit, vec![], [0, 0]); }
      let input_first_address_frame = self.gb.frame();
      if input_first_address_frame == self.last_input_frame[0] {
          println!("found multiple first inputs in frame {}, skipping possible input", self.last_input_frame[0]);
          self.gb.run_until(&[R::JOYPAD_READ_LOCKED_ADDRESS]);
          continue;
      }
      let state_at_input = self.gb.save_state();
      self.gb.run_until(&[R::JOYPAD_READ_LAST_ADDRESS]);
      let input_last_address_frame = self.gb.frame();
      if input_last_address_frame == self.last_input_frame[1] {
          println!("found multiple last inputs in frame {}, skipping possible input", self.last_input_frame[1]);
          self.gb.run_until(&[R::JOYPAD_READ_LOCKED_ADDRESS]);
          continue;
      }
      return (0, state_at_input, [input_first_address_frame, input_last_address_frame]);
    }
  }
  /// Assumes the current execution is stopped at a usable input by ```run_to_next_vblank_until```,
  /// checks whether the input is actually used and skips over inputs that don't affect the execution path.
  /// Executes until any of the given addresses are about to be executed, returning the hit address, or until the next decision point is reached, returning 0.
  fn skip_irrelevant_vblanks_until(&mut self, addresses: &[i32], mut state_at_input: Vec<u8>, mut input_frame: [u32; 2], allow_hit_after_relevant_input_read: bool) -> i32 {
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
  /// Runs until any of the given addresses is hit, or the next decision point is reached.
  pub fn step_until(&mut self, addresses: &[i32]) -> i32 {
    assert!(!self.is_at_input);
    let (hit, s, input_frame) = self.run_to_next_vblank_until(addresses);
    if hit != 0 { return hit; }
    self.skip_irrelevant_vblanks_until(addresses, s, input_frame, /* allow_hit_after_relevant_input_read = */ false)
  }
  /// Runs until any of the given addresses is hit, or any new input is read (whether or not it is a relevant input) and forwards to relevant input.
  pub fn step_until_or_any_vblank(&mut self, addresses: &[i32]) -> i32 {
    assert!(!self.is_at_input);
    let (hit, s, input_frame) = self.run_to_next_vblank_until(addresses);
    if hit != 0 { return hit; }
    self.skip_irrelevant_vblanks_until(&[], s, input_frame, /* allow_hit_after_relevant_input_read = */ false)
  }
  /// Runs until any of the given addresses is hit, or the next relevant input is being used.
  /// This operation can render the execution unsavable if the input use happens between the reading and the use of the next input.
  /// This operation is useful to evalutate metrics as the result of pressing an input, which happen before the next input is used.
  pub fn run_until_or_next_input_use(&mut self, addresses: &[i32]) -> i32 {
    assert!(!self.is_at_input);
    let (hit, s, input_frame) = self.run_to_next_vblank_until(addresses);
    if hit != 0 { return hit; }
    self.skip_irrelevant_vblanks_until(addresses, s, input_frame, /* allow_hit_after_relevant_input_read = */ true)
  }
}
impl <R: JoypadAddresses + RngAddresses> Gb<R> {
  #[allow(dead_code)]
  pub fn create_inputs(&mut self) -> Vec<Input> {
    assert!(!self.skipped_relevant_inputs);
    let tmp = self.save();

    self.restore_initial_state();

    let mut result: Vec<Input> = vec![];

    for &input in tmp.inputs.iter() {
      self.gb.set_input(input);
      result.resize(self.gb.frame() as usize, Input::empty());
      result[self.gb.frame() as usize - 1] |= input & if R::JOYPAD_READ_FIRST_ADDRESS == R::JOYPAD_READ_HI_ADDRESS { inputs::HI_INPUTS } else { inputs::LO_INPUTS };
      self.gb.run_until(&[R::JOYPAD_READ_LAST_ADDRESS]);
      result.resize(self.gb.frame() as usize, Input::empty());
      result[self.gb.frame() as usize - 1] |= input & if R::JOYPAD_READ_LAST_ADDRESS == R::JOYPAD_READ_HI_ADDRESS { inputs::HI_INPUTS } else { inputs::LO_INPUTS };
      self.gb.run_until(&[R::JOYPAD_READ_LOCKED_ADDRESS]);
      self.is_at_input = false;
      self.step();
    }

    self.restore(&tmp);
    while result.last().unwrap_or(&Input::A).is_empty() { result.pop(); }
    println!("creating inputs done: #inputs: {}", result.len());
    result
  }
}