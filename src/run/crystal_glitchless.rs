use crate::run::*;
#[allow(unused_imports)] use gambatte::inputs::*;
#[allow(unused_imports)] use montas::constants::*;
#[allow(unused_imports)] use montas::segment::battle::*;
#[allow(unused_imports)] use montas::segment::battle::gen2::*;
#[allow(unused_imports)] use montas::segment::overworld::gen2::*;

#[allow(dead_code)]
pub fn start() {
  // let inputs = crate::bk2::read_bk2_inputs("temp/crystal_sync_demo_2.bk2").unwrap();
  // let mut r: GbRunner<Crystal> = GbRunner::single_with_screen(&inputs);
  let mut r: GbRunner<Crystal> = GbRunner::pool_with_screen(&[]);

  run(&mut r);

  r.run(IdentifyInputSegment::new());

  r.debug_segment_end("temp/crystal_glitchless_90");
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
  // {
  //   r.run(TextSegment::new());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(A)); // 3 nite
  // }
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // what // 3 oclock // ?
  // {
  //   r.run(TextSegment::new());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(A)); // 3:52
  // }
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // whoa // 52 min // ?
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // day // dark
  // r.run(SkipTextsSegment::new(1)); // outside
  // r.run(SkipTextsSegment::new(16)); // oak speech
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // oak speech
  // { // Name: I (+50 frames)
  //   r.run(MoveSegment::new(A)); // custom name
  //   r.run(MoveSegment::new(L)); // left to I
  //   r.run(MoveSegment::new(A)); // input I
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
  // r.save("crystal_intro_90");

  // r.load("crystal_intro_90");
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(7, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new());
  // r.run(WalkToSegment::new(7, 2));
  // r.run(WalkToSegment::new(7, 3));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(7)); // mom speech
  // r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // received // PokeGear // . // Pokemon Gear
  // r.run(SkipTextsSegment::new(4)); // mom speech
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose Sunday
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A).with_skip_ends(1)); // Sunday // is it?
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // no DST
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A).with_skip_ends(1)); // 3:52 AM // confirm time
  // r.run(SkipTextsSegment::new(3)); // mom speech
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // know phone
  // r.run(SkipTextsSegment::new(5)); // mom speech
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(7, 6));
  // r.run(WalkToSegment::new(7, 7));
  // r.run(WarpSegment::new().with_input(D));
  // r.save("crystal_left_house_90");

  // r.load("crystal_left_house_90");
  // r.run(WalkToSegment::new(6, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new());
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(12)); // elm speech
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose to help
  // r.run(SkipTextsSegment::new(6)); // elm speech
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(15)); // elm speech
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(5)); // elm speech
  // if CHOOSE_TOTODILE { // totodile
  //   r.run(WalkToSegment::new(7, 4));
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::NoEvents)));
  //   r.run(TurnSegment::new(U));
  //   r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  //   r.run(MoveSegment::new(B)); // close picture
  //   r.run(SkipTextsSegment::new(1)); // choose Totodile
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose Totodile
  // } else {
  //   r.run(TurnSegment::new(D));
  //   r.run(WalkToSegment::new(4, 3));
  //   r.run(WalkToSegment::new(5, 3));
  //   r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  //   r.run(MoveSegment::new(B)); // close picture
  //   r.run(SkipTextsSegment::new(1)); // choose Cyndaquil
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose Cyndaquil
  // }
  // r.save("crystal_choose_starter_90");

  // r.load("crystal_choose_starter_90");
  // r.run(SkipTextsSegment::new(2)); // elm speech
  // r.debug_print_states();
  // r.run(TextSegment::new().with_skip_ends(2).with_unbounded_buffer()); // Player received // mon // !
  // r.debug_print_states();
  // r.save("crystal_choose_starter_unbounded_90");

  // r.load("crystal_choose_starter_unbounded_90");
  // r.run(DelaySegment::new(MoveSegment::with_metric(A|B, Gen2DVMetric {}.filter(|v| {
  //   // if v.atk < 15 || v.def < 11 || v.spc < 12 || v.spd < 7 || v.def & 1 == 0 || (v.spd & 1 == 0 && v.spc & 1 == 0) { return false; } // totodile
  //   if v.atk < 15 || v.def < 6 || v.spc < 14 || v.spd < 10 { return false; } // cyndaquil
  //   log::info!("Chosen DVs: {:?}", v); true
  // }).into_unit())));
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // nickname to // Totodile // you
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
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(11)); // elm speech
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(4, 7));
  // r.run(WalkStepSegment::new(D).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(2)); // aide speech
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player received // potion // .
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // potion // in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // item pocket // .
  // r.run(SkipTextsSegment::new(2)); // aide speech
  // r.run(SkipScriptSegment::new());
  // r.run(WalkToSegment::new(4, 11));
  // r.run(WarpSegment::new().with_input(D));
  // r.save("crystal_after_elm_90");

  // r.load("crystal_after_elm_90");
  // r.run(WalkToSegment::new(-1, 8).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 29
  // r.run(WalkToSegment::new(9, 6));
  // r.run(JumpLedgeSegment::new(L));
  // r.run(WalkToSegment::new(-1, 7).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Cherrygrove
  // r.run(WalkToSegment::new(17, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 30
  // r.run(WalkToSegment::new(17, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new());
  // r.save("crystal_entered_mr_pokemon_house_90");

  // r.load("crystal_entered_mr_pokemon_house_90");
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(2)); // Mr.Pokemon speech
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(2)); // Mr.Pokemon speech
  // r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // // // put // mystery egg // in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // key poket // .
  // r.run(SkipTextsSegment::new(10)); // Mr.Pokemon speech
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(23)); // Oak speech
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // got pokedex // speech
  // r.run(SkipTextsSegment::new(5)); // Oak speech
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(4)); // Mr.Pokemon speech
  // r.run(TurnSegment::new(L));
  // r.run(WalkStepSegment::new(D));
  // r.run(WarpSegment::new().with_input(D));
  // r.save("crystal_after_mr_pokemon_house_90");

  // r.load("crystal_after_mr_pokemon_house_90");
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(4)); // Elm phone call
  // r.run(TextSegment::new().with_skip_ends(6)); // Click // ... // ... // ...
  // r.run_debug(WalkToSegment::new(7, 54).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Cherrygrove
  // r.run(WalkToSegment::new(33, 7).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new());
  // r.save("crystal_before_rival1_90");

  // r.load("crystal_before_rival1_90");
  // if CHOOSE_TOTODILE { // Totodile
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(7).with_expected_move(Move::Tackle)); // Rival
  //   r.run(KOSegment::new(Move::Scratch, EnemyAttack { mov: Move::Tackle, attack_type: EnemyAttackType::Failed } ));
  //   r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_level_up());
  // } else {
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(7).with_expected_move(Move::Leer)); // Rival
  //   r.save("crystal_test_tmp");
  //   r.load("crystal_test_tmp");
  //   // Totodile Turn 1
  //   r.run(SelectMoveSegment::new(Move::Leer));
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2MoveSuccessMetric {}.expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())
  //       ));
  //   r.run(TextSegment::new().with_skip_ends(2)); // enemy // mon // defense fell
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(A).with_unbounded_buffer()); // mon // defense fell
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Leer)))); // confirm        
  //   r.save("crystal_test_tmp2");
  //   r.load("crystal_test_tmp2");
  //   r.run(KOSegment::new(Move::Tackle, EnemyAttack { mov: Move::Leer, attack_type: EnemyAttackType::StatUpDown } ));
  //   r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_level_up().with_learn_move()); // learn smokescreen
  // }
  // r.save("crystal_after_rival1_90"); // best: crystal_after_rival1_90

  // r.load("crystal_after_rival1_90"); // 21253/29  //  21240/26
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(5)); // ... ... ... // name is ??? // world's greatest // mon // trainer
  // r.run(SkipScriptSegment::new());
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(40, 7).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // MapConnection
  // r.run(WalkToSegment::new(14, 12));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(42, 9));
  // r.run(JumpLedgeSegment::new(R));
  // r.run(WalkToSegment::new(60, 9).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // MapConnection
  // r.run(WalkToSegment::new(6, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new());
  // r.run(WalkToSegment::new(4, 5).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(7)); // mon stolen
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // mon stolen
  // r.run(MoveSegment::new(D)); // J
  // r.run(MoveSegment::new(A)); // J
  // r.run(MoveSegment::new(START)); // confirm
  // r.run(MoveSegment::new(A)); // confirm
  // r.run(MoveSegment::new(B)); // ?
  // r.run(SkipTextsSegment::new(2)); // so J was his name // thanks
  // r.run(SkipScriptSegment::new());
  // r.run(WalkStepSegment::new(R));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(5)); // terrible // discovery // give egg
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(1)); // this?
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(21)); // egg // great discovery // what // incredible // potential
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(4, 7));
  // r.run(WalkToSegment::new(4, 8).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(2)); // take balls
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // received // balls // !
  // r.run(SkipTextsSegment::new(4)); // take balls
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // ball // in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // ball pocket // .
  // r.run(SkipScriptSegment::new());
  // r.run(WalkToSegment::new(4, 11));
  // r.run(WarpSegment::new().with_input(D));
  // r.save("crystal_after_elm2_90");

  // r.load("crystal_after_elm2_90");
  // r.run(WalkToSegment::new(-1, 8).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Route 29
  // r.run(WalkToSegment::new(53, 9).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Catch tutorial cutscene
  // r.run(SkipTextsSegment::new(3)); // Catch tutorial
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Decline
  // r.run(SkipTextsSegment::new(3)); // Catch tutorial
  // r.run(SkipScriptSegment::new()); // Catch tutorial cutscene end
  // r.run(WalkToSegment::new(9, 6));
  // r.run(JumpLedgeSegment::new(L));
  // r.run(WalkToSegment::new(-1, 7).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Cherrygrove
  // r.run(WalkToSegment::new(17, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Route 30
  // { // catch poliwag
  //   r.run(WalkToSegment::new(12, 40));
  //   r.save("crystal_test_tmp");
  //   r.load("crystal_test_tmp");
  //   r.run(WalkToSegment::new(12, 38));
  //   r.run_debug(WalkToSegment::new(12, 37).into(OverworldInteractionResult::RandomEncounter { species: Pokemon::Poliwag, level: 4 }));
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::RandomEncounter { species: Pokemon::Poliwag, level: 4 })));
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // Wild // Pidgey // appeared!
  //   r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(B)); // Go // mon // !
  //   r.run(MoveSegment::new(D)); // pack
  //   r.run(MoveSegment::new(A)); // select pack
  //   r.run(VerifyInputSegment::new("BattlePack")); // InitGFX
  //   r.run(VerifyInputSegment::new("BattlePack")); // InitItemsPocket
  //   r.run(VerifyInputSegment::new("BattlePack")); // ItemsPocketMenu
  //   r.run(MoveSegment::new(R)); // go to balls
  //   r.run(VerifyInputSegment::new("BattlePack")); // InitBallsPocket
  //   r.run(VerifyInputSegment::new("BattlePack")); // BallsPocketMenu
  //   r.run(MoveSegment::new(A)); // select pokeball
  //   r.run(MoveSegment::new(NIL));
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A)
  //       .seq(TextSegment::with_metric(CatchSuccessMetric {}).with_skip_ends(2))
  //       )); // Throw pokeball
  //   r.save("crystal_test_tmp2");
  //   r.load("crystal_test_tmp2");
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // Gotcha! // mon // caught!
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon's // date added
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // to the pokedex
  //   r.run(MoveSegment::new(A)); // confirm pokedex
  //   r.run(MoveSegment::new(B)); // confirm pokedex
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // Give a nickname to // mon // ?
  //   r.run(MoveSegment::new(R)); // K
  //   r.run(MoveSegment::new(D)); // K
  //   r.run(MoveSegment::new(A)); // K
  //   r.run(MoveSegment::new(START)); // confirm naming
  //   r.run(MoveSegment::new(A)); // confirm naming
  //   r.run(VerifyInputSegment::new("NamingScreenJoypadLoop")); // closing takes extra loop though input
  //   r.run(VerifyInputSegment::new("BattlePack")); // Pack_QuitRunScript
  //   r.run(VerifyInputSegment::new("BattlePack")); // exit
  // // }
  // r.run(SkipScriptSegment::new()); // Caught
  // r.run_debug(WalkToSegment::new(5, 24).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Mikey
  // r.save("crystal_before_mikey_90"); // best: crystal_before_mikey_g_

  // r.load("crystal_before_mikey_90");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Pidgey
  // if CHOOSE_TOTODILE {
  //   r.run(OHKOSegment::new(Move::Scratch).crit());
  // } else {
  //   r.run(OHKOSegment::new(Move::Tackle).crit());
  // }
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(NextTrainerMonSegment::new(Pokemon::Rattata, 4).with_expected_move(Move::Tackle));
  // if CHOOSE_TOTODILE { // Totodile
  //   r.run(KOSegment::new(Move::Scratch, EnemyAttack { mov: Move::Tackle, attack_type: EnemyAttackType::Failed } ));
  //   r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_level_up().with_learn_move()); // learn rage
  // } else {
  //   r.run(KOSegment::new(Move::Tackle, EnemyAttack { mov: Move::Tackle, attack_type: EnemyAttackType::Failed } ));
  //   r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_level_up());
  // }
  // r.save("crystal_after_mikey_90");

  // r.load("crystal_after_mikey_90");
  // r.run(SkipScriptSegment::new()); // Mikey SeenByTrainer end
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(6, 0));
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
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Route 31
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
  // r.save("crystal_before_abe_90");

  // r.load("crystal_before_abe_90");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2).with_expected_move(Move::Peck)); // Abe
  // if CHOOSE_TOTODILE {
  //   r.run(KOSegment::new(Move::Scratch, EnemyAttack { mov: Move::Peck, attack_type: EnemyAttackType::Hit } ));
  // } else {
  //   // Spearow Turn 1
  //   r.run(SelectMoveSegment::new(Move::Smokescreen));
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(9, 15).debug_print().expect(FightTurnResult::Hit { damage: 7, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())
  //       ));
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // enemy // mon // accuracy fell
  //   // Spearow Turn 2
  //   r.save("crystal_test_tmp");
  //   r.load("crystal_test_tmp");
  //   r.run(SelectMoveSegment::new(Move::Leer));
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(9, 15).debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       ));
  //   r.run(TextSegment::new()); // but it missed!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // enemy // mon // defense fell
  //   // Spearow Turn 3-5
  //   r.save("crystal_test_tmp2");
  //   r.load("crystal_test_tmp2");
  //   r.run(KOSegment::new(Move::Tackle, EnemyAttack { mov: Move::Peck, attack_type: EnemyAttackType::Failed } ));
  // }
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_level_up());
  // r.save("crystal_after_abe_90_"); // best: crystal_after_abe_90

  // r.load("crystal_after_abe_90_");
  // r.run(TurnSegment::new(D));
  // // r.run(WalkToSegment::new(3, 7));
  // // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::NoEvents))); // RNG wait
  // r.run(WalkToSegment::new(4, 6).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rod
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(4).with_expected_move(Move::Tackle)); // Rod
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
  //   r.run(NextTrainerMonSegment::new(Pokemon::Pidgey, 7).with_level_up().with_expected_move(Move::Tackle));
  //   r.run(KOSegment::new(Move::Scratch, EnemyAttack { mov: Move::Tackle, attack_type: EnemyAttackType::Failed } ));
  // } else {
  //   r.run(KOSegment::new(Move::Tackle, EnemyAttack { mov: Move::Tackle, attack_type: EnemyAttackType::Failed } ));
  //   r.run(NextTrainerMonSegment::new(Pokemon::Pidgey, 7).with_level_up().with_expected_move(Move::Tackle));
  //   r.run(KOSegment::new(Move::Tackle, EnemyAttack { mov: Move::Tackle, attack_type: EnemyAttackType::Failed } ));
  // }
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_rod_90"); // best: crystal_after_rod_g_

  // r.load("crystal_after_rod_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(5, 3));
  // r.run(WalkToSegment::new(5, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript)))); // Talk to Falkner
  // if CHOOSE_TOTODILE {
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(9).with_expected_move(Move::Tackle)); // Falkner
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
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(9).with_expected_move(Move::MudSlap)); // Falkner
  //   r.run(KOSegment::new(Move::Tackle, EnemyAttack { mov: Move::MudSlap, attack_type: EnemyAttackType::HitWithoutEffect } ).with_berry_recovery_after_turn(2));
  //   r.run(NextTrainerMonSegment::new(Pokemon::Pidgeotto, 9).with_level_up().with_expected_move(Move::Gust));
  //   r.save("crystal_test_tmp2");
  //   r.load("crystal_test_tmp2");
  //   // Pidgeotto turn 1
  //   r.run(MoveSegment::new(NIL)); // Rng Delay
  //   r.run(MoveSegment::new(NIL)); // Rng Delay
  //   r.run(SelectMoveSegment::new(Move::Leer));
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())
  //       ));
  //   r.run(TextSegment::new().with_skip_ends(2)); // enemy // mon // defense fell
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(9, 15).debug_print().expect(FightTurnResult::Hit { damage: 7 }).and_then(Gen2AIChooseMoveMetric {}.assert_eq(Move::Gust))).with_skip_ends(3).with_allowed_end_inputs(B).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.save("crystal_test_tmp3_");
  //   r.load("crystal_test_tmp3_");
  //   // Pidgeotto turn 2-3
  //   r.run(KOSegment::new(Move::Tackle, EnemyAttack { mov: Move::Gust, attack_type: EnemyAttackType::Hit } ));
  // // }
  // r.run(EndTrainerBattleSegment::with_defeat_texts(5).with_level_up());
  // r.save("crystal_after_falkner_90_"); // best: crystal_after_falkner_90_

  // r.load("crystal_after_falkner_90_");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(4)); // Falkner text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(9)); // Falkner text
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 15));
  // r.run(WarpSegment::new().with_input(D));
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(5)); // Elm phone call
  // r.run(TextSegment::new().with_skip_ends(6)); // Click
  // r.run(WalkToSegment::new(31, 25).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Center
  // r.run(WalkToSegment::new(4, 5));
  // r.run(WalkToSegment::new(4, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript)))); // Talk to Aide
  // r.run(SkipTextsSegment::new(3)); // Aide
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Accept egg
  // r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // Take egg
  // r.run(SkipTextsSegment::new(7)); // Aide
  // r.run(SkipScriptSegment::new());
  // r.run(WalkToSegment::new(4, 7));
  // r.run(WarpSegment::new().with_input(D)); // Leave Center
  // {
  //   r.run(WalkToSegment::new(9, 17).into(OverworldInteractionResult::Warped));
  //   r.run(WarpSegment::new()); // enter Mart
  //   r.run(WalkToSegment::new(3, 3));
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::NoEvents)));
  //   r.run(TurnSegment::new(L));
  //   r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // mart
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Escape Rope
  //   r.run(TextSegment::new());
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(U|R));
  //   r.run(MoveSegment::new(U|L));
  //   r.run(MoveSegment::new(U|R));
  //   r.run(MoveSegment::new(U|L));
  //   r.run(MoveSegment::new(A)); // x6
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3).with_confirm_input(A)); // buy
  //   r.run(SkipTextsSegment::new(1));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // X Attack
  //   r.run(TextSegment::new());
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // x2
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3).with_confirm_input(A)); // buy
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A));
  //   r.run(MoveSegment::new(B)); // close
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // close
  //   r.run(SkipTextsSegment::new(1));
  //   r.run(TurnSegment::new(R));
  //   r.run(WalkToSegment::new(3, 7));
  //   r.run(WarpSegment::new().with_input(D)); // Leave Mart
  // }
  // r.run(WalkToSegment::new(14, 36).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Route 32
  // r.run(WalkToSegment::new(7, 71).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(3)); // Slowpoketail
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Slowpoketail
  // r.run(SkipTextsSegment::new(1)); // Slowpoketail
  // r.run(WalkToSegment::new(6, 79).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Cave
  // r.run(WalkToSegment::new(17, 30));
  // r.run(WalkToSegment::new(17, 31));
  // r.run(WarpSegment::new().with_input(D)); // Leave Cave
  // r.run(WalkToSegment::new(9, 10));
  // r.run(JumpLedgeSegment::new(L));
  // r.run(WalkToSegment::new(-1, 14).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Azalea
  // r.run(WalkToSegment::new(9, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Kurt's house
  // r.run(WalkToSegment::new(3, 3));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript)))); // Talk to Kurt
  // r.run(SkipTextsSegment::new(16)); // Kurt
  // r.run(SkipScriptSegment::new());
  // r.run(WalkToSegment::new(3, 7));
  // r.run(WarpSegment::new().with_input(D)); // Leave Kurt's house
  // r.run(WalkToSegment::new(31, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Slowpoke Well
  // r.run(WalkToSegment::new(15, 10).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.save("crystal_before_well_rocket1_90");

  // r.load("crystal_before_well_rocket1_90");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(6).with_expected_move(Move::Tackle)); // Rocket
  // if CHOOSE_TOTODILE {
  //   // Rattata Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(6, 9).debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //     ));
  //   r.run(TextSegment::new()); // But it failed
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(6+1, 10).debug_print().expect(FightTurnResult::Hit { damage: 6, }).and_then(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle))).with_skip_ends(3).with_allowed_end_inputs(B).with_unbounded_buffer())
  //       )); // Rage //// mon // used // move // !
  //   // Rattata Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(6, 9).debug_print().expect(FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer())
  //     ));
  //   r.run(TextSegment::new()); // rage is building
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(12+2, 20).debug_print().expect(FightTurnResult::CriticalHit { damage: 19, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Rage //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit!
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle)).with_allowed_end_inputs(B).with_skip_ends(1)) // Rattata
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Rattata
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(6, 9).debug_print().expect(FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer())
  //     ));
  //   r.run(TextSegment::new()); // rage is building
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(18+3, 30).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &25 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Rage //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit!
  //   r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_level_up());
  // } else {
  //   r.run(KOSegment::new(Move::Tackle, EnemyAttack { mov: Move::Tackle, attack_type: EnemyAttackType::Failed } ));
  //   r.run(NextTrainerMonSegment::new(Pokemon::Rattata, 9).with_expected_move(Move::Tackle));
  //   r.run(KOSegment::new(Move::Tackle, EnemyAttack { mov: Move::Tackle, attack_type: EnemyAttackType::Failed } ));
  //   r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_level_up().with_learn_move()); // learned Ember
  // }
  // r.save("crystal_after_well_rocket1_90_");

  // r.load("crystal_after_well_rocket1_90_");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(14, 4).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // if CHOOSE_TOTODILE {
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2).with_expected_move(Move::LeechLife)); // Rocket
  //   // Zubat Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(6, 10).debug_print().expect(FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(3, 4).debug_print().expect(FightTurnResult::Hit { damage: 2, })).with_skip_ends(3).with_unbounded_buffer())
  //     ));
  //   r.run(SkipTextsSegment::new(1)); // sucked HP
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // rage is building
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::LeechLife)))); // confirm        
  //   // Zubat Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(12, 20).debug_print().expect(FightTurnResult::Hit { damage: 11, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(3, 4).debug_print().expect(FightTurnResult::Hit { damage: 2, })).with_skip_ends(3).with_unbounded_buffer())
  //     ));
  //   r.run(SkipTextsSegment::new(1)); // sucked HP
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // rage is building
  //   // Zubat Turn 3
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(18, 30).debug_print().expect(FightTurnResult::Hit { damage: 17, })).with_skip_ends(3).with_unbounded_buffer())
  //     ));
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::PoisonSting)).with_allowed_end_inputs(B).with_skip_ends(1)) // Ekans
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Ekans
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(4, 6).with_effect().debug_print().expect(FightTurnResult::HitWithoutEffect { damage: 3, })).with_skip_ends(3).with_unbounded_buffer())
  //     ));
  //   r.save("crystal_test_tmp");
  //   r.load("crystal_test_tmp");
  //   // r.run(SkipTextsSegment::new(2)); // A was wrapped
  //   r.run(TextSegment::new()); // rage is building
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(20, 32).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &30 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Rage //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit!
  //   r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_level_up().with_learn_move()); // learned Water Gun
  // } else {
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Rocket
  //   r.run(OHKOSegment::new(Move::Ember).crit());
  //   r.save("crystal_test_tmp");
  //   r.load("crystal_test_tmp");
  //   r.run(NextTrainerMonSegment::new(Pokemon::Ekans, 11).with_expected_move(Move::Wrap));
  //   r.run(KOSegment::new(Move::Ember, EnemyAttack { mov: Move::Wrap, attack_type: EnemyAttackType::Failed } ).has_effect());
  //   r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_level_up());
  // }
  // r.save("crystal_after_well_rocket2_90"); // best: crystal_after_well_rocket2_g_

  // r.load("crystal_after_well_rocket2_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(7, 6).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Rocket
  // if CHOOSE_TOTODILE {
  //   r.run(OHKOSegment::new(Move::Scratch).crit());
  //   r.run(NextTrainerMonSegment::new(Pokemon::Zubat, 9));
  //   r.run(OHKOSegment::new(Move::WaterGun).crit());
  //   r.run(NextTrainerMonSegment::new(Pokemon::Zubat, 9));
  //   r.run(OHKOSegment::new(Move::WaterGun).crit());
  // } else {
  //   r.run(OHKOSegment::new(Move::Ember).crit());
  //   r.run(NextTrainerMonSegment::new(Pokemon::Zubat, 9));
  //   r.run(OHKOSegment::new(Move::Ember).crit());
  //   r.run(NextTrainerMonSegment::new(Pokemon::Zubat, 9));
  //   r.run(OHKOSegment::new(Move::Ember).crit());
  // }
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_well_rocket3_90");

  // r.load("crystal_after_well_rocket3_90");
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 4));
  // r.run(WalkToSegment::new(5, 3).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3).with_expected_move(Move::PoisonGas)); // Rocket
  // if CHOOSE_TOTODILE {
  //   // Koffing Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Water Gun
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(10, 18).debug_print().expect(FightTurnResult::CriticalHit { damage: 17, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(TextSegment::new()); // critical hit!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer()) // mon // used // move // !
  //   ));
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // but it failed!
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::PoisonGas)))); // confirm        
  //   // Koffing Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(10, 18).debug_print().expect(FightTurnResult::Hit { damage: 10, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer()) // mon // used // move // !
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // poisoned
  //   // Koffing Turn 3
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(10, 18).debug_print().expect(FightTurnResult::Hit { damage: 10, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  // } else {
  //   r.run(KOSegment::new(Move::Ember, EnemyAttack { mov: Move::PoisonGas, attack_type: EnemyAttackType::Poisoned } ).has_effect());
  // }
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2).with_level_up());
  // r.save("crystal_after_well_rocket4_90"); // best: crystal_after_well_rocket4_90

  // r.load("crystal_after_well_rocket4_90");
  // if !CHOOSE_TOTODILE {
  //   r.run(TextSegment::new().with_skip_ends(2)); // evolution
  //   for _ in 0..14*4 { r.run(VerifyInputSegment::new("EvolutionAnimation")); }
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // evolution
  //   r.run(TextSegment::new().with_skip_ends(2)); // evolution
  // }
  // r.run(SkipScriptSegment::new()); // Rocket MapScript
  // r.run(SkipTextsSegment::new(6)); // Rocket
  // r.run(SkipScriptSegment::new()); // Kurt enters
  // r.run(SkipTextsSegment::new(4)); // Kurt
  // r.run(SkipScriptSegment::new()); // teleport to Kurt's house
  // r.run(WalkToSegment::new(3, 7));
  // r.run(WarpSegment::new().with_input(D)); // Leave Kurt's house
  // r.run(WalkToSegment::new(10, 15).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // Enter Gym
  // if CHOOSE_TOTODILE {
  //   r.run(WalkToSegment::new(5, 11).into(OverworldInteractionResult::SeenByTrainer));
  // } else {
  //   r.run(WalkToSegment::new(4, 11).into(OverworldInteractionResult::SeenByTrainer));
  // }
  // r.run(SkipScriptSegment::new()); // Amy&May
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Amy&May
  // if CHOOSE_TOTODILE {
  //   r.run(OHKOSegment::new(Move::Scratch).crit());
  //   r.run(NextTrainerMonSegment::new(Pokemon::Spinarak, 10).with_level_up());
  //   r.run(OHKOSegment::new(Move::Scratch).crit());
  // } else {
  //   r.run(OHKOSegment::new(Move::Ember));
  //   r.run(NextTrainerMonSegment::new(Pokemon::Ledyba, 10).with_level_up());
  //   r.run(OHKOSegment::new(Move::Ember));
  // }
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_amy_may_90");

  // r.load("crystal_after_amy_may_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(0, 5).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Josh
  // if CHOOSE_TOTODILE {
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(4).with_expected_move(Move::StunSpore)); // Josh
  //   r.run(KOSegment::new(Move::Scratch, EnemyAttack { mov: Move::StunSpore, attack_type: EnemyAttackType::Failed } ));
  // } else {
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(4)); // Josh
  //   r.run(OHKOSegment::new(Move::Ember));
  // }
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_josh_90");

  // r.load("crystal_after_josh_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(4, 7));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript)))); // Talk to Bugsy
  // if CHOOSE_TOTODILE {
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(6).with_expected_move(Move::Tackle)); // Bugsy
  //   // Metapod Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(D)); // Rage
  //   r.run(MoveSegment::new(NIL)); // Rage
  //   r.run(MoveSegment::new(D)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(6, 10).debug_print().expect(FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(4, 6).debug_print().expect(FightTurnResult::Hit { damage: 3, })).with_skip_ends(3).with_unbounded_buffer())
  //     ));
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // rage is building
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Tackle)))); // confirm        
  //   // Metapod Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(12, 20).debug_print().expect(FightTurnResult::Hit { damage: 11, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(4, 6).debug_print().expect(FightTurnResult::Hit { damage: 3, })).with_skip_ends(3).with_unbounded_buffer())
  //     ));
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // rage is building
  //   // Metapod Turn 3
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(18, 30).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage, } = v { damage >= &24 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //     ));
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.save("crystal_test_tmp");
  //   r.load("crystal_test_tmp");
  //   r.run(TextSegment::new().with_skip_ends(2)); // mon // grew to level // X
  //   r.run(DelaySegment::new(MoveSegment::with_metric(A|B, Gen2SwitchMonMetric {}.debug_print().assert_eq((Pokemon::Kakuna, 14)))));
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::PoisonSting)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Kakuna
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Kakuna Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(21, 30).debug_print().expect(FightTurnResult::Hit { damage: 20, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(3, 3).with_effect().debug_print().expect(FightTurnResult::HitWithoutEffect { damage: 2, })).with_skip_ends(3).with_unbounded_buffer())
  //     ));
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // rage is building
  //   // Kakuna Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(28, 40).debug_print().filter(|v| if let FightTurnResult::Hit { damage, } = v { damage >= &18 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //     ));
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.save("crystal_test_tmp2");
  //   r.load("crystal_test_tmp2");
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::FuryCutter)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Kakuna
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Scyther Turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Rage
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Rage //// mon // used // move // !
  //   r.run(TextSegment::new()); // but it failed!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(20, 32).debug_print().expect(FightTurnResult::CriticalHit { damage: 31, })).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // critical hit
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::FuryCutter)))); // confirm
  //   // Scyther Turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // neutral
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Rage //// mon // used // move // !
  //   r.run(TextSegment::new()); // but it failed!
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(20, 32).debug_print().filter(|v| if let FightTurnResult::Hit { damage } = v { damage >= &19 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  // } else {
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(6)); // Bugsy
  //   r.run(OHKOSegment::new(Move::Ember));
  //   r.run(NextTrainerMonSegment::new(Pokemon::Kakuna, 14).with_level_up());
  //   r.run(OHKOSegment::new(Move::Ember));
  //   r.run(NextTrainerMonSegment::new(Pokemon::Scyther, 16).with_expected_move(Move::FuryCutter));
  //   r.run(KOSegment::new(Move::Ember, EnemyAttack { mov: Move::FuryCutter, attack_type: EnemyAttackType::Failed } ).crit_last().has_effect());
  // }
  // r.run(EndTrainerBattleSegment::with_defeat_texts(4).with_level_up());
  // r.save("crystal_after_bugsy_90"); // best: crystal_after_bugsy_g_

  // r.load("crystal_after_bugsy_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(6)); // Bugsy text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(6)); // Bugsy text
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(4, 15));
  // r.run(WarpSegment::new().with_input(D)); // Leave Gym
  // r.run(WalkToSegment::new(5, 11).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Rival Scene
  // if CHOOSE_TOTODILE {
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(7).with_expected_move(Move::Hypnosis)); // Rival
  //   // Gastly
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Water Gun
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Hypnosis //// mon // used // move // !
  //   r.run(TextSegment::new()); // but it failed
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(18, 33).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &32 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.save("crystal_test_tmp");
  //   r.load("crystal_test_tmp");
  //   r.run(TextSegment::new().with_unbounded_buffer()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().filter(|v| v == &Move::RazorLeaf).into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Bayleef
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Bayleef turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(D)); // Scratch
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Razor Leaf //// mon // used // move // !
  //   r.run(TextSegment::new()); // but it failed
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(8, 14).debug_print().expect(FightTurnResult::CriticalHit { damage: 14 })).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // critical hit
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::RazorLeaf)))); // confirm
  //   // Bayleef turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Scratch
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Razor Leaf //// mon // used // move // !
  //   r.run(TextSegment::new()); // but it failed
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(8, 14).debug_print().expect(FightTurnResult::CriticalHit { damage: 14 })).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // critical hit
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::RazorLeaf)))); // confirm
  //   // Bayleef turn 3
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Scratch
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Razor Leaf //// mon // used // move // !
  //   r.run(TextSegment::new()); // but it failed
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(8, 14).debug_print().expect(FightTurnResult::CriticalHit { damage: 14 })).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // critical hit
  //   // Bayleef turn 4
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Scratch
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Razor Leaf //// mon // used // move // !
  //   r.run(TextSegment::new()); // but it failed
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(8, 14).debug_print().expect(FightTurnResult::Hit { damage: 8 })).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  //   r.save("crystal_test_tmp2");
  //   r.load("crystal_test_tmp2");
  //   r.run(TextSegment::new()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Supersonic)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Zubat
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Zubat turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Scratch
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(17, 30).debug_print().expect(FightTurnResult::Hit { damage: 16 })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // didn't affect mon
  //   // Zubat turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Ember
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(17, 30).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &23 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Scratch //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  // } else {
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(7)); // Rival
  //   r.run(OHKOSegment::new(Move::Ember).crit()); // Gastly
  //   r.run(NextTrainerMonSegment::new(Pokemon::Croconaw, 16).with_expected_move(Move::Leer).with_buffer_size(1024));
  //   // Croconaw turn 1
  //   r.run(SelectMoveSegment::new(Move::Ember).with_buffer_size(1024));
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey))).with_buffer_size(4096)
  //       .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(7, 13).with_effect().expect(FightTurnResult::CriticalHit { damage: 13, })).with_skip_ends(3).with_buffer_size(4096))
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  //   r.run(SkipTextsSegment::new(1)); // not very effective
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // burned
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // defense down
  //   r.run(TextSegment::new().with_allowed_end_inputs(A).with_unbounded_buffer()); // hurt by burn
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.expect(Move::Leer)))); // confirm
  //   // Croconaw turn 2
  //   r.run(SelectMoveSegment::new(Move::Ember));
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(7, 13).expect(FightTurnResult::CriticalHit { damage: 13, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // not very effective
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // defense down
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // hurt by burn
  //   // Croconaw turn 3
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   r.run(SelectMoveSegment::new(Move::Ember));
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(7, 13).expect(FightTurnResult::CriticalHit { damage: 13, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  //   r.run(SkipTextsSegment::new(1)); // not very effective
  //   r.run(NextTrainerMonSegment::new(Pokemon::Zubat, 14).with_level_up());
  //   r.run(OHKOSegment::new(Move::Ember).crit()); // Zubat
  // }
  // r.run(EndTrainerBattleSegment::with_defeat_texts(3));
  // r.save("crystal_after_rival2_90__"); // crystal_after_rival2_90__

  // r.load("crystal_after_rival2_90__");
  // if CHOOSE_TOTODILE {
  //   r.run(TextSegment::new().with_skip_ends(2)); // evolution
  //   for _ in 0..14*4 { r.run(VerifyInputSegment::new("EvolutionAnimation")); }
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // evolution
  //   r.run(TextSegment::new().with_skip_ends(2)); // evolution
  // }
  // r.run(SkipScriptSegment::new()); // Rival MapScript
  // r.run(SkipTextsSegment::new(12)); // rival
  // r.run(SkipScriptSegment::new()); // Rival MapScript
  // r.run(WalkToSegment::new(2, 11));
  // r.run(WarpSegment::new().with_input(L)); // Enter gate
  // r.run(WalkToSegment::new(0, 5));
  // r.run(WarpSegment::new().with_input(L)); // Enter forest
  // r.run(WalkToSegment::new(14, 32));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(2)); // Farfetch'd
  // r.run(SkipScriptSegment::new()); // Farfetch'd MapScript
  // r.run(WalkToSegment::new(15, 24));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(1)); // Farfetch'd
  // r.run(SkipScriptSegment::new()); // Farfetch'd MapScript
  // r.run(WalkToSegment::new(15, 28));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(1)); // Farfetch'd
  // r.run(SkipScriptSegment::new()); // Farfetch'd MapScript
  // r.run(WalkToSegment::new(12, 35));
  // r.run(WalkToSegment::new(11, 35));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(1)); // Farfetch'd
  // r.run(SkipScriptSegment::new()); // Farfetch'd MapScript
  // r.run(WalkToSegment::new(5, 29));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(7)); // farfetch'd owner
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got HM01
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put HM01 in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(4)); // farfetch'd owner
  // r.run(TurnSegment::new(D));
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
  //   r.run(MoveSegment::new(L)); // TMs
  //   r.run(VerifyInputSegment::new("Pack")); // InitTMHMPocket
  //   r.run(VerifyInputSegment::new("Pack")); // TMHMPocketMenu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // HM01
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // teach
  //   r.run(SkipTextsSegment::new(1)); // Booted up HM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // contains // move // .
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // teach // move // to mon?
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // A
  //   r.run(OverrideMoveSegment::new(0)); // override tackle  
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
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Goldenrod
  // r.run(WalkToSegment::new(29, 29).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter bike shop
  // r.run(WalkToSegment::new(4, 2));
  // r.run(WalkToSegment::new(5, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(3)); // bike shop
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // take bike
  // r.run(SkipTextsSegment::new(3)); // bike shop
  // r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // put bicycle in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the Key pocket
  // r.run(SkipTextsSegment::new(2)); // bike shop
  // r.run(TurnSegment::new(U));
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
  //   r.run(MoveSegment::new(L)); // Key Items
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
  //   //   r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  //   // }
  // }
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(19, 1).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gate house
  // r.run(WalkToSegment::new(3, 4));
  // r.run(WalkToSegment::new(2, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(4)); // Get farfetch'd
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Get farfetch'd
  // r.run(SkipTextsSegment::new(4)); // Get farfetch'd
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // Get farfetch'd
  // r.run(SkipTextsSegment::new(4)); // Get farfetch'd
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(4, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter route 35
  // r.run(WalkToSegment::new(3, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gate house
  // r.run(WalkToSegment::new(3, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter park
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(33, 19));
  // r.run(WarpSegment::new().with_input(R)); // enter gate house
  // r.run(WalkToSegment::new(9, 5));
  // r.run(WarpSegment::new().with_input(R)); // enter route 36
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(33, 14));
  // r.run(WalkToSegment::new(33, 13));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(9)); // floria
  // r.run(SkipScriptSegment::new()); // floria
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(18, 9));
  // r.run(WarpSegment::new().with_input(L)); // enter gate house
  // r.run(WalkToSegment::new(0, 5));
  // r.run(WarpSegment::new().with_input(L)); // enter park
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(11, 47));
  // r.run(WarpSegment::new().with_input(D)); // enter gate house
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(WalkToSegment::new(6, 1));
  // r.run_debug(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Turned on PC
  // r.run(MoveSegment::new(A)); // Bills PC
  // r.run(SkipTextsSegment::new(2)); // Bills PC accessed
  // { // deposit egg
  //   r.run(MoveSegment::new(D)); // deposit
  //   r.run(MoveSegment::new(A)); // deposit
  //   r.run(VerifyInputSegment::new("_DepositPKMN")); // Init
  //   r.run(MoveSegment::new(D)); // Egg
  //   r.run(MoveSegment::new(D|L)); // Egg
  //   r.run(MoveSegment::new(A)); // Egg
  //   r.run(VerifyInputSegment::new("_DepositPKMN")); // WhatsUp
  //   r.run(VerifyInputSegment::new("_DepositPKMN")); // Submenu
  //   r.run(MoveSegment::new(A)); // Deposit
  //   r.run(VerifyInputSegment::new("_DepositPKMN")); // Init
  //   r.run(MoveSegment::new(B)); // Cancel
  //   r.run(VerifyInputSegment::new("_DepositPKMN")); // quit
  //   r.run(VerifyInputSegment::new("_DepositPKMN")); // quit
  //   r.run(MoveSegment::new(B)); // Cancel
  // }
  // // { // restore PP
  // //   r.debug_write_memory(0xdcd9, 0xf3); // change to Raikou
  // //   r.debug_write_memory(0xdd0f, 0xf3); // change to Raikou
  // //   r.run(MoveSegment::new(D)); // deposit
  // //   r.run(MoveSegment::new(A)); // deposit
  // //   r.run(VerifyInputSegment::new("_DepositPKMN")); // Init
  // //   r.run(MoveSegment::new(D)); // Poliwag
  // //   r.run(MoveSegment::new(A)); // Poliwag
  // //   r.run(VerifyInputSegment::new("_DepositPKMN")); // WhatsUp
  // //   r.run(VerifyInputSegment::new("_DepositPKMN")); // Submenu
  // //   r.run(MoveSegment::new(A)); // Deposit
  // //   r.run(VerifyInputSegment::new("_DepositPKMN")); // Init
  // //   r.run(MoveSegment::new(B)); // Cancel
  // //   r.run(VerifyInputSegment::new("_DepositPKMN")); // quit
  // //   r.run(VerifyInputSegment::new("_DepositPKMN")); // quit
  // //   r.run(MoveSegment::new(U)); // withdraw
  // //   r.run(MoveSegment::new(A)); // withdraw
  // //   r.run(VerifyInputSegment::new("_WithdrawPKMN")); // Init
  // //   r.run(MoveSegment::new(A)); // Poliwag
  // //   r.run(VerifyInputSegment::new("_WithdrawPKMN")); // WhatsUp
  // //   r.run(VerifyInputSegment::new("_WithdrawPKMN")); // Submenu
  // //   r.run(MoveSegment::new(A)); // withdraw
  // //   r.run(VerifyInputSegment::new("_WithdrawPKMN")); // Init
  // //   r.run(MoveSegment::new(B)); // Cancel
  // //   r.run(VerifyInputSegment::new("_WithdrawPKMN")); // quit
  // //   r.run(VerifyInputSegment::new("_WithdrawPKMN")); // quit
  // //   r.run(MoveSegment::new(B)); // Cancel
  // // }
  // r.run(MoveSegment::new(NIL)); // Cancel
  // r.run(MoveSegment::new(B)); // Cancel
  // r.run(TurnSegment::new(U)); 
  // r.run(WalkToSegment::new(4, 6));
  // r.run(WalkToSegment::new(4, 7));
  // r.run(WarpSegment::new().with_input(D)); // enter route 35
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(9, 33));
  // r.run(WarpSegment::new().with_input(D)); // enter gate house
  // r.run(WalkToSegment::new(4, 7));
  // r.run(WarpSegment::new().with_input(D)); // enter Goldenrod
  // r.run(WalkToSegment::new(24, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.run(WalkToSegment::new(13, 13).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Carrie
  // r.save("crystal_before_carrie_90");

  // r.load("crystal_before_carrie_90");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2).with_expected_move(Move::Charm)); // Carrie
  // print_battle_stats(r);
  // if CHOOSE_TOTODILE {
  //   r.run(KOSegment::new(Move::Cut, EnemyAttack { mov: Move::Charm, attack_type: EnemyAttackType::StatWayUpDown } ));
  //   r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // } else {
  //   r.run(KOSegment::new(Move::Ember, EnemyAttack { mov: Move::Charm, attack_type: EnemyAttackType::StatWayUpDown } ).has_effect());
  //   r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_level_up());
  // }
  // r.save("crystal_after_carrie_90");

  // r.load("crystal_after_carrie_90");
  // // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // RNG delay
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(8, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(6).with_expected_move(Move::DoubleSlap)); // Whitney
  // if CHOOSE_TOTODILE {
  //   // Clefairy Turn 1
  //   r.run(SelectMoveSegment::new(Move::Rage));
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(8, 12).debug_print().expect(FightTurnResult::Hit { damage: 8, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(4, 6).debug_print().expect(FightTurnResult::Hit { damage: 3, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Rage //// mon // used // move // !
  //   r.run(TextSegment::new().with_unbounded_buffer()); // rage is building
  //   r.run_debug(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A|B,
  //         Gen2MultiHitCountMetric {}.debug_print().expect(3).and_then(
  //         Gen2AdditionalMultiHitMetric {}.debug_print().expect(FightTurnResult::Hit { damage: 3, }))) // confirm
  //   ));
  //   r.run(TextSegment::new().with_unbounded_buffer()); // rage is building
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A|B, Gen2AdditionalMultiHitMetric {}.debug_print().expect(FightTurnResult::Hit { damage: 3, })) // confirm
  //   ));
  //   r.run(SkipTextsSegment::new(1)); // rage is building
  //   r.save("crystal_test_tmp2");
  //   r.load("crystal_test_tmp2");
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // hit n times
  //   // Clefairy Turn 2
  //   r.run(SelectMoveSegment::new(Move::Rage));
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(32, 48).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &45 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Rage //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  //   r.save("crystal_test_tmp3");
  //   r.load("crystal_test_tmp3");
  //   r.run(TextSegment::new().with_unbounded_buffer()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().filter(|v| v == &Move::Rollout).into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Miltank
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Miltank turn 1
  //   r.run(SelectMoveSegment::new(Move::Rage));
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Rage //// mon // used // move // !
  //   r.run(TextSegment::new()); // but it failed
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(20, 32).debug_print().expect(FightTurnResult::CriticalHit { damage: 31, })).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(TextSegment::new().with_allowed_end_inputs(A).with_unbounded_buffer()); // critical hit
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Rollout)))); // confirm
  //   // Miltank turn 2
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   r.run(SelectMoveSegment::new(Move::Rage));
  //   r.save("crystal_test_tmp");
  //   r.load("crystal_test_tmp");
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(7, 12).debug_print().expect(FightTurnResult::Hit { damage: 5 })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Rage //// mon // used // move // !
  //   r.run(TextSegment::new()); // rage is building
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(25, 40).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &37 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  // } else {
  //   r.run(KOSegment::new(Move::Ember, EnemyAttack { mov: Move::DoubleSlap, attack_type: EnemyAttackType::HitFailed } ).has_effect());
  //   r.run(NextTrainerMonSegment::new(Pokemon::Miltank, 20).with_expected_move(Move::Rollout));
  //   r.run(KOSegment::new(Move::Ember, EnemyAttack { mov: Move::Rollout, attack_type: EnemyAttackType::Failed } ).has_effect().crit_last());
  // }
  // r.run(EndTrainerBattleSegment::with_defeat_texts(4).with_level_up());
  // r.save("crystal_after_whitney_90"); // best: crystal_after_whitney_g_

  // r.load("crystal_after_whitney_90");
  // r.run(SkipTextsSegment::new(3)); // Whitney
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(8, 5).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Whitney cry scene
  // r.run(SkipTextsSegment::new(3)); // Lass
  // r.run(SkipScriptSegment::new()); // Whitney cry scene
  // r.run(WalkToSegment::new(8, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(4)); // Whitney
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(4)); // Whitney text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(4)); // Whitney text
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(3, 17));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(WarpSegment::new().with_input(D)); // leave Gym
  // r.run(WalkToSegment::new(29, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter flower shop
  // r.run(WalkToSegment::new(2, 6));
  // r.run(WalkToSegment::new(3, 6));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(4)); // Get watering can
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(2, 6));
  // r.run(WalkToSegment::new(2, 5));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(3)); // Get watering can
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // received // squirtbottle // !
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // squirtbottle // in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // key pocket // .
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(2, 7));
  // r.run(WarpSegment::new().with_input(D)); // leave house
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // // { // teleport
  // // //  r.debug_write_memory(0xdd11, 100); // learn teleport
  // //   r.run(MoveSegment::new(START)); // Open menu
  // //   r.run(MoveSegment::new(D));
  // //   r.run(MoveSegment::new(A)); // mon
  // //   r.run(MoveSegment::new(D));
  // //   // r.run(MoveSegment::new(NIL));
  // //   // r.run(MoveSegment::new(D));
  // //   r.run(MoveSegment::new(A)); // Abra
  // //   r.run(MoveSegment::new(NIL));
  // //   r.run(MoveSegment::new(A)); // Teleport
  // //   r.run(SkipTextsSegment::new(1)); // Teleport
  // //   r.run(SkipScriptSegment::new()); // Teleport MapScript
  // //   r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // //   r.run(TurnSegment::new(U));
  // //   r.run(WalkToSegment::new(-1, 9).into(OverworldInteractionResult::MapConnection));
  // //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Route 36
  // //   r.run(WalkToSegment::new(36, 9));
  // //   r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // // }
  // { // bike
  //   r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  //   r.run(WalkToSegment::new(19, 1).into(OverworldInteractionResult::Warped));
  //   r.run(WarpSegment::new()); // enter gate house
  //   r.run(WalkToSegment::new(4, 0).into(OverworldInteractionResult::Warped));
  //   r.run(WarpSegment::new()); // enter route 35
  //   r.run(WalkToSegment::new(3, 5).into(OverworldInteractionResult::Warped));
  //   r.run(WarpSegment::new()); // enter gate house
  //   r.run(WalkToSegment::new(3, 0).into(OverworldInteractionResult::Warped));
  //   r.run(WarpSegment::new()); // enter park
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // RNG Delay
  //   r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  //   r.run(WalkToSegment::new(32, 19));
  //   r.run(WalkToSegment::new(33, 19));
  //   r.run(WarpSegment::new().with_input(R)); // enter gate house
  //   r.run(WalkToSegment::new(9, 5));
  //   r.run(WarpSegment::new().with_input(R)); // enter route 36
  //   r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  //   r.run(WalkToSegment::new(35, 10));
  //   r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript)))); // 119254/536 Egg:119955 Pidgey:119960 Poliwag:119993 Raikou:120035 Entei:120056
  // }
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Sudowoodo
  // r.run(SkipTextsSegment::new(1)); // use squirtbottle
  // r.run(SkipScriptSegment::new()); // Sudowoodo MapScript
  // r.run(SkipTextsSegment::new(3)); // use squirtbottle
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // wild // mon // appeared
  // r.run(TextSegment::new().with_skip_ends(2)); // Go // mon // !
  // r.run(MoveSegment::new(D));
  // r.run(MoveSegment::new(R));
  // r.run(MoveSegment::new(A));
  // r.run(SkipTextsSegment::new(1)); // got away safely
  // r.run(SkipScriptSegment::new()); // Sudowoodo MapScript
  // r.run(WalkToSegment::new(31, -1).into(OverworldInteractionResult::MapConnection)); // Pidgey:121262 Poliwag:121278 Quilava:121290 Raikou:121298 Entei:121311 Jynx:121380
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Route 37
  // r.run(WalkToSegment::new(7, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Ecruteak
  // r.run(WalkToSegment::new(5, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter burned tower
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(SkipScriptSegment::new()); // Eusine MapScript
  // r.run(SkipTextsSegment::new(8)); // Eusine
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(11, 9).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Rival MapScript
  // r.save("crystal_before_rival3_90");

  // r.load("crystal_before_rival3_90");
  // if CHOOSE_TOTODILE {
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(8).with_expected_move(Move::Lick)); // Rival
  //   // Haunter turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Water Gun
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(6, 9).with_effect().debug_print().expect(FightTurnResult::HitWithoutEffect { damage: 5, })).with_skip_ends(3).with_unbounded_buffer()) // mon // used // move // !
  //     .seq(TextSegment::with_metric(
  //         Gen2NormalHitMetric::with_expected_max_damage(15, 27).debug_print().expect(FightTurnResult::CriticalHit { damage: 27, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Water Gun //// mon // used // move // !
  //   r.run(TextSegment::new().with_allowed_end_inputs(A).with_unbounded_buffer()); // critical hit
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().assert_eq(Move::Lick)))); // confirm
  //   // Haunter turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Water Gun
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::EnemyFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(6, 9).with_effect().debug_print().expect(FightTurnResult::HitWithoutEffect { damage: 5, })).with_skip_ends(3).with_unbounded_buffer()) // mon // used // move // !
  //     .seq(TextSegment::with_metric(
  //         Gen2NormalHitMetric::with_expected_max_damage(15, 27).debug_print().expect(FightTurnResult::CriticalHit { damage: 27, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Water Gun //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(TextSegment::new().with_unbounded_buffer()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Supersonic)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Magnemite
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Magnemite turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Water Gun
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(16, 30).debug_print().expect(FightTurnResult::Hit { damage: 16, })).with_skip_ends(3).with_unbounded_buffer())
  //       .seq(TextSegment::with_metric(
  //           Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Water Gun //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed
  //   // Magnemite turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(NIL)); // Water Gun
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(16, 30).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &26 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Water Gun //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  //   r.save("crystal_test_tmp3");
  //   r.load("crystal_test_tmp3");
  //   { // override move
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // trying to learn
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // move // .
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // but // mon // can't learn more
  //     r.run(SkipTextsSegment::new(1)); // than four moves
  //     r.run(SkipTextsSegment::new(1)); // delete to make room
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // for // move // ?
  //     r.run(SkipTextsSegment::new(1).with_confirm_input(D)); // forget Leer
  //     r.run(MoveSegment::new(A));
  //     r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // 1, 2, poof!
  //     r.run(MoveSegment::new(A)); // confirm text line
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // mon // forgot // move // .
  //     r.run(SkipTextsSegment::new(1)); // and
  //     r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // mon // learned // move // !
  //   }
  //   r.run(TextSegment::new().with_unbounded_buffer()); // sent out
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::new(A|B).seq(
  //       TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().filter(|v| v == &Move::RazorLeaf).into_unit()).with_allowed_end_inputs(Input::all() - A).with_skip_ends(1)) // Bayleef
  //     )
  //   );
  //   println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  //   println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  //   println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  //   println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  //   // Bayleef turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(D)); // Cut
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(14, 24).debug_print().expect(FightTurnResult::CriticalHit { damage: 23, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Cut //// mon // used // move // !
  //   r.run(TextSegment::new().with_allowed_end_inputs(A)); // critical hit
  //   r.run(
  //     DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //     .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  //   );
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed
  //   // Bayleef turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(D)); // Bite
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(11, 20).with_effect().debug_print().expect(FightTurnResult::CriticalHit { damage: 19, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Ember //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  //   r.save("crystal_test_tmp2");
  //   r.load("crystal_test_tmp2");
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // flinched
  //   // Bayleef turn 3
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Cut
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(14, 24).debug_print().expect(FightTurnResult::CriticalHit { damage: 23, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Cut //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  //   r.save("crystal_test_tmp");
  //   r.load("crystal_test_tmp");
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
  //   // Zubat turn 1
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(D)); // Bite
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(19, 36).with_effect().debug_print().expect(FightTurnResult::Hit { damage: 19, })).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Bite //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // flinched
  //   // Zubat turn 2
  //   r.run(MoveSegment::new(A)); // Fight
  //   r.run(MoveSegment::new(U)); // Cut
  //   r.run(
  //     DelaySegment::new(
  //       MoveSegment::with_metric(A,
  //           BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
  //       .seq(TextSegment::with_metric(
  //           Gen2NormalHitMetric::with_expected_max_damage(25, 45).debug_print().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &33 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //       )); // Cut //// mon // used // move // !
  //   r.run(SkipTextsSegment::new(1)); // critical hit
  // } else {
  //   r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(8).with_expected_move(Move::Spite)); // Rival
  //   r.run(KOSegment::new(Move::Ember, EnemyAttack { mov: Move::Spite, attack_type: EnemyAttackType::Spite } ).has_effect());
  //   r.save("crystal_test_tmp");
  //   r.load("crystal_test_tmp");
  //   r.run(NextTrainerMonSegment::new(Pokemon::Croconaw, 22).with_expected_move(Move::Leer));
  //   r.save("crystal_test_tmp2_");
    // r.load("crystal_test_tmp2_");
    // // Croconaw turn 1
    // r.run(SelectMoveSegment::new(Move::Ember)); // Fight
    // r.run(
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         BattleMoveOrderMetric {}.expect(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.expect(BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(8, 15).with_effect().expect(FightTurnResult::CriticalHit { damage: 13, })).with_skip_ends(3).with_unbounded_buffer())
    //     )); // Ember //// mon // used // move // !
    // r.save("crystal_test_tmp3");
    // r.load("crystal_test_tmp3");
    // r.run(SkipTextsSegment::new(1)); // critical hit
    // r.run(SkipTextsSegment::new(1)); // not very effective
    // r.run(TextSegment::new().with_allowed_end_inputs(A)); // burned
    // r.run(
    //   DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
    //   .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
    // );
    // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // Stat down
    // r.run(TextSegment::new().with_allowed_end_inputs(A).with_unbounded_buffer()); // hurt by burn
    // r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.expect(Move::Leer)))); // confirm
    // r.save("crystal_test_tmp4");
  //   r.load("crystal_test_tmp4");
  //   r.run(KOSegment::new(Move::Cut, EnemyAttack { mov: Move::Leer, attack_type: EnemyAttackType::StatUpDown } ).with_burn_damage());
  //   r.run(NextTrainerMonSegment::new(Pokemon::Zubat, 20).with_level_up().with_skip_learning_move().with_expected_move(Move::Supersonic));
  //   r.run(KOSegment::new(Move::Cut, EnemyAttack { mov: Move::Supersonic, attack_type: EnemyAttackType::Failed } ));
  //   r.run(NextTrainerMonSegment::new(Pokemon::Magnemite, 18));
  //   r.run(OHKOSegment::new(Move::Ember));
  // }
  // r.run(EndTrainerBattleSegment::with_defeat_texts(3));
  // r.save("crystal_after_rival3_90"); // best: crystal_after_rival3_g__

  // r.load("crystal_after_rival3_90");
  // r.run(SkipTextsSegment::new(3)); // Rival
  // r.run(SkipScriptSegment::new()); // Rival MapScript
  // r.run(SkipTextsSegment::new(4)); // Rival
  // r.run(WarpSegment::new()); // fall
  // r.run(SkipScriptSegment::new()); // Rival MapScript
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // RNG delay
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(10, 6).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Suicune MapScript
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // RNG delay
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   // r.run(MoveSegment::new(NIL)); // RNG delay
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(VerifyInputSegment::new("Pack")); // InitGFX
  //   r.run(VerifyInputSegment::new("Pack")); // InitItemsPocket
  //   r.run(VerifyInputSegment::new("Pack")); // ItemsPocketMenu
  //   r.run(MoveSegment::new(D));
  //   // r.run(MoveSegment::new(NIL)); // RNG delay
  //   r.run(MoveSegment::new(A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Use
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(SkipTextsSegment::new(1)); // Used Escape Rope
  //   for _ in 0..32 {
  //     r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::ScriptRunning(PlayerEventScript::MapScript))));
  //   }
  //   r.run(MoveSegment::with_metric(NIL, RoamMonLocationMetric::new(Pokemon::Raikou).debug_print().expect((1, 12)))); // expect Raikou on Route 38
  //   r.run(SkipScriptSegment::new()); // Escape Rope MapScript
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::ForcedMovement)));
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(6, 10));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(11, 23));
  // r.run(WalkToSegment::new(17, 36).into(OverworldInteractionResult::MapConnection).with_unbounded_buffer());
  // r.run(MoveSegment::with_metric(NIL, RoamMonLocationMetric::new(Pokemon::Raikou).debug_print().expect((10, 4))).with_unbounded_buffer()); // Enter Route 37, expect Raikou
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // // r.run_debug(WalkToSegment::new(8, 1).with_unbounded_buffer());
  // r.run_debug(WalkToSegment::new(8, 2).with_unbounded_buffer());
  // r.run(MoveSegment::new(NIL).with_unbounded_buffer());
  // r.run(DelaySegment::new(
  //   MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Turned)).with_unbounded_buffer()
  //   .seq(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::ScriptRunning(PlayerEventScript::JoyChangeFacing))).with_unbounded_buffer())
  //   .seq(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::ScriptRunning(PlayerEventScript::JoyChangeFacing))).with_unbounded_buffer())
  //   .seq(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::ScriptRunning(PlayerEventScript::JoyChangeFacing))).with_unbounded_buffer())
  //   .seq(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::RandomEncounter { species: Pokemon::Raikou, level: 40 }).and_then(Gen2DVMetric {}.filter(|v| {
  //       if v.atk < 15 || v.spc < 15 { return false; } // raikou
  //       log::info!("Chosen DVs: {:?}", v); true
  //     }).into_unit())))
  // ));
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // Wild // Raikou // appeared!
  // r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(B)); // Go // mon // !
  // r.run(MoveSegment::new(D)); // pack
  // r.run(MoveSegment::new(A)); // select pack
  // r.run(VerifyInputSegment::new("BattlePack")); // InitGFX
  // r.run(VerifyInputSegment::new("BattlePack")); // InitItemsPocket
  // r.run(VerifyInputSegment::new("BattlePack")); // ItemsPocketMenu
  // r.run(MoveSegment::new(R)); // go to balls
  // r.run(VerifyInputSegment::new("BattlePack")); // InitBallsPocket
  // r.run(VerifyInputSegment::new("BattlePack")); // BallsPocketMenu
  // r.run(MoveSegment::new(A)); // select pokeball
  // r.run(MoveSegment::new(NIL));
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::new(A)
  //     .seq(TextSegment::with_metric(CatchSuccessMetric {}).with_skip_ends(2).with_unbounded_buffer())
  //     )); // Throw pokeball
  // r.save("crystal_test_tmp_");
  // r.load("crystal_test_tmp_");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // Gotcha! // mon // caught!
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon's // date added
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // to the pokedex
  // r.run(MoveSegment::new(A)); // confirm pokedex
  // r.run(MoveSegment::new(B)); // confirm pokedex
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // Give a nickname to // mon // ?
  // r.run(MoveSegment::new(R)); // B
  // r.run(MoveSegment::new(A)); // B
  // r.run(MoveSegment::new(START)); // confirm naming
  // r.run(MoveSegment::new(A)); // confirm naming
  // r.run(VerifyInputSegment::new("NamingScreenJoypadLoop")); // closing takes extra loop though input
  // r.run(VerifyInputSegment::new("BattlePack")); // Pack_QuitRunScript
  // r.run(VerifyInputSegment::new("BattlePack")); // exit
  // r.run(SkipScriptSegment::new()); // After catch scripts
  // r.run(WalkToSegment::new(8, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::MapConnection))); // Enter Ecruteak
  // r.run(WalkToSegment::new(23, 21).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter dance theater
  // r.run(WalkToSegment::new(1, 2));
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Quilava
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D)); 
  //   r.run(MoveSegment::new(A)); // switch
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // switch
  //   r.run(MoveSegment::new(B)); // close
  //   r.run(MoveSegment::new(START)); // close
  // }
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.save("crystal_before_kimono1_90");

  // r.load("crystal_before_kimono1_90");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // kimono
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Flareon
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_kimono1_90");

  // r.load("crystal_after_kimono1_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(2, 2));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // kimono
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Espeon
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_kimono2_90");

  // r.load("crystal_after_kimono2_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(5, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // kimono
  // r.run(OHKOSegment::new(Move::QuickAttack).crit()); // Umbreon
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_kimono3_90");

  // r.load("crystal_after_kimono3_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(8, 1));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // kimono
  // r.run(OHKOSegment::new(Move::QuickAttack).crit()); // Vaporeon
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_kimono4_90");

  // r.load("crystal_after_kimono4_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(10, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // kimono
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Jolteon
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_kimono5_90");

  // r.load("crystal_after_kimono5_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(7, 8));
  // r.run(WalkToSegment::new(7, 9));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(10)); // Get Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got HM03
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put HM03 in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(3)); // Get Surf
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(6, 13));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(6, 27).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.run(WalkToSegment::new(4, 13).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Sage Ping
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Sage Ping
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Gastly
  // r.run(NextTrainerMonSegment::new(Pokemon::Gastly, 16));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Gastly
  // r.run(NextTrainerMonSegment::new(Pokemon::Gastly, 16));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Gastly
  // r.run(NextTrainerMonSegment::new(Pokemon::Gastly, 16));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Gastly
  // r.run(NextTrainerMonSegment::new(Pokemon::Gastly, 16));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Gastly
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_sage_ping_90");

  // r.load("crystal_after_sage_ping_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(6, 13));
  // r.run(WalkToSegment::new(6, 11));
  // r.run(WalkToSegment::new(3, 11));
  // r.run(WalkToSegment::new(3, 7).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Sage Jeffrey
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(4)); // Sage Jeffrey
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Haunter
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_sage_jeffrey_90");

  // r.load("crystal_after_sage_jeffrey_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(6, 7));
  // r.run(WalkToSegment::new(6, 5).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Medium Martha
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Medium Martha
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Gastly
  // r.run(NextTrainerMonSegment::new(Pokemon::Haunter, 20).with_level_up().with_override_move(Move::Leer)); // learn Spark
  // r.run(OHKOSegment::new(Move::Spark)); // Haunter
  // r.run(NextTrainerMonSegment::new(Pokemon::Gastly, 20));
  // r.run(OHKOSegment::new(Move::Spark)); // Gastly
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_medium_martha_90");

  // r.load("crystal_after_medium_martha_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(5, 3));
  // r.run(WalkToSegment::new(5, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(16)); // Morty
  // r.run(OHKOSegment::new(Move::Spark)); // Gastly
  // r.run(NextTrainerMonSegment::new(Pokemon::Haunter, 21));
  // r.run(OHKOSegment::new(Move::Spark)); // Haunter
  // r.run(NextTrainerMonSegment::new(Pokemon::Gengar, 25));
  // r.run(OHKOSegment::new(Move::Spark)); // Gengar
  // r.run(NextTrainerMonSegment::new(Pokemon::Haunter, 23));
  // r.run(OHKOSegment::new(Move::Spark)); // Haunter
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2));
  // r.save("crystal_after_morty_90");

  // r.load("crystal_after_morty_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(4)); // Morty text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(3)); // Morty text
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(4, 4).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new());
  // r.run(SkipScriptSegment::new()); // fall
  // r.run(WalkToSegment::new(4, 17));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(0, 19));
  // r.run(WarpSegment::new().with_input(L)); // enter gatehouse
  // r.run(WalkToSegment::new(1, 5));
  // r.run(WalkToSegment::new(0, 5));
  // r.run(WarpSegment::new().with_input(L)); // enter Route 38
  // {
  //   r.run(WalkToSegment::new(26, 6)); // work around consistent spinner
  //   r.run(WalkToSegment::new(25, 6)); // work around consistent spinner
  //   for _ in 0..9 {
  //     r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  //   }
  // }
  // r.run(WalkToSegment::new(15, 5));
  // r.run(WalkToSegment::new(-1, 8).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Route 39
  // r.run(WalkToSegment::new(11, 18));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Pokefan Ruth
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Pikachu
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_pokefan_ruth_90");

  // r.load("crystal_after_pokefan_ruth_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(12, 22));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(8, 36).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Olivine
  // r.run(WalkToSegment::new(13, 12).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Rival script
  // r.run(SkipTextsSegment::new(16)); // Rival text
  // r.run(SkipScriptSegment::new()); // Rival script
  // r.run(WalkToSegment::new(29, 27).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter light house
  // r.run(WalkToSegment::new(3, 11).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 2f
  // r.run(WalkToSegment::new(17, 10));
  // r.run(WalkToSegment::new(17, 9));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Gentleman Alfred
  // r.run(OHKOSegment::new(Move::QuickAttack).crit()); // Noctowl
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_gentleman_alfred_90");

  // r.load("crystal_after_gentleman_alfred_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(5, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 3f // 168539
  // r.run(WalkToSegment::new(14, 5).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Gentleman Preston
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Gentleman Preston
  // r.run(OHKOSegment::new(Move::Spark)); // Growlithe
  // r.run(NextTrainerMonSegment::new(Pokemon::Growlithe, 18));
  // r.run(OHKOSegment::new(Move::Spark)); // Growlithe
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_gentleman_preston_90");

  // r.load("crystal_after_gentleman_preston_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(13, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 4f
  // r.run(WalkToSegment::new(13, 2));
  // r.run(WalkToSegment::new(12, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(SkipScriptSegment::new()); // Lass Connie
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Lass Connie
  // r.run(OHKOSegment::new(Move::QuickAttack).crit()); // Marill
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_lass_connie_90");

  // r.load("crystal_after_lass_connie_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(10, 2));
  // r.run(WalkToSegment::new(8, 2));
  // r.run(WalkToSegment::new(8, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new());
  // r.run(SkipScriptSegment::new()); // fall
  // r.run(WalkToSegment::new(9, 4));
  // r.run(WalkToSegment::new(9, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 4f
  // r.run(WalkToSegment::new(9, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 5f
  // r.run(WalkToSegment::new(9, 15).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 6f
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(8, 10));
  // r.run(WalkToSegment::new(8, 9));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(10)); // Jasmine text
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(VerifyInputSegment::new("Pack")); // InitGFX
  //   r.run(VerifyInputSegment::new("Pack")); // InitItemsPocket
  //   r.run(VerifyInputSegment::new("Pack")); // ItemsPocketMenu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Use
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(SkipTextsSegment::new(1)); // Used Escape Rope
  //   r.run(SkipScriptSegment::new()); // Escape Rope MapScript
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::ForcedMovement)));
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(7, 21).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Strength house
  // r.run(WalkToSegment::new(4, 5));
  // r.run(WalkToSegment::new(4, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(SkipTextsSegment::new(6)); // Get Strength
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got HM04
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put HM04 in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(3)); // Get Strength
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(3, 6));
  // r.run(WalkToSegment::new(3, 7));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(-1, 24).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Route 40
  // r.run(WalkToSegment::new(10, 13));
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
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
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A));
  //   r.run(SkipTextsSegment::new(1)); // Booted up HM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // contains // move // .
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // teach // move // to mon?
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Poliwag
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // Poliwag learned Surf
  //   r.run(VerifyInputSegment::new("Pack")); // TMHMPocketMenu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A));
  //   r.run(SkipTextsSegment::new(1)); // Booted up HM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // contains // move // .
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // teach // move // to mon?
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // B
  //   r.run(OverrideMoveSegment::new(2)); // override Roar with Strength
  //   r.run(VerifyInputSegment::new("Pack")); // TMHMPocketMenu
  //   r.run(MoveSegment::new(B));
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Poliwag
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Surf
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // }
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(WalkToSegment::new(2, 36).surfing().into(OverworldInteractionResult::MapConnection)); // 178509 // 178509
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Route 41
  // r.run(WalkToSegment::new(-1, 5).surfing().into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Cianwood
  // r.run(WalkToSegment::new(23, 32).surfing());
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(8, 43).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Gym
  // r.run(WalkToSegment::new(4, 12).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Blackbelt Yoshi
  // r.save("crystal_before_blackbelt_yoshi_90");

  // r.load("crystal_before_blackbelt_yoshi_90");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Blackbelt Yoshi
  // r.run(OHKOSegment::new(Move::Strength)); // Hitmonlee
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Blackbelt Lao
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Hitmonchan
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_level_up());
  // r.save("crystal_after_blackbelt_lao_90");

  // r.load("crystal_after_blackbelt_lao_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(4, 9).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Blackbelt Nob
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Blackbelt Nob
  // r.run(OHKOSegment::new(Move::Spark)); // Machop
  // r.run(NextTrainerMonSegment::new(Pokemon::Machoke, 25));
  // r.run(OHKOSegment::new(Move::Spark)); // Machoke
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_blackbelt_nob_90");

  // r.load("crystal_after_blackbelt_nob_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(5, 9));
  // r.run(WalkToSegment::new(5, 8));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(1)); // Use Strength
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Use Strength
  // r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // Use Strength
  // r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(3, 8));
  // r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(3, 7));
  // r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(4, 5).into(OverworldInteractionResult::SeenByTrainer)); // 187874
  // r.run(SkipScriptSegment::new()); // Blackbelt Lung
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Blackbelt Lung
  // r.run(OHKOSegment::new(Move::Spark)); // Mankey
  // r.run(NextTrainerMonSegment::new(Pokemon::Mankey, 23));
  // r.run(OHKOSegment::new(Move::Spark)); // Mankey
  // r.run(NextTrainerMonSegment::new(Pokemon::Primeape, 25));
  // r.run(OHKOSegment::new(Move::Spark)); // Primeape
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_blackbelt_lung_90");

  // r.load("crystal_after_blackbelt_lung_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(4, 2));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(8)); // Chuck
  // r.run(SkipScriptSegment::new()); // Chuck
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(4)); // Chuck
  // r.run(OHKOSegment::new(Move::Spark)); // Primeape
  // r.run(NextTrainerMonSegment::new(Pokemon::Poliwrath, 30));
  // r.run(OHKOSegment::new(Move::Spark)); // Poliwrath
  // r.run(EndTrainerBattleSegment::with_defeat_texts(3));
  // r.save("crystal_after_chuck_90");

  // r.load("crystal_after_chuck_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(4)); // Chuck text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(3)); // Chuck text
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(4, 17));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(8, 46));
  // r.run(WalkToSegment::new(9, 46));
  // r.run(WalkToSegment::new(10, 46));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(6)); // Get Fly
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got HM02
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put HM02 in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(7)); // Get Fly
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(15, 47).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter House
  // r.run(WalkToSegment::new(2, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(7)); // Get Medicine
  // r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // got Secretpotion
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put in Key pocket
  // r.run(SkipTextsSegment::new(2)); // Get Medicine
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(2, 7));
  // r.run(WarpSegment::new().with_input(D)); // leave
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
  //   r.run(MoveSegment::new(A));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A));
  //   r.run(SkipTextsSegment::new(1)); // Booted up HM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // contains // move // .
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // teach // move // to mon?
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // Kenya learned Fly
  //   r.run(VerifyInputSegment::new("Pack")); // TMHMPocketMenu
  //   r.run(MoveSegment::new(B));
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  // }
  // r.run(MoveSegment::new(D));
  // r.run(MoveSegment::new(D|L));
  // r.run(MoveSegment::new(D|R));
  // r.run(MoveSegment::new(A));
  // r.run(SkipScriptSegment::new()); // Fly
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(35, 27));
  // r.run(WarpSegment::new().with_input(R)); // enter gate house
  // r.run(WalkToSegment::new(9, 5));
  // r.run(WarpSegment::new().with_input(R)); // enter Route 42
  // { // Mt. Mortar Route
  //   r.run(WalkToSegment::new(10, 5).into(OverworldInteractionResult::Warped));
  //   r.run(WarpSegment::new()); // enter Mt. Mortar
  //   r.run(WalkToSegment::new(11, 21).into(OverworldInteractionResult::Warped));
  //   r.run(WarpSegment::new()); // enter Mt. Mortar b2f
  //   r.run(WalkToSegment::new(29, 46));
  //   r.run(WalkToSegment::new(29, 47));
  //   r.run(WarpSegment::new().with_input(D)); // leave
  //   r.run(WalkToSegment::new(37, 32));
  //   r.run(WalkToSegment::new(37, 33));
  //   r.run(WarpSegment::new().with_input(D)); // leave
  // }
  // // { // Surf route +196f
  // //   r.run(WalkToSegment::new(13, 9));
  // //   r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // //   r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // //   r.run(SkipScriptSegment::new()); // Surf MapScript
  // //   r.run(WalkToSegment::new(20, 10).surfing());
  // //   r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // //   r.run(WalkToSegment::new(32, 9));
  // //   r.run(WalkToSegment::new(33, 9));
  // //   r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // //   r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // //   r.run(SkipScriptSegment::new()); // Surf MapScript
  // //   r.run(WalkToSegment::new(42, 9).surfing());
  // //   r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // // }
  // r.run(WalkToSegment::new(60, 7).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Mahogany // 199838 // 200034
  // r.run(WalkToSegment::new(9, 1).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gate house
  // r.run(WalkToSegment::new(4, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Route 43
  // r.run(WalkToSegment::new(8, 0));
  // r.run(WalkToSegment::new(8, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Lake of Rage
  // r.run(WalkToSegment::new(18, 28));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(WalkToSegment::new(18, 23).surfing());
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(1)); // Gyarados
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // wild // mon // appeared
  // r.run(TextSegment::new().with_skip_ends(2)); // Go // mon // !
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Gyarados
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // gained XP
  // r.run(SkipScriptSegment::new()); // Gyarados MapScript
  // r.save("crystal_after_gyarados_90");

  // r.load("crystal_after_gyarados_90");
  // r.run(TextSegment::new().with_skip_ends(2)); // got Red Scale
  // r.run(MoveSegment::new(NIL)); // got Red Scale
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // got Red Scale
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put in Item pocket
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(18, 28).surfing());
  // r.run(WalkToSegment::new(20, 28));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(15)); // Lance
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // agree to help
  // r.run(SkipTextsSegment::new(6)); // Lance
  // r.run(SkipScriptSegment::new()); // Lance MapScript
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  // }
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(MoveSegment::new(D));
  // r.run(MoveSegment::new(D|L));
  // r.run(MoveSegment::new(A)); // Mahogany
  // r.run(SkipScriptSegment::new()); // Fly
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(11, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Hideout
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(TextSegment::new()); // Lance
  // r.run(SkipScriptSegment::new()); // casual Hyperbeaming
  // r.run(SkipTextsSegment::new(3)); // Lance
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(SkipTextsSegment::new(1)); // Lance
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(SkipTextsSegment::new(2)); // Lance
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(WalkToSegment::new(7, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Hideout b1f
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(24, 2).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Rocket alarm script
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Rocket
  // r.run(OHKOSegment::new(Move::Spark)); // Drowzee
  // r.run(NextTrainerMonSegment::new(Pokemon::Zubat, 19).with_level_up());
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Zubat
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_hideout_rocket1_90");

  // r.load("crystal_after_hideout_rocket1_90");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Rocket
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Zubat
  // r.run(NextTrainerMonSegment::new(Pokemon::Grimer, 17));
  // r.run(OHKOSegment::new(Move::Spark)); // Grimer
  // r.run(NextTrainerMonSegment::new(Pokemon::Rattata, 18));
  // r.run(OHKOSegment::new(Move::Spark)); // Grimer
  // r.run(EndTrainerBattleSegment::with_defeat_texts(4));
  // r.save("crystal_after_hideout_rocket2_90");

  // r.load("crystal_after_hideout_rocket2_90");
  // r.run(WalkToSegment::new(17, 12).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Scientist Jed
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Scientist Jed
  // r.run(OHKOSegment::new(Move::Spark)); // Magnemite
  // r.run(NextTrainerMonSegment::new(Pokemon::Magnemite, 20));
  // r.run(OHKOSegment::new(Move::Strength)); // Magnemite
  // r.run(NextTrainerMonSegment::new(Pokemon::Magnemite, 20));
  // r.run(OHKOSegment::new(Move::Strength)); // Magnemite
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_scientist_jed_90");

  // r.load("crystal_after_scientist_jed_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(19, 12));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::BgRead))));
  // r.run(SkipTextsSegment::new(1)); // press switch
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(24, 7).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // deactivated alarm script
  // r.run(WalkToSegment::new(22, 16).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // deactivated alarm script
  // r.run(WalkToSegment::new(8, 16).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // deactivated alarm script
  // r.run(WalkToSegment::new(4, 16));
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(WalkToSegment::new(3, 15));
  // r.run(WalkToSegment::new(3, 14).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Hideout b2f
  // r.run(WalkToSegment::new(4, 13).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(SkipTextsSegment::new(6)); // Lance
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(WalkToSegment::new(21, 13));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Rocket
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Venonat
  // r.run(NextTrainerMonSegment::new(Pokemon::Venonat, 18));
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Venonat
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_hideout_rocket3_90");

  // r.load("crystal_after_hideout_rocket3_90");
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(27, 14).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Hideout b3f
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(SkipTextsSegment::new(7)); // Lance
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(WalkToSegment::new(19, 7));
  // r.run(WalkToSegment::new(20, 7));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Rocket
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Venonat
  // r.run(NextTrainerMonSegment::new(Pokemon::Gloom, 18));
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Gloom
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_hideout_rocket4_90");

  // r.load("crystal_after_hideout_rocket4_90");
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(SkipTextsSegment::new(4)); // Get Password
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(6, 14));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.expect(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(6)); // Rocket
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Raticate
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_hideout_rocket5_90");

  // r.load("crystal_after_hideout_rocket5_90");
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(SkipTextsSegment::new(3)); // Get Password
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(27, 3));
  // r.run(WalkToSegment::new(27, 2).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Hideout b2f // 227229
  // r.run(WalkToSegment::new(3, 2).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Hideout b3f
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(8, 10).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Rival script
  // r.run(SkipTextsSegment::new(15)); // Rival
  // r.run(SkipScriptSegment::new()); // Rival script
  // r.run(WalkToSegment::new(10, 10));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::BgThenRead))));
  // r.run(SkipTextsSegment::new(3)); // Open door
  // r.run(WalkToSegment::new(10, 8).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Giovanni
  // r.run(SkipTextsSegment::new(9)); // Giovanni
  // r.run(SkipScriptSegment::new()); // Giovanni
  // r.run(StartTrainerBattleSegment::new()); // Giovanni
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Zubat
  // r.run(NextTrainerMonSegment::new(Pokemon::Raticate, 24));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Raticate
  // r.run(NextTrainerMonSegment::new(Pokemon::Koffing, 22));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2));
  // r.save("crystal_after_hideout_giovanni_90");

  // r.load("crystal_after_hideout_giovanni_90");
  // r.run(SkipTextsSegment::new(2)); // Giovanni
  // r.run(SkipScriptSegment::new()); // Giovanni
  // r.run(WalkToSegment::new(7, 4));
  // r.run(WalkToSegment::new(7, 3));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(2)); // Murkrow
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(3, 2).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Hideout b2f
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(27, 1));
  // r.run(WalkToSegment::new(27, 2).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Hideout b3f
  // r.run(WalkToSegment::new(28, 8));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectItemball))));
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player received // item // .
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // potion // in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // item pocket // .
  // r.run(WalkToSegment::new(27, 14).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Hideout b2f
  // r.run(WalkToSegment::new(15, 14));
  // r.run(WalkToSegment::new(15, 13));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::BgThenRead))));
  // r.run(SkipTextsSegment::new(3)); // Open door
  // r.run(WalkToSegment::new(15, 11).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Rocket ambush
  // r.run(SkipTextsSegment::new(1)); // Rocket ambush
  // r.run(SkipScriptSegment::new()); // Rocket ambush
  // r.run(SkipTextsSegment::new(8)); // Rocket ambush
  // r.run(SkipScriptSegment::new()); // Rocket ambush
  // r.run(SkipTextsSegment::new(2)); // Rocket ambush
  // r.run(SkipScriptSegment::new()); // Rocket ambush
  // r.run(SkipTextsSegment::new(5)); // Rocket ambush
  // r.run(SkipScriptSegment::new()); // Rocket ambush
  // r.run(StartTrainerBattleSegment::new()); // Executive
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Arbok
  // r.run(NextTrainerMonSegment::new(Pokemon::Gloom, 23));
  // r.run(OHKOSegment::new(Move::Strength)); // Gloom
  // r.run(NextTrainerMonSegment::new(Pokemon::Murkrow, 25));
  // r.run(OHKOSegment::new(Move::Strength)); // Murkrow
  // r.run(EndTrainerBattleSegment::with_defeat_texts(4).with_level_up());
  // r.save("crystal_after_hideout_rocket6_90");

  // r.load("crystal_after_hideout_rocket6_90");
  // r.run(SkipTextsSegment::new(11)); // Rocket ambush
  // r.run(SkipTextsSegment::new(4)); // Lance
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(SkipTextsSegment::new(4)); // Lance
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(SkipTextsSegment::new(10)); // Lance
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(WalkToSegment::new(8, 5));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))).with_buffer_size(1)); // 242545
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // wild // mon // appeared
  // r.run(TextSegment::new().with_skip_ends(2)); // Go // mon // !
  // r.run(OHKOSegment::new(Move::Strength)); // Electrode
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // gained XP
  // r.run(SkipScriptSegment::new()); // Electrode MapScript
  // r.save("crystal_after_electrode1_90");

  // r.load("crystal_after_electrode1_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(8, 7));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))).with_buffer_size(1)); // 242545
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // wild // mon // appeared
  // r.run(TextSegment::new().with_skip_ends(2)); // Go // mon // !
  // r.run(OHKOSegment::new(Move::Strength)); // Electrode
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // gained XP
  // r.run(SkipScriptSegment::new()); // Electrode MapScript
  // r.save("crystal_after_electrode2_90");

  // r.load("crystal_after_electrode2_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(8, 9));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))).with_buffer_size(1)); // 242545
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // wild // mon // appeared
  // r.run(TextSegment::new().with_skip_ends(2)); // Go // mon // !
  // r.run(OHKOSegment::new(Move::Strength)); // Electrode
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // gained XP
  // r.run(SkipScriptSegment::new()); // Electrode MapScript
  // r.save("crystal_after_electrode3_90");

  // r.load("crystal_after_electrode3_90");
  // r.run(SkipTextsSegment::new(8)); // Lance
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player received // HM06 // .
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // HM06 // in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // TM pocket // .
  // r.run(SkipTextsSegment::new(15)); // Lance
  // r.run(SkipScriptSegment::new()); // Lance
  // // { // Walking
  // //   r.run(WalkToSegment::new(3, 13));
  // //   r.run(WalkToSegment::new(3, 14).into(OverworldInteractionResult::Warped));
  // //   r.run(WarpSegment::new()); // enter Hideout b1f
  // //   r.run(WalkToSegment::new(5, 15).into(OverworldInteractionResult::Warped));
  // //   r.run(WarpSegment::new()); // warp
  // //   r.run(WalkToSegment::new(27, 2).into(OverworldInteractionResult::Warped));
  // //   r.run(WarpSegment::new()); // enter Hideout 1f
  // //   r.run(TurnSegment::new(U));
  // //   r.run(WalkToSegment::new(4, 6));
  // //   r.run(WalkToSegment::new(4, 7));
  // //   r.run(WarpSegment::new().with_input(D)); // leave
  // // }
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(VerifyInputSegment::new("Pack")); // InitGFX
  //   r.run(VerifyInputSegment::new("Pack")); // InitItemsPocket
  //   r.run(VerifyInputSegment::new("Pack")); // ItemsPocketMenu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Use
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(SkipTextsSegment::new(1)); // Used Escape Rope
  //   r.run(SkipScriptSegment::new()); // Escape Rope MapScript
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::ForcedMovement)));
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu))); // 248811
  // r.run(WalkToSegment::new(9, 14).into(OverworldInteractionResult::CountStepEvent));
  // r.run(SkipScriptSegment::new()); // Bike Shop
  // r.run(SkipTextsSegment::new(6)); // Bike Shop
  // r.run(TextSegment::new().with_skip_ends(6)); // Bike Shop
  // r.run_debug(WalkToSegment::new(6, 13).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Gym
  // r.run(WalkToSegment::new(2, 14));
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(U, WalkType::Walk))));
  // for _ in 0..3 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(U, WalkType::Ice)))); }
  // r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(U, WalkType::Walk))));
  // for _ in 0..4 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(U, WalkType::Ice)))); }
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // for _ in 0..4 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Walked(R, WalkType::Ice)))); }
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(D, WalkType::Walk))));
  // for _ in 0..7 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(D, WalkType::Ice)))); }
  // r.run(MoveSegment::with_metric(L, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(L, WalkType::Walk))));
  // for _ in 0..2 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(L, WalkType::Ice)))); }
  // r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(U, WalkType::Walk))));
  // for _ in 0..8 { r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(U, WalkType::Ice)))); }
  // r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Walked(R, WalkType::Walk))));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(R, WalkType::Ice))));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(11)); // Pryce
  // r.run(OHKOSegment::new(Move::Strength)); // Seel
  // r.run(NextTrainerMonSegment::new(Pokemon::Piloswine, 31));
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Piloswine
  // r.run(NextTrainerMonSegment::new(Pokemon::Dewgong, 29));
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Dewgong
  // r.run(EndTrainerBattleSegment::with_defeat_texts(4));
  // r.save("crystal_after_pryce_90");

  // r.load("crystal_after_pryce_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(5)); // Pryce text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(4)); // Pryce text
  // r.run(TurnSegment::new(R));
  // r.run(MoveSegment::with_metric(L, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(L, WalkType::Walk))));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(L, WalkType::Ice))));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(D, WalkType::Walk))));
  // for _ in 0..8 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(D, WalkType::Ice)))); }
  // r.run(MoveSegment::with_metric(L, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(L, WalkType::Walk))));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Walked(D, WalkType::Ice))));
  // r.run(WalkToSegment::new(4, 16));
  // r.run(WalkToSegment::new(4, 17));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  // }
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(MoveSegment::new(D));
  // r.run(MoveSegment::new(D|L));
  // r.run(MoveSegment::new(D|R));
  // r.run(MoveSegment::new(D|L));
  // r.run(MoveSegment::new(A)); // Olivine
  // r.run(SkipScriptSegment::new()); // Fly
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(29, 27).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter light house
  // r.run(WalkToSegment::new(3, 11).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 2f
  // r.run(WalkToSegment::new(5, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 3f
  // r.run(WalkToSegment::new(13, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 4f
  // r.run(WalkToSegment::new(10, 2));
  // r.run(WalkToSegment::new(8, 2));
  // r.run(WalkToSegment::new(8, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new());
  // r.run(SkipScriptSegment::new()); // fall
  // r.run(WalkToSegment::new(9, 4));
  // r.run(WalkToSegment::new(9, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 4f
  // r.run(WalkToSegment::new(9, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 5f
  // r.run(WalkToSegment::new(9, 15).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 6f
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(8, 10));
  // r.run(WalkToSegment::new(8, 9));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(SkipTextsSegment::new(1)); // Jasmine text
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Jasmine text
  // r.run(SkipTextsSegment::new(13)); // Jasmine text
  // r.run(SkipScriptSegment::new()); // Jasmine MapScript
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(VerifyInputSegment::new("Pack")); // InitGFX
  //   r.run(VerifyInputSegment::new("Pack")); // InitItemsPocket
  //   r.run(VerifyInputSegment::new("Pack")); // ItemsPocketMenu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Use
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(SkipTextsSegment::new(1)); // Used Escape Rope
  //   r.run(SkipScriptSegment::new()); // Escape Rope MapScript
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::ForcedMovement)));
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(10, 11).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.run(WalkToSegment::new(5, 5));
  // r.run(WalkToSegment::new(5, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.save("crystal_before_jasmine_90");

  // r.load("crystal_before_jasmine_90");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(10)); // Jasmine
  // r.run(OHKOSegment::new(Move::Spark)); // Magnemite
  // r.run(NextTrainerMonSegment::new(Pokemon::Magnemite, 30));
  // r.run(OHKOSegment::new(Move::Spark)); // Magnemite
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(NextTrainerMonSegment::new(Pokemon::Steelix, 35).with_expected_move(Move::IronTail));
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // r.run(MoveSegment::new(D)); // pack
  // r.run(MoveSegment::new(A)); // select pack
  // r.run(VerifyInputSegment::new("BattlePack")); // InitGFX
  // r.run(VerifyInputSegment::new("BattlePack")); // InitItemsPocket
  // r.run(VerifyInputSegment::new("BattlePack")); // ItemsPocketMenu
  // r.run(MoveSegment::new(D));
  // r.run(MoveSegment::new(NIL));
  // r.run(MoveSegment::new(D));
  // r.run(MoveSegment::new(A)); // X Attack
  // r.run(MoveSegment::new(NIL));
  // r.run(MoveSegment::new(A)); // Use
  // r.run(MoveSegment::new(B)); // close
  // r.run(
  //   DelaySegment::new(MoveSegment::new(A)
  //   .seq(VerifyInputSegment::new("BattlePack"))
  //   .seq(MoveSegment::with_metric(NIL, BattleObedienceMetric {}.expect(BattleObedience::Obey))) // confirm
  //   .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  // );
  // r.run(TextSegment::new().with_allowed_end_inputs(A).with_unbounded_buffer()); // but it failed!
  // r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::IronTail)))); // confirm
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // // Steelix Turn 2-4
  // r.run(MoveSegment::new(U)); // back to Fight
  // r.run(KOSegment::new(Move::Strength, EnemyAttack { mov: Move::IronTail, attack_type: EnemyAttackType::Failed } ));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(4));
  // r.save("crystal_after_jasmine_90");

  // r.load("crystal_after_jasmine_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(2)); // Jasmine text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(2)); // Jasmine text
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 15));
  // r.run(WarpSegment::new().with_input(D));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::CountStepEvent)));
  // r.run(SkipTextsSegment::new(9)); // Elm phone call
  // r.run(TextSegment::new().with_skip_ends(6)); // Click // ... // ... // ...
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(U|L));
  //   r.run(MoveSegment::new(U|R));
  //   r.run(MoveSegment::new(U|L));
  //   r.run(MoveSegment::new(A)); // Goldenrod
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(5, 15).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter tower
  // r.run(WalkToSegment::new(12, 1));
  // r.run(WalkToSegment::new(13, 1));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(5)); // Rocket
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Raticate
  // r.run(NextTrainerMonSegment::new(Pokemon::Raticate, 24).with_level_up());
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Raticate
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket1_90");

  // r.load("crystal_after_radio_tower_rocket1_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(15, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 2f
  // r.run(WalkToSegment::new(10, 2).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(4)); // Rocket
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Arbok
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket2_90");

  // r.load("crystal_after_radio_tower_rocket2_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(6, 1));
  // r.run(WalkToSegment::new(5, 1));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Rocket
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Zubat
  // r.run(NextTrainerMonSegment::new(Pokemon::Zubat, 26));
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Zubat
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket3_90");

  // r.load("crystal_after_radio_tower_rocket3_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(3, 4));
  // r.run(WalkToSegment::new(2, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Rocket
  // r.run(OHKOSegment::new(Move::Spark)); // Grimer
  // r.run(NextTrainerMonSegment::new(Pokemon::Grimer, 23));
  // r.run(OHKOSegment::new(Move::Spark)); // Grimer
  // r.run(NextTrainerMonSegment::new(Pokemon::Muk, 25));
  // r.run(OHKOSegment::new(Move::Spark)); // Muk
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket4_90");

  // r.load("crystal_after_radio_tower_rocket4_90");
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(0, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 3f
  // r.run(WalkToSegment::new(4, 1));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Rocket
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(NextTrainerMonSegment::new(Pokemon::Grimer, 23));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Grimer
  // r.run(NextTrainerMonSegment::new(Pokemon::Rattata, 23));
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Rattata
  // r.run(NextTrainerMonSegment::new(Pokemon::Zubat, 23));
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Zubat
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket5_90");

  // r.load("crystal_after_radio_tower_rocket5_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(8, 6));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Scientist Marc
  // r.run(OHKOSegment::new(Move::Spark)); // Magnemite
  // r.run(NextTrainerMonSegment::new(Pokemon::Magnemite, 27));
  // r.run(OHKOSegment::new(Move::Spark)); // Magnemite
  // r.run(NextTrainerMonSegment::new(Pokemon::Magnemite, 27));
  // r.run(OHKOSegment::new(Move::Spark)); // Magnemite
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_scientist_marc_90");

  // r.load("crystal_after_scientist_marc_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(7, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 4f
  // r.run(WalkToSegment::new(6, 4));
  // for _ in 0..9 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::NoEvents))); }
  // r.run(WalkToSegment::new(4, 3));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Scientist Rich
  // r.run(OHKOSegment::new(Move::Spark)); // Porygon
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2).with_level_up());
  // r.save("crystal_after_scientist_rich_90");

  // r.load("crystal_after_scientist_rich_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(0, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 5f
  // r.run(WalkToSegment::new(0, 3).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Executive
  // r.run(SkipTextsSegment::new(1)); // Executive
  // r.run(SkipScriptSegment::new()); // Executive
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(8)); // Executive
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(NextTrainerMonSegment::new(Pokemon::Koffing, 30));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(NextTrainerMonSegment::new(Pokemon::Koffing, 30));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(NextTrainerMonSegment::new(Pokemon::Weezing, 32));
  // r.run(OHKOSegment::new(Move::Spark)); // Weezing
  // r.run(NextTrainerMonSegment::new(Pokemon::Koffing, 30));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(NextTrainerMonSegment::new(Pokemon::Koffing, 30));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket6_90");

  // r.load("crystal_after_radio_tower_rocket6_90");
  // r.run(SkipTextsSegment::new(5)); // Executive
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player received // Basement Key // .
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // Basement Key // in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // key pocket // .
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(0, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 4f
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(WalkToSegment::new(9, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 3f
  // r.run(WalkToSegment::new(0, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 2f
  // r.run(WalkToSegment::new(15, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 1f
  // r.run(WalkToSegment::new(3, 7));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(9, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter underground
  // r.run(WalkToSegment::new(21, 25).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter underground
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(2, 4));
  // r.run(WalkToSegment::new(2, 5));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Pokemaniac Donald
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Slowpoke
  // r.run(NextTrainerMonSegment::new(Pokemon::Slowpoke, 10));
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Slowpoke
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_pokemaniac_donald_90");

  // r.load("crystal_after_pokemaniac_donald_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(4, 9));
  // r.run(WalkToSegment::new(5, 9));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(4)); // Supernerd Teru
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Magnemite
  // r.run(NextTrainerMonSegment::new(Pokemon::Voltorb, 11));
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Voltorb
  // r.run(NextTrainerMonSegment::new(Pokemon::Magnemite, 7));
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Magnemite
  // r.run(NextTrainerMonSegment::new(Pokemon::Magnemite, 9));
  // r.run(OHKOSegment::new(Move::QuickAttack)); // Magnemite
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_supernerd_teru_90");

  // r.load("crystal_after_supernerd_teru_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(18, 7));
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
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A));
  //   r.run(SkipTextsSegment::new(1)); // Booted up TM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // contains // move // .
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // teach // move // to mon?
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // B
  //   r.run(OverrideMoveSegment::new(3)); // override Quick Attack with Iron Tail
  //   r.run(VerifyInputSegment::new("Pack")); // TMHMPocketMenu
  //   r.run(MoveSegment::new(B));
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(MoveSegment::new(B)); // close main menu
  // }
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::BgRead))));
  // r.run(SkipTextsSegment::new(1)); // Open Door
  // r.run(WalkToSegment::new(18, 6).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter basement
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(WalkToSegment::new(22, 27).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter basement
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(19, 4).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Rival
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(8)); // Rival
  // r.run(OHKOSegment::new(Move::IronTail)); // Golbat
  // r.run(NextTrainerMonSegment::new(Pokemon::Magnemite, 28));
  // r.run(OHKOSegment::new(Move::Spark)); // Magnemite
  // r.run(NextTrainerMonSegment::new(Pokemon::Haunter, 30));
  // r.run(OHKOSegment::new(Move::Spark)); // Haunter
  // r.run(NextTrainerMonSegment::new(Pokemon::Sneasel, 32).with_level_up());
  // r.run(OHKOSegment::new(Move::Spark)); // Sneasel
  // r.run(NextTrainerMonSegment::new(Pokemon::Feraligatr, 32));
  // r.run(OHKOSegment::new(Move::Spark)); // Feraligatr
  // r.run(EndTrainerBattleSegment::with_defeat_texts(4));
  // r.save("crystal_after_rival4_90");

  // r.load("crystal_after_rival4_90");
  // r.run(SkipTextsSegment::new(12)); // Rival
  // r.run(SkipScriptSegment::new()); // Rival
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(17, 4).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Rocket
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Rattata
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket7_90");

  // r.load("crystal_after_radio_tower_rocket7_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(11, 4).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Rocket
  // r.run(OHKOSegment::new(Move::Spark)); // Muk
  // r.run(NextTrainerMonSegment::new(Pokemon::Koffing, 23));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(NextTrainerMonSegment::new(Pokemon::Rattata, 25));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Rattata
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket8_90");

  // r.load("crystal_after_radio_tower_rocket8_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(5, 2));
  // r.run(WalkToSegment::new(4, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(4)); // Rocket
  // r.run(OHKOSegment::new(Move::Spark)); // Koffing
  // r.run(NextTrainerMonSegment::new(Pokemon::Muk, 24));
  // r.run(OHKOSegment::new(Move::Spark)); // Muk
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket9_90");

  // r.load("crystal_after_radio_tower_rocket9_90");
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(2, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::BgRead))));
  // r.run(SkipTextsSegment::new(1)); // Press switch
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Press switch
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(10, 3));
  // r.run(WalkToSegment::new(10, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::BgRead))));
  // r.run(SkipTextsSegment::new(1)); // Press switch
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Press switch
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(16, 3));
  // r.run(WalkToSegment::new(16, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::BgRead))));
  // r.run(SkipTextsSegment::new(1)); // Press switch
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Press switch
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(3, 8).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Burglar Eddie
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Burglar Eddie
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Growlithe
  // r.run(NextTrainerMonSegment::new(Pokemon::Koffing, 24));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_burglar_eddie_90");

  // r.load("crystal_after_burglar_eddie_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(10, 12).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Burglar Duncan
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Burglar Duncan
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(NextTrainerMonSegment::new(Pokemon::Magmar, 25));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Magmar
  // r.run(NextTrainerMonSegment::new(Pokemon::Koffing, 23).with_level_up());
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_burglar_duncan_90");

  // r.load("crystal_after_burglar_duncan_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(18, 12));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(4)); // Rocket
  // r.run(OHKOSegment::new(Move::IronTail)); // Gloom
  // r.run(NextTrainerMonSegment::new(Pokemon::Gloom, 25));
  // r.run(OHKOSegment::new(Move::IronTail)); // Gloom
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket10_90");

  // r.load("crystal_after_radio_tower_rocket10_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(22, 10).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter basement
  // r.run(WalkToSegment::new(8, 12).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Rocket
  // r.run(OHKOSegment::new(Move::Spark)); // Koffing
  // r.run(NextTrainerMonSegment::new(Pokemon::Golbat, 24));
  // r.run(OHKOSegment::new(Move::IronTail)); // Golbat
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket11_90");

  // r.load("crystal_after_radio_tower_rocket11_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(16, 3).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Rocket
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Grimer
  // r.run(NextTrainerMonSegment::new(Pokemon::Weezing, 23));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Weezing
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket12_90");

  // r.load("crystal_after_radio_tower_rocket12_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(9, 5).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Rocket
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(NextTrainerMonSegment::new(Pokemon::Koffing, 25));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket13_90");

  // r.load("crystal_after_radio_tower_rocket13_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(13, 8));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(7)); // Director
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player received // Card Key // .
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // Card Key // in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // key pocket // .
  // r.run(SkipTextsSegment::new(10)); // Director
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(VerifyInputSegment::new("Pack")); // InitGFX
  //   r.run(VerifyInputSegment::new("Pack")); // InitItemsPocket
  //   r.run(VerifyInputSegment::new("Pack")); // ItemsPocketMenu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Use
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(SkipTextsSegment::new(1)); // Used Escape Rope
  //   r.run(SkipScriptSegment::new()); // Escape Rope MapScript
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::ForcedMovement)));
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(5, 15).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter tower
  // r.run(WalkToSegment::new(15, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 2f
  // r.run(WalkToSegment::new(0, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 3f
  // r.run(WalkToSegment::new(14, 4));
  // r.run(WalkToSegment::new(14, 3));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::BgRead))));
  // r.run(SkipTextsSegment::new(2)); // Use card key
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(16, 3).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Rocket
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Rocket
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Raticate
  // r.run(NextTrainerMonSegment::new(Pokemon::Koffing, 26));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket14_90");

  // r.load("crystal_after_radio_tower_rocket14_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(17, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 4f
  // r.run(WalkToSegment::new(15, 1));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Executive
  // r.run(OHKOSegment::new(Move::IronTail).crit()); // Golbat
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket15_90");

  // r.load("crystal_after_radio_tower_rocket15_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(12, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 5f
  // r.run(WalkToSegment::new(16, 2).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Executive
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Executive
  // r.run(OHKOSegment::new(Move::Spark)); // Arbok
  // r.run(NextTrainerMonSegment::new(Pokemon::Vileplume, 32).with_level_up());
  // r.run(OHKOSegment::new(Move::IronTail).crit()); // Vileplume
  // r.run(NextTrainerMonSegment::new(Pokemon::Murkrow, 32));
  // r.run(OHKOSegment::new(Move::IronTail)); // Murkrow
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2));
  // r.save("crystal_after_radio_tower_rocket16_90");

  // r.load("crystal_after_radio_tower_rocket16_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(16, 5).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Executive
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(11)); // Executive
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Houndour
  // r.run(NextTrainerMonSegment::new(Pokemon::Koffing, 33));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Koffing
  // r.run(NextTrainerMonSegment::new(Pokemon::Houndoom, 35));
  // r.run(OHKOSegment::new(Move::ThunderShock).crit()); // Houndoom
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_radio_tower_rocket17_90");

  // r.load("crystal_after_radio_tower_rocket17_90");
  // r.run(SkipTextsSegment::new(6)); // Executive
  // r.run(SkipScriptSegment::new()); // Director
  // r.run(SkipTextsSegment::new(5)); // Director
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player received // Clear Bell // .
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // Clear Bell // in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // key pocket // .
  // r.run(SkipTextsSegment::new(20)); // Director
  // r.run(SkipScriptSegment::new()); // Director
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(WalkToSegment::new(12, 1));
  // r.run(WalkStepSegment::new(U).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 4f
  // r.run(WalkToSegment::new(17, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 3f
  // r.run(WalkToSegment::new(0, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 2f
  // r.run(WalkToSegment::new(15, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter 1f
  // r.run(WalkToSegment::new(12, 3));
  // r.run(WalkToSegment::new(12, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(SkipTextsSegment::new(6)); // Get Radio Card
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Get Flute
  // r.run(SkipTextsSegment::new(2)); // Question 1
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Question 1
  // r.run(SkipTextsSegment::new(2)); // Question 2
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Question 2
  // r.run(SkipTextsSegment::new(1)); // Question 3
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Question 3
  // r.run(SkipTextsSegment::new(2)); // Question 4
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Question 4
  // r.run(SkipTextsSegment::new(3)); // Question 5
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Question 5
  // r.run(SkipTextsSegment::new(2)); // Get Radio Card
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // Get Radio Card
  // r.run(SkipTextsSegment::new(2)); // Get Radio Card
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(3, 7));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(A)); // Mahogany
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(20, 9).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 44
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(WalkToSegment::new(8, 11));
  // r.run(WalkToSegment::new(9, 11));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Wait for spinner
  // r.run(WalkToSegment::new(56, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter ice path
  // r.run(WalkToSegment::new(14, 17).into(OverworldInteractionResult::Walked(R, WalkType::Ice)));
  // for _ in 0..1 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Walked(R, WalkType::Ice)))); }
  // bump_turn_slide(r, D, 4);
  // bump_turn_slide(r, R, 4);
  // bump_turn_slide(r, D, 2);
  // bump_turn_slide(r, R, 1);
  // r.run(WalkToSegment::new(15, 13).into(OverworldInteractionResult::Walked(U, WalkType::Ice)));
  // for _ in 0..1 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Walked(U, WalkType::Ice)))); }
  // bump_turn_slide(r, L, 6);
  // bump_turn_slide(r, U, 10);
  // bump_turn_slide(r, L, 1);
  // bump_turn_slide(r, D, 9);
  // bump_turn_slide(r, L, 1);
  // bump_turn_slide(r, U, 8);
  // bump_turn_slide(r, L, 5);
  // bump_turn_slide(r, D, 2);
  // bump_turn_slide(r, R, 9);
  // bump_turn_slide(r, D, 3);
  // bump_turn_slide(r, R, 5);
  // r.run(WalkToSegment::new(22, 9).into(OverworldInteractionResult::Walked(R, WalkType::Ice)));
  // for _ in 0..1 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Walked(R, WalkType::Ice)))); }
  // bump_turn_slide(r, U, 3);
  // bump_turn_slide(r, L, 1);
  // bump_turn_slide(r, D, 1);
  // bump_turn_slide(r, R, 8);
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectItemball))));
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got HM07
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put HM07 in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(29, 7).into(OverworldInteractionResult::Walked(L, WalkType::Ice)));
  // for _ in 0..7 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Walked(L, WalkType::Ice)))); }
  // bump_turn_slide(r, U, 1);
  // bump_turn_slide(r, R, 3);
  // bump_turn_slide(r, D, 3);
  // bump_turn_slide(r, R, 3);
  // bump_turn_slide(r, U, 3);
  // bump_turn_slide(r, R, 1);
  // bump_turn_slide(r, D, 2);
  // bump_turn_slide(r, L, 6);
  // bump_turn_slide(r, D, 1);
  // bump_turn_slide(r, L, 2);
  // r.run(WalkToSegment::new(37, 6));
  // r.run(WalkToSegment::new(37, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter b2f
  // r.run(WalkToSegment::new(7, 9));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(1)); // Use Strength
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Use Strength
  // r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // Use Strength
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(8, 8));
  // r.run(MoveSegment::with_metric(L, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(9, 8));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(8, 8));
  // r.run(WalkToSegment::new(7, 8));
  // r.run(MoveSegment::with_metric(L, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(5, 9));
  // r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(5, 8));
  // r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 7));
  // r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 4));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(6, 6));
  // r.run(MoveSegment::with_metric(L, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(4, 5));
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(5, 7).into(OverworldInteractionResult::ScriptRunning(PlayerEventScript::MapScript)));
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(1));
  // r.run(WalkToSegment::new(9, 9));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(9, 10));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(R));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(9, 11));
  // r.run(WalkToSegment::new(5, 12).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter b3f
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(SkipScriptSegment::new()); // fall
  // r.run(TurnSegment::new(U));
  // for _ in 0..4 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Walked(U, WalkType::Ice)))); }
  // bump_turn_slide(r, R, 4);
  // r.run(WalkToSegment::new(9, 11).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter b4f
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(15, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter b3f
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(8, 4).into(OverworldInteractionResult::Walked(D, WalkType::Ice)));
  // for _ in 0..4 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Walked(D, WalkType::Ice)))); }
  // r.run(JumpLedgeSegment::new(D));
  // for _ in 0..4 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Walked(D, WalkType::Ice)))); }
  // bump_turn_slide(r, L, 5);
  // r.run(WalkToSegment::new(3, 15).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter b2f
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(17, 28).into(OverworldInteractionResult::Walked(D, WalkType::Ice)));
  // for _ in 0..1 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Walked(D, WalkType::Ice)))); }
  // bump_turn_slide(r, R, 2);
  // bump_turn_slide(r, D, 4);
  // bump_turn_slide(r, L, 2);
  // bump_turn_slide(r, U, 2);
  // bump_turn_slide(r, L, 4);
  // bump_turn_slide(r, U, 1);
  // bump_turn_slide(r, L, 2);
  // r.run(WalkToSegment::new(5, 25).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter b1f
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(36, 16));
  // r.run(JumpLedgeSegment::new(D));
  // for _ in 0..4 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Walked(D, WalkType::Ice)))); }
  // r.run(WalkToSegment::new(36, 27));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // r.run(WalkToSegment::new(18, 11).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.run(WalkToSegment::new(2, 14));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Cooltrainer Paul
  // r.run(OHKOSegment::new(Move::IronTail)); // Dratini
  // r.run(NextTrainerMonSegment::new(Pokemon::Dratini, 34));
  // r.run(OHKOSegment::new(Move::IronTail)); // Dratini
  // r.run(NextTrainerMonSegment::new(Pokemon::Dratini, 34));
  // r.run(OHKOSegment::new(Move::Strength)); // Dratini
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_cooltrainer_paul_90");

  // r.load("crystal_after_cooltrainer_paul_90");
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(1, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym 2f
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(3, 11).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Cooltrainer Fran
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Cooltrainer Fran
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Seadra
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_cooltrainer_fran_90");

  // r.load("crystal_after_cooltrainer_fran_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(8, 13));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(1)); // Use Strength
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Use Strength
  // r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // Use Strength
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(8, 14));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(8, 15));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // for y in (10..=17).rev() {
  //   r.run(WalkToSegment::new(6, y));
  //   r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  //   r.run(TurnSegment::new(R));
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  //   r.run(TurnSegment::new(D));
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  //   r.run(TurnSegment::new(L));
  // }
  // r.run(WalkToSegment::new(6, 9));
  // r.run(MoveSegment::with_metric(U, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(5, 7));
  // r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(6, 7));
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(5, 6).into(OverworldInteractionResult::ScriptRunning(PlayerEventScript::MapScript)));
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(1));
  // r.run(WalkToSegment::new(5, 1).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Cooltrainer Cody
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Cooltrainer Cody
  // r.run(OHKOSegment::new(Move::Strength)); // Horsea
  // r.run(NextTrainerMonSegment::new(Pokemon::Seadra, 36).with_level_up());
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Seadra
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_cooltrainer_cody_90");

  // r.load("crystal_after_cooltrainer_cody_90");
  // r.run(TurnSegment::new(R));
  // for x in 6..=8 {
  //   r.run(MoveSegment::with_metric(R, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  //   r.run(TurnSegment::new(D));
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  //   r.run(TurnSegment::new(L));
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  //   r.run(TurnSegment::new(U));
  //   r.run(WalkToSegment::new(x, 1));
  // }
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // Push boulder
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents))); // wait for boulder to move
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::ScriptRunning(PlayerEventScript::MapScript))));
  // r.run(SkipScriptSegment::new());
  // r.run(SkipTextsSegment::new(1));
  // r.run(WalkToSegment::new(8, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym 1f
  // r.run(SkipScriptSegment::new()); // fall
  // r.run(WalkToSegment::new(9, 3).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Cooltrainer Lola
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(4)); // Cooltrainer Lola
  // r.run(OHKOSegment::new(Move::IronTail)); // Dratini
  // r.run(NextTrainerMonSegment::new(Pokemon::Dragonair, 36));
  // r.run(OHKOSegment::new(Move::IronTail)); // Dragonair
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_cooltrainer_lola_90");

  // r.load("crystal_after_cooltrainer_lola_90");
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(6, 3));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(8)); // Clair
  // r.run(OHKOSegment::new(Move::IronTail)); // Dragonair
  // r.run(NextTrainerMonSegment::new(Pokemon::Dragonair, 37));
  // r.run(OHKOSegment::new(Move::IronTail)); // Dragonair
  // r.run(NextTrainerMonSegment::new(Pokemon::Dragonair, 37));
  // r.run(OHKOSegment::new(Move::IronTail)); // Dragonair
  // r.run(NextTrainerMonSegment::new(Pokemon::Kingdra, 40));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Kingdra
  // r.run(EndTrainerBattleSegment::with_defeat_texts(3));
  // r.save("crystal_after_clair_90");

  // r.load("crystal_after_clair_90");
  // r.run(SkipTextsSegment::new(13)); // Clair
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(7, 9).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym 2f
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(1, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym 1f
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(4, 16));
  // r.run(WalkToSegment::new(4, 17));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(WalkToSegment::new(22, 13));
  // r.run(WalkToSegment::new(22, 12));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(WalkToSegment::new(20, 3).surfing());
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(20, 1).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter den
  // r.run(WalkToSegment::new(3, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter den
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 15));
  // r.run(WarpSegment::new().with_input(D)); // enter den
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::SeenByTrainer)));
  // r.run(SkipScriptSegment::new()); // Cooltrainer Darin
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Cooltrainer Darin
  // r.run(OHKOSegment::new(Move::IronTail)); // Dragonair
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_cooltrainer_darin_90");

  // r.load("crystal_after_cooltrainer_darin_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(10, 6));
  // r.run(WalkToSegment::new(10, 7));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(WalkToSegment::new(10, 19).surfing());
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
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A));
  //   r.run(SkipTextsSegment::new(1)); // Booted up TM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // contains // move // .
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // teach // move // to mon?
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Poliwag
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // Poliwag learned Whirlpool
  //   r.run(VerifyInputSegment::new("Pack")); // TMHMPocketMenu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A));
  //   r.run(SkipTextsSegment::new(1)); // Booted up TM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // contains // move // .
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // teach // move // to mon?
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Poliwag
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // Poliwag learned Waterfall
  //   r.run(VerifyInputSegment::new("Pack")); // TMHMPocketMenu
  //   r.run(MoveSegment::new(B));
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Poliwag
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Whirlpool
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Whirlpool!
  // }
  // r.run(WalkToSegment::new(14, 30).surfing());
  // r.run(WalkToSegment::new(19, 29).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter den house
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(SkipScriptSegment::new()); // Den MapScript
  // r.run(SkipTextsSegment::new(8)); // Den Test
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Den Test Question 1
  // r.run(MoveSegment::new(A)); // Den Test Question 1
  // r.run(SkipTextsSegment::new(1)); // Den Test
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Den Test Question 2
  // r.run(MoveSegment::new(A)); // Den Test Question 2
  // r.run(SkipTextsSegment::new(2)); // Den Test
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Den Test Question 3
  // r.run(MoveSegment::new(D)); // Den Test Question 3
  // r.run(MoveSegment::new(A)); // Den Test Question 3
  // r.run(SkipTextsSegment::new(2)); // Den Test
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Den Test Question 4
  // r.run(MoveSegment::new(A)); // Den Test Question 4
  // r.run(SkipTextsSegment::new(2)); // Den Test
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Den Test Question 5
  // r.run(MoveSegment::new(D)); // Den Test Question 5
  // r.run(MoveSegment::new(A)); // Den Test Question 5
  // r.run(SkipTextsSegment::new(7)); // Den Test
  // r.run(SkipScriptSegment::new()); // Den MapScript
  // r.run(SkipTextsSegment::new(5)); // Clair
  // r.run(SkipScriptSegment::new()); // Den MapScript
  // r.run(SkipTextsSegment::new(1)); // Clair
  // r.run(SkipScriptSegment::new()); // Den MapScript
  // r.run(SkipTextsSegment::new(2)); // Clair
  // r.run(SkipScriptSegment::new()); // Den MapScript
  // r.run(SkipTextsSegment::new(6)); // Den Test
  // r.run(SkipScriptSegment::new()); // Den MapScript
  // r.run(SkipTextsSegment::new(1)); // Clair
  // r.run(SkipScriptSegment::new()); // Den MapScript
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(SkipTextsSegment::new(9)); // Clair
  // r.run(SkipScriptSegment::new()); // Den MapScript
  // r.run(SkipTextsSegment::new(4)); // Den Test
  // r.run(SkipScriptSegment::new()); // Den MapScript
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(5, 9));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(SkipScriptSegment::new()); // Den MapScript
  // r.run(TextSegment::new()); // Clair
  // r.run(SkipScriptSegment::new()); // Den MapScript
  // r.run(SkipTextsSegment::new(2)); // Clair
  // r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // player got TM // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(16)); // Clair
  // r.run(SkipScriptSegment::new()); // Den MapScript
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(VerifyInputSegment::new("Pack")); // InitGFX
  //   r.run(VerifyInputSegment::new("Pack")); // InitTMHMPocket
  //   r.run(VerifyInputSegment::new("Pack")); // TMHMPocketMenu
  //   r.run(MoveSegment::new(R));
  //   r.run(VerifyInputSegment::new("Pack")); // InitItemsPocket
  //   r.run(VerifyInputSegment::new("Pack")); // ItemsPocketMenu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Use
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(VerifyInputSegment::new("Pack")); // quit
  //   r.run(SkipTextsSegment::new(1)); // Used Escape Rope
  //   r.run(SkipScriptSegment::new()); // Escape Rope MapScript
  //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::ForcedMovement)));
  // }
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::CountStepEvent)));
  // r.run(SkipTextsSegment::new(4)); // Elm phone call
  // r.run(TextSegment::new().with_skip_ends(6)); // Click // ... // ... // ...
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // New Bark
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(WalkToSegment::new(16, 9));
  // r.run(WalkToSegment::new(17, 9));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(WalkToSegment::new(20, 9).surfing().into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 27
  // r.run(WalkToSegment::new(19, 9).surfing());
  // r.run(WalkToSegment::new(19, 10).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Kanto
  // r.run(SkipTextsSegment::new(5)); // Kanto
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(26, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Tohjo Falls
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(11, 15));
  // r.run(WalkToSegment::new(11, 14));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(WalkToSegment::new(11, 12).surfing());
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Waterfall
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Waterfall!
  // r.run(SkipScriptSegment::new()); // Waterfall MapScript
  // r.run(TurnSegment::new(D));
  // r.run_debug(WalkToSegment::new(18, 6).surfing().into(OverworldInteractionResult::ForcedMovement));
  // for _ in 0..4 { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::ForcedMovement))); }
  // r.run(WalkToSegment::new(22, 14).surfing());
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(25, 14));
  // r.run(WalkToSegment::new(25, 15));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(WalkToSegment::new(36, 7));
  // r.run(WalkToSegment::new(36, 6));
  // r.run(WalkToSegment::new(36, 7));
  // r.run(WalkToSegment::new(36, 6));
  // r.run(WalkToSegment::new(39, 7));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(WalkToSegment::new(49, 7).surfing());
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(61, 7));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(WalkToSegment::new(72, 11).surfing());
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(80, 11).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 26
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(WalkToSegment::new(8, 96));
  // r.run(WalkToSegment::new(9, 96));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(WalkToSegment::new(11, 95).surfing());
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(15, 57).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Heal House
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(WalkToSegment::new(2, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(5)); // heal
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(2, 7));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(5, 9));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(5)); // Cooltrainer Beth
  // r.run(OHKOSegment::new(Move::Spark)); // Rapidash
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_level_up().with_skip_learning_move());
  // r.save("crystal_after_cooltrainer_beth_90");

  // r.load("crystal_after_cooltrainer_beth_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(7, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Indigo Plateau
  // r.run(WalkToSegment::new(10, 11).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Check badges
  // r.run(SkipTextsSegment::new(4)); // Check badges
  // r.run(WalkToSegment::new(10, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Indigo Plateau
  // r.run(WalkToSegment::new(11, 55));
  // r.run(WalkToSegment::new(3, 55));
  // r.run(WalkToSegment::new(1, 49).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Indigo Plateau
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(18, 42));
  // r.run(WalkToSegment::new(13, 31).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Indigo Plateau
  // r.run(WalkToSegment::new(18, 13));
  // r.run(WalkToSegment::new(12, 8).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Rival
  // r.save("crystal_before_rival5_90");

  // r.load("crystal_before_rival5_90");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(9)); // Rival
  // r.run(OHKOSegment::new(Move::Spark)); // Sneasel
  // r.run(NextTrainerMonSegment::new(Pokemon::Magneton, 34));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Magneton
  // r.run(NextTrainerMonSegment::new(Pokemon::Haunter, 35));
  // r.run(OHKOSegment::new(Move::Spark)); // Haunter
  // r.run(NextTrainerMonSegment::new(Pokemon::Kadabra, 35));
  // r.run(OHKOSegment::new(Move::Spark)); // Kadabra
  // r.run(NextTrainerMonSegment::new(Pokemon::Feraligatr, 38));
  // r.run(OHKOSegment::new(Move::Spark)); // Feraligatr
  // r.run(NextTrainerMonSegment::new(Pokemon::Golbat, 36));
  // r.run(OHKOSegment::new(Move::IronTail).crit()); // Golbat
  // r.run(EndTrainerBattleSegment::with_defeat_texts(5));
  // r.save("crystal_after_rival5_90");

  // r.load("crystal_after_rival5_90");
  // r.run(SkipTextsSegment::new(8)); // Rival
  // r.run(SkipScriptSegment::new()); // Rival
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(13, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Indigo Plateau
  // r.run(WalkToSegment::new(10, 6));
  // r.run(WalkToSegment::new(10, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Indigo Plateau
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(WalkToSegment::new(16, 4).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // rival battle trigger
  // r.run(WalkToSegment::new(14, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Will's room
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(WalkToSegment::new(5, 8));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(9)); // Will
  // r.run(OHKOSegment::new(Move::IronTail).crit()); // Xatu
  // r.run(NextTrainerMonSegment::new(Pokemon::Jynx, 41));
  // r.run(OHKOSegment::new(Move::Strength)); // Jynx
  // r.run(NextTrainerMonSegment::new(Pokemon::Exeggutor, 41).with_level_up());
  // r.run(OHKOSegment::new(Move::IronTail).crit()); // Exeggutor
  // r.run(NextTrainerMonSegment::new(Pokemon::Xatu, 42));
  // r.run(OHKOSegment::new(Move::IronTail).crit()); // Xatu
  // r.run(NextTrainerMonSegment::new(Pokemon::Slowbro, 41));
  // r.run(OHKOSegment::new(Move::Spark)); // Slowbro
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_will_90_"); // crystal_after_will_90_

  // r.load("crystal_after_will_90_");
  // r.run(SkipTextsSegment::new(6)); // Will
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(4, 2).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Koga's room
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(WalkToSegment::new(4, 7));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(11)); // Koga
  // r.run(OHKOSegment::new(Move::Spark)); // Ariados
  // r.run(NextTrainerMonSegment::new(Pokemon::Forretress, 43));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Forretress
  // r.run(NextTrainerMonSegment::new(Pokemon::Venomoth, 41));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Venomoth
  // r.run(NextTrainerMonSegment::new(Pokemon::Muk, 42));
  // r.run(OHKOSegment::new(Move::IronTail).crit()); // Muk
  // r.run(NextTrainerMonSegment::new(Pokemon::Crobat, 44).with_level_up());
  // r.run(OHKOSegment::new(Move::IronTail).crit()); // Crobat
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2));
  // r.save("crystal_after_koga_90");

  // r.load("crystal_after_koga_90");
  // r.run(SkipTextsSegment::new(6)); // Koga
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(4, 2).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Bruno's room
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(WalkToSegment::new(4, 7));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(11)); // Bruno
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Hitmontop
  // r.run(NextTrainerMonSegment::new(Pokemon::Onix, 43));
  // r.run(OHKOSegment::new(Move::IronTail)); // Onix
  // r.run(NextTrainerMonSegment::new(Pokemon::Hitmonlee, 42));
  // r.run(OHKOSegment::new(Move::IronTail)); // Hitmonlee
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(TextSegment::new().with_skip_ends(2)); // mon // gained // num XP
  // r.run(DelaySegment::new(MoveSegment::with_metric(A|B, Gen2SwitchMonMetric {}.assert_eq((Pokemon::Hitmonchan, 42)))));
  // r.run(SkipTextsSegment::new(1)); // sent out
  // r.run(TextSegment::with_metric(Gen2AIChooseMoveMetric {}.assert_eq(Move::MachPunch)).with_allowed_end_inputs(B).with_skip_ends(1)); // Hitmonchan
  // print_battle_stats(r);
  // r.run(SelectMoveSegment::new(Move::Spark));
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::with_metric(A, BattleMoveOrderMetric {}.debug_print().assert_eq(MoveOrder::EnemyFirst))
  //     .seq(TextSegment::with_metric(
  //         Gen2NormalHitMetric::with_expected_max_damage(19+2, 42).debug_print().expect(FightTurnResult::Hit { damage: 16+1, })).with_skip_ends(3).with_unbounded_buffer())
  //     .seq(TextSegment::with_metric(
  //         Gen2NormalHitMetric::with_expected_max_damage(75, 132).debug_print().filter(|r| if let FightTurnResult::CriticalHit { damage, } = r { damage >= &103 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //     )); // Spark //// mon // used // move // !
  // r.run(SkipTextsSegment::new(1)); // critical hit!
  // r.run(NextTrainerMonSegment::new(Pokemon::Machamp, 46));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Machamp
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_bruno_90");

  // r.load("crystal_after_bruno_90");
  // r.run(SkipTextsSegment::new(3)); // Bruno
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(4, 2).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Karen's room
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(WalkToSegment::new(4, 7));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(8).with_expected_move(Move::SandAttack)); // Karen
  // r.run(KOSegment::new(Move::Spark, EnemyAttack { mov: Move::SandAttack, attack_type: EnemyAttackType::StatUpDown } ).has_effect()); // Umbreon
  // r.run(NextTrainerMonSegment::new(Pokemon::Vileplume, 42).with_level_up());
  // r.run(OHKOSegment::new(Move::IronTail).crit()); // Vileplume
  // r.run(NextTrainerMonSegment::new(Pokemon::Gengar, 45));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Gengar
  // r.run(NextTrainerMonSegment::new(Pokemon::Houndoom, 47));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Houndoom
  // r.run(NextTrainerMonSegment::new(Pokemon::Murkrow, 44));
  // r.run(OHKOSegment::new(Move::IronTail)); // Murkrow
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2));
  // r.save("crystal_after_karen_90");

  // r.load("crystal_after_karen_90");
  // r.run(SkipTextsSegment::new(9)); // Bruno
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(4, 2).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Lance's room
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(WalkToSegment::new(4, 5).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Lance
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(11).with_expected_move(Move::HyperBeam)); // Lance
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(MoveSegment::new(D)); // pack
  // r.run(MoveSegment::new(A)); // select pack
  // r.run(VerifyInputSegment::new("BattlePack")); // InitGFX
  // r.run(VerifyInputSegment::new("BattlePack")); // InitItemsPocket
  // r.run(VerifyInputSegment::new("BattlePack")); // ItemsPocketMenu
  // r.run(MoveSegment::new(D));
  // r.run(MoveSegment::new(A)); // X Attack
  // r.run(MoveSegment::new(NIL));
  // r.run(MoveSegment::new(A)); // Use
  // r.run(MoveSegment::new(B)); // close
  // r.run(
  //   DelaySegment::new(MoveSegment::new(A)
  //   .seq(VerifyInputSegment::new("BattlePack"))
  //   .seq(MoveSegment::with_metric(NIL, BattleObedienceMetric {}.expect(BattleObedience::Obey))) // confirm
  //   .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  // );
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // r.run(MoveSegment::new(U)); // back to Fight
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Gyarados
  // r.run(NextTrainerMonSegment::new(Pokemon::Dragonite, 47));
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Dragonite
  // r.run(NextTrainerMonSegment::new(Pokemon::Dragonite, 47));
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Dragonite
  // r.run(NextTrainerMonSegment::new(Pokemon::Dragonite, 50).with_level_up());
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Dragonite
  // r.run(NextTrainerMonSegment::new(Pokemon::Aerodactyl, 46));
  // r.run(OHKOSegment::new(Move::IronTail)); // Aerodactyl
  // r.run(NextTrainerMonSegment::new(Pokemon::Charizard, 46));
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Charizard
  // r.run(EndTrainerBattleSegment::with_defeat_texts(6));
  // r.save("crystal_after_lance_90");

  // r.load("crystal_after_lance_90");
  // r.run(SkipTextsSegment::new(7)); // Lance
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(SkipTextsSegment::new(2)); // Mary
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(SkipTextsSegment::new(11)); // Oak
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(SkipTextsSegment::new(2)); // Mary
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(SkipTextsSegment::new(3)); // Lance
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(SkipTextsSegment::new(2)); // Mary
  // r.run(SkipScriptSegment::new()); // Lance
  // r.run(SkipTextsSegment::new(16)); // Lance
  // r.run(SkipScriptSegment::new()); // Lance
  // for _ in 0..29 { r.run(VerifyInputSegment::new("PrintLetterDelay")); }
  // r.run(ResetAfterHofSegment::new(NIL));
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(MoveSegment::new(A));
  // r.run(MoveSegment::new(START));
  // r.run(MoveSegment::new(A)); // Continue
  // r.run(MoveSegment::new(NIL));
  // r.run(MoveSegment::new(A)); // confirm
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(12, 6).into(OverworldInteractionResult::CountStepEvent));
  // r.run(SkipScriptSegment::new()); // Elm phone call
  // r.run(SkipTextsSegment::new(4)); // Elm phone call
  // r.run(TextSegment::new().with_skip_ends(6)); // Click // ... // ... // ...
  // r.run(WalkToSegment::new(6, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Elm's lab
  // r.run(WalkToSegment::new(5, 4));
  // r.run(WalkToSegment::new(5, 3));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(5)); // Elm
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got SSTicket
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put SSTicket in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the Key pocket
  // r.run(SkipTextsSegment::new(5)); // Elm
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 11));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(A)); // Olivine
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(19, 27));
  // r.run(WarpSegment::new().with_input(D)); // enter SS Aqua
  // r.run(WalkToSegment::new(15, 4).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter SS Aqua
  // r.run(WalkToSegment::new(3, 14).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter SS Aqua
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(7, 15).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // SS Aqua ticket check
  // r.run(SkipTextsSegment::new(1)); // SS Aqua ticket check
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // SS Aqua ticket check
  // r.run(SkipTextsSegment::new(3)); // SS Aqua ticket check
  // r.run(SkipScriptSegment::new()); // SS Aqua boarding
  // r.run(SkipTextsSegment::new(2)); // SS Aqua boarding
  // r.run(SkipScriptSegment::new()); // SS Aqua boarding
  // r.run(WalkToSegment::new(25, 6).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // Girl missing scene
  // r.run(SkipTextsSegment::new(4)); // Girl missing scene
  // r.run(SkipScriptSegment::new()); // Girl missing scene
  // r.run(WalkToSegment::new(30, 14).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter SS Aqua b1f
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(31, 7).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // blocking path
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(6)); // Sailor
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(31, 13).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter SS Aqua 1f
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(19, 8));
  // r.run(WarpSegment::new().with_input(D)); // enter Sailor room
  // r.run(WalkToSegment::new(2, 26));
  // r.run(WalkToSegment::new(3, 26));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(5)); // Sailor Stanly
  // r.run(OHKOSegment::new(Move::Spark)); // Machop
  // r.run(NextTrainerMonSegment::new(Pokemon::Machoke, 33));
  // r.run(OHKOSegment::new(Move::Spark)); // Machoke
  // r.run(NextTrainerMonSegment::new(Pokemon::Psyduck, 26));
  // r.run(OHKOSegment::new(Move::Strength)); // Psyduck
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_sailor_stanly_90");

  // r.load("crystal_after_sailor_stanly_90");
  // r.run(SkipTextsSegment::new(3)); // Sailor
  // r.run(SkipScriptSegment::new()); // Sailor
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(2, 24).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter SS Aqua 1f
  // r.run(WalkToSegment::new(30, 14).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter SS Aqua b1f
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 11).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter SS Aqua 1f
  // r.run(WalkToSegment::new(3, 13).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter captain's cabin
  // r.run(WalkToSegment::new(1, 25));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(7)); // Girl
  // r.run(SkipScriptSegment::new()); // Girl
  // r.run(SkipTextsSegment::new(2)); // Girl
  // r.run(SkipScriptSegment::new()); // Girl
  // r.run(SkipTextsSegment::new(5)); // Gramps
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got item
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put item in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the item pocket
  // r.run(SkipTextsSegment::new(2)); // Ship arrived
  // r.run(TurnSegment::new(D));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(WalkToSegment::new(31, 15));
  // r.run(WalkToSegment::new(31, 13));
  // r.run(WalkToSegment::new(25, 3));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(2)); // Ship arrived
  // r.run(SkipScriptSegment::new()); // Ship arrived
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Vermilion
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(16, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 6
  // r.run(WalkToSegment::new(6, 1).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gate house
  // r.run(WalkToSegment::new(4, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Saffron
  // r.run(WalkToSegment::new(34, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(WalkToSegment::new(11, 15).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // navigate teleporters
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(19, 15).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // navigate teleporters
  // if false { // equal
  //   r.run(TurnSegment::new(U));
  //   r.run(WalkToSegment::new(15, 9).into(OverworldInteractionResult::Warped));
  //   r.run(WarpSegment::new()); // navigate teleporters
  //   r.run(WalkToSegment::new(15, 5).into(OverworldInteractionResult::Warped));
  //   r.run(WarpSegment::new()); // navigate teleporters
  // } else {
  //   r.run(WalkToSegment::new(19, 11).into(OverworldInteractionResult::Warped));
  //   r.run(WarpSegment::new()); // navigate teleporters
  //   r.run(WalkToSegment::new(1, 11).into(OverworldInteractionResult::Warped));
  //   r.run(WarpSegment::new()); // navigate teleporters
  //   r.run(TurnSegment::new(U));
  // }
  // r.run(WalkToSegment::new(1, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // navigate teleporters
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(11, 8));
  // r.run(WalkToSegment::new(10, 8));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript)))); // 471813
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(10)); // Sabrina
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Espeon
  // r.run(NextTrainerMonSegment::new(Pokemon::MrMime, 46).with_level_up());
  // r.run(OHKOSegment::new(Move::Spark).crit()); // MrMime
  // r.run(NextTrainerMonSegment::new(Pokemon::Alakazam, 48));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Alakazam
  // r.run(EndTrainerBattleSegment::with_defeat_texts(6));
  // r.save("crystal_after_sabrina_90");

  // r.load("crystal_after_sabrina_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(7)); // Sabrina text
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(11, 9).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // navigate teleporters
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(1, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // navigate teleporters
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(15, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // navigate teleporters
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(19, 9).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // navigate teleporters
  // r.run(WalkToSegment::new(19, 17).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // navigate teleporters
  // r.run(WalkToSegment::new(9, 16));
  // r.run(WalkToSegment::new(9, 17));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(18, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gate house
  // r.run(WalkToSegment::new(4, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Route 5
  // r.run(WalkToSegment::new(15, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Cerulean
  // r.run(WalkToSegment::new(40, 22).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 9
  // r.run(WalkToSegment::new(3, 8));
  // r.run(WalkToSegment::new(4, 8));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1)); // tree can be cut
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use cut
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used cut !
  // r.run(WalkToSegment::new(8, 10));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(23, 10));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Camper Dean
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Golduck
  // r.run(NextTrainerMonSegment::new(Pokemon::Sandslash, 31));
  // r.run(OHKOSegment::new(Move::IronTail)); // Sandslash
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_camper_dean_90");

  // r.load("crystal_after_camper_dean_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(36, 4));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(55, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(56, 18).surfing().into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 10
  // r.run(WalkToSegment::new(15, 10).surfing());
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(3, 9).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Power Plant
  // r.run(WalkToSegment::new(13, 10));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(6)); // Manager
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(5, 12).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // MapScript
  // r.run(SkipTextsSegment::new(4)); // Help
  // r.run(SkipScriptSegment::new()); // MapScript
  // r.run(WalkToSegment::new(3, 16));
  // r.run(WalkToSegment::new(3, 17));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // Cerulean
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(30, 23).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(SkipTextsSegment::new(6)); // Rocket
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(SkipTextsSegment::new(2)); // Rocket
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(SkipTextsSegment::new(4)); // Rocket
  // r.run(SkipScriptSegment::new()); // SceneScript
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(21, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 24
  // r.run(WalkToSegment::new(9, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 25
  // r.run(WalkToSegment::new(10, 8));
  // r.run(WalkToSegment::new(11, 8));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3)); // Schoolboy Dudley
  // r.run(OHKOSegment::new(Move::IronTail)); // Oddish
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_schoolboy_dudley_90");

  // r.load("crystal_after_schoolboy_dudley_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(14, 11));
  // r.run(WalkToSegment::new(15, 11));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Lass Ellen
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Wigglytuff
  // r.run(NextTrainerMonSegment::new(Pokemon::Granbull, 34));
  // r.run(OHKOSegment::new(Move::Spark)); // Granbull
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_lass_ellen_90");

  // r.load("crystal_after_lass_ellen_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(19, 8));
  // r.run(WalkToSegment::new(20, 8));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Schoolboy Joe
  // r.run(OHKOSegment::new(Move::IronTail)); // Tangela
  // r.run(NextTrainerMonSegment::new(Pokemon::Vaporeon, 33));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Vaporeon
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_schoolboy_joe_90");

  // r.load("crystal_after_schoolboy_joe_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(25, 6).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Lass Laura
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Lass Laura
  // r.run(OHKOSegment::new(Move::IronTail)); // Gloom
  // r.run(NextTrainerMonSegment::new(Pokemon::Pidgeotto, 31).with_level_up());
  // r.run(OHKOSegment::new(Move::Strength)); // Pidgeotto
  // r.run(NextTrainerMonSegment::new(Pokemon::Bellossom, 31));
  // r.run(OHKOSegment::new(Move::Strength)); // Bellossom
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_lass_laura_90");

  // r.load("crystal_after_lass_laura_90");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Camper Lloyd
  // r.run(OHKOSegment::new(Move::IronTail)); // Nidoking
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_camper_lloyd_90");

  // r.load("crystal_after_camper_lloyd_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(28, 10).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Lass Shannon
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Lass Shannon
  // r.run(OHKOSegment::new(Move::IronTail)); // Gloom
  // r.run(NextTrainerMonSegment::new(Pokemon::Paras, 29));
  // r.run(OHKOSegment::new(Move::Strength)); // Paras
  // r.run(NextTrainerMonSegment::new(Pokemon::Parasect, 32));
  // r.run(OHKOSegment::new(Move::Strength)); // Parasect
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_lass_shannon_90");

  // r.load("crystal_after_lass_shannon_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(30, 7));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(R));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(5)); // Supernerd Pat
  // r.run(OHKOSegment::new(Move::Spark)); // Porygon
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_supernerd_pat_90");

  // r.load("crystal_after_supernerd_pat_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(42, 7).into(OverworldInteractionResult::MapCoordEvent));
  // r.run(SkipScriptSegment::new()); // MapScript
  // r.run(SkipTextsSegment::new(13)); // Misty
  // r.run(SkipScriptSegment::new()); // MapScript
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // Cerulean
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(30, 23).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.run(WalkToSegment::new(9, 3));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(L));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(WalkToSegment::new(6, 3).surfing());
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(6)); // Schoolboy Joe
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Golduck
  // r.run(NextTrainerMonSegment::new(Pokemon::Quagsire, 42));
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Quagsire
  // r.run(NextTrainerMonSegment::new(Pokemon::Lapras, 44));
  // r.run(OHKOSegment::new(Move::Spark)); // Lapras
  // r.run(NextTrainerMonSegment::new(Pokemon::Starmie, 47));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Starmie
  // r.run(EndTrainerBattleSegment::with_defeat_texts(3).with_level_up());
  // r.save("crystal_after_misty_90");

  // r.load("crystal_after_misty_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got badge
  // r.run(SkipTextsSegment::new(4)); // Misty text
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(6, 4).surfing());
  // r.run(WalkToSegment::new(3, 7));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(D));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::BgHiddenItem))));
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got Machine Part
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put Machine Part in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the Key pocket
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(5, 15));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // // { // teleport
  // //   r.debug_write_memory(0xdd41, 100); // learn teleport
  // //   r.debug_write_memory(0xdcb2, 7); // set teleport point
  // //   r.debug_write_memory(0xdcb3, 14); // set teleport point
  // //   r.run(MoveSegment::new(START)); // Open menu
  // //   r.run(MoveSegment::new(D));
  // //   r.run(MoveSegment::new(A)); // mon
  // //   r.run(MoveSegment::new(D));
  // //   r.run(MoveSegment::new(NIL));
  // //   r.run(MoveSegment::new(D));
  // //   r.run(MoveSegment::new(A)); // Abra
  // //   r.run(MoveSegment::new(NIL));
  // //   r.run(MoveSegment::new(A)); // Teleport
  // //   r.run(SkipTextsSegment::new(1)); // Teleport
  // //   r.run(SkipScriptSegment::new()); // Teleport MapScript
  // //   r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // //   r.run(TurnSegment::new(U));
  // //   r.run(WalkToSegment::new(14, -1).into(OverworldInteractionResult::MapConnection));
  // //   r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Enter Route 9
  // //   r.run(WalkToSegment::new(55, 4));
  // // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(40, 22).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 9
  // r.run(WalkToSegment::new(3, 8));
  // r.run(WalkToSegment::new(4, 8));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1)); // tree can be cut
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use cut
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used cut !
  // r.run(WalkToSegment::new(8, 10));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(36, 4));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(55, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(56, 18).surfing().into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 10
  // r.run(WalkToSegment::new(15, 10).surfing());
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(3, 9).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Power Plant
  // r.run(WalkToSegment::new(13, 10));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(6)); // Manager
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(4)); // Manager
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(3, 16));
  // r.run(WalkToSegment::new(3, 17));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Saffron
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(WalkToSegment::new(9, 31));
  // r.run(WalkToSegment::new(39, 23));
  // r.run(WarpSegment::new().with_input(R)); // enter gate house
  // r.run(WalkToSegment::new(9, 5));
  // r.run(WarpSegment::new().with_input(R)); // enter Route 8
  // r.run(WalkToSegment::new(12, 6));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(40, 8).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Lavender
  // r.run(WalkToSegment::new(14, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Radio Tower
  // r.run(WalkToSegment::new(9, 3));
  // r.run(WalkToSegment::new(9, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(5)); // get Expn Card
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got Expn Card
  // r.run(SkipTextsSegment::new(3)); // get Expn Card
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(3, 6));
  // r.run(WalkToSegment::new(3, 7));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(U|L));
  //   r.run(MoveSegment::new(A)); // Vermilion
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(13, 16));
  // r.run(WalkToSegment::new(13, 17));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1)); // tree can be cut
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use cut
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used cut !
  // r.run(WalkToSegment::new(10, 19).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Gym
  // r.run(WalkToSegment::new(6, 8).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Gentleman Gregory
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Gentleman Gregory
  // r.run(OHKOSegment::new(Move::Strength)); // Pikachu
  // r.run(NextTrainerMonSegment::new(Pokemon::Flaaffy, 33));
  // r.run(OHKOSegment::new(Move::Strength)); // Flaaffy
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2));
  // r.save("crystal_after_gentleman_gregory_90");

  // r.load("crystal_after_gentleman_gregory_90");
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(5, 3));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(9)); // Surge
  // r.run(OHKOSegment::new(Move::Strength)); // Raichu
  // r.run(NextTrainerMonSegment::new(Pokemon::Electrode, 40));
  // r.run(OHKOSegment::new(Move::Strength)); // Electrode
  // r.run(NextTrainerMonSegment::new(Pokemon::Magneton, 40));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Magneton
  // r.run(NextTrainerMonSegment::new(Pokemon::Electrode, 40));
  // r.run(OHKOSegment::new(Move::Strength)); // Electrode
  // r.run(NextTrainerMonSegment::new(Pokemon::Electabuzz, 46));
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Electabuzz
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2));
  // r.save("crystal_after_surge_90");

  // r.load("crystal_after_surge_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(3)); // Surge text
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 17));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(13, 19));
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // PokeGear
  //   r.run(VerifyInputSegment::new("PokeGear")); // PokegearClock_Init
  //   r.run(MoveSegment::new(R));
  //   r.run(VerifyInputSegment::new("PokeGear")); // PokegearPhone_Init
  //   r.run(MoveSegment::new(R));
  //   r.run(VerifyInputSegment::new("PokeGear")); // PokegearRadio_Init
  //   r.run(MoveSegment::new(U));
  //   for _ in 0..19 {
  //     r.run(MoveSegment::new(U|A));
  //     r.run(MoveSegment::new(U|R));
  //   }
  //   r.run(MoveSegment::new(NIL)); // needed to register that Pokeflute is playing
  //   r.run(MoveSegment::new(B));
  //   r.run(VerifyInputSegment::new("PokeGear")); // Pokegear done

  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // A
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // cut
  // }
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used cut !
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(WalkToSegment::new(34, 10));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(4)); // Snorlax
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // wild // mon // appeared
  // r.run(TextSegment::new().with_skip_ends(2)); // Go // mon // !
  // r.run(MoveSegment::new(D));
  // r.run(MoveSegment::new(R));
  // r.run(MoveSegment::new(A));
  // r.run(SkipTextsSegment::new(1)); // got away safely
  // r.run(SkipScriptSegment::new()); // Snorlax
  // r.run(WalkToSegment::new(34, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Diglett Cave
  // r.run(WalkToSegment::new(5, 32));
  // r.run(WalkToSegment::new(5, 31).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Diglett Cave
  // r.run(WalkToSegment::new(3, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Diglett Cave
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(15, 4));
  // r.run(WalkToSegment::new(15, 5));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(WalkToSegment::new(5, 9));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  // r.run(TurnSegment::new(U));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1)); // tree can be cut
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use cut
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used cut !
  // r.run(WalkToSegment::new(8, -1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Pewter
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(WalkToSegment::new(16, 17).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.run(WalkToSegment::new(4, 5).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Camper Jerry
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(5)); // Camper Jerry
  // r.run(OHKOSegment::new(Move::Strength).crit()); // Sandslash
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_camper_jerry_90");

  // r.load("crystal_after_camper_jerry_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(5, 3));
  // r.run(WalkToSegment::new(5, 2));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(8)); // Brock
  // r.run(OHKOSegment::new(Move::IronTail)); // Graveler
  // r.run(NextTrainerMonSegment::new(Pokemon::Rhyhorn, 41));
  // r.run(OHKOSegment::new(Move::IronTail)); // Rhyhorn
  // r.run(NextTrainerMonSegment::new(Pokemon::Onix, 44));
  // r.run(OHKOSegment::new(Move::IronTail)); // Onix
  // r.run(NextTrainerMonSegment::new(Pokemon::Kabutops, 42).with_level_up());
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Kabutops
  // r.run(NextTrainerMonSegment::new(Pokemon::Omastar, 42));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Omastar
  // r.run(EndTrainerBattleSegment::with_defeat_texts(5));
  // r.save("crystal_after_brock_90");

  // r.load("crystal_after_brock_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(4)); // Brock text
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 13));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(18, 36).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 2
  // r.run(WalkToSegment::new(4, 36));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(8, 54).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Viridian
  // r.run(WalkToSegment::new(28, 36).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 1
  // r.run(WalkToSegment::new(12, 18));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(12, 26));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(9, 36).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Pallet
  // r.run(WalkToSegment::new(4, 12));
  // r.run(WalkToSegment::new(4, 13));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(WalkToSegment::new(4, 18).surfing().into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 21
  // r.run(WalkToSegment::new(3, 13).surfing());
  // r.run(WalkToSegment::new(1, 36).surfing().into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Cinnabar
  // r.run(WalkToSegment::new(4, 10).surfing());
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(7, 6));
  // r.run(WalkToSegment::new(8, 6));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(25)); // Blue text
  // r.run(SkipScriptSegment::new()); // Blue
  // r.run(TurnSegment::new(L));
  // r.run(WalkToSegment::new(8, 8));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(13, 12));
  // r.run(WalkToSegment::new(13, 13));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(WalkToSegment::new(20, 16).surfing().into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 20
  // r.run(WalkToSegment::new(38, 11).surfing());
  // r.run(WalkToSegment::new(38, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.run(WalkToSegment::new(5, 3));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(10)); // Blaine
  // r.run(OHKOSegment::new(Move::Spark)); // Magcargo
  // r.run(NextTrainerMonSegment::new(Pokemon::Magmar, 45));
  // r.run(OHKOSegment::new(Move::Spark)); // Magmar
  // r.run(NextTrainerMonSegment::new(Pokemon::Rapidash, 50));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Rapidash
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2));
  // r.save("crystal_after_blaine_90");

  // r.load("crystal_after_blaine_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(3)); // Blaine text
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 5));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // r.run(WalkToSegment::new(41, 8));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used Surf!
  // r.run(SkipScriptSegment::new()); // Surf MapScript
  // r.run(WalkToSegment::new(60, 2).surfing().into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 19
  // r.run(WalkToSegment::new(4, 13).surfing());
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(7, 3).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gate house
  // r.run(WalkToSegment::new(4, 0).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter fuchsia
  // r.run(WalkToSegment::new(8, 27).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.run(WalkToSegment::new(1, 9));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipScriptSegment::new()); // Janine
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(5)); // Janine
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Crobat
  // r.run(NextTrainerMonSegment::new(Pokemon::Weezing, 36));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Weezing
  // r.run(NextTrainerMonSegment::new(Pokemon::Weezing, 36));
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Weezing
  // r.run(NextTrainerMonSegment::new(Pokemon::Ariados, 33).with_level_up());
  // r.run(OHKOSegment::new(Move::ThunderShock)); // Ariados
  // r.run(NextTrainerMonSegment::new(Pokemon::Venomoth, 39));
  // r.run(OHKOSegment::new(Move::ThunderShock).crit()); // Venomoth
  // r.run(EndTrainerBattleSegment::with_defeat_texts(3));
  // r.save("crystal_after_janine_90");

  // r.load("crystal_after_janine_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(3)); // Janine text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(4)); // Janine text
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 16));
  // r.run(WalkToSegment::new(5, 17));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(A)); // Saffron
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(0, 25));
  // r.run(WarpSegment::new().with_input(L)); // enter gate house
  // r.run(WalkToSegment::new(0, 5));
  // r.run(WarpSegment::new().with_input(L)); // enter Route 7
  // r.run(WalkToSegment::new(-1, 1).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Celadon
  // r.run(WalkToSegment::new(28, 34));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().assert_eq(OverworldInteractionResult::Interact(InteractType::TileCollision))));
  // r.run(SkipTextsSegment::new(1)); // tree can be cut
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // use cut
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // used cut !
  // r.run(WalkToSegment::new(10, 29).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.run(WalkToSegment::new(4, 11).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Twins Jo&Zoe
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Twins Jo&Zoe
  // r.run(OHKOSegment::new(Move::IronTail)); // Victreebel
  // r.run(NextTrainerMonSegment::new(Pokemon::Vileplume, 35));
  // r.run(OHKOSegment::new(Move::Strength)); // Vileplume
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_twins_jo_zoe_90");

  // r.load("crystal_after_twins_jo_zoe_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(2, 10));
  // r.run(WalkToSegment::new(2, 9));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectTrainer))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Picnicker Tanya
  // r.run(OHKOSegment::new(Move::IronTail)); // Exeggutor
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_picnicker_tanya_90");

  // r.load("crystal_after_picnicker_tanya_90");
  // r.run(TurnSegment::new(D));
  // r.run(WalkToSegment::new(4, 5).into(OverworldInteractionResult::SeenByTrainer));
  // r.run(SkipScriptSegment::new()); // Beauty Julia
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Beauty Julia
  // r.run(OHKOSegment::new(Move::IronTail)); // Paras
  // r.run(NextTrainerMonSegment::new(Pokemon::Exeggcute, 32));
  // r.run(OHKOSegment::new(Move::Strength)); // Exeggcute
  // r.run(NextTrainerMonSegment::new(Pokemon::Parasect, 35));
  // r.run(OHKOSegment::new(Move::Strength)); // Parasect
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_beauty_julia_90");

  // r.load("crystal_after_beauty_julia_90");
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(5, 5));
  // r.run(WalkToSegment::new(5, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(10)); // Erika
  // r.run(OHKOSegment::new(Move::Spark)); // Tangela
  // r.run(NextTrainerMonSegment::new(Pokemon::Jumpluff, 41));
  // r.run(OHKOSegment::new(Move::Spark)); // Jumpluff
  // r.run(NextTrainerMonSegment::new(Pokemon::Victreebel, 46));
  // r.run(OHKOSegment::new(Move::IronTail).crit()); // Victreebel
  // r.run(NextTrainerMonSegment::new(Pokemon::Bellossom, 46).with_level_up().with_override_move(Move::ThunderShock)); // learn Crunch
  // r.run(OHKOSegment::new(Move::Crunch).crit()); // Bellossom
  // r.run(EndTrainerBattleSegment::with_defeat_texts(3));
  // r.save("crystal_after_erika_90");

  // r.load("crystal_after_erika_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(7)); // Erika text
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // put TM in
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the TM pocket
  // r.run(SkipTextsSegment::new(4)); // Erika text
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 16));
  // r.run(WalkToSegment::new(5, 17));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(U|L));
  //   r.run(MoveSegment::new(A)); // Viridian
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(32, 7).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter gym
  // r.run(WalkToSegment::new(5, 5));
  // r.run(WalkToSegment::new(5, 4));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(12)); // Blue
  // r.run(OHKOSegment::new(Move::Spark)); // Pidgeot
  // r.run(NextTrainerMonSegment::new(Pokemon::Rhydon, 56));
  // r.run(OHKOSegment::new(Move::Crunch).crit()); // Rhydon
  // r.run(NextTrainerMonSegment::new(Pokemon::Alakazam, 54));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Alakazam
  // r.run(NextTrainerMonSegment::new(Pokemon::Exeggutor, 58));
  // r.run(OHKOSegment::new(Move::Crunch).crit()); // Exeggutor
  // r.run(NextTrainerMonSegment::new(Pokemon::Arcanine, 58));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Arcanine
  // r.run(NextTrainerMonSegment::new(Pokemon::Gyarados, 58).with_level_up());
  // r.run(OHKOSegment::new(Move::Spark)); // Gyarados
  // r.run(EndTrainerBattleSegment::with_defeat_texts(5));
  // r.save("crystal_after_blue_90");

  // r.load("crystal_after_blue_90");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player got badge
  // r.run(SkipTextsSegment::new(4)); // Blue text
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(5, 17));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // Pallet
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(WalkToSegment::new(12, 11).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Oak's Lab
  // r.run(WalkToSegment::new(4, 3));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.run(SkipTextsSegment::new(22)); // Oak
  // r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // Oak
  // r.run(SkipTextsSegment::new(4)); // Oak
  // r.run(TurnSegment::new(R));
  // r.run(WalkToSegment::new(4, 11));
  // r.run(WarpSegment::new().with_input(D)); // leave
  // { // Menuing
  //   r.run(MoveSegment::new(START)); // Open menu
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // mon menu
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Kenya
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Fly
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(U|R));
  //   r.run(MoveSegment::new(A)); // Viridian
  //   r.run(SkipScriptSegment::new()); // Fly
  // }
  // r.run(MoveSegment::with_metric(SELECT, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::SelectMenu)));
  // r.run(TurnSegment::new(U));
  // r.run(WalkToSegment::new(-1, 17).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Route 22 // 594615
  // r.run(WalkToSegment::new(21, 6));
  // r.run(JumpLedgeSegment::new(D));
  // r.run(WalkToSegment::new(13, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Indigo Plateau
  // r.run(WalkToSegment::new(2, 6));
  // r.run(WalkToSegment::new(2, 7));
  // r.run(WarpSegment::new().with_input(D)); // enter Route 28
  // r.run(WalkToSegment::new(-1, 12).into(OverworldInteractionResult::MapConnection));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::MapConnection))); // Mt.Silver
  // r.run(WalkToSegment::new(18, 11).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Mt.Silver
  // r.run(WalkToSegment::new(4, 20));
  // r.run(WalkToSegment::new(4, 16));
  // r.run(WalkToSegment::new(15, 16));
  // r.run(WalkToSegment::new(8, 5));
  // r.run(WalkToSegment::new(15, 1).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Mt.Silver b2f
  // r.run(WalkToSegment::new(11, 5).into(OverworldInteractionResult::Warped));
  // r.run(WarpSegment::new()); // enter Mt.Silver b3f
  // r.run(WalkToSegment::new(9, 11));
  // r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  // r.save("crystal_before_red_90");

  // r.load("crystal_before_red_90");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Red
  // r.run(OHKOSegment::new(Move::Crunch).crit()); // Pikachu
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(NextTrainerMonSegment::new(Pokemon::Espeon, 73));
  // print_battle_stats(r);
  // r.run(SelectMoveSegment::new(Move::Spark));
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::with_metric(A,
  //         BattleMoveOrderMetric {}.assert_eq(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.assert_eq(BattleObedience::Obey)))
  //     .seq(TextSegment::with_metric(
  //         Gen2NormalHitMetric::with_expected_max_damage(75, 130).with_effect().expect(FightTurnResult::Hit { damage: 74, })).with_skip_ends(3).with_unbounded_buffer())
  //     )); // Spark //// mon // used // move // !
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(SkipTextsSegment::new(1)); // Paralyzed
  // r.run(TextSegment::new()); // Paralyzed
  // r.run(DelaySegment::new(MoveSegment::with_metric(A|B, Gen2FullyParalyzedMetric {})));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Fully paralyzed
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Espeon
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // r.run(NextTrainerMonSegment::new(Pokemon::Snorlax, 75).with_expected_move(Move::Amnesia));
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(SelectMoveSegment::new(Move::IronTail));
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::with_metric(A,
  //         BattleMoveOrderMetric {}.assert_eq(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.assert_eq(BattleObedience::Obey)))
  //     .seq(TextSegment::with_metric(
  //         Gen2NormalHitMetric::with_expected_max_damage(81, 144).with_effect().filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage == &132 } else { false }).into_unit()).with_skip_ends(3).with_unbounded_buffer())
  //     )); // Iron Tail //// mon // used // move // !
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(SkipTextsSegment::new(1)); // critical hit!
  // r.run(TextSegment::new().with_skip_ends(2)); // mon // defense // lowered
  // r.run(
  //   DelaySegment::new(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey)) // confirm
  //   .seq(TextSegment::with_metric(Gen2MoveSuccessMetric {}.debug_print().expect(FightTurnResult::Succeeded)).with_skip_ends(3).with_unbounded_buffer())) // mon // used // move // !
  // );
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // mon // spcdef
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // went way up
  // r.run(OHKOSegment::new(Move::IronTail).crit()); // Snorlax
  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");
  // r.run(NextTrainerMonSegment::new(Pokemon::Venusaur, 77).with_expected_move(Move::SolarBeam));
  // r.save("crystal_test_tmp2");
  // r.load("crystal_test_tmp2");
  // r.run(SelectMoveSegment::new(Move::Crunch));
  // r.run(
  //   DelaySegment::new(
  //     MoveSegment::with_metric(A,
  //         BattleMoveOrderMetric {}.assert_eq(MoveOrder::PlayerFirst).and_then(BattleObedienceMetric {}.assert_eq(BattleObedience::Obey)))
  //     .seq(TextSegment::with_metric(
  //         Gen2NormalHitMetric::with_expected_max_damage(50, 86).with_effect().expect(FightTurnResult::CriticalHit { damage: 83 })).with_skip_ends(3).with_unbounded_buffer())
  //     )); // Crunch //// mon // used // move // !
  // r.run(SkipTextsSegment::new(1)); // critical hit!
  // r.save("crystal_test_tmp3");
  // r.load("crystal_test_tmp3");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // spcdef // lowered
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // mon // took in sunlight
  // r.run(OHKOSegment::new(Move::Crunch).crit()); // Venusaur
  // r.run(NextTrainerMonSegment::new(Pokemon::Charizard, 77));
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Charizard
  // r.run(NextTrainerMonSegment::new(Pokemon::Blastoise, 77).with_level_up());
  // r.run(OHKOSegment::new(Move::Spark).crit()); // Blastoise
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1));
  // r.save("crystal_after_red_90");

  r.load("crystal_after_red_90");
  r.run(SkipTextsSegment::new(1)); // Red






  // r.save("crystal_test_tmp");
  // r.load("crystal_test_tmp");

  // r.run_debug(TextSegment::new().with_skip_ends(1));
  // r.run_debug(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit()));
}

#[allow(dead_code)]
fn bump_turn_slide(r: &mut GbRunner<Crystal>, direction: Input, length: usize) {
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoEvents)));
  r.run(TurnSegment::new(direction));
  for _ in 0..length { r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::Walked(direction, WalkType::Ice)))); }
}


#[allow(dead_code)]
fn print_battle_stats(r: &mut GbRunner<Crystal>) {
  println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  println!("Player: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  println!("Enemy: {:?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
}