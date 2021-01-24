use crate::run::*;
use montas::metric::*;
use montas::metric::overworld::gen1::*;
#[allow(unused_imports)] use gambatte::inputs::*;
#[allow(unused_imports)] use montas::constants::*;
#[allow(unused_imports)] use montas::segment::battle::*;
#[allow(unused_imports)] use montas::segment::battle::gen1::*;
#[allow(unused_imports)] use montas::segment::overworld::gen1::*;

#[allow(dead_code)]
pub fn start() {
  let mut r: GbRunner<Blue> = GbRunner::single_with_screen(&[]);

  run(&mut r);

  r.run(IdentifyInputSegment::new());

  r.debug_segment_end("temp/blue_glitchless");
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
  // r.save("blue_intro");

  r.load("blue_intro");
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
  r.save("blue_test");

  // r.load("blue_test");
  // r.run(OverworldMoveSegment::wait()); // Skip PalletTownScript0
  // r.run(TextSegment::new().with_allowed_end_inputs(A)); // it's dangerous to go outside, take this
  // r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // r.run(SkipTextsSegment::new(6).with_confirm_input(B)); // it's dangerous to go outside, take this
  // r.run(OverworldMoveSegment::wait()); // Skip PalletTownScript load
  // r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(L)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(D)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(R)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(R)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(R)); // Follow oak
  // r.run(OverworldMoveSegment::auto_walk(U)); // Follow oak
  // r.run(SkipTextsSegment::new(18).with_confirm_input(B)); // oak speech choose a mon
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkStepSegment::new(D));
  // r.run(WalkStepSegment::new(R));
  // r.run(WalkStepSegment::new(R));
  // r.run(OverworldMoveSegment::collide(U));
  // r.run(OverworldMoveSegment::interact_with(3));
  // r.run(VerifyInputSegment::new("WaitForTextScrollButtonPress").with_input(B)); // skip squirtle image
  // r.run(VerifyInputSegment::new("ShowPokedexDataInternal").with_input(A)); // skip squirtle dex
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B).with_buffer_size(256)); // choose squirtle
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A).with_buffer_size(256)); // choose squirtle
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B).with_buffer_size(2048)); // choose squirtle
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B).with_skip_ends(4).with_buffer_size(2048)); // received a // squirtle // ! // Do you want
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A).with_buffer_size(2048)); // nickname to // squirtle // ?
  // r.run(MoveSegment::new(NIL).with_buffer_size(2048)); // delay for A repress
  // r.run(MoveSegment::new(A).with_buffer_size(2048)); // delay for A repress
  // r.run(DelaySegment::new(MoveSegment::with_metric(START, Gen1DVMetric {}.filter(|v| {
  //   // if v.atk < 15 || v.def < 11 || v.spc < 12 || v.spd < 7 || v.def & 1 == 0 || (v.spd & 1 == 0 && v.spc & 1 == 0) { return false; } // totodile
  //   if v.atk < 15 || v.spc < 15 || v.spd < 15 { return false; } // squirtle DVs
  //   log::info!("Chosen DVs: {:?}", v); true
  // }).into_unit())));
  // r.save("blue_chosen_starter");

  // r.load("blue_chosen_starter"); // DVs: 15 / 6 / 15 / 15
  // r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // rival chooses bulbasaur
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // rival received // bulbasaur // !
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkStepSegment::new(L));
  // r.run(WalkStepSegment::new(L));
  // r.run(WalkStepSegment::new(D));
  // r.run(WalkStepSegment::new(D).into(OverworldInteractionResult::NoOverworldInput)); // run into rival fight
  // r.run(SkipTextsSegment::new(4).with_confirm_input(B)); // rival pre-fight text
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 225 }))); // initiate rival fight
  // r.run(StartTrainerBattleSegment::new());

  // r.run(SelectMoveSegment::new(Move::TailWhip)); // Fight
  // r.run(DelaySegment::new(
  //   MoveSegment::with_metric(A,
  //     ExpectedAIChooseMoveMetric { expected_move: Some(Move::Growl) }.and_then(BattleMoveOrderMetric {}).expect(MoveOrder::PlayerFirst))
  //   .seq(TextSegment::with_metric(MoveEffectMetric {}.debug_print().expect(MoveEffectResult::Success)).with_skip_ends(4))
  // ));
  // r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(A));
  // r.run(DelaySegment::new(
  //   MoveSegment::new(B)
  //   .seq(TextSegment::with_metric(MoveEffectMetric {}.debug_print().expect(MoveEffectResult::Failed)).with_skip_ends(4))
  // ));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed
  // r.run(KOSegment::new(Move::Tackle, EnemyAttack {mov: Move::Growl, attack_type: EnemyAttackType::EffectFailed }));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(3).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_after_rival1");

  // r.load("blue_after_rival1");
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoAction))); // advance map script (abSs buttons allowed)

  // r.run(SkipTextsSegment::new(4)); // after rival1 texts
  // r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkStepSegment::new(D));
  // r.run(WalkStepSegment::new(D));
  // r.run(WalkStepSegment::new(D));
  // r.run(WalkStepSegment::new(D));
  // r.run(WalkStepSegment::new(D).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp

  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(10, -1));
  // r.run(WalkToSegment::new(11, -1)); // Enter Viridian
  // r.run(WalkToSegment::new(29, 19).into(OverworldInteractionResult::NoOverworldInput)); // Enter Mart, starts cutscene
  // r.save("blue_enter_viridian_mart");

  // r.load("blue_enter_viridian_mart");
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // mart cutscene
  // r.run(OverworldMoveSegment::auto_walk(U)); // mart cutscene
  // r.run(OverworldMoveSegment::auto_walk(U)); // mart cutscene
  // r.run(OverworldMoveSegment::auto_walk(L)); // mart cutscene
  // r.run(SkipTextsSegment::new(4).with_confirm_input(B)); // mart cutscene
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoAction)));
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(3, 6));
  // r.run(WalkToSegment::new(3, 7).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp

  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(29, 26));
  // r.run(OverworldMoveSegment::jump_ledge(D)); // Jump ledge
  // r.run(WalkToSegment::new(21, 36)); // enter Route 1
  // r.run(WalkToSegment::new(14, 18));
  // r.run(OverworldMoveSegment::jump_ledge(D)); // Jump ledge
  // r.run(WalkToSegment::new(12, 26));
  // r.run(OverworldMoveSegment::jump_ledge(D)); // Jump ledge
  // r.run(WalkToSegment::new(11, 36)); // enter pallet town
  // r.run(WalkToSegment::new(12, 11)); // enter oak's lab
  // r.save("blue_test");

  // r.load("blue_test");
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(4, 2)); // next to oak
  // r.run(OverworldMoveSegment::collide(R)); // Turn towards Oak
  // r.run(OverworldMoveSegment::interact_with(5)); // Talk to Oak
  // r.run(SkipTextsSegment::new(41).with_confirm_input(B)); // Oak speech
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(4, 11).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.save("blue_after_oak_parcel");

  // r.load("blue_after_oak_parcel");
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(10, -1)); // enter Route 1
  // r.run(WalkToSegment::new(11, -1)); // Enter Viridian
  // r.run(WalkToSegment::new(18, -1)); // Enter Route 2
  // r.run(WalkToSegment::new(3, 43)); // Enter Viridian Forest
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(5, 0)); // Enter Viridian Forest
  // r.save("blue_test");

  // r.load("blue_test");
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(25, 29).with_buffer_size(256));
  // r.run(DelaySegment::new(MoveSegment::with_metric(U, OverworldInteractionMetric {}.filter(|v| {
  //   if let OverworldInteractionResult::WildEncounter { species: Pokemon::Pikachu, level: 5, dvs } = v {
  //     log::info!("Pika");
  //     if dvs.hp() <= 4 && dvs.def <= 9 {
  //       log::info!("Pika DVs: {:?}", dvs); true
  //     } else { false }
  //   } else { false }
  // }).into_unit())));
  // r.save("blue_pikachu_encounter");

  // r.load("blue_pikachu_encounter");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Wild // Pikachu // appeared
  // r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(B));
  // r.run_merge(FightTurnSegment::new(Move::Tackle, false, 6, EnemyAttack { attack_type: EnemyAttackType::Hit { damage: 10}, mov: Move::ThunderShock } ));
  // r.run_merge(FightTurnSegment::new(Move::Tackle, false, 6, EnemyAttack { attack_type: EnemyAttackType::Hit { damage: 11}, mov: Move::ThunderShock } ));
  // r.run(KOSegment::new(Move::Tackle, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::Growl } ));
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  // r.save("blue_after_pikachu_encounter");

  // r.load("blue_after_pikachu_encounter");
  // r.run(WalkToSegment::new(2, 19));
  // r.run(OverworldMoveSegment::interact_with(4)); // Bugcatcher
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1));
  // r.save("blue_viridian_bugcatcher");

  // r.load("blue_viridian_bugcatcher");
  // r.run(KOSegment::new(Move::Tackle, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::StringShot } ));
  // r.save("blue_test");

  // r.load("blue_test");
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2).with_level_up().with_learn_move().with_name_in_defeat_texts());
  // r.run(WalkToSegment::new(1, 0).into(OverworldInteractionResult::NoOverworldInput)); // Leave Forest
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(U)); // edge warp
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(5, 0)); // Leave Viridian Forest
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(8, -1)); // enter Pewter City
  // r.run(WalkToSegment::new(16, 17)); // enter Gym
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(4, 2)); // stand in front of brock
  // r.run(OverworldMoveSegment::interact_with(1)); // Brock
  // r.save("blue_test2");

  // r.load("blue_test2");
  // r.save("blue_before_brock");

  // r.load("blue_before_brock");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(9));
  // r.run(KOSegment::new(Move::Bubble, EnemyAttack { attack_type: EnemyAttackType::HitFailed, mov: Move::Tackle } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.save("blue_test");

  // r.load("blue_test");
  // r.run(NextTrainerMonSegment::new().with_level_up());
  // r.run(KOSegment::new(Move::Bubble, EnemyAttack { attack_type: EnemyAttackType::Hit { damage: 7 }, mov: Move::Tackle } ));
  // r.save("blue_test2");

  // r.load("blue_test2");
  // r.run(EndTrainerBattleSegment::with_defeat_texts(10).with_level_up().with_name_in_defeat_texts());
  // r.run(SkipTextsSegment::new(14).with_confirm_input(B)); // Brock speech
  // r.run(WalkToSegment::new(4, 13).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.save("blue_after_brock");

  // r.load("blue_after_brock");
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(23, 17)); // Enter Mart
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(3, 5));
  // r.run(WalkToSegment::new(2, 5));
  // r.run(OverworldMoveSegment::interact_with(1)); // Mart
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // buy
  // r.run(TextSegment::new()); // buy text
  // r.run(MoveSegment::new(D));
  // r.run(MoveSegment::new(D | L | A)); // Escape Rope
  // for _ in 0..5 {
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(NIL));
  // }
  // r.run(MoveSegment::new(U));
  // r.run(MoveSegment::new(A));
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // buy Escape Rope x7
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // buy Escape Rope x7
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // buy Escape Rope x7
  // r.run(MoveSegment::new(A)); // Poke Ball
  // r.run(MoveSegment::new(U));
  // r.run(MoveSegment::new(NIL));
  // r.run(MoveSegment::new(U));
  // r.run(MoveSegment::new(A));
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // buy Poke Ball x3
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // buy Poke Ball x3
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // buy Poke Ball x3
  // r.run(MoveSegment::new(B)); // Close
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Close
  // r.run(WalkToSegment::new(3, 6));
  // r.run(WalkToSegment::new(3, 7).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(40, 18)); // Enter Route 3
  // r.run(WalkToSegment::new(11, 6)); // Bugcatcher
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoAction))); // advance map script (abSs buttons allowed)
  // r.save("blue_route3_bugcatcher1");

  // r.load("blue_route3_bugcatcher1");
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 202 }))); // initiate bugcatcher fight
  // r.run(StartTrainerBattleSegment::new());
  // // r.run_merge(FightTurnSegment::new(Move::Bubble, false, 9, EnemyAttack { attack_type: EnemyAttackType::Hit { damage: 4}, mov: Move::Tackle } ).with_expected_effect(Some(MoveEffectResult::NoEffect)));
  // r.run(KOSegment::new(Move::Bubble, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::StringShot } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(NextTrainerMonSegment::new().with_level_up());
  // r.save("blue_test");

  // r.load("blue_test");
  // // r.run_merge(FightTurnSegment::new(Move::Bubble, true, 14, EnemyAttack { attack_type: EnemyAttackType::HitNoEffect { damage: 4}, mov: Move::PoisonSting } ).with_expected_effect(Some(MoveEffectResult::NoEffect)));
  // r.run(KOSegment::new(Move::Bubble, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::StringShot } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(NextTrainerMonSegment::new());
  // r.save("blue_test2");

  // r.load("blue_test2");
  // r.run(KOSegment::new(Move::Bubble, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::StringShot } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_route3_after_bugcatcher1");

  // r.load("blue_route3_after_bugcatcher1");
  // r.run(WalkToSegment::new(12, 4)); // Youngster
  // r.run(WalkToSegment::new(13, 4)); // Youngster
  // r.run(OverworldMoveSegment::interact_with(3)); // Youngster
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2));
  // r.run_merge(FightTurnSegment::new(Move::Bubble, false, 9, EnemyAttack { attack_type: EnemyAttackType::Hit { damage: 6}, mov: Move::Tackle } ).with_expected_effect(Some(MoveEffectResult::Success)));
  // r.save("blue_test");

  // r.load("blue_test");
  // r.run(KOSegment::new(Move::Bubble, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::TailWhip } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(NextTrainerMonSegment::new().with_level_up());
  // r.save("blue_test2");

  // r.load("blue_test2");
  // r.run(KOSegment::new(Move::Bubble, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::Leer } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_route3_after_youngster");

  // r.load("blue_route3_after_youngster");
  // r.run(WalkToSegment::new(18, 5)); // Bugcatcher
  // r.run(OverworldMoveSegment::interact_with(5)); // Bugcatcher
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1));
  // r.run(KOSegment::new(Move::Bubble, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::StringShot } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(NextTrainerMonSegment::new().with_level_up());
  // r.save("blue_test");

  // r.load("blue_test");
  // r.run(KOSegment::new(Move::Bubble, EnemyAttack { attack_type: EnemyAttackType::StatUpDown, mov: Move::Harden } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(NextTrainerMonSegment::new());
  // r.save("blue_test2");

  // r.load("blue_test2");
  // r.run(KOSegment::new(Move::Bubble, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::StringShot } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(NextTrainerMonSegment::new());
  // r.run(KOSegment::new(Move::Bubble, EnemyAttack { attack_type: EnemyAttackType::StatUpDown, mov: Move::Harden } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2).with_name_in_defeat_texts());
  // r.save("blue_route3_after_bugcatcher2");

  // r.load("blue_route3_after_bugcatcher2");
  // r.run(WalkToSegment::new(24, 5)); // Bugcatcher
  // r.run(OverworldMoveSegment::collide(D));
  // r.run(OverworldMoveSegment::interact_with(8));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1));
  // r.run(KOSegment::new(Move::Bubble, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::StringShot } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(NextTrainerMonSegment::new().with_level_up().with_learn_move());
  // r.run(KOSegment::new(Move::WaterGun, EnemyAttack { attack_type: EnemyAttackType::StatUpDown, mov: Move::Harden } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_route3_after_bugcatcher3");

  // r.load("blue_route3_after_bugcatcher3");
  // r.run(WalkToSegment::new(31, 11).with_buffer_size(256));
  // r.run(WalkToSegment::new(32, 11).with_buffer_size(256));
  // r.run(DelaySegment::new(MoveSegment::with_metric(R, OverworldInteractionMetric {}.filter(|v| {
  //   if let OverworldInteractionResult::WildEncounter { species: Pokemon::Pidgey, level: _, dvs: _ } = v { true } else { false }
  // }).into_unit())));
  // r.save("blue_pidgey_encounter");

  // r.load("blue_pidgey_encounter");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Wild // Pidgey // appeared
  // r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(B));
  // r.run(MoveSegment::new(D|A)); // Items
  // r.run(MoveSegment::new(D|L)); // Down
  // r.run(DelaySegment::new(MoveSegment::new(D|R|A).seq(TextSegment::with_metric(CatchSuccessMetric {}).with_skip_ends(2)))); // Use Pokeball
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Pidgey caught
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Pidgey caught
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // Pidgey Pokedex
  // r.run(MoveSegment::new(A)); // Confirm dex
  // r.run(MoveSegment::new(B)); // Confirm dex
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Nickname?
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // no Nickname
  // r.save("blue_after_pidgey_encounter");

  // r.load("blue_after_pidgey_encounter");
  // r.run(WalkToSegment::new(59, -1)); // Enter Route 4
  // r.run(WalkToSegment::new(18, 5).with_buffer_size(64)); // Enter Mt. Moon
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(17, 11).with_buffer_size(64)); // Enter Mt. Moon UF2
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(17, 11).with_buffer_size(64)); // Enter Mt. Moon UF3
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(29, 6).with_buffer_size(64)); // Enter Mt. Moon UF3
  // r.run(OverworldMoveSegment::collide(U));
  // r.run(OverworldMoveSegment::interact_with(9));
  // r.save("blue_test");

  // r.load("blue_test");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // collect item text, don't hold open text box
  // r.run(OverworldMoveSegment::open_main_menu());
  // r.run(MoveSegment::new(D));
  // r.run(MoveSegment::new(NIL));
  // r.run(MoveSegment::new(D));
  // r.run(MoveSegment::new(A)); // items
  // r.run(MoveSegment::new(D));
  // r.run(MoveSegment::new(D|L));
  // r.run(MoveSegment::new(D|R));
  // r.run(MoveSegment::new(A)); // TM01
  // r.run(MoveSegment::new(NIL));
  // r.run(MoveSegment::new(A)); // use
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Booted up TM
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // It contains // move // !
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // Teach Mega Punch
  // r.run(MoveSegment::new(NIL));
  // r.run(MoveSegment::new(A)); // Squirtle
  // r.run(OverrideMoveSegment::new(1).with_confirm_input(A)); // override Tail Whip
  // r.run(MoveSegment::new(B)); // close
  // r.run(MoveSegment::new(START)); // close
  // r.run(WalkToSegment::new(25, 9)); // Enter Mt. Moon UF2
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(25, 9)); // Enter Mt. Moon
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(5, 5)); // Enter Mt. Moon UF2
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(21, 17).with_unbounded_buffer()); // Enter Mt. Moon UF3
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(10, 16).with_unbounded_buffer()); // Rocket
  // r.run(OverworldMoveSegment::collide(R));
  // r.run(OverworldMoveSegment::interact_with(2));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(3));
  // r.save("blue_mt_moon_rocket");

  // r.load("blue_mt_moon_rocket");
  // r.run(KOSegment::new(Move::MegaPunch, EnemyAttack { attack_type: EnemyAttackType::Hit { damage: 7 }, mov: Move::Tackle } ));
  // r.run(NextTrainerMonSegment::new());
  // r.save("blue_test");

  // r.load("blue_test");
  // r.run(OHKOSegment::new(Move::MegaPunch));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_mt_moon_after_rocket");

  // r.load("blue_mt_moon_after_rocket");
  // r.run(TextSegment::new().with_skip_ends(2));
  // for _ in 0..72 {
  //   r.run(VerifyInputSegment::new("Evolution_CheckForCancel").with_input(NIL)); // Don't cancel evolution
  // }
  // r.run(TextSegment::new().with_skip_ends(4));
  // r.run(WalkToSegment::new(13, 8).into(OverworldInteractionResult::NoOverworldInput)); // Super Nerd
  // r.run(SkipTextsSegment::new(3).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 208 }))); // initiate Super Nerd fight
  // r.run(StartTrainerBattleSegment::new());
  // r.run_merge(FightTurnSegment::new(Move::WaterGun, true, 33, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::Disable } ).with_exact_damage());
  // r.run(OHKOSegment::new(Move::Bubble));
  // r.run(NextTrainerMonSegment::new());
  // r.save("blue_test");
  // r.load("blue_test");
  // r.run(KOSegment::new(Move::MegaPunch, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::Screech }));
  // r.run(NextTrainerMonSegment::new());
  // r.save("blue_test2");
  // r.load("blue_test2");
  // r.run_merge(FightTurnSegment::new(Move::Bubble, false, 8, EnemyAttack { attack_type: EnemyAttackType::HitFailed, mov: Move::Smog } ).with_exact_damage().with_expected_effect(Some(MoveEffectResult::NoEffect)));
  // r.run(OHKOSegment::new(Move::WaterGun));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_mt_moon_after_super_nerd");

  // r.load("blue_mt_moon_after_super_nerd");
  // r.run(WalkToSegment::new(13, 7)); // Fossil
  // r.run(OverworldMoveSegment::interact_with(7));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose Helix Fossil
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // choose Helix Fossil
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B).with_unbounded_buffer()); // choose Helix Fossil
  // r.run(OverworldMoveSegment::turn(D).with_unbounded_buffer());
  // r.run(WalkToSegment::new(4, 7).with_unbounded_buffer()); // Leave Mt. Moon
  // r.run(WalkToSegment::new(5, 7).with_unbounded_buffer()); // Leave Mt. Moon
  // r.run(WalkToSegment::new(27, 3).with_unbounded_buffer()); // Leave Mt. Moon
  // r.run(OverworldMoveSegment::turn(U).with_unbounded_buffer());
  // r.run(WalkToSegment::new(64, 8).with_unbounded_buffer());
  // r.run(OverworldMoveSegment::jump_ledge(D).with_unbounded_buffer()); // Jump ledge
  // r.run(WalkToSegment::new(64, 14).with_unbounded_buffer());
  // r.run(WalkToSegment::new(65, 14).with_unbounded_buffer());
  // r.run(DelaySegment::new(MoveSegment::with_metric(R, OverworldInteractionMetric {}.filter(|v| {
  //   if let OverworldInteractionResult::WildEncounter { species: Pokemon::Sandshrew, level: _, dvs: _ } = v { true } else { false }
  // }).into_unit())));
  // r.save("blue_sandshrew_encounter");

  // r.load("blue_sandshrew_encounter");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Wild // Sandshrew // appeared
  // r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(B));
  // r.run(MoveSegment::new(D|A)); // Items
  // r.run(MoveSegment::new(D|L)); // Down
  // r.run(DelaySegment::new(MoveSegment::new(D|R|A).seq(TextSegment::with_metric(CatchSuccessMetric {}).with_skip_ends(2)))); // Use Pokeball
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Sandshrew caught
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Sandshrew caught
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // Sandshrew Pokedex
  // r.run(MoveSegment::new(A)); // Confirm dex
  // r.run(MoveSegment::new(B)); // Confirm dex
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Nickname?
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // no Nickname
  // r.save("blue_after_sandshrew_encounter");

  // r.load("blue_after_sandshrew_encounter");
  // r.run(WalkToSegment::new(90, 11)); // Enter Cerulean City
  // { // 63769 - 63054 = 715       69075-176   69715-142
  //   r.run(WalkToSegment::new(13, 25)); // Enter Bike Shop
  //   r.run(OverworldMoveSegment::turn(L));
  //   r.run(WalkToSegment::new(6, 5)); // Enter Bike Shop
  //   r.run(WalkToSegment::new(6, 4)); // Enter Bike Shop
  //   r.run(OverworldMoveSegment::interact_with(1));
  //   r.run(SkipTextsSegment::new(2).with_confirm_input(A)); // Bike Shop
  //   r.run(MoveSegment::new(B)); // cancel
  //   r.run(MoveSegment::new(A)); // cancel (IT)
  //   r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  //   r.run(WalkToSegment::new(3, 6)); // Leave Bike Shop
  //   r.run(WalkToSegment::new(3, 7).into(OverworldInteractionResult::NoOverworldInput)); // Leave Bike Shop
  //   r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  //   r.run(OverworldMoveSegment::turn(L));
  // }
  // r.run(WalkToSegment::new(30, 19)); // Enter Gym
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(5, 3).into(OverworldInteractionResult::NoOverworldInput)); // Jr. Trainer F
  // r.save("blue_cerulean_jrtrainerf");

  // r.load("blue_cerulean_jrtrainerf");
  // r.run(Gen1ITSkipTextsSegment::new(2).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 206 }))); // initiate Jr. Trainer F fight
  // r.run(Gen1ITStartTrainerBattleSegment::new());
  // // r.run_merge(Gen1ITFightTurnSegment::new(Move::MegaPunch, true, 27, EnemyAttack { attack_type: EnemyAttackType::Hit { damage: 7}, mov: Move::Peck } ));
  // r.run(Gen1ITKOSegment::new(Move::MegaPunch, EnemyAttack { attack_type: EnemyAttackType::Hit { damage: 7}, mov: Move::Peck } ));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_cerulean_after_jrtrainerf");

  // r.load("blue_cerulean_after_jrtrainerf");
  // r.run(WalkToSegment::new(5, 2)); // Misty
  // r.run(OverworldMoveSegment::collide(L));
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(9)); // Misty

  // r.run(Gen1ITKOSegment::new(Move::MegaPunch, EnemyAttack { attack_type: EnemyAttackType::HitFailed, mov: Move::Tackle } ));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITKOSegment::new(Move::MegaPunch, EnemyAttack { attack_type: EnemyAttackType::HitFailed, mov: Move::Tackle } ));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(4).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_cerulean_after_misty");

  // r.load("blue_cerulean_after_misty");
  // r.run(Gen1ITSkipTextsSegment::new(8).with_confirm_input(B)); // Misty after battle texts
  // r.run(Gen1ITSkipTextsSegment::new(1).with_confirm_input(B)); // Misty after battle texts
  // r.run(WalkToSegment::new(5, 13).into(OverworldInteractionResult::NoOverworldInput)); // Leave Gym
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(19, 17)); // Enter Center
  // r.run(WalkToSegment::new(3, 3)); // Enter Center
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(Gen1ITSkipTextsSegment::new(5).with_confirm_input(A)); // choose healing, disables IT
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // healing done
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // healing done
  // r.run(VerifyInputSegment::new("TextCommand0A").with_input(Input::B)); // skip delay
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // healing done
  // r.run(WalkToSegment::new(3, 7).into(OverworldInteractionResult::NoOverworldInput)); // Leave Center
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(A)); // TM11
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Booted up TM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // It contains // move // !
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // Teach BubbleBeam
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Squirtle
  //   r.run(OverrideMoveSegment::new(0).with_confirm_input(A)); // override Tackle
  //   r.run(MoveSegment::new(B)); // close
  //   r.run(MoveSegment::new(START)); // close
  // }
  // {
  //   r.run(WalkToSegment::new(13, 25)); // Enter Bike Shop
  //   r.run(OverworldMoveSegment::turn(L));
  //   r.run(WalkToSegment::new(6, 5)); // Enter Bike Shop
  //   r.run(WalkToSegment::new(6, 4)); // Enter Bike Shop
  //   r.run(OverworldMoveSegment::interact_with(1));
  //   r.run(SkipTextsSegment::new(2).with_confirm_input(A)); // Bike Shop
  //   r.run(MoveSegment::new(B)); // cancel
  //   r.run(MoveSegment::new(A)); // cancel (IT)
  //   r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  //   r.run(WalkToSegment::new(3, 6)); // Leave Bike Shop
  //   r.run(WalkToSegment::new(3, 7).into(OverworldInteractionResult::NoOverworldInput)); // Leave Bike Shop
  //   r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  //   r.run(OverworldMoveSegment::turn(L));
  // }
  // r.run(WalkToSegment::new(21, 7)); // Trigger Rival fight
  // r.run(WalkToSegment::new(21, 6).into(OverworldInteractionResult::NoOverworldInput)); // Trigger Rival fight
  // r.save("blue_cerulean_rival");

  // r.load("blue_cerulean_rival");
  // r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(8)); // Rival
  // r.run_merge(Gen1ITFightTurnSegment::new(Move::Bubble, false, 9, EnemyAttack { attack_type: EnemyAttackType::CriticalHit { damage: 15 }, mov: Move::Gust } ).with_exact_damage().with_expected_effect(Some(MoveEffectResult::NoEffect)));
  // r.run(Gen1ITKOSegment::new(Move::BubbleBeam, EnemyAttack { attack_type: EnemyAttackType::CriticalHit { damage: 15 }, mov: Move::QuickAttack } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(Gen1ITNextTrainerMonSegment::new().with_level_up());
  // r.save("blue_test");
  // r.load("blue_test");
  // r.run(Gen1ITOHKOSegment::new(Move::MegaPunch));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // { // Mega Punch fail into crit Vine Whip red bar
  //   r.run(SelectMoveSegment::new(Move::MegaPunch));
  //   r.run(DelaySegment::new(
  //     MoveSegment::with_metric(Input::A,
  //       ExpectedAIChooseMoveMetric { expected_move: Some(Move::VineWhip) }.and_then(Gen1NormalHitMetric::with_expected_max_damage(29, 46)).expect(FightTurnResult::Failed))));
  //   r.run(DelaySegment::new(
  //     MoveSegment::with_metric(Input::B, TrainerAIMetric {}.expect(TrainerAIAction::NoAction).and_then(Gen1NormalHitMetric::with_expected_max_damage(18, 30)).expect(FightTurnResult::CriticalHit { damage: 28 }))));
  //   r.run(MoveSegment::new(Input::A)); // critical hit
  //   r.run(MoveSegment::new(Input::B)); // very effective
  // }
  // r.run(Gen1ITOHKOSegment::new(Move::MegaPunch));
  // r.save("blue_test2");
  // r.load("blue_test2");
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(2).with_name_in_defeat_texts());
  // r.run(Gen1ITSkipTextsSegment::new(14).with_confirm_input(B)); // Rival after battle texts
  // r.save("blue_cerulean_after_rival");

  // r.load("blue_cerulean_after_rival");
  // r.run(WalkToSegment::new(21, -1)); // Enter Route 24
  // r.run(WalkToSegment::new(11, 32)); // Nugget 1
  // r.run(OverworldMoveSegment::interact_with(7));
  // r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(4)); // Nugget1
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_after_nuggetl");

  // r.load("blue_after_nuggetl");
  // r.run(WalkToSegment::new(10, 29)); // Nugget 2
  // r.run(OverworldMoveSegment::interact_with(6));
  // r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Nugget2
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_after_nugget2");

  // r.load("blue_after_nugget2");
  // r.run(WalkToSegment::new(11, 26)); // Nugget 3
  // r.run(OverworldMoveSegment::interact_with(5));
  // r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Nugget3
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_after_nugget3");

  // r.load("blue_after_nugget3");
  // r.run(WalkToSegment::new(10, 23)); // Nugget 4
  // r.run(OverworldMoveSegment::interact_with(4));
  // r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Nugget4
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITNextTrainerMonSegment::new().with_level_up());
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_after_nugget4");

  // r.load("blue_after_nugget4");
  // r.run(WalkToSegment::new(11, 20)); // Nugget 5
  // r.run(OverworldMoveSegment::interact_with(3));
  // r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Nugget5
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_after_nugget5");

  // r.load("blue_after_nugget5");
  // r.run(WalkToSegment::new(10, 15).into(OverworldInteractionResult::NoOverworldInput)); // Trigger Rocket fight
  // r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(15)); // Rocket
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.run(Gen1ITSkipTextsSegment::new(3).with_confirm_input(B)); // Rocket after battle texts
  // r.save("blue_after_nugget6");

  // r.load("blue_after_nugget6");
  // r.run(WalkToSegment::new(20, 9)); // Route 25
  // { // 88223
  // r.run(WalkToSegment::new(14, 7)); // Hiker
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoAction))); // advance map script (abSs buttons allowed)
  // r.run(Gen1ITSkipTextsSegment::new(2).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 209 }))); // initiate Hiker fight
  // r.run(Gen1ITStartTrainerBattleSegment::new()); // Hiker
  // r.run(Gen1ITOHKOSegment::new(Move::Bubble));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts().with_level_up());
  // }
  // { // 88517
  //   r.run(WalkToSegment::new(8, 6)); // Hiker
  //   r.run(WalkToSegment::new(8, 5)); // Hiker
  //   r.run(OverworldMoveSegment::interact_with(7));
  //   r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Hiker
  //   r.run(Gen1ITOHKOSegment::new(Move::MegePunch));
  //   r.run(Gen1ITNextTrainerMonSegment::new());
  //   r.run(Gen1ITOHKOSegment::new(Move::Bubble));
  //   r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts().with_level_up());
  // }
  // r.save("blue_route25_after_hiker_b");

  // r.load("blue_route25_after_hiker_b");
  // r.run(WalkToSegment::new(18, 7)); // Lass1
  // r.run(OverworldMoveSegment::collide(D));
  // r.run(OverworldMoveSegment::interact_with(4));
  // r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Lass1
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_route25_after_lass1_b");

  // r.load("blue_route25_after_lass1_b");
  // r.run(WalkToSegment::new(24, 6).into(OverworldInteractionResult::NoOverworldInput)); // Jr Trainer M
  // // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoAction))); // advance map script (abSs buttons allowed)
  // r.run(Gen1ITSkipTextsSegment::new(2).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 205 }))); // initiate Jr Trainer M fight
  // r.run(Gen1ITStartTrainerBattleSegment::new()); // Jr Trainer M
  // r.run(Gen1ITOHKOSegment::new(Move::Bubble));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_route25_after_jrtrainerm_b");

  // r.load("blue_route25_after_jrtrainerm_b");
  // r.run(WalkToSegment::new(35, 4)); // Lass2
  // r.run(WalkToSegment::new(36, 4)); // Lass2
  // r.run(OverworldMoveSegment::interact_with(6));
  // r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Lass2
  // r.run(Gen1ITOHKOSegment::new(Move::MegaPunch));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITOHKOSegment::new(Move::Bubble));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITOHKOSegment::new(Move::MegaPunch));
  // {
  //   // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // }
  // {
    // r.save("blue_test");
  //   r.load("blue_test");
  //   r.run(Gen1ITSkipTextsSegment::new(9).with_confirm_input(A)); // fainted, xp, level, override move
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(D)); //forget Water Gun
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D | A));
  //   r.run(TextSegment::new().expect_conflicting_inputs()); // 1, 2 and
  //   r.run(VerifyInputSegment::new("TextCommand0A").with_input(B)); // skip delay
  //   r.run(TextSegment::new().expect_conflicting_inputs()); // 1, 2 and
  //   r.run(VerifyInputSegment::new("TextCommand0A").with_input(B)); // skip delay
  //   r.run(MoveSegment::new(A)); // Cofirm text box
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // mon // forgot // move // .
  //   r.run(SkipTextsSegment::new(1)); // and
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3).with_confirm_input(B)); // mon // learned // move // !
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // I defeated U
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // after battle text
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // got money
  // }
  // r.save("blue_route25_after_lass2_b");

  // r.load("blue_route25_after_lass2_b");
  // r.run(WalkToSegment::new(45, 3)); // Enter Bill's House
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(4, 5)); // Bill
  // r.run(WalkToSegment::new(5, 5)); // Bill
  // r.run(OverworldMoveSegment::interact_with(1));
  // // {
  // //   r.run(Gen1ITSkipTextsSegment::new(11).with_confirm_input(A)); // agree to help (cancels IT)
  // // }
  // {
  //   r.run(SkipTextsSegment::new(10).with_confirm_input(B));
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // agree to help
  // }
  // r.run(SkipTextsSegment::new(4).with_confirm_input(B));
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(1, 5)); // Desk
  // r.run(OverworldMoveSegment::collide(U));
  // r.run(MoveSegment::new(A)); // interact with desk
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B));
  // r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(3, 4)); // Bill
  // r.run(OverworldMoveSegment::collide(R));
  // r.run(OverworldMoveSegment::interact_with(2));
  // r.run(SkipTextsSegment::new(8).with_confirm_input(B));
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B));
  // r.run(SkipTextsSegment::new(9).with_confirm_input(B));
  // r.save("blue_test");
  // r.load("blue_test");
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(VerifyInputSegment::new("CloseStartMenu").with_input(B)); // don't main menu open
  //   r.run(OverworldMoveSegment::warp());
  // }
  // {
  //   r.run(WalkToSegment::new(13, 25)); // Enter Bike Shop
  //   r.run(OverworldMoveSegment::turn(L));
  //   r.run(WalkToSegment::new(6, 5)); // Enter Bike Shop
  //   r.run(WalkToSegment::new(6, 4)); // Enter Bike Shop
  //   r.run(OverworldMoveSegment::interact_with(1));
  //   r.run(SkipTextsSegment::new(2).with_confirm_input(A)); // Bike Shop
  //   r.run(MoveSegment::new(B)); // cancel
  //   r.run(MoveSegment::new(A)); // cancel (IT)
  //   r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  //   r.run(WalkToSegment::new(3, 6)); // Leave Bike Shop
  //   r.run(WalkToSegment::new(3, 7).into(OverworldInteractionResult::NoOverworldInput)); // Leave Bike Shop
  //   r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  //   r.run(OverworldMoveSegment::turn(L));
  // }
  // r.run(WalkToSegment::new(27, 11)); // Enter house
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(3, 0)); // Leave house
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(30, 9).into(OverworldInteractionResult::NoOverworldInput)); // Trigger Rocket fight
  // // {
  // //   r.run(SkipTextsSegment::new(4).with_confirm_input(B));
  // //   r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 230 }))); // initiate rival fight
  // //   r.run(StartTrainerBattleSegment::new()); // Rocket
  // //   r.run(OHKOSegment::new(Move::BubbleBeam));
  // //   r.save("blue_test2");
  // //   r.load("blue_test2");
  // //   r.run(NextTrainerMonSegment::new().with_level_up().with_override_move(Move::WaterGun));
  // //   r.run(OHKOSegment::new(Move::Bite));
  // //   r.run(EndTrainerBattleSegment::with_defeat_texts(2).with_name_in_defeat_texts());
  // //   r.run(SkipTextsSegment::new(3).with_confirm_input(B)); // Rocket after battle texts
  // // }
  // {
  //   r.run(Gen1ITSkipTextsSegment::new(4).with_confirm_input(B));
  //   r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 230 }))); // initiate rival fight
  //   r.run(Gen1ITStartTrainerBattleSegment::new()); // Rocket
  //   r.run(Gen1ITOHKOSegment::new(Move::BubbleBeam));
  //   r.save("blue_test2");
  //   r.load("blue_test2");
  //   r.run(Gen1ITNextTrainerMonSegment::new());
  //   r.run(Gen1ITOHKOSegment::new(Move::Bite));
  //   r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(2).with_name_in_defeat_texts());
  //   r.run(Gen1ITSkipTextsSegment::new(3).with_confirm_input(B)); // Rocket after battle texts
  // }
  // r.save("blue_after_cerulean_rocket_b");

  // r.load("blue_after_cerulean_rocket_b");
  // r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // // { // 102609 - 100387 = 2222
  // //   r.run(WalkToSegment::new(33, 18));
  // //   r.run(OverworldMoveSegment::jump_ledge(D)); // Jump ledge
  // //   r.run(WalkToSegment::new(13, 25)); // Enter Bike Shop
  // //   r.run(OverworldMoveSegment::turn(L));
  // //   r.run(WalkToSegment::new(6, 5)); // Enter Bike Shop
  // //   r.run(WalkToSegment::new(6, 4)); // Enter Bike Shop
  // //   r.run(OverworldMoveSegment::interact_with(1));
  // //   r.run(SkipTextsSegment::new(2).with_confirm_input(A)); // Bike Shop
  // //   r.run(MoveSegment::new(B)); // cancel
  // //   r.run(MoveSegment::new(A)); // cancel (IT)
  // //   r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // //   r.run(WalkToSegment::new(3, 6)); // Leave Bike Shop
  // //   r.run(WalkToSegment::new(3, 7).into(OverworldInteractionResult::NoOverworldInput)); // Leave Bike Shop
  // //   r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // //   r.run(OverworldMoveSegment::turn(L));
  // //   r.run(WalkToSegment::new(27, 11)); // Enter house
  // //   r.run(OverworldMoveSegment::turn(D));
  // //   r.run(WalkToSegment::new(3, 0)); // Leave house
  // //   r.run(OverworldMoveSegment::turn(L));
  // // } // b: 101238
  // r.run(WalkToSegment::new(28, 36)); // Enter Route 5
  // r.run(WalkToSegment::new(17, 27)); // Underground
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(4, 4)); // Underground
  // r.run(OverworldMoveSegment::turn(D));
  // // r.run(WalkToSegment::new(4, 4)); // Underground
  // // r.run(MoveSegment::new(A)); // pick up item // 102826 - 102799 = 27
  // // r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // // { // 103007 - 102799 = 208
  // //   r.run(WalkToSegment::new(4, 32)); // Underground
  // //   r.run(WalkToSegment::new(4, 33)); // Underground
  // //   r.run(MoveSegment::new(A)); // pick up item // 102826 - 102799 = 27
  // //   r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // // }
  // r.run(WalkToSegment::new(2, 40)); // Underground
  // r.run(WalkToSegment::new(2, 41)); // Underground
  // r.run(WalkToSegment::new(4, 7).into(OverworldInteractionResult::NoOverworldInput)); // Leave Bike Shop
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.run(OverworldMoveSegment::turn(U));
  // r.save("blue_route6");

  // r.load("blue_route6");
  // r.run(WalkToSegment::new(11, 28)); // F
  // r.run(WalkToSegment::new(11, 29)); // F
  // r.run(OverworldMoveSegment::interact_with(5));
  // r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(1)); // F
  // r.run(Gen1ITOHKOSegment::new(Move::Bite));
  // r.run(Gen1ITNextTrainerMonSegment::new()/*.with_level_up()*/);
  // r.run(Gen1ITOHKOSegment::new(Move::Bite));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITOHKOSegment::new(Move::Bite));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.run(WalkToSegment::new(10, 30)); // M
  // r.run(WalkToSegment::new(10, 31)); // M
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoAction))); // advance map script (abSs buttons allowed)
  // r.run(Gen1ITSkipTextsSegment::new(1).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 205 }))); // initiate M fight
  // r.run(Gen1ITStartTrainerBattleSegment::new()); // M
  // r.run(Gen1ITOHKOSegment::new(Move::Bite));
  // r.run(Gen1ITNextTrainerMonSegment::new().with_level_up());
  // r.run(Gen1ITOHKOSegment::new(Move::Bite));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.run(WalkToSegment::new(9, 36)); // Vermilion City
  // r.save("blue_vermilion");

  // r.load("blue_vermilion");
  // r.run(WalkToSegment::new(18, 30).into(OverworldInteractionResult::NoOverworldInput)); // Dock
  // r.run(Gen1ITSkipTextsSegment::new(4).with_confirm_input(B)); // Dock
  // r.run(WalkToSegment::new(18, 31).into(OverworldInteractionResult::NoOverworldInput)); // Dock
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.run(WalkToSegment::new(14, 2).into(OverworldInteractionResult::NoOverworldInput)); // Dock
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(7, 7)); // Evade NPC
  // r.run(WalkToSegment::new(2, 6)); // 1F
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(3, 11)); // Avoid stairs
  // r.run(WalkToSegment::new(37, 9)); // Rival encounter
  // r.run(WalkToSegment::new(37, 8).into(OverworldInteractionResult::NoOverworldInput)); // Rival encounter
  // r.save("blue_ssanne_rival");

  // r.load("blue_ssanne_rival");
  // r.run(Gen1ITStartTrainerBattleSegment::new().with_pre_battle_texts(7)); // Rival
  // r.run(Gen1ITOHKOSegment::new(Move::MegaPunch));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITOHKOSegment::new(Move::Bite));
  // r.run(Gen1ITNextTrainerMonSegment::new());
  // r.run(Gen1ITKOSegment::new(Move::Bite, EnemyAttack { attack_type: EnemyAttackType::Hit { damage: 14 }, mov: Move::Confusion } ));
  // r.run(Gen1ITNextTrainerMonSegment::new().with_level_up());
  // r.save("blue_test");
  // r.load("blue_test");
  // { // Bite flinch
  //   r.run(SelectMoveSegment::new(Move::Bite));
  //   r.run(DelaySegment::new( MoveSegment::with_metric(Input::A, 
  //     Gen1NormalHitMetric::with_expected_max_damage(25, 40).expect(FightTurnResult::Hit { damage: 23 }).and_then(MoveEffectMetric {}.expect(MoveEffectResult::Success)))));
  //   r.run(MoveSegment::new(Input::B)); // enemy flinched
  // }
  // r.run(Gen1ITOHKOSegment::new(Move::Bite));
  // r.run(Gen1ITEndTrainerBattleSegment::with_defeat_texts(3).with_name_in_defeat_texts());
  // r.run(Gen1ITSkipTextsSegment::new(5).with_confirm_input(B)); // Rival after battle texts
  // r.save("blue_ssanne_after_rival");

  // r.load("blue_ssanne_after_rival");
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(36, 4)); // 2F
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(4, 4)); // Captain
  // r.run(WalkToSegment::new(4, 3)); // Captain
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(Gen1ITSkipTextsSegment::new(13).with_confirm_input(B)); // Captain
  // r.run(WalkToSegment::new(0, 7)); // 1F
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(2, 4)); // 0F
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(7, 7)); // Evade NPC
  // r.run(WalkToSegment::new(26, 0).into(OverworldInteractionResult::NoOverworldInput)); // Evade NPC
  // r.run(DelaySegment::new(MoveSegment::with_metric(U, VermilionFirstTrashCanMetric {}.debug_print().expect(4))));
  // r.run(MoveSegment::new(U)); // can't move this frame
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(15, 16)); // Cut bush
  // r.run(WalkToSegment::new(15, 17)); // Cut bush
  // r.save("blue_test");
  // r.load("blue_test");
  // {
  //   r.run(OverworldMoveSegment::open_main_menu()); // Cancels IT
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(A)); // HM01
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Booted up TM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // It contains // move // !
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // Teach Cut
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // Sandshrew
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3).with_confirm_input(A)); // Learned Cut
  //   r.run(MoveSegment::new(B)); // Back to main menu
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // Mon
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Sandshrew
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Cut
  //   r.run(MoveSegment::new(B)); // Hacked away
  // }
  // r.run(WalkToSegment::new(12, 19)); // Enter Gym
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(4, 9)); // Can 1
  // r.run(OverworldMoveSegment::collide(L));
  // r.run(DelaySegment::new(MoveSegment::with_metric(A, VermilionSecondTrashCanMetric {}.debug_print().expect(7))));
  // r.run(SkipTextsSegment::new(4).with_confirm_input(B)); // First switch
  // r.run(OverworldMoveSegment::collide(R));
  // r.run(MoveSegment::new(A));
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Second switch
  // r.run(WalkToSegment::new(5, 3)); // Surge
  // r.run(WalkToSegment::new(5, 2)); // Surge
  // r.save("blue_surge");

  // r.load("blue_surge");
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(10)); // Surge
  // r.run(KOSegment::new(Move::MegaPunch, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::Screech } ));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::Bite));
  // r.run(NextTrainerMonSegment::new());
  // r.save("blue_test");
  // r.load("blue_test");
  // r.run_merge(FightTurnSegment::new(Move::BubbleBeam, true, 42, EnemyAttack { attack_type: EnemyAttackType::HitFailed, mov: Move::Thunderbolt } ).with_expected_effect(Some(MoveEffectResult::Success)));
  // r.run(OHKOSegment::new(Move::Bite));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(3).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_after_surge");

  // r.load("blue_after_surge");
  // r.run(SkipTextsSegment::new(5).with_confirm_input(B)); // Surge after fight texts
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Got TM
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Surge after fight texts
  // r.run(WalkToSegment::new(5, 17).into(OverworldInteractionResult::NoOverworldInput)); // Dock
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(15, 19)); // Cut bush
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Mon
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // Sandshrew
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Cut
  //   r.run(MoveSegment::new(B)); // Hacked away
  // }
  // r.run(WalkToSegment::new(9, 13)); // Fanclub
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(1, 1)); // Fanclub
  // r.run(WalkToSegment::new(2, 1)); // Fanclub
  // r.run(OverworldMoveSegment::interact_with(5));
  // r.run(SkipTextsSegment::new(6).with_confirm_input(B)); // Fanclub
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // hear about mons
  // r.run(SkipTextsSegment::new(19).with_confirm_input(B)); // Fanclub
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Got Voucher
  // r.run(SkipTextsSegment::new(5).with_confirm_input(B)); // Fanclub
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(VerifyInputSegment::new("CloseStartMenu").with_input(B)); // don't main menu open
  //   r.run(OverworldMoveSegment::warp());
  // }
  // r.save("blue_test");
  // r.load("blue_test");
  // r.run(WalkToSegment::new(13, 25)); // Enter Bike Shop
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(6, 5)); // Enter Bike Shop
  // r.run(WalkToSegment::new(6, 4)); // Enter Bike Shop
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(SkipTextsSegment::new(5).with_confirm_input(B)); // Get Bike
  // r.run(WalkToSegment::new(3, 6)); // Leave Bike Shop
  // r.run(WalkToSegment::new(3, 7).into(OverworldInteractionResult::NoOverworldInput)); // Leave Bike Shop
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(SELECT)); // swap
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(SELECT)); // bike
  //   r.run(MoveSegment::new(U|R));
  //   r.run(MoveSegment::new(U|L));
  //   r.run(MoveSegment::new(U|R));
  //   r.run(MoveSegment::new(U|L));
  //   r.run(MoveSegment::new(U|R));
  //   r.run(MoveSegment::new(U|L));
  //   r.run(MoveSegment::new(U|R));
  //   r.run(MoveSegment::new(U|L));
  //   r.run(MoveSegment::new(U|R));
  //   r.run(MoveSegment::new(A)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(19, 26)); // Bush
  // r.run(WalkToSegment::new(19, 27)); // Bush
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // sandshrew
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // cut
  //   r.run(MoveSegment::new(B)); // Hacked away
  // }
  // r.run(WalkToSegment::new(40, 17)); // Route 9
  // r.run(WalkToSegment::new(4, 8)); // Bush
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // sandshrew
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // cut
  //   r.run(MoveSegment::new(B)); // Hacked away
  // }
  // r.run(WalkToSegment::new(13, 8)); // JrTrainerF
  // r.run(WalkToSegment::new(13, 9)); // JrTrainerF
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // JrTrainerF
  // r.run(OHKOSegment::new(Move::Bite));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::Bite));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::Bite));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::Bite));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_route9_after_jrtrainerf");

  // r.load("blue_route9_after_jrtrainerf");
  // r.run(WalkToSegment::new(12, 10));
  // r.run(OverworldMoveSegment::jump_ledge(D)); // Jump ledge
  // r.run(WalkToSegment::new(40, 10));
  // r.run(WalkToSegment::new(40, 9)); // Bugcatcher
  // r.run(OverworldMoveSegment::interact_with(9));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Bugcatcher
  // r.run(OHKOSegment::new(Move::Bite));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::Bite));
  // r.run(NextTrainerMonSegment::new().with_level_up());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_route9_after_bugcatcher");

  // r.load("blue_route9_after_bugcatcher");
  // r.run(WalkToSegment::new(51, 4));
  // r.run(OverworldMoveSegment::jump_ledge(D)); // Jump ledge
  // r.run(WalkToSegment::new(60, 8)); // Route 10
  // r.run(WalkToSegment::new(8, 17)); // Rock Tunnel
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(23, 6));
  // r.run(WalkToSegment::new(23, 7)); // Pokemaniac
  // r.run(OverworldMoveSegment::interact_with(4));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Pokemaniac
  // r.run(OHKOSegment::new(Move::Bubble));
  // r.run(NextTrainerMonSegment::new());
  // { // Bite flinch
  //   r.run(SelectMoveSegment::new(Move::Bite));
  //   r.run(DelaySegment::new( MoveSegment::new(A).seq(TextSegment::with_metric( 
  //     Gen1NormalHitMetric::with_expected_max_damage(25, 41).expect(FightTurnResult::CriticalHit { damage: 40 })).with_skip_ends(4).with_unbounded_buffer())));
  //   r.run(TextSegment::new().with_allowed_end_inputs(A).with_unbounded_buffer()); // critical hit
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, MoveEffectMetric {}.expect(MoveEffectResult::Success))));
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // flinched
  // }
  // r.run(OHKOSegment::new(Move::Bite));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_rock_tunnel_after_pokemaniac1");

  // r.load("blue_rock_tunnel_after_pokemaniac1");
  // r.run(WalkToSegment::new(37, 3)); // B1F
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(27, 30)); // Pokemaniac
  // r.run(OverworldMoveSegment::interact_with(8));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Pokemaniac
  // { // Bite flinch
  //   r.run(SelectMoveSegment::new(Move::Bite));
  //   r.run(DelaySegment::new( MoveSegment::new(A).seq(TextSegment::with_metric( 
  //     Gen1NormalHitMetric::with_expected_max_damage(24, 38).expect(FightTurnResult::CriticalHit { damage: 37 })).with_skip_ends(4).with_unbounded_buffer())));
  //   r.run(TextSegment::new().with_allowed_end_inputs(A).with_unbounded_buffer()); // critical hit
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, MoveEffectMetric {}.expect(MoveEffectResult::Success))));
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // flinched
  // }
  // r.run(OHKOSegment::new(Move::MegaPunch));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_rock_tunnel_after_pokemaniac2");

  // r.load("blue_rock_tunnel_after_pokemaniac2");
  // r.run(WalkToSegment::new(14, 30)); // Lass
  // r.run(WalkToSegment::new(14, 29)); // Lass
  // r.run(OverworldMoveSegment::interact_with(6));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Lass
  // r.run(OHKOSegment::new(Move::MegaPunch));
  // r.run(NextTrainerMonSegment::new().with_level_up());
  // r.run(OHKOSegment::new(Move::Bite));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_rock_tunnel_after_lass");

  // r.load("blue_rock_tunnel_after_lass");
  // r.run(WalkToSegment::new(27, 3)); // 1F
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(17, 11)); // B1F
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(8, 10)); // Hiker
  // r.run(WalkToSegment::new(7, 10)); // Hiker
  // r.run(OverworldMoveSegment::interact_with(2));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Hiker
  // r.run(OHKOSegment::new(Move::Bubble));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::Bubble));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::Bubble));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_rock_tunnel_after_hiker");

  // r.load("blue_rock_tunnel_after_hiker");
  // r.run(WalkToSegment::new(3, 3)); // 1F
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(24, 24)); // JrTrainerF
  // r.run(WalkToSegment::new(23, 24)); // JrTrainerF
  // r.run(OverworldMoveSegment::interact_with(6));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // JrTrainerF
  // r.run(OHKOSegment::new(Move::MegaPunch));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::Bite));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::MegaPunch));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_rock_tunnel_after_jrtrainerf");

  // r.load("blue_rock_tunnel_after_jrtrainerf");
  // r.run(WalkToSegment::new(15, 33)); // leave
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(9, 72)); // Lavender Town
  // r.run(WalkToSegment::new(-1, 8)); // Route 8
  // r.run(WalkToSegment::new(47, 13)); // Juggler
  // r.run(OverworldMoveSegment::interact_with(8));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Juggler
  // r.run(OHKOSegment::new(Move::MegaPunch));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::Bite));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_route_8_after_juggler");

  // r.load("blue_route_8_after_juggler");
  // r.run(WalkToSegment::new(13, 3)); // underground
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(4, 4)); // underground
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(A|B)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(21, 3)); // Elixer
  // r.run(WalkToSegment::new(21, 4)); // Elixer
  // r.run(MoveSegment::new(A)); // Elixer
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Elixer
  // r.run(WalkToSegment::new(2, 4)); // underground
  // r.run(WalkToSegment::new(2, 5)); // underground
  // r.run(WalkToSegment::new(4, 7).into(OverworldInteractionResult::NoOverworldInput)); // underground
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(A|B)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(-1, 3)); // Celadon City
  // r.save("blue_celadon");

  // r.load("blue_celadon");
  // r.run(WalkToSegment::new(10, 13)); // Mart
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(12, 1)); // 2F
  // r.run(OverworldMoveSegment::turn(U));
  // { // buy TM05 x2
  //   r.run(WalkToSegment::new(9, 3));
  //   r.run(WalkToSegment::new(8, 3));
  //   r.run(OverworldMoveSegment::interact_with(2));
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Buy
  //   r.run(TextSegment::new()); // Take a look
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(D | L));
  //   r.run(MoveSegment::new(D | R));
  //   r.run(MoveSegment::new(D | L));
  //   r.run(MoveSegment::new(D | R));
  //   r.run(MoveSegment::new(D | L));
  //   r.run(MoveSegment::new(A)); // TM05
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A));
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // buy TM05 x2
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // buy TM05 x2
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // buy TM05 x2
  //   r.run(MoveSegment::new(B)); // Close
  //   r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Close
  // }
  // r.run(WalkToSegment::new(16, 1)); // 3F
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(12, 1)); // 4F
  // r.run(OverworldMoveSegment::turn(U));
  // { // buy Doll x2
  //   r.run(WalkToSegment::new(5, 5));
  //   r.run(OverworldMoveSegment::collide(D));
  //   r.run(OverworldMoveSegment::interact_with(1));
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Buy
  //   r.run(TextSegment::new().with_allowed_end_inputs(B)); // Take a look
  //   r.run(MoveSegment::new(A)); // Pokedoll
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A));
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // buy Pokedoll x2
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // buy Pokedoll x2
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // buy Pokedoll x2
  //   r.run(MoveSegment::new(B)); // Close
  //   r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Close
  // }
  // r.run(WalkToSegment::new(16, 1)); // 5F
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(12, 1)); // 6F
  // r.run(OverworldMoveSegment::turn(U));
  // r.save("blue_test");
  // r.load("blue_test");
  // r.run(WalkToSegment::new(12, 3));
  // r.run(OverworldMoveSegment::collide(U));
  // r.run(OverworldMoveSegment::interact_with(5));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Vanding Machine text
  // r.run(MoveSegment::new(A)); // Fresh Water
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // Vanding Machine text
  // r.run(OverworldMoveSegment::interact_with(5));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Vanding Machine text // buy: 219-3
  // r.run(MoveSegment::new(D|A)); // Soda Pop
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B).with_unbounded_buffer()); // Vanding Machine text
  // r.run(WalkToSegment::new(7, 4).with_unbounded_buffer());
  // r.run(WalkToSegment::new(6, 4).with_unbounded_buffer());
  // r.run(DelaySegment::new(MoveSegment::with_metric(A, OverworldInteractionMetric {}.expect(OverworldInteractionResult::DisplayText { id: 2 }))));
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // give drink
  // r.run(MoveSegment::new(NIL)); // give drink
  // r.run(MoveSegment::new(A)); // give drink
  // r.run(SkipTextsSegment::new(4).with_confirm_input(B)); // Yay
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Received TM13
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // contains Ice Beam
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Can freeze
  // r.run(WalkToSegment::new(15, 2)); // 5F
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(16, 1)); // 4F
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(12, 1)); // 3F
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(16, 1)); // 2F
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(12, 1)); // 1F
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(16, 6));
  // r.run(WalkToSegment::new(16, 7).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.save("blue_test2");
  // r.load("blue_test2");
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(A|B)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(-1, 18));
  // r.run(WalkToSegment::new(34, 10));
  // r.run(OverworldMoveSegment::collide(U));
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // sandshrew
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // cut
  //   r.run(MoveSegment::new(B)); // Hacked away
  // }
  // r.run(WalkToSegment::new(25, 4));
  // r.run(WalkToSegment::new(24, 4).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(L)); // edge warp
  // r.run(WalkToSegment::new(0, 2).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(L)); // edge warp
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(A|B)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(7, 5)); // Fly House
  // r.run(WalkToSegment::new(2, 4)); // Fly
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(SkipTextsSegment::new(5).with_confirm_input(B)); // Fly
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(2, 7).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp // 154886
  // r.save("blue_test3");
  // r.load("blue_test3");
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(SELECT)); // swap balls
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(SELECT)); // swap poke doll
  //   r.run(MoveSegment::new(U|A)); // TM05
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Booted up TM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // It contains // move // !
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // Teach Mega Kick
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Wartortle
  //   r.run(OverrideMoveSegment::new(2).with_confirm_input(B)); // override Bubble
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(A)); // HM02
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Booted up HM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // It contains // move // !
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // Teach Fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Pidgey
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3).with_confirm_input(B)); // Learned Fly
  //   r.run(MoveSegment::new(U|A)); // Ice Beam
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Booted up TM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // It contains // move // !
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // Teach Ice Beam
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // Wartortle
  //   r.run(OverrideMoveSegment::new(1).with_confirm_input(A)); // override Mega Punch
  //   r.run(MoveSegment::new(B)); // cancel
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // pidgey
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Lavender
  //   r.run(OverworldMoveSegment::warp());
  // }
  // r.save("blue_test4");
  // r.load("blue_test4");
  // r.run(WalkToSegment::new(14, 5)); // Enter Tower
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(18, 9)); // 2F
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(16, 5)); // Rival
  // r.run(WalkToSegment::new(15, 5).into(OverworldInteractionResult::NoOverworldInput)); // Rival
  // r.run(SkipTextsSegment::new(6).with_confirm_input(B)); // rival pre-fight text
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 242 }))); // initiate rival fight
  // r.run(StartTrainerBattleSegment::new());
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(NextTrainerMonSegment::new());
  // { // Bite flinch
  //   r.run(SelectMoveSegment::new(Move::Bite));
  //   r.run(DelaySegment::new( MoveSegment::new(A).seq(TextSegment::with_metric( 
  //     Gen1NormalHitMetric::with_expected_max_damage(25, 40).expect(FightTurnResult::CriticalHit { damage: 39 })).with_skip_ends(4).with_unbounded_buffer())));
  //   r.run(TextSegment::new().with_allowed_end_inputs(A).with_unbounded_buffer()); // critical hit
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, MoveEffectMetric {}.expect(MoveEffectResult::Success))));
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // flinched
  // }
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.save("blue_test5");
  // r.load("blue_test5");
  // r.run(NextTrainerMonSegment::new().with_level_up().with_skip_learning_move());
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2).with_name_in_defeat_texts());
  // r.save("blue_tower_after_rival");

  // r.load("blue_tower_after_rival");
  // r.run(SkipTextsSegment::new(10).with_confirm_input(B)); // rival post-fight text
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(3, 9)); // 3F
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(18, 9)); // 4F
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(17, 7)); // Channeler
  // r.run(WalkToSegment::new(16, 7)); // Channeler
  // r.run(OverworldMoveSegment::interact_with(2));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Channeler
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_tower_after_channeler1");

  // r.load("blue_tower_after_channeler1");
  // r.run(WalkToSegment::new(3, 9).with_buffer_size(64)); // 5F
  // r.run(OverworldMoveSegment::turn(D).with_buffer_size(64));
  // r.run(WalkToSegment::new(11, 9).into(OverworldInteractionResult::NoOverworldInput).with_buffer_size(64)); // Heal pad
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // heal pad // 166054
  // r.run(WalkToSegment::new(18, 9)); // 6F
  // // r.debug_write_memory(0xD16D, 1); // Set to 1HP
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(15, 5)); // Channeler
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoAction))); // advance map script (abSs buttons allowed)
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 245 }))); // initiate Channeler fight
  // r.run(StartTrainerBattleSegment::new()); // M
  // r.save("blue_test");
  // r.load("blue_test");
  // { // Lick turn
  //   r.run(SelectMoveSegment::new(Move::Bite));
  //   r.run(DelaySegment::new(MoveSegment::with_metric(A, ExpectedAIChooseMoveMetric { expected_move: Some(Move::Lick) })));
  //   r.run(TextSegment::new().with_skip_ends(4).with_unbounded_buffer()); // A used Bite
  //   r.run(TextSegment::new().with_allowed_end_inputs(A).with_unbounded_buffer()); // didn't affect
  //   r.run(DelaySegment::new(MoveSegment::new(B).seq(TextSegment::with_metric(Gen1NormalHitMetric { expected_max_damage: 4, expected_max_crit_damage: 7 }.expect(FightTurnResult::Hit { damage: 4 }).and_then(MoveEffectMetric {}.expect(MoveEffectResult::NoEffect))).with_skip_ends(4).with_allowed_end_inputs(B)))); // Gastly used Lick
  // }
  // { // Night Shade turn
  //   r.run(SelectMoveSegment::new(Move::Bite));
  //   r.run(DelaySegment::new(MoveSegment::with_metric(A, ExpectedAIChooseMoveMetric { expected_move: Some(Move::NightShade) })));
  //   r.run(TextSegment::new().with_skip_ends(4)); // A used Bite
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(0).with_confirm_input(B)); // didn't affect
  //   r.run(TextSegment::with_metric(Gen1MoveSuccessMetric {}.expect(FightTurnResult::Succeeded)).with_skip_ends(4).with_allowed_end_inputs(B)); // Gastly used Night Shade
  // }
  // { // Night Shade turn
  //   r.run(SelectMoveSegment::new(Move::Bite));
  //   r.run(DelaySegment::new(MoveSegment::with_metric(A, ExpectedAIChooseMoveMetric { expected_move: Some(Move::NightShade) })));
  //   r.run(TextSegment::new().with_skip_ends(4)); // A used Bite
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(0).with_confirm_input(B)); // didn't affect
  //   r.run(TextSegment::with_metric(Gen1MoveSuccessMetric {}.expect(FightTurnResult::Succeeded)).with_skip_ends(4).with_allowed_end_inputs(B)); // Gastly used Night Shade
  // }
  // { // Night Shade turn
  //   r.run(SelectMoveSegment::new(Move::Bite));
  //   r.run(DelaySegment::new(MoveSegment::with_metric(A, ExpectedAIChooseMoveMetric { expected_move: Some(Move::NightShade) })));
  //   r.run(TextSegment::new().with_skip_ends(4)); // A used Bite
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(0).with_confirm_input(B)); // didn't affect
  //   r.run(TextSegment::with_metric(Gen1MoveSuccessMetric {}.expect(FightTurnResult::Succeeded)).with_skip_ends(4).with_allowed_end_inputs(B)); // Gastly used Night Shade
  // }
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts()); // 167850 - 167752 = 98 // 168174(324) // 168488(314) // 168800(312) // 169035 (235)
  // r.save("blue_tower_after_channeler2");

  // r.load("blue_tower_after_channeler2");
  // // r.debug_write_memory(0xD16D, 1); // Set to 1HP
  // r.run(WalkToSegment::new(11, 5)); // Channeler
  // r.run(WalkToSegment::new(10, 5)); // Channeler
  // r.run(OverworldMoveSegment::interact_with(2));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Channeler
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts()); // 169439 - 169341 = 98
  // r.save("blue_tower_after_channeler3");

  // r.load("blue_tower_after_channeler3");
  // // r.debug_write_memory(0xD16D, 1); // Set to 1HP
  // r.run(WalkToSegment::new(6, 6)); // Rare Candy
  // r.run(WalkToSegment::new(6, 7)); // Rare Candy
  // r.run(OverworldMoveSegment::interact_with(4));
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Rare Candy
  // r.run(WalkToSegment::new(10, 16).into(OverworldInteractionResult::NoOverworldInput)); // Ghost
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Ghost
  // r.run(MoveSegment::new(NIL)); // Marowak encounter
  // r.run(SkipTextsSegment::new(1).with_skip_ends(1).with_confirm_input(B)); // Wild // Ghost // appeared
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Can't be ID'd
  // r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(B));
  // r.run(MoveSegment::new(D|A)); // Item
  // r.run(MoveSegment::new(D|L));
  // r.run(MoveSegment::new(D|R|A)); // Doll
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Use doll
  // r.run(SkipTextsSegment::new(4).with_confirm_input(B)); // Marowak gone // 170806 - 170754 = 52
  // r.save("blue_tower_after_marowak");

  // r.load("blue_tower_after_marowak");
  // // r.debug_write_memory(0xD16D, 1); // Set to 1HP
  // r.run(WalkToSegment::new(9, 16)); // 7F
  // r.run(OverworldMoveSegment::turn(D));
  // r.run(WalkToSegment::new(10, 11)); // Rocket
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoAction))); // advance map script (abSs buttons allowed)
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 230 }))); // initiate Rocket fight
  // r.run(StartTrainerBattleSegment::new()); // Rocket
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // not forget this // 173461 - 173159 = 302
  // r.save("blue_tower_after_rocket1");

  // r.load("blue_tower_after_rocket1");
  // // r.debug_write_memory(0xD16D, 1); // Set to 1HP
  // r.run(WalkToSegment::new(10, 9).into(OverworldInteractionResult::NoOverworldInput)); // Rocket
  // r.run(SkipTextsSegment::new(4).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 230 }))); // initiate Rocket fight
  // r.run(StartTrainerBattleSegment::new()); // Rocket
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new().with_level_up());
  // r.run(OHKOSegment::new(Move::MegaKick));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.run(SkipTextsSegment::new(3).with_confirm_input(B)); // not forget this // 176154 - 175800 = 354
  // r.save("blue_tower_after_rocket2");

  // r.load("blue_tower_after_rocket2");
  // // r.debug_write_memory(0xD16D, 1); // Set to 1HP
  // r.run(WalkToSegment::new(10, 7)); // Rocket
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoAction))); // advance map script (abSs buttons allowed)
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 230 }))); // initiate Rocket fight
  // r.run(StartTrainerBattleSegment::new()); // Rocket
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(KOSegment::new(Move::BubbleBeam, EnemyAttack { mov: Move::QuickAttack, attack_type: EnemyAttackType::Hit { damage: 10 }}));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // not forget this // 179214 - 178868 = 346
  // r.save("blue_tower_after_rocket3");

  // r.load("blue_tower_after_rocket3");
  // r.run(WalkToSegment::new(10, 4));
  // r.run(OverworldMoveSegment::interact_with(4));
  // r.run(SkipTextsSegment::new(12).with_confirm_input(B)); // Fuji
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(2, 1));
  // r.run(OverworldMoveSegment::collide(R));
  // r.run(OverworldMoveSegment::interact_with(5)); // Fuji
  // r.run(SkipTextsSegment::new(5).with_confirm_input(B)); // Fuji
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Poke FLute
  // r.run(SkipTextsSegment::new(4).with_confirm_input(B)); // Fuji
  // r.run(WalkToSegment::new(2, 7).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.save("blue_test");
  // r.load("blue_test");
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(D)); // pidgey
  //   r.run(MoveSegment::new(A)); // pidgey
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Celadon
  //   r.run(OverworldMoveSegment::warp());
  // }
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(A|B)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(-1, 18));
  // r.run(WalkToSegment::new(27, 10));
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(D|R));
  //   r.run(MoveSegment::new(A)); // flute
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Played flute
  //   r.run(VerifyInputSegment::new("CloseStartMenu").with_input(B)); // don't main menu open
  //   r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Snorlax fight
  //   r.run(MoveSegment::new(NIL)); // Snorlax encounter
  // }
  // r.save("blue_test2");
  // r.load("blue_test2");
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Wild // Snorlax // appeared
  // r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(B));
  // r.run(MoveSegment::new(R));
  // r.run(MoveSegment::new(D|A));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Got away
  // r.run(WalkToSegment::new(24, 10).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(L)); // edge warp
  // r.run(WalkToSegment::new(0, 8).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(L)); // edge warp
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(1, 17));
  // r.run(WalkToSegment::new(1, 18).into(OverworldInteractionResult::Walked { direction: D }));
  // for _ in 0..121 {
  //   r.run(OverworldMoveSegment::auto_walk(D));
  // }
  // for _ in 0..6 {
  //   r.run(OverworldMoveSegment::walk(R));
  // }
  // r.run(MoveSegment::new(A)); // Max Elixer
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Elixer
  // for _ in 0..21 {
  //   r.run(OverworldMoveSegment::auto_walk(D));
  // }
  // r.run(OverworldMoveSegment::jump_ledge(D)); // Jump ledge
  // r.run(WalkToSegment::new(32, 8));
  // r.run(WalkToSegment::new(33, 8).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(R)); // edge warp
  // r.run(WalkToSegment::new(7, 4).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(R)); // edge warp
  // r.save("blue_fuchsia");

  // r.load("blue_fuchsia");
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(A|B)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(50, 8)); // Fuchsia
  // r.run(WalkToSegment::new(18, 20));
  // r.run(OverworldMoveSegment::collide(U)); // Cut bush
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // sandshrew
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // cut
  //   r.run(MoveSegment::new(B)); // Hacked away
  // }
  // r.run(WalkToSegment::new(16, 13));
  // r.run(WalkToSegment::new(16, 12));
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // sandshrew
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // cut
  //   r.run(MoveSegment::new(B)); // Hacked away
  // }
  // r.run(WalkToSegment::new(18, 3)); // Enter Safari // 187256
  // r.run(WalkToSegment::new(3, 2).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Welcome
  // r.run(OverworldMoveSegment::auto_walk(R));
  // r.run(SkipTextsSegment::new(3).with_confirm_input(B)); // Welcome
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Yes
  // r.run(SkipTextsSegment::new(7).with_confirm_input(B)); // Welcome
  // r.save("blue_safari");

  // r.load("blue_safari");
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(A|B)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(29, 11).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(R)); // edge warp
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(0, 5).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(L)); // edge warp
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(3, 35).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(19, 5));
  // r.run(WalkToSegment::new(19, 6));
  // r.run(OverworldMoveSegment::interact_with(4));
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Gold Teeth
  // r.run(WalkToSegment::new(3, 3)); // Surf house
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(3, 5));
  // r.run(WalkToSegment::new(3, 4));
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(SkipTextsSegment::new(7).with_confirm_input(B)); // Surf
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get HM
  // r.run(WalkToSegment::new(3, 7).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.save("blue_test");
  // r.load("blue_test");
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(D|A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(VerifyInputSegment::new("CloseStartMenu").with_input(B)); // don't main menu open
  //   r.run(OverworldMoveSegment::warp());
  // }
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(U)); // pidgey
  //   r.run(MoveSegment::new(A)); // pidgey
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Celadon
  //   r.run(OverworldMoveSegment::warp());
  // }
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(U|A)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(35, 30));
  // r.run(WalkToSegment::new(35, 31));
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // sandshrew
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // cut
  //   r.run(MoveSegment::new(B)); // Hacked away
  // }
  // r.run(WalkToSegment::new(12, 27)); // Enter Gym
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(1, 4));
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // sandshrew
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // cut
  //   r.run(MoveSegment::new(B)); // Hacked away
  // }
  // r.run(WalkToSegment::new(3, 4)); // Beauty
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoAction))); // advance map script (abSs buttons allowed)
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 218 }))); // initiate Beauty fight
  // r.run(StartTrainerBattleSegment::new()); // Beauty
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_celadon_after_beauty");

  // r.load("blue_celadon_after_beauty");
  // r.run(WalkToSegment::new(4, 4)); // Erika
  // r.run(OverworldMoveSegment::collide(U));
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(13)); // Erika
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(NextTrainerMonSegment::new().with_level_up());
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.save("blue_test");
  // r.load("blue_test");
  // r.run(EndTrainerBattleSegment::with_defeat_texts(3).with_name_in_defeat_texts());
  // r.run(SkipTextsSegment::new(7).with_confirm_input(B)); // Erika after texts
  // r.run(WalkToSegment::new(5, 5));
  // r.run(WalkToSegment::new(5, 6));
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // sandshrew
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // cut
  //   r.run(MoveSegment::new(B)); // Hacked away
  // }
  // r.run(WalkToSegment::new(5, 17).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   for _ in 0..9 {
  //     r.run(MoveSegment::new(D|L));
  //     r.run(MoveSegment::new(D|R));
  //   }
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(A)); // HM03
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Booted up TM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // It contains // move // !
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // Teach Surf
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Wartortle
  //   r.run(OverrideMoveSegment::new(3).with_confirm_input(A)); // override Bite
  //   r.run(MoveSegment::new(B)); // cancel
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // pidgey
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Fuchsia
  //   r.run(OverworldMoveSegment::warp());
  // }
  // r.run(WalkToSegment::new(5, 27)); // Enter Gym // 200210
  // r.save("blue_test2");
  // r.load("blue_test2");
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(7, 9));
  // r.run(OverworldMoveSegment::collide(U));
  // r.run(OverworldMoveSegment::interact_with(3));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(2)); // Juggler
  // r.run(OHKOSegment::new(Move::Surf));
  // r.run(NextTrainerMonSegment::new().with_level_up());
  // r.run(OHKOSegment::new(Move::Surf));
  // r.run(NextTrainerMonSegment::new());
  // r.run(KOSegment::new(Move::Surf, EnemyAttack { mov: Move::Disable, attack_type: EnemyAttackType::EffectFailed }));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::Surf));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_fuchsia_after_juggler1");

  // r.load("blue_fuchsia_after_juggler1");
  // r.run(WalkToSegment::new(1, 7)); // Juggler
  // r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::NoAction))); // advance map script (abSs buttons allowed)
  // r.run(SkipTextsSegment::new(3).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 221 }))); // initiate Juggler fight
  // r.run(StartTrainerBattleSegment::new()); // Juggler
  // r.run(OHKOSegment::new(Move::MegaKick));
  // r.run(NextTrainerMonSegment::new());
  // r.save("blue_test");
  // r.load("blue_test");
  // r.run_merge(FightTurnSegment::new(Move::MegaKick, true, 82, EnemyAttack { attack_type: EnemyAttackType::EffectFailed, mov: Move::PoisonGas } ).with_exact_damage());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_fuchsia_after_juggler2");

  // r.load("blue_fuchsia_after_juggler2");
  // r.run(TextSegment::new().with_skip_ends(2));
  // for _ in 0..72 {
  //   r.run(VerifyInputSegment::new("Evolution_CheckForCancel").with_input(NIL)); // Don't cancel evolution
  // }
  // r.run(TextSegment::new().with_skip_ends(4));
  // r.run(WalkToSegment::new(3, 10)); // Koga
  // r.run(OverworldMoveSegment::collide(R));
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(9)); // Koga
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.save("blue_test");
  // r.load("blue_test");
  // r.run(KOSegment::new(Move::BubbleBeam, EnemyAttack { attack_type: EnemyAttackType::Hit { damage: 30 }, mov: Move::Sludge } ).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new().with_level_up());
  // r.save("blue_test2");
  // r.load("blue_test2");
  // { // Selfdestruct
  //   r.run(SelectMoveSegment::new(Move::BubbleBeam));
  //   r.run(DelaySegment::new(MoveSegment::with_metric(A, ExpectedAIChooseMoveMetric { expected_move: Some(Move::SelfDestruct) })));
  //   r.run(TextSegment::with_metric(Gen1NormalHitMetric { expected_max_damage: 34, expected_max_crit_damage: 64 }.expect(FightTurnResult::Failed)).with_skip_ends(4).with_unbounded_buffer()); // A used BubbleBeam
  //   r.run(TextSegment::new().with_allowed_end_inputs(A).with_unbounded_buffer()); // attack missed
  //   r.run(DelaySegment::new(MoveSegment::with_metric(B, TrainerAIMetric {}.expect(TrainerAIAction::NoAction)).seq(TextSegment::with_metric(Gen1MoveSuccessMetric {}.expect(FightTurnResult::Failed)).with_skip_ends(4).with_allowed_end_inputs(B).with_unbounded_buffer()))); // Weezing used SelfDestruct
  // r.save("blue_test3");
    // r.load("blue_test3");
    // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // missed
  // }
  // r.run(EndTrainerBattleSegment::with_defeat_texts(3).with_name_in_defeat_texts());
  // r.run(SkipTextsSegment::new(8).with_confirm_input(B)); // after texts
  // r.save("blue_fuchsia_after_koga");

  // r.load("blue_fuchsia_after_koga");
  // r.run(WalkToSegment::new(5, 16));
  // r.run(WalkToSegment::new(5, 17).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(B|A)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(22, 30));
  // r.run(OverworldMoveSegment::jump_ledge(R)); // Jump ledge
  // r.run(WalkToSegment::new(27, 27)); // Strength house
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(2, 5));
  // r.run(WalkToSegment::new(2, 4));
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(SkipTextsSegment::new(10).with_confirm_input(B)); // after texts
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // got HM
  // r.run(WalkToSegment::new(4, 6));
  // r.run(WalkToSegment::new(4, 7).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.save("blue_test");
  // r.load("blue_test");
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(A)); // items
  //   for _ in 0..9 {
  //     r.run(MoveSegment::new(D|L));
  //     r.run(MoveSegment::new(D|R));
  //   }
  //   r.run(MoveSegment::new(D|L));
  //   r.run(MoveSegment::new(A)); // HM04
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Booted up TM
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // It contains // move // !
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // Teach Strength
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // Sandshrew
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(3).with_confirm_input(A)); // Learned Strength
  //   r.run(MoveSegment::new(U|L));
  //   r.run(MoveSegment::new(U|A)); // Max Elixer
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Blastoise
  //   r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // PP restored
  //   r.run(MoveSegment::new(B)); // cancel
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // pidgey
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // fly
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Pallet
  //   r.run(OverworldMoveSegment::warp());
  // }
  // r.run(WalkToSegment::new(4, 12));
  // r.run(WalkToSegment::new(4, 13));
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // Blastoise
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // Surf
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Got on
  // }
  // r.save("blue_test2");
  // r.load("blue_test2");
  // r.run(OverworldMoveSegment::auto_walk(D));
  // r.run(OverworldMoveSegment::wait()); // surf start
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(4, 18));
  // r.run(WalkToSegment::new(3, 90).with_unbounded_buffer()); // 217125
  // r.run(WalkToSegment::new(4, 4).with_unbounded_buffer());
  // r.run(WalkToSegment::new(6, 3).with_unbounded_buffer());
  // r.run(WalkToSegment::new(5, 10).with_unbounded_buffer()); // 2F
  // r.run(OverworldMoveSegment::wait()); // stairs? (AB ebabled)
  // r.run(DelaySegment::new(MoveSegment::with_metric(L, OverworldInteractionMetric {}.expect(OverworldInteractionResult::Turned { direction: L }))));
  // r.run(WalkToSegment::new(6, 1)); // 3F
  // r.save("blue_test3");
  // r.load("blue_test3");
  // r.run(OverworldMoveSegment::wait()); // stairs? (AB ebabled)
  // r.run(DelaySegment::new(MoveSegment::with_metric(R, OverworldInteractionMetric {}.expect(OverworldInteractionResult::Turned { direction: R }))).with_unbounded_buffer());
  // r.run(WalkToSegment::new(10, 6).with_unbounded_buffer());
  // r.run(OverworldMoveSegment::collide(U));
  // r.run(MoveSegment::new(A)); // Switch
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Switch
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Press
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Who wouldn't?
  // r.run(WalkToSegment::new(16, 14).with_unbounded_buffer().into(OverworldInteractionResult::FlyWarpOrDungeonWarp));
  // r.run(OverworldMoveSegment::warp());
  // r.run(WalkToSegment::new(21, 23).with_unbounded_buffer()); // B1F
  // r.save("blue_test4");
  // r.load("blue_test4");
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(18, 26).with_unbounded_buffer()); // Switch
  // r.run(OverworldMoveSegment::collide(U));
  // r.run(MoveSegment::new(A)); // Switch
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Switch
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Press
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Who wouldn't?
  // r.run(WalkToSegment::new(20, 4).with_unbounded_buffer()); // Switch
  // r.run(OverworldMoveSegment::collide(U));
  // r.run(MoveSegment::new(A)); // Switch
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Switch
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Press
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Who wouldn't?
  // r.run(WalkToSegment::new(5, 12).with_unbounded_buffer()); // Secret Key
  // r.run(OverworldMoveSegment::interact_with(8));
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Secret Key
  // r.save("blue_test5");
  // r.load("blue_test5");
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(D|A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(VerifyInputSegment::new("CloseStartMenu").with_input(B)); // don't main menu open
  //   r.run(OverworldMoveSegment::warp());
  // }
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(D)); // pidgey
  //   r.run(MoveSegment::new(A)); // pidgey
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Cinnabar
  //   r.run(OverworldMoveSegment::warp());
  // }
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(U|A)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(18, 3)); // Gym
  // r.save("blue_cinnabar_gym");

  // r.load("blue_cinnabar_gym");
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(15, 9)); // Q1
  // r.run(WalkToSegment::new(15, 8)); // Q1
  // r.run(MoveSegment::new(A)); // Q1
  // r.run(SkipTextsSegment::new(8).with_confirm_input(B)); // Q1
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Yes
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Q1
  // r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // r.run(WalkToSegment::new(10, 2)); // Q2
  // r.run(OverworldMoveSegment::collide(U));
  // r.run(MoveSegment::new(A)); // Q2
  // r.run(SkipTextsSegment::new(9).with_confirm_input(B)); // Q2
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // No
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Q2
  // r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // r.run(WalkToSegment::new(9, 8)); // Q3
  // r.run(OverworldMoveSegment::collide(U));
  // r.run(MoveSegment::new(A)); // Q3
  // r.run(SkipTextsSegment::new(8).with_confirm_input(B)); // Q3
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // No
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Q3
  // r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // r.run(WalkToSegment::new(9, 14)); // Q4
  // r.run(OverworldMoveSegment::collide(U));
  // r.run(MoveSegment::new(A)); // Q4
  // r.run(SkipTextsSegment::new(10).with_confirm_input(B)); // Q4
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // No
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Q4
  // r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // r.run(WalkToSegment::new(1, 15)); // Q5
  // r.run(WalkToSegment::new(1, 14)); // Q5
  // r.run(MoveSegment::new(A)); // Q5
  // r.run(SkipTextsSegment::new(10).with_confirm_input(B)); // Q5
  // r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // Yes
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Q5
  // r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // r.run(WalkToSegment::new(1, 9)); // Q6
  // r.run(WalkToSegment::new(1, 8)); // Q6
  // r.run(MoveSegment::new(A)); // Q6
  // r.run(SkipTextsSegment::new(8).with_confirm_input(B)); // Q6
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // No
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // Q6
  // r.run(VerifyInputSegment::new("HoldTextDisplayOpen").with_input(B)); // don't hold text box open
  // r.run(WalkToSegment::new(3, 5)); // Blaine
  // r.run(WalkToSegment::new(3, 4)); // Blaine
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.save("blue_cinnabar_blaine");

  // r.load("blue_cinnabar_blaine");
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(6)); // Blaine
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.save("blue_test");
  // r.load("blue_test");
  // { // Super Potion
  //   r.run(SelectMoveSegment::new(Move::BubbleBeam));
  //   r.run(DelaySegment::new(MoveSegment::with_metric(A, TrainerAIMetric {}.expect(TrainerAIAction::Potion))));
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Blaine used super potion
  //   r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(A)); // on Rapidash
  //   r.run(DelaySegment::new(MoveSegment::new(B).seq(TextSegment::with_metric(Gen1NormalHitMetric { expected_max_damage: 72, expected_max_crit_damage: 138 }.filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &113 } else { false } ).into_unit()).with_skip_ends(4).with_unbounded_buffer()))); // A used BubbleBeam
  //   r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // crit, effective
  // }
  // r.run(NextTrainerMonSegment::new().with_level_up()); // 230598 - 230567 = 31
  // r.save("blue_test2");
  // r.load("blue_test2");
  // { // Super Potion
  //   r.run(SelectMoveSegment::new(Move::Surf));
  //   r.run(DelaySegment::new(MoveSegment::with_metric(A, TrainerAIMetric {}.expect(TrainerAIAction::Potion))));
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Blaine used super potion
  //   r.run(TextSegment::new().with_skip_ends(2).with_allowed_end_inputs(A)); // on Arcanine
  //   r.run(DelaySegment::new(MoveSegment::new(B).seq(TextSegment::with_metric(Gen1NormalHitMetric { expected_max_damage: 102, expected_max_crit_damage: 188 }.filter(|v| if let FightTurnResult::CriticalHit { damage } = v { damage >= &149 } else { false } ).into_unit()).with_skip_ends(4).with_unbounded_buffer()))); // A used Surf
  //   r.run(SkipTextsSegment::new(2).with_confirm_input(B)); // crit, effective
  // }
  // r.save("blue_test3");
  // r.load("blue_test3");
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2).with_name_in_defeat_texts());
  // r.run(SkipTextsSegment::new(6).with_confirm_input(B)); // after texts
  // r.save("blue_cinnabar_after_blaine");

  // r.load("blue_cinnabar_after_blaine");
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(D|A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(VerifyInputSegment::new("CloseStartMenu").with_input(B)); // don't main menu open
  //   r.run(OverworldMoveSegment::warp());
  // }
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(D)); // pidgey
  //   r.run(MoveSegment::new(A)); // pidgey
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Celadon
  //   r.run(OverworldMoveSegment::warp());
  // }
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(U|A)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(50, 10));
  // r.run(WalkToSegment::new(11, 10).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(R)); // edge warp
  // r.run(WalkToSegment::new(3, 4).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(SkipTextsSegment::new(15).with_confirm_input(B)); // Give drink
  // r.run(WalkToSegment::new(5, 4).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(R)); // edge warp
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(B|A)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(20, 10));
  // r.save("blue_saffron");

  // r.load("blue_saffron");
  // r.run(WalkToSegment::new(18, 21));
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(26, 0)); // 1F
  // r.run(OverworldMoveSegment::wait()); // stairs? (AB ebabled)
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(26, 0)); // 2F
  // r.run(OverworldMoveSegment::wait()); // stairs? (AB ebabled)
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(24, 0)); // 3F
  // r.run(OverworldMoveSegment::wait()); // stairs? (AB ebabled)
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(26, 0)); // 4F
  // r.run(OverworldMoveSegment::wait()); // stairs? (AB ebabled)
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(8, 14));
  // r.run(WalkToSegment::new(8, 15));
  // r.run(OverworldMoveSegment::interact_with(2));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Rocket
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_saffron_after_rocket1");

  // r.load("blue_saffron_after_rocket1");
  // r.run(WalkToSegment::new(9, 15));
  // r.run(WalkToSegment::new(17, 16));
  // r.run(WalkToSegment::new(17, 15));
  // r.run(WalkToSegment::new(20, 16));
  // r.run(OverworldMoveSegment::interact_with(8));
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Secret Key
  // r.run(WalkToSegment::new(9, 15));
  // r.run(WalkToSegment::new(17, 16));
  // r.run(WalkToSegment::new(17, 15));
  // r.run(WalkToSegment::new(9, 13));
  // r.run(WalkToSegment::new(8, 13));
  // r.run(MoveSegment::new(A)); // Door
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Secret Key
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Secret Key
  // r.run(WalkToSegment::new(3, 15));
  // r.run(WalkToSegment::new(18, 9));
  // r.run(OverworldMoveSegment::collide(L));
  // r.run(MoveSegment::new(A)); // Door
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Secret Key
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Secret Key
  // r.run(WalkToSegment::new(11, 11));
  // r.run(WalkToSegment::new(3, 2).into(OverworldInteractionResult::NoOverworldInput));
  // r.save("blue_saffron_rival");

  // r.load("blue_saffron_rival");
  // r.run(SkipTextsSegment::new(10).with_confirm_input(B)); // rival pre-fight text
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 242 }))); // initiate rival fight
  // r.run(StartTrainerBattleSegment::new());
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(KOSegment::new(Move::IceBeam, EnemyAttack { mov: Move::Leer, attack_type: EnemyAttackType::EffectFailed }).with_expected_effect(MoveEffectResult::NoEffect));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::MegaKick));
  // r.run(NextTrainerMonSegment::new().with_level_up());
  // r.run(OHKOSegment::new(Move::IceBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(2).with_name_in_defeat_texts());
  // r.save("blue_saffron_after_rival");

  // r.load("blue_saffron_after_rival");
  // r.run(SkipTextsSegment::new(14).with_confirm_input(B)); // rival pre-fight text
  // r.run(OverworldMoveSegment::turn(U));
  // r.run(WalkToSegment::new(5, 7));
  // r.run(WalkToSegment::new(2, 16));
  // r.run(OverworldMoveSegment::collide(R));
  // r.run(OverworldMoveSegment::interact_with(4));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(1)); // Rocket
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_saffron_after_rocket2");

  // r.load("blue_saffron_after_rocket2");
  // r.run(WalkToSegment::new(6, 15));
  // r.run(WalkToSegment::new(6, 14));
  // r.run(MoveSegment::new(A)); // Door
  // r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Secret Key
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // Secret Key
  // r.run(WalkToSegment::new(6, 13).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(SkipTextsSegment::new(7).with_confirm_input(B)); // Giovanni pre-fight text
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 229 }))); // initiate Giovanni fight
  // r.run(StartTrainerBattleSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts().with_level_up().with_skip_learning_move());
  // r.run(SkipTextsSegment::new(7).with_confirm_input(B)); // Giovanni post-fight text
  // r.save("blue_saffron_after_giovanni");

  // r.load("blue_saffron_after_giovanni");
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(D|A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(VerifyInputSegment::new("CloseStartMenu").with_input(B)); // don't main menu open
  //   r.run(OverworldMoveSegment::warp());
  // }
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(D)); // pidgey
  //   r.run(MoveSegment::new(A)); // pidgey
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // fly
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // Saffron
  //   r.run(OverworldMoveSegment::warp());
  // }
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(U|A)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(36, 31));
  // r.run(WalkToSegment::new(34, 3)); // Enter Gym
  // r.run(OverworldMoveSegment::turn(L));
  // r.run(WalkToSegment::new(11, 15));
  // r.run(WalkToSegment::new(15, 15));
  // r.run(WalkToSegment::new(15, 5));
  // r.run(WalkToSegment::new(1, 5));
  // r.run(WalkToSegment::new(9, 10));
  // r.run(WalkToSegment::new(9, 9));
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(8)); // Sabrina
  // r.save("blue_test");
  // r.load("blue_test");
  // r.run(OHKOSegment::new(Move::MegaKick));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::Surf));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::Surf));
  // r.run(NextTrainerMonSegment::new());
  // r.save("blue_test2");
  // r.load("blue_test2");
  // r.run(KOSegment::new(Move::MegaKick, EnemyAttack { mov: Move::Psybeam, attack_type: EnemyAttackType::HitFailed }));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(6).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_saffron_after_sabrina");

  // r.load("blue_saffron_after_sabrina");
  // r.run(SkipTextsSegment::new(9).with_confirm_input(B)); // after texts
  // r.run(WalkToSegment::new(11, 11));
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(D|A)); // Escape Rope
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // use
  //   r.run(VerifyInputSegment::new("CloseStartMenu").with_input(B)); // don't main menu open
  //   r.run(OverworldMoveSegment::warp());
  // }
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // mon
  //   r.run(MoveSegment::new(D)); // pidgey
  //   r.run(MoveSegment::new(A)); // pidgey
  //   r.run(MoveSegment::new(NIL));
  //   r.run(MoveSegment::new(A)); // fly
  //   r.run(MoveSegment::new(U));
  //   r.run(MoveSegment::new(A)); // Viridian
  //   r.run(OverworldMoveSegment::warp());
  // }
  // {
  //   r.run(OverworldMoveSegment::open_main_menu());
  //   r.run(MoveSegment::new(D));
  //   r.run(MoveSegment::new(A)); // items
  //   r.run(MoveSegment::new(U|A)); // bike
  //   r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(B)); // Get on Bike
  // }
  // r.run(WalkToSegment::new(32, 7)); // Enter Gym
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(15, 5).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(SkipTextsSegment::new(1).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 231 }))); // initiate trainer fight
  // r.run(StartTrainerBattleSegment::new()); // Cooltrainer
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_viridian_after_cooltrainerm");

  // r.load("blue_viridian_after_cooltrainerm");
  // r.run(WalkToSegment::new(10, 4).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(SkipTextsSegment::new(2).with_confirm_input(B));
  // r.run(MoveSegment::with_metric(D, OverworldInteractionMetric {}.assert_eq(OverworldInteractionResult::TrainerBattle { species: 224 }))); // initiate trainer fight
  // r.run(StartTrainerBattleSegment::new()); // Blackbelt
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(1).with_name_in_defeat_texts());
  // r.save("blue_viridian_after_blackbelt");

  // r.load("blue_viridian_after_blackbelt");
  // r.run(WalkToSegment::new(16, 16));
  // r.run(WalkToSegment::new(16, 17).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(32, 7)); // Enter Gym
  // r.run(OverworldMoveSegment::turn(R));
  // r.run(WalkToSegment::new(15, 6));
  // r.run(WalkToSegment::new(2, 3));
  // r.run(WalkToSegment::new(2, 2));
  // r.run(OverworldMoveSegment::interact_with(1));
  // r.run(StartTrainerBattleSegment::new().with_pre_battle_texts(10)); // Giovanni
  // r.save("blue_test");
  // r.load("blue_test");
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new().with_level_up());
  // r.save("blue_test2");
  // r.load("blue_test2");
  // r.run(KOSegment::new(Move::BubbleBeam, EnemyAttack { mov: Move::Growl, attack_type: EnemyAttackType::EffectFailed }));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(NextTrainerMonSegment::new());
  // r.run(OHKOSegment::new(Move::BubbleBeam));
  // r.run(EndTrainerBattleSegment::with_defeat_texts(5).with_name_in_defeat_texts().with_level_up());
  // r.save("blue_viridian_after_giovanni");

  // r.load("blue_viridian_after_giovanni");
  // r.run(SkipTextsSegment::new(8).with_confirm_input(B)); // after texts
  // r.run(SkipTextsSegment::new(1).with_skip_ends(0).with_confirm_input(B)); // Got TM
  // r.run(SkipTextsSegment::new(5).with_confirm_input(B)); // after fight texts
  // r.run(WalkToSegment::new(15, 6));
  // r.run(WalkToSegment::new(16, 16));
  // r.run(WalkToSegment::new(16, 17).into(OverworldInteractionResult::NoOverworldInput));
  // r.run(VerifyInputSegment::new("CheckWarpsNoCollisionLoop").with_input(D)); // edge warp





  // println!("Player: {:#?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:#?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:#?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:#?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));

  // r.run(TextSegment::new().with_skip_ends(0));

  // let map = r.get_state_metric_fn(|gb| { montas::segment::overworld::gen1::Map::default().load_gen1_map(gb) });
  // log::info!("tile collisions:\n{}", map.tile_string());
  // log::info!("allowed movements:\n{}", map.tile_allowed_movements_string());

  r.run_debug(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().into_unit()));
  // r.run_debug(MoveSegment::with_metric(D, OverworldInteractionMetric {}.debug_print().into_unit()));
  // r.run_debug(MoveSegment::with_metric(R, OverworldInteractionMetric {}.debug_print().into_unit()));
  // r.save("blue_test2");

  // r.load("blue_test2");
}