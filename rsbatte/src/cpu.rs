use crate::mem::Memory;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct TwoRegisters {
  #[cfg(target_endian = "little")] r2: u8,
  r1: u8,
  #[cfg(target_endian = "big")] r2: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
union RegisterPair {
  r16: u16,
  t: TwoRegisters,
}
impl Default for RegisterPair {
  #[inline]
  fn default() -> Self { RegisterPair { r16: 0 } }
}
impl RegisterPair {
  fn inc(&mut self) {
    unsafe { self.r16 = self.r16.wrapping_add(1) }
  }
}

#[derive(Default)]
struct Registers {
  a: u8,
  bc: RegisterPair,
  de: RegisterPair,
  hl: RegisterPair,
  pc: u16,
  sp: u16,

  czf: RegisterPair,
  hf: u8,
}
impl Registers {
  #[inline(always)] fn b(&mut self) -> &mut u8 { unsafe { &mut self.bc.t.r1 } }
  #[inline(always)] fn c(&mut self) -> &mut u8 { unsafe { &mut self.bc.t.r2 } }
  #[inline(always)] fn d(&mut self) -> &mut u8 { unsafe { &mut self.de.t.r1 } }
  #[inline(always)] fn e(&mut self) -> &mut u8 { unsafe { &mut self.de.t.r2 } }
  #[inline(always)] fn h(&mut self) -> &mut u8 { unsafe { &mut self.hl.t.r1 } }
  #[inline(always)] fn l(&mut self) -> &mut u8 { unsafe { &mut self.hl.t.r2 } }
  #[inline(always)] fn bc(&mut self) -> &mut u16 { unsafe { &mut self.bc.r16 } }
  #[inline(always)] fn de(&mut self) -> &mut u16 { unsafe { &mut self.de.r16 } }
  #[inline(always)] fn hl(&mut self) -> &mut u16 { unsafe { &mut self.hl.r16 } }
  #[inline(always)] fn cf(&mut self) -> &mut u8 { unsafe { &mut self.czf.t.r1 } }
  #[inline(always)] fn zf(&mut self) -> &mut u8 { unsafe { &mut self.czf.t.r2 } }
  #[inline(always)] fn czf(&mut self) -> &mut u16 { unsafe { &mut self.czf.r16 } }
}

pub struct CPU {
  hit_interrupt_address: i32,

  // savable state
  memory: Memory,
  cycle_counter: u64,
  last_sound_update_cycle: u64,
  reg: Registers,
}

impl CPU {
  pub fn new(rom_data: &[u8]) -> CPU {
    let memory = Memory::new(rom_data);
    CPU {
      hit_interrupt_address: 0,

      memory,
      cycle_counter: 8,
      last_sound_update_cycle: 8,
      reg: Registers::default(),
    }
  }
  pub fn reset(&mut self) {
    self.memory.reset();
    self.cycle_counter = 8;
    self.last_sound_update_cycle = 8;
    self.reg = Registers::default();
  } 

  pub fn run_for(&mut self, samples: &mut u32) -> bool {
    self.process(u64::from(*samples) * 2);
    let has_blit = self.memory.has_blit(self.cycle_counter);

    *samples = ((self.cycle_counter - self.last_sound_update_cycle) >> 1) as u32;
    self.last_sound_update_cycle += u64::from(*samples) << 1;
    has_blit
  }

  #[inline(always)] fn pc_add(&mut self, offset: u16) { self.reg.pc = self.reg.pc.wrapping_add(offset); }
  #[inline(always)] fn pc_read(&mut self) -> u8 { let value = self.memory.read_memory_map(self.reg.pc); self.pc_add(1); self.cycle_counter += 4; value }
  #[inline(always)] fn pc_read_u16_le(&mut self) -> u16 { let value = self.memory.read_memory_map_u16_le(self.reg.pc); self.pc_add(2); self.cycle_counter += 8; value }
  #[inline(always)] fn write(&mut self, address: u16, value: u8) { self.memory.write(address, value, self.cycle_counter); self.cycle_counter += 4; }

  fn handle_oam_dma_call(&mut self) {
    self.memory.oam_dma(self.memory.read_ioamhram(0xff81));
    self.cycle_counter += 8 + 12 + 8 + 16 * 40 - 4 + 16; // TODO check when used
    // register a, stack values and flags not used
  }

  fn process(&mut self, cycles: u64) {
    self.hit_interrupt_address = 0;
    self.memory.set_end_time(self.cycle_counter, cycles);

    while self.memory.is_active() {
      if self.memory.is_halted() {
        if self.cycle_counter < self.memory.next_event_time() {
          self.cycle_counter = (self.memory.next_event_time() + 3) & !3;
        }
      } else {
        while self.memory.next_event_time() < self.cycle_counter {
          debug_assert!(self.reg.pc < 0x8000); // in ROM
          {
            let rom_address = self.memory.is_interrupt(self.reg.pc);
            if rom_address > 0 {
              self.hit_interrupt_address = if rom_address < 0x4000 { rom_address as i32 } else {
                ((rom_address as i32 >> 14) << 16) | 0x4000 | (rom_address as i32 & 0x3fff)
              };
              self.memory.set_end_time(self.cycle_counter, 0);
              continue;
            }
          }
          match self.pc_read() {
            0x01 => { // ld bc, d16
              *self.reg.bc() = self.pc_read_u16_le();
            },
            0x02 => { // ld (bc), a
            let bc = *self.reg.bc();
              self.write(bc, self.reg.a);
            },
            0x03 => { // inc bc
              self.reg.bc.inc();
              self.cycle_counter += 4;
            },
            0x04 => { // inc b
              *self.reg.b() = self.reg.b().wrapping_add(1);
              *self.reg.zf() = *self.reg.b();
            },
            0x05 => { // dec b
              *self.reg.b() = self.reg.b().wrapping_sub(1);
              *self.reg.zf() = *self.reg.b();
            },
            0x06 => { // ld b, d8
              *self.reg.b() = self.pc_read();
            },
            0x07 => { // rlca
              self.reg.a = self.reg.a << 1 | self.reg.a >> 7;
            },
            0x08 => { // ld (a16), sp
              let addr = self.pc_read_u16_le();
              self.write(addr, self.reg.sp as u8);
              self.write(addr.wrapping_add(1), (self.reg.sp >> 8) as u8);
            },
            0x0c => { // inc c
              *self.reg.c() = self.reg.c().wrapping_add(1);
              *self.reg.zf() = *self.reg.c();
            },
            0x0d => { // dec c
              *self.reg.c() = self.reg.c().wrapping_sub(1);
              *self.reg.zf() = *self.reg.c();
            },
            0x0e => { // ld c, d8
              *self.reg.c() = self.pc_read();
            },
            0x0f => { // rrca
              *self.reg.cf() = self.reg.a;
              self.reg.a = self.reg.a << 7 | self.reg.a >> 1;
            },
            0x11 => { // ld de, d16
              *self.reg.de() = self.pc_read_u16_le();
            },
            0x12 => { // ld (de), a
              let de = *self.reg.de();
              self.write(de, self.reg.a);
            },
            0x13 => { // inc de
              self.reg.de.inc();
              self.cycle_counter += 4;
            },
            0x14 => { // inc d
              *self.reg.d() = self.reg.d().wrapping_add(1);
              *self.reg.zf() = *self.reg.d();
            },
            0x15 => { // dec d
              *self.reg.d() = self.reg.d().wrapping_sub(1);
              *self.reg.zf() = *self.reg.d();
            },
            0x16 => { // ld d, d8
              *self.reg.d() = self.pc_read();
            },
            0x17 => { // rla
              let oldcf = *self.reg.cf() & 1;
              *self.reg.czf() = u16::from(self.reg.a) << 1;
              self.reg.a = *self.reg.zf() | oldcf;
            },
            0x1c => { // inc e
              *self.reg.e() = self.reg.e().wrapping_add(1);
              *self.reg.zf() = *self.reg.e();
            },
            0x1d => { // dec e
              *self.reg.e() = self.reg.e().wrapping_sub(1);
              *self.reg.zf() = *self.reg.e();
            },
            0x1e => { // ld e, d8
              *self.reg.e() = self.pc_read();
            },
            0x1f => { // rra
              let oldcf = *self.reg.cf() << 7;
              *self.reg.cf() = self.reg.a;
              self.reg.a = self.reg.a >> 1 | oldcf;
            },
            0x21 => { // ld hl, d16
              *self.reg.hl() = self.pc_read_u16_le();
            },
            0x23 => { // inc hl
              self.reg.hl.inc();
              self.cycle_counter += 4;
            },
            0x24 => { // inc h
              *self.reg.h() = self.reg.h().wrapping_add(1);
              *self.reg.zf() = *self.reg.h();
            },
            0x25 => { // dec h
              *self.reg.h() = self.reg.h().wrapping_sub(1);
              *self.reg.zf() = *self.reg.h();
            },
            0x26 => { // ld h, d8
              *self.reg.h() = self.pc_read();
            },
            0x2c => { // inc l
              *self.reg.l() = self.reg.l().wrapping_add(1);
              *self.reg.zf() = *self.reg.l();
            },
            0x2d => { // dec l
              *self.reg.l() = self.reg.l().wrapping_sub(1);
              *self.reg.zf() = *self.reg.l();
            },
            0x2e => { // ld l, d8
              *self.reg.l() = self.pc_read();
            },
            0x31 => { // ld sp, d16
              self.reg.sp = self.pc_read_u16_le();
            },
            0x33 => { // inc sp
              self.reg.sp = self.reg.sp.wrapping_add(1);
              self.cycle_counter += 4;
            },
            0x3c => { // inc a
              self.reg.a = self.reg.a.wrapping_add(1);
              *self.reg.zf() = self.reg.a;
            },
            0x3d => { // dec a
              self.reg.a = self.reg.a.wrapping_sub(1);
              *self.reg.zf() = self.reg.a;
            },
            0x3e => { // ld a, d8
              self.reg.a = self.pc_read();
            },
            _ => {} // do nothing
          }
        }
      }
      self.cycle_counter = self.memory.event(self.cycle_counter);
    }
    panic!("unimplemented");
  }
}
