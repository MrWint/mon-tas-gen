use std::fs::File;
use std::io::Read;
use std::os::raw::c_void;
use std::sync::atomic::{ATOMIC_USIZE_INIT, AtomicUsize, Ordering};

bitflags! {
  #[derive(Serialize, Deserialize)]
  pub struct Input: u8 {
    const DOWN   = 0b10000000;
    const UP     = 0b01000000;
    const LEFT   = 0b00100000;
    const RIGHT  = 0b00010000;
    const START  = 0b00001000;
    const SELECT = 0b00000100;
    const B      = 0b00000010;
    const A      = 0b00000001;
  }
}
#[allow(dead_code)]
pub mod inputs {
  use super::Input;

  pub const A: Input         = Input::A;
  pub const B: Input         = Input::B;
  pub const START: Input     = Input::START;
  pub const SELECT: Input    = Input::SELECT;
  pub const U: Input         = Input::UP;
  pub const D: Input         = Input::DOWN;
  pub const L: Input         = Input::LEFT;
  pub const R: Input         = Input::RIGHT;
  pub const HI_INPUTS: Input = Input { bits: 0b11110000 };
  pub const LO_INPUTS: Input = Input { bits: 0b00001111 };
  pub const NIL: Input       = Input { bits: 0b00000000 };
}

#[derive(Default)]
#[repr(C)]
pub struct Registers {
	pub pc: i32,
	pub sp: i32,
	pub a: i32,
	pub b: i32,
	pub c: i32,
	pub d: i32,
	pub e: i32,
	pub f: i32,
	pub h: i32,
	pub l: i32,
}

#[link(name = "gambatte")]
extern {
  fn initSdlOutput(numScreens: u32, scaleFactor: u32);
  fn createGb(screen: i32, equal_length_frames: bool) -> *mut c_void;
  fn destroyGb(gb: *mut c_void);
  fn loadGbcBios(gb: *mut c_void, biosdata: *const u8);
  fn loadRom(gb: *mut c_void, romfiledata: *const u8, romfilelength: u32);

  fn setInput(gb: *mut c_void, keymask: u32);
  fn step(gb: *mut c_void);
  fn stepUntil(gb: *mut c_void, interruptAddresses: *const i32, numInterruptAddresses: i32) -> i32;

  fn reset(gb: *mut c_void);

  fn loadState(gb: *mut c_void, data: *const u8, len: i32) -> i32;
  fn saveState(gb: *mut c_void, data: *mut u8, len: i32) -> i32;

  fn getNumFrames(gb: *mut c_void) -> u32;
  fn isOnFrameBoundaries(gb: *mut c_void) -> bool;
  fn getCycleCount(gb: *mut c_void) -> u64;

  fn readMemory(gb: *mut c_void, address: u16) -> u8;
  fn writeMemory(gb: *mut c_void, address: u16, value: u8);
  fn readRegisters(gb: *mut c_void, registers: *mut Registers);
  fn readDivState(gb: *mut c_void) -> u16;
}

/// Thin Rust FFI wrapper around libgambatte Gameboy emulator.
/// 
/// # Examples
///
/// ```
/// Gambatte::init_screens(/* num_screens= */ 1, /* scale_factor= */ 3);
///
/// let gb = Gambatte::create_on_screen(/* screen= */ 0, /* equal_length_frames= */ false);
/// gb.load_gbc_bios("roms/gbc_bios.bin");
/// gb.load_rom("roms/crystal.gbc");
/// ```
pub struct Gambatte {
  /// Pointer to  gambatte object used to identify the instance in FFI calls.
  gb: *mut c_void,
  /// Byte content of the loaded Gameboy ROM.
  rom_data: Vec<u8>,
}

impl Gambatte {
  /// Initialize SDL window to render video output of Gambatte instances on.
  pub fn init_screens(num_screens: u32, scale_factor: u32) {
    unsafe {
      initSdlOutput(num_screens, scale_factor);
    }
  }

