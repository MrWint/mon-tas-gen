use crate::run::*;
#[allow(unused_imports)] use gambatte::inputs::*;
#[allow(unused_imports)] use montas::constants::*;
#[allow(unused_imports)] use montas::segment::battle::*;
#[allow(unused_imports)] use montas::segment::battle::gen2::*;
#[allow(unused_imports)] use montas::segment::overworld::gen2::*;

#[allow(dead_code)]
pub fn start() {
  let mut r: GbRunner<Crystal> = GbRunner::pool_with_screen();

  run(&mut r);

  r.run(IdentifyInputSegment::new());

  r.debug_segment_end("temp/crystal_glitchless");
}

const CHOOSE_TOTODILE: bool = false;

fn run(r: &mut GbRunner<Crystal>) {
  // r.run(MoveSegment::new(A));
  // r.run(MoveSegment::new(START));
  // { // Options: in main menu
  //   r.run(MoveSegment::new(D)); // options
  //   r.run(MoveSegment::new(L|A)); // fast options
  //   r.run(MoveSegment::new(B)); // back
  // }
  // r.run(MoveSegment::new(A)); // new game
  // if false { // choose Boy
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A));
  // } else { // choose Girl
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(D));
  //   r.run(MoveSegment::new(A));
  // }
  // r.run(SkipTextsSegment::new(3));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // time: 10:..
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // what // 10 oclock // ?
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // ? // How many minutes?
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // whoa // 00 min // ?
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // day // overslept
  // r.run(SkipTextsSegment::new(16)); // oak speech
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // oak speech
  // { // Name: J (+50 frames)
  //   r.run(MoveSegment::new(A)); // custom name
  //   r.run(MoveSegment::new(D)); // down to J
  //   r.run(MoveSegment::new(A)); // input J
  //   r.run(MoveSegment::new(START)); // go to end
  //   r.run(MoveSegment::new(A)); // confirm
  //   r.run(VerifyInputSegment::new("NamingScreenJoypadLoop")); // closing takes extra loop though input
  // }
  // // { // Name: Chris
  // //   r.run(MoveSegment::new(D)); // Name: Chris
  // //   r.run(MoveSegment::new(A)); // Name: Chris
  // // }
  // r.run(SkipTextsSegment::new(7)); // skip texts until game start
  // r.debug_print_states();
  // r.run(TextSegment::new()); // ... seeing you later
  // r.debug_print_states();
  // // { // Options: in-game
  // //   r.run(MoveSegment::new(START)); // Open menu
  // //   r.run(MoveSegment::new(U));
  // //   r.run(MoveSegment::new(NIL));
  // //   r.run(MoveSegment::new(U));
  // //   r.run(MoveSegment::new(L|A)); // fast options
  // //   r.run(MoveSegment::new(B));
  // //   r.run(MoveSegment::new(START));
  // // }
  // r.save("crystal_intro_g");

  // r.load("crystal_intro_g");
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(7, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new());
  // r.run(WalkToSegment::new(7, 2));
  // r.run(WalkStepSegment::new(D));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(7)); // mom speech
  // r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // received // PokeGear // . // Pokemon Gear
  // r.run(SkipTextsSegment::new(4)); // mom speech
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose Sunday
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A).with_skip_ends(1)); // Sunday // is it?
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // no DST
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A).with_skip_ends(1)); // 10:00 AM // confirm time
  // r.run(SkipTextsSegment::new(3)); // mom speech
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // know phone
  // r.run(SkipTextsSegment::new(5)); // mom speech
  // r.run(TurnSegment::new(R));
  // r.debug_print_states();
  // r.run(WalkToSegment::new(7, 7));
  // r.run(WarpSegment::new().with_input(D));
  // r.debug_print_states();
  // r.save("crystal_left_house_g");

  // r.load("crystal_left_house_g");
  // r.run(WalkToSegment::new(6, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new());
  // r.debug_print_states();
  // r.run(SkipScriptSegment::new());
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(12)); // elm speech
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose to help
  // r.run(SkipTextsSegment::new(6)); // elm speech
  // r.run(SkipScriptSegment::new());
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(15)); // elm speech
  // r.run(SkipScriptSegment::new());
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(5)); // elm speech
  // if CHOOSE_TOTODILE { // totodile
  //   r.run(WalkToSegment::new(7, 4));
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::NoEvents)));
  //   r.debug_print_states();
  //   r.run(TurnSegment::new(U));
  //   r.debug_print_states();
  //   r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  //   r.debug_print_states();
  //   r.run(MoveSegment::new(B)); // close picture
  //   r.run(SkipTextsSegment::new(1)); // choose Totodile
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose Totodile
  // } else {
  //   r.run(TurnSegment::new(U));
  //   r.run(WalkToSegment::new(4, 3));
  //   r.run(WalkToSegment::new(5, 3));
  //   r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  //   r.debug_print_states();
  //   r.run(MoveSegment::new(B)); // close picture
  //   r.run(SkipTextsSegment::new(1)); // choose Cyndaquil
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose Cyndaquil
  // }
  // r.save("crystal_choose_starter_g_c");

  // r.load("crystal_choose_starter_g_c");
  // r.run(SkipTextsSegment::new(2).with_buffer_size(256)); // elm speech
  // r.debug_print_states();
  // r.run(TextSegment::new().with_skip_ends(2).with_buffer_size(8192/4)); // Player received // mon // !
  // r.debug_print_states();
  // r.save("crystal_choose_starter_unbounded_g_c");

  // r.load("crystal_choose_starter_unbounded_g_c");
  // r.run_debug(DelaySegment::new(MoveSegment::with_metric(A | B, Gen2DVMetric {}.filter(|v| {
  //   if v.atk < 14 || v.def < 11 || v.spc < 12 || v.spd < 7 { return false; } // totodile
  // //   if v.atk < 15 || (v.def & 1) == 0 || v.spc < 15 || v.spd < 14 { return false; }
  //   log::debug!("Chosen DVs: {:?}", v); true
  // }).into_unit())));
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // nickname to // Totodile // you
  // r.debug_print_states();
  // // { // no nickname
  // //   r.run(SkipTextsSegment::new(1).with_confirm_input(B));
  // // }
  // { // Name: A
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A));
  //   r.run(MoveSegment::new(NIL)); // A
  //   r.run(MoveSegment::new(A)); // input A
  //   r.run(MoveSegment::new(START)); // go to end
  //   r.run(MoveSegment::new(A)); // confirm
  //   r.run(VerifyInputSegment::new("NamingScreenJoypadLoop")); // closing takes extra loop though input
  // }
  // r.debug_print_states();
  // r.run(SkipScriptSegment::new());
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(11)); // elm speech
  // r.run(TurnSegment::new(D));
  // r.debug_print_states();
  // r.run(WalkToSegment::new(4, 7));
  // r.run(WalkStepSegment::new(D).into(OverworldInteractionResult::MapCoordEvent));
  // r.debug_print_states();
  // r.run(SkipScriptSegment::new());
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(2)); // aide speech
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player received // potion // .
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // potion // in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // item pocket // .
  // r.run(SkipTextsSegment::new(2)); // aide speech
  // r.run(SkipScriptSegment::new());
  // r.debug_print_states();
  // r.run(WalkToSegment::new(4, 11));
  // r.run(WarpSegment::new().with_input(D));
  // r.debug_print_states();
  // r.save("crystal_test_after_elm_g_c");

  // r.load("crystal_test_after_elm_g_c");
  // r.run(WalkToSegment::new(-1, 8).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::new(NIL)); // MapConnection
  // r.debug_print_states();
  // r.run(WalkToSegment::new(9, 6));
  // r.run(JumpLedgeSegment::new(L));
  // r.debug_print_states();
  // r.run(WalkToSegment::new(-1, 7).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::new(NIL)); // MapConnection
  // r.debug_print_states();
  // r.run(WalkToSegment::new(17, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::new(NIL)); // MapConnection
  // r.debug_print_states();
  // r.run(WalkToSegment::new(17, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new());
  // r.debug_print_states();
  // r.save("crystal_test_entered_mr_pokemon_house_g_c");

  // r.load("crystal_test_entered_mr_pokemon_house_g_c");
  // r.run(SkipScriptSegment::new());
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(2)); // Mr.Pokemon speech
  // r.run(SkipScriptSegment::new());
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(2)); // Mr.Pokemon speech
  // r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // // // put // mystery egg // in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // key poket // .
  // r.run(SkipTextsSegment::new(10)); // Mr.Pokemon speech
  // r.run(SkipScriptSegment::new());
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(23)); // Oak speech
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // got pokedex // speech
  // r.run(SkipTextsSegment::new(5)); // Oak speech
  // r.run(SkipScriptSegment::new());
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(4)); // Mr.Pokemon speech
  // r.run(TurnSegment::new(D));
  // r.debug_print_states();
  // r.run(WalkStepSegment::new(D));
  // r.debug_print_states();
  // r.run(WarpSegment::new().with_input(D));
  // r.debug_print_states();
  // r.save("crystal_test_after_mr_pokemon_house_g_c");

  // r.load("crystal_test_after_mr_pokemon_house_g_c");
  // r.run(MoveLoopSegment::new(OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v != &OverworldInteractionResult::CountStepEvent}).into_unit()));
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(4)); // Elm phone call
  // r.run(TextSegment::new().with_skip_ends(6)); // Click // ... // ... // ...
  // r.run(WalkToSegment::new(7, 54).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::new(NIL)); // MapConnection
  // r.debug_print_states();
  // r.run(WalkToSegment::new(33, 7).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new());
  // r.debug_print_states();
  // r.save("crystal_test_before_rival1_g_c");

  // r.load("crystal_test_before_rival1_g_c");
  // r.run(SkipTextsSegment::new(7)); // pre-battle texts
  // r.run(SkipTextsSegment::new(2)); // trainer wants to battle // trainer sent out
  // if CHOOSE_TOTODILE { // Totodile
  //   r.run(TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5)); // chikorita // ! // Go // Totodile // !
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:#?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:#?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(FightTurnResult::CriticalHit { damage: 8, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // but it failed!
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle)))); // confirm
  //   // Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(FightTurnResult::CriticalHit { damage: 8, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   // Turn 3
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  // } else {
  //   r.run(TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Leer)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5)); // Totodile // ! // Go // Cyndaquil // !
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:#?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:#?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(4, 6).debug_print().expect(FightTurnResult::CriticalHit { damage: 6, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Tackle // ! // mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // but it failed!
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Leer)))); // confirm
  //   // Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(4, 6).debug_print().expect(FightTurnResult::CriticalHit { damage: 6, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Tackle // ! // mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // but it failed!
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Leer)))); // confirm
  //   // Turn 3
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(4, 6).debug_print().expect(FightTurnResult::CriticalHit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Tackle // ! // mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   // Turn 4
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(4, 6).debug_print().expect(FightTurnResult::Hit { damage: 4, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Tackle // ! // mon // used // move // !
  // }
  // r.save("crystal_test_rival1_defeated_g_c");

  // r.load("crystal_test_rival1_defeated_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  // if !CHOOSE_TOTODILE { // learned smokescreen
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // mon // learned // move // !
  // }
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(1)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(5)); // ... ... ... // name is ??? // world's greatest // mon // trainer
  // r.run(SkipScriptSegment::new());
  // r.run(TurnSegment::new(U));
  // r.run_debug(WalkToSegment::new(40, 7).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::MapConnection))); // MapConnection
  // r.debug_print_states();
  // r.run_debug(WalkToSegment::new(14, 12));
  // r.run(JumpLedgeSegment::new(D));
  // r.debug_print_states();
  // r.run_debug(WalkToSegment::new(42, 9));
  // r.run(JumpLedgeSegment::new(R));
  // r.debug_print_states();
  // r.run_debug(WalkToSegment::new(60, 9).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::MapConnection))); // MapConnection
  // r.debug_print_states();
  // r.run_debug(WalkToSegment::new(6, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new());
  // r.debug_print_states();
  // r.run_debug(WalkToSegment::new(4, 5).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(7)); // mon stolen
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // mon stolen
  // r.run(MoveSegment::new(R)); // B
  // r.run(MoveSegment::new(A)); // B
  // r.run(MoveSegment::new(START)); // confirm
  // r.run(MoveSegment::new(A)); // confirm
  // r.run(MoveSegment::new(B)); // ?
  // r.run(SkipTextsSegment::new(2)); // so B was his name // thanks
  // r.run(SkipScriptSegment::new());
  // r.run(WalkStepSegment::new(R));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v == &OverworldInteractionResult::NoEvents}).into_unit()));
  // r.debug_print_states();
  // r.run(TurnSegment::new(U));
  // r.debug_print_states();
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.debug_print_states();
  // r.run(SkipTextsSegment::new(5)); // terrible // discovery // give egg
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(1)); // this?
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(21)); // egg // great discovery // what // incredible // potential
  // r.run(TurnSegment::new(D));
  // r.debug_print_states();
  // r.run_debug(WalkToSegment::new(4, 7));
  // r.run_debug(WalkToSegment::new(4, 8).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(2)); // take balls
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // received // balls // !
  // r.run(SkipTextsSegment::new(4)); // take balls
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // ball // in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // ball pocket // .
  // r.run(SkipScriptSegment::new());
  // r.run(WalkToSegment::new(4, 11));
  // r.run(WarpSegment::new().with_input(D));
  // r.debug_print_states();
  // r.save("crystal_test_after_elm2_g_c");

  // r.load("crystal_test_after_elm2_g_c");
  // r.run(WalkToSegment::new(-1, 8).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::MapConnection))); // Enter Route 29
  // r.run(WalkToSegment::new(53, 9).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Catch tutorial cutscene
  // r.run(SkipTextsSegment::new(3)); // Catch tutorial
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Decline
  // r.run(SkipTextsSegment::new(3)); // Catch tutorial
  // r.run(SkipScriptSegment::new()); // Catch tutorial cutscene end
  // r.run(WalkToSegment::new(9, 6));
  // r.run(JumpLedgeSegment::new(L));
  // r.run(WalkToSegment::new(-1, 7).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::MapConnection))); // Enter Cherrygrove
  // r.run(WalkToSegment::new(17, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::MapConnection))); // Enter Route 30
  // // { // test catch pidgey
  // //   r.run(WalkToSegment::new(12, 40));
  // //   r.save("crystal_test_tmp");
  // //   r.load("crystal_test_tmp");
  // //   r.run(WalkToSegment::new(12, 39).into(OverworldInteractionResult::RandomEncounter { species: Pokemon::Pidgey, level: 3 }));
  // //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::RandomEncounter { species: Pokemon::Pidgey, level: 3 })));
  // //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // Wild // Pidgey // appeared!
  // //   r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(B)); // Go // mon // !
  // //   r.run(MoveSegment::new(D)); // pack
  // //   r.run(MoveSegment::new(A)); // select pack
  // //   r.run(VerifyInputSegment::new("BattlePack")); // InitGFX
  // //   r.run(VerifyInputSegment::new("BattlePack")); // InitItemsPocket
  // //   r.run(VerifyInputSegment::new("BattlePack")); // ItemsPocketMenu
  // //   r.run(MoveSegment::new(R)); // go to balls
  // //   r.run(VerifyInputSegment::new("BattlePack")); // InitBallsPocket
  // //   r.run(VerifyInputSegment::new("BattlePack")); // BallsPocketMenu
  // //   r.run(MoveSegment::new(A)); // select pokeball
  // //   r.run(MoveSegment::new(NIL));
  // //   r.run(
  // //     DelaySegment::new(
  // //       MoveSegment::new(A)
  // //       .seq(TextSegment::with_metric(CatchSuccessMetric {}).with_skip_ends(2))
  // //       )); // Throw pokeball
  // //   r.save("crystal_test_tmp2");
  // //   r.load("crystal_test_tmp2");
  // //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // Gotcha! // mon // caught!
  // //   r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon's // date added
  // //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // to the pokedex
  // //   r.run(MoveSegment::new(A)); // confirm pokedex
  // //   r.run(MoveSegment::new(B)); // confirm pokedex
  // //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Give a nickname to // mon // ?
  // //   r.run(VerifyInputSegment::new("BattlePack")); // Pack_QuitRunScript
  // //   r.run(VerifyInputSegment::new("BattlePack")); // exit
  // //   // r.run_debug(TextSegment::new().with_skip_ends(0));
  // //   // r.run(MoveSegment::with_metric(NIL, FnMetric::new(|gb| { println!("wJumptableIndex: {}, wPackJumptableIndex: {}", gb.gb.read_memory(0xCF63), gb.gb.read_memory(0xCF64)); Some(()) } ))); // select pack
  // //   // r.run(VerifyInputSegment::new("BattlePack")); // InitItemsPocket
  // //   // r.run(MoveSegment::new(L)); // go to balls
  // // }
  // r.run(WalkToSegment::new(5, 24).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Mikey
  // r.run(SkipTextsSegment::new(2)); // Mikey
  // r.save("crystal_test_before_mikey_g_c"); // 18m36.570s, 17m42.642s, 17m50.992s, 17m58.664s

  // r.load("crystal_test_before_mikey_g_c");
  // r.run(SkipTextsSegment::new(2)); // trainer wants to battle // trainer sent out
  // r.run(TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5)); // Pidgey // ! // Go // mon // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  //   // Pidgey
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(9, 16).debug_print().filter(|r| if let FightTurnResult::CriticalHit { damage, } = r { damage >= &13 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit!
  // } else {
  //   // Pidgey
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(8, 14).debug_print().filter(|r| if let FightTurnResult::CriticalHit { damage, } = r { damage >= &13 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit!
  // }
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1)); // sent out
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)); // Rattata
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  //   // Rattata Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::CriticalHit { damage: 11, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   // Rattata Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::Hit { damage: 6, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  // } else {
  //   // Rattata Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(6, 10).debug_print().expect(FightTurnResult::CriticalHit { damage: 9, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   // Rattata Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(6, 10).debug_print().expect(FightTurnResult::CriticalHit { damage: 8, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit!
  // }
  // r.save("crystal_test_mikey_defeated_g_c");

  // r.load("crystal_test_mikey_defeated_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  // if CHOOSE_TOTODILE { // Rage
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // mon // learned // move // !
  // }
  // r.run(SkipTextsSegment::new(1)); // trainer was defeated
  // r.run(SkipTextsSegment::new(1)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new()); // Mikey SeenByTrainer end
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(6, 0));
  // r.save("crystal_test_tmp");

  // r.load("crystal_test_tmp");
  // // { // voluntarily call mom // +65f
  // //   r.run(MoveSegment::new(START)); // open start menu
  // //   r.run(MoveSegment::new(D));
  // //   r.run(MoveSegment::new(NIL));
  // //   r.run(MoveSegment::new(D));
  // //   r.run(MoveSegment::new(NIL));
  // //   r.run(MoveSegment::new(D));
  // //   r.run(MoveSegment::new(A)); // PokeGear
  // //   r.run(VerifyInputSegment::new("PokeGear")); // PokegearClock_Init
  // //   r.run(MoveSegment::new(R));
  // //   r.run(VerifyInputSegment::new("PokeGear")); // PokegearPhone_Init
  // //   r.run(MoveSegment::new(A));
  // //   r.run(MoveSegment::new(NIL));
  // //   r.run(MoveSegment::new(A)); // confirm call
  // //   r.run(VerifyInputSegment::new("PokeGear")); // PokegearPhone_MakePhoneCall
  // //   r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // ... // hello
  // //   r.run(SkipTextsSegment::new(3)); // heard from prof // long trip // told me
  // //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // don't save money
  // //   r.run(SkipTextsSegment::new(2)); // wont save money // keep rooting
  // //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // for you
  // //   r.run(MoveSegment::new(B));
  // //   r.run(VerifyInputSegment::new("PokeGear")); // Pokegear done
  // //   r.run(MoveSegment::new(START)); // close start menu
  // // }
  // r.run(WalkToSegment::new(6, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::MapConnection))); // Enter Route 31
  // { // forcibly talk to mom
  //   r.run(SkipScriptSegment::new()); // Mom call
  //   r.run(SkipTextsSegment::new(1)); // hello?
  //   r.run(SkipTextsSegment::new(3)); // heard from prof // long trip // told me
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // don't save money
  //   r.run(SkipTextsSegment::new(3)); // wont save money // keep rooting // for you
  //   r.run(TextSegment::new().with_skip_ends(6)); // click // ...
  // }
  // r.run(WalkToSegment::new(4, 7));
  // r.run(WarpSegment::new().with_input(L)); // enter gate house
  // r.run(WalkToSegment::new(0, 5));
  // r.run(WarpSegment::new().with_input(L)); // enter Violet
  // r.run(WalkToSegment::new(18, 17).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Gym
  // r.run(WalkToSegment::new(4, 10).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Abe
  // r.run(SkipTextsSegment::new(2)); // Abe
  // r.save("crystal_test_before_abe_g_c");

  // r.load("crystal_test_before_abe_g_c");
  // r.run(SkipTextsSegment::new(1)); // trainer wants to battle
  // r.run(TextSegment::new()); // trainer sent out
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::new(A|B).seq(
  //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().filter(|m| m == &Move::Peck).into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
  //   )
  // ); // Spearow // ! // Go // Totodile // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  //   // Spearow Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   // r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(MoveSegment::new(D)); // Leer
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer())
  //       // .seq(TextSegment::with_metric(
  //       //     Gen2NormalHitMetric::with_expected_max_damage(6, 10).debug_print().expect(FightTurnResult::CriticalHit { damage: 9, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // critical hit!
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // enemy // mon // defense fell
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   // Spearow Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   // r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(MoveSegment::new(U)); // Scratch
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer())
  //       // .seq(TextSegment::with_metric(
  //       //     Gen2NormalHitMetric::with_expected_max_damage(6, 10).debug_print().expect(FightTurnResult::CriticalHit { damage: 9, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(9, 16).debug_print().expect(FightTurnResult::CriticalHit { damage: 14, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // critical hit!
  //   // Spearow Turn 3
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer())
  //       // .seq(TextSegment::with_metric(
  //       //     Gen2NormalHitMetric::with_expected_max_damage(6, 10).debug_print().expect(FightTurnResult::CriticalHit { damage: 9, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(9, 16).debug_print().expect(FightTurnResult::CriticalHit { damage: 14, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // critical hit!
  // } else {
  //   // Spearow Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // SmokeScreen
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(10, 18).debug_print().expect(FightTurnResult::Hit { damage: 8, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())
  //       ));
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // enemy // mon // accuracy fell
  //   // // Spearow Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Leer
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(10, 18).debug_print().expect(FightTurnResult::HitMissed)).with_skip_ends(3).with_unbounded_buffer())
  //       ));
  //   r.run(TextSegment::new()); // but it missed!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // enemy // mon // defense fell
  //   // // Spearow Turn 3
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Tackle
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(10, 18).debug_print().expect(FightTurnResult::HitMissed)).with_skip_ends(3).with_unbounded_buffer())
  //       ));
  //   r.run(TextSegment::new()); // but it missed!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::CriticalHit { damage: 11, })).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // critical hit!
  //   // // Spearow Turn 4
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Tackle
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(10, 18).debug_print().expect(FightTurnResult::HitMissed)).with_skip_ends(3).with_unbounded_buffer())
  //       ));
  //   r.run(TextSegment::new()); // but it missed!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::CriticalHit { damage: 11, })).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // critical hit!
  //   // // Spearow Turn 5
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Tackle
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(10, 18).debug_print().expect(FightTurnResult::HitMissed)).with_skip_ends(3).with_unbounded_buffer())
  //       ));
  //   r.run(TextSegment::new()); // but it missed!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::Hit { damage: 6, })).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  // }
  // r.save("crystal_test_after_abe_g_c");

  // r.load("crystal_test_after_abe_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(1)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new()); // Abe SeenByTrainer end
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(4, 6).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rod
  // r.run(SkipTextsSegment::new(4)); // Rod
  // r.save("crystal_test_before_rod_g_c");

  // r.load("crystal_test_before_rod_g_c");
  // r.run(SkipTextsSegment::new(1)); // trainer wants to battle
  // r.run(TextSegment::new()); // trainer sent out
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::new(A|B).seq(
  //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
  //   )
  // ); // Pidgey // ! // Go // Totodile // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  //   // Pidgey Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(8, 14).debug_print().expect(FightTurnResult::CriticalHit { damage: 12, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   r.run(SkipTextsSegment::new(1)); // A recovered using a
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // Berry
  //   // Pidgey Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(8, 14).debug_print().expect(FightTurnResult::CriticalHit { damage: 12, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // critical hit!
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  //   r.run(SkipTextsSegment::new(1)); // sent out
  //   r.run(TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)); // Pidgey
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Pidgey Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(8, 14).debug_print().expect(FightTurnResult::CriticalHit { damage: 12, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   // Pidgey Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(8, 14).debug_print().expect(FightTurnResult::CriticalHit { damage: 12, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // critical hit!
  // } else {
  //   // Pidgey Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::CriticalHit { damage: 12, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   // Pidgey Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::CriticalHit { damage: 11, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // critical hit!
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  //   r.run(SkipTextsSegment::new(1)); // sent out
  //   r.run(TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)); // Pidgey
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Pidgey Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::CriticalHit { damage: 12, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   // Pidgey Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::CriticalHit { damage: 11, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // critical hit!
  // }
  // r.save("crystal_test_after_rod_g_c");

  // r.load("crystal_test_after_rod_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(1)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new()); // Rod SeenByTrainer end
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(5, 3));
  // r.run(WalkToSegment::new(5, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript)))); // Talk to Falkner
  // r.run(SkipTextsSegment::new(9)); // Falkner
  // r.save("crystal_test_before_falkner_g_c");

  // r.load("crystal_test_before_falkner_g_c");
  // r.run(SkipTextsSegment::new(1)); // trainer wants to battle
  // r.run(TextSegment::new()); // trainer sent out
  // if CHOOSE_TOTODILE {
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
  //     )
  //   ); // Pidgey // ! // Go // Totodile // !
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Pidgey Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(FightTurnResult::Hit { damage: 4, }).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey))).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(4, 6).debug_print().expect(FightTurnResult::Hit { damage: 3, })).with_skip_ends(3).with_unbounded_buffer()) // mon // used // move // !
  //       )); // Rage //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // rage is building
  //   // Pidgey Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(10, 16).debug_print().expect(FightTurnResult::Hit { damage: 8, }).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey))).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(4, 6).debug_print().expect(FightTurnResult::Hit { damage: 3, })).with_skip_ends(3).with_unbounded_buffer()) // mon // used // move // !
  //       )); // Rage //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // rage is building
  //   // Pidgey Turn 3
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(15, 24).debug_print().expect(FightTurnResult::Hit { damage: 12, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Rage //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Gust)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1))
  //     )
  //   ); // Pidgeotto
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Pidgeotto Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer())
  //     )); //  mon // used // move // !
  //   r.run(TextSegment::new().with_unbounded_buffer()); // rage is building
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A|B, BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)).with_unbounded_buffer()
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(20, 32).debug_print().expect(FightTurnResult::CriticalHit { damage: 32, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Rage //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit!
  // } else {
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::new(A|B).seq(
    //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::MudSlap)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
    //   )
    // ); // Pidgey // ! // Go // Totodile // !
    // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
    // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
    // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
    // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
    // // Pidgey Turn 1
    // r.run(MoveSegment::new(A)); // Fight
    // r.run(MoveSegment::new(NIL)); // neutral
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(
    //         Gen2NormalHitMetric::with_expected_max_damage(6, 10).debug_print().expect(FightTurnResult::CriticalHit { damage: 9, })).with_skip_ends(3).with_unbounded_buffer())
    //     )); // Tackle //// mon // used // move // !
    // r.run(TextSegment::new()); // critical hit!
    // r.run(
    //   DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
    //   .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(6, 8).with_effect().debug_print().expect(FightTurnResult::HitWithoutEffect { damage: 5, })).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
    // );
    // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // very effective
    // // Pidgey Turn 2
    // r.run(MoveSegment::new(A)); // Fight
    // r.run(MoveSegment::new(NIL)); // neutral
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(
    //         Gen2NormalHitMetric::with_expected_max_damage(6, 10).debug_print().expect(FightTurnResult::CriticalHit { damage: 9, })).with_skip_ends(3).with_unbounded_buffer())
    //     )); // Tackle //// mon // used // move // !
    // r.run(TextSegment::new()); // critical hit!
    // r.run(
    //   DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
    //   .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(6, 8).with_effect().debug_print().expect(FightTurnResult::HitWithoutEffect { damage: 5, })).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
    // );
    // r.run(SkipTextsSegment::new(1)); // very effective
    // r.run(SkipTextsSegment::new(1)); // A recovered using a
    // r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // Berry
    // // Pidgey Turn 3
    // r.run(MoveSegment::new(A)); // Fight
    // r.run(MoveSegment::new(NIL)); // neutral
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(
    //         Gen2NormalHitMetric::with_expected_max_damage(6, 10).debug_print().expect(FightTurnResult::Hit { damage: 6, })).with_skip_ends(3).with_unbounded_buffer())
    //     )); // Tackle //// mon // used // move // !
    // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
    // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
    // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
    // r.run(TextSegment::new()); // sent out
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::new(A|B).seq(
    //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Gust)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1))
    //   )
    // ); // Pidgeotto
    // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
    // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
    // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
    // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
    // // Pidgeotto turn 1
    // r.run(MoveSegment::new(A)); // Fight
    // r.run(MoveSegment::new(D)); // Leer
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())
    //     ));
    // r.run(TextSegment::new().with_skip_ends(2)); // enemy // mon // defense fell
    // r.run(
    //   DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
    //   .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(9, 15).debug_print().expect(FightTurnResult::Hit { damage: 7 })).with_skip_ends(3).with_allowed_end_inputs(B).with_unbounded_buffer())) // mon // used // move // !
    // );
    // // Pidgeotto turn 2
    // r.run(MoveSegment::new(A)); // Fight
    // r.run(MoveSegment::new(U)); // Tackle
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(9, 16).debug_print().expect(FightTurnResult::CriticalHit { damage: 16 })).with_skip_ends(3).with_unbounded_buffer())
    //     ));
    // r.run(TextSegment::new()); // critical hit!
    // r.run(
    //   DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
    //   .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(9, 15).debug_print().expect(FightTurnResult::Hit { damage: 7 })).with_skip_ends(3).with_allowed_end_inputs(B).with_unbounded_buffer())) // mon // used // move // !
    // );
    // // Pidgeotto turn 3
    // r.run(MoveSegment::new(A)); // Fight
    // r.run(MoveSegment::new(NIL)); // Tackle
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(9, 16).debug_print().expect(FightTurnResult::CriticalHit { damage: 16 })).with_skip_ends(3).with_unbounded_buffer())
    //     ));
    // r.run(SkipTextsSegment::new(1)); // critical hit!
  // }
  // r.save("crystal_test_after_falkner_g_c");

  // r.load("crystal_test_after_falkner_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(5)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new()); // Falkner MapScript
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(4)); // Falkner text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(9)); // Falkner text
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(5, 15));
  // r.run(WarpSegment::new().with_input(D)); // 47183
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(5)); // Elm phone call
  // r.run(TextSegment::new().with_skip_ends(6)); // Click
  // r.run(WalkToSegment::new(31, 25).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Center
  // r.run(WalkToSegment::new(4, 5));
  // r.run(WalkToSegment::new(4, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript)))); // Talk to Aide
  // r.run(SkipTextsSegment::new(3)); // Aide
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Accept egg
  // r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // Take egg
  // r.run(SkipTextsSegment::new(7)); // Aide
  // r.run(SkipScriptSegment::new());
  // r.run(WalkToSegment::new(4, 7));
  // r.run(WarpSegment::new().with_input(D)); // Leave Center
  // r.run(WalkToSegment::new(14, 36).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::MapConnection))); // Enter Route 32
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(WalkToSegment::new(7, 71).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(5)); // Slowpoketail
  // r.run(WalkToSegment::new(6, 79).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Cave
  // r.run(WalkToSegment::new(17, 30));
  // r.run(WalkToSegment::new(17, 31));
  // r.run(WarpSegment::new().with_input(D)); // Leave Cave
  // r.run(WalkToSegment::new(9, 10));
  // r.run(JumpLedgeSegment::new(L));
  // r.run(WalkToSegment::new(-1, 14).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::MapConnection))); // Enter Azalea
  // r.run(WalkToSegment::new(9, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Kurt's house
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(WalkToSegment::new(3, 3));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript)))); // Talk to Kurt
  // r.run(SkipTextsSegment::new(16)); // Kurt
  // r.run(SkipScriptSegment::new());
  // r.run(WalkToSegment::new(3, 7));
  // r.run(WarpSegment::new().with_input(D)); // Leave Kurt's house
  // r.run(WalkToSegment::new(31, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Slowpoke Well
  // r.run(WalkToSegment::new(15, 10).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.save("crystal_test_before_well_rocket1_g_c");

  // r.load("crystal_test_before_well_rocket1_g_c");
  // r.run(SkipTextsSegment::new(6)); // Rocket
  // r.run(SkipTextsSegment::new(1)); // trainer wants to battle
  // r.run(TextSegment::new()); // trainer sent out
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::new(A|B).seq(
  //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
  //   )
  // ); // Rattata // ! // Go // mon // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  // } else {
  //   // Rattata Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(9, 16).debug_print().expect(FightTurnResult::CriticalHit { damage: 16, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // tackle //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   // Rattata Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(9, 16).debug_print().expect(FightTurnResult::Hit { damage: 9, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Tackle //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Rattata
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Rattata Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(9, 16).debug_print().expect(FightTurnResult::CriticalHit { damage: 16, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // tackle //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   // Rattata Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(9, 16).debug_print().expect(FightTurnResult::Hit { damage: 9, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Tackle //// mon // used // move // !
  // }
  // r.save("crystal_test_after_well_rocket1_g_c");

  // r.load("crystal_test_after_well_rocket1_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  // if !CHOOSE_TOTODILE { // learned Ember
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // mon // learned // move // !
  // }
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(1)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new()); // Rocket MapScript
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(14, 4).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.save("crystal_test_before_well_rocket2_g_c");

  // r.load("crystal_test_before_well_rocket2_g_c");
  // r.run(SkipTextsSegment::new(2)); // Rocket
  // r.run(SkipTextsSegment::new(1)); // trainer wants to battle
  // r.run(TextSegment::new()); // trainer sent out
  // r.run_debug(
  //   DelaySegment::new(
  //     MoveSegment::new(A|B).seq(
  //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
  //   )
  // ); // Zubat // ! // Go // mon // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  // } else {
  //   // Zubat
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Ember
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(15, 27).debug_print().expect(FightTurnResult::CriticalHit { damage: 27, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit!
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Wrap)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Ekans
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Ekans Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(12, 21).with_effect().debug_print().expect(FightTurnResult::CriticalHitWithoutEffect { damage: 20, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   // Ekans Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(12, 21).debug_print().expect(FightTurnResult::Hit { damage: 10, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  // }
  // r.save("crystal_test_after_well_rocket2_g_c");

  // r.load("crystal_test_after_well_rocket2_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  // if CHOOSE_TOTODILE { // learned Water Gun
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // mon // learned // move // !
  // }
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(1)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new()); // Rocket MapScript
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(7, 6).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.save("crystal_test_before_well_rocket3_g_c");

  // r.load("crystal_test_before_well_rocket3_g_c");
  // r.run(SkipTextsSegment::new(3)); // Rocket
  // r.run(SkipTextsSegment::new(1)); // trainer wants to battle
  // r.run(TextSegment::new()); // trainer sent out
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::new(A|B).seq(
  //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().filter(|v| v != &Move::QuickAttack).into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
  //   )
  // ); // Rattata // ! // Go // mon // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  // } else {
  //   // Rattata
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Ember
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(21, 39).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &22 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit!
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Zubat
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Zubat
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Ember
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(18, 33).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &27 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit!
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Zubat
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Zubat
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Ember
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(18, 33).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &27 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit!
  // }
  // r.save("crystal_test_after_well_rocket3_g_c");

  // r.load("crystal_test_after_well_rocket3_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(1)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new()); // Rocket MapScript
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(5, 4));
  // r.run(WalkToSegment::new(5, 3).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.save("crystal_test_before_well_rocket4_g_c");

  // r.load("crystal_test_before_well_rocket4_g_c");
  // r.run(SkipTextsSegment::new(3)); // Rocket
  // r.run(SkipTextsSegment::new(1)); // trainer wants to battle
  // r.run(TextSegment::new()); // trainer sent out
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::new(A|B).seq(
  //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.expect(Move::PoisonGas)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
  //   )
  // ); // Koffing // ! // Go // mon // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  // } else {
  //   // Koffing Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Ember
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(13, 24).with_effect().debug_print().expect(FightTurnResult::CriticalHitWithoutEffect { damage: 24, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   // Koffing Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(13, 24).debug_print().expect(FightTurnResult::Hit { damage: 13, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  // }
  // r.save("crystal_test_after_well_rocket4_g_c");

  // r.load("crystal_test_after_well_rocket4_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(2)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(TextSegment::new().with_skip_ends(2)); // evolution
  // for _ in 0..14*4 { r.run(VerifyInputSegment::new("EvolutionAnimation")); }
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // evolution
  // r.run(TextSegment::new().with_skip_ends(2)); // evolution
  // r.run(SkipScriptSegment::new()); // Rocket MapScript
  // r.run(SkipTextsSegment::new(6)); // Rocket
  // r.run(SkipScriptSegment::new()); // Kurt enters
  // r.run(SkipTextsSegment::new(4)); // Kurt
  // r.run(SkipScriptSegment::new()); // teleport to Kurt's house
  // r.run(WalkToSegment::new(3, 7));
  // r.run(WarpSegment::new().with_input(D)); // Leave Kurt's house
  // r.run(WalkToSegment::new(10, 15).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // Enter Gym
  // r.run(WalkToSegment::new(4, 11).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Amy&May
  // r.save("crystal_test_before_amy_may_g_c");

  // r.load("crystal_test_before_amy_may_g_c");
  // r.run(SkipTextsSegment::new(2)); // Amy
  // r.run(SkipTextsSegment::new(1)); // trainer wants to battle
  // r.run(TextSegment::new()); // trainer sent out
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::new(A|B).seq(
  //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
  //   )
  // ); // Spinarak // ! // Go // mon // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  // } else {
  //   // Spinarak
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Ember
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(42, 78).debug_print().filter(|v| if let FightTurnResult::Hit { damage } = v { damage >= &28 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // very effective
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Ledyba
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Ledyba
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Ember
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(32, 60).debug_print().filter(|v| if let FightTurnResult::Hit { damage } = v { damage >= &28 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // very effective
  // }
  // r.save("crystal_test_after_amy_may_g_c");

  // r.load("crystal_test_after_amy_may_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(1)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new()); // Amy&May MapScript
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(0, 5).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Josh
  // r.save("crystal_test_before_josh_g_c");

  // r.load("crystal_test_before_josh_g_c");
  // r.run(SkipTextsSegment::new(4)); // Josh
  // r.run(SkipTextsSegment::new(1)); // trainer wants to battle
  // r.run(TextSegment::new()); // trainer sent out
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::new(A|B).seq(
  //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
  //   )
  // ); // Paras // ! // Go // mon // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  // } else {
  //   // Paras
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Ember
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(72, 132).debug_print().filter(|v| if let FightTurnResult::Hit { damage } = v { damage >= &34 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // very effective
  // }
  // r.save("crystal_test_after_josh_g_c");

  // r.load("crystal_test_after_josh_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(1)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new()); // Josh MapScript
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(4, 7));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript)))); // Talk to Bugsy
  // r.run(SkipTextsSegment::new(6)); // Bugsy
  // r.save("crystal_test_before_bugsy_g_c");

  // r.load("crystal_test_before_bugsy_g_c");
  // r.run(SkipTextsSegment::new(1)); // trainer wants to battle
  // r.run(TextSegment::new()); // trainer sent out
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::new(A|B).seq(
  //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
  //   )
  // ); // Metapod // ! // Go // mon // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  // } else {
    // // Metapod
    // r.run(MoveSegment::new(A)); // Fight
    // r.run(MoveSegment::new(U)); // Ember
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(
    //         Gen2NormalHitMetric::with_expected_max_damage(50, 96).debug_print().filter(|v| if let FightTurnResult::Hit { damage } = v { damage >= &40 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
    //     )); // Ember //// mon // used // move // !
    // r.run(SkipTextsSegment::new(1)); // very effective
    // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
    // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
    // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
    // r.run(TextSegment::new()); // sent out
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::new(A|B).seq(
    //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Kakuna
    //   )
    // );
    // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
    // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
    // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
    // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
    // // Kakuna
    // r.run(MoveSegment::new(A)); // Fight
    // r.run(MoveSegment::new(NIL)); // Ember
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(
    //         Gen2NormalHitMetric::with_expected_max_damage(54, 102).debug_print().filter(|v| if let FightTurnResult::Hit { damage } = v { damage >= &38 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
    //     )); // Ember //// mon // used // move // !
    // r.run(SkipTextsSegment::new(1)); // very effective
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::FuryCutter)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Scyther
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Scyther Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Ember
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(TextSegment::new()); // but it failed!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(24, 42).with_effect().debug_print().expect(FightTurnResult::HitWithoutEffect { damage: 23, })).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // very effective
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::FuryCutter)))); // confirm
  //   // Scyther Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(TextSegment::new()); // but it failed!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(24, 42).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &27 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  //   r.run(SkipTextsSegment::new(1)); // very effective
  // }
  // r.save("crystal_test_after_bugsy_g_c");

  // r.load("crystal_test_after_bugsy_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(4)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new()); // Bugsy MapScript
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(6)); // Bugsy text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(6)); // Bugsy text
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(4, 15));
  // r.run(WarpSegment::new().with_input(D)); // Leave Gym
  // r.run(WalkToSegment::new(5, 11).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Rival Scene
  // r.save("crystal_test_before_rival2_g_c");

  // r.load("crystal_test_before_rival2_g_c");
  // r.run(SkipTextsSegment::new(7)); // Rival
  // r.run(SkipTextsSegment::new(1)); // trainer wants to battle
  // r.run(TextSegment::new()); // trainer sent out
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::new(A|B).seq(
  //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
  //   )
  // ); // Gastly // ! // Go // mon // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  // } else {
  //   // Gastly
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Ember
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(24+1, 45+3).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &32 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().filter(|v| v == &Move::Leer).into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Croconaw
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
    // // Croconaw turn 1
    // r.run(MoveSegment::new(A)); // Fight
    // r.run(MoveSegment::new(NIL)); // Ember
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(7, 13).with_effect().debug_print().expect(FightTurnResult::CriticalHit { damage: 13, })).with_skip_ends(3).with_unbounded_buffer())
    //     )); // Ember //// mon // used // move // !
    // r.run(SkipTextsSegment::new(1)); // critical hit
    // r.run(SkipTextsSegment::new(1)); // not very effective
    // r.run(TextSegment::new().with_allowed_end_inputs(A)); // burned
    // r.run(
    //   DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
    //   .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
    // );
    // r.run(SkipTextsSegment::new(1)); // but it failed
    // r.run(TextSegment::new().with_allowed_end_inputs(A)); // hurt by burn
    // r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Leer)))); // confirm
    // // Croconaw turn 2
    // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
    // r.run(MoveSegment::new(A)); // Fight
    // r.run(MoveSegment::new(NIL)); // Ember
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(7, 13).debug_print().expect(FightTurnResult::CriticalHit { damage: 13, })).with_skip_ends(3).with_unbounded_buffer())
    //     )); // Ember //// mon // used // move // !
    // r.run(SkipTextsSegment::new(1)); // critical hit
    // r.run(TextSegment::new().with_allowed_end_inputs(A)); // not very effective
    // r.run(
    //   DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
    //   .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
    // );
    // r.run(SkipTextsSegment::new(1)); // but it failed
    // r.run(TextSegment::new().with_allowed_end_inputs(A)); // hurt by burn
    // r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().into_unit()))); // confirm
    // // Croconaw turn 3
    // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
    // r.run(MoveSegment::new(A)); // Fight
    // r.run(MoveSegment::new(NIL)); // Ember
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(7, 13).debug_print().expect(FightTurnResult::CriticalHit { damage: 13, })).with_skip_ends(3).with_unbounded_buffer())
    //     )); // Ember //// mon // used // move // !
    // r.run(SkipTextsSegment::new(1)); // critical hit
    // r.run(SkipTextsSegment::new(1)); // not very effective
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Zubat
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Zubat
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Ember
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(24+1, 45+3).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &39 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  // }
  // r.save("crystal_test_after_rival2_g_c");

  // r.load("crystal_test_after_rival2_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(3)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new()); // Rival MapScript
  // r.run(SkipTextsSegment::new(12)); // rival
  // r.run(SkipScriptSegment::new()); // Rival MapScript
  // r.run(WalkToSegment::new(2, 11));
  // r.run(WarpSegment::new().with_input(L)); // Enter gate
  // r.run(WalkToSegment::new(0, 5));
  // r.run(WarpSegment::new().with_input(L)); // Enter forest
  // r.run(WalkToSegment::new(14, 32));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(2)); // Farfetch'd
  // r.run(SkipScriptSegment::new()); // Farfetch'd MapScript
  // r.run(WalkToSegment::new(15, 24));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(1)); // Farfetch'd
  // r.run(SkipScriptSegment::new()); // Farfetch'd MapScript
  // r.run(WalkToSegment::new(15, 28));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(1)); // Farfetch'd
  // r.run(SkipScriptSegment::new()); // Farfetch'd MapScript
  // r.run(WalkToSegment::new(12, 35));
  // r.run(WalkToSegment::new(11, 35));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(1)); // Farfetch'd
  // r.run(SkipScriptSegment::new()); // Farfetch'd MapScript
  // r.run(WalkToSegment::new(5, 29));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(7)); // farfetch'd owner
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got HM01
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put HM01 in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(4)); // farfetch'd owner
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(8, 26));
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(VerifyInputSegment::new("Pack")); // InitGFX
  //   r.run(VerifyInputSegment::new("Pack")); // InitItemsPocket
  //   r.run(VerifyInputSegment::new("Pack")); // ItemsPocketMenu
  //   r.run(MoveSegment::new(L));
  //   r.run(VerifyInputSegment::new("Pack")); // InitTMHMPocket
  //   r.run(VerifyInputSegment::new("Pack")); // TMHMPocketMenu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A));
  //   r.run(SkipTextsSegment::new(1)); // Booted up HM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // contains // move // .
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // teach // move // to mon?
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A));
  //   { // override move
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // trying to learn
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // move // .
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // but // mon // can't learn more
  //     r.run(SkipTextsSegment::new(1)); // than four moves
  //     r.run(SkipTextsSegment::new(1)); // delete to make room
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // for // move // ?
  //     r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // forget Tackle
  //     r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // 1, 2, poof!
  //     r.run(MoveSegment::new(A)); // confirm text line
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // mon // forgot // move // .
  //     r.run(SkipTextsSegment::new(1)); // and
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // mon // learned // move // !
  //   }
  //   r.run(VerifyInputSegment::new("Pack")); // TMHMPocketMenu
  //   r.run(MoveSegment::new(B));
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  // }
  // { // cut from menu
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // cut
  // }
  // // { // cut from overworld +38f
  // //   r.run(MoveSegment::new(START));
  // //   r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // //   r.run(SkipTextsSegment::new(1)); // tree can be cut
  // //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use cut
  // // }
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used cut !
  // r.run(WalkToSegment::new(1, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gate
  // r.run(WalkToSegment::new(4, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter route 34
  // r.run(WalkToSegment::new(8, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::MapConnection))); // Enter Goldenrod
  // r.run(WalkToSegment::new(29, 29).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter bike shop
  // r.run(WalkToSegment::new(4, 2));
  // r.run(WalkToSegment::new(5, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(3)); // bike shop
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // take bike
  // r.run(SkipTextsSegment::new(3)); // bike shop
  // r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // put bicycle in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the Key pocket
  // r.run(SkipTextsSegment::new(2)); // bike shop
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(3, 6));
  // r.run(WalkToSegment::new(3, 7));
  // r.run(WarpSegment::new().with_input(D)); // leave bike shop
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(VerifyInputSegment::new("Pack")); // InitGFX
  //   r.run(VerifyInputSegment::new("Pack")); // InitTMHMPocket
  //   r.run(VerifyInputSegment::new("Pack")); // TMHMPocketMenu
  //   r.run(MoveSegment::new(L));
  //   r.run(VerifyInputSegment::new("Pack")); // InitKeyItemsPocket
  //   r.run(VerifyInputSegment::new("Pack")); // KeyItemsPocketMenu
  //   r.run(MoveSegment::new(A)); // bike
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // sel
  //   r.run(MoveSegment::new(B)); // registered
  //   r.run(VerifyInputSegment::new("Pack")); // KeyItemsPocketMenu
  //   { // bike from menu
  //     r.run(MoveSegment::new(A)); // bike
  //     r.run(MoveSegment::new(NIL));
  //     r.run(MoveSegment::new(A)); // use
  //     r.run(VerifyInputSegment::new("Pack")); // quit
  //     r.run(VerifyInputSegment::new("Pack")); // quit
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got on the // bicycle // .
  //   }
  //   // { // bike from overworld +24f
  //   //   r.run(MoveSegment::new(B)); // cancel
  //   //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   //   r.run(MoveSegment::new(START));
  //   //   r.run(MoveSegment::new(SELECT));
  //   // }
  // }
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(19, 1).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gate house
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(WalkToSegment::new(4, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter route 35
  // r.run(WalkToSegment::new(3, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gate house
  // r.run(WalkToSegment::new(3, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter park
  // r.run(MoveSegment::new(SELECT));
  // r.run(WalkToSegment::new(33, 19));
  // r.run(WarpSegment::new().with_input(R)); // enter gate house
  // r.run(WalkToSegment::new(9, 5));
  // r.run(WarpSegment::new().with_input(R)); // enter route 36
  // r.run(MoveSegment::new(SELECT));
  // r.run(WalkToSegment::new(33, 14));
  // r.run(WalkToSegment::new(33, 13));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(SkipTextsSegment::new(9)); // floria
  // r.run(SkipScriptSegment::new()); // floria
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(18, 9));
  // r.run(WarpSegment::new().with_input(L)); // enter gate house
  // r.run(WalkToSegment::new(0, 5));
  // r.run(WarpSegment::new().with_input(L)); // enter park
  // r.run(MoveSegment::new(SELECT));
  // r.run(WalkToSegment::new(11, 47));
  // r.run(WarpSegment::new().with_input(D)); // enter gate house
  // r.run(WalkToSegment::new(4, 7));
  // r.run(WarpSegment::new().with_input(D)); // enter route 35
  // r.run(MoveSegment::new(SELECT));
  // r.run(WalkToSegment::new(9, 33));
  // r.run(WarpSegment::new().with_input(D)); // enter gate house
  // r.run(WalkToSegment::new(4, 7));
  // r.run(WarpSegment::new().with_input(D)); // enter Goldenrod
  // r.run(WalkToSegment::new(24, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.run(WalkToSegment::new(13, 13).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Carrie
  // r.save("crystal_test_before_carrie_g_c");

  // r.load("crystal_test_before_carrie_g_c");
  // r.run(SkipTextsSegment::new(2)); // Carrie
  // r.run(SkipTextsSegment::new(1)); // trainer wants to battle
  // r.run(TextSegment::new()); // trainer sent out
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::new(A|B).seq(
  //     TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Charm)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5))
  //   )
  // ); // Snubbull // ! // Go // mon // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // if CHOOSE_TOTODILE {
  // } else {
  //   // Snubbull Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(21+1, 39+3).with_effect().debug_print().expect(FightTurnResult::CriticalHitWithoutEffect { damage: 38, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  //   // Snubbull Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(21+1, 39+3).debug_print().expect(FightTurnResult::Hit { damage: 20, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  // }
  // r.save("crystal_test_after_carrie_g_c");

  // r.load("crystal_test_after_carrie_g_c");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  // r.run(SkipTextsSegment::new(1)); // ??? was defeated
  // r.run(SkipTextsSegment::new(1)); // defeat text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning
  // r.run(SkipScriptSegment::new()); // Carrie MapScript
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(8, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.save("crystal_test_before_whitney_g_c");

  r.load("crystal_test_before_whitney_g_c");


  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");

  // r.run_debug(TextSegment::new().with_skip_ends(0));
  // r.run_debug(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit()));
}