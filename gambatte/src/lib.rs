#[macro_use] extern crate bitflags;
extern crate byteorder;
#[macro_use] extern crate serde_derive;

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use std::io::Cursor;
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
  fn gambatte_destroy(gb: *mut c_void);
  fn gambatte_loadgbcbios(gb: *mut c_void, biosdata: *const u8);
  fn gambatte_load(gb: *mut c_void, romfiledata: *const u8, romfilelength: u32);

  fn gambatte_setvideobuffer(gb: *mut c_void, videobuf: *mut u32, pitch: i32);

  fn gambatte_setinputgetter(gb: *mut c_void, cb: extern fn(*mut c_void) -> u32, target: *mut c_void);
  fn gambatte_setrtccallback(gb: *mut c_void, cb: extern fn(*mut c_void) -> u32, target: *mut c_void);

  fn gambatte_runfor(gb: *mut c_void, samples: *mut u32) -> i32;
  fn gambatte_reset(gb: *mut c_void, now: u32);

  fn gambatte_setinterruptaddresses(gb: *mut c_void, interruptAddresses: *const i32, numInterruptAddresses: i32);
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

pub struct Gambatte {
  /// Pointer to gambatte object used to identify the instance in FFI calls.
  gb: *mut c_void,
  input_getter: Box<InputGetter>, // Boxed to place it on the heap with a fixed address for Gambatte to point to.
  pub frame: Box<FrameCounter>, // Boxed to place it on the heap with a fixed address for Gambatte to point to.
}

impl Gambatte {
  /// Create a new Gambatte instance.
  #[allow(dead_code)]
  pub fn create() -> Gambatte {
    let input_getter = Box::new(InputGetter { input: inputs::NIL });
    let frame = Box::new(0);
    unsafe {
      let gb = gambatte_create();

      let input_getter_ptr = Box::into_raw(input_getter);
      gambatte_setinputgetter(gb, input_getter_fn, input_getter_ptr as *mut c_void);
      let input_getter = Box::from_raw(input_getter_ptr);

      let frame_ptr = Box::into_raw(frame);
      gambatte_setrtccallback(gb, rtc_fn, frame_ptr as *mut c_void);
      let frame = Box::from_raw(frame_ptr);

      Gambatte {
        gb,
        input_getter,
        frame
      }
    }
  }

  /// Loads the GBC BIOS ROM from a file.
  pub fn load_gbc_bios(&self, bios_data: &[u8]) {
    unsafe {
      gambatte_loadgbcbios(self.gb, bios_data.as_ptr());
    }
  }
  /// Loads the game ROM from a file.
  pub fn load_rom(&self, rom_data: &[u8]) {
    unsafe {
      gambatte_load(self.gb, rom_data.as_ptr(), rom_data.len() as u32);
    }
  }

  pub unsafe fn set_video_buffer(&self, videobuf: *mut u32, pitch: i32) {
    gambatte_setvideobuffer(self.gb, videobuf, pitch);
  }

  pub fn set_input(&mut self, input: Input) {
    self.input_getter.input = input;
  }
  // Runs for at least the given number of samples. Returns whether a new video frame has been rendered.
  pub fn run_for(&self, samples: &mut u32) -> bool {
    unsafe {
      gambatte_runfor(self.gb, samples as *mut u32) >= 0
    }
  }
  /// Loads the game ROM from a file.
  pub fn set_hit_interrupt_address(&self, interrupts: &[i32]) {
    unsafe { gambatte_setinterruptaddresses(self.gb, interrupts.as_ptr(), interrupts.len() as i32); }
  }
  pub fn get_hit_interrupt_address(&self) -> i32 {
    unsafe { gambatte_gethitinterruptaddress(self.gb) }
  }

  pub fn reset(&self) {
    unsafe { gambatte_reset(self.gb, (u64::from(*self.frame + 1) * 4389 / 262_144) as u32); } // temporarily add a frame since BizHawk increases the frame before checking for resets, so current time is accurate.
  }

  /// Restores a stored internal Gambatte state from the given byte data.
  pub fn load_state(&mut self, reader: &mut Cursor<&[u8]>) {
    const EXTRA_DATA_LENGTH: usize = 5;

    let remaining_data_len = reader.get_ref().len() - reader.position() as usize;
    let actual_len = unsafe { gambatte_newstateload(self.gb, reader.get_ref().as_ptr(), remaining_data_len as i32) } as usize;
    assert!(actual_len + EXTRA_DATA_LENGTH <= remaining_data_len, "load failed, actual length {} larger than provided buffer length {}", actual_len + EXTRA_DATA_LENGTH, remaining_data_len);

    self.input_getter.input = Input::from_bits_truncate(reader.read_u8().unwrap());
    *self.frame = reader.read_u32::<LittleEndian>().unwrap();
  }
  /// Stores the current internal Gambatte state to byte data.
  pub fn save_state(&self, writer: &mut Cursor<Vec<u8>>) {
    const EXTRA_DATA_LENGTH: usize = 5;

    let save_state_size = unsafe { gambatte_newstatelen(self.gb) } as usize;
    let remaining_capacity = writer.get_ref().len() - writer.position() as usize;
    if save_state_size > remaining_capacity {
      let new_size = writer.position() as usize + save_state_size + EXTRA_DATA_LENGTH;
      println!("save_state Vec too small, increase to {}", new_size);
      writer.get_mut().resize(new_size, 0);
    }
    let actual_len = unsafe { gambatte_newstatesave(self.gb, writer.get_mut().as_mut_ptr(), save_state_size as i32) } as usize;
    assert!(actual_len == save_state_size);

    writer.write_u8(self.input_getter.input.bits()).unwrap();
    writer.write_u32::<LittleEndian>(*self.frame).unwrap();
  }

  /// Reads a byte from the given address from the memory bus, without causing emulation side-effects.
  #[allow(dead_code)]
  pub fn read_memory(&self, address: u16) -> u8 {
    unsafe {
      gambatte_cpuread(self.gb, address)
    }
  }
  /// Writes a byte to the memory bus, as if written by the game, including side-effects and memory-mapped areas.
  #[allow(dead_code)]
  pub fn write_memory(&self, address: u16, value: u8) {
    unsafe {
      gambatte_cpuwrite(self.gb, address, value);
    }
  }
  /// Reads the current state of the Gameboy's registers, without causing emulation side-effects.
  #[allow(dead_code)]
  pub fn read_registers(&self) -> Registers {
    let mut registers = Registers::default();
    unsafe {
      gambatte_getregs(self.gb, &mut registers);
    }
    registers
  }
  /// Reads the current state of the Gameboy's DIV counter (used for RNG), without causing emulation side-effects.
  /// The result is a value in [0x0, 0x3fff].
  #[allow(dead_code)]
  pub fn read_div_state(&self) -> u16 {
    unsafe {
      gambatte_getdivstate(self.gb)
    }
  }
}

impl Drop for Gambatte {
    fn drop(&mut self) {
        unsafe { gambatte_destroy(self.gb); }
    }
}