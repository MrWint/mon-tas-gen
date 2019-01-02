pub struct MemPtrs {
  memchunk: Vec<u8>,
  interrupt_memchunk: Vec<bool>,
  memory_map: [*mut u8; 16],
}

const wrambanks: usize = 8;
const srambanks: usize = 4; // 32kB RAM

impl MemPtrs {
  pub fn new(rom_data: &[u8]) -> MemPtrs {
    assert!(rom_data[0x0149] == 0x03); // 32kB RAM

    let rom_size = rom_data.len();
    assert!(rom_size.count_ones() == 1); // is power of 2
    assert!(rom_size >= 0x8000); // at least 2 ROM banks
    let rombanks = rom_size >> 14; // rom_size / 0x4000;

	  let memchunk_len = rombanks * 0x4000 + 0x4000 + srambanks * 0x2000 + wrambanks * 0x1000;
    let mut memchunk = vec![0u8; memchunk_len];
    memchunk[0..rom_size].copy_from_slice(rom_data);

    let rom_data_ptr = memchunk.as_mut_ptr();
    let vram_data_ptr = unsafe { rom_data_ptr.add(rom_size) };
    let sram_data_ptr = unsafe { vram_data_ptr.add(0x4000) };
    let wram_data_ptr = unsafe { sram_data_ptr.add(srambanks * 0x2000) };

    let interrupt_memchunk = vec![false; rombanks * 0x4000];

    let memory_map = unsafe { [
      rom_data_ptr, rom_data_ptr, rom_data_ptr, rom_data_ptr, // ROM bank 0 loaded
      rom_data_ptr, rom_data_ptr, rom_data_ptr, rom_data_ptr, // ROM bank 1 loaded
      vram_data_ptr.offset(-0x8000), vram_data_ptr.offset(-0x8000), // VRAM bank 0 loaded
      sram_data_ptr.offset(-0xa000), vram_data_ptr.offset(-0xa000), // SRAM bank 0 loaded (assuming no reads or writes while SRAM is disabled)
      wram_data_ptr.offset(-0xc000), wram_data_ptr.offset(-0xc000), // WRAM bank 0+1 loaded
      0 as *mut u8, 0 as *mut u8, // no echo RAM loaded
    ] };

    MemPtrs {
      memchunk,
      interrupt_memchunk,
      memory_map,
    }
  }
}