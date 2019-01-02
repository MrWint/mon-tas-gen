use crate::newstate::{SyncObject, SyncState};

pub struct MemPtrs {
  memchunk: Vec<u8>,
  interrupt_memchunk: Vec<bool>,
  rom_data_ptr: *mut u8,
  vram_data_ptr: *mut u8,
  sram_data_ptr: *mut u8,
  wram_data_ptr: *mut u8,
  memory_map: [*mut u8; 16],
}
impl SyncObject for MemPtrs {
  fn sync<S: SyncState>(&mut self, s: &mut S) {
    s.sync(unsafe { &mut *(self.vram_data_ptr as *mut [u8; RAM_SIZE]) });
    for i in 0..16 { s.sync_offset(&mut self.memory_map[i], self.memchunk.as_mut_ptr()); }
  }
}

const WRAM_BANKS: usize = 8;
const SRAM_BANKS: usize = 4; // 32kB RAM
const RAM_SIZE: usize = 0x4000 + SRAM_BANKS * 0x2000 + WRAM_BANKS * 0x1000;

impl MemPtrs {
  pub fn new(rom_data: &[u8]) -> MemPtrs {
    assert!(rom_data[0x0149] == 0x03); // 32kB RAM

    let rom_size = rom_data.len();
    assert!(rom_size.count_ones() == 1); // is power of 2
    assert!(rom_size >= 0x8000); // at least 2 ROM banks
    let rombanks = rom_size >> 14; // rom_size / 0x4000;

	  let memchunk_len = rombanks * 0x4000 + RAM_SIZE;
    let mut memchunk = vec![0u8; memchunk_len];
    memchunk[0..rom_size].copy_from_slice(rom_data);

    let rom_data_ptr = memchunk.as_mut_ptr();
    let vram_data_ptr = unsafe { rom_data_ptr.add(rom_size) };
    let sram_data_ptr = unsafe { vram_data_ptr.add(0x4000) };
    let wram_data_ptr = unsafe { sram_data_ptr.add(SRAM_BANKS * 0x2000) };

    let interrupt_memchunk = vec![false; rombanks * 0x4000];

    let memory_map = unsafe { [
      rom_data_ptr, rom_data_ptr, rom_data_ptr, rom_data_ptr, // ROM bank 0 loaded
      rom_data_ptr, rom_data_ptr, rom_data_ptr, rom_data_ptr, // ROM bank 1 loaded
      vram_data_ptr.offset(-0x8000), vram_data_ptr.offset(-0x8000), // VRAM bank 0 loaded
      sram_data_ptr.offset(-0xa000), vram_data_ptr.offset(-0xa000), // SRAM bank 0 loaded (assuming no reads or writes while SRAM is disabled)
      wram_data_ptr.offset(-0xc000), wram_data_ptr.offset(-0xc000), // WRAM bank 0+1 loaded
      wram_data_ptr.offset(-0xe000), wram_data_ptr.offset(-0xe000), // echo RAM loaded
    ] };

    MemPtrs {
      memchunk,
      interrupt_memchunk,
      memory_map,
      rom_data_ptr,
      vram_data_ptr,
      sram_data_ptr,
      wram_data_ptr,
    }
  }

  pub fn set_rombank(&mut self, new_rombank: u8) {
    assert!(new_rombank >= 1);
    let base_ptr = unsafe { self.rom_data_ptr.offset(isize::from(new_rombank) * 0x4000 - 0x4000) };
    self.memory_map[0x4] = base_ptr;
    self.memory_map[0x5] = base_ptr;
    self.memory_map[0x6] = base_ptr;
    self.memory_map[0x7] = base_ptr;
  }

  pub fn set_vrambank(&mut self, new_vrambank: u8) {
    assert!(new_vrambank <= 1);
    let base_ptr = unsafe { self.vram_data_ptr.offset(isize::from(new_vrambank) * 0x2000 - 0x8000) };
    self.memory_map[0x8] = base_ptr;
    self.memory_map[0x9] = base_ptr;
  }

  pub fn set_srambank(&mut self, new_srambank: u8) {
    assert!(new_srambank <= 3);
    let base_ptr = unsafe { self.sram_data_ptr.offset(isize::from(new_srambank) * 0x2000 - 0xA000) };
    self.memory_map[0xA] = base_ptr;
    self.memory_map[0xB] = base_ptr;
  }

  pub fn set_wrambank(&mut self, new_wrambank: u8) {
    assert!(new_wrambank >= 1 && new_wrambank <= 7);
    // let new_wrambank = if new_wrambank & 0x7 == 0 { 1 } else new_wrambank & 0x7; 
    self.memory_map[0xD] = unsafe { self.wram_data_ptr.offset(isize::from(new_wrambank) * 0x1000 - 0xD000) };
    self.memory_map[0xF] = unsafe { self.wram_data_ptr.offset(isize::from(new_wrambank) * 0x1000 - 0xF000) };
  }

  pub fn set_interrupt(&mut self, rom_address: usize) {
    self.interrupt_memchunk[rom_address] = true;
  }
  pub fn clear_interrupt(&mut self, rom_address: usize) {
    self.interrupt_memchunk[rom_address] = false;
  }
  pub fn is_interrupt(&mut self, memory_address: u16) {
    self.interrupt_memchunk[rom_address] = false;
  }
}