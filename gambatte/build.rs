extern crate cc;

use std::path::Path;

fn main() {
  cc::Build::new()
      .cpp(true)
      .flag_if_supported("-Wno-unused-parameter")
      .flag_if_supported("-Wno-array-bounds")
      .include(Path::new("libgambatte/src"))
      .file("libgambatte/src/cinterface.cpp")
      .file("libgambatte/src/cpu.cpp")
      .file("libgambatte/src/interruptrequester.cpp")
      .file("libgambatte/src/memory.cpp")
      .file("libgambatte/src/mem/cartridge.cpp")
      .file("libgambatte/src/mem/memptrs.cpp")
      .file("libgambatte/src/mem/rtc.cpp")
      .file("libgambatte/src/newstate.cpp")
      .file("libgambatte/src/sound.cpp")
      .file("libgambatte/src/tima.cpp")
      .file("libgambatte/src/video.cpp")
      .file("libgambatte/src/video/lyc_irq.cpp")
      .file("libgambatte/src/video/ly_counter.cpp")
      .file("libgambatte/src/video/next_m0_time.cpp")
      .file("libgambatte/src/video/ppu.cpp")
      .file("libgambatte/src/video/sprite_mapper.cpp")
      .compile("gambatte");
}