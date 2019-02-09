#![feature(test)]

use gambatte::inputs::*;
use montas::gbexecutor::*;
use montas::rom::*;
use montas::segment::*;

#[test]
fn meta_test() {
  let mut gbe = SingleGb::<Crystal>::with_screen();
  let states = vec![gbe.get_initial_state()];

  let sb = DelaySegment::new(MoveSegment::with_metric(A, NullMetric {})).execute(&mut gbe, states);
  let sb = MoveSegment::new(START).with_max_skips(10).execute(&mut gbe, sb);
  let sb = MoveSegment::new(D).execute(&mut gbe, sb); // options
  let sb = MoveSegment::new(L|A).execute(&mut gbe, sb); // fast options
  let sb = MoveSegment::new(B).execute(&mut gbe, sb); // back
  MoveSegment::new(A).execute(&mut gbe, sb); // new game
}
