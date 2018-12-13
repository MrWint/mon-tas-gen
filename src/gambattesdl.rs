const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

const SAMPLES_PER_FRAME: u32 = 35112;

pub use gambatters::Input;
pub use gambatters::inputs;
pub use gambatters::Registers;

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use std::fs::File;
use std::io::{Cursor, Read};
use std::ptr::Unique;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Sdl {
  num_screens: u32,
  screen_update_tx: Sender<u32>,
  surface_base_pointer: Unique<u32>,
  surface_pitch: usize,
}

impl Sdl {
  pub fn init_sdl(num_screens: u32, scale_factor: u32) -> Sdl {
    let (screen_update_tx, screen_update_rx) = channel::<u32>();
    let (surface_base_ptr_tx, surface_base_ptr_rx) = channel::<(Unique<u32>, usize)>();

    thread::spawn(move || {
      let sdl_context = sdl2::init().unwrap();
      let mut event_pump = sdl_context.event_pump().unwrap();

      let video_subsystem = sdl_context.video().unwrap();
      let window = video_subsystem.window("gambatte output", num_screens * scale_factor * WIDTH, scale_factor * HEIGHT)
          .position_centered()
          .build()
          .unwrap();

      let surface = sdl2::surface::Surface::from_pixelmasks(num_screens * WIDTH, HEIGHT, sdl2::pixels::PixelMasks {
        bpp: 32,
        rmask: 0x00ff_0000,
        gmask: 0x0000_ff00,
        bmask: 0x0000_00ff,
        amask: 0x0000_0000,
      }).unwrap();
      {
        let pitch: usize = surface.pitch() as usize / ::std::mem::size_of::<u32>();
        let pointer: *mut u32 = unsafe {(*surface.raw()).pixels } as *mut u32;
        surface_base_ptr_tx.send((Unique::new(pointer).unwrap(), pitch)).unwrap(); // send base pointer back to main thread.
      }

      loop {
        while let Some(_) = event_pump.poll_event() {} // Work through window events to keep it responsive. All events are discarded.
        match screen_update_rx.recv_timeout(Duration::from_millis(50)) {
          Ok(_screen) => { // update screen
            let mut window_surface = window.surface(&event_pump).unwrap();
            surface.blit_scaled(None, &mut window_surface, None).unwrap();
            window_surface.update_window().unwrap();
          },
          Err(::std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
          Err(::std::sync::mpsc::RecvTimeoutError::Timeout) => {},
        };
      }
    });

    let (surface_base_pointer, surface_pitch) = surface_base_ptr_rx.recv().unwrap();

    Sdl {
      num_screens,
      screen_update_tx,
      surface_base_pointer: surface_base_pointer,
      surface_pitch,
    }
  }

  fn get_screen_buffer_pointer_and_pitch(&self, screen: u32) -> (*mut u32, usize) {
    (unsafe { self.surface_base_pointer.as_ptr().offset(screen as isize * WIDTH as isize) }, self.surface_pitch)
  }

  fn update_screen(&self, screen: u32) {
    self.screen_update_tx.send(screen).unwrap();
  }
}

pub struct Gambatte {
  gb: gambatters::Gambatte,
  /// Byte content of the loaded Gameboy ROM.
  rom_data: Vec<u8>,
  equal_length_frames: bool,
  is_on_frame_boundaries: bool,
  overflow_samples: u32,
  cycle_count: u64,
  sdl: Option<Sdl>,
  screen: u32,
  save_state_size_guess: usize,
}

impl Gambatte {
  pub fn handle_sdl_events() { /* Compatibility function, does nothing. */ }

  /// Create a new Gambatte instance not attached to any output screen.
  #[allow(dead_code)]
  pub fn create(equal_length_frames: bool) -> Gambatte {
    let gb = gambatters::Gambatte::create();
    Gambatte {
      gb,
      rom_data: vec![],
      equal_length_frames,
      is_on_frame_boundaries: true,
      overflow_samples: 0,
      cycle_count: 0,
      sdl: None,
      screen: 0,
      save_state_size_guess: 0,
    }
  }
  /// Create a new Gambatte instance attached to an output screen.
  #[allow(dead_code)]
  pub fn create_on_screen(sdl: Sdl, screen: u32, equal_length_frames: bool) -> Gambatte {
    let gb = Self::create(equal_length_frames);

    let (pointer, pitch) = sdl.get_screen_buffer_pointer_and_pitch(screen);
    unsafe { gb.gb.set_video_buffer(pointer, pitch); }

    Gambatte {
      sdl: Some(sdl),
      screen,
      ..gb
    }
  }

  /// Loads the GBC BIOS ROM from a file.
  pub fn load_gbc_bios(&self, file_name: &str) {
    let bios_data = Gambatte::load_file(file_name);
    self.gb.load_gbc_bios(&bios_data);
  }
  /// Loads the game ROM from a file.
  pub fn load_rom(&mut self, file_name: &str) {
    self.rom_data = Gambatte::load_file(file_name);
    self.gb.load_rom(&self.rom_data);
  }

