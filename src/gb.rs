use gambatte::*;
use rom::*;
use std::marker::PhantomData;

#[derive(Serialize, Deserialize)]
pub struct State {
  // inherent state of the GB
  gb_state: Vec<u8>,
  inputs: Vec<Input>,
  last_input_frame: [u32; 2], // first, last
  pub is_at_input: bool,
  ignored_inputs: Input, // inputs ignored by next input use

  // derived state for StateBuffer decisions
  pub rng_state: u32,
  pub frame: u32,
}

pub struct Gb<R> {
  pub gb: Gambatte,
  root: Vec<u8>,
  skipped_relevant_inputs: bool, // whether relevant inputs were skipped over, making this inherently unsavable in any way.
  _rom: PhantomData<R>,

  inputs: Vec<Input>,
  last_input_frame: [u32; 2], // first, last
  pub is_at_input: bool,
  ignored_inputs: Input, // inputs ignored by next input use
}

impl <R: BasicRomInfo + JoypadAddresses> Gb<R> {
  pub fn create(mut gb: Gambatte) -> Self {
    gb.load_gbc_bios("roms/gbc_bios.bin");
    gb.load_rom(R::ROM_NAME);
    let root = gb.save_state();
    let mut pgb = Gb {
      gb: gb,
      root: root,
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
      frame: self.gb.frame(),
      rng_state: if self.is_at_input { self.get_rng_state() } else { 0 },
    }
  }
  fn get_rng_state(&self) -> u32 {
    ((self.gb.read_div_state() as u32) << 16) + self.gb.read_memory_word_be(R::RNG_MEM_ADDRESS) as u32
  }
}
impl <R> Gb<R> {
  pub fn restore(&mut self, s: &State) {
    // load inherent state
    self.gb.load_state(&s.gb_state);
    self.inputs.clone_from(&s.inputs);
    self.last_input_frame.clone_from(&s.last_input_frame);
    self.is_at_input = s.is_at_input;
    self.ignored_inputs = s.ignored_inputs;
    self.skipped_relevant_inputs = false;
  }
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
  pub fn restore_initial_state(&mut self) {
    self.gb.load_state(&self.root);
    self.inputs.clear();
    self.last_input_frame[0] = 0;
    self.last_input_frame[1] = 0;
    self.is_at_input = false;
    self.skipped_relevant_inputs = false;
    self.step(); // move to first decision point
  }

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
  pub fn step(&mut self) {
    self.step_until_or_any_vblank(&[]); // slightly more performant than step_until
  }
  fn step_to_next_vblank_until(&mut self, addresses: &[i32]) -> (i32, Vec<u8>) {
    let mut s = vec![];
    loop {
      let hit = self.gb.run_until(&[&[R::JOYPAD_READ_FIRST_ADDRESS], addresses].concat());
      if hit != R::JOYPAD_READ_FIRST_ADDRESS { return (hit, s); }
      if self.gb.frame() == self.last_input_frame[0] {
          println!("found multiple first inputs in frame {}, skipping possible input", self.last_input_frame[0]);
          self.gb.run_until(&[R::JOYPAD_READ_LOCKED_ADDRESS]);
          continue;
      }
      s = self.gb.save_state();
      self.gb.run_until(&[R::JOYPAD_READ_LAST_ADDRESS]);
      if self.gb.frame() == self.last_input_frame[1] {
          println!("found multiple last inputs in frame {}, skipping possible input", self.last_input_frame[1]);
          self.gb.run_until(&[R::JOYPAD_READ_LOCKED_ADDRESS]);
          continue;
      }
      return (0, s);
    }
  }
  fn skip_irrelevant_vblanks_until(&mut self, addresses: &[i32], mut s: Vec<u8>) -> i32 {
    loop {
      self.gb.run_until(&[R::JOYPAD_READ_LOCKED_ADDRESS]);

      let mut hit_addresses = false;
      let mut hit;
      loop {
        self.ignored_inputs = Input::empty();
        hit = if hit_addresses {
          self.gb.run_until(&[&[R::JOYPAD_READ_FIRST_ADDRESS], R::JOYPAD_USE_ADDRESSES].concat())
        } else {
          self.gb.run_until(&[&[R::JOYPAD_READ_FIRST_ADDRESS], R::JOYPAD_USE_ADDRESSES, addresses].concat())
        };
        if !hit_addresses && addresses.contains(&hit) {
          hit_addresses = true;
        } else if hit == R::JOYPAD_READ_FIRST_ADDRESS {
          break;
        } else { // R::JOYPAD_USE_ADDRESSES
          let mut discard = false;
          for &(use_add, ignore_mask_mem_add) in R::JOYPAD_USE_IGNORE_MASK_MEM_ADDRESSES {
            if hit == use_add {
              self.ignored_inputs = Input::from_bits_truncate(self.gb.read_memory(ignore_mask_mem_add));
              if self.ignored_inputs.bits() == 0xff { discard = true; } // discard if all inputs ignored
              break;
            }
          }
          for &(use_add, keep_add, discard_add) in R::JOYPAD_USE_DISCARD_ADDRESSES {
            if hit == use_add {
              if self.gb.run_until(&[keep_add, discard_add]) == discard_add { discard = true; }
              break;
            }
          }
          if !discard { break; }
        }
      }

      if hit != R::JOYPAD_READ_FIRST_ADDRESS {
        self.gb.load_state(&s);
        self.last_input_frame[0] = self.gb.frame();
        self.gb.run_until(&[R::JOYPAD_READ_LAST_ADDRESS]);
        self.last_input_frame[1] = self.gb.frame();
        self.gb.load_state(&s);
        self.is_at_input = true;
        return 0;
      } else if hit_addresses {
        self.gb.load_state(&s);
        return self.gb.run_until(addresses);
      }
      s = self.gb.save_state();
    }
  }
  // runs until addresses are hit, or the next relevant input vblank occurs.
  pub fn step_until(&mut self, addresses: &[i32]) -> i32 {
    assert!(!self.is_at_input);
    let (hit, s) = self.step_to_next_vblank_until(addresses);
    if hit != 0 { return hit; }
    self.skip_irrelevant_vblanks_until(addresses, s)
  }
  // runs until addresses are hit, or any vblank occurs (whether or not it is a relevant input) and forwards to relevant input.
  pub fn step_until_or_any_vblank(&mut self, addresses: &[i32]) -> i32 {
    assert!(!self.is_at_input);
    let (hit, s) = self.step_to_next_vblank_until(addresses);
    if hit != 0 { return hit; }
    self.skip_irrelevant_vblanks_until(&[], s)
  }
  pub fn run_until_or_next_input_use(&mut self, addresses: &[i32]) -> i32 {
    assert!(!self.is_at_input);
    let hit = self.step_until(addresses);
    if hit != 0 { return hit; }
    self.skipped_relevant_inputs = true; // this state is busted after this point
    self.input(Input::empty());
    let hit = self.step_until(&[addresses, R::JOYPAD_USE_ADDRESSES].concat());
    if addresses.contains(&hit) { hit } else { 0 }
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