  /// Create a new Gambatte instance not attached to any output screen.
  #[allow(dead_code)]
  pub fn create(equal_length_frames: bool) -> Gambatte {
    unsafe {
      Gambatte {
        gb: createGb(-1, equal_length_frames),
        rom_data: vec![],
      }
    }
  }
  /// Create a new Gambatte instance attached to an output screen. Requires a screen to be created using ```init_screens``` beforehand.
  #[allow(dead_code)]
  pub fn create_on_screen(screen: i32, equal_length_frames: bool) -> Gambatte {
    unsafe {
      Gambatte {
        gb: createGb(screen, equal_length_frames),
        rom_data: vec![],
      }
    }
  }

  /// Loads the GBC BIOS ROM from a file.
  pub fn load_gbc_bios(&self, file_name: &str) {
    let bios_data = Gambatte::load_file(file_name);
    unsafe {
      loadGbcBios(self.gb, bios_data.as_ptr());
    }
  }
  /// Loads the game ROM from a file.
  pub fn load_rom(&mut self, file_name: &str) {
    self.rom_data = Gambatte::load_file(file_name);
    unsafe {
      loadRom(self.gb, self.rom_data.as_ptr(), self.rom_data.len() as u32);
    }
  }

  /// Changes the input buttons pressed, indefinitely until it is changed again.
  pub fn set_input(&self, input: Input) {
    unsafe {
      setInput(self.gb, input.bits() as u32);
    }
  }
  /// Runs the emulation until the next frame (as defined by BizHawk's timing).
  #[allow(dead_code)]
  pub fn step(&self) {
    unsafe {
      step(self.gb);
    }
  }
  /// Runs the emulation until the next frame (as defined by BizHawk's timing), or until the execution reaches one of the given addresses.
  pub fn step_until(&self, addresses: &[i32]) -> i32 {
    unsafe {
      stepUntil(self.gb, addresses.as_ptr(), addresses.len() as i32)
    }
  }
  /// Performs a hard reset of the Gameboy.
  #[allow(dead_code)]
  pub fn reset(&self) {
    unsafe {
      reset(self.gb);
    }
  }
  /// Runs the emulation until the execution reaches one of the given addresses.
  pub fn run_until(&self, addresses: &[i32]) -> i32 {
    loop {
      unsafe {
        let hit = stepUntil(self.gb, addresses.as_ptr(), addresses.len() as i32);
        if hit != 0 { return hit; }
      }
    }
  }

  /// Restores a stored internal Gambatte state from the given byte data.
  pub fn load_state(&self, data: &[u8]) {
    unsafe {
      let actual_len = loadState(self.gb, data.as_ptr(), data.len() as i32);
      assert!(actual_len <= data.len() as i32, "load failed, actual length {} larger than provided buffer length {}", actual_len, data.len());
    }
  }
  /// Stores the current internal Gambatte state to byte data.
  pub fn save_state(&self) -> Vec<u8> {
    static LAST_SAVE_STATE_SIZE: AtomicUsize = ATOMIC_USIZE_INIT; // Cached last state size, to avoid multiple attempts.

    let mut save_state_size_guess = LAST_SAVE_STATE_SIZE.load(Ordering::Relaxed);
    let mut data: Vec<u8> = Vec::with_capacity(save_state_size_guess);
    loop {
      data.resize(save_state_size_guess, 0);
      let actual_len = unsafe { saveState(self.gb, data.as_mut_ptr(), data.len() as i32) as usize };
      if actual_len < save_state_size_guess {
        println!("shrink save state size from {} to {}", save_state_size_guess, actual_len);
        LAST_SAVE_STATE_SIZE.store(actual_len, Ordering::Relaxed);
        data.truncate(actual_len);
        data.shrink_to_fit();
        break;
      }
      if actual_len == save_state_size_guess { break; }
      println!("expand save state size from {} to {}", save_state_size_guess, actual_len);
      LAST_SAVE_STATE_SIZE.store(actual_len, Ordering::Relaxed);
      save_state_size_guess = actual_len;
    }
    data
  }

