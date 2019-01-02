use crate::mem::rtc::{Rtc, RtcCallback};
use crate::mem::memptrs::MemPtrs;

struct Cartridge {
  mem_ptrs: MemPtrs,
  rtc: Option<Rtc>,
  cur_rambank: u8,
}

impl Cartridge {
  fn new<R: 'static + RtcCallback>(rom_data: &[u8], rtc_callback: Option<R>) -> Cartridge {
    assert!(rom_data[0x0147] >= 0x0F && rom_data[0x0147] <= 0x1e); // MBC3 or MBC5
    let mem_ptrs = MemPtrs::new(rom_data);

    let rtc = if rom_data[0x0147] <= 0x10 { Some(Rtc::new(rtc_callback.unwrap())) } else { None };

    Cartridge {
      mem_ptrs,
      rtc,
      cur_rambank: 1,
    }
  }
}