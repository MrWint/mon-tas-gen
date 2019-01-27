#![feature(test)]



extern crate test;

use gambatte::*;
use gambatte::inputs::*;
use montas::gb::*;
use montas::gbexecutor::*;
use montas::rom::*;
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
  let mut gb = Gb::<Crystal>::create(false /* equal length frames */, NoScreen {});

  b.iter(|| {
    gb.restore_initial_state();
    gb.input(A); gb.step();
    black_box(&mut gb);
  });
}

#[bench]
fn new_game_movesegment(b: &mut Bencher) {
  let mut gbe = SingleGb::<Crystal>::no_screen();
  let states = vec![gbe.get_initial_state()];

  b.iter(|| {
    let sb = MoveSegment::new(A).execute(&mut gbe, &states);
    black_box(sb);
  });
}

#[bench]
fn new_game_direct_gambatte(b: &mut Bencher) {
  let mut gambatte = Gambatte::create("roms/gbc_bios.bin", "roms/crystal.gbc", false /* equal length frames */, NoScreen {});
  let initial_gambatte_state = gambatte.save_state();

  b.iter(|| {
    gambatte.load_state(&initial_gambatte_state);
    gambatte.set_input(A);
    for _i in 0..479 { gambatte.step(); }
    black_box(&mut gambatte);
  });
}
