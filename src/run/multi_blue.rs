#[allow(unused_imports)] use gambatte::inputs::*;
#[allow(unused_imports)] use montas::constants::*;
#[allow(unused_imports)] use montas::metric::*;
#[allow(unused_imports)] use montas::metric::battle::*;
#[allow(unused_imports)] use montas::metric::battle::gen1::*;
#[allow(unused_imports)] use montas::metric::overworld::gen1::*;
use montas::multi::*;
use montas::rom::*;
use montas::sdl::*;

const EQUAL_LENGTH_FRAMES: bool = true;
const RTC_DIVISOR_OFFSET: i32 = 0;

#[allow(dead_code)]
pub fn start() {
  log::set_max_level(log::LevelFilter::Debug);

  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let blue_gb = Gb::<Blue>::create(EQUAL_LENGTH_FRAMES, RTC_DIVISOR_OFFSET, SdlScreen::new(sdl.clone(), 0));
  let mut r = SingleGbRunner::new(blue_gb);

  // r.run(SkipIntroPlan::new().with_auto_pass_after(214)); // Logo
  // r.run(SkipIntroPlan::new().with_auto_pass_after(322)); // Intro cutscene
  // r.run(SkipIntroPlan::new().with_no_up_select_b()); // main menu
  // r.run(MainMenuPlan::new()); // main menu
  // r.run(SkipTextsPlan::new(13)); // oak speech
  // r.run(IntroNameMenuPlan::choose_custom_name()); // own name
  // r.run(NamingScreenPlan::with_letter(b'B'));
  // r.run(SkipTextsPlan::new(5)); // oak speech
  // r.run(IntroNameMenuPlan::choose_custom_name()); // rival name
  // r.run(NamingScreenPlan::with_letter(b'U'));
  // r.run(SkipTextsPlan::new(7)); // oak speech
  // r.run(TextPlan::new()); // ... awaits let's go // #inputs: 2500(2728)
  // r.save("multi_blue_intro_");
  // r.load("multi_blue_intro_");
  // r.run(WalkToPlan::new(7, 1)); // go down stairs
  // r.run(WalkToPlan::new(3, 6)); // go outside
  // r.run(WalkToPlan::new(3, 7)); // go outside
  // r.run(EdgeWarpPlan::new()); // go outside
  // r.run(WalkToPlan::new(10, 1)); // trigger oak cutscene
  // r.run(OverworldWaitPlan::new()); // Skip PalletTownScript0
  // r.run(TextPlan::new()); // it's dangerous to go outside, take this
  // r.run(HoldTextDisplayOpenPlan::new()); // close text box
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan::new())); // oak speech
  // r.run(OverworldWaitPlan::new()); // Skip PalletTownScript load
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow Oak
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(L)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(R)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(R)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(R)); // Follow oak
  // r.run(OverworldWaitPlan::auto_walk(U)); // Follow oak
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())); // Fed up with waiting
  // r.run(SeqPlan::new(SkipTextsPlan::new(12), HoldTextDisplayOpenPlan::new())); // you can have one, choose
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())); // What about me?
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())); // Can have one too
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(WalkToPlan::new(7, 4)); // stand before squirtle
  // r.run(OverworldTurnPlan::new(U)); // turn towards squirtle
  // r.run(OverworldInteractPlan::with(3)); // Interact with Squirtle Ball
  // r.run(TextScrollWaitPlan::new()); // Scroll dex text 1
  // r.run(TextScrollWaitPlan::new()); // Scroll dex text 2
  // r.run(SkipTextsPlan::new(1)); // so you want Squirtle
  // r.run(TextPlan::new()); // so you want Squirtle?
  // r.run(TwoOptionMenuPlan::yes()); // choose Squirtle
  // r.run(SkipTextsPlan::new(1)); // looks really energetic
  // r.save("multi_blue_test3");
  // r.load("multi_blue_test3");
  // r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // received Squirtle! Do you want...
  // r.run(TextPlan::new().with_skip_ends(2)); // give nickname?
  // r.run(TwoOptionMenuPlan::yes()); // give nickname
  //   r.run(NamingScreenPlan::with_metric(b'S', Gen1DVMetric {}.filter(|v| {
  //     // log::info!("Inspect DVs: {:?}", v);
  //     // if v.atk != 14 || v.def < 0 || v.def > 2 || v.spc != 15 || v.spd != 15 || v.hp() != 3 { return false; } // squirtle DVs
  //     if v.atk != 14 || v.def < 6 || v.def > 8 || v.spc != 15 || v.spd != 15 || v.hp() != 3 { return false; } // squirtle DVs
  //     log::info!("Chosen DVs: {:?}", v); true
  //   }).into_unit()));
  // r.save("multi_blue_chosen_starter_"); // DVs: 14 / 2|6 / 15 / 15 // #inputs: 5914
  // r.load("multi_blue_chosen_starter_");
  // r.run(HoldTextDisplayOpenPlan::new());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())); // I'll take this one then
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan::new())); // rival received // bulbasaur // !
  // r.run(WalkToPlan::new(5, 6)); // trigger rival fight
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())); // Rival fight
  // r.run(OverworldWaitPlan::trainer_battle(225)); // Rival fight
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Rival fight
  // // r.run(FightTurnPlan::new(AttackDesc::stat_up_down(Move::TailWhip), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Growl)), None));
  // r.run(FightKOPlan::new(Move::Tackle, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Growl))));
  // r.run(EndTrainerBattlePlan::with_level_up(3)); // Rival1 fight // #inputs: 9080 9087 / 9094
  // r.save("multi_blue_after_rival1_");
  // r.load("multi_blue_after_rival1_");
  // r.run(OverworldWaitPlan::new()); // advance map script (abSs buttons allowed)
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())); // after rival1 texts
  // r.run(WalkToPlan::new(5, 11)); // Leave lab
  // r.run(EdgeWarpPlan::new()); // go outside // inputs: 9691 - 9617 = 74
  // r.run(WalkToPlan::new(10, -1));
  // r.run(WalkToPlan::new(11, -1)); // Enter Viridian
  // r.run(WalkToPlan::new(29, 19)); // Enter Mart, starts cutscene
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())); // mart cutscene
  // r.run(OverworldWaitPlan::auto_walk(U)); // mart cutscene
  // r.run(OverworldWaitPlan::auto_walk(U)); // mart cutscene
  // r.run(OverworldWaitPlan::auto_walk(L)); // mart cutscene
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())); // mart cutscene
  // {
  //   r.run(OverworldOpenStartMenuPlan::new()); // Open start menu
  //   r.run(StartMenuPlan::options()); // main menu
  //   r.run(ChangeOptionsPlan::new()); // set options
  //   r.run(StartMenuPlan::close()); // main menu
  // }
  // r.run(WalkToPlan::new(3, 6));
  // r.run(WalkToPlan::new(3, 7));
  // r.run(EdgeWarpPlan::new()); // go outside // inputs: 11931 - 11865 = 66
  // r.run(WalkToPlan::new(29, 26));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(21, 36)); // enter Route 1
  // r.run(WalkToPlan::new(14, 18));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(12, 26));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(11, 36)); // enter pallet town
  // r.run(WalkToPlan::new(12, 11)); // enter oak's lab
  // r.run(WalkToPlan::new(4, 2)); // next to oak
  // r.run(OverworldTurnPlan::new(R)); // turn towards Oak
  // r.run(OverworldInteractPlan::with(5)); // Talk to Oak
  // r.run(SeqPlan::new(SkipTextsPlan::new(10), HoldTextDisplayOpenPlan::new())); // Oak speech: special pokeball, thank you
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())); // Oak speech: Gramps
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())); // Oak speech: What came for?
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())); // Oak speech: Have something for you
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan::new())); // Oak speech: hi-tech encyclopedia
  // r.run(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan::new())); // Oak speech: Took Pokedex
  // r.run(SeqPlan::new(SkipTextsPlan::new(10), HoldTextDisplayOpenPlan::new())); // Oak speech: greatest undertaking
  // r.run(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan::new())); // Oak speech: leave it to me
  // r.run(WalkToPlan::new(4, 11)); // leave
  // r.run(EdgeWarpPlan::new()); // go outside // inputs: 16078 / 16086
  // r.save("multi_blue_after_oak_parcel_");
  // r.load("multi_blue_after_oak_parcel_");
  // r.run(WalkToPlan::new(10, -1)); // enter Route 1
  // r.run(WalkToPlan::new(11, -1)); // Enter Viridian
  // r.run(WalkToPlan::new(18, -1)); // Enter Route 2
  // r.run(WalkToPlan::new(3, 43)); // Enter Viridian Forest
  // r.run(WalkToPlan::new(5, 0)); // Enter Viridian Forest
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // // r.run(WalkToPlan::new(25, 42));
  // // r.run(WalkToPlan::new(25, 41));
  // // // r.run(WalkToPlan::new(25, 23));
  // r.run(WalkToEncounterPlan::new(2, 19, OverworldInteractionMetric.filter(|v| {
  //   if let OverworldInteractionResult::WildEncounter { species: Pokemon::Pikachu, level: 5, dvs } = v {
  //     log::info!("Pika");
  //     if dvs.hp() <= 4 && dvs.def <= 9 {
  //       log::info!("Pika DVs: {:?}", dvs); true
  //     } else { false }
  //   } else { false }
  // }).into_unit())); // #inputs: 19401 // 19423
  // r.save("multi_blue_pikachu_encounter_");
  // r.load("multi_blue_pikachu_encounter_");
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Wild Pikachu appeared!
  // r.run(TextPlan::new().with_skip_ends(2)); // Go, Pikachu!
  // r.run(FightTurnPlan::new(AttackDesc::hit(Move::Tackle, 6..=6), EnemyAttackDesc::Attack(AttackDesc::hit_no_side_effect(Move::ThunderShock, 10..=10)), None));
  // r.run(FightTurnPlan::new(AttackDesc::hit(Move::Tackle, 6..=6), EnemyAttackDesc::Attack(AttackDesc::hit_no_side_effect(Move::ThunderShock, 10..=10)), None));
  // r.run(FightTurnPlan::new(AttackDesc::hit(Move::Tackle, 6..=6), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Growl)), None));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // mon // grew to level // X // #inputs: 21351 // 21320 / 21037
  // r.save("multi_blue_after_pikachu_encounter_");
  // r.load("multi_blue_after_pikachu_encounter_");
  // r.run(WalkToPlan::new(2, 19));
  // r.run(OverworldInteractPlan::with(4)); // Bugcatcher
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1));
  // r.run(FightKOPlan::new(Move::Tackle, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::StringShot))));
  // r.run(EndTrainerBattlePlan::with_learn_move(2)); // Bugcatcher fight // #inputs: 25260 / 25269
  // r.save("multi_blue_viridian_after_bugcatcher_");
  // r.load("multi_blue_viridian_after_bugcatcher_");
  // r.run(WalkToPlan::new(1, 0)); // Leave Forest
  // r.run(EdgeWarpPlan::new()); // edge warp
  // r.run(WalkToPlan::new(5, 0)); // Leave Viridian Forest
  // r.run(WalkToPlan::new(8, -1)); // enter Pewter City
  // r.run(WalkToPlan::new(16, 17)); // enter Gym
  // r.run(WalkToPlan::new(4, 2)); // stand in front of Brock
  // r.run(OverworldInteractPlan::with(1)); // Brock
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(9));
  // r.run(BattleMenuPlan::fight());
  // r.run(SelectMoveMenuPlan::new(Move::Tackle).use_select());
  // r.run(SelectMoveMenuPlan::new(Move::Bubble).use_select());
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightKOPlan::new(Move::Bubble, Some(MoveEffectResult::NoEffect), EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::Tackle))).skip_battle_menu());
  // r.save("multi_blue_test2"); // 28821
  // r.load("multi_blue_test2");
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::Bubble, Some(MoveEffectResult::NoEffect), EnemyAttackDesc::Attack(AttackDesc::hit(Move::Tackle, 7..=7))));
  // r.run(EndTrainerBattlePlan::with_level_up(10)); // Brock fight //  #inputs: 30584 / 30590
  // r.save("multi_blue_after_brock_");
  // r.load("multi_blue_after_brock_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Brock speech
  // r.run(SeqPlan::new(SkipTextsPlan::new(13), HoldTextDisplayOpenPlan)); // Brock speech
  // r.run(WalkToPlan::new(4, 13));
  // r.run(EdgeWarpPlan::new()); // leave gym
  // r.run(WalkToPlan::new(23, 17)); // Enter Mart
  // r.run(WalkToPlan::new(3, 5));
  // r.run(WalkToPlan::new(2, 5));
  // r.run(OverworldInteractPlan::with(1)); // Mart
  // r.run(TextPlan::new()); // How can I help you
  // r.run(BuySellQuitMenuPlan::buy());
  // r.run(TextPlan::new()); // Take your time
  // r.run(SeqPlan::new(ListMenuPlan::choose(2), ChooseQuantityMenuPlan::new(7))); // Choose Escape Rope x7
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Item? // That will be
  // r.run(TextPlan::new().with_skip_ends(2)); // Price // Okay?
  // r.run(TwoOptionMenuPlan::yes()); // buy
  // r.run(SkipTextsPlan::new(1)); // Here you go
  // r.run(SeqPlan::new(ListMenuPlan::choose(0), ChooseQuantityMenuPlan::new(3))); // Choose Poke Ball x3
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Item? // That will be
  // r.run(TextPlan::new().with_skip_ends(2)); // Price // Okay?
  // r.run(TwoOptionMenuPlan::yes()); // buy
  // r.run(SkipTextsPlan::new(1)); // Here you go
  // r.run(ListMenuPlan::quit()); // exit buy menu
  // r.run(TextPlan::new()); // Anything else?
  // r.run(BuySellQuitMenuPlan::quit());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Thank you!
  // r.run(WalkToPlan::new(3, 6));
  // r.run(WalkToPlan::new(3, 7));
  // r.run(EdgeWarpPlan::new()); // go outside
  // r.run(WalkToPlan::new(40, 17)); // Enter Route 3
  // r.run(WalkToPlan::new(11, 6)); // Bugcatcher
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Bugcatcher
  // r.run(OverworldWaitPlan::trainer_battle(202)); // Bugcatcher fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Bugcatcher fight
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(SeqPlan::new(
  //   FightKOPlan::new(Move::Bubble, Some(MoveEffectResult::NoEffect), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::StringShot))),
  //   NextTrainerMonPlan::with_level_up()));
  // r.run(FightKOPlan::new(Move::Bubble, Some(MoveEffectResult::NoEffect), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::StringShot))));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(FightKOPlan::new(Move::Bubble, Some(MoveEffectResult::NoEffect), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::StringShot))));
  // r.run(EndTrainerBattlePlan::new(1)); // Bugcatcher fight //  #inputs: 36237 / 36240
  // r.save("multi_blue_route3_after_bugcatcher1__");
  // r.load("multi_blue_route3_after_bugcatcher1__");
  // r.run(WalkToPlan::new(12, 4)); // Youngster
  // r.run(WalkToPlan::new(13, 4)); // Youngster
  // r.run(OverworldInteractPlan::with(3)); // Youngster
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(2));
  // r.run(FightTurnPlan::new(AttackDesc::hit_with_side_effect(Move::Bubble, 8..=9, MoveEffectResult::Success), EnemyAttackDesc::Attack(AttackDesc::hit(Move::Tackle, 5..=5)), None));
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(SeqPlan::new(
  //   FightKOPlan::new(Move::Bubble, Some(MoveEffectResult::NoEffect), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::TailWhip))),
  //   NextTrainerMonPlan::with_level_up()));
  // r.run(SeqPlan::new(
  //   FightKOPlan::new(Move::Bubble, Some(MoveEffectResult::NoEffect), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Leer))),
  //   EndTrainerBattlePlan::new(1))); // Youngster fight //  #inputs: 38932 / 38943
  // r.save("multi_blue_route3_after_youngster_");
  // r.load("multi_blue_route3_after_youngster_");
  // r.run(WalkToPlan::new(18, 5)); // Bugcatcher
  // r.run(OverworldInteractPlan::with(5)); // Bugcatcher
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1));
  // r.run(SeqPlan::new(
  //   FightKOPlan::new(Move::Bubble, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::StringShot))),
  //   NextTrainerMonPlan::with_level_up()));
  // r.run(SeqPlan::new(
  //   FightKOPlan::new(Move::Bubble, None, EnemyAttackDesc::Attack(AttackDesc::stat_up_down(Move::Harden))),
  //   NextTrainerMonPlan::new()));
  // r.run(SeqPlan::new(
  //   FightKOPlan::new(Move::Bubble, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::StringShot))),
  //   NextTrainerMonPlan::new()));
  // r.run(SeqPlan::new(
  //   FightKOPlan::new(Move::Bubble, None, EnemyAttackDesc::Attack(AttackDesc::stat_up_down(Move::Harden))),
  //   EndTrainerBattlePlan::new(2))); // Bugcatcher fight //  #inputs: 42505 / 42487
  // r.save("multi_blue_route3_after_bugcatcher2_");
  // r.load("multi_blue_route3_after_bugcatcher2_");
  // r.run(WalkToPlan::new(24, 5)); // Bugcatcher
  // r.run(OverworldTurnPlan::new(D));
  // r.run(OverworldInteractPlan::with(8));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1));
  // r.run(FightKOPlan::new(Move::Bubble, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::StringShot))));
  // r.run(NextTrainerMonPlan::with_learn_move());
  // r.run(BattleMenuPlan::fight());
  // r.run(SelectMoveMenuPlan::new(Move::Bubble).use_select());
  // r.run(SelectMoveMenuPlan::new(Move::WaterGun).use_select());
  // r.run(FightKOPlan::new(Move::WaterGun, None, EnemyAttackDesc::NoAttack).skip_battle_menu());
  // r.run(EndTrainerBattlePlan::new(1)); // Bugcatcher fight //  #inputs: 44720 + 135 / 44703
  // r.save("multi_blue_route3_after_bugcatcher3_");
  // r.load("multi_blue_route3_after_bugcatcher3_");
  // r.run(WalkToEncounterPlan::new(37, 8, OverworldInteractionMetric.filter(|v| {
  //     if let OverworldInteractionResult::WildEncounter { species: Pokemon::Pidgey, level: _, dvs: _ } = v {
  //       log::info!("Pidgey");
  //       true
  //     } else { false }
  //   }).into_unit()));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Wild Pidgey appeared!
  // r.run(TextPlan::new().with_skip_ends(2)); // Go, Squirtle!
  // r.run(BattleMenuPlan::items());
  // r.run(SeqPlan::new(ListMenuPlan::choose(2), TextPlan::with_metric(CatchSuccessMetric, false).with_skip_ends(2))); // Poke Ball
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Pidgey was
  // r.run(SkipTextsPlan::new(1)); // caught!
  // r.run(SkipTextsPlan::new(1)); // New dex entry
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Pidgey!
  // r.run(TextScrollWaitPlan::new()); // dex entry
  // r.run(TextScrollWaitPlan::new()); // Scroll dex text 2
  // r.run(SkipTextsPlan::new(1)); // Give nickname?
  // r.run(TextPlan::new().with_skip_ends(2)); // to Pidgey?
  // r.run(TwoOptionMenuPlan::no());
  // r.run(WalkToPlan::new(59, -1)); // Enter Route 4
  // r.run(WalkToPlan::new(18, 5)); // Enter Mt. Moon // 47289 + 106 / 47267
  // r.save("multi_blue_mt_moon_");
  // r.load("multi_blue_mt_moon_");
  // r.run(WalkToPlan::new(17, 11)); // Enter Mt. Moon UF2
  // r.run(WalkToPlan::new(17, 11)); // Enter Mt. Moon UF3
  // r.run(WalkToPlan::new(29, 6)); // Enter Mt. Moon UF3
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with(9));
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found TM01
  // r.run(WalkToPlan::new(25, 9)); // Enter Mt. Moon UF2
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(3)); // TM01
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up TM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Mega Punch?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(0)); // Squirtle
  //   r.run(OverrideMovePlan::choose(1)); // Override Tail Whip
  //   r.run(ListMenuPlan::quit());
  //   r.run(StartMenuPlan::close());
  // }
  // r.run(WalkToPlan::new(25, 9)); // Enter Mt. Moon
  // r.run(WalkToPlan::new(5, 5)); // Enter Mt. Moon UF2
  // r.run(WalkToPlan::new(21, 17)); // Enter Mt. Moon UF3
  // r.run(WalkToPlan::new(10, 16)); // Rocket
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with(2));
  // r.save("multi_blue_test");
  // r.load("multi_blue_test"); // 52818
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(3));
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(FightKOPlan::new(Move::MegaPunch, None, EnemyAttackDesc::Attack(AttackDesc::hit(Move::Tackle, 7..=7))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::MegaPunch, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up(1)); // #inputs: 54874 + 150 / 54852
  // r.save("multi_blue_mt_moon_after_rocket_");
  // r.load("multi_blue_mt_moon_after_rocket_");
  // r.run(TextPlan::new().with_skip_ends(2));
  // r.run(EvolutionPlan::dont_cancel());
  // r.run(TextPlan::new().with_skip_ends(4));
  // r.run(WalkToPlan::new(13, 8)); // Super Nerd
  // r.run(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan::new())); // Super Nerd fight
  // r.run(OverworldWaitPlan::trainer_battle(208)); // Super Nerd fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Super Nerd fight
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightTurnPlan::new(AttackDesc::hit_no_side_effect(Move::Bubble, 10..=12), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Disable)), None));
  // r.run(FightKOPlan::new(Move::WaterGun, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(FightKOPlan::new(Move::MegaPunch, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Screech))));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_blue_test3");
  // r.load("multi_blue_test3");
  // r.run(FightTurnPlan::new(AttackDesc::crit(Move::WaterGun, 24..=27), EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::Smog)), None));
  // r.run(FightKOPlan::new(Move::Bubble, Some(MoveEffectResult::NoEffect), EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up(1)); // #inputs: 58926 + 155 / 58901
  // r.save("multi_blue_mt_moon_after_super_nerd_");
  // r.load("multi_blue_mt_moon_after_super_nerd_");
  // r.run(WalkToPlan::new(13, 7)); // Fossil
  // r.run(OverworldInteractPlan::with(7));
  // r.run(TextPlan::new()); // choose Helix Fossil?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan)); // chosen Helix Fossil
  // r.run(SeqPlan::new(TextPlan::new(), HoldTextDisplayOpenPlan)); // I choose this one then
  // r.run(WalkToPlan::new(4, 7)); // Leave Mt. Moon
  // r.run(WalkToPlan::new(5, 7)); // Leave Mt. Moon
  // r.run(WalkToPlan::new(27, 3)); // Leave Mt. Moon
  // r.run(SeqPlan::new(SeqPlan::new(
  //     WalkToPlan::new(65, 8),
  //     OverworldJumpLedgePlan::new(D)),
  //     WalkToEncounterPlan::new(90, 11, OverworldInteractionMetric.filter(|v| {
  //     if let OverworldInteractionResult::WildEncounter { species: Pokemon::Sandshrew, level: _, dvs: _ } = v {
  //       log::info!("Sandshrew");
  //       true
  //     } else { false }
  //   }).into_unit())));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Wild Sandshrew appeared!
  // r.run(TextPlan::new().with_skip_ends(2)); // Go, Wartortle!
  // r.run(BattleMenuPlan::items());
  // r.run(SeqPlan::new(ListMenuPlan::choose(2), TextPlan::with_metric(CatchSuccessMetric, false).with_skip_ends(2))); // Poke Ball
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Sandshrew was
  // r.run(SkipTextsPlan::new(1)); // caught!
  // r.run(SkipTextsPlan::new(1)); // New dex entry
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Sandshrew!
  // r.run(TextScrollWaitPlan::new()); // dex entry
  // r.run(TextScrollWaitPlan::new()); // Scroll dex text 2
  // r.run(SkipTextsPlan::new(1)); // Give nickname?
  // r.run(TextPlan::new().with_skip_ends(2)); // to Sandshrew?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(NamingScreenPlan::with_letter(b'S'));
  // r.run(WalkToPlan::new(90, 11)); // Enter Cerulean City // #inputs: 62507 + 151 / 62478
  // r.save("multi_blue_cerulean_");
  // r.load("multi_blue_cerulean_");
  // {
  //   r.run(WalkToPlan::new(13, 25)); // Enter Bike Shop
  //   r.run(WalkToPlan::new(6, 5)); // Enter Bike Shop
  //   r.run(WalkToPlan::new(6, 4)); // Enter Bike Shop
  //   r.run(OverworldInteractPlan::with(1));
  //   r.run(SkipTextsPlan::new(2)); // Bike Shop
  //   r.run(BikeShopMenuPlan::trigger_instant_text());
  //   r.run(SeqPlan::new(TextScrollWaitPlan::new(), HoldTextDisplayOpenPlan)); // cancel (IT)
  //   r.run(WalkToPlan::new(3, 6)); // Leave Bike Shop
  //   r.run(WalkToPlan::new(3, 7)); // Leave Bike Shop
  //   r.run(EdgeWarpPlan::new());
  // }
  // r.run(WalkToPlan::new(30, 19)); // Enter Gym
  // r.run(WalkToPlan::new(5, 3)); // Jr. Trainer F
  // r.run(SeqPlan::new(SkipTextsITPlan::new(2), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::trainer_battle(206)); // Jr. Trainer F fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(0));
  // r.run(FightKOITPlan::new(Move::MegaPunch, None, EnemyAttackDesc::Attack(AttackDesc::hit(Move::Peck, 2..=8))));
  // r.run(EndTrainerBattlePlan::with_level_up_it(1)); // #inputs: 65884 + 149 / 65854
  // r.save("multi_blue_cerulean_after_jrtrainerf_");
  // r.load("multi_blue_cerulean_after_jrtrainerf_");
  // r.run(WalkToPlan::new(5, 2)); // Misty
  // r.run(OverworldTurnPlan::new(L));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(9)); // Misty
  // r.run(FightKOITPlan::new(Move::MegaPunch, None, EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::Tackle))));
  // r.run(NextTrainerMonPlan::new_it());
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightKOITPlan::new(Move::MegaPunch, None, EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::Tackle))));
  // r.run(EndTrainerBattlePlan::with_level_up_it(4)); // #inputs: 68482 + 160 / 68449
  // r.save("multi_blue_cerulean_after_misty_");
  // r.load("multi_blue_cerulean_after_misty_");
  // r.run(SeqPlan::new(SkipTextsITPlan::new(8), HoldTextDisplayOpenPlan));
  // r.run(SeqPlan::new(SkipTextsITPlan::new(1), HoldTextDisplayOpenPlan));
  // r.run(WalkToPlan::new(5, 13)); // Leave Gym
  // r.run(EdgeWarpPlan::new()); // edge warp
  // r.run(WalkToPlan::new(19, 17)); // Enter Center
  // r.run(WalkToPlan::new(3, 3)); // Enter Center
  // r.run(OverworldInteractPlan::with(1)); // Center
  // r.run(SkipTextsITPlan::new(3)); // Center
  // r.run(SeqPlan::new(TextCommandPausePlan::new(), TwoOptionMenuPlan::yes())); // Center // cancels IT
  // r.run(TextPlan::new()); // Center
  // r.run(SkipTextsPlan::new(2)); // Center
  // r.run(SeqPlan::new(TextCommandPausePlan::new(), SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan))); // Center
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(4));
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up TM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach BubbleBeam?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(0)); // Wartortle
  //   r.run(OverrideMovePlan::choose(0)); // Override Water Gun
  //   r.run(ListMenuPlan::quit());
  //   r.run(StartMenuPlan::close());
  // }
  // r.run(WalkToPlan::new(3, 7)); // Leave Center
  // r.run(EdgeWarpPlan::new()); // edge warp
  // {
  //   r.run(WalkToPlan::new(13, 25)); // Enter Bike Shop
  //   r.run(WalkToPlan::new(6, 5)); // Enter Bike Shop
  //   r.run(WalkToPlan::new(6, 4)); // Enter Bike Shop
  //   r.run(OverworldInteractPlan::with(1));
  //   r.run(SkipTextsPlan::new(2)); // Bike Shop
  //   r.run(BikeShopMenuPlan::trigger_instant_text());
  //   r.run(SeqPlan::new(TextScrollWaitPlan::new(), HoldTextDisplayOpenPlan)); // cancel (IT)
  //   r.run(WalkToPlan::new(3, 6)); // Leave Bike Shop
  //   r.run(WalkToPlan::new(3, 7)); // Leave Bike Shop
  //   r.run(EdgeWarpPlan::new());
  // } // #inputs: 71738
  // r.run(WalkToPlan::new(21, 7)); // Trigger Rival fight
  // r.run(WalkToPlan::new(21, 6)); // Trigger Rival fight
  // r.run(SeqPlan::new(SkipTextsITPlan::new(8), HoldTextDisplayOpenPlan)); // Rival
  // r.run(OverworldWaitPlan::trainer_battle(225)); // initiate Rival fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(0));
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightTurnITPlan::new(AttackDesc::hit_no_side_effect(Move::Bubble, 9..=10), EnemyAttackDesc::Attack(AttackDesc::crit(Move::Gust, 15..=16)), None));
  // r.run(FightTurnITPlan::new(AttackDesc::crit(Move::BubbleBeam, 44..=49), EnemyAttackDesc::Attack(AttackDesc::hit(Move::QuickAttack, 10..=10)), None));
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(NextTrainerMonPlan::with_level_up_it());
  // r.run(FightKOITPlan::new(Move::MegaPunch, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::MegaPunch, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.save("multi_blue_test3");
  // r.load("multi_blue_test3");
  // r.run(FightTurnITPlan::new(AttackDesc::hit_failed(Move::MegaPunch), EnemyAttackDesc::Attack(AttackDesc::crit(Move::VineWhip, 30..=30)), None));
  // r.run(FightKOITPlan::new(Move::MegaPunch, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new_it(2)); // Rival fight //  #inputs: 75933 + 156 / 75895
  // r.save("multi_blue_cerulean_after_rival_");
  // r.load("multi_blue_cerulean_after_rival_");
  // r.run(SeqPlan::new(SkipTextsITPlan::new(14), HoldTextDisplayOpenPlan)); // Rival after battle texts
  // r.run(WalkToPlan::new(21, -1)); // Enter Route 24
  // r.run(WalkToPlan::new(11, 32)); // Nugget 1
  // r.run(OverworldInteractPlan::with(7));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(4)); // Nugget1
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up_it(1));
  // r.save("multi_blue_after_nugget1_"); //  #inputs: 78208 + 156 / 78170
  // r.load("multi_blue_after_nugget1_");
  // r.run(WalkToPlan::new(10, 29)); // Nugget 2
  // r.run(OverworldInteractPlan::with(6));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(1)); // Nugget2
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new_it(1));
  // r.save("multi_blue_after_nugget2_"); //  #inputs: 79637 + 156 / 79599
  // r.load("multi_blue_after_nugget2_");
  // r.run(WalkToPlan::new(11, 26)); // Nugget 3
  // r.run(OverworldInteractPlan::with(5));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(1)); // Nugget3
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new_it(1));
  // r.save("multi_blue_after_nugget3_"); // #inputs: 81373 + 156 / 81335
  // r.load("multi_blue_after_nugget3_");
  // r.run(WalkToPlan::new(10, 23)); // Nugget 4
  // r.run(OverworldInteractPlan::with(4));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(1)); // Nugget4
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up_it());
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new_it(1));
  // r.save("multi_blue_after_nugget4_"); //  #inputs: 82826 + 157 / 82787
  // r.load("multi_blue_after_nugget4_");
  // r.run(WalkToPlan::new(11, 20)); // Nugget 5
  // r.run(OverworldInteractPlan::with(3));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(1)); // Nugget5
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new_it(1)); // #inputs: 84002 + 158 / 83964
  // r.save("multi_blue_after_nugget5_");
  // r.load("multi_blue_after_nugget5_");
  // r.run(WalkToPlan::new(10, 15)); // Nugget 6
  // r.run(SkipTextsITPlan::new(3));
  // r.run(SkipTextsITPlan::new(1)); // got nugget
  // r.run(SeqPlan::new(SkipTextsITPlan::new(11), HoldTextDisplayOpenPlan)); // Rocket
  // r.run(OverworldWaitPlan::trainer_battle(230)); // initiate Rocket fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(0));
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new_it(1)); // #inputs: 85906 + 158 / 85867
  // r.save("multi_blue_after_nugget6_");
  // r.load("multi_blue_after_nugget6_");
  // r.run(SeqPlan::new(SkipTextsITPlan::new(3), HoldTextDisplayOpenPlan)); // Rocket after battle texts
  // r.run(WalkToPlan::new(20, 9)); // Route 25
  // r.run(WalkToPlan::new(8, 6)); // Hiker
  // r.run(WalkToPlan::new(8, 5)); // Hiker
  // r.run(OverworldInteractPlan::with(7));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(2)); // Hiker
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::Bubble, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up_it(1)); // #inputs: 88074 + 158 / 88035
  // r.save("multi_blue_route25_after_hiker_");
  // r.load("multi_blue_route25_after_hiker_");
  // r.run(WalkToPlan::new(18, 7)); // Lass1
  // r.run(OverworldTurnPlan::new(D));
  // r.run(OverworldInteractPlan::with(4));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(1)); // Lass1
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new_it(1)); // #inputs: 89778 + 158 / 89739
  // r.save("multi_blue_route25_after_lass1_");
  // r.load("multi_blue_route25_after_lass1_");
  // r.run(WalkToPlan::new(24, 6)); // Jr Trainer M
  // r.run(SeqPlan::new(SkipTextsITPlan::new(2), HoldTextDisplayOpenPlan)); // Jr Trainer M
  // r.run(OverworldWaitPlan::trainer_battle(205)); // initiate Jr Trainer M fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(0)); // Jr Trainer M
  // r.run(FightKOITPlan::new(Move::Bubble, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new_it(1)); // #inputs: 91444 + 157 / 91405
  // r.save("multi_blue_route25_after_jrtrainerm_");
  // r.load("multi_blue_route25_after_jrtrainerm_");
  // r.run(WalkToPlan::new(35, 4)); // Lass2
  // r.run(WalkToPlan::new(36, 4)); // Lass2
  // r.run(OverworldInteractPlan::with(6));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(2)); // Lass2
  // r.run(FightKOITPlan::new(Move::MegaPunch, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::Bubble, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::MegaPunch, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_move_override_it(1, 2)); // override Tackle with Bite // #inputs: 94006 + 164 / 93968
  // r.save("multi_blue_route25_after_lass2_");
  // r.load("multi_blue_route25_after_lass2_");
  // r.run(WalkToPlan::new(45, 3)); // Enter Bill's House
  // r.run(WalkToPlan::new(4, 5)); // Bill
  // r.run(WalkToPlan::new(5, 5)); // Bill
  // r.run(OverworldInteractPlan::with(1));
  // r.run(SkipTextsPlan::new(10));
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan));
  // r.run(WalkToPlan::new(1, 5)); // Desk
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item()); // interact with desk
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan));
  // r.run(WalkToPlan::new(3, 4)); // Bill
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with(2));
  // r.run(SkipTextsPlan::new(8));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // SS Anne Ticket
  // r.run(SeqPlan::new(SkipTextsPlan::new(9), HoldTextDisplayOpenPlan));
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(1)); // Escape Rope
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(StartMenuClosePlan::new());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // {
  //   r.run(WalkToPlan::new(13, 25)); // Enter Bike Shop
  //   r.run(WalkToPlan::new(6, 5)); // Enter Bike Shop
  //   r.run(WalkToPlan::new(6, 4)); // Enter Bike Shop
  //   r.run(OverworldInteractPlan::with(1));
  //   r.run(SkipTextsPlan::new(2)); // Bike Shop
  //   r.run(BikeShopMenuPlan::trigger_instant_text());
  //   r.run(SeqPlan::new(TextScrollWaitPlan::new(), HoldTextDisplayOpenPlan)); // cancel (IT)
  //   r.run(WalkToPlan::new(3, 6)); // Leave Bike Shop
  //   r.run(WalkToPlan::new(3, 7)); // Leave Bike Shop
  //   r.run(EdgeWarpPlan::new());
  // }
  // r.run(WalkToPlan::new(27, 11)); // Enter house
  // r.run(WalkToPlan::new(3, 0)); // Leave house
  // r.run(WalkToPlan::new(30, 9)); // Trigger Rocket fight
  // r.run(SeqPlan::new(SkipTextsITPlan::new(4), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::trainer_battle(230)); // initiate Rocket fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(0)); // Rocket
  // r.run(FightKOITPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(BattleMenuPlan::fight());
  // r.run(SelectMoveMenuPlan::new(Move::BubbleBeam).use_select());
  // r.run(SelectMoveMenuPlan::new(Move::Bite).use_select());
  // r.run(FightKOITPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack).skip_battle_menu());
  // r.run(EndTrainerBattlePlan::new_it(2)); // #inputs: 100303 + 180 / 100269
  // r.save("multi_blue_after_cerulean_rocket_");
  // r.load("multi_blue_after_cerulean_rocket_");
  // r.run(SeqPlan::new(SkipTextsITPlan::new(3), HoldTextDisplayOpenPlan)); // Rocket after battle texts
  // r.run(WalkToPlan::new(28, 36)); // Enter Route 5
  // r.run(WalkToPlan::new(17, 27)); // Underground
  // r.run(WalkToPlan::new(4, 4)); // Underground
  // {
  //   r.run(WalkToPlan::new(4, 4)); // Hidden item
  //   r.run(OverworldInteractPlan::with_hidden_item());
  //   r.run(HoldTextDisplayOpenPlan); // Found Full Heal
  // }
  // r.run(WalkToPlan::new(2, 40)); // Underground
  // r.run(WalkToPlan::new(2, 41)); // Underground
  // r.run(WalkToPlan::new(4, 7)); // Leave
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(11, 28)); // F
  // r.run(WalkToPlan::new(11, 29)); // F
  // r.run(OverworldInteractPlan::with(5));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(1)); // F
  // r.run(FightKOITPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new_it(1)); // #inputs: 104808 + 180 / 104799
  // r.save("multi_blue_after_route6_jrtrainerf_");
  // r.load("multi_blue_after_route6_jrtrainerf_");
  // r.run(WalkToPlan::new(10, 30)); // M
  // r.run(WalkToPlan::new(10, 31)); // M
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsITPlan::new(1), HoldTextDisplayOpenPlan)); // M
  // r.run(OverworldWaitPlan::trainer_battle(205)); // M fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(0)); // M fight
  // r.run(FightKOITPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up_it());
  // r.run(FightKOITPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new_it(1)); // #inputs: 106378 + 182 / 106370
  // r.save("multi_blue_after_route6_jrtrainerm_");
  // r.load("multi_blue_after_route6_jrtrainerm_");
  // r.run(WalkToPlan::new(9, 36)); // Vermilion City
  // r.run(WalkToPlan::new(18, 30)); // Dock
  // r.run(SeqPlan::new(SkipTextsITPlan::new(4), HoldTextDisplayOpenPlan)); // Dock
  // r.run(WalkToPlan::new(18, 31)); // Dock
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(14, 2)); // Dock
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(7, 7)); // Evade NPC
  // r.run(WalkToPlan::new(2, 6)); // 1F
  // r.run(WalkToPlan::new(3, 11)); // Avoid stairs
  // r.run(WalkToPlan::new(37, 9)); // Rival encounter
  // r.run(WalkToPlan::new(37, 8)); // Rival encounter
  // r.run(SeqPlan::new(SkipTextsITPlan::new(7), HoldTextDisplayOpenPlan)); // Rival
  // r.run(OverworldWaitPlan::trainer_battle(242)); // initiate Rival fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts_it(0));
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightKOITPlan::new(Move::MegaPunch, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new_it());
  // r.run(FightKOITPlan::new(Move::Bite, None, EnemyAttackDesc::Attack(AttackDesc::hit(Move::Confusion, 12..=13))));
  // r.run(NextTrainerMonPlan::with_level_up_it());
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(FightTurnITPlan::new(AttackDesc::hit_with_side_effect(Move::Bite, 20..=25, MoveEffectResult::Success), EnemyAttackDesc::NoAttack, None));
  // r.run(FightKOITPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new_it(3)); // Rival fight //  #inputs: 111760 + 187 / 111755
  // r.save("multi_blue_ssanne_after_rival_");
  // r.load("multi_blue_ssanne_after_rival_");
  // r.run(SeqPlan::new(SkipTextsITPlan::new(5), HoldTextDisplayOpenPlan)); // Rival texts
  // r.run(WalkToPlan::new(36, 4)); // 2F
  // r.run(WalkToPlan::new(4, 4)); // Captain
  // r.run(WalkToPlan::new(4, 3)); // Captain
  // r.run(OverworldInteractPlan::with(1));
  // r.run(SeqPlan::new(SkipTextsITPlan::new(13), HoldTextDisplayOpenPlan)); // got HM01
  // r.run(WalkToPlan::new(0, 7)); // 1F
  // r.run(WalkToPlan::new(2, 4)); // 0F
  // r.run(WalkToPlan::new(7, 7)); // Evade NPC
  // r.run(WalkToPlan::new(26, 0)); // Evade NPC
  // r.run(EdgeWarpPlan::with_metric(VermilionFirstTrashCanMetric.expect(4)));
  // r.run(WalkToPlan::new(15, 16)); // Cut bush
  // r.run(WalkToPlan::new(15, 17)); // Cut bush
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(7+1));
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up TM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Cut?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // learned move
  //   r.run(ListMenuPlan::quit());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(0)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(12, 19)); // Enter Gym
  // r.run(WalkToPlan::new(4, 9)); // Can 1
  // r.run(OverworldTurnPlan::new(L));
  // r.run(OverworldInteractPlan::with_hidden_item_metric(VermilionSecondTrashCanMetric.expect(7))); // First trash can
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan)); // First switch
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with_hidden_item()); // Second trash can
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Second switch
  // r.run(WalkToPlan::new(5, 3)); // Surge
  // r.run(WalkToPlan::new(5, 2)); // Surge
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(10)); // Surge
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(FightKOPlan::new(Move::MegaPunch, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Screech))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_blue_test3");
  // r.load("multi_blue_test3");
  // r.run(FightTurnPlan::new(AttackDesc::crit_with_side_effect(Move::BubbleBeam, 42..=42, MoveEffectResult::Success), EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::Thunderbolt)), None));
  // r.run(FightKOPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up(3)); // Surge fight //  #inputs: 121101+150 / 121096
  // r.save("multi_blue_after_surge_");
  // r.load("multi_blue_after_surge_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(5), HoldTextDisplayOpenPlan)); // Surge after fight texts
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Got TM
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Surge after fight texts
  // r.run(WalkToPlan::new(5, 17)); // leave
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(15, 19)); // Cut bush
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(0)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(9, 13)); // Fanclub
  // r.run(WalkToPlan::new(1, 1)); // Fanclub
  // r.run(WalkToPlan::new(2, 1)); // Fanclub
  // r.run(OverworldInteractPlan::with(5));
  // r.run(SkipTextsPlan::new(7)); // Fanclub
  // r.run(TextPlan::new()); // hear about mons?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SkipTextsPlan::new(18)); // Fanclub
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Got Voucher
  // r.run(SeqPlan::new(SkipTextsPlan::new(5), HoldTextDisplayOpenPlan)); // Fanclub
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(1)); // Escape Rope
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(StartMenuClosePlan::new());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // r.run(WalkToPlan::new(13, 25)); // Enter Bike Shop
  // r.run(WalkToPlan::new(6, 5)); // Enter Bike Shop
  // r.run(WalkToPlan::new(6, 4)); // Enter Bike Shop
  // r.run(OverworldInteractPlan::with(1));
  // r.run(SeqPlan::new(SkipTextsPlan::new(5), HoldTextDisplayOpenPlan)); // Get Bike
  // r.run(WalkToPlan::new(3, 6)); // Leave Bike Shop
  // r.run(WalkToPlan::new(3, 7)); // Leave Bike Shop
  // r.run(EdgeWarpPlan::new());
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::swap(0)); // Pokeballs
  //   r.run(ListMenuPlan::swap(9+1)); // Bike
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(19, 26)); // Bush
  // r.run(WalkToPlan::new(19, 27)); // Bush
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(0)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(40, 17)); // Route 9
  // r.run(WalkToPlan::new(4, 8)); // Bush
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(0)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(13, 8)); // JrTrainerF
  // r.run(WalkToPlan::new(13, 9)); // JrTrainerF
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(2)); // JrTrainerF
  // r.run(FightKOPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_blue_route9_after_jrtrainerf_"); // #inputs: 128967 + 150 / 128975
  // r.load("multi_blue_route9_after_jrtrainerf_");
  // r.run(WalkToPlan::new(12, 10));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(40, 10));
  // r.run(WalkToPlan::new(40, 9)); // Bugcatcher
  // r.run(OverworldInteractPlan::with(9));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Bugcatcher
  // r.run(BattleMenuPlan::fight());
  // r.run(FightKOPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_blue_route9_after_bugcatcher_"); // #inputs: 131503 + 153 / 131510
  // r.load("multi_blue_route9_after_bugcatcher_");
  // r.run(WalkToPlan::new(51, 4));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(60, 8)); // Route 10
  // r.run(WalkToPlan::new(8, 17)); // Rock Tunnel
  // r.run(WalkToPlan::new(23, 6));
  // r.run(WalkToPlan::new(23, 7)); // Pokemaniac
  // r.run(OverworldInteractPlan::with(4));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Pokemaniac
  // r.run(FightKOPlan::new(Move::Bubble, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightTurnPlan::new(AttackDesc::crit_with_side_effect(Move::Bite, 38..=41, MoveEffectResult::Success), EnemyAttackDesc::NoAttack, None));
  // r.run(FightKOPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 134364 + 153 / 134368
  // r.save("multi_blue_rock_tunnel_after_pokemaniac1_");
  // r.load("multi_blue_rock_tunnel_after_pokemaniac1_");
  // r.run(WalkToPlan::new(37, 3)); // B1F
  // r.run(WalkToPlan::new(27, 30)); // Pokemaniac
  // r.run(OverworldInteractPlan::with(8));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Pokemaniac
  // r.run(FightTurnPlan::new(AttackDesc::crit_with_side_effect(Move::Bite, 35..=38, MoveEffectResult::Success), EnemyAttackDesc::NoAttack, None));
  // r.run(FightKOPlan::new(Move::MegaPunch, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 136164 + 153 / 136166
  // r.save("multi_blue_rock_tunnel_after_pokemaniac2_");
  // r.load("multi_blue_rock_tunnel_after_pokemaniac2_");
  // r.run(WalkToPlan::new(14, 30)); // Lass
  // r.run(WalkToPlan::new(14, 29)); // Lass
  // r.run(OverworldInteractPlan::with(6));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(2)); // Lass
  // r.run(FightKOPlan::new(Move::MegaPunch, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 138056 + 183 / 138055
  // r.save("multi_blue_rock_tunnel_after_lass_");
  // r.load("multi_blue_rock_tunnel_after_lass_");
  // r.run(WalkToPlan::new(27, 3)); // 1F
  // r.run(WalkToPlan::new(17, 11)); // B1F
  // r.run(WalkToPlan::new(8, 10)); // Hiker
  // r.run(WalkToPlan::new(7, 10)); // Hiker
  // r.run(OverworldInteractPlan::with(2));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Hiker
  // r.run(FightKOPlan::new(Move::Bubble, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Bubble, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Bubble, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_blue_rock_tunnel_after_hiker_"); // #inputs: 141214 + 182 / 141213
  // r.load("multi_blue_rock_tunnel_after_hiker_");
  // r.run(WalkToPlan::new(3, 3)); // 1F
  // r.run(WalkToPlan::new(24, 24)); // JrTrainerF
  // r.run(WalkToPlan::new(23, 24)); // JrTrainerF
  // r.run(OverworldInteractPlan::with(6));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // JrTrainerF
  // r.run(FightKOPlan::new(Move::MegaPunch, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::MegaPunch, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up(1));
  // r.save("multi_blue_rock_tunnel_after_jrtrainerf_"); // #inputs: 143984 + 183 / 143983
  // r.load("multi_blue_rock_tunnel_after_jrtrainerf_");
  // r.run(WalkToPlan::new(15, 33)); // leave
  // r.run(WalkToPlan::new(15, 60)); // ledge
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(9, 72)); // Lavender Town
  // r.run(WalkToPlan::new(-1, 8)); // Route 8
  // r.run(WalkToPlan::new(47, 13)); // Juggler
  // r.run(OverworldInteractPlan::with(8));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Juggler
  // r.run(FightKOPlan::new(Move::MegaPunch, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Bite, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 146585 + 188 / 146584
  // r.save("multi_blue_route_8_after_juggler_");
  // r.load("multi_blue_route_8_after_juggler_");
  // r.run(WalkToPlan::new(13, 3)); // underground
  // r.run(WalkToPlan::new(4, 4)); // underground
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // // {
  // //   r.run(WalkToPlan::new(21, 3)); // Elixer
  // //   r.run(WalkToPlan::new(21, 4)); // Elixer
  // //   r.run(OverworldInteractPlan::with_hidden_item()); // Elixer
  // //   r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Elixer
  // // }
  // r.run(WalkToPlan::new(2, 4)); // underground
  // r.run(WalkToPlan::new(2, 5)); // underground
  // r.run(WalkToPlan::new(4, 7)); // underground
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(-1, 3)); // Celadon City // #inputs: 148581 + 188 / 148349
  // r.save("multi_blue_celadon_");
  // r.load("multi_blue_celadon_");
  // r.run(WalkToPlan::new(10, 13)); // Mart
  // r.run(WalkToPlan::new(12, 1)); // 2F
  // { // buy TM07 x2
  //   r.run(WalkToPlan::new(9, 3));
  //   r.run(WalkToPlan::new(8, 3));
  //   r.run(OverworldInteractPlan::with(2));
  //   r.run(TextPlan::new()); // How can I help you
  //   r.run(BuySellQuitMenuPlan::buy());
  //   r.run(TextPlan::new()); // Take your time
  //   r.run(SeqPlan::new(ListMenuPlan::choose(6), ChooseQuantityMenuPlan::new(2))); // Choose TM05 x2
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Item? // That will be
  //   r.run(TextPlan::new().with_skip_ends(2)); // Price // Okay?
  //   r.run(TwoOptionMenuPlan::yes()); // buy
  //   r.run(SkipTextsPlan::new(1)); // Here you go
  //   r.run(ListMenuPlan::quit()); // exit buy menu
  //   r.run(TextPlan::new()); // Anything else?
  //   r.run(BuySellQuitMenuPlan::quit());
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Thank you!
  // }
  // r.run(WalkToPlan::new(16, 1)); // 3F
  // r.run(WalkToPlan::new(12, 1)); // 4F
  // { // buy Doll x2
  //   r.run(WalkToPlan::new(5, 5));
  //   r.run(OverworldTurnPlan::new(D));
  //   r.run(OverworldInteractPlan::with(1));
  //   r.run(TextPlan::new()); // How can I help you
  //   r.run(BuySellQuitMenuPlan::buy());
  //   r.run(TextPlan::new()); // Take your time
  //   r.run(SeqPlan::new(ListMenuPlan::choose(0), ChooseQuantityMenuPlan::new(2))); // Choose Doll x2
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Item? // That will be
  //   r.run(TextPlan::new().with_skip_ends(2)); // Price // Okay?
  //   r.run(TwoOptionMenuPlan::yes()); // buy
  //   r.run(SkipTextsPlan::new(1)); // Here you go
  //   r.run(ListMenuPlan::quit()); // exit buy menu
  //   r.run(TextPlan::new()); // Anything else?
  //   r.run(BuySellQuitMenuPlan::quit());
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Thank you!
  // }
  // r.run(WalkToPlan::new(16, 1)); // 5F
  // r.run(WalkToPlan::new(12, 1)); // 6F
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(WalkToPlan::new(12, 3));
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with(5));
  // r.run(SkipTextsPlan::new(1)); // Vanding Machine text
  // r.run(VendingMachineMenuPlan::fresh_water());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(1), HoldTextDisplayOpenPlan)); // Vending Machine text
  // r.run(OverworldInteractPlan::with(5));
  // r.run(SkipTextsPlan::new(1)); // Vanding Machine text
  // r.run(VendingMachineMenuPlan::soda_pop());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(1), HoldTextDisplayOpenPlan)); // Vending Machine text
  // r.run(WalkToPlan::new(7, 4));
  // r.run(WalkToPlan::new(6, 4));
  // r.run(OverworldInteractPlan::with(2));
  // r.run(SkipTextsPlan::new(2));
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(VendingMachineMenuPlan::fresh_water());
  // r.run(SkipTextsPlan::new(4)); // Yay
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Received TM13
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // contains Ice Beam
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Can freeze
  // r.run(WalkToPlan::new(15, 2)); // 5F
  // r.run(WalkToPlan::new(16, 1)); // 4F
  // r.run(WalkToPlan::new(12, 1)); // 3F
  // r.run(WalkToPlan::new(16, 1)); // 2F
  // r.run(WalkToPlan::new(12, 1)); // 1F
  // r.run(WalkToPlan::new(16, 6));
  // r.run(WalkToPlan::new(16, 7));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(-1, 18));
  // r.run(WalkToPlan::new(34, 10)); //  #inputs: 177743+30
  // r.run(OverworldTurnPlan::new(U));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(0)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(WalkToPlan::new(25, 4));
  // r.run(WalkToPlan::new(24, 4));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(0, 2));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(7, 5)); // Fly House
  // r.run(WalkToPlan::new(2, 4)); // Fly
  // r.run(OverworldInteractPlan::with(1));
  // r.run(SeqPlan::new(SkipTextsPlan::new(5), HoldTextDisplayOpenPlan)); // Fly // #inputs: 154809+30
  // r.run(WalkToPlan::new(2, 7));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::swap(2)); // Poke Ball
  //   r.run(ListMenuPlan::swap(12)); // Doll
  //   r.run(ListMenuPlan::choose(11)); // TM05
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up TM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Mega Kick?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(0)); // Wartortle
  //   r.run(OverrideMovePlan::choose(1)); // Forget Mega Punch
  //   r.run(ListMenuPlan::choose(15)); // HM02
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up TM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Fly?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // learned move
  //   r.run(ListMenuPlan::choose(14)); // TM13
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up TM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Ice Beam?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(0)); // Wartortle
  //   r.run(OverrideMovePlan::choose(3)); // Forget Bubble
  //   r.run(ListMenuPlan::quit());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(PartyMonMenuPlan::choose(0)); // Fly
  //   r.run(FlyToPlan::to_lavender_town());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // r.run(WalkToPlan::new(14, 5)); // Enter Tower
  // r.run(WalkToPlan::new(18, 9)); // 2F
  // r.run(WalkToPlan::new(16, 5)); // Rival
  // r.run(WalkToPlan::new(15, 5)); // Rival
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan)); // Rival
  // r.run(OverworldWaitPlan::trainer_battle(242)); // initiate Rival fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.save("multi_blue_test3");
  // r.load("multi_blue_test3");
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::crit_with_side_effect(Move::Bite, 34..=40, MoveEffectResult::Success), EnemyAttackDesc::NoAttack, None));
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.save("multi_blue_test4");
  // r.load("multi_blue_test4");
  // r.run(NextTrainerMonPlan::with_skip_learning_move());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(2)); // Rival fight //  #inputs: 161374 + 223 / 161137
  // r.save("multi_blue_tower_after_rival_");
  // r.load("multi_blue_tower_after_rival_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(10), HoldTextDisplayOpenPlan)); // rival post-fight text
  // r.run(WalkToPlan::new(3, 9)); // 3F
  // r.run(WalkToPlan::new(18, 9)); // 4F
  // r.run(WalkToPlan::new(17, 7)); // Channeler
  // r.run(WalkToPlan::new(16, 7)); // Channeler
  // r.run(OverworldInteractPlan::with(2));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Channeler
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up(1)); // #inputs: 164851 + 224 / 164613
  // r.save("multi_blue_tower_after_channeler1_");
  // r.load("multi_blue_tower_after_channeler1_");
  // r.run(WalkToPlan::new(3, 9)); // 5F
  // r.run(WalkToPlan::new(11, 9)); // Heal pad
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // heal pad
  // r.run(WalkToPlan::new(18, 9)); // 6F
  // r.run(WalkToPlan::new(15, 5)); // Channeler
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Channeler
  // r.run(OverworldWaitPlan::trainer_battle(245)); // Channeler fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // { // Night Shade turn
  //   r.run(BattleMenuPlan::fight());
  //   r.run(SeqPlan::new(
  //     SelectMoveMenuPlan::with_metric(Move::IceBeam, ExpectedAIChooseMoveMetric { expected_move: Some(Move::NightShade) }),
  //     TextPlan::with_metric(Gen1MoveSuccessMetric.expect(FightTurnResult::Failed), false).with_skip_ends(4))); // A used IceBeam
  //   r.run(SeqPlan::new(
  //     SkipTextsPlan::new(1), // didn't affect
  //     TextPlan::with_metric(Gen1MoveSuccessMetric.expect(FightTurnResult::Succeeded), false).with_skip_ends(4))); // Gastly used Night Shade
  // }
  // r.save("multi_blue_test2_"); // 167217
  // r.load("multi_blue_test2_");
  // { // Night Shade turn
  //   r.run(BattleMenuPlan::fight());
  //   r.run(SeqPlan::new(
  //     SelectMoveMenuPlan::with_metric(Move::IceBeam, ExpectedAIChooseMoveMetric { expected_move: Some(Move::NightShade) }),
  //     TextPlan::with_metric(Gen1MoveSuccessMetric.expect(FightTurnResult::Failed), false).with_skip_ends(4))); // A used IceBeam
  //   r.run(SeqPlan::new(
  //     SkipTextsPlan::new(1), // didn't affect
  //     TextPlan::with_metric(Gen1MoveSuccessMetric.expect(FightTurnResult::Succeeded), false).with_skip_ends(4))); // Gastly used Night Shade
  // }
  // { // Night Shade turn
  //   r.run(BattleMenuPlan::fight());
  //   r.run(SeqPlan::new(
  //     SelectMoveMenuPlan::with_metric(Move::IceBeam, ExpectedAIChooseMoveMetric { expected_move: Some(Move::NightShade) }),
  //     TextPlan::with_metric(Gen1MoveSuccessMetric.expect(FightTurnResult::Failed), false).with_skip_ends(4))); // A used IceBeam
  //   r.run(SeqPlan::new(
  //     SkipTextsPlan::new(1), // didn't affect
  //     TextPlan::with_metric(Gen1MoveSuccessMetric.expect(FightTurnResult::Succeeded), false).with_skip_ends(4))); // Gastly used Night Shade
  // }
  // // { // Night Shade turn
  // //   r.run(BattleMenuPlan::fight());
  // //   r.run(SelectMoveMenuPlan::with_metric(Move::Bite, ExpectedAIChooseMoveMetric { expected_move: Some(Move::NightShade) }));
  // //   r.run(TextPlan::new().with_skip_ends(4)); // A used Bite
  // //   r.run(SeqPlan::new(
  // //     SkipTextsPlan::new(1), // didn't affect
  // //     TextPlan::with_metric(Gen1MoveSuccessMetric.expect(FightTurnResult::Succeeded), false).with_skip_ends(4))); // Gastly used Night Shade
  // // }
  // // { // Night Shade turn
  // //   r.run(BattleMenuPlan::fight());
  // //   r.run(SelectMoveMenuPlan::with_metric(Move::Bite, ExpectedAIChooseMoveMetric { expected_move: Some(Move::NightShade) }));
  // //   r.run(TextPlan::new().with_skip_ends(4)); // A used Bite
  // //   r.run(SeqPlan::new(
  // //     SkipTextsPlan::new(1), // didn't affect
  // //     TextPlan::with_metric(Gen1MoveSuccessMetric.expect(FightTurnResult::Succeeded), false).with_skip_ends(4))); // Gastly used Night Shade
  // // }
  // // { // Lick turn // TODO: swap with Rattata QA if Def 6 or 8
  // //   r.run(BattleMenuPlan::fight());
  // //   r.run(SelectMoveMenuPlan::with_metric(Move::Bite, ExpectedAIChooseMoveMetric { expected_move: Some(Move::Lick) }));
  // //   r.run(TextPlan::new().with_skip_ends(4)); // A used Bite
  // //   r.run(SeqPlan::new(
  // //     SkipTextsPlan::new(1), // didn't affect
  // //     TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(4, 7).expect(FightTurnResult::Hit { damage: 3 }).and_then(MoveEffectMetric).expect(MoveEffectResult::NoEffect), false).with_skip_ends(4))); // Gastly used Lick
  // // }
  // r.run(BattleMenuPlan::fight());
  // r.run(SelectMoveMenuPlan::new(Move::BubbleBeam).use_select());
  // r.run(SelectMoveMenuPlan::new(Move::Bite).use_select());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack).skip_battle_menu());
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 168583 + 199 / 168337
  // r.save("multi_blue_tower_after_channeler2_");
  // r.load("multi_blue_tower_after_channeler2_");
  // r.run(WalkToPlan::new(11, 5)); // Channeler
  // r.run(WalkToPlan::new(10, 5)); // Channeler
  // r.run(OverworldInteractPlan::with(2));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Channeler
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 170008 + 195 / 169760
  // r.save("multi_blue_tower_after_channeler3_");
  // r.load("multi_blue_tower_after_channeler3_");
  // r.run(WalkToPlan::new(6, 6)); // Rare Candy
  // r.run(WalkToPlan::new(6, 7)); // Rare Candy
  // r.run(OverworldInteractPlan::with(4));
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Rare Candy
  // r.run(WalkToPlan::new(10, 16)); // Ghost
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Ghost
  // r.run(OverworldWaitPlan::with_metric(OverworldInteractionMetric.filter(|r| if let OverworldInteractionResult::WildEncounter { species: Pokemon::Marowak, ..} = r { true } else { false })));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Wild // Ghost // appeared
  // r.run(SkipTextsPlan::new(1)); // Can't be ID'd
  // r.run(TextPlan::new().with_skip_ends(2));
  // r.run(BattleMenuPlan::items());
  // r.run(ListMenuPlan::choose(2)); // Doll
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Use doll
  // r.run(SkipTextsPlan::new(1)); // Marowak gone
  // r.run(TextPlan::new()); // Marowak gone
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Marowak gone // #inputs: 171323 / 171075
  // r.save("multi_blue_tower_after_marowak_");
  // r.load("multi_blue_tower_after_marowak_");
  // r.run(WalkToPlan::new(9, 16)); // 7F
  // r.run(WalkToPlan::new(10, 11)); // Rocket
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::trainer_battle(230)); // Rocket fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Rocket
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 173578 + 206 / 173330
  // r.save("multi_blue_tower_after_rocket1_");
  // r.load("multi_blue_tower_after_rocket1_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // not forget this
  // r.run(WalkToPlan::new(10, 9)); // Rocket
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::trainer_battle(230)); // Rocket fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Rocket
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::MegaKick, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 175837 + 208 / 175591
  // r.save("multi_blue_tower_after_rocket2_");
  // r.load("multi_blue_tower_after_rocket2_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan)); // not forget this
  // r.run(WalkToPlan::new(10, 7)); // Rocket
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::trainer_battle(230)); // Rocket fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Rocket
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(FightTurnPlan::new(AttackDesc::hit(Move::BubbleBeam, 50..=999), EnemyAttackDesc::Attack(AttackDesc::hit(Move::QuickAttack, 5..=5)), None));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // // r.run(FightTurnPlan::new(AttackDesc::hit(Move::BubbleBeam, 61..=999), EnemyAttackDesc::Attack(AttackDesc::hit(Move::QuickAttack, 8..=9)), None));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 178779 + 198 / 178525
  // r.save("multi_blue_tower_after_rocket3_");
  // r.load("multi_blue_tower_after_rocket3_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // not forget this
  // r.run(WalkToPlan::new(10, 4));
  // r.run(OverworldInteractPlan::with(4));
  // r.run(SeqPlan::new(SkipTextsPlan::new(12), HoldTextDisplayOpenPlan)); // Fuji
  // r.run(WalkToPlan::new(2, 1));
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with(5)); // Fuji
  // r.run(SkipTextsPlan::new(5)); // Fuji
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Poke FLute
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan)); // Fuji
  // r.run(WalkToPlan::new(2, 7));
  // r.run(EdgeWarpPlan::new());
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(PartyMonMenuPlan::choose(0)); // Fly
  //   r.run(FlyToPlan::to_celadon_city());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // // r.run(WalkToPlan::new(41, 9)); // Center
  // // r.run(WalkToPlan::new(3, 3)); // Center
  // // r.run(OverworldInteractPlan::with(1)); // Center
  // // r.run(SkipTextsPlan::new(3)); // Center
  // // r.run(TwoOptionMenuPlan::yes()); // Center
  // // r.run(TextPlan::new()); // Center
  // // r.run(SkipTextsPlan::new(2)); // Center
  // // r.run(SeqPlan::new(TextCommandPausePlan::new(), SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan))); // Center
  // // r.run(WalkToPlan::new(3, 7)); // go outside
  // // r.run(EdgeWarpPlan::new()); // go outside
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(-1, 18));
  // r.run(WalkToPlan::new(27, 10));
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::swap(3)); // Helix Fossil
  //   r.run(ListMenuPlan::swap(15)); // Rare Candy
  //   r.run(ListMenuPlan::choose(16)); // Flute
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Played flute
  //   r.run(StartMenuClosePlan::new());
  //   r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Snorlax fight
  //   r.run(OverworldWaitPlan::with_metric(OverworldInteractionMetric.filter(|r| if let OverworldInteractionResult::WildEncounter { species: Pokemon::Snorlax, .. } = r { true } else { false })));
  // }
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Wild Snorlax appeared!
  // r.run(TextPlan::new().with_skip_ends(2)); // Go, Wartortle!
  // r.run(BattleMenuPlan::run());
  // r.run(SkipTextsPlan::new(1)); // Got away safely! // #inputs: 183080 + 197 / 182880
  // r.save("multi_blue_after_snorlax_");
  // r.load("multi_blue_after_snorlax_");
  // r.run(WalkToPlan::new(24, 10));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(0, 8));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(1, 17));
  // r.run(WalkToPlan::new(1, 18));
  // r.run(WalkToPlan::new(6, 121));
  // r.run(WalkToPlan::new(7, 121));
  // r.run(OverworldInteractPlan::with_hidden_item()); // Max Elixer
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Max Elixer
  // r.run(WalkToPlan::new(7, 142));
  // r.run(OverworldJumpLedgePlan::new(D));
  // r.run(WalkToPlan::new(32, 8));
  // r.run(WalkToPlan::new(33, 8));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(7, 4));
  // r.run(EdgeWarpPlan::new());
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(50, 8)); // Fuchsia
  // r.run(WalkToPlan::new(18, 20));
  // r.run(OverworldTurnPlan::new(U)); // Cut bush
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(0)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(16, 13));
  // r.run(WalkToPlan::new(16, 12));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(0)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(18, 3)); // Enter Safari // 186582
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(WalkToPlan::new(3, 2));
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Welcome
  // r.run(OverworldWaitPlan::auto_walk(R));
  // r.run(SkipTextsPlan::new(3)); // Welcome
  // r.run(TextPlan::new()); // Do Safari?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan)); // Welcome
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(29, 11));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(0, 5));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(3, 35));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(19, 5));
  // r.run(WalkToPlan::new(19, 6));
  // r.run(OverworldInteractPlan::with(4));
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Gold Teeth
  // r.run(WalkToPlan::new(3, 3)); // Surf house
  // r.run(WalkToPlan::new(3, 5));
  // r.run(WalkToPlan::new(3, 4));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(SkipTextsPlan::new(7)); // Surf
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan)); // Get HM
  // r.run(WalkToPlan::new(3, 7));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(1)); // Escape Rope
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(StartMenuClosePlan::new());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(PartyMonMenuPlan::choose(0)); // Fly
  //   r.run(FlyToPlan::to_celadon_city());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // r.save("multi_blue_test3");
  // r.load("multi_blue_test3");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(35, 30));
  // r.run(WalkToPlan::new(35, 31));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(0)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(12, 27)); // Enter Gym
  // r.run(WalkToPlan::new(1, 4));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(0)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(3, 4)); // Beauty
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Beauty
  // r.run(OverworldWaitPlan::trainer_battle(218)); // Beauty fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Beauty fight
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 195288 + 199 / 195090
  // r.save("multi_blue_celadon_after_beauty_");
  // r.load("multi_blue_celadon_after_beauty_");
  // r.run(WalkToPlan::new(4, 4)); // Erika
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(13)); // Erika
  // // r.run(FightTurnPlan::new(AttackDesc::hit_failed(Move::MegaKick), EnemyAttackDesc::Attack(AttackDesc::crit(Move::RazorLeaf, 78..=78)), None));
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(3)); // #inputs: 198168 + 198 / 197969
  // r.save("multi_blue_celadon_after_erika_");
  // r.load("multi_blue_celadon_after_erika_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan)); // after texts
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // not enough space
  // r.run(WalkToPlan::new(5, 5));
  // r.run(WalkToPlan::new(5, 6));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(0)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(5, 17));
  // r.run(EdgeWarpPlan::new()); // edge warp
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::swap(2)); // Doll
  //   r.run(ListMenuPlan::swap(17)); // Max Elixer
  //   r.run(ListMenuPlan::choose(19)); // HM03
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up HM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Surf?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(0)); // Wartortle
  //   r.run(OverrideMovePlan::choose(2)); // Forget Bite
  //   r.run(ListMenuPlan::quit());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(PartyMonMenuPlan::choose(0)); // Fly
  //   r.run(FlyToPlan::to_fuchsia_city());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // r.run(WalkToPlan::new(5, 27)); // Enter Gym
  // r.run(WalkToPlan::new(7, 9));
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with(3));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(2)); // Juggler
  // r.run(FightKOPlan::new(Move::Surf, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::Surf, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::MegaKick, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Disable))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::MegaKick, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 203870 + 211 / 203666
  // r.save("multi_blue_fuchsia_after_juggler1_");
  // r.load("multi_blue_fuchsia_after_juggler1_");
  // r.run(WalkToPlan::new(1, 7)); // Juggler
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan)); // Juggler
  // r.run(OverworldWaitPlan::trainer_battle(221)); // Juggler fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Juggler
  // r.run(FightKOPlan::new(Move::MegaKick, None, EnemyAttackDesc::NoAttack));
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(NextTrainerMonPlan::new());
  // // r.run(FightTurnPlan::new(AttackDesc::crit(Move::Surf, 63..=68), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Disable)), None));
  // r.run(FightTurnPlan::new(AttackDesc::crit(Move::Surf, 63..=68), EnemyAttackDesc::Attack(AttackDesc::hit(Move::Headbutt, 14..=14)), None));
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up(1)); // #inputs: 206800 + 210 / 206682
  // r.save("multi_blue_fuchsia_after_juggler2_");
  // r.load("multi_blue_fuchsia_after_juggler2_");
  // r.run(TextPlan::new().with_skip_ends(2));
  // r.run(EvolutionPlan::dont_cancel());
  // r.run(TextPlan::new().with_skip_ends(4));
  // r.run(WalkToPlan::new(3, 10)); // Koga
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(9)); // Koga
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Disable))));
  // // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::Attack(AttackDesc::hit_no_side_effect(Move::Sludge, 30..=30))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // { // Selfdestruct turn
  //   r.run(BattleMenuPlan::fight());
  //   r.run(SeqPlan::new(
  //     SelectMoveMenuPlan::with_metric(Move::MegaKick, ExpectedAIChooseMoveMetric { expected_move: Some(Move::SelfDestruct) }),
  //     TextPlan::with_metric(Gen1MoveSuccessMetric.expect(FightTurnResult::Failed), false).with_skip_ends(4))); // A used BubbleBeam
  //   r.run(SeqPlan::new(
  //     SkipTextsPlan::with_metric(1, TrainerAIMetric.expect(TrainerAIAction::NoAction)), // but it failed
  //     TextPlan::with_metric(Gen1MoveSuccessMetric.expect(FightTurnResult::Failed), false).with_skip_ends(4))); // Weezing uses Self Destruct
  //   r.run(SkipTextsPlan::new(1)); // missed
  // }
  // r.run(EndTrainerBattlePlan::new(3)); // #inputs: 211304 + 218 / 210911
  // r.save("multi_blue_fuchsia_after_koga_");
  // r.load("multi_blue_fuchsia_after_koga_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan)); // after texts
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // not enough space
  // r.run(WalkToPlan::new(5, 16));
  // r.run(WalkToPlan::new(5, 17));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(2)); // Max Elixer
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(PartyMenuPlan::choose(0)); // Blastoise
  //   r.run(SkipTextsPlan::new(1)); // PP restored
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(22, 30));
  // r.run(OverworldJumpLedgePlan::new(R)); // Jump ledge
  // r.run(WalkToPlan::new(27, 27)); // Strength house
  // r.run(WalkToPlan::new(2, 5));
  // r.run(WalkToPlan::new(2, 4));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(SkipTextsPlan::new(10)); // after texts
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan)); // Get HM04
  // r.run(WalkToPlan::new(4, 6));
  // r.run(WalkToPlan::new(4, 7));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(PartyMonMenuPlan::choose(0)); // Fly
  //   r.run(FlyToPlan::to_pallet_town());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // r.run(WalkToPlan::new(4, 12));
  // r.run(WalkToPlan::new(4, 13));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(0)); // Blastoise
  //   r.run(PartyMonMenuPlan::choose(0)); // Surf
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Got on
  //   r.run(OverworldWaitPlan::auto_walk(D));
  // }
  // r.run(WalkToPlan::new(4, 18));
  // r.run(WalkToPlan::new(3, 90));
  // r.run(WalkToPlan::new(4, 4));
  // r.run(WalkToPlan::new(6, 3));
  // r.run(WalkToPlan::new(5, 10)); // 2F
  // r.run(WalkToPlan::new(6, 1)); // 3F
  // r.run(WalkToPlan::new(10, 6)); // 2F
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item());
  // r.run(SkipTextsPlan::new(1)); // Switch
  // r.run(TextPlan::new()); // press?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Who wouldn't?
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(WalkToPlan::new(16, 14));
  // r.run(OverworldWaitPlan::fly_warp());
  // r.run(WalkToPlan::new(21, 23)); // B1F
  // r.run(WalkToPlan::new(18, 26)); // Switch
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item());
  // r.run(SkipTextsPlan::new(1)); // Switch
  // r.run(TextPlan::new()); // press?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Who wouldn't?
  // r.run(WalkToPlan::new(20, 4)); // Switch
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item());
  // r.run(SkipTextsPlan::new(1)); // Switch
  // r.run(TextPlan::new()); // press?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Who wouldn't?
  // r.run(WalkToPlan::new(5, 12)); // Secret Key
  // r.run(OverworldInteractPlan::with(8));
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Secret Key
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(1)); // Escape Rope
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(StartMenuClosePlan::new());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(PartyMonMenuPlan::choose(0)); // Fly
  //   r.run(FlyToPlan::to_cinnabar_island());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(18, 3)); // Gym
  // r.run(WalkToPlan::new(15, 9)); // Q1
  // r.run(WalkToPlan::new(15, 8)); // Q1
  // r.run(OverworldInteractPlan::with_hidden_item()); // Q1
  // r.run(SkipTextsPlan::new(8)); // Q1
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Q1
  // r.run(WalkToPlan::new(10, 2)); // Q2
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item()); // Q2
  // r.run(SkipTextsPlan::new(9)); // Q2
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::no());
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Q2
  // r.run(WalkToPlan::new(9, 8)); // Q3
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item()); // Q3
  // r.run(SkipTextsPlan::new(8)); // Q3
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::no());
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Q3
  // r.run(WalkToPlan::new(9, 14)); // Q4
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item()); // Q4
  // r.run(SkipTextsPlan::new(10)); // Q4
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::no());
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Q4
  // r.run(WalkToPlan::new(1, 15)); // Q5
  // r.run(WalkToPlan::new(1, 14)); // Q5
  // r.run(OverworldInteractPlan::with_hidden_item()); // Q5
  // r.run(SkipTextsPlan::new(10)); // Q5
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Q5
  // r.run(WalkToPlan::new(1, 9)); // Q6
  // r.run(WalkToPlan::new(1, 8)); // Q6
  // r.run(OverworldInteractPlan::with_hidden_item()); // Q6
  // r.run(SkipTextsPlan::new(8)); // Q6
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::no());
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Q6
  // r.run(WalkToPlan::new(3, 5)); // Blaine
  // r.run(WalkToPlan::new(3, 4)); // Blaine
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(6)); // Blaine
  // r.save("multi_blue_test3");
  // r.load("multi_blue_test3");
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::Attack(AttackDesc::hit_no_side_effect(Move::Stomp, 21..=21))));
  // // {
  // //   r.run(BattleMenuPlan::fight());
  // //   r.run(SelectMoveMenuPlan::with_metric(Move::BubbleBeam, TrainerAIMetric.expect(TrainerAIAction::Potion)));
  // //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Used Super Potion
  // //   r.run(SeqPlan::new(
  // //     SkipTextsPlan::new(1).with_skip_ends(2), // On Rapidash
  // //     TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(72, 138).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 113 } else { false }), false).with_skip_ends(4))); // Blastoise used BubbleBeam
  // //   r.run(SkipTextsPlan::new(2)); // Crit + effective
  // // }
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.save("multi_blue_test4");
  // r.load("multi_blue_test4");
  // {
  //   r.run(BattleMenuPlan::fight());
  //   r.run(SelectMoveMenuPlan::with_metric(Move::Surf, TrainerAIMetric.expect(TrainerAIAction::Potion)));
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Used Super Potion
  //   r.run(SeqPlan::new(
  //     SkipTextsPlan::new(1).with_skip_ends(2), // On Arcanine
  //     TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(102, 188).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 149 } else { false }), false).with_skip_ends(4))); // Blastoise used Surf
  //   r.run(SkipTextsPlan::new(2)); // Crit + effective
  // }
  // r.run(EndTrainerBattlePlan::new(2)); // #inputs: 230506 + 237 / 230262
  // r.save("multi_blue_after_blaine_");
  // r.load("multi_blue_after_blaine_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(5), HoldTextDisplayOpenPlan)); // after texts
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // not enough space
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(1)); // Escape Rope
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(StartMenuClosePlan::new());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(PartyMonMenuPlan::choose(0)); // Fly
  //   r.run(FlyToPlan::to_celadon_city());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(50, 10));
  // r.run(WalkToPlan::new(11, 10));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(3, 4));
  // r.run(SeqPlan::new(SkipTextsPlan::new(15), HoldTextDisplayOpenPlan)); // Give drink // #inputs: 252542
  // r.run(WalkToPlan::new(5, 4));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(20, 10));
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(WalkToPlan::new(18, 21)); // enter Silph Co
  // r.run(WalkToPlan::new(26, 0)); // 2F
  // r.run(WalkToPlan::new(26, 0)); // 3F
  // r.run(WalkToPlan::new(24, 0)); // 4F
  // r.run(WalkToPlan::new(26, 0)); // 5F
  // {
  //   r.run(WalkToPlan::new(14, 3));
  //   r.run(WalkToPlan::new(13, 3));
  //   r.run(OverworldInteractPlan::with_hidden_item()); // Elixer
  //   r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Elixer
  // }
  // r.run(WalkToPlan::new(8, 14));
  // r.run(WalkToPlan::new(8, 15));
  // r.run(OverworldInteractPlan::with(2));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Rocket
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up(1)); // #inputs: 236282 + 336 / 236270
  // r.save("multi_blue_saffron_after_rocket1_");
  // r.load("multi_blue_saffron_after_rocket1_");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(2)); // Rare Candy
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(PartyMenuPlan::choose(0)); // Blastoise
  //   r.run(TextScrollWaitPlan::new()); // Rose to lv 40
  //   r.run(TextScrollWaitPlan::new()); // Rose to lv 40
  //   // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Rose to lv 40
  //   r.run(ListMenuPlan::quit());
  //   r.run(StartMenuPlan::close());
  // }
  // r.run(WalkToPlan::new(9, 15));
  // r.run(WalkToPlan::new(17, 16));
  // r.run(WalkToPlan::new(17, 15));
  // r.run(WalkToPlan::new(20, 16));
  // r.run(OverworldInteractPlan::with(8));
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Card Key
  // r.run(WalkToPlan::new(9, 15));
  // r.run(WalkToPlan::new(17, 16));
  // r.run(WalkToPlan::new(17, 15));
  // r.run(WalkToPlan::new(9, 13));
  // r.run(WalkToPlan::new(8, 13));
  // r.run(OverworldInteractPlan::with_card_key_door()); // Door
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Card Key
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Card Key
  // r.run(WalkToPlan::new(3, 15));
  // r.run(WalkToPlan::new(18, 9));
  // r.run(OverworldTurnPlan::new(L));
  // r.run(OverworldInteractPlan::with_card_key_door()); // Door
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Card Key
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Card Key
  // r.run(WalkToPlan::new(11, 11));
  // r.run(WalkToPlan::new(3, 2));
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Rival
  // r.run(SeqPlan::new(SkipTextsPlan::new(9), HoldTextDisplayOpenPlan)); // Rival
  // r.run(OverworldWaitPlan::trainer_battle(242)); // initiate Rival fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Leer))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::MegaKick, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(2)); // #inputs: 242561 + 343 / 242806
  // r.save("multi_blue_saffron_after_rival_");
  // r.load("multi_blue_saffron_after_rival_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(14), HoldTextDisplayOpenPlan)); // Rival after texts
  // r.run(WalkToPlan::new(5, 7));
  // r.run(WalkToPlan::new(2, 16));
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with(4));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Rocket
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 245960 + 344 / 246030
  // r.save("multi_blue_saffron_after_rocket2_");
  // r.load("multi_blue_saffron_after_rocket2_");
  // r.run(WalkToPlan::new(6, 15));
  // r.run(WalkToPlan::new(6, 14));
  // r.run(OverworldInteractPlan::with_card_key_door()); // Door
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Card Key
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Card Key
  // r.run(WalkToPlan::new(6, 13));
  // r.run(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan)); // Giovanni
  // r.run(OverworldWaitPlan::trainer_battle(229)); // Giovanni fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Giovanni fight
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_skip_learning_move());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 249738 + 346 / 249775
  // r.save("multi_blue_saffron_after_giovanni_");
  // r.load("multi_blue_saffron_after_giovanni_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan)); // Giovanni post-fight text
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(1)); // Escape Rope
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(StartMenuClosePlan::new());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(PartyMonMenuPlan::choose(0)); // Fly
  //   r.run(FlyToPlan::to_saffron_city());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(36, 31));
  // r.run(WalkToPlan::new(34, 3)); // Enter Gym
  // r.run(WalkToPlan::new(11, 15));
  // r.run(WalkToPlan::new(15, 15));
  // r.run(WalkToPlan::new(15, 5));
  // r.run(WalkToPlan::new(1, 5));
  // r.run(WalkToPlan::new(9, 10));
  // r.run(WalkToPlan::new(9, 9));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(8)); // Sabrina
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightKOPlan::new(Move::MegaKick, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::Surf, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Surf, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(FightKOPlan::new(Move::MegaKick, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Recover))));
  // r.run(EndTrainerBattlePlan::new(6)); // #inputs: 256200 + 337 / 256131 256106
  // r.save("multi_blue_saffron_after_sabrina__");
  // r.load("multi_blue_saffron_after_sabrina_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan)); // after texts
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // not enough space
  // r.run(WalkToPlan::new(11, 11));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(1)); // Escape Rope
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(StartMenuClosePlan::new());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(PartyMonMenuPlan::choose(0)); // Fly
  //   r.run(FlyToPlan::to_viridian_city());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(32, 7)); // Enter Gym
  // r.run(WalkToPlan::new(15, 5));
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::trainer_battle(231)); // Cooltrainer fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Cooltrainer
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 259998 + 340 / 259931
  // r.save("multi_blue_viridian_after_cooltrainerm_");
  // r.load("multi_blue_viridian_after_cooltrainerm_");
  // r.run(WalkToPlan::new(10, 4));
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::trainer_battle(224)); // Blackbelt fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Blackbelt
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 262515 + 342 / 262493
  // r.save("multi_blue_viridian_after_blackbelt_");
  // r.load("multi_blue_viridian_after_blackbelt_");
  // r.run(WalkToPlan::new(16, 16));
  // r.run(WalkToPlan::new(16, 17));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(32, 7)); // Enter Gym
  // r.run(WalkToPlan::new(15, 6));
  // r.run(WalkToPlan::new(2, 3));
  // r.run(WalkToPlan::new(2, 2));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(10)); // Giovanni
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // {
  //   r.run(BattleMenuPlan::fight());
  //   r.run(SelectMoveMenuPlan::with_metric(Move::BubbleBeam, TrainerAIMetric.expect(TrainerAIAction::GuardSpec)));
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Used Guard Spec.
  //   r.run(SeqPlan::new(
  //     SkipTextsPlan::new(1).with_skip_ends(2), // On Dugtrio
  //     TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(126, 216).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 88 } else { false }), false).with_skip_ends(4))); // Blastoise used BubbleBeam
  //   r.run(SkipTextsPlan::new(1)); // effective
  // }
  // // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::Attack(AttackDesc::hit(Move::Slash, 19..=23)))); // min dmg 129
  // r.save("multi_blue_test3");
  // r.load("multi_blue_test3");
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.save("multi_blue_test4");
  // r.load("multi_blue_test4");
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // mon // gained // num XP
  // // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // level up
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // I defeated U
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // First defeat text
  // r.run(SkipTextsPlan::new(3)); // Additional defeat texts
  // r.run(TextPlan::new()); // Additional defeat texts
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // player got // X for winning // #inputs: 268135 + 248 / 267715
  // r.save("multi_blue_viridian_after_giovanni_");
  // r.load("multi_blue_viridian_after_giovanni_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan)); // after texts
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan)); // after texts
  // r.run(WalkToPlan::new(15, 6));
  // r.run(WalkToPlan::new(16, 16));
  // r.run(WalkToPlan::new(16, 17));
  // r.run(EdgeWarpPlan::new());
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::swap(1)); // Nugget
  //   r.run(ListMenuPlan::swap(17)); // Elixer
  //   r.run(ListMenuPlan::choose(15)); // HM04
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up HM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Strength?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // learned move
  //   r.run(ListMenuPlan::choose(19)); // TM27
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up TM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Fissure?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(0)); // Blastoise
  //   r.run(OverrideMovePlan::choose(0)); // Override BubbleBeam
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(OverworldJumpLedgePlan::new(D)); // 49
  // r.run(WalkToPlan::new(-1, 17)); // Route 22
  // r.run(WalkToPlan::new(29, 5)); // Rival
  // r.run(SeqPlan::new(SkipTextsPlan::new(10), HoldTextDisplayOpenPlan)); // Rival
  // r.run(OverworldWaitPlan::trainer_battle(242)); // initiate Rival fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::Attack(AttackDesc::hit(Move::Bite, 20..=28))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_blue_test3");
  // r.load("multi_blue_test3");
  // {
  //   r.run(BattleMenuPlan::fight());
  //   r.run(SelectMoveMenuPlan::with_metric(Move::MegaKick, AIChooseMoveMetric.expect(Move::Reflect)));
  //   r.run(TextPlan::new().with_skip_ends(4)); // Used reflect
  //   r.run(SeqPlan::new(
  //     SkipTextsPlan::new(1), // gained armor
  //     TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(50, 165).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 123 } else { false }), false).with_skip_ends(4))); // Blastoise used MegaKick
  //   r.run(SkipTextsPlan::new(1)); // critical hit
  // }
  // // r.run(FightKOPlan::new(Move::MegaKick, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Recover))));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::new(2)); // #inputs:  275994 / 275805 275785
  // r.save("multi_blue_route22_after_rival__");
  // r.load("multi_blue_route22_after_rival_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(5), HoldTextDisplayOpenPlan)); // Rival
  // r.run(WalkToPlan::new(21, 6));
  // r.run(OverworldJumpLedgePlan::new(D));
  // r.run(WalkToPlan::new(8, 5));
  // r.run(WalkToPlan::new(4, 2));
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Badge check
  // r.run(WalkToPlan::new(4, 0));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(7, 136));
  // r.run(SkipTextsPlan::new(1)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Badge check
  // r.run(WalkToPlan::new(9, 119));
  // r.run(SkipTextsPlan::new(1)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Badge check
  // r.run(WalkToPlan::new(10, 105));
  // r.run(SkipTextsPlan::new(1)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Badge check
  // r.run(WalkToPlan::new(10, 104));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(0)); // Blastoise
  //   r.run(PartyMonMenuPlan::choose(0)); // Surf
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Got on
  //   r.run(OverworldWaitPlan::auto_walk(U));
  // }
  // r.run(WalkToPlan::new(10, 96));
  // r.run(SkipTextsPlan::new(1)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Badge check
  // r.run(WalkToPlan::new(10, 85));
  // r.run(SkipTextsPlan::new(1)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Badge check
  // r.run(WalkToPlan::new(8, 71));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(11, 56));
  // r.run(SkipTextsPlan::new(1)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Badge check
  // r.run(WalkToPlan::new(7, 35));
  // r.run(SkipTextsPlan::new(1)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Badge check
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Badge check
  // r.run(WalkToPlan::new(4, 31));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(1)); // Strength
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Used Strength
  // }
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(WalkToPlan::new(5, 14));
  // r.run(OverworldPushBoulderPlan::new(D));
  // r.run(WalkToPlan::new(4, 16));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(5, 16));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(6, 16));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(7, 16));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(7, 17));
  // r.run(WalkToPlan::new(9, 17));
  // r.run(OverworldPushBoulderPlan::new(U));
  // r.run(WalkToPlan::new(9, 16));
  // r.run(OverworldPushBoulderPlan::new(U));
  // r.run(WalkToPlan::new(8, 14));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(9, 14));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(10, 14));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(11, 14));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(12, 14));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(13, 14));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(14, 14));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(16, 15));
  // r.run(OverworldPushBoulderPlan::new(U));
  // r.run(WalkToPlan::new(16, 14));
  // r.run(OverworldPushBoulderPlan::new(U));
  // r.run(WalkToPlan::new(15, 12));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(17, 11));
  // r.run(OverworldPushBoulderPlan::new(D));
  // r.run(WalkToPlan::new(1, 1)); // 2F
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(1)); // Strength
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Used Strength
  // }
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(WalkToPlan::new(5, 14));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(3, 13));
  // r.run(OverworldPushBoulderPlan::new(D));
  // r.run(WalkToPlan::new(3, 14));
  // r.run(OverworldPushBoulderPlan::new(D));
  // r.run(WalkToPlan::new(4, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(3, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(23, 7)); // 3F
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(1)); // Strength
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Used Strength
  // }
  // r.save("multi_blue_test3");
  // r.load("multi_blue_test3");
  // r.run(WalkToPlan::new(22, 5));
  // r.run(WalkToPlan::new(22, 4));
  // r.run(OverworldPushBoulderPlan::new(U));
  // r.run(WalkToPlan::new(22, 3));
  // r.run(OverworldPushBoulderPlan::new(U));
  // r.run(WalkToPlan::new(23, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(22, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(21, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(20, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(19, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(18, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(17, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(16, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(15, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(14, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(13, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(12, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(11, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(10, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(9, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(8, 1));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(6, 0));
  // r.run(OverworldPushBoulderPlan::new(D));
  // r.run(WalkToPlan::new(7, 2));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(6, 2));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(5, 2));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(4, 2));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(2, 1));
  // r.run(OverworldPushBoulderPlan::new(D));
  // r.run(WalkToPlan::new(2, 2));
  // r.run(OverworldPushBoulderPlan::new(D));
  // r.run(WalkToPlan::new(2, 3));
  // r.run(OverworldPushBoulderPlan::new(D));
  // r.run(WalkToPlan::new(1, 5));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(17, 5));
  // r.run(WalkToPlan::new(20, 15));
  // r.run(WalkToPlan::new(21, 15));
  // r.run(OverworldPushBoulderPlan::new(R));
  // r.run(WalkToPlan::new(23, 15));
  // r.run(OverworldWaitPlan::fly_warp());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Sandshrew
  //   r.run(PartyMonMenuPlan::choose(1)); // Strength
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Used Strength
  // }
  // r.save("multi_blue_test4");
  // r.load("multi_blue_test4");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(24, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(23, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(22, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(21, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(20, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(19, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(18, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(17, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(16, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(15, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(14, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(13, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(12, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(11, 16));
  // r.run(OverworldPushBoulderPlan::new(L));
  // r.run(WalkToPlan::new(22, 14));
  // r.run(WalkToPlan::new(25, 14));
  // r.run(WalkToPlan::new(26, 8));
  // r.run(WalkToPlan::new(29, 7));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(10, -1));
  // r.run(WalkToPlan::new(10, 5));
  // r.run(WalkToPlan::new(15, 9));
  // r.run(WalkToPlan::new(15, 8));
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(OverworldInteractPlan::with_hidden_item()); // PC
  // r.run(SkipTextsPlan::new(1)); // PC access
  // r.run(PCMainMenuPlan::mon()); // Bill's PC
  // r.run(SkipTextsPlan::new(2)); // Bill's PC
  // r.run(BillsPCMenuPlan::deposit()); // Bill's PC
  // r.run(ListMenuPlan::choose(1)); // Deposit Pidgey
  // r.run(DepositWithdrawMenuPlan::deposit()); // Deposit Pidgey
  // r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Pidgey deposited
  // r.run(TextPlan::new()); // Pidgey deposited
  // r.run(BillsPCMenuPlan::deposit()); // Bill's PC
  // r.run(ListMenuPlan::choose(1)); // Deposit Sandshrew
  // r.run(DepositWithdrawMenuPlan::deposit()); // Deposit Sandshrew
  // r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Sandshrew deposited
  // r.run(TextPlan::new()); // Sandshrew deposited
  // r.run(BillsPCMenuPlan::quit()); // Bill's PC
  // r.run(PCMainMenuPlan::quit()); // PC
  // r.run(HoldTextDisplayOpenPlan); // PC
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(WalkToPlan::new(8, 0));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(WalkToPlan::new(4, 2));
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(9));
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_blue_test3");
  // r.load("multi_blue_test3");
  // r.run(FightKOPlan::new(Move::MegaKick, None, EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::DoubleSlap))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 296106 + 274 / 295856
  // r.save("multi_blue_after_lorelei_");
  // r.load("multi_blue_after_lorelei_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan)); // after text
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(1)); // Elixer
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(PartyMenuPlan::choose(0)); // Blastoise
  //   r.run(SkipTextsPlan::new(1)); // PP restored
  //   r.run(ListMenuPlan::quit());
  //   r.run(StartMenuPlan::close());
  // }
  // r.run(WalkToPlan::new(4, 1));
  // r.run(WalkToPlan::new(4, 0));
  // r.run(EdgeWarpPlan::new());
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(WalkToPlan::new(4, 2));
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(10));
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::Surf, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::with_level_up(1)); // #inputs: 300347 + 280 / 300234
  // r.save("multi_blue_after_bruno_");
  // r.load("multi_blue_after_bruno_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // after text
  // r.run(WalkToPlan::new(4, 1));
  // r.run(WalkToPlan::new(4, 0));
  // r.run(EdgeWarpPlan::new());
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(WalkToPlan::new(4, 2));
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(12));
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightTurnPlan::new(AttackDesc::crit_no_side_effect(Move::IceBeam, 53..=63), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Hypnosis)), None));
  // r.run(FightKOPlan::new(Move::Surf, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Hypnosis))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(FightKOPlan::new(Move::Surf, None, EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::DreamEater))));
  // // r.run(FightKOPlan::new(Move::Surf, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Toxic))));
  // r.run(EndTrainerBattlePlan::new(2)); // #inputs: 305280 + 289 / 305186 305164
  // r.save("multi_blue_after_agatha__");
  // r.load("multi_blue_after_agatha_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan)); // after text
  // r.run(WalkToPlan::new(4, 1));
  // r.run(WalkToPlan::new(4, 0));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(5, 1));
  // r.run(SeqPlan::new(SkipTextsPlan::new(13), HoldTextDisplayOpenPlan)); // Lance
  // r.run(OverworldWaitPlan::trainer_battle(247)); // initiate Lance fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.run(FightKOPlan::new(Move::MegaKick, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Leer))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_skip_learning_move());
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Supersonic))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(3)); // #inputs: 311078 + 297 / 310946
  // r.save("multi_blue_after_lance_");
  // r.load("multi_blue_after_lance_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(14), HoldTextDisplayOpenPlan)); // after text
  // r.run(WalkToPlan::new(5, 0));
  // r.run(EdgeWarpPlan::new());
  // r.run(SeqPlan::new(SkipTextsPlan::new(18), HoldTextDisplayOpenPlan)); // Rival
  // r.run(OverworldWaitPlan::trainer_battle(243)); // initiate Rival fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.run(FightKOPlan::new(Move::IceBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(FightKOPlan::new(Move::MegaKick, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Recover))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new()); // 314889
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(FightKOPlan::new(Move::MegaKick, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Leer))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Surf, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Leer))));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::new(6)); // #inputs: 317883 + 361 / 317619 317620
  // r.save("multi_blue_after_champion__");
  // r.load("multi_blue_after_champion_");
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan)); // after text
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // after text
  // r.run(SkipTextsPlan::new(6)); // after text
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // after text
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // after text
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(SeqPlan::new(SkipTextsPlan::new(12), HoldTextDisplayOpenPlan)); // after text
  // r.run(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan)); // after text
  // r.run(SeqPlan::new(SkipTextsPlan::new(15), HoldTextDisplayOpenPlan)); // after text
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(TextPlan::new().with_skip_ends(2));
  // r.run(SkipTextsPlan::new(2)); // #inputs: 321577 + 361
  // r.save("multi_blue_test3");
  // r.load("multi_blue_test3");
  // r.run(TextPlan::new());


  // r.save("multi_blue_test");
  // r.load("multi_blue_test");

  // r.debug_print_state_fn(MoveInfosFn::new(Who::Player));
  // r.debug_print_state_fn(BattleMonInfoFn::new(Who::Player));
  // r.debug_print_state_fn(MoveInfosFn::new(Who::Enemy));
  // r.debug_print_state_fn(BattleMonInfoFn::new(Who::Enemy));

  r.debug_segment_end("temp/multi_blue");
}
