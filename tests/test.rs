#![feature(test)]

extern crate montas;
extern crate test;

use montas::gambatte::*;
use montas::gambatte::inputs::*;
use montas::gb::*;
use montas::rom::*;
use montas::segment::*;

#[test]
fn meta_test() {
  Gambatte::init_screens(1 /* num screens */, 3 /* scale */);
  let mut gb = Gb::<Crystal>::create(Gambatte::create_on_screen(0 /* screen */, false /* equal length frames */));
  let states = vec![gb.save()];

  let sb = DelaySegment::new(MoveSegment::with_metric(A, NullMetric::new())).with_debug_output(true).execute(&mut gb, states);
  let sb = MoveSegment::new(START).with_max_skips(10).execute(&mut gb, sb);
  let sb = MoveSegment::new(D).execute(&mut gb, sb); // options
  let sb = MoveSegment::new(L|A).execute(&mut gb, sb); // fast options
  let sb = MoveSegment::new(B).execute(&mut gb, sb); // back
  MoveSegment::new(A).execute(&mut gb, sb); // new game
}
