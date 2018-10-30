#![feature(test)]

extern crate montas;
extern crate test;

use montas::gambatte::*;
use montas::gambatte::inputs::*;
use montas::gb::*;
use montas::rom::*;
use montas::segment::*;
use montas::segment::*;
use test::{Bencher, black_box};

// #[bench]
// fn new_game_screen(b: &mut Bencher) {
//   Gambatte::init_screens(1 /* num screens */, 1 /* scale */);
//   let mut gb = Gb::<Crystal>::create(Gambatte::create_on_screen(0 /* screen */, false /* equal length frames */));

//   b.iter(|| {
//     gb.restore_initial_state();
//     gb.input(A); gb.step();
//     gb.input(START); gb.step();
//     gb.input(D); gb.step();
//     gb.input(L|A); gb.step();
//     gb.input(B); gb.step();
//     gb.input(A); gb.step();
//     black_box(&mut gb);
//   });
// }

#[bench]
fn new_game_step(b: &mut Bencher) {
  let mut gb = Gb::<Crystal>::create(Gambatte::create(false /* equal length frames */));

  b.iter(|| {
    gb.restore_initial_state();
    gb.input(A); gb.step();
    black_box(&mut gb);
  });
}

#[bench]
fn new_game_movesegment(b: &mut Bencher) {
  let mut gb = Gb::<Crystal>::create(Gambatte::create(false /* equal length frames */));

  b.iter(|| {
    gb.restore_initial_state();
    let states = vec![gb.save()];
    let sb = MoveSegment::new(A).execute(&mut gb, states);
    black_box(sb);
  });
}

#[bench]
fn new_game_direct_gambatte(b: &mut Bencher) {
  let mut gambatte = Gambatte::create(false /* equal length frames */);
  gambatte.load_gbc_bios("roms/gbc_bios.bin");
  gambatte.load_rom("roms/crystal.gbc");
  let initial_gambatte_state = gambatte.save_state();

  b.iter(|| {
    gambatte.load_state(&initial_gambatte_state);
    gambatte.set_input(A);
    for _i in 0..479 { gambatte.step(); }
    black_box(&mut gambatte);
  });
}
