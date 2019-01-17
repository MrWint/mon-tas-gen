use crate::mem::rtc::{Rtc};
use crate::mem::memptrs::MemPtrs;
use crate::newstate::{SyncObject, SyncState};

pub struct Cartridge {
  mem_ptrs: MemPtrs,
  rtc: Option<Rtc>,
  cur_srambank: u8,
}
impl SyncObject for Cartridge {
  fn sync<S: SyncState>(&mut self, s: &mut S) {
    s.sync_object(&mut self.mem_ptrs);
    if let Some(ref mut rtc) = self.rtc { s.sync_object(rtc); }
    s.sync(&mut self.cur_srambank);
  }
}

impl Cartridge {
  pub fn new(rom_data: &[u8]) -> Cartridge {
    assert!(rom_data[0x0147] >= 0x0F && rom_data[0x0147] <= 0x1e); // MBC3 or MBC5
    let mem_ptrs = MemPtrs::new(rom_data);

    let rtc = if rom_data[0x0147] <= 0x10 { Some(Rtc::new()) } else { None };

    Cartridge {
      mem_ptrs,
      rtc,
      cur_srambank: 0,
    }
  }

	pub fn rom_write(&mut self, address: u16, data: u8) {
    if address == 0x2000 { // ROM bank switch
      self.mem_ptrs.set_rombank(if data == 0 { 1 } else { data })
    } else if address == 0x4000 { // RAM bank switch
      self.cur_srambank = data;
      if self.cur_srambank <= 0x3 { self.mem_ptrs.set_srambank(self.cur_srambank); }
    } else if address == 0x6000 { // RTC latch
      if let Some(ref mut rtc) = self.rtc { rtc.latch(); }
    }
	}

  pub fn reset(&mut self) {
    self.mem_ptrs.reset();
    if let Some(ref mut rtc) = self.rtc { rtc.reset(); }
    self.cur_srambank = 0;
  }

  #[inline]
  pub fn read(&self, address: u16) -> u8 {
    if address == 0xa000 && self.cur_srambank >= 0x8 { self.rtc.as_ref().unwrap().read_rtc(self.cur_srambank) } else { self.mem_ptrs.read(address) }
  }

  #[inline] pub fn read_memory_map(&self, address: u16) -> u8 { self.mem_ptrs.read(address) }
  #[inline] pub fn read_memory_map_u16_le(&self, address: u16) -> u16 { self.mem_ptrs.read_u16_le(address) }
  #[inline] pub fn set_interrupt(&mut self, rom_address: usize) { self.mem_ptrs.set_interrupt(rom_address) }
  #[inline] pub fn clear_interrupt(&mut self, rom_address: usize) { self.mem_ptrs.clear_interrupt(rom_address) }
  #[inline] pub fn is_interrupt(&mut self, address: u16) -> usize { self.mem_ptrs.is_interrupt(address) }
  #[inline]
  pub fn set_current_rtc(&mut self, current_rtc: std::num::Wrapping<u32>) {
    if let Some(ref mut rtc) = self.rtc { rtc.set_current_rtc(current_rtc) }
  }
}