  /// Number of frames (as defined by BizHawk's timing) that have been emulated so far, including the current frame if not on a frame boundary.
  pub fn frame(&self) -> u32 {
    unsafe {
      getNumFrames(self.gb)
    }
  }
  /// Whether the emulation is currently stopped at the boundary between two frames (as defined by BizHawk's timing).
  #[allow(dead_code)]
  pub fn is_on_frame_boundaries(&self) -> bool {
    unsafe {
      isOnFrameBoundaries(self.gb)
    }
  }
  /// Total number of sound samples that have been emitted so far (~35112 per frame, depending on the timing method).
  #[allow(dead_code)]
  pub fn get_cycle_count(&self) -> u64 {
    unsafe {
      getCycleCount(self.gb)
    }
  }

  /// Returns the byte at the given ROM address.
  #[allow(dead_code)]
  pub fn read_rom(&self, address: i32) -> u8 {
    self.rom_data[Gambatte::convert_address(address)]
  }
  /// Returns the 2-byte word (Little Endian) starting at the given ROM address.
  #[allow(dead_code)]
  pub fn read_rom_word_le(&self, address: i32) -> u16 {
    return ((self.read_rom(address + 1) as u16) << 8) + self.read_rom(address) as u16;
  }
  /// Converts ROM addresses from input form (bank*0x10000 + address) to byte position in the ROM data.
  fn convert_address(address: i32) -> usize {
    let bank = address as usize >> 16;
    let add = address as usize & 0xffff;
    assert!(add < 0x8000 && (add >= 0x4000 || bank == 0));
    return add + bank.saturating_sub(1)*0x4000;
  }

  /// Reads a byte from the given address from the memory bus, without causing emulation side-effects.
  #[allow(dead_code)]
  pub fn read_memory(&self, address: u16) -> u8 {
    unsafe {
      readMemory(self.gb, address)
    }
  }
  /// Reads a 2-byte word (Big Endian) from the given address from the memory bus, without causing emulation side-effects.
  #[allow(dead_code)]
  pub fn read_memory_word_be(&self, address: u16) -> u16 {
    return ((self.read_memory(address) as u16) << 8) + self.read_memory(address + 1) as u16;
  }
  /// Reads a 2-byte word (Little Endian) from the given address from the memory bus, without causing emulation side-effects.
  #[allow(dead_code)]
  pub fn read_memory_word_le(&self, address: u16) -> u16 {
    return ((self.read_memory(address + 1) as u16) << 8) + self.read_memory(address) as u16;
  }
  /// Writes a byte to the memory bus, as if written by the game, including side-effects and memory-mapped areas.
  #[allow(dead_code)]
  pub fn write_memory(&self, address: u16, value: u8) {
    unsafe {
      writeMemory(self.gb, address, value);
    }
  }
  /// Reads the current state of the Gameboy's registers, without causing emulation side-effects.
  #[allow(dead_code)]
  pub fn read_registers(&self) -> Registers {
    let mut registers = Registers::default();
    unsafe {
      readRegisters(self.gb, &mut registers);
    }
    registers
  }
  /// Reads the current state of the Gameboy's DIV counter (used for RNG), without causing emulation side-effects.
  #[allow(dead_code)]
  pub fn read_div_state(&self) -> u16 {
    unsafe {
      readDivState(self.gb)
    }
  }

  /// Helper function to load the byte contents of a file into memory.
  fn load_file(file_name: &str) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    let mut f = File::open(file_name).expect("file not found");
    f.read_to_end(&mut result).unwrap();
    result
  }
}

impl Drop for Gambatte {
    fn drop(&mut self) {
        unsafe { destroyGb(self.gb); }
    }
}