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
  // r.run(SkipTextsPlan::new(1).with_skip_ends(2)); // Wild Pikachu appeared!
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
  // r.save("multi_yellow_viridian_after_bugcatcher2"); // #inputs: 34984
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
  r.load("multi_yellow_test");
  r.run(FightKOPlan::new(Move::HornAttack, None, EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::Scratch))));
  r.run(EndTrainerBattlePlan::with_learn_move(3)); // JrTrainerM fight //  #inputs: 
  r.save("multi_red_pewter_after_jrtrainerm"); // #inputs: 39601
  // r.load("multi_red_pewter_after_jrtrainerm");
  // r.run(WalkToPlan::new(4, 2)); // stand in front of Brock
  // r.run(OverworldInteractPlan::with(1)); // Brock
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(9));
  // r.run(FightKOPlan::new(Move::Ember, Some(MoveEffectResult::NoEffect), EnemyAttackDesc::Attack(AttackDesc::hit_failed(Move::Tackle))));
  // r.run(NextTrainerMonPlan::with_level_up());
  // r.run(FightKOPlan::new(Move::Ember, Some(MoveEffectResult::NoEffect), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Screech))));
  // r.run(EndTrainerBattlePlan::with_level_up(10)); // Brock fight //  #inputs: 33490
  // r.save("multi_red_after_brock_");
  // r.load("multi_red_after_brock_");

  // r.save("multi_red_test");
  // r.load("multi_red_test");

  // r.debug_print_state_fn(MoveInfosFn::new(Who::Player));
  // r.debug_print_state_fn(BattleMonInfoFn::new(Who::Player));
  // r.debug_print_state_fn(MoveInfosFn::new(Who::Enemy));
  // r.debug_print_state_fn(BattleMonInfoFn::new(Who::Enemy));

  r.debug_segment_end("temp/multi_yellow");
}
