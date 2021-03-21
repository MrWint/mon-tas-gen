#[allow(unused_imports)] use gambatte::inputs::*;
#[allow(unused_imports)] use montas::constants::*;
#[allow(unused_imports)] use montas::metric::*;
#[allow(unused_imports)] use montas::metric::battle::*;
#[allow(unused_imports)] use montas::metric::battle::gen1::*;
#[allow(unused_imports)] use montas::metric::overworld::gen1::*;
use montas::multi::*;
use montas::rom::*;
use montas::sdl::*;

const EQUAL_LENGTH_FRAMES: bool = false;
const RTC_DIVISOR_OFFSET: i32 = 0;

#[allow(dead_code)]
pub fn start() {
  log::set_max_level(log::LevelFilter::Debug);

  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let yellow_gb = Gb::<Yellow>::create(EQUAL_LENGTH_FRAMES, RTC_DIVISOR_OFFSET, SdlScreen::new(sdl.clone(), 0));
  let mut r = SingleGbRunner::new(yellow_gb);

  // r.run(SkipIntroPlan::new()); // Logo
  // r.run(SkipYellowIntroPlan::new()); // Intro cutscene
  // r.run(SkipYellowTitlePlan::new()); // main menu
  // r.run(MainMenuPlan::new()); // main menu
  // r.run(SkipTextsPlan::new(13)); // oak speech
  // r.run(IntroNameMenuPlan::choose_custom_name()); // own name
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(NamingScreenPlan::with_letter(b'A'));
  // // r.save("multi_yellow_test2");
  // // r.load("multi_yellow_test2");
  // r.run(SkipTextsPlan::new(5)); // oak speech
  // r.run(IntroNameMenuPlan::choose_custom_name()); // rival name
  // r.run(NamingScreenPlan::with_letter(b'A').with_fill_letters(6));
  // r.run(SkipTextsPlan::new(7)); // oak speech
  // r.run(TextPlan::new()); // ... awaits let's go
  // r.save("multi_yellow_intro_");
  // r.load("multi_yellow_intro_");
  // r.run(WalkToPlan::new(7, 1)); // go down stairs
  // r.run(WalkToPlan::new(3, 6)); // go outside
  // r.run(WalkToPlan::new(3, 7)); // go outside
  // r.run(EdgeWarpPlan::new()); // go outside
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(WalkToPlan::new(10, 0)); // trigger oak cutscene
  // r.run(TextPlan::new()); // it's dangerous to go outside, take this
  // r.run(HoldTextDisplayOpenPlan::new()); // close text box
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())); // oak speech
  // r.run(OverworldWaitPlan::with_metric(OverworldInteractionMetric.filter(|r| if let OverworldInteractionResult::WildEncounter { species: Pokemon::Pikachu, ..} = r { true } else { false })));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Wild Pikachu appeared!
  // r.run(SkipTextsPlan::new(1).with_skip_ends(5)); // Oak used ball // Pikachu
  // r.run(SkipTextsPlan::new(1)); // caught.
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())); // oak speech
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan::new())); // oak speech
  // r.run(OverworldWaitPlan::new()); // Skip PalletTownScript load
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow Oak
  // r.run(OverworldWaitPlan::auto_walk(D)); // Follow oak
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
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())); // Fed up with waiting
  // r.run(SeqPlan::new(SkipTextsPlan::new(9), HoldTextDisplayOpenPlan::new())); // you can have one, choose
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())); // What about me?
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())); // Can have one too
  // r.run(WalkToPlan::new(7, 4)); // stand before ball
  // r.run(OverworldTurnPlan::new(U)); // turn towards ball
  // r.run(OverworldInteractPlan::with(2)); // Interact with Ball
  // r.run(HoldTextDisplayOpenPlan::new()); // close text box
  // r.run(SkipTextsPlan::new(2));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1));
  // r.run(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan::new())); // Come here
  // r.run(OverworldWaitPlan::new()); // Skip Lab script
  // r.run(OverworldWaitPlan::auto_walk(L)); // Go to Oak
  // r.run(OverworldWaitPlan::auto_walk(D)); // Go to oak
  // r.run(OverworldWaitPlan::auto_walk(L)); // Go to oak
  // r.run(OverworldWaitPlan::auto_walk(L)); // Go to oak
  // r.run(OverworldWaitPlan::auto_walk(L)); // Go to oak
  // r.run(OverworldWaitPlan::auto_walk(U)); // Go to oak
  // r.run(OverworldWaitPlan::auto_walk(U)); // Go to oak
  // r.run(SkipTextsPlan::new(5));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(3));
  // r.run(TextPlan::new().with_skip_ends(2)); // give nickname?
  // r.run(TwoOptionMenuPlan::no_with_metric(Gen1DVMetric {}.filter(|v| {
  //       if v.hp() > 4 { return false; } // Pikachu DVs, 18HP total
  //       log::info!("Chosen DVs: {:?}", v); true
  //     }).into_unit())); // don't give nickname
  // r.run(HoldTextDisplayOpenPlan::new()); // close text box
  // r.save("multi_yellow_after_pikachu");
  // r.load("multi_yellow_after_pikachu");
  // r.run(WalkToPlan::new(5, 6)); // trigger rival fight
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())); // Rival fight
  // r.run(OverworldWaitPlan::trainer_battle(225)); // Rival fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Rival fight
  // r.run(FightTurnPlan::new(AttackDesc::effect_failed(Move::Growl), EnemyAttackDesc::Attack(AttackDesc::crit(Move::Tackle, 9..=9)), None));
  // r.run(FightTurnPlan::new(AttackDesc::effect_failed(Move::Growl), EnemyAttackDesc::Attack(AttackDesc::crit(Move::Tackle, 9..=10)), None));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Pikachu fainted
  // r.run(SkipTextsPlan::new(1)); // Rival: I'm great
  // r.save("multi_yellow_after_rival1"); // #inputs: 9778
  // r.load("multi_yellow_after_rival1");
  // r.run(OverworldWaitPlan::new()); // advance map script (ab buttons allowed)
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())); // after rival1 texts
  // r.run(OverworldWaitPlan::new()); // advance map script (ab buttons allowed)
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())); // Oak text
  // r.run(OverworldWaitPlan::new()); // advance map script (ab buttons allowed)
  // r.run(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan::new())); // Oak text
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(5, 11).with_move_past_pikachu()); // Leave lab
  // r.run(EdgeWarpPlan::new()); // go outside // inputs: 10724
  // r.run(WalkToPlan::new(10, -1));
  // r.run(WalkToPlan::new(11, -1)); // Enter Viridian
  // r.run(WalkToPlan::new(29, 19)); // Enter Mart, starts cutscene
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())); // mart cutscene
  // r.run(OverworldWaitPlan::auto_walk(U)); // mart cutscene
  // r.run(OverworldWaitPlan::auto_walk(U)); // mart cutscene
  // r.run(OverworldWaitPlan::auto_walk(L)); // mart cutscene
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())); // mart cutscene
  // r.run(WalkToPlan::new(3, 6));
  // r.run(WalkToPlan::new(3, 7));
  // r.run(EdgeWarpPlan::new()); // go outside // inputs: 12999
  // r.run(WalkToPlan::new(29, 26));
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(21, 36)); // enter Route 1
  // r.run(WalkToPlan::new(14, 18));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(12, 26));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(11, 36)); // enter pallet town
  // r.run(WalkToPlan::new(12, 11)); // enter oak's lab // inputs: 14517
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(5, 8)); // next to oak
  // r.run(WalkToPlan::new(5, 1)); // next to oak
  // r.run(OverworldTurnPlan::new(D)); // turn towards Oak
  // r.run(OverworldInteractPlan::with(3)); // Talk to Oak
  // r.run(SeqPlan::new(SkipTextsPlan::new(12), HoldTextDisplayOpenPlan::new())); // Oak speech: special pokeball, thank you
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())); // Oak speech: Gramps
  // r.run(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan::new())); // Oak speech: Gramps
  // r.run(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan::new())); // Oak speech: Have something for you
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan::new())); // Oak speech: hi-tech encyclopedia
  // r.run(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan::new())); // Oak speech: Took Pokedex
  // r.run(SeqPlan::new(SkipTextsPlan::new(10), HoldTextDisplayOpenPlan::new())); // Oak speech: greatest undertaking
  // r.run(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan::new())); // Oak speech: leave it to me
  // r.run(WalkToPlan::new(4, 11).with_move_past_pikachu()); // leave
  // r.run(EdgeWarpPlan::new()); // go outside // inputs: 17373
  // r.save("multi_yellow_after_oak_parcel");
  // r.load("multi_yellow_after_oak_parcel");
  // r.run(WalkToPlan::new(10, -1)); // enter Route 1
  // r.run(WalkToPlan::new(11, -1)); // Enter Viridian
  // r.run(WalkToPlan::new(29, 19)); // Enter Mart
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(OverworldOpenStartMenuPlan::new()); // Open start menu
  // r.run(StartMenuPlan::options_yellow()); // main menu
  // r.run(ChangeOptionsPlan::new().for_yellow()); // set options
  // r.run(StartMenuPlan::close()); // main menu
  // r.run(WalkToPlan::new(3, 5));
  // r.run(WalkToPlan::new(2, 5));
  // r.run(OverworldInteractPlan::with(1)); // Mart
  // r.run(TextPlan::new()); // How can I help you
  // r.run(BuySellQuitMenuPlan::buy());
  // r.run(TextPlan::new()); // Take your time
  // r.run(SeqPlan::new(ListMenuPlan::choose(0), ChooseQuantityMenuPlan::new(3))); // Choose Pokeball x3
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Item? // That will be
  // r.run(TextPlan::new().with_skip_ends(2)); // Price // Okay?
  // r.run(TwoOptionMenuPlan::yes()); // buy
  // r.run(SkipTextsPlan::new(1)); // Here you go
  // r.run(ListMenuPlan::quit()); // exit buy menu
  // r.run(TextPlan::new()); // Anything else?
  // r.run(BuySellQuitMenuPlan::quit());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Thank you! // inputs: 19588
  // r.run(WalkToPlan::new(3, 6));
  // r.run(WalkToPlan::new(3, 7));
  // r.run(EdgeWarpPlan::new()); // go outside
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(WalkToPlan::new(19, 9));
  // r.run(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan)); // Catch tutorial
  // r.run(OverworldWaitPlan::new()); // advance map script (ab buttons allowed)
  // r.run(OverworldWaitPlan::with_metric(OverworldInteractionMetric.filter(|r| if let OverworldInteractionResult::WildEncounter { species: Pokemon::Rattata, ..} = r { true } else { false })));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Wild Rattata appeared!
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(TextPlan::new().with_skip_ends(2));
  // r.run(SkipTextsPlan::new(1)); // Shoot! It was close too
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan)); // Catch tutorial // inputs: 21558
  // r.run(WalkToPlan::new(18, -1)); // Enter Route 2
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(4, 52)); // Catch Nidoran
  // r.save("multi_yellow_before_nido_encounter");
  // r.load("multi_yellow_before_nido_encounter");
  // r.run(WalkToPlan::new(7, 48)); // Catch Nidoran
  // r.run(WalkToPlan::new(8, 48)); // Catch Nidoran
  // r.run(OverworldEncounterPlan::new(U, OverworldInteractionMetric.filter(|v| {
  //   if let OverworldInteractionResult::WildEncounter { species: Pokemon::NidoranM, level: 6, dvs } = v {
  //     // log::info!("Nidoran");
  //     if dvs.hp() > 9 || dvs.atk < 15 || dvs.def > 9 || dvs.spd < 13 || dvs.spc < 13 { return false; }
  //     log::info!("Chosen Nidoran DVs: {:?}", dvs);
  //     true
  //   } else { false }
  // }).into_unit()));
  // r.save("multi_yellow_after_nido_encounter_"); // delay: 28 frames; Chosen Nidoran DVs: DVs { atk: 15, def: 4, spd: 14, spc: 13 } #inputs: 22549
  // r.load("multi_yellow_after_nido_encounter_");
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Wild Nidoran appeared!
  // r.run(TextPlan::new().with_skip_ends(2)); // Go, Pikachu!
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(BattleMenuPlan::items());
  // r.run(ListMenuPlan::choose(0)); // Poke Ball
  // r.run(TextPlan::with_metric(CatchSuccessMetric, false).with_skip_ends(2));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Nidoran was
  // r.run(SkipTextsPlan::new(1)); // caught!
  // r.run(SkipTextsPlan::new(1)); // New dex entry
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Nidoran!
  // r.run(TextScrollWaitPlan::new()); // dex entry
  // r.run(TextScrollWaitPlan::new()); // Scroll dex text 2
  // r.run(SkipTextsPlan::new(1)); // Give nickname?
  // r.run(TextPlan::new().with_skip_ends(2)); // to Nidoran?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(NamingScreenPlan::with_letter(b'A'));
  // r.save("multi_yellow_nidoran");
  // r.load("multi_yellow_nidoran");
  // r.run(WalkToPlan::new(3, 43)); // Enter Viridian Forest
  // r.run(WalkToPlan::new(5, 0)); // Enter Viridian Forest
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(25, 40));
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(WalkToPlan::new(25, 21));
  // r.run(OverworldEncounterPlan::new(U, OverworldInteractionMetric.filter(|v| {
  //   if let OverworldInteractionResult::WildEncounter { species: Pokemon::Pidgeotto, level: 9, dvs } = v {
  //     log::info!("Pidgeotto");
  //     if dvs.hp() > 3 || dvs.atk < 7 { return false; } // 30 HP
  //     log::info!("{:?}", dvs);
  //     true
  //   } else { false }
  // }).into_unit()));
  // r.save("multi_yellow_after_pidgeotto_encounter");
  // r.load("multi_yellow_after_pidgeotto_encounter");
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Wild Pidgeotto appeared!
  // r.run(TextPlan::new().with_skip_ends(2)); // Go, Pikachu!
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(FightTurnPlan::new(AttackDesc::crit_no_side_effect(Move::ThunderShock, 13..=13), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::SandAttack)), None));
  // r.run(FightTurnPlan::new(AttackDesc::crit_no_side_effect(Move::ThunderShock, 13..=13), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::SandAttack)), None));
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(BattleMenuPlan::fight());
  // r.run(SelectMoveMenuPlan::with_metric(Move::ThunderShock, AIChooseMoveMetric.expect(Move::Gust)));
  // r.run(TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(15, 25).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 18 } else { false }), false).with_skip_ends(4));
  // r.run(SkipTextsPlan::new(1)); // Critical Hit
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Pikachu fainted
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(TextPlan::new()); // use next Pokemon?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(PartyMenuPlan::choose(1)); // Choose Nidoran
  // r.run(TextPlan::new().with_skip_ends(2)); // Go, Nidoran!
  // r.run(FightTurnPlan::new(AttackDesc::hit(Move::Tackle, 4..=4), EnemyAttackDesc::Attack(AttackDesc::crit(Move::Gust, 21..=21)), None));
  // r.save("multi_yellow_test4");
  // r.load("multi_yellow_test4");
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // mon // grew to level // X
  // r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // mon // learned // move
  // r.save("multi_yellow_after_pidgeotto"); // #inputs: 27791
  // r.load("multi_yellow_after_pidgeotto");
  // r.run(WalkToPlan::new(13, 16));
  // r.run(OverworldTurnPlan::new(D));
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(OverworldInteractPlan::with(6));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(2));
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(FightTurnPlan::new(AttackDesc::crit(Move::Tackle, 10..=10), EnemyAttackDesc::Attack(AttackDesc::hit(Move::Tackle, 4..=4)), None));
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::NoAttack));
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::StringShot))));
  // r.save("multi_yellow_test4");
  // r.load("multi_yellow_test4");
  // r.run(EndTrainerBattlePlan::with_level_up(1));
  // r.save("multi_yellow_viridian_after_bugcatcher1"); // #inputs: 31277
  // r.load("multi_yellow_viridian_after_bugcatcher1");
  // r.run(WalkToPlan::new(11, 9));
  // r.run(OverworldEncounterPlan::new(U, OverworldInteractionMetric.filter(|v| {
  //   if let OverworldInteractionResult::WildEncounter { species: Pokemon::Pidgey, level: _, dvs: _ } = v {
  //     log::info!("Pidgey");
  //     true
  //   } else { false }
  // }).into_unit()));
  // r.save("multi_yellow_after_pidgey_encounter");
  // r.load("multi_yellow_after_pidgey_encounter");
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Wild Pidgey appeared!
  // r.run(TextPlan::new().with_skip_ends(2)); // Go, Nidoran!
  // r.run(BattleMenuPlan::items());
  // r.run(SeqPlan::new(ListMenuPlan::choose(0), TextPlan::with_metric(CatchSuccessMetric, false).with_skip_ends(2))); // Poke Ball
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Pidgey was
  // r.run(SkipTextsPlan::new(1)); // caught!
  // r.run(SkipTextsPlan::new(1)); // New dex entry
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Pidgey!
  // r.run(TextScrollWaitPlan::new()); // dex entry
  // r.run(TextScrollWaitPlan::new()); // Scroll dex text 2
  // r.run(SkipTextsPlan::new(1)); // Give nickname?
  // r.run(TextPlan::new().with_skip_ends(2)); // to Pidgey?
  // r.run(TwoOptionMenuPlan::no());
  // r.save("multi_yellow_pidgey"); // #inputs: 32598
  // r.load("multi_yellow_pidgey");
  // r.run(WalkToPlan::new(2, 19));
  // r.run(OverworldInteractPlan::with(4)); // Bugcatcher
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1));
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(BattleMenuPlan::fight());
  // r.run(SelectMoveMenuPlan::new(Move::Leer).use_select());
  // r.run(SelectMoveMenuPlan::new(Move::HornAttack).use_select());
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::StringShot))).skip_battle_menu());
  // r.run(EndTrainerBattlePlan::with_level_up(2)); // Bugcatcher fight // #inputs: 
  // r.save("multi_yellow_viridian_after_bugcatcher2"); // #inputs: 34982
  // r.load("multi_yellow_viridian_after_bugcatcher2");
  // r.run(WalkToPlan::new(1, 0)); // Leave Forest
  // r.run(EdgeWarpPlan::new()); // edge warp
  // r.run(WalkToPlan::new(5, 0)); // Leave Viridian Forest
  // r.run(WalkToPlan::new(8, -1)); // enter Pewter City
  // r.run(WalkToPlan::new(16, 17)); // enter Gym
  // r.run(WalkToPlan::new(3, 8)); // align with Trainer
  // r.run(WalkToPlan::new(3, 7)); // align with Trainer
  // r.run(OverworldInteractPlan::with(2)); // JrTrainerM
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(3));
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::Attack(AttackDesc::hit(Move::Scratch, 5..=5))));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::Scratch))));
  // r.run(EndTrainerBattlePlan::with_learn_move(3)); // JrTrainerM fight //  #inputs: 39561
  // r.save("multi_red_pewter_after_jrtrainerm");
  // r.load("multi_red_pewter_after_jrtrainerm");
  // r.run(WalkToPlan::new(4, 2)); // stand in front of Brock
  // r.run(OverworldInteractPlan::with(1)); // Brock
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(9));
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::DoubleKick, AIChooseMoveMetric.expect(Move::Tackle)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(10, 14).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 10 } else { false }), false).with_skip_ends(4))); // A used Double Kick
  // r.run(SkipTextsPlan::new(2));
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(SeqPlan::new(
  //   SkipTextsPlan::new(1).with_skip_ends(1), // hit two times
  //   TextPlan::with_metric(Gen1MoveSuccessMetric.filter(|v| v == &FightTurnResult::Failed), false).with_skip_ends(4))); // Geodude used Tackle
  // r.run(SkipTextsPlan::new(1)); // attack failed
  // r.run(FightKOPlan::new(Move::DoubleKick, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SeqPlan::new(
  //       SelectMoveMenuPlan::with_metric(Move::DoubleKick, AIChooseMoveMetric.expect(Move::Bide)),
  //       TextPlan::new().with_skip_ends(4)), // Onix used Bide
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(6, 10).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 6 } else { false }), false).with_skip_ends(4))); // A used Double Kick
  // r.run(SkipTextsPlan::new(2)); // 2x effective
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // hit two times
  // r.save("multi_yellow_test4");
  // r.load("multi_yellow_test4");
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::new(Move::DoubleKick),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(6, 10).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 10 } else { false }), false).with_skip_ends(4))); // A used Double Kick
  // r.run(SkipTextsPlan::new(3)); // crit + 2x effective
  // r.run(EndTrainerBattlePlan::with_level_up(10)); // Brock fight //  #inputs: 43401
  // r.save("multi_yellow_after_brock");
  // r.load("multi_yellow_after_brock");
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Brock speech
  // r.run(SeqPlan::new(SkipTextsPlan::new(13), HoldTextDisplayOpenPlan)); // Brock speech
  // r.run(WalkToPlan::new(4, 13));
  // r.run(EdgeWarpPlan::new()); // leave gym
  // r.run(WalkToPlan::new(40, 17)); // Enter Route 3
  // r.run(WalkToPlan::new(11, 6)); // Bugcatcher
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Bugcatcher
  // r.run(OverworldWaitPlan::trainer_battle(202)); // Bugcatcher fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Bugcatcher fight
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // Bugcatcher fight //  #inputs: 48095
  // r.save("multi_yellow_route3_after_bugcatcher1");
  // r.load("multi_yellow_route3_after_bugcatcher1");
  // r.run(WalkToPlan::new(12, 4)); // Youngster
  // r.run(WalkToPlan::new(13, 4)); // Youngster
  // r.run(OverworldInteractPlan::with(3)); // Youngster
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(2));
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(FightTurnPlan::new(AttackDesc::crit(Move::DoubleKick, 29..=30), EnemyAttackDesc::Attack(AttackDesc::hit(Move::QuickAttack, 9..=9)), None));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(FightTurnPlan::new(AttackDesc::crit(Move::HornAttack, 26..=28), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Leer)), None));
  // r.run(FightKOPlan::new(Move::DoubleKick, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up(1)); // Youngster fight //  #inputs: 50643
  // r.save("multi_yellow_route3_after_youngster");
  // r.load("multi_yellow_route3_after_youngster");
  // r.run(WalkToPlan::new(18, 5)); // Bugcatcher
  // r.run(OverworldInteractPlan::with(5)); // Bugcatcher
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1));
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(2)); // Bugcatcher fight //  #inputs: 53856
  // r.save("multi_yellow_route3_after_bugcatcher2");
  // r.load("multi_yellow_route3_after_bugcatcher2");
  // r.run(WalkToPlan::new(24, 5)); // Bugcatcher
  // r.run(OverworldTurnPlan::new(D));
  // r.run(OverworldInteractPlan::with(8));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1));
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(FightTurnPlan::new(AttackDesc::crit(Move::HornAttack, 30..=33), EnemyAttackDesc::Attack(AttackDesc::stat_up_down(Move::Harden)), None));
  // r.run(FightKOPlan::new(Move::DoubleKick, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // Bugcatcher fight //  #inputs: 56370
  // r.save("multi_yellow_route3_after_bugcatcher3");
  // r.load("multi_yellow_route3_after_bugcatcher3");
  // r.run(TextPlan::new().with_skip_ends(2));
  // r.run(EvolutionPlan::dont_cancel());
  // r.run(TextPlan::new().with_skip_ends(4));
  // r.run(WalkToPlan::new(59, -1)); // Enter Route 4
  // r.run(WalkToPlan::new(18, 5)); // Enter Mt. Moon
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(4, 2)); // Moon Stone
  // r.run(WalkToPlan::new(3, 2)); // Moon Stone
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(OverworldInteractPlan::with(9));
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Moon Stone
  // r.run(WalkToPlan::new(5, 5)); // UF2
  // r.run(WalkToPlan::new(21, 17)); // UF3
  // r.run(WalkToPlan::new(13, 8)); // Super Nerd // 63115
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::trainer_battle(208)); // Super Nerd fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Super Nerd fight
  // r.save("multi_yellow_test4");
  // r.load("multi_yellow_test4");
  // r.run(FightTurnPlan::new(AttackDesc::crit(Move::HornAttack, 35..=35), EnemyAttackDesc::Attack(AttackDesc::hit(Move::Pound, 5..=5)), None));
  // r.run(FightKOPlan::new(Move::DoubleKick, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_yellow_test5");
  // r.load("multi_yellow_test5");
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightTurnPlan::new(AttackDesc::crit(Move::HornAttack, 24..=25), EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::Smog)), None));
  // r.run(FightKOPlan::new(Move::Tackle, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // Super Nerd fight //  #inputs: 66208
  // r.save("multi_yellow_mtmoon_after_supernerd");
  // r.load("multi_yellow_mtmoon_after_supernerd");
  // r.run(WalkToPlan::new(13, 7)); // Fossil
  // r.run(OverworldInteractPlan::with(8));
  // r.run(TextPlan::new()); // choose Helix Fossil?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan)); // chosen Helix Fossil
  // r.run(SeqPlan::new(TextPlan::new(), HoldTextDisplayOpenPlan)); // I choose this one then
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(3, 5)); // Rocket
  // r.run(SeqPlan::new(TextPlan::new(), HoldTextDisplayOpenPlan)); // Rocket text
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Rocket text
  // r.run(OverworldWaitPlan::trainer_battle(230)); // Rocket fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Rocket fight
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::DoubleKick, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(SeqPlan::new(
  //   FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::Smog))),
  //   EndTrainerBattlePlan::new(1))); // Rocket fight //  #inputs: 70109
  // r.save("multi_yellow_mtmoon_after_rocket");
  // r.load("multi_yellow_mtmoon_after_rocket");
  // r.run(SkipTextsPlan::new(1)); // Rocket text
  // r.run(SeqPlan::new(TextPlan::new(), HoldTextDisplayOpenPlan)); // Rocket text
  // r.run(WalkToPlan::new(4, 7)); // Leave Mt. Moon
  // r.run(WalkToPlan::new(5, 7)); // Leave Mt. Moon
  // r.run(WalkToPlan::new(27, 3)); // Leave Mt. Moon
  // r.run(WalkToPlan::new(78, 8));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(90, 10)); // Enter Cerulean City
  // r.run(WalkToPlan::new(19, 17)); // Enter Center
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(13, 5)); // PC
  // r.run(WalkToPlan::new(13, 4)); // PC
  // r.run(OverworldInteractPlan::with_hidden_item()); // PC
  // r.run(SkipTextsPlan::with_metric(1, FnMetric::new(|gb| if gb.gb().read_memory(0xc224) < 9 { None } else { Some(()) }))); // PC access // eliminate states where NPC moves into the return path
  // r.run(PCMainMenuPlan::mon()); // Bill's PC
  // r.run(SkipTextsPlan::new(2)); // Bill's PC
  // r.run(BillsPCMenuPlan::deposit()); // Bill's PC
  // r.run(ListMenuPlan::choose(0)); // Deposit Pikachu
  // r.run(DepositWithdrawMenuPlan::deposit()); // Deposit Pikachu
  // r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Pikachu deposited
  // r.run(BillsPCMenuPlan::quit()); // Bill's PC
  // r.run(PCMainMenuPlan::quit()); // PC
  // r.run(HoldTextDisplayOpenPlan); // PC
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(WalkToPlan::new(3, 3)); // Center
  // r.run(OverworldInteractPlan::with(1)); // Center
  // r.run(SkipTextsPlan::new(3)); // Center
  // r.run(SeqPlan::new(TextCommandPausePlan::new(), TextPlan::new())); // Center
  // r.run(TwoOptionMenuPlan::yes()); // Center
  // r.run(TextPlan::new()); // Center
  // r.run(SkipTextsPlan::new(2)); // Center
  // r.run(SeqPlan::new(TextCommandPausePlan::new(), SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan))); // Center
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(WalkToPlan::new(3, 7)); // go outside
  // r.run(EdgeWarpPlan::new()); // go outside
  // r.run(WalkToPlan::new(30, 19)); // enter Gym
  // r.run(OverworldOpenStartMenuPlan::new());
  // r.run(StartMenuPlan::items());
  // r.run(ListMenuPlan::choose(2)); // Moon Stone
  // r.run(ItemUseTossMenuPlan::use_()); // Moon Stone
  // r.run(PartyMenuPlan::choose(0)); // Nidorino
  // r.run(TextPlan::new().with_skip_ends(2));
  // r.run(EvolutionPlan::forced());
  // r.run(TextPlan::new().with_skip_ends(4));
  // r.run(ListMenuPlan::quit());
  // r.run(StartMenuPlan::close());
  // r.run(WalkToPlan::new(5, 3)); // JrTrainerF fight
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::trainer_battle(206)); // initiate Jr. Trainer F fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.run(SeqPlan::new(
  //   FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::TailWhip))),
  //   EndTrainerBattlePlan::with_level_up(1))); // JrTrainerF fight //  #inputs: 78024
  // r.save("multi_yellow_cerulean_after_jrtrainerf");
  // r.load("multi_yellow_cerulean_after_jrtrainerf");
  // r.run(WalkToPlan::new(5, 2));
  // r.run(OverworldTurnPlan::new(L));
  // r.run(OverworldInteractPlan::with(1)); // Misty
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(9)); // Misty
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(FightTurnPlan::new(AttackDesc::crit(Move::HornAttack, 34..=40), EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::WaterGun)), None));
  // r.run(FightKOPlan::new(Move::DoubleKick, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(FightTurnPlan::new(AttackDesc::crit(Move::HornAttack, 21..=25), EnemyAttackDesc::Attack(AttackDesc::crit(Move::WaterGun, 59..=60)), None));
  // r.run(SeqPlan::new(
  //   FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::WaterGun))),
  //   EndTrainerBattlePlan::with_level_up(4))); //  #inputs: 81814
  // r.save("multi_yellow_cerulean_after_misty");
  // r.load("multi_yellow_cerulean_after_misty");
  // r.run(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan));
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(3));
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up TM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach BubbleBeam?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(0)); // Nidoking
  //   r.run(OverrideMovePlan::choose(0)); // Override Horn Attack
  //   r.run(ListMenuPlan::quit());
  //   r.run(StartMenuPlan::close());
  // }
  // r.run(WalkToPlan::new(5, 13)); // Leave Gym
  // r.run(EdgeWarpPlan::new()); // edge warp
  // r.run(WalkToPlan::new(21, 7)); // Trigger Rival fight
  // r.run(WalkToPlan::new(21, 6)); // Trigger Rival fight // #inputs: 84261
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan)); // Rival
  // r.run(OverworldWaitPlan::trainer_battle(225)); // initiate Rival fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::DoubleKick, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(2)); // Rival fight //  #inputs: 87454
  // r.save("multi_yellow_cerulean_after_rival");
  // r.load("multi_yellow_cerulean_after_rival");
  // r.run(SeqPlan::new(SkipTextsPlan::new(14), HoldTextDisplayOpenPlan)); // Rival after battle texts
  // r.run(WalkToPlan::new(21, -1)); // Enter Route 24
  // r.run(WalkToPlan::new(11, 32)); // Nugget 1
  // r.run(OverworldInteractPlan::with(7));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(4)); // Nugget1
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_after_nuggetl"); //  #inputs: 90446
  // r.load("multi_yellow_after_nuggetl");
  // r.run(WalkToPlan::new(10, 29)); // Nugget 2
  // r.run(OverworldInteractPlan::with(6));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Nugget2
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_after_nugget2"); //  #inputs: 92415
  // r.load("multi_yellow_after_nugget2");
  // r.run(WalkToPlan::new(11, 26)); // Nugget 3
  // r.run(OverworldInteractPlan::with(5));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Nugget3
  // r.run(FightTurnPlan::new(AttackDesc::hit(Move::BubbleBeam, 34..=999), EnemyAttackDesc::Attack(AttackDesc::crit(Move::QuickAttack, 9..=10)), None));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_after_nugget3"); // #inputs: 95073
  // r.load("multi_yellow_after_nugget3");
  // r.run(WalkToPlan::new(10, 23)); // Nugget 4
  // r.run(OverworldInteractPlan::with(4));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Nugget4
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_after_nugget4"); //  #inputs: 97005
  // r.load("multi_yellow_after_nugget4");
  // r.run(WalkToPlan::new(11, 20)); // Nugget 5
  // r.run(OverworldInteractPlan::with(3));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Nugget5
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_move_override(1, 1)); // override Tackle
  // r.save("multi_yellow_after_nugget5"); // #inputs: 98949
  // r.load("multi_yellow_after_nugget5");
  // r.run(WalkToPlan::new(10, 15)); // Nugget 6
  // r.run(SkipTextsPlan::new(3));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // got nugget
  // r.run(SeqPlan::new(SkipTextsPlan::new(11), HoldTextDisplayOpenPlan)); // Rocket
  // r.run(OverworldWaitPlan::trainer_battle(230)); // initiate Rocket fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(BattleMenuPlan::fight());
  // r.run(SelectMoveMenuPlan::new(Move::BubbleBeam).use_select());
  // r.run(SelectMoveMenuPlan::new(Move::Thrash).use_select());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(64, 102).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 37 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(75, 120).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 39 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_after_nugget6"); // #inputs: 101753
  // r.load("multi_yellow_after_nugget6");
  // r.run(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan)); // Rocket after battle texts
  // r.run(WalkToPlan::new(6, 6)); // Charmander
  // r.run(OverworldInteractPlan::with(9));
  // r.run(SkipTextsPlan::new(5));
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(TextPlan::new().with_skip_ends(2)); // Got Charmander
  // r.run(SkipTextsPlan::new(1)); // Give nickname?
  // r.run(TextPlan::new().with_skip_ends(2)); // to Charmander?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(NamingScreenPlan::with_letter(b'C'));
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Take care of Charmander
  // r.run(WalkToPlan::new(20, 8)); // Route 25
  // r.run(WalkToPlan::new(14, 7)); // Hiker
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Hiker
  // r.run(OverworldWaitPlan::trainer_battle(209)); // initiate Hiker fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Hiker
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_route25_after_hiker"); // #inputs: 104918
  // r.load("multi_yellow_route25_after_hiker");
  // r.run(WalkToPlan::new(18, 7)); // Lass1
  // r.run(OverworldTurnPlan::new(D));
  // r.run(OverworldInteractPlan::with(4));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Lass1
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(67, 108).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 41 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(56, 89).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 43 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_route25_after_lass1"); //  #inputs: 106991
  // r.load("multi_yellow_route25_after_lass1");
  // r.run(WalkToPlan::new(24, 6)); // Jr Trainer M
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Jr Trainer M
  // r.run(OverworldWaitPlan::trainer_battle(205)); // initiate Jr Trainer M fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Jr Trainer M
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(75, 120).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 34 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_level_up_and_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(70, 119).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 36 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_route25_after_jrtrainerm"); // #inputs: 109247
  // r.load("multi_yellow_route25_after_jrtrainerm");
  // r.run(WalkToPlan::new(35, 4)); // Lass2
  // r.run(WalkToPlan::new(36, 4)); // Lass2
  // r.run(OverworldInteractPlan::with(6));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(2)); // Lass2
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(64, 108).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 36 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(78, 133).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 35 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(64, 108).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 36 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_route25_after_lass2"); // #inputs: 111833
  // r.load("multi_yellow_route25_after_lass2");
  // r.run(WalkToPlan::new(38, 4)); // Ether
  // r.run(OverworldInteractPlan::with_hidden_item());
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Ether
  // r.run(WalkToPlan::new(45, 3)); // Enter Bill's House
  // r.run(WalkToPlan::new(4, 5)); // Bill
  // r.run(WalkToPlan::new(5, 5)); // Bill
  // r.run(OverworldInteractPlan::with(1));
  // r.run(SkipTextsPlan::new(10));
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan));
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(1, 5)); // Desk
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item()); // interact with desk
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::new()); // map script
  // r.run(OverworldWaitPlan::auto_walk(R)); // map script
  // r.run(OverworldWaitPlan::auto_walk(R)); // map script
  // r.run(OverworldWaitPlan::auto_walk(R)); // map script
  // r.run(SkipTextsPlan::new(8));
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // SS Anne Ticket
  // r.run(SeqPlan::new(SkipTextsPlan::new(9), HoldTextDisplayOpenPlan));
  // r.run(WalkToPlan::new(3, 6)); // leave
  // r.run(WalkToPlan::new(3, 7)); // leave
  // r.run(EdgeWarpPlan::new());
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(WalkToPlan::new(-1, 11)); // Route 24
  // r.run(WalkToPlan::new(14, 36)); // Cearulean
  // r.run(WalkToPlan::new(24, 2)); // Cearulean
  // r.run(OverworldJumpLedgePlan::new(D));
  // r.run(WalkToPlan::new(27, 11)); // Enter house
  // r.run(WalkToPlan::new(3, 0)); // Leave house
  // r.run(WalkToPlan::new(30, 9)); // Trigger Rocket fight
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::trainer_battle(230)); // initiate Rocket fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Rocket
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(56, 94).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 53 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(58, 98).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 50 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(EndTrainerBattlePlan::with_level_up(2));
  // r.save("multi_yellow_cerulean_after_rocket"); // #inputs: 119059
  // r.load("multi_yellow_cerulean_after_rocket");
  // r.run(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan)); // Rocket after battle texts
  // r.run(WalkToPlan::new(28, 36)); // Enter Route 5
  // r.run(WalkToPlan::new(17, 27)); // Underground
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(4, 4)); // Underground
  // r.run(WalkToPlan::new(4, 4)); // Underground
  // r.run(OverworldInteractPlan::with_hidden_item()); // pick up item
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Full Restore
  // r.run(WalkToPlan::new(2, 40)); // Underground
  // r.run(WalkToPlan::new(2, 41)); // Underground
  // r.run(WalkToPlan::new(4, 7)); // Leave Underground
  // r.run(EdgeWarpPlan::new());
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(WalkToPlan::new(11, 28)); // F
  // r.run(WalkToPlan::new(11, 29)); // F
  // r.run(OverworldInteractPlan::with(5));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // F
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(75, 122).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 41 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(75, 122).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 41 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(75, 122).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 41 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_route6_after_jrtrainerf"); // #inputs: 124426
  // r.load("multi_yellow_route6_after_jrtrainerf");
  // r.run(WalkToPlan::new(10, 30)); // M
  // r.run(WalkToPlan::new(10, 31)); // M
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // JrTrainerM
  // r.run(OverworldWaitPlan::trainer_battle(205)); // JrTrainerM fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // JrTrainerM fight
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(88, 144).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 41 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::EnemyFirst)), // Quick Attack
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(9, 13).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage == 12 } else { false }), false).with_skip_ends(4))); // Enemy Raticate used Quick Attack
  // r.run(SeqPlan::new(
  //   SkipTextsPlan::new(1), // critical hit
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(58, 94).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 46 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_route6_after_jrtrainerm"); // #inputs: 126717
  // r.load("multi_yellow_route6_after_jrtrainerm");
  // r.run(WalkToPlan::new(9, 36)); // Vermilion City
  // r.run(WalkToPlan::new(18, 30)); // Dock
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan)); // Dock
  // r.run(WalkToPlan::new(18, 31)); // Dock
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(14, 2)); // Dock
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(7, 7)); // Evade NPC
  // r.run(WalkToPlan::new(2, 6)); // 1F
  // r.run(WalkToPlan::new(3, 11)); // Avoid stairs
  // r.run(WalkToPlan::new(37, 9)); // Rival encounter
  // r.run(WalkToPlan::new(37, 8)); // Rival encounter
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan)); // Rival
  // r.run(OverworldWaitPlan::trainer_battle(242)); // initiate Rival fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(MoveEffectMetric.expect(MoveEffectResult::Thrash { turns: 3 }).and_then(Gen1NormalHitMetric::with_expected_max_damage(79, 129)).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 47 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(83, 136).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 38 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(40, 65).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 48 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SkipTextsPlan::new(1)); // critical hit
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_level_up_and_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(57, 92).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 55 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(EndTrainerBattlePlan::new(3)); // Rival fight //  #inputs: 132778
  // r.save("multi_yellow_ssanne_after_rival");
  // r.load("multi_yellow_ssanne_after_rival");
  // r.run(SeqPlan::new(SkipTextsPlan::new(5), HoldTextDisplayOpenPlan)); // Rival texts
  // r.run(WalkToPlan::new(36, 4)); // 2F
  // r.run(WalkToPlan::new(4, 4)); // Captain
  // r.run(WalkToPlan::new(4, 3)); // Captain
  // r.run(OverworldInteractPlan::with(1));
  // r.run(SkipTextsPlan::new(4));
  // r.run(TextPlan::new()); // Rub Rub
  // r.run(SkipTextsPlan::new(8));
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan)); // got HM01
  // r.run(WalkToPlan::new(0, 7)); // 1F
  // r.run(WalkToPlan::new(2, 4)); // 0F
  // r.run(WalkToPlan::new(7, 7)); // Evade NPC
  // r.run(WalkToPlan::new(26, 0)); // Evade NPC
  // r.run(EdgeWarpPlan::with_metric(VermilionFirstTrashCanMetric.expect(4)));
  // r.run(WalkToPlan::new(15, 16)); // Cut bush
  // r.run(WalkToPlan::new(15, 17)); // Cut bush
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(OverworldOpenStartMenuPlan::new());
  // r.run(StartMenuPlan::items());
  // r.run(ListMenuPlan::choose(8));
  // r.run(ItemUseTossMenuPlan::use_());
  // r.run(SkipTextsPlan::new(1)); // Booted up TM
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  // r.run(TextPlan::new().with_skip_ends(2)); // Teach Cut?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(PartyMenuPlan::choose(2)); // Charmander
  // r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // learned move
  // r.run(ListMenuPlan::choose(6));
  // r.run(ItemUseTossMenuPlan::use_());
  // r.run(SkipTextsPlan::new(1)); // Booted up TM
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  // r.run(TextPlan::new().with_skip_ends(2)); // Teach Dig?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(PartyMenuPlan::choose(2)); // Charmander
  // r.run(OverrideMovePlan::choose(0));
  // r.run(ListMenuPlan::quit());
  // r.run(StartMenuPlan::mon());
  // r.run(PartyMenuPlan::choose(2)); // Charmander
  // r.run(PartyMonMenuPlan::choose(1)); // Cut
  // r.run(TextScrollWaitPlan::new()); // used cut
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(WalkToPlan::new(12, 19)); // Enter Gym
  // r.run(WalkToPlan::new(4, 9)); // Can 1
  // r.run(OverworldTurnPlan::new(L));
  // r.run(OverworldInteractPlan::with_hidden_item_metric(VermilionSecondTrashCanMetric.expect(7))); // First trash can
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan)); // First switch
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with_hidden_item()); // Second trash can
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Second switch
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(WalkToPlan::new(5, 3)); // Surge
  // r.run(WalkToPlan::new(5, 2)); // Surge
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(10)); // Surge
  // r.run(FightTurnPlan::new(AttackDesc::hit_with_side_effect(Move::BubbleBeam, 11..=16, MoveEffectResult::Success), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Growl)), None));
  // r.run(FightKOPlan::new(Move::Thrash, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(3)); // Surge fight //  #inputs: 142372
  // r.save("multi_yellow_after_surge");
  // r.load("multi_yellow_after_surge");
  // r.run(SeqPlan::new(SkipTextsPlan::new(5), HoldTextDisplayOpenPlan)); // Surge after fight texts
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Got TM
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Surge after fight texts
  // r.run(WalkToPlan::new(5, 17)); // leave
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(15, 19)); // Cut bush
  // r.run(OverworldOpenStartMenuPlan::new());
  // r.run(StartMenuPlan::mon());
  // r.run(PartyMenuPlan::choose(2)); // Charmander
  // r.run(PartyMonMenuPlan::choose(1)); // Cut
  // r.run(TextScrollWaitPlan::new()); // used cut
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
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
  // r.run(WalkToPlan::new(2, 6)); // leave Fanclub
  // r.run(WalkToPlan::new(2, 7)); // leave Fanclub
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(40, 14)); // Route 8
  // r.run(WalkToPlan::new(4, 5)); // Enter Diglett's cave
  // r.run(OverworldOpenStartMenuPlan::new());
  // r.run(StartMenuPlan::mon());
  // r.run(PartyMenuPlan::choose(2)); // Charmander
  // r.run(PartyMonMenuPlan::choose(0)); // Dig
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(OverworldWaitPlan::fly_warp());
  // r.run(WalkToPlan::new(13, 25)); // Enter Bike Shop
  // r.run(WalkToPlan::new(6, 5)); // Enter Bike Shop
  // r.run(WalkToPlan::new(6, 4)); // Enter Bike Shop
  // r.run(OverworldInteractPlan::with(1));
  // r.run(SeqPlan::new(SkipTextsPlan::new(5), HoldTextDisplayOpenPlan)); // Get Bike
  // r.run(WalkToPlan::new(3, 6)); // Leave Bike Shop
  // r.run(WalkToPlan::new(3, 7)); // Leave Bike Shop
  // r.run(EdgeWarpPlan::new());
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(OverworldOpenStartMenuPlan::new());
  // r.run(StartMenuPlan::items());
  // r.run(ListMenuPlan::swap(0)); // Pokeballs
  // r.run(ListMenuPlan::swap(9)); // Bike
  // r.run(ListMenuPlan::choose(8)); // TM24
  // r.run(ItemUseTossMenuPlan::use_());
  // r.run(SkipTextsPlan::new(1)); // Booted up TM
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  // r.run(TextPlan::new().with_skip_ends(2)); // Teach Thunderbolt?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(PartyMenuPlan::choose(0)); // Nidoking
  // r.run(OverrideMovePlan::choose(3)); // Forget Double Kick
  // r.run(ListMenuPlan::choose(0)); // Bike
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // r.run(WalkToPlan::new(19, 26)); // Bush
  // r.run(WalkToPlan::new(19, 27)); // Bush
  // r.run(OverworldOpenStartMenuPlan::new());
  // r.run(StartMenuPlan::mon());
  // r.run(PartyMenuPlan::choose(2)); // Charmander
  // r.run(PartyMonMenuPlan::choose(1)); // Cut
  // r.run(TextScrollWaitPlan::new()); // used cut
  // r.run(WalkToPlan::new(40, 17)); // Route 9
  // r.run(WalkToPlan::new(4, 8)); // Bush
  // r.run(OverworldOpenStartMenuPlan::new());
  // r.run(StartMenuPlan::mon());
  // r.run(PartyMenuPlan::choose(2)); // Charmander
  // r.run(PartyMonMenuPlan::choose(1)); // Cut
  // r.run(TextScrollWaitPlan::new()); // used cut
  // r.run(WalkToPlan::new(13, 8)); // JrTrainerF
  // r.run(WalkToPlan::new(13, 9)); // JrTrainerF
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(2)); // JrTrainerF
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(MoveEffectMetric.expect(MoveEffectResult::Thrash { turns: 3 }).and_then(Gen1NormalHitMetric::with_expected_max_damage(59, 95)).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 47 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(79, 128).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 48 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_level_up_and_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(61, 103).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 47 } else { false }), false).with_skip_ends(0))); // A thrashing about
  //   r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(81, 138).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 48 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_route9_after_jrtrainerf"); // #inputs: 152290
  // r.load("multi_yellow_route9_after_jrtrainerf");
  // r.run(WalkToPlan::new(12, 10));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(40, 10));
  // r.run(WalkToPlan::new(40, 9)); // Bugcatcher
  // r.run(OverworldInteractPlan::with(9));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Bugcatcher
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(74, 126).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 51 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(81, 138).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 49 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(59, 99).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 57 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_route9_after_bugcatcher"); // #inputs: 155070
  // r.load("multi_yellow_route9_after_bugcatcher");
  // r.run(WalkToPlan::new(51, 4));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(60, 8)); // Route 10
  // r.run(WalkToPlan::new(8, 17)); // Rock Tunnel
  // r.run(WalkToPlan::new(23, 6));
  // r.run(WalkToPlan::new(23, 7)); // Pokemaniac
  // r.run(OverworldInteractPlan::with(4));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Pokemaniac
  // r.run(BattleMenuPlan::fight());
  // r.run(SelectMoveMenuPlan::new(Move::Thrash).use_select());
  // r.run(SelectMoveMenuPlan::new(Move::BubbleBeam).use_select());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack).skip_battle_menu());
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_rock_tunnel_after_pokemaniac1"); // #inputs: 158027
  // r.load("multi_yellow_rock_tunnel_after_pokemaniac1");
  // r.run(WalkToPlan::new(37, 3)); // B1F
  // r.run(WalkToPlan::new(27, 30)); // Pokemaniac
  // r.run(OverworldInteractPlan::with(8));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Pokemaniac
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_rock_tunnel_after_pokemaniac2"); // #inputs: 159747
  // r.load("multi_yellow_rock_tunnel_after_pokemaniac2");
  // r.run(WalkToPlan::new(14, 30)); // Lass
  // r.run(WalkToPlan::new(14, 29)); // Lass
  // r.run(OverworldInteractPlan::with(6));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(2)); // Lass
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(58, 95).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 55 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(62, 101).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 55 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_rock_tunnel_after_lass"); // #inputs: 161765
  // r.load("multi_yellow_rock_tunnel_after_lass");
  // r.run(WalkToPlan::new(27, 3)); // 1F
  // r.run(WalkToPlan::new(17, 11)); // B1F
  // r.run(WalkToPlan::new(8, 10)); // Hiker
  // r.run(WalkToPlan::new(7, 10)); // Hiker
  // r.run(OverworldInteractPlan::with(2));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Hiker
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::BubbleBeam, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_rock_tunnel_after_hiker"); // #inputs: 165321
  // r.load("multi_yellow_rock_tunnel_after_hiker");
  // r.run(WalkToPlan::new(3, 3)); // 1F
  // r.run(WalkToPlan::new(24, 24)); // JrTrainerF
  // r.run(WalkToPlan::new(23, 24)); // JrTrainerF
  // r.run(OverworldInteractPlan::with(6));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // JrTrainerF
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(86, 147).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 49 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(63, 108).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 51 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(79, 135).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 49 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(EndTrainerBattlePlan::new(1));
  // r.save("multi_yellow_rock_tunnel_after_jrtrainerf"); // #inputs: 168063
  // r.load("multi_yellow_rock_tunnel_after_jrtrainerf");
  // r.run(WalkToPlan::new(15, 33)); // leave
  // r.run(WalkToPlan::new(14, 53)); // bush
  // r.run(WalkToPlan::new(15, 53)); // bush
  // r.run(OverworldInteractPlan::with_hidden_item());
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Max Ether
  // r.run(WalkToPlan::new(15, 60)); // ledge
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(9, 72)); // Lavender Town
  // r.run(WalkToPlan::new(-1, 8)); // Route 8
  // r.run(WalkToPlan::new(47, 13)); // Juggler
  // r.run(OverworldInteractPlan::with(8));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Juggler
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up(1));
  // r.save("multi_yellow_route_8_after_juggler"); // #inputs: 171154
  // r.load("multi_yellow_route_8_after_juggler");
  // r.run(WalkToPlan::new(13, 3)); // underground
  // r.run(WalkToPlan::new(4, 4)); // underground
  // r.run(OverworldOpenStartMenuPlan::new());
  // r.run(StartMenuPlan::items());
  // r.run(ListMenuPlan::choose(0)); // Bike
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // r.run(WalkToPlan::new(21, 3)); // Elixer
  // r.run(WalkToPlan::new(21, 4)); // Elixer
  // r.run(OverworldInteractPlan::with_hidden_item()); // Elixer
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Elixer
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
  // r.run(WalkToPlan::new(-1, 3)); // Celadon City
  // r.save("multi_yellow_celadon");
  // r.load("multi_yellow_celadon");
  // r.run(WalkToPlan::new(10, 13)); // Mart
  // r.run(WalkToPlan::new(12, 1)); // 2F
  // { // buy TM07 x2
  //   r.run(WalkToPlan::new(9, 3));
  //   r.run(WalkToPlan::new(8, 3));
  //   r.run(OverworldInteractPlan::with(2));
  //   r.run(TextPlan::new()); // How can I help you
  //   r.run(BuySellQuitMenuPlan::buy());
  //   r.run(TextPlan::new()); // Take your time
  //   r.run(SeqPlan::new(ListMenuPlan::choose(3), ChooseQuantityMenuPlan::new(2))); // Choose TM07 x2
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
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(12, 3));
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with(5));
  // r.run(SkipTextsPlan::new(1)); // Vanding Machine text
  // r.run(VendingMachineMenuPlan::fresh_water());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(1), HoldTextDisplayOpenPlan)); // Vanding Machine text
  // r.run(WalkToPlan::new(15, 2)); // 5F
  // { // buy XSpd x6
  //   r.run(WalkToPlan::new(5, 5));
  //   r.run(OverworldTurnPlan::new(U));
  //   r.run(OverworldInteractPlan::with(3));
  //   r.run(TextPlan::new()); // How can I help you
  //   r.run(BuySellQuitMenuPlan::buy());
  //   r.run(TextPlan::new()); // Take your time
  //   r.run(SeqPlan::new(ListMenuPlan::choose(5), ChooseQuantityMenuPlan::new(7))); // Choose XSpd x7
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Item? // That will be
  //   r.run(TextPlan::new().with_skip_ends(2)); // Price // Okay?
  //   r.run(TwoOptionMenuPlan::yes()); // buy
  //   r.run(SkipTextsPlan::new(1)); // Here you go
  //   r.run(ListMenuPlan::quit()); // exit buy menu
  //   r.run(TextPlan::new()); // Anything else?
  //   r.run(BuySellQuitMenuPlan::quit());
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Thank you!
  // }
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(WalkToPlan::new(1, 1)); // Elevator
  // r.run(WalkToPlan::new(3, 2)); // Elevator
  // r.run(WalkToPlan::new(3, 1)); // Elevator
  // r.run(OverworldInteractPlan::with(1)); // Elevator
  // r.run(SeqPlan::new(ListMenuPlan::choose(0), HoldTextDisplayOpenPlan)); // 1F
  // r.run(WalkToPlan::new(2, 2)); // Elevator
  // r.run(WalkToPlan::new(2, 3)); // Elevator
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(2, 6)); // leave
  // r.run(WalkToPlan::new(2, 7)); // leave
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(-1, 18));
  // r.run(WalkToPlan::new(34, 10)); //  #inputs: 177743
  // r.run(OverworldTurnPlan::new(U));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(1)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(25, 5));
  // r.run(WalkToPlan::new(24, 5));
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
  // r.run(SeqPlan::new(SkipTextsPlan::new(5), HoldTextDisplayOpenPlan)); // Fly // #inputs: 178979
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(WalkToPlan::new(2, 7));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::swap(1)); // TM34
  //   r.run(ListMenuPlan::swap(12)); // Doll
  //   r.run(ListMenuPlan::choose(11)); // TM07
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up TM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Horn Drill?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(0)); // Nidoking
  //   r.run(OverrideMovePlan::choose(0)); // Forget Bubblebeam
  //   r.run(ListMenuPlan::choose(15)); // HM02
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up TM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Fly?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // learned move
  //   r.run(ListMenuPlan::quit());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(PartyMonMenuPlan::choose(0)); // Fly
  //   r.save("multi_yellow_test4");
  //   r.load("multi_yellow_test4");
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
  // r.save("multi_yellow_test5");
  // r.load("multi_yellow_test5");
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(MoveEffectMetric.expect(MoveEffectResult::Thrash { turns: 3 }).and_then(Gen1NormalHitMetric::with_expected_max_damage(79, 130)).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 54 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(55, 90).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 46 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(51, 84).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 53 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SkipTextsPlan::new(1)); // critical hit
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(63, 103).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 66 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SkipTextsPlan::new(1)); // critical hit
  // r.run(EndTrainerBattlePlan::with_level_up(2)); // Rival fight //  #inputs: 185042
  // r.save("multi_yellow_tower_after_rival");
  // r.load("multi_yellow_tower_after_rival");
  // r.run(SeqPlan::new(SkipTextsPlan::new(10), HoldTextDisplayOpenPlan)); // rival post-fight text
  // r.run(WalkToPlan::new(3, 9)); // 3F
  // r.run(WalkToPlan::new(18, 9)); // 4F
  // r.run(WalkToPlan::new(17, 7)); // Channeler
  // r.run(WalkToPlan::new(16, 7)); // Channeler
  // r.run(OverworldInteractPlan::with(2));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Channeler
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 188557
  // r.save("multi_yellow_tower_after_channeler1");
  // r.load("multi_yellow_tower_after_channeler1");
  // r.run(WalkToPlan::new(13, 10)); // Elixer
  // r.run(OverworldInteractPlan::with(4)); // Elixer
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Elixer
  // r.run(WalkToPlan::new(3, 9)); // 5F
  // r.run(WalkToPlan::new(11, 9)); // Heal pad
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // heal pad
  // r.run(WalkToPlan::new(18, 9)); // 6F
  // r.run(WalkToPlan::new(15, 5)); // Channeler
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Channeler
  // r.run(OverworldWaitPlan::trainer_battle(245)); // Bugcatcher fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 191669
  // r.save("multi_yellow_tower_after_channeler2");
  // r.load("multi_yellow_tower_after_channeler2");
  // r.run(WalkToPlan::new(11, 5)); // Channeler
  // r.run(WalkToPlan::new(10, 5)); // Channeler
  // r.run(OverworldInteractPlan::with(2));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Channeler
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 193258
  // r.save("multi_yellow_tower_after_channeler3");
  // r.load("multi_yellow_tower_after_channeler3");
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
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(BattleMenuPlan::items());
  // r.run(ListMenuPlan::choose(1)); // Doll
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Use doll
  // r.run(SkipTextsPlan::new(1)); // Marowak gone
  // r.run(TextPlan::new()); // Marowak gone
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Marowak gone
  // r.save("multi_yellow_tower_after_marowak");
  // r.load("multi_yellow_tower_after_marowak");
  // r.run(WalkToPlan::new(9, 16)); // 7F
  // r.run(WalkToPlan::new(10, 12)); // Rocket
  // r.run(SeqPlan::new(TextPlan::new(), HoldTextDisplayOpenPlan)); // Rocket text
  // r.run(SeqPlan::new(SkipTextsPlan::new(5), HoldTextDisplayOpenPlan)); // Rocket text
  // r.run(OverworldWaitPlan::trainer_battle(230)); // Rocket fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Rocket fight
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::new(1)); // Rocket fight //  #inputs: 197919
  // r.save("multi_yellow_tower_after_rocket");
  // r.load("multi_yellow_tower_after_rocket");
  // r.run(SkipTextsPlan::new(1)); // Rocket after text
  // r.run(SeqPlan::new(TextPlan::new(), HoldTextDisplayOpenPlan)); // Rocket after text
  // r.run(WalkToPlan::new(10, 4));
  // r.run(OverworldInteractPlan::with(3));
  // r.run(SeqPlan::new(SkipTextsPlan::new(12), HoldTextDisplayOpenPlan)); // Fuji
  // r.run(WalkToPlan::new(2, 1));
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with(5)); // Fuji
  // r.run(SkipTextsPlan::new(5)); // Fuji
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Poke FLute
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan)); // Fuji
  // r.run(WalkToPlan::new(2, 7));
  // r.run(EdgeWarpPlan::new());
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(PartyMonMenuPlan::choose(0)); // Fly
  //   r.run(FlyToPlan::to_celadon_city());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // r.run(WalkToPlan::new(41, 9)); // Center
  // r.run(WalkToPlan::new(3, 3)); // Center
  // r.run(OverworldInteractPlan::with(1)); // Center
  // r.run(SkipTextsPlan::new(3)); // Center
  // r.run(TwoOptionMenuPlan::yes()); // Center
  // r.run(TextPlan::new()); // Center
  // r.run(SkipTextsPlan::new(2)); // Center
  // r.run(SeqPlan::new(TextCommandPausePlan::new(), SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan))); // Center
  // r.run(WalkToPlan::new(3, 7)); // go outside
  // r.run(EdgeWarpPlan::new()); // go outside
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(-1, 18));
  // r.run(WalkToPlan::new(27, 10));
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::swap(1)); // Doll
  //   r.run(ListMenuPlan::swap(14)); // XSpd
  //   r.run(ListMenuPlan::choose(17)); // Flute
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Played flute
  //   r.run(StartMenuClosePlan::new());
  //   r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Snorlax fight
  //   r.run(OverworldWaitPlan::with_metric(OverworldInteractionMetric.filter(|r| if let OverworldInteractionResult::WildEncounter { species: Pokemon::Snorlax, ..} = r { true } else { false })));
  // }
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Wild Snorlax appeared!
  // r.run(TextPlan::new().with_skip_ends(2)); // Go, Nidoking!
  // r.run(BattleMenuPlan::run());
  // r.run(SkipTextsPlan::new(1)); // Got away safely! // #inputs: 203604
  // r.save("multi_yellow_after_snorlax");
  // r.load("multi_yellow_after_snorlax");
  // r.run(WalkToPlan::new(24, 10));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(0, 8));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(11, 18));
  // r.run(WalkToPlan::new(13, 142));
  // r.run(OverworldJumpLedgePlan::new(D));
  // r.run(WalkToPlan::new(32, 8));
  // r.run(WalkToPlan::new(33, 8));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(7, 4));
  // r.run(EdgeWarpPlan::new()); // #inputs: 206154
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(50, 8)); // Fuchsia
  // r.run(WalkToPlan::new(18, 3)); // Enter Safari // 207167
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(WalkToPlan::new(3, 2));
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Welcome
  // r.run(OverworldWaitPlan::auto_walk(R));
  // r.run(SkipTextsPlan::new(3)); // Welcome
  // r.run(TextPlan::new()); // Do Safari?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Welcome
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan)); // Welcome // #inputs: 207903
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
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(SkipTextsPlan::new(7)); // Surf
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan)); // Get HM // #inputs: 211561
  // r.run(WalkToPlan::new(3, 7));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(0)); // Dig
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(1)); // Pidgey
  //   r.run(PartyMonMenuPlan::choose(0)); // Fly
  //   r.run(FlyToPlan::to_fuchsia_city());
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(5, 27)); // Enter Gym
  // r.run(WalkToPlan::new(7, 9));
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with(3));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(2)); // Juggler
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(MoveEffectMetric.expect(MoveEffectResult::Thrash { turns: 3 }).and_then(Gen1NormalHitMetric::with_expected_max_damage(61, 104)).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 83 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SkipTextsPlan::new(1)); // critical hit
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(61, 104).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 83 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SkipTextsPlan::new(1)); // critical hit
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_level_up_and_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(89, 147).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 70 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(68, 112).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 83 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SkipTextsPlan::new(1)); // critical hit
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 216690
  // r.save("multi_yellow_fuchsia_after_juggler1");
  // r.load("multi_yellow_fuchsia_after_juggler1");
  // r.run(WalkToPlan::new(1, 7)); // Juggler
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan)); // Juggler
  // r.run(OverworldWaitPlan::trainer_battle(221)); // Juggler fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Juggler
  // r.run(BattleMenuPlan::fight());
  // r.run(SeqPlan::new(
  //   SelectMoveMenuPlan::with_metric(Move::Thrash, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)),
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(61, 101).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 90 } else { false }), false).with_skip_ends(4))); // A used Thrash
  // r.run(SkipTextsPlan::new(1)); // critical hit
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(SeqPlan::new(
  //   NextTrainerMonPlan::with_metric(AIChooseMoveMetric.expect(Move::Confusion).and_then(BattleMoveOrderMetric).expect(MoveOrder::PlayerFirst)),
  //   SeqPlan::new(
  //     TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(44, 72).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage >= 39 } else { false }).into_unit(), false).with_skip_ends(0), // A thrashing about
  //     TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(62, 116).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage <= 99+2 } else { false }), false).with_skip_ends(4)))); // Hypno used confusion
  // r.run(SkipTextsPlan::new(1)); // critical hit
  // r.run(SeqPlan::new(
  //   SkipTextsPlan::with_metric(1, BattleMoveOrderMetric.expect(MoveOrder::PlayerFirst)), // super effective
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(44, 72).filter(|v| if let FightTurnResult::CriticalHit { damage } = v { *damage >= 68 } else { false }), false).with_skip_ends(0))); // A thrashing about
  // r.run(SkipTextsPlan::new(1)); // critical hit
  // r.run(EndTrainerBattlePlan::with_level_up(1)); // #inputs: 219998
  // r.save("multi_yellow_fuchsia_after_juggler2");
  // r.load("multi_yellow_fuchsia_after_juggler2");
  // r.run(WalkToPlan::new(3, 10)); // Koga
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(9)); // Koga
  // r.run(BattleMenuPlan::items());
  // r.run(ListMenuPlan::choose(1)); // XSpd
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Used XSpd
  // r.run(SeqPlan::new(
  //   SkipTextsPlan::with_metric(1, AIChooseMoveMetric.expect(Move::SleepPowder).and_then(TrainerAIMetric).expect(TrainerAIAction::NoAction)).with_skip_ends(2), // Speed rose
  //   TextPlan::with_metric(MoveEffectMetric.expect(MoveEffectResult::Failed), false).with_skip_ends(4))); // Venonat used Sleep Powder
  // r.run(SkipTextsPlan::new(1)); // Didin't affect Nidoking // 221455
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::with_level_up(3)); // #inputs: 223671
  // r.save("multi_yellow_fuchsia_after_koga");
  // r.load("multi_yellow_fuchsia_after_koga");
  // r.run(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan)); // after texts
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // not enough space
  // r.run(WalkToPlan::new(5, 16));
  // r.run(WalkToPlan::new(5, 17));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
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
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan)); // Get HM04 // #inputs: 225902
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(4, 6));
  // r.run(WalkToPlan::new(4, 7));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(4)); // Ether
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(PartyMenuPlan::choose(0)); // Nidoking
  //   r.run(TextPlan::new()); // which technique?
  //   r.run(SelectMoveMenuPlan::new(Move::HornDrill));
  //   r.run(SkipTextsPlan::new(1)); // PP restored
  //   r.run(ListMenuPlan::swap(2)); // Helix Fossil
  //   r.run(ListMenuPlan::swap(9)); // Elixer
  //   r.run(ListMenuPlan::choose(17)); // HM03
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up HM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Surf?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(0)); // Nidoking
  //   r.run(OverrideMovePlan::choose(2)); // Override Leer
  //   r.run(ListMenuPlan::quit());
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
  //   r.run(PartyMenuPlan::choose(0)); // Nidoking
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
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Who wouldn't? // #inputs: 230872
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(WalkToPlan::new(16, 14));
  // r.run(OverworldWaitPlan::fly_warp());
  // r.run(WalkToPlan::new(21, 23)); // B1F
  // r.run(WalkToPlan::new(19, 24));
  // r.run(OverworldInteractPlan::with(5));
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found TM14
  // r.run(WalkToPlan::new(18, 26)); // Switch
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item());
  // r.run(SkipTextsPlan::new(1)); // Switch
  // r.run(TextPlan::new()); // press?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Who wouldn't? // #inputs: 232451
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(WalkToPlan::new(20, 4)); // Switch
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item());
  // r.run(SkipTextsPlan::new(1)); // Switch
  // r.run(TextPlan::new()); // press?
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Who wouldn't? // #inputs: 233493
  // r.save("multi_yellow_test4");
  // r.load("multi_yellow_test4");
  // r.run(WalkToPlan::new(5, 12)); // Secret Key
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(19)); // TM14
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up HM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Blizzard?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(0)); // Nidoking
  //   r.run(OverrideMovePlan::choose(1)); // Override Thrash
  //   r.run(ListMenuPlan::quit());
  //   r.run(StartMenuPlan::close());
  // }
  // r.run(OverworldInteractPlan::with(8));
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Secret Key
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(0)); // Dig
  //   r.run(OverworldWaitPlan::fly_warp());
  // }
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
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(1)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(12, 27)); // Enter Gym
  // r.run(WalkToPlan::new(1, 4));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(1)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(3, 4)); // Beauty
  // r.run(OverworldWaitPlan::new()); // Point-blank Trainer script load
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Beauty
  // r.run(OverworldWaitPlan::trainer_battle(218)); // Beauty fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Beauty fight
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 238715
  // r.save("multi_yellow_celadon_after_beauty");
  // r.load("multi_yellow_celadon_after_beauty");
  // r.run(WalkToPlan::new(4, 4)); // Erika
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(13)); // Erika
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up(3)); // #inputs: 241804
  // r.save("multi_yellow_celadon_after_erika");
  // r.load("multi_yellow_celadon_after_erika");
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan)); // after texts
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // not enough space
  // r.run(WalkToPlan::new(5, 5));
  // r.run(WalkToPlan::new(5, 6));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(1)); // Cut
  //   r.run(TextScrollWaitPlan::new()); // used cut
  // }
  // r.run(WalkToPlan::new(5, 17));
  // r.run(EdgeWarpPlan::new()); // edge warp
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
  // r.run(SkipTextsPlan::new(9)); // Q1
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Q1 // #inputs: 244353
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(10, 2)); // Q2
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item()); // Q2
  // r.run(SkipTextsPlan::new(2)); // Q2
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::no());
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Q2 // #inputs: 245050
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(WalkToPlan::new(9, 8)); // Q3
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item()); // Q3
  // r.run(SkipTextsPlan::new(1)); // Q3
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::no());
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Q3
  // r.run(WalkToPlan::new(9, 14)); // Q4
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_hidden_item()); // Q4
  // r.run(SkipTextsPlan::new(3)); // Q4
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::no());
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Q4
  // r.run(WalkToPlan::new(1, 15)); // Q5
  // r.run(WalkToPlan::new(1, 14)); // Q5
  // r.run(OverworldInteractPlan::with_hidden_item()); // Q5
  // r.run(SkipTextsPlan::new(3)); // Q5
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::yes());
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Q5
  // r.run(WalkToPlan::new(1, 9)); // Q6
  // r.run(WalkToPlan::new(1, 8)); // Q6
  // r.run(OverworldInteractPlan::with_hidden_item()); // Q6
  // r.run(SkipTextsPlan::new(1)); // Q6
  // r.run(TextPlan::new());
  // r.run(TwoOptionMenuPlan::no());
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Q6
  // r.run(WalkToPlan::new(3, 5)); // Blaine
  // r.run(WalkToPlan::new(3, 4)); // Blaine
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(6)); // Blaine
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(BattleMenuPlan::items());
  // r.run(ListMenuPlan::choose(1)); // XSpd
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Used XSpd
  // r.run(SeqPlan::new(
  //   SkipTextsPlan::with_metric(1, AIChooseMoveMetric.expect(Move::QuickAttack).and_then(TrainerAIMetric).expect(TrainerAIAction::NoAction)).with_skip_ends(2), // Speed rose
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(18, 41).filter(|v| if let FightTurnResult::Hit { damage } = v { (14..=(17-2)).contains(damage) } else { false }), false).with_skip_ends(4))); // Ninetales used Quick Attack
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::new(2)); // #inputs: 250710
  // r.save("multi_yellow_after_blaine");
  // r.load("multi_yellow_after_blaine");
  // r.run(SeqPlan::new(SkipTextsPlan::new(5), HoldTextDisplayOpenPlan)); // after texts
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // not enough space
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(0)); // Dig
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
  // r.run(WalkToPlan::new(3, 3));
  // r.run(SeqPlan::new(SkipTextsPlan::new(15), HoldTextDisplayOpenPlan)); // Give drink // #inputs: 252542
  // r.run(WalkToPlan::new(5, 3));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(20, 10));
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(WalkToPlan::new(18, 21)); // enter Silph Co
  // r.run(WalkToPlan::new(26, 0)); // 1F
  // r.run(WalkToPlan::new(26, 0)); // 2F
  // r.run(WalkToPlan::new(24, 0)); // 3F
  // r.run(WalkToPlan::new(26, 0)); // 4F
  // r.run(WalkToPlan::new(14, 3));
  // r.run(WalkToPlan::new(13, 3));
  // r.run(OverworldInteractPlan::with_hidden_item());
  // r.run(SeqPlan::new(TextPlan::new().with_skip_ends(2), HoldTextDisplayOpenPlan)); // Found Elixer
  // r.run(WalkToPlan::new(8, 14));
  // r.run(WalkToPlan::new(8, 15));
  // r.run(OverworldInteractPlan::with(2));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(1)); // Rocket
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up(1)); // #inputs: 256272
  // r.save("multi_yellow_saffron_after_rocket1");
  // r.load("multi_yellow_saffron_after_rocket1");
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
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
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
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
  // r.run(BattleMenuPlan::items());
  // r.run(ListMenuPlan::choose(2)); // Elixer
  // r.run(PartyMenuPlan::choose(0)); // Nidoking
  // r.run(SeqPlan::new(
  //   SkipTextsPlan::with_metric(1, AIChooseMoveMetric.expect(Move::Psybeam).and_then(TrainerAIMetric).expect(TrainerAIAction::NoAction)), // PP was restored
  //   TextPlan::with_metric(Gen1MoveSuccessMetric.expect(FightTurnResult::Failed), false).with_skip_ends(4))); // Kadabra used Psybeam
  // r.run(SkipTextsPlan::new(1)); // Move failed
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::new(2)); // #inputs: 263017
  // r.save("multi_yellow_saffron_after_rival");
  // r.load("multi_yellow_saffron_after_rival");
  // r.run(SeqPlan::new(SkipTextsPlan::new(14), HoldTextDisplayOpenPlan)); // after texts
  // r.run(WalkToPlan::new(5, 7));
  // r.run(WalkToPlan::new(3, 3));
  // r.run(SeqPlan::new(TextPlan::new(), HoldTextDisplayOpenPlan)); // Rocket
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan)); // Rocket
  // r.run(OverworldWaitPlan::trainer_battle(230)); // Rocket fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Rocket fight
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // Rocket fight //  #inputs: 266747
  // r.save("multi_yellow_saffron_after_rocket");
  // r.load("multi_yellow_saffron_after_rocket");
  // r.run(SkipTextsPlan::new(2)); // after text
  // r.run(SeqPlan::new(TextPlan::new(), HoldTextDisplayOpenPlan));
  // r.run(WalkToPlan::new(6, 14));
  // r.run(OverworldTurnPlan::new(U));
  // r.run(OverworldInteractPlan::with_card_key_door()); // Door
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // Card Key
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // Card Key
  // r.run(WalkToPlan::new(6, 13));
  // r.run(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan)); // Giovanni
  // r.run(OverworldWaitPlan::trainer_battle(229)); // Giovanni fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Giovanni fight
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::with_level_up(1)); // #inputs: 270969
  // r.save("multi_yellow_saffron_after_giovanni");
  // r.load("multi_yellow_saffron_after_giovanni");
  // r.run(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan)); // Giovanni post-fight text
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(0)); // Dig
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
  // r.run(WalkToPlan::new(5, 3));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(20, 10));
  // r.run(WalkToPlan::new(34, 3)); // Enter Gym // 273063
  // r.run(WalkToPlan::new(11, 15));
  // r.run(WalkToPlan::new(15, 15));
  // r.run(WalkToPlan::new(15, 5));
  // r.run(WalkToPlan::new(1, 5));
  // r.run(WalkToPlan::new(9, 10));
  // r.run(WalkToPlan::new(9, 9));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(8)); // Sabrina
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(BattleMenuPlan::items());
  // r.run(ListMenuPlan::choose(1)); // XSpd
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Used XSpd
  // r.run(SeqPlan::new(
  //   SkipTextsPlan::with_metric(1, AIChooseMoveMetric.expect(Move::Flash).and_then(TrainerAIMetric).expect(TrainerAIAction::NoAction)).with_skip_ends(2), // Speed rose
  //   TextPlan::with_metric(MoveEffectMetric.expect(MoveEffectResult::Failed), false).with_skip_ends(4))); // Abra used Flash
  // r.run(SkipTextsPlan::new(1)); // But it failed
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::with_level_up(6)); // #inputs: 277089
  // r.save("multi_yellow_saffron_after_sabrina");
  // r.load("multi_yellow_saffron_after_sabrina");
  // r.run(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan)); // after texts
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // not enough space
  // r.run(WalkToPlan::new(11, 11));
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::mon());
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(0)); // Dig
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
  //   r.run(ListMenuPlan::choose(8)); // Max Ether
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(PartyMenuPlan::choose(0)); // Nidoking
  //   r.run(TextPlan::new()); // which technique?
  //   r.run(SelectMoveMenuPlan::new(Move::HornDrill));
  //   r.run(SkipTextsPlan::new(1)); // PP restored
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(WalkToPlan::new(32, 7)); // Enter Gym
  // r.run(WalkToPlan::new(15, 5));
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::trainer_battle(231)); // Cooltrainer fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Cooltrainer
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 281316
  // r.save("multi_yellow_viridian_after_cooltrainerm");
  // r.load("multi_yellow_viridian_after_cooltrainerm");
  // r.run(WalkToPlan::new(10, 4));
  // r.run(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan));
  // r.run(OverworldWaitPlan::trainer_battle(224)); // Blackbelt fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Blackbelt
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Thunderbolt, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 284145
  // r.save("multi_yellow_viridian_after_blackbelt");
  // r.load("multi_yellow_viridian_after_blackbelt");
  // r.run(WalkToPlan::new(16, 16));
  // r.run(WalkToPlan::new(16, 17));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(32, 7)); // Enter Gym
  // r.run(WalkToPlan::new(15, 6));
  // r.run(WalkToPlan::new(2, 3));
  // r.run(WalkToPlan::new(2, 2));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(10)); // Giovanni
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(BattleMenuPlan::items());
  // r.run(ListMenuPlan::choose(1)); // XSpd
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Used XSpd
  // r.run(SkipTextsPlan::with_metric(1, TrainerAIMetric.expect(TrainerAIAction::GuardSpec)).with_skip_ends(2)); // Speed rose
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Used Guard Spec
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // on Dugtrio
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // enemy // mon // fainted
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // mon // gained // num XP
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // I defeated U
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // First defeat text
  // r.run(SkipTextsPlan::new(3)); // Additional defeat texts
  // r.run(TextPlan::new()); // Additional defeat texts
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // player got // X for winning // #inputs: 290003
  // r.save("multi_yellow_viridian_after_giovanni");
  // r.load("multi_yellow_viridian_after_giovanni");
  // r.run(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan)); // after texts
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan)); // after texts
  // r.run(WalkToPlan::new(15, 6));
  // r.run(WalkToPlan::new(16, 16));
  // r.run(WalkToPlan::new(16, 17));
  // r.run(EdgeWarpPlan::new());
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(16)); // HM04
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up HM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Strength?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(OverrideMovePlan::choose(0)); // Override Dig
  //   r.run(ListMenuPlan::choose(19)); // TM27
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(SkipTextsPlan::new(1)); // Booted up TM
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // It contains // move // !
  //   r.run(TextPlan::new().with_skip_ends(2)); // Teach Fissure?
  //   r.run(TwoOptionMenuPlan::yes());
  //   r.run(PartyMenuPlan::choose(0)); // Nidoking
  //   r.run(OverrideMovePlan::choose(3)); // Override Thunderbolt
  //   r.run(ListMenuPlan::choose(0)); // Bike
  //   r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), StartMenuClosePlan::new())); // Get on Bike
  // }
  // r.run(OverworldJumpLedgePlan::new(D));
  // r.run(WalkToPlan::new(-1, 17)); // Route 22
  // r.run(WalkToPlan::new(29, 5)); // Rival
  // r.run(SeqPlan::new(SkipTextsPlan::new(10), HoldTextDisplayOpenPlan)); // Rival
  // r.run(OverworldWaitPlan::trainer_battle(242)); // initiate Rival fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(BattleMenuPlan::items());
  // r.run(ListMenuPlan::choose(1)); // XSpd
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Used XSpd
  // r.run(SeqPlan::new(
  //   SkipTextsPlan::with_metric(1, AIChooseMoveMetric.expect(Move::Slash).and_then(TrainerAIMetric).expect(TrainerAIAction::NoAction)).with_skip_ends(2), // Speed rose
  //   TextPlan::with_metric(Gen1NormalHitMetric::with_expected_max_damage(30, 70).filter(|v| if let FightTurnResult::Hit { damage } = v { *damage <= 27 } else { false }), false).with_skip_ends(4))); // Sandslash used Swift
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack)); // 295560
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::with_level_up(2)); // #inputs: 298795
  // r.save("multi_yellow_route22_after_rival");
  // r.load("multi_yellow_route22_after_rival");
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
  //   r.run(PartyMenuPlan::choose(0)); // Nidoking
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
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(0)); // Strength
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Used Strength // #inputs: 304235
  // }
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
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
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(0)); // Strength
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Used Strength // #inputs: 306673
  // }
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
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
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(0)); // Strength
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Used Strength // #inputs: 308107
  // }
  // r.save("multi_yellow_test3");
  // r.load("multi_yellow_test3");
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
  //   r.run(PartyMenuPlan::choose(2)); // Charmander
  //   r.run(PartyMonMenuPlan::choose(0)); // Strength
  //   r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Used Strength // #inputs: 311735
  // }
  // r.save("multi_yellow_test4");
  // r.load("multi_yellow_test4");
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(2)); // Elixer
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(PartyMenuPlan::choose(0)); // Nidoking
  //   r.run(SkipTextsPlan::new(1)); // PP restored
  // r.run(ListMenuPlan::choose(0)); // Bike
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
  // r.run(OverworldInteractPlan::with_hidden_item()); // PC
  // r.run(SkipTextsPlan::new(1)); // PC access
  // r.run(PCMainMenuPlan::mon()); // Bill's PC
  // r.run(SkipTextsPlan::new(2)); // Bill's PC
  // r.run(BillsPCMenuPlan::deposit()); // Bill's PC
  // r.run(ListMenuPlan::choose(1)); // Deposit Pidgey
  // r.run(DepositWithdrawMenuPlan::deposit()); // Deposit Pidgey
  // r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Pidgey deposited
  // r.run(BillsPCMenuPlan::deposit()); // Bill's PC
  // r.run(ListMenuPlan::choose(1)); // Deposit Charmander
  // r.run(DepositWithdrawMenuPlan::deposit()); // Deposit Charmander
  // r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // Charmander deposited
  // r.run(BillsPCMenuPlan::quit()); // Bill's PC
  // r.run(PCMainMenuPlan::quit()); // PC
  // r.run(HoldTextDisplayOpenPlan); // PC // #inputs: 315177
  // r.save("multi_yellow_test5");
  // r.load("multi_yellow_test5");
  // r.run(WalkToPlan::new(8, 0));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(OverworldWaitPlan::auto_walk(U));
  // r.run(WalkToPlan::new(4, 2));
  // r.run(OverworldTurnPlan::new(R));
  // r.run(OverworldInteractPlan::with(1));
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(9));
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 319964
  // r.save("multi_yellow_after_lorelei");
  // r.load("multi_yellow_after_lorelei");
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan)); // after text
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
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::new(1)); // #inputs: 324457
  // r.save("multi_yellow_after_bruno");
  // r.load("multi_yellow_after_bruno");
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
  // r.run(BattleMenuPlan::items());
  // r.run(ListMenuPlan::choose(1)); // XSpd
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Used XSpd
  // r.run(SeqPlan::new(
  //   SkipTextsPlan::with_metric(1, AIChooseMoveMetric.expect(Move::Lick).and_then(TrainerAIMetric).expect(TrainerAIAction::NoAction)).with_skip_ends(2), // Speed rose
  //   TextPlan::with_metric(Gen1MoveSuccessMetric.expect(FightTurnResult::Failed), false).with_skip_ends(4))); // Gengar used Lick
  // r.run(SkipTextsPlan::new(1)); // But it failed // 326329
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up()); // 326844
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack)); // 327628 - 327411 = 217
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::with_level_up(2)); // #inputs: 329390
  // r.save("multi_yellow_after_agatha");
  // r.load("multi_yellow_after_agatha");
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan)); // after text
  // {
  //   r.run(OverworldOpenStartMenuPlan::new());
  //   r.run(StartMenuPlan::items());
  //   r.run(ListMenuPlan::choose(2)); // Elixer
  //   r.run(ItemUseTossMenuPlan::use_());
  //   r.run(PartyMenuPlan::choose(0)); // Nidoking
  //   r.run(SkipTextsPlan::new(1)); // PP restored
  //   r.run(ListMenuPlan::quit());
  //   r.run(StartMenuPlan::close());
  // }
  // r.run(WalkToPlan::new(4, 1));
  // r.run(WalkToPlan::new(4, 0));
  // r.run(EdgeWarpPlan::new());
  // r.run(WalkToPlan::new(5, 1));
  // r.run(SeqPlan::new(SkipTextsPlan::new(13), HoldTextDisplayOpenPlan)); // Lance
  // r.run(OverworldWaitPlan::trainer_battle(247)); // initiate Lance fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::HyperBeam))));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightKOPlan::new(Move::Blizzard, None, EnemyAttackDesc::NoAttack));
  // r.run(EndTrainerBattlePlan::new(3)); // #inputs: 335173
  // r.save("multi_yellow_after_lance");
  // r.load("multi_yellow_after_lance");
  // r.run(SeqPlan::new(SkipTextsPlan::new(14), HoldTextDisplayOpenPlan)); // after text
  // r.run(WalkToPlan::new(5, 0));
  // r.run(EdgeWarpPlan::new());
  // r.run(SeqPlan::new(SkipTextsPlan::new(18), HoldTextDisplayOpenPlan)); // Rival
  // r.run(OverworldWaitPlan::trainer_battle(243)); // initiate Rival fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0));
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(BattleMenuPlan::items());
  // r.run(ListMenuPlan::choose(1)); // XSpd
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Used XSpd
  // r.run(SeqPlan::new(
  //   SkipTextsPlan::with_metric(1, AIChooseMoveMetric.expect(Move::Kinesis).and_then(TrainerAIMetric).expect(TrainerAIAction::NoAction)).with_skip_ends(2), // Speed rose
  //   TextPlan::with_metric(MoveEffectMetric.expect(MoveEffectResult::Failed), false).with_skip_ends(4))); // Alazkzam uses Kinesis
  // r.run(SkipTextsPlan::new(1)); // But it failed
  // r.save("multi_yellow_test2");
  // r.load("multi_yellow_test2");
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst))); // 338996 // Fissure: 339097 - 338996 = 101 (-5)
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::new());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::HornDrill), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightTurnPlan::new(AttackDesc::ohko(Move::Fissure), EnemyAttackDesc::NoAttack, Some(MoveOrder::PlayerFirst)));
  // r.run(EndTrainerBattlePlan::new(6)); // #inputs: 341875
  // r.save("multi_yellow_after_champion");
  // r.load("multi_yellow_after_champion");
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan)); // after text
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // after text
  // r.run(SkipTextsPlan::new(6)); // after text
  // r.run(SkipTextsPlan::new(1).with_skip_ends(1)); // after text
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)); // after text
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");
  // r.run(SeqPlan::new(SkipTextsPlan::new(12), HoldTextDisplayOpenPlan)); // after text
  // r.run(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan)); // after text
  // r.run(SeqPlan::new(SkipTextsPlan::new(15), HoldTextDisplayOpenPlan)); // after text
  // r.save("multi_yellow_test2");
  r.load("multi_yellow_test2");
  r.run(TextPlan::new().with_skip_ends(2));
  r.run(SkipTextsPlan::new(2));

  
  // r.save("multi_yellow_test");
  // r.load("multi_yellow_test");

  // r.debug_print_state_fn(MoveInfosFn::new(Who::Player));
  // r.debug_print_state_fn(BattleMonInfoFn::new(Who::Player));
  // r.debug_print_state_fn(MoveInfosFn::new(Who::Enemy));
  // r.debug_print_state_fn(BattleMonInfoFn::new(Who::Enemy));

  r.debug_segment_end("temp/multi_yellow");
}
