#![feature(ptr_offset_from)]

pub mod cpu;
pub mod mem;
pub mod newstate;

pub type RsBatte = crate::cpu::CPU;
