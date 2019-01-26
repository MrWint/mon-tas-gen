#[macro_use] extern crate bitflags;
extern crate byteorder;
#[macro_use] extern crate serde_derive;

const SAMPLES_PER_FRAME: u32 = 35112;

use std::fs::File;
use std::io::Read;
use std::os::raw::c_void;

bitflags! {
  #[derive(Serialize, Deserialize)]
  pub struct Input: u8 {
    const DOWN   = 0b1000_0000;
    const UP     = 0b0100_0000;
    const LEFT   = 0b0010_0000;
    const RIGHT  = 0b0001_0000;
    const START  = 0b0000_1000;
    const SELECT = 0b0000_0100;
    const B      = 0b0000_0010;
    const A      = 0b0000_0001;
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
  pub const HI_INPUTS: Input = Input { bits: 0b1111_0000 };
  pub const LO_INPUTS: Input = Input { bits: 0b0000_1111 };
  pub const NIL: Input       = Input { bits: 0b0000_0000 };
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

extern {
  fn gambatte_create() -> *mut c_void;
  fn gambatte_loadgbcbios(gb: *mut c_void, biosdata: *const u8);
  fn gambatte_load(gb: *mut c_void, romfiledata: *const u8, romfilelength: usize, now: i64, flags: u32, div: u32);
  fn gambatte_setlayers(gb: *mut c_void, layers: u32);
  fn gambatte_destroy(gb: *mut c_void);

  fn gambatte_setvideobuffer(gb: *mut c_void, videobuf: *mut u32, pitch: i32);

  fn gambatte_setinputgetter(gb: *mut c_void, cb: extern fn(*mut c_void) -> u32, target: *mut c_void);
  fn gambatte_setrtccallback(gb: *mut c_void, cb: extern fn(*mut c_void) -> u32, target: *mut c_void);

  fn gambatte_runfor(gb: *mut c_void, samples: *mut u32) -> i32;
  fn gambatte_reset(gb: *mut c_void, now: i64, div: u32);

  fn gambatte_setinterruptaddresses(gb: *mut c_void, interruptAddresses: *const i32, numInterruptAddresses: i32);
  fn gambatte_clearinterruptaddresses(gb: *mut c_void);
  fn gambatte_gethitinterruptaddress(gb: *mut c_void) -> i32;

  fn gambatte_newstatelen(gb: *mut c_void) -> i32;
  fn gambatte_newstatesave(gb: *mut c_void, data: *mut u8, len: i32) -> i32;
  fn gambatte_newstateload(gb: *mut c_void, data: *const u8, len: i32) -> i32;

  fn gambatte_cpuread(gb: *mut c_void, address: u16) -> u8;
  fn gambatte_cpuwrite(gb: *mut c_void, address: u16, value: u8);
  fn gambatte_getregs(gb: *mut c_void, registers: *mut Registers);
  fn gambatte_getdivstate(gb: *mut c_void) -> u16;
}

pub struct InputGetter {
  input: Input,
}
extern fn input_getter_fn(context: *mut c_void) -> u32 {
  unsafe { u32::from((*(context as *mut InputGetter)).input.bits()) }
}
pub type FrameCounter = u32;

extern fn rtc_fn(context: *mut c_void) -> u32 {
  let frame: u64 = unsafe { u64::from(*(context as *mut FrameCounter)) };
  (frame * 4389 / 262_144) as u32
}

pub trait ScreenUpdateCallback {
  fn get_screen_buffer_pointer_and_pitch(&self) -> Option<(*mut u32, usize)>;
  fn update_screen(&self);
}

pub struct NoScreen;
impl ScreenUpdateCallback for NoScreen {
  fn get_screen_buffer_pointer_and_pitch(&self) -> Option<(*mut u32, usize)> { None }
  fn update_screen(&self) {}
}

#[derive(Serialize, Deserialize)]
pub struct SaveState {
  gambatte_state: Vec<u8>,
  input: Input,
  frame: u32,
  is_on_frame_boundaries: bool,
  overflow_samples: u32,
  cycle_count: u64,
}

pub struct Gambatte {
  /// Pointer to gambatte object used to identify the instance in FFI calls.
  gb: *mut c_void,
  input_getter: Box<InputGetter>, // Boxed to place it on the heap with a fixed address for Gambatte to point to.
  pub frame: Box<FrameCounter>, // Boxed to place it on the heap with a fixed address for Gambatte to point to.
  /// Byte content of the loaded Gameboy ROM.
  rom_data: Vec<u8>,
  equal_length_frames: bool,
  is_on_frame_boundaries: bool,
  overflow_samples: u32,
  cycle_count: u64,
  screen_update_callback: Box<ScreenUpdateCallback>, // trait object to avoid generics.
}

impl Gambatte {
  /// Create a new Gambatte instance.
  pub fn create<S: ScreenUpdateCallback + 'static>(bios_file_name: &str, rom_file_name: &str, equal_length_frames: bool, screen_update_callback: S) -> Gambatte {
    let bios_data = load_file(bios_file_name);
    let rom_data = load_file(rom_file_name);
    unsafe {
      let gb = gambatte_create();
      gambatte_loadgbcbios(gb, bios_data.as_ptr());
      gambatte_load(gb, rom_data.as_ptr(), rom_data.len(), 0 /*now*/, 2 /*GBA_CGB*/, 0 /*div*/);
      gambatte_setlayers(gb, 7);

      let input_getter = Box::new(InputGetter { input: inputs::NIL });
      let input_getter_ptr = Box::into_raw(input_getter);
      gambatte_setinputgetter(gb, input_getter_fn, input_getter_ptr as *mut c_void);
      let input_getter = Box::from_raw(input_getter_ptr);

      let frame = Box::new(0);
      let frame_ptr = Box::into_raw(frame);
      gambatte_setrtccallback(gb, rtc_fn, frame_ptr as *mut c_void);
      let frame = Box::from_raw(frame_ptr);

      if let Some((videobuf, pitch)) = screen_update_callback.get_screen_buffer_pointer_and_pitch() {
        gambatte_setvideobuffer(gb, videobuf, pitch as i32);
      }

      Gambatte {
        gb,
        input_getter,
        frame,
        rom_data,
        equal_length_frames,
        is_on_frame_boundaries: true,
        overflow_samples: 0,
        cycle_count: 0,
        screen_update_callback: Box::new(screen_update_callback),
      }
    }
  }

  /// Changes the input buttons pressed, indefinitely until it is changed again.
  #[inline]
  pub fn set_input(&mut self, input: Input) {
    self.input_getter.input = input;
  }
  fn step_internal(&mut self) -> i32 {
    if self.is_on_frame_boundaries { *self.frame += 1 };
    let mut hit_interrupt_address: i32;

    loop {
      let mut emusamples: u32 = SAMPLES_PER_FRAME - self.overflow_samples;
      
      if unsafe { gambatte_runfor(self.gb, (&mut emusamples) as *mut u32) } >= 0 { // check for new video frame
        self.screen_update_callback.update_screen();
      }

      self.overflow_samples += emusamples;
      self.cycle_count += u64::from(emusamples);
      hit_interrupt_address = unsafe { gambatte_gethitinterruptaddress(self.gb) };

      if hit_interrupt_address != 0 { // go into frame
        break;
      }

      if !self.equal_length_frames { // old frame timing
        self.overflow_samples = 0; // completed frame
        break;
      }

      if self.overflow_samples >= SAMPLES_PER_FRAME { // new frame timing
        self.overflow_samples -= SAMPLES_PER_FRAME;
        break;
      }
    }
    self.is_on_frame_boundaries = hit_interrupt_address == 0;
    hit_interrupt_address
  }
  /// Runs the emulation until the next frame (as defined by BizHawk's timing).
  pub fn step(&mut self) {
    self.step_internal();
  }
  /// Runs the emulation until the next frame (as defined by BizHawk's timing), or until the execution reaches one of the given addresses.
  pub fn step_until(&mut self, addresses: &[i32]) -> i32 {
    unsafe { gambatte_setinterruptaddresses(self.gb, addresses.as_ptr(), addresses.len() as i32); }
    let hit_address = self.step_internal();
    unsafe { gambatte_clearinterruptaddresses(self.gb); }
    hit_address
  }
  /// Runs the emulation until the execution reaches one of the given addresses.
  pub fn run_until(&mut self, addresses: &[i32]) -> i32 {
    unsafe { gambatte_setinterruptaddresses(self.gb, addresses.as_ptr(), addresses.len() as i32); }
    loop {
      let hit_address = self.step_internal();
      if hit_address != 0 {
      unsafe { gambatte_clearinterruptaddresses(self.gb); }
        return hit_address;
      }
    }
  }

  /// Performs a hard reset of the Gameboy.
  #[allow(dead_code)]
  pub fn reset(&mut self) {
    if !self.is_on_frame_boundaries { // forward to next frame boundary
      self.step();
    }
    unsafe { gambatte_reset(self.gb, i64::from(*self.frame + 1) * 4389 / 262_144, 0 /*div*/); } // temporarily add a frame since BizHawk increases the frame before checking for resets, so current time is accurate.
    self.set_input(inputs::NIL);
    self.step(); // BizHawk completes a frame on the reset input
  }

  /// Restores a stored internal Gambatte state from the given byte data.
  pub fn load_state(&mut self, s: &SaveState) {
    let success = unsafe { gambatte_newstateload(self.gb, s.gambatte_state.as_ptr(), s.gambatte_state.len() as i32) };
    assert!(success == 1);

    self.input_getter.input = s.input;
    *self.frame = s.frame;
    self.is_on_frame_boundaries = s.is_on_frame_boundaries;
    self.overflow_samples = s.overflow_samples;
    self.cycle_count = s.cycle_count;
  }

  /// Stores the current internal Gambatte state to byte data.
  pub fn save_state(&self) -> SaveState {
    let save_state_size = unsafe { gambatte_newstatelen(self.gb) } as usize;
    let mut gambatte_state = unsafe { // Avoid calling memset, Vec will be initialized with garbage.
      let mut data = Vec::with_capacity(save_state_size);
      let resized_data = Vec::from_raw_parts(data.as_mut_ptr(), save_state_size, save_state_size);
      ::std::mem::forget(data);
      resized_data
    };
    let success = unsafe { gambatte_newstatesave(self.gb, gambatte_state.as_mut_ptr(), save_state_size as i32) };
    assert!(success == 1);

    SaveState {
      gambatte_state,
      input: self.input_getter.input,
      frame: *self.frame,
      is_on_frame_boundaries: self.is_on_frame_boundaries,
      overflow_samples: self.overflow_samples,
      cycle_count: self.cycle_count,
    }
  }

  /// Number of frames (as defined by BizHawk's timing) that have been emulated so far, including the current frame if not on a frame boundary.
  pub fn frame(&self) -> u32 {
    *self.frame
  }
  /// Whether the emulation is currently stopped at the boundary between two frames (as defined by BizHawk's timing).
  #[allow(dead_code)]
  pub fn is_on_frame_boundaries(&self) -> bool {
    self.is_on_frame_boundaries
  }
  /// Total number of sound samples that have been emitted so far (~35112 per frame, depending on the timing method).
  #[allow(dead_code)]
  pub fn get_cycle_count(&self) -> u64 {
    self.cycle_count
  }

  /// Returns the byte at the given ROM address.
  #[allow(dead_code)]
  pub fn read_rom(&self, address: i32) -> u8 {
    self.rom_data[convert_address(address)]
  }
  /// Returns the 2-byte word (Little Endian) starting at the given ROM address.
  #[allow(dead_code)]
  pub fn read_rom_word_le(&self, address: i32) -> u16 {
    (u16::from(self.read_rom(address + 1)) << 8) + u16::from(self.read_rom(address))
  }

  /// Reads a byte from the given address from the memory bus, without causing emulation side-effects.
  #[inline]
  pub fn read_memory(&self, address: u16) -> u8 {
    unsafe { gambatte_cpuread(self.gb, address) }
  }
  /// Reads a 2-byte word (Big Endian) from the given address from the memory bus, without causing emulation side-effects.
  #[allow(dead_code)]
  pub fn read_memory_word_be(&self, address: u16) -> u16 {
    (u16::from(self.read_memory(address)) << 8) + u16::from(self.read_memory(address + 1))
  }
  /// Reads a 2-byte word (Little Endian) from the given address from the memory bus, without causing emulation side-effects.
  #[allow(dead_code)]
  pub fn read_memory_word_le(&self, address: u16) -> u16 {
    (u16::from(self.read_memory(address + 1)) << 8) + u16::from(self.read_memory(address))
  }
  /// Writes a byte to the memory bus, as if written by the game, including side-effects and memory-mapped areas.
  #[inline]
  pub fn write_memory(&self, address: u16, value: u8) {
    unsafe {
      gambatte_cpuwrite(self.gb, address, value);
    }
  }
  /// Reads the current state of the Gameboy's registers, without causing emulation side-effects.
  #[inline]
  pub fn read_registers(&self) -> Registers {
    let mut registers = Registers::default();
    unsafe {
      gambatte_getregs(self.gb, &mut registers);
    }
    registers
  }
  /// Reads the current state of the Gameboy's DIV counter (used for RNG), without causing emulation side-effects.
  /// The result is a value in [0x0, 0x3fff].
  #[inline]
  pub fn read_div_state(&self) -> u16 {
    unsafe {
      gambatte_getdivstate(self.gb)
    }
  }
}

/// Helper function to load the byte contents of a file into memory.
fn load_file(file_name: &str) -> Vec<u8> {
  let mut result: Vec<u8> = vec![];
  let mut f = File::open(file_name).expect("file not found");
  f.read_to_end(&mut result).unwrap();
  result
}
/// Converts ROM addresses from input form (bank*0x10000 + address) to byte position in the ROM data.
fn convert_address(address: i32) -> usize {
  let bank = address as usize >> 16;
  let add = address as usize & 0xffff;
  assert!(add < 0x8000 && (add >= 0x4000 || bank == 0));
  add + bank.saturating_sub(1)*0x4000
}

impl Drop for Gambatte {
    fn drop(&mut self) {
        unsafe { gambatte_destroy(self.gb); }
    }
}