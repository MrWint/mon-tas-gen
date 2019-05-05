const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

use gambatte::*;
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
        let pitch: usize = surface.pitch() as usize / std::mem::size_of::<u32>();
        let pointer: *mut u32 = unsafe {(*surface.raw()).pixels } as *mut u32;
        surface_base_ptr_tx.send((Unique::new(pointer).unwrap(), pitch)).unwrap(); // send base pointer back to main thread.
      }

      loop {
        while let Some(_) = event_pump.poll_event() {} // Work through window events to keep it responsive. All events are discarded.
        let mut window_surface = window.surface(&event_pump).unwrap();
        surface.blit_scaled(None, &mut window_surface, None).unwrap();
        window_surface.update_window().unwrap();
        match screen_update_rx.recv_timeout(Duration::from_millis(10)) {
          Ok(_screen) => { // update screen
            while screen_update_rx.try_recv().is_err() {} // clear all events
            thread::sleep(Duration::from_millis(10)); // wait at least 1/100 second before next event
          },
          Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
          Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {},
        };
      }
    });

    let (surface_base_pointer, surface_pitch) = surface_base_ptr_rx.recv().unwrap();

    Sdl {
      num_screens,
      screen_update_tx,
      surface_base_pointer,
      surface_pitch,
    }
  }
}

pub struct SdlScreen {
  sdl: Sdl,
  screen: u32,
}
impl SdlScreen {
  pub fn new(sdl: Sdl, screen: u32) -> SdlScreen {
    assert!(screen < sdl.num_screens);
    SdlScreen {
      sdl,
      screen,
    }
  }
}

impl ScreenUpdateCallback for SdlScreen {
  fn get_screen_buffer_pointer_and_pitch(&self) -> Option<(*mut u32, usize)> {
    Some((unsafe { self.sdl.surface_base_pointer.as_ptr().offset(self.screen as isize * WIDTH as isize) }, self.sdl.surface_pitch))
  }

  fn update_screen(&self) {
    // Commented out: Degrades performance over time
    // self.sdl.screen_update_tx.send(self.screen).unwrap();
  }
}
