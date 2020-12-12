use crate::run::*;
use gambatte::inputs::*;
use montas::segment::overworld::gen1::*;

#[allow(dead_code)]
pub fn start() {
  let mut r: GbRunner<Blue> = GbRunner::single_with_screen(&[]);

  run(&mut r);

  r.run(IdentifyInputSegment::new());

  r.debug_segment_end("temp/blue_test");
}

fn run(r: &mut GbRunner<Blue>) {
  // r.run(MoveSegment::new(START)); // logo
  // r.run(MoveSegment::new(A)); // intro cutscene
  // r.run(MoveSegment::new(START)); // main menu
  // r.run(MoveSegment::new(A)); // new game
  // r.run(SkipTextsSegment::new(13).with_confirm_input(B)); // skip texts until player name
  // // { // Name: Blue
  // //   r.run(MoveSegment::new(D|A)); // Name: Blue
  // // }
  // { // Name: I
  //   r.run(MoveSegment::new(A)); // Custom Name
  //   r.run(MoveSegment::new(L)); // move cursor
  //   r.run(MoveSegment::new(A)); // I
  //   r.run(MoveSegment::new(START)); // confirm
  // }
  // r.run(SkipTextsSegment::new(5).with_confirm_input(B)); // skip texts until rival name
  // // { // Name: Red
  // //   r.run(MoveSegment::new(D|A)); // Name: Red
  // // }
  // { // Name: U (+3 frames)
  //   r.run(MoveSegment::new(A)); // Custom Name
  //   r.run(MoveSegment::new(R)); // move cursor
  //   r.run(MoveSegment::new(D)); // move cursor
  //   r.run(MoveSegment::new(R)); // move cursor
  //   r.run(MoveSegment::new(D)); // move cursor
  //   r.run(MoveSegment::new(A)); // U
  //   r.run(MoveSegment::new(START)); // confirm
  // }
  // r.run(SkipTextsSegment::new(7)); // skip texts until game start
  // r.run(TextSegment::new()); // ...awaits, let's go
  // r.save("blue_test");

  r.load("blue_test");
  r.run(MoveSegment::with_metric(START, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::DisplayText { id: 0 }))); // Open main menu
  r.run(MoveSegment::new(U)); // move cursor
  r.run(MoveSegment::new(NIL)); // move cursor
  r.run(MoveSegment::new(U)); // move cursor
  r.run(MoveSegment::new(A)); // options
  r.run(MoveSegment::new(L)); // text speed fast
  r.run(MoveSegment::new(D)); // battle animations...
  r.run(MoveSegment::new(L)); // ...off
  r.run(MoveSegment::new(D)); // battle style...
  r.run(MoveSegment::new(L)); // ...set
  r.run(MoveSegment::new(B)); // back
  r.run(MoveSegment::new(START)); // close main menu

  r.run(WalkStepSegment::new(R));
  r.run(WalkStepSegment::new(U));
  r.run(WalkStepSegment::new(U));
  r.run(WalkStepSegment::new(U));
  r.run(WalkStepSegment::new(U));
  r.run(WalkStepSegment::new(U));
  r.run(WalkStepSegment::new(R));
  r.run(WalkStepSegment::new(R));
  r.run(WalkStepSegment::new(R));

  r.run(OverworldMoveSegment::turn(L));
  r.run(WalkStepSegment::new(D));
  r.run(WalkStepSegment::new(D));
  r.run(WalkStepSegment::new(D));
  r.run(WalkStepSegment::new(D));
  r.run(WalkStepSegment::new(D));
  r.run(WalkStepSegment::new(L));
  r.run(WalkStepSegment::new(L));
  r.run(WalkStepSegment::new(L));
  r.run(WalkStepSegment::new(L));
  r.run(WalkStepSegment::new(D).into(OverworldInteractionResult::NoOverworldInput));
  r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp

  r.run(OverworldMoveSegment::turn(L));
  r.run(WalkStepSegment::new(R));
  r.run(WalkStepSegment::new(R));
  r.run(WalkStepSegment::new(R));
  r.run(WalkStepSegment::new(R));
  r.run(WalkStepSegment::new(R));
  r.run(WalkStepSegment::new(U));
  r.run(WalkStepSegment::new(U));
  r.run(WalkStepSegment::new(U));
  r.run(WalkStepSegment::new(U));
  r.run(WalkStepSegment::new(U));
  r.run(OverworldMoveSegment::wait()); // Skip PalletTownScript0
//   r.save("blue_test");

//   r.load("blue_test");
  r.run(TextSegment::new().with_allowed_end_inputs(A)); // it's dangerous to go outside, take this
  r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  r.run(SkipTextsSegment::new(6).with_confirm_input(B)); // it's dangerous to go outside, take this
  r.run(OverworldMoveSegment::wait()); // Skip PalletTownScript load
  r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(L)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(R)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(R)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(R)); // Follow oak
  r.run(OverworldMoveSegment::auto_walk(U)); // Follow oak
  r.run(SkipTextsSegment::new(18).with_confirm_input(B)); // oak speech choose a mon
  r.run(OverworldMoveSegment::turn(D));
  r.run(WalkStepSegment::new(D));
  r.run(WalkStepSegment::new(R));
  r.run(WalkStepSegment::new(R));
  r.run(OverworldMoveSegment::collide(U));
  r.run(OverworldMoveSegment::interact_with(3));
  r.run(VerifyInputSegment::new("WaitForTextScrollButtonPress").with_input(B)); // skip squirtle image
  r.run(VerifyInputSegment::new("ShowPokedexDataInternal").with_input(A)); // skip squirtle dex
  r.run(SkipTextsSegment::new(1).with_confirm_input(B).with_buffer_size(256)); // choose squirtle
  r.run(SkipTextsSegment::new(1).with_confirm_input(A).with_buffer_size(256)); // choose squirtle
  r.run(SkipTextsSegment::new(1).with_confirm_input(B).with_buffer_size(2048)); // choose squirtle
  r.run(SkipTextsSegment::new(1).with_confirm_input(B).with_skip_ends(4).with_buffer_size(2048)); // received a // squirtle // ! // Do you want
  r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A).with_buffer_size(2048)); // nickname to // squirtle // ?
  r.run(MoveSegment::new(NIL).with_buffer_size(2048)); // delay for A repress
  r.run(MoveSegment::new(A).with_buffer_size(2048)); // delay for A repress
  r.run(DelaySegment::new(MoveSegment::with_metric(START, Gen1DVMetric {}.filter(|v| {
    // if v.atk < 15 || v.def < 11 || v.spc < 12 || v.spd < 7 || v.def & 1 == 0 || (v.spd & 1 == 0 && v.spc & 1 == 0) { return false; } // totodile
    if v.atk < 15 || v.spc < 15 || v.spd < 15 { return false; } // squirtle DVs
    log::info!("Chosen DVs: {:?}", v); true
  }).into_unit())));

  //r.run_debug(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().into_unit()));
}