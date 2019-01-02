#![allow(dead_code)]

use crate::newstate::{SyncObject, SyncState};
use std::num::Wrapping;

pub trait RtcCallback {
  /// Returns the current rtc clock value. It increases by 1 every second.
  fn current_rtc(&self) -> Wrapping<u32>;
}

const REGISTER_SECONDS: usize = 0x0;
const REGISTER_MINUTES: usize = 0x1;
const REGISTER_HOURS: usize = 0x2;
const REGISTER_DAYS_LOW: usize = 0x3;
const REGISTER_DAYS_HIGH: usize = 0x4;
pub struct Rtc {
  rtc_callback: Box<dyn RtcCallback>, // trait object to avoid generics.

  // savable state.
  registers: [u8; 5], // rtc registers, holding the latched time and flags.
  base_rtc: Wrapping<u32>, // Specifies rtc value at 0:00. When halted, current rtc is always 0.
  ready_to_latch: bool, // whether the last clock latch data input was 0
}
impl SyncObject for Rtc {
  fn sync<S: SyncState>(&mut self, s: &mut S) {
    s.sync(&mut self.registers);
    s.sync(&mut self.base_rtc.0);
    s.sync(&mut self.ready_to_latch);
  }
}

impl Rtc {
  pub fn new<R: 'static + RtcCallback>(rtc_callback: R) -> Rtc {
    Rtc {
      rtc_callback: Box::new(rtc_callback),
      registers: [0; 5],
      base_rtc: Wrapping(0),
      ready_to_latch: true,
    }
  }

  pub fn init_state(&mut self) {
    // base_rtc is kept in savedata.
    self.registers = [0; 5];
    self.ready_to_latch = true;
    // selected_register is not reset.
  }

  #[inline]
  fn is_halted(&self) -> bool {
    self.registers[REGISTER_DAYS_HIGH] & 0x40 != 0
  }

  #[inline]
  fn get_current_time(&self) -> u32 {
    (if self.is_halted() { Wrapping(0) } else { self.rtc_callback.current_rtc() } - self.base_rtc).0
  }

  fn set_days_high(&mut self, new_dh: u8) {
    let old_highdays = (self.get_current_time() / (86400 * 0x100)) & 0x1;
    self.base_rtc += Wrapping(old_highdays * (86400 * 0x100));
    self.base_rtc -= Wrapping(u32::from(new_dh & 0x1) * (86400 * 0x100));
    
    let new_is_halted = new_dh & 0x40 != 0;
    if new_is_halted && !self.is_halted() { // halt
      self.base_rtc -= self.rtc_callback.current_rtc();
    } else if !new_is_halted && self.is_halted() { // unhalt
      self.base_rtc += self.rtc_callback.current_rtc();
    }
  }

  fn set_days_low(&mut self, new_lowdays: u8) {
    let old_lowdays = (self.get_current_time() / 86400) & 0xFF;
    self.base_rtc += Wrapping(old_lowdays * 86400);
    self.base_rtc -= Wrapping(u32::from(new_lowdays) * 86400);
  }

  fn set_hours(&mut self, new_hours: u8) {
    let old_hours = (self.get_current_time() / 3600) % 24;
    self.base_rtc += Wrapping(old_hours * 3600);
    self.base_rtc -= Wrapping(u32::from(new_hours) * 3600);
  }

  fn set_minutes(&mut self, new_minutes: u8) {
    let old_minutes = (self.get_current_time() / 60) % 60;
    self.base_rtc += Wrapping(old_minutes * 60);
    self.base_rtc -= Wrapping(u32::from(new_minutes) * 60);
  }

  fn set_seconds(&mut self, new_seconds: u8) {
    let old_seconds = self.get_current_time() % 60;
    self.base_rtc += Wrapping(old_seconds);
    self.base_rtc -= Wrapping(u32::from(new_seconds));
  }

  pub fn read_rtc(&self, register: u8) -> u8 {
    self.registers[usize::from(register - 8)]
  }

  pub fn write_rtc(&mut self, register: u8, value: u8) {
    match register {
      0x8 => self.set_seconds(value),
      0x9 => self.set_minutes(value),
      0xa => self.set_hours(value),
      0xb => self.set_days_low(value),
      0xc => self.set_days_high(value),
      _ => panic!("write to invalid RTC register {:2x}, value {:2x}", register, value),
    }
    self.registers[usize::from(register - 8)] = value;
  }

  pub fn latch(&mut self, value: u8) {
    if value == 1 && self.ready_to_latch {
      let mut current_time = self.get_current_time();
      
      while current_time > 0x1FF * 86400 {
        self.base_rtc += Wrapping(0x1FF * 86400);
        current_time -= 0x1FF * 86400;
        self.registers[REGISTER_DAYS_HIGH] |= 0x80;
      }
      
      self.registers[REGISTER_DAYS_HIGH] &= 0xFE;
      self.registers[REGISTER_DAYS_HIGH] |= (current_time / (86400 * 0x100)) as u8 & 0x1;
      self.registers[REGISTER_DAYS_LOW] = (current_time / 86400) as u8;
      current_time %= 86400;
      
      self.registers[REGISTER_HOURS] = (current_time / 3600) as u8;
      current_time %= 3600;
      
      self.registers[REGISTER_MINUTES] = (current_time / 60) as u8;
      current_time %= 60;
      
      self.registers[REGISTER_SECONDS] = current_time as u8;
    }
    self.ready_to_latch = value == 0;
  }
}