  /// Changes the input buttons pressed, indefinitely until it is changed again.
  pub fn set_input(&mut self, input: Input) {
    self.gb.set_input(input);
  }
  /// Runs the emulation until the next frame (as defined by BizHawk's timing).
  pub fn step(&mut self) {
    self.step_internal();
  }
  /// Runs the emulation until the next frame (as defined by BizHawk's timing), or until the execution reaches one of the given addresses.
  pub fn step_until(&mut self, addresses: &[i32]) -> i32 {
    self.gb.set_hit_interrupt_addresses(addresses);
    let hit_address = self.step_internal();
    self.gb.clear_hit_interrupt_addresses();
    hit_address
  }
  fn step_internal(&mut self) -> i32 {
    if self.is_on_frame_boundaries { *self.gb.frame += 1 };
    let mut hit_interrupt_address: i32;

    loop {
      let mut emusamples: u32 = SAMPLES_PER_FRAME - self.overflow_samples;
      if self.gb.run_for(&mut emusamples) { // check for new video frame
        for sdl in &self.sdl { sdl.update_screen(self.screen); }
      }

      self.overflow_samples += emusamples;
      self.cycle_count += u64::from(emusamples);
      hit_interrupt_address = self.gb.get_hit_interrupt_address();

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
  /// Performs a hard reset of the Gameboy.
  #[allow(dead_code)]
  pub fn reset(&mut self) {
    if !self.is_on_frame_boundaries { // forward to next frame boundary
      self.step();
    }
    self.gb.reset();
    self.set_input(inputs::NIL);
    self.step(); // BizHawk completes a frame on the reset input
  }
  /// Runs the emulation until the execution reaches one of the given addresses.
  pub fn run_until(&mut self, addresses: &[i32]) -> i32 {
    self.gb.set_hit_interrupt_addresses(addresses);
    loop {
      let hit_address = self.step_internal();
      if hit_address != 0 {
        self.gb.clear_hit_interrupt_addresses();
        return hit_address;
      }
    }
  }

  /// Restores a stored internal Gambatte state from the given byte data.
  pub fn load_state(&mut self, data: &[u8]) {
    const EXTRA_DATA_LENGTH: usize = 13;

    let mut reader = Cursor::new(data);

    self.gb.load_state(&mut reader);
    let remaining_data_len = data.len() - reader.position() as usize;
    assert!(EXTRA_DATA_LENGTH <= remaining_data_len, "load failed, actual extra length {} larger than provided buffer length {}", EXTRA_DATA_LENGTH, remaining_data_len);

    self.is_on_frame_boundaries = reader.read_u8().unwrap() != 0;
    self.overflow_samples = reader.read_u32::<LittleEndian>().unwrap();
    self.cycle_count = reader.read_u64::<LittleEndian>().unwrap();
  }
  /// Stores the current internal Gambatte state to byte data.
  pub fn save_state(&mut self) -> Vec<u8> {
    let mut data: Vec<u8> = {
      let mut data = unsafe { // Avoid calling memset, Vec will be initialized with garbage.
        let mut data = Vec::with_capacity(self.save_state_size_guess);
        let resized_data = Vec::from_raw_parts(data.as_mut_ptr(), self.save_state_size_guess, self.save_state_size_guess);
        ::std::mem::forget(data);
        resized_data
      };
      let mut writer = Cursor::new(data);
      self.gb.save_state(&mut writer);
      writer.write_u8(u8::from(self.is_on_frame_boundaries)).unwrap();
      writer.write_u32::<LittleEndian>(self.overflow_samples).unwrap();
      writer.write_u64::<LittleEndian>(self.cycle_count).unwrap();
      writer.into_inner()
    };
    if data.len() < self.save_state_size_guess {
      println!("shrink save state size guess from {} to {}", self.save_state_size_guess, data.len());
      self.save_state_size_guess = data.len();
      data.shrink_to_fit();
    } else if data.len() > self.save_state_size_guess {
      println!("expand save state size guess from {} to {}", self.save_state_size_guess, data.len());
      self.save_state_size_guess = data.len();
    }
    data
  }

  /// Number of frames (as defined by BizHawk's timing) that have been emulated so far, including the current frame if not on a frame boundary.
  pub fn frame(&self) -> u32 {
    *self.gb.frame
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
    self.rom_data[Gambatte::convert_address(address)]
  }
  /// Returns the 2-byte word (Little Endian) starting at the given ROM address.
  #[allow(dead_code)]
  pub fn read_rom_word_le(&self, address: i32) -> u16 {
    (u16::from(self.read_rom(address + 1)) << 8) + u16::from(self.read_rom(address))
  }
  /// Converts ROM addresses from input form (bank*0x10000 + address) to byte position in the ROM data.
  fn convert_address(address: i32) -> usize {
    let bank = address as usize >> 16;
    let add = address as usize & 0xffff;
    assert!(add < 0x8000 && (add >= 0x4000 || bank == 0));
    add + bank.saturating_sub(1)*0x4000
  }

  /// Reads a byte from the given address from the memory bus, without causing emulation side-effects.
  #[allow(dead_code)]
  pub fn read_memory(&self, address: u16) -> u8 {
    self.gb.read_memory(address)
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
  #[allow(dead_code)]
  pub fn write_memory(&self, address: u16, value: u8) {
    self.gb.write_memory(address, value)
  }
  /// Reads the current state of the Gameboy's registers, without causing emulation side-effects.
  #[allow(dead_code)]
  pub fn read_registers(&self) -> Registers {
    self.gb.read_registers()
  }
  /// Reads the current state of the Gameboy's DIV counter (used for RNG), without causing emulation side-effects.
  /// The result is a value in [0x0, 0x3fff].
  #[allow(dead_code)]
  pub fn read_div_state(&self) -> u16 {
    self.gb.read_div_state()
  }

  /// Helper function to load the byte contents of a file into memory.
  fn load_file(file_name: &str) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    let mut f = File::open(file_name).expect("file not found");
    f.read_to_end(&mut result).unwrap();
    result
  }
}
