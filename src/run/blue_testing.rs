use crate::run::*;
use gambatte::inputs::*;
use montas::segment::overworld::gen1::*;

#[allow(dead_code)]
pub fn start() {
  let mut r: GbRunner<Blue, _> = GbRunner::single_with_screen();

  run(&mut r);

  r.run(IdentifyInputSegment::new());

  r.debug_segment_end("temp/blue_test");
}

fn run<G: GbExecutor<Blue>>(r: &mut GbRunner<Blue, G>) {
  r.run_debug(DelaySegment::new(MoveSegment::with_metric_fn(START, |_gb| Some(()))));
  // r.run(MoveSegment::new(START).with_max_skips(10));
  r.debug_print_states();
  r.run(MoveSegment::new(A).with_max_skips(10));
  r.debug_print_states();
  r.run(MoveSegment::new(START).with_max_skips(10));
  r.debug_print_states();
//    r.run(MoveSegment::new(A).with_max_skips(10));
  r.run(MoveSegment::new(D|A)); // options
  r.run(MoveSegment::new(L)); // text speed fast
  r.run(MoveSegment::new(D)); // battle animations...
  r.run(MoveSegment::new(L)); // ...off
  r.run(MoveSegment::new(D)); // battle style...
  r.run(MoveSegment::new(L)); // ...set
  r.run(MoveSegment::new(B)); // back
  r.run(MoveSegment::new(A)); // new game
  r.debug_print_states();
  r.run(SkipTextsSegment::new(13).with_confirm_input(B)); // skip texts until player name
  r.run(MoveSegment::new(D|A)); // Name: Blue
  r.run(SkipTextsSegment::new(5).with_confirm_input(B)); // skip texts until rival name
  r.run(MoveSegment::new(D|A)); // Name: Red
  r.run(SkipTextsSegment::new(7)); // skip texts until game start
  r.run(TextSegment::new()); // ...awaits, let's go
  r.debug_print_states();
  r.save("blue_test");

  r.load("blue_test");
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();

  r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(L, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(L, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(L, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(L, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(IdentifyInputSegment::new());
  r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();

  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();

  r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.save("blue_test");

  r.load("blue_test");
  r.run(SkipTextsSegment::new(6)); // it's dangerous to go outside, take this
  r.run(VerifyInputSegment::new("HoldTextDisplayOpen"));
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
}