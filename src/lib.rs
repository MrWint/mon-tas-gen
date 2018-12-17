#![feature(ptr_internals)]
#![feature(unsized_locals)]

extern crate bincode;
extern crate byteorder;
extern crate gambatte;
extern crate sdl2;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate flate2;
extern crate num_cpus;

pub mod ftii;
pub mod gb;
pub mod gbexecutor;
pub mod rom;
pub mod sdl;
pub mod segment;
pub mod statebuffer;
pub mod util;
