#![feature(test)]

use gambatte::*;
use gambatte::inputs::*;
use montas::gb::*;
use montas::rom::*;
use montas::segment::*;
use montas::sdl::*;

#[test]
fn meta_test() {
  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let mut gb = Gb::<Crystal>::create(false /* equal length frames */, SdlScreen::new(sdl, 0 /* screen */));
  let states = vec![gb.save()];

  let sb = DelaySegment::new(MoveSegment::with_metric(A, NullMetric {})).with_debug_output(true).execute(&mut gb, states);
  let sb = MoveSegment::new(START).with_max_skips(10).execute(&mut gb, sb);
  let sb = MoveSegment::new(D).execute(&mut gb, sb); // options
  let sb = MoveSegment::new(L|A).execute(&mut gb, sb); // fast options
  let sb = MoveSegment::new(B).execute(&mut gb, sb); // back
  MoveSegment::new(A).execute(&mut gb, sb); // new game
}
