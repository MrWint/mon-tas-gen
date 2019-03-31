use crate::run::*;
use gambatte::inputs::*;
use montas::segment::overworld::gen1::*;

#[allow(dead_code)]
pub fn start() {
  let mut r: GbRunner<Yellow, _> = GbRunner::pool_with_screen();

  run(&mut r);

  r.run(IdentifyInputSegment::new());

  r.debug_segment_end("temp/yellow_test");
}

fn run<G: GbExecutor<Yellow>>(r: &mut GbRunner<Yellow, G>) {
  r.run(MoveSegment::new(START).with_max_skips(15).with_buffer_size(4096));
  r.debug_print_states();
  r.run(MoveSegment::new(A).with_max_skips(15).with_buffer_size(4096));
  r.debug_print_states();
  r.run(MoveSegment::new(START).with_max_skips(15).with_buffer_size(4096));
  r.debug_print_states();
  r.run_debug(DelaySegment::new(MoveSegment::with_metric(A, TrainerIDMetric{}.filter(|v| { v == &0x26F1 }).into_unit()))); // new game
  r.debug_print_states();
  r.save("yellow_after_tid");

  r.load("yellow_after_tid");
  r.run(SkipTextsSegment::new(13)); // skip texts until player name
  r.run(MoveSegment::new(D)); // Name: Yellow
  r.run(MoveSegment::new(A)); // Name: Yellow
  r.run(SkipTextsSegment::new(5)); // skip texts until rival name
  r.run(MoveSegment::new(D)); // Name: Blue
  r.run(MoveSegment::new(A)); // Name: Blue
  r.run(SkipTextsSegment::new(7)); // skip texts until game start
  r.run(TextSegment::new()); // ...awaits, let's go
  r.debug_print_states();
  r.run(MoveSegment::new(START));
  r.run(MoveSegment::new(U));
  r.run(MoveSegment::new(NIL));
  r.run(MoveSegment::new(U));
  r.run(MoveSegment::new(L|A));
  r.run(MoveSegment::new(B));
  r.run(MoveSegment::new(START));
  r.save("yellow_after_intro_256_div");

  r.load("yellow_after_intro_256_div");
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
r.run(MoveSegment::new(NIL));
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
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.save("yellow_grass_256_wait1_");

  r.load("yellow_grass_256_wait1_");
  r.run(TextSegment::new()); // Don't go out!
  r.run(VerifyInputSegment::new("HoldTextDisplayOpen"));
  r.run(SkipTextsSegment::new(2)); // oak in grass
  r.run(VerifyInputSegment::new("HoldTextDisplayOpen"));
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states(); // WildEncounter
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // Wild // Pikachu // appeared
  r.run(SkipTextsSegment::new(1).with_skip_ends(6)); // Oak used // Pokeball // ! // Alright // Pikachu // was
  r.run(SkipTextsSegment::new(1)); // caught
  r.run(SkipTextsSegment::new(1)); // oak in grass
  r.run(VerifyInputSegment::new("HoldTextDisplayOpen"));
  r.run(SkipTextsSegment::new(6)); // oak in grass
  r.run(VerifyInputSegment::new("HoldTextDisplayOpen"));
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(SkipTextsSegment::new(2)); // Fed up with waiting!
  r.run(VerifyInputSegment::new("HoldTextDisplayOpen"));
  r.run(SkipTextsSegment::new(9)); // oak speech
  r.run(VerifyInputSegment::new("HoldTextDisplayOpen"));
  r.run(SkipTextsSegment::new(2)); // What about me?
  r.run(VerifyInputSegment::new("HoldTextDisplayOpen"));
  r.run(SkipTextsSegment::new(2)); // oak speech
  r.run(VerifyInputSegment::new("HoldTextDisplayOpen"));
  r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.save("yellow_before_collect_pikachu_256_wait1_");

  r.load("yellow_before_collect_pikachu_256_wait1_");
  r.run(VerifyInputSegment::new("HoldTextDisplayOpen")); // Release A button to start sequence
  r.run(SkipTextsSegment::new(2)); // Rival snatches
  r.run(SkipTextsSegment::new(1).with_skip_ends(2));
  r.run(SkipTextsSegment::new(7)); // Rival snatches
  r.run(VerifyInputSegment::new("HoldTextDisplayOpen"));
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit())); r.debug_print_states();
  r.run(SkipTextsSegment::new(4)); // get Pikachu
  r.save("yellow_before_collect_pikachu_2_256_wait1_");

  r.load("yellow_before_collect_pikachu_2_256_wait1_");
  r.run(SkipTextsSegment::new(1).with_buffer_size(4096)); // get Pikachu
  r.run(SkipTextsSegment::new(1).with_skip_ends(4).with_buffer_size(4096)); // get Pikachu
  r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(A).with_buffer_size(4096)); // Nickname?
  r.run_debug(DelaySegment::new(MoveSegment::with_metric(B, Gen1DVMetric {}.filter(|v| {
    let dvs = u16::from(v.atk) << 12 | u16::from(v.def) << 8 | u16::from(v.spd) << 4 | u16::from(v.spc);
    let div = ((dvs >> 8) as u8).wrapping_sub(dvs as u8);
    if ::rand::random::<u8>() < 0x04 {
      print!("{:X}", div);
      // print!("/{:X}", v.div_state >> 6);
      // print!("/{}", to_human_readable_time(v.cycle_count));
      print!(" ");
    }
    if v.atk != 7 { return false; }
    if v.spc != 8 && v.spc != 9 { return false; }
    if v.def != 0 && v.def != 1 { return false; }
    if v.spd != 12 && v.spd != 13 { return false; }
    println!("Chosen DVs: {:?}", v); true
  }).into_unit())));
  r.save("yellow_after_collect_pikachu_256_div");

  r.load("yellow_after_collect_pikachu_256_div");
}