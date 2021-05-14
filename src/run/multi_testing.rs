#[allow(unused_imports)] use gambatte::inputs::*;
#[allow(unused_imports)] use montas::constants::*;
#[allow(unused_imports)] use montas::metric::*;
#[allow(unused_imports)] use montas::metric::overworld::gen1::*;
use montas::multi::*;
use montas::rom::*;
use montas::sdl::*;

const EQUAL_LENGTH_FRAMES: bool = true;
const RTC_DIVISOR_OFFSET: i32 = 0;

#[allow(dead_code)]
pub fn start() {
  log::set_max_level(log::LevelFilter::Debug);

  let sdl = Sdl::init_sdl(3 /* num screens */, 3 /* scale */);
  let blue_gb = Gb::<Blue>::create(EQUAL_LENGTH_FRAMES, RTC_DIVISOR_OFFSET, SdlScreen::new(sdl.clone(), 0));
  let blue_executor = MultiGbExecutor::new(blue_gb, blue_plan());
  let red_gb = Gb::<Red>::create(EQUAL_LENGTH_FRAMES, RTC_DIVISOR_OFFSET, SdlScreen::new(sdl.clone(), 1));
  let red_executor = MultiGbExecutor::new(red_gb, red_plan());
  let yellow_gb = Gb::<Yellow>::create(EQUAL_LENGTH_FRAMES, RTC_DIVISOR_OFFSET, SdlScreen::new(sdl, 2));
  let yellow_executor = MultiGbExecutor::new(yellow_gb, yellow_plan());
  let mut r = MultiGbRunner::new([
    Box::new(blue_executor),
    Box::new(red_executor),
    Box::new(yellow_executor),
  ]);

  // r.load("multi_all_intro");
  // r.load("multi_all_pika_caught");
  // r.load("multi_test");
  // r.run_until(5500);
  // r.save("multi_test");
  // r.load("multi_test");
  // r.run_until(5800);
  // r.save("multi_before_blue_starter");
  // r.load("multi_before_blue_starter");
  // // r.run();
  // r.save("multi_after_blue_starter");
  // r.load_final_only("multi_after_blue_starter");
  // r.load("multi_test2"); // #inputs: 18749
  // r.run();
  // r.save("multi_after_blue_pikachu");
  // r.load_final_only("multi_after_blue_pikachu");
  // r.run_until(22300);
  // r.save("multi_before_yellow_nidoran_");
  r.load("multi_before_yellow_nidoran_");
  r.run();
  r.save("multi_after_yellow_nidoran"); // #inputs:
  std::thread::sleep(std::time::Duration::from_millis(1000));

  r.debug_segment_end("temp/multi_testing");
}

fn blue_plan() -> ListPlan<Blue> {
  ListPlan::new(vec![
    Box::new(SkipIntroPlan::new().with_auto_pass_after(214)), // Logo
    Box::new(SkipIntroPlan::new().with_auto_pass_after(322)), // Intro cutscene
    Box::new(SkipIntroPlan::new().with_no_up_select_b()), // main menu
    Box::new(MainMenuPlan::new()), // main menu
    Box::new(SkipTextsPlan::new(13)), // oak speech
    Box::new(IntroNameMenuPlan::choose_custom_name()), // own name
    Box::new(NamingScreenPlan::with_letter(b'B')),
    Box::new(SkipTextsPlan::new(5)), // oak speech
    Box::new(IntroNameMenuPlan::choose_custom_name()), // rival name
    Box::new(NamingScreenPlan::with_letter(b'U')),
    Box::new(SkipTextsPlan::new(7)), // oak speech
    Box::new(TextPlan::new()), // ... awaits let's go // #inputs: 2500(2728)
    Box::new(WalkToPlan::new(7, 1)), // go down stairs
    Box::new(WalkToPlan::new(3, 6)), // go outside
    Box::new(WalkToPlan::new(3, 7)), // go outside
    Box::new(EdgeWarpPlan::new()), // go outside
    Box::new(WalkToPlan::new(10, 1)), // trigger oak cutscene
    Box::new(OverworldWaitPlan::new()), // Skip PalletTownScript0
    Box::new(TextPlan::new()), // it's dangerous to go outside, take this
    Box::new(HoldTextDisplayOpenPlan::new()), // close text box
    Box::new(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan::new())), // oak speech
    Box::new(OverworldWaitPlan::new()), // Skip PalletTownScript load
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow Oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(L)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(R)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(R)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(R)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(U)), // Follow oak
    Box::new(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())), // Fed up with waiting
    Box::new(SeqPlan::new(SkipTextsPlan::new(12), HoldTextDisplayOpenPlan::new())), // you can have one, choose
    Box::new(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())), // What about me?
    Box::new(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())), // Can have one too
    Box::new(WalkToPlan::new(7, 4)), // stand before squirtle
    Box::new(OverworldTurnPlan::new(U)), // turn towards squirtle
    Box::new(OverworldInteractPlan::with(3)), // Interact with Squirtle Ball
    Box::new(TextScrollWaitPlan::new()), // Scroll dex text 1
    Box::new(TextScrollWaitPlan::new()), // Scroll dex text 2
    Box::new(SkipTextsPlan::new(1)), // so you want Squirtle
    Box::new(TextPlan::new()), // so you want Squirtle?
    Box::new(TwoOptionMenuPlan::yes()), // choose Squirtle
    Box::new(SkipTextsPlan::new(1)), // looks really energetic
    Box::new(SkipTextsPlan::new(1).with_skip_ends(3)), // received Squirtle! Do you want...
    Box::new(TextPlan::new().with_skip_ends(2)), // give nickname?
    Box::new(TwoOptionMenuPlan::yes()), // give nickname
    Box::new(NamingScreenPlan::with_metric(b'S', Gen1DVMetric {}.filter(|v| {
      if v.atk != 14 || v.def < 6 || v.def > 8 || v.spc < 11 || v.spd < 15 || v.hp() > 3 || v.hp() < 2 { return false; } // squirtle DVs
      log::info!("Squirtle Candidate DVs: {:?}", v);
      if v.atk != 14 || v.def < 6 || v.def > 8 || v.spc != 15 || v.spd != 15 || v.hp() != 3 { return false; } // squirtle DVs
      log::info!("Squirtle DVs: {:?}", v); true
    }).into_unit())), // DVs: 14 / 6 / 15 / 15 // #inputs: 5914
    // multi_after_blue_starter
    Box::new(HoldTextDisplayOpenPlan::new()),
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // I'll take this one then
    Box::new(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan::new())), // rival received // bulbasaur // !
    Box::new(WalkToPlan::new(5, 6)), // trigger rival fight
    Box::new(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())), // Rival fight
    Box::new(OverworldWaitPlan::trainer_battle(225)), // Rival fight
    Box::new(StartTrainerBattlePlan::with_pre_battle_texts(0)), // Rival fight
    Box::new(FightKOPlan::new(Move::Tackle, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Growl)))),
    Box::new(EndTrainerBattlePlan::with_level_up(3)), // Rival1 fight // #inputs: 9080 9087 / 9094
    Box::new(OverworldWaitPlan::new()), // advance map script (abSs buttons allowed)
    Box::new(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())), // after rival1 texts
    Box::new(WalkToPlan::new(5, 11)), // Leave lab
    Box::new(EdgeWarpPlan::new()), // go outside // inputs: 9691 - 9617 = 74
    Box::new(WalkToPlan::new(10, -1)),
    Box::new(WalkToPlan::new(11, -1)), // Enter Viridian
    Box::new(WalkToPlan::new(29, 19)), // Enter Mart, starts cutscene
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // mart cutscene
    Box::new(OverworldWaitPlan::auto_walk(U)), // mart cutscene
    Box::new(OverworldWaitPlan::auto_walk(U)), // mart cutscene
    Box::new(OverworldWaitPlan::auto_walk(L)), // mart cutscene
    Box::new(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())), // mart cutscene
    Box::new(OverworldOpenStartMenuPlan::new()), // Open start menu
    Box::new(StartMenuPlan::options()), // main menu
    Box::new(ChangeOptionsPlan::new()), // set options
    Box::new(StartMenuPlan::close()), // main menu
    Box::new(WalkToPlan::new(3, 6)),
    Box::new(WalkToPlan::new(3, 7)),
    Box::new(EdgeWarpPlan::new()), // go outside // inputs: 11931 - 11865 = 66
    Box::new(WalkToPlan::new(29, 26)),
    Box::new(OverworldJumpLedgePlan::new(D)), // Jump ledge
    Box::new(WalkToPlan::new(21, 36)), // enter Route 1
    Box::new(WalkToPlan::new(14, 18)),
    Box::new(OverworldJumpLedgePlan::new(D)), // Jump ledge
    Box::new(WalkToPlan::new(12, 26)),
    Box::new(OverworldJumpLedgePlan::new(D)), // Jump ledge
    Box::new(WalkToPlan::new(11, 36)), // enter pallet town
    Box::new(WalkToPlan::new(12, 11)), // enter oak's lab
    Box::new(WalkToPlan::new(4, 2)), // next to oak
    Box::new(OverworldTurnPlan::new(R)), // turn towards Oak
    Box::new(OverworldInteractPlan::with(5)), // Talk to Oak
    Box::new(SeqPlan::new(SkipTextsPlan::new(10), HoldTextDisplayOpenPlan::new())), // Oak speech: special pokeball, thank you
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // Oak speech: Gramps
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // Oak speech: What came for?
    Box::new(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())), // Oak speech: Have something for you
    Box::new(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan::new())), // Oak speech: hi-tech encyclopedia
    Box::new(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan::new())), // Oak speech: Took Pokedex
    Box::new(SeqPlan::new(SkipTextsPlan::new(10), HoldTextDisplayOpenPlan::new())), // Oak speech: greatest undertaking
    Box::new(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan::new())), // Oak speech: leave it to me
    Box::new(WalkToPlan::new(4, 11)), // leave
    Box::new(EdgeWarpPlan::new()), // go outside // inputs: 16078 / 16086
    Box::new(WalkToPlan::new(10, -1)), // enter Route 1
    Box::new(WalkToPlan::new(11, -1)), // Enter Viridian
    Box::new(WalkToPlan::new(18, -1)), // Enter Route 2
    Box::new(WalkToPlan::new(3, 43)), // Enter Viridian Forest
    Box::new(WalkToPlan::new(5, 0)), // Enter Viridian Forest
    Box::new(WalkToEncounterPlan::new(2, 19, OverworldInteractionMetric.filter(|v| {
      if let OverworldInteractionResult::WildEncounter { species: Pokemon::Pikachu, level: 5, dvs } = v {
        if dvs.hp() <= 4 && dvs.def <= 9 {
          log::info!("Blue Pika DVs: {:?}", dvs); true
        } else { false }
      } else { false }
    }).into_unit())), // #inputs: 19407
    // multi_after_blue_pikachu
    Box::new(SkipTextsPlan::new(1).with_skip_ends(2)), // Wild Pikachu appeared!
    Box::new(TextPlan::new().with_skip_ends(2)), // Go, Pikachu!
    Box::new(FightTurnPlan::new(AttackDesc::hit(Move::Tackle, 6..=6), EnemyAttackDesc::Attack(AttackDesc::hit_no_side_effect(Move::ThunderShock, 10..=10)), None)),
    Box::new(FightTurnPlan::new(AttackDesc::hit(Move::Tackle, 6..=6), EnemyAttackDesc::Attack(AttackDesc::hit_no_side_effect(Move::ThunderShock, 10..=10)), None)),
    Box::new(FightTurnPlan::new(AttackDesc::hit(Move::Tackle, 6..=6), EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::Growl)), None)),
    Box::new(SkipTextsPlan::new(1).with_skip_ends(2)), // enemy // mon // fainted
    Box::new(SkipTextsPlan::new(1).with_skip_ends(2)), // mon // gained // num XP
    Box::new(SkipTextsPlan::new(1).with_skip_ends(2)), // mon // grew to level // X // #inputs:
    Box::new(WalkToPlan::new(2, 19)),
    Box::new(OverworldInteractPlan::with(4)), // Bugcatcher
    Box::new(StartTrainerBattlePlan::with_pre_battle_texts(1)),
    Box::new(FightKOPlan::new(Move::Tackle, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::StringShot)))),
    Box::new(EndTrainerBattlePlan::with_learn_move(2)), // Bugcatcher fight // #inputs:
  ])
}

fn red_plan() -> ListPlan<Red> {
  ListPlan::new(vec![
    Box::new(SkipIntroPlan::new().with_auto_pass_after(214)), // Logo
    Box::new(SkipIntroPlan::new().with_auto_pass_after(322)), // Intro cutscene
    Box::new(SkipIntroPlan::new().with_no_up_select_b()), // main menu
    Box::new(MainMenuPlan::new()), // main menu
    Box::new(SkipTextsPlan::new(13)), // oak speech
    Box::new(IntroNameMenuPlan::choose_custom_name()), // own name
    Box::new(NamingScreenPlan::with_letter(b'R')),
    Box::new(SkipTextsPlan::new(5)), // oak speech
    Box::new(IntroNameMenuPlan::choose_custom_name()), // rival name
    Box::new(NamingScreenPlan::with_letter(b'U')),
    Box::new(SkipTextsPlan::new(7)), // oak speech
    Box::new(TextPlan::new()), // ... awaits let's go
    Box::new(OverworldOpenStartMenuPlan::new()), // Open start menu
    Box::new(StartMenuPlan::options()), // main menu
    Box::new(ChangeOptionsPlan::new()), // set options
    Box::new(StartMenuPlan::close()), // main menu
    Box::new(WalkToPlan::new(7, 1)), // go down stairs
    Box::new(WalkToPlan::new(3, 6)), // go outside
    Box::new(WalkToPlan::new(3, 7)), // go outside
    Box::new(EdgeWarpPlan::new()), // go outside
    Box::new(WalkToPlan::new(10, 1)), // trigger oak cutscene
    Box::new(OverworldWaitPlan::new()), // Skip PalletTownScript0
    Box::new(TextPlan::new()), // it's dangerous to go outside, take this
    Box::new(HoldTextDisplayOpenPlan::new()), // close text box
    Box::new(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan::new())), // oak speech
    Box::new(OverworldWaitPlan::new()), // Skip PalletTownScript load
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow Oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(L)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(R)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(R)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(R)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(U)), // Follow oak
    Box::new(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())), // Fed up with waiting
    Box::new(SeqPlan::new(SkipTextsPlan::new(12), HoldTextDisplayOpenPlan::new())), // you can have one, choose
    Box::new(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())), // What about me?
    Box::new(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())), // Can have one too
    Box::new(OverworldTurnPlan::new(R)), // turn towards Charmander
    Box::new(OverworldInteractPlan::with(2)), // Interact with Charmander Ball
    Box::new(TextScrollWaitPlan::new()), // Scroll dex text 1
    Box::new(TextScrollWaitPlan::new()), // Scroll dex text 2
    Box::new(SkipTextsPlan::new(1)), // so you want Charmander
    Box::new(TextPlan::new()), // so you want Charmander?
    Box::new(TwoOptionMenuPlan::yes()), // choose Charmander
    Box::new(SkipTextsPlan::new(1)), // looks really energetic
    Box::new(SkipTextsPlan::new(1).with_skip_ends(3)), // received Charmander! Do you want...
    Box::new(TextPlan::new().with_skip_ends(2)), // give nickname?
    Box::new(TwoOptionMenuPlan::yes()), // give nickname
    Box::new(NamingScreenPlan::with_metric(b'C', Gen1DVMetric {}.filter(|v| {
      if v.hp() < 15 || v.atk < 6 || v.def < 10 || v.spc < 14 { return false; } // Charmander DVs
      log::info!("Charmander DVs: {:?}", v); true
    }).into_unit())), //  // DVs:  11, def: 15, spd: 9, spc: 15 #inputs: 5846
    Box::new(HoldTextDisplayOpenPlan::new()),
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // I'll take this one then
    Box::new(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan::new())), // rival received // squirtle // !
    Box::new(WalkToPlan::new(5, 6)), // trigger rival fight
    Box::new(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())), // Rival fight
    Box::new(OverworldWaitPlan::trainer_battle(225)), // Rival fight
    Box::new(StartTrainerBattlePlan::with_pre_battle_texts(0)), // Rival fight
    Box::new(SeqPlan::new(
      FightKOPlan::new(Move::Scratch, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::TailWhip))),
      EndTrainerBattlePlan::with_level_up(3))), // Rival1 fight
    Box::new(OverworldWaitPlan::new()), // advance map script (abSs buttons allowed)
    Box::new(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())), // after rival1 texts
    Box::new(WalkToPlan::new(5, 11)), // Leave lab
    Box::new(EdgeWarpPlan::new()), // go outside // inputs: 9543
    Box::new(WalkToPlan::new(10, -1)),
    Box::new(WalkToPlan::new(11, -1)), // Enter Viridian
    Box::new(WalkToPlan::new(29, 19)), // Enter Mart, starts cutscene
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // mart cutscene
    Box::new(OverworldWaitPlan::auto_walk(U)), // mart cutscene
    Box::new(OverworldWaitPlan::auto_walk(U)), // mart cutscene
    Box::new(OverworldWaitPlan::auto_walk(L)), // mart cutscene
    Box::new(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())), // mart cutscene
    Box::new(WalkToPlan::new(3, 6)),
    Box::new(WalkToPlan::new(3, 7)),
    Box::new(EdgeWarpPlan::new()), // go outside // inputs: 11802
    Box::new(WalkToPlan::new(29, 26)),
    Box::new(OverworldJumpLedgePlan::new(D)), // Jump ledge
    Box::new(WalkToPlan::new(21, 36)), // enter Route 1
    Box::new(WalkToPlan::new(14, 18)),
    Box::new(OverworldJumpLedgePlan::new(D)), // Jump ledge
    Box::new(WalkToPlan::new(12, 26)),
    Box::new(OverworldJumpLedgePlan::new(D)), // Jump ledge
    Box::new(WalkToPlan::new(11, 36)), // enter pallet town
    Box::new(WalkToPlan::new(12, 11)), // enter oak's lab // inputs: 13302
    Box::new(WalkToPlan::new(4, 2)), // next to oak
    Box::new(OverworldTurnPlan::new(R)), // turn towards Oak
    Box::new(OverworldInteractPlan::with(5)), // Talk to Oak
    Box::new(SeqPlan::new(SkipTextsPlan::new(10), HoldTextDisplayOpenPlan::new())), // Oak speech: special pokeball, thank you
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // Oak speech: Gramps
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // Oak speech: What came for?
    Box::new(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())), // Oak speech: Have something for you
    Box::new(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan::new())), // Oak speech: hi-tech encyclopedia
    Box::new(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan::new())), // Oak speech: Took Pokedex
    Box::new(SeqPlan::new(SkipTextsPlan::new(10), HoldTextDisplayOpenPlan::new())), // Oak speech: greatest undertaking
    Box::new(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan::new())), // Oak speech: leave it to me
    Box::new(WalkToPlan::new(4, 11)), // leave
    Box::new(EdgeWarpPlan::new()), // go outside // inputs: 15949
    Box::new(WalkToPlan::new(10, -1)), // enter Route 1
    Box::new(WalkToPlan::new(11, -1)), // Enter Viridian
    Box::new(WalkToPlan::new(18, -1)), // Enter Route 2
    Box::new(WalkToPlan::new(3, 43)), // Enter Viridian Forest
    Box::new(WalkToPlan::new(5, 0)), // Enter Viridian Forest
    Box::new(WalkToPlan::new(2, 19)),
    Box::new(OverworldInteractPlan::with(4)), // Bugcatcher
    Box::new(StartTrainerBattlePlan::with_pre_battle_texts(1)),
    Box::new(FightTurnPlan::new(AttackDesc::crit(Move::Scratch, 7..=7), EnemyAttackDesc::Attack(AttackDesc::crit_no_side_effect(Move::PoisonSting, 6..=6)), None)),
    Box::new(FightTurnPlan::new(AttackDesc::crit(Move::Scratch, 6..=7), EnemyAttackDesc::Attack(AttackDesc::crit_no_side_effect(Move::PoisonSting, 6..=6)), None)),
    Box::new(FightKOPlan::new(Move::Scratch, None, EnemyAttackDesc::Attack(AttackDesc::hit_no_side_effect(Move::PoisonSting, 4..=4)))),
    Box::new(EndTrainerBattlePlan::with_level_up(2)), // Bugcatcher fight // #inputs: 23643
  ])
}

fn yellow_plan() -> ListPlan<Yellow> {
  ListPlan::new(vec![
    Box::new(SkipIntroPlan::new()), // Logo
    Box::new(SkipYellowIntroPlan::new()), // Intro cutscene
    Box::new(SkipYellowTitlePlan::new()), // main menu
    Box::new(MainMenuPlan::new()), // main menu
    Box::new(SkipTextsPlan::new(13)), // oak speech
    Box::new(IntroNameMenuPlan::choose_custom_name()), // own name
    Box::new(NamingScreenPlan::with_letter(b'Y')),
    Box::new(SkipTextsPlan::new(5)), // oak speech
    Box::new(IntroNameMenuPlan::choose_custom_name()), // rival name
    Box::new(NamingScreenPlan::with_letter(b'U').with_fill_letters(6)),
    Box::new(SkipTextsPlan::new(7)), // oak speech
    Box::new(TextPlan::new()), // ... awaits let's go
    Box::new(WalkToPlan::new(7, 1)), // go down stairs
    Box::new(WalkToPlan::new(3, 6)), // go outside
    Box::new(WalkToPlan::new(3, 7)), // go outside
    Box::new(EdgeWarpPlan::new()), // go outside
    Box::new(WalkToPlan::new(10, 0)), // trigger oak cutscene
    Box::new(TextPlan::new()), // it's dangerous to go outside, take this
    Box::new(HoldTextDisplayOpenPlan::new()), // close text box
    Box::new(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())), // oak speech
    Box::new(OverworldWaitPlan::with_metric(OverworldInteractionMetric.filter(|r| if let OverworldInteractionResult::WildEncounter { species: Pokemon::Pikachu, ..} = r { true } else { false }).into_unit())),
    Box::new(SkipTextsPlan::new(1).with_skip_ends(2)), // Wild Pikachu appeared!
    Box::new(SkipTextsPlan::new(1).with_skip_ends(5)), // Oak used ball // Pikachu
    Box::new(SkipTextsPlan::new(1)), // caught.
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // oak speech
    Box::new(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan::new())), // oak speech
    Box::new(OverworldWaitPlan::new()), // Skip PalletTownScript load
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow Oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(L)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(R)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(R)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(R)), // Follow oak
    Box::new(OverworldWaitPlan::auto_walk(U)), // Follow oak
    Box::new(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())), // Fed up with waiting
    Box::new(SeqPlan::new(SkipTextsPlan::new(9), HoldTextDisplayOpenPlan::new())), // you can have one, choose
    Box::new(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())), // What about me?
    Box::new(SeqPlan::new(SkipTextsPlan::new(2), HoldTextDisplayOpenPlan::new())), // Can have one too
    Box::new(WalkToPlan::new(7, 4)), // stand before ball
    Box::new(OverworldTurnPlan::new(U)), // turn towards ball
    Box::new(OverworldInteractPlan::with(2)), // Interact with Ball
    Box::new(HoldTextDisplayOpenPlan::new()), // close text box
    Box::new(SkipTextsPlan::new(2)),
    Box::new(SkipTextsPlan::new(1).with_skip_ends(1)),
    Box::new(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan::new())), // Come here
    Box::new(OverworldWaitPlan::new()), // Skip Lab script
    Box::new(OverworldWaitPlan::auto_walk(L)), // Go to Oak
    Box::new(OverworldWaitPlan::auto_walk(D)), // Go to oak
    Box::new(OverworldWaitPlan::auto_walk(L)), // Go to oak
    Box::new(OverworldWaitPlan::auto_walk(L)), // Go to oak
    Box::new(OverworldWaitPlan::auto_walk(L)), // Go to oak
    Box::new(OverworldWaitPlan::auto_walk(U)), // Go to oak
    Box::new(OverworldWaitPlan::auto_walk(U)), // Go to oak
    Box::new(SkipTextsPlan::new(5)),
    Box::new(SkipTextsPlan::new(1).with_skip_ends(3)),
    Box::new(TextPlan::new().with_skip_ends(2)), // give nickname?
    Box::new(TwoOptionMenuPlan::no_with_metric(Gen1DVMetric {}.filter(|v| {
      if v.hp() > 4 { return false; } // Pikachu DVs, 18HP total
      log::info!("Pikachu DVs: {:?}", v); true
    }).into_unit())), // don't give nickname
    Box::new(HoldTextDisplayOpenPlan::new()), // close text box
    Box::new(WalkToPlan::new(5, 6)), // trigger rival fight
    Box::new(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())), // Rival fight
    Box::new(OverworldWaitPlan::trainer_battle(225)), // Rival fight
    Box::new(StartTrainerBattlePlan::with_pre_battle_texts(0)), // Rival fight
    Box::new(FightTurnPlan::new(AttackDesc::effect_failed(Move::Growl), EnemyAttackDesc::Attack(AttackDesc::crit(Move::Tackle, 9..=9)), None)),
    Box::new(FightTurnPlan::new(AttackDesc::effect_failed(Move::Growl), EnemyAttackDesc::Attack(AttackDesc::crit(Move::Tackle, 9..=10)), None)),
    Box::new(SkipTextsPlan::new(1).with_skip_ends(1)), // Pikachu fainted
    Box::new(SkipTextsPlan::new(1)), // Rival: I'm great
    Box::new(OverworldWaitPlan::new()), // advance map script (ab buttons allowed)
    Box::new(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())), // after rival1 texts
    Box::new(OverworldWaitPlan::new()), // advance map script (ab buttons allowed)
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // Oak text
    Box::new(OverworldWaitPlan::new()), // advance map script (ab buttons allowed)
    Box::new(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan::new())), // Oak text
    Box::new(WalkToPlan::new(5, 11).with_move_past_pikachu()), // Leave lab
    Box::new(EdgeWarpPlan::new()), // go outside // inputs: 10724
    Box::new(WalkToPlan::new(10, -1)),
    Box::new(WalkToPlan::new(11, -1)), // Enter Viridian
    Box::new(WalkToPlan::new(29, 19)), // Enter Mart, starts cutscene
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // mart cutscene
    Box::new(OverworldWaitPlan::auto_walk(U)), // mart cutscene
    Box::new(OverworldWaitPlan::auto_walk(U)), // mart cutscene
    Box::new(OverworldWaitPlan::auto_walk(L)), // mart cutscene
    Box::new(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())), // mart cutscene
    Box::new(WalkToPlan::new(3, 6)),
    Box::new(WalkToPlan::new(3, 7)),
    Box::new(EdgeWarpPlan::new()), // go outside // inputs: 12999
    Box::new(WalkToPlan::new(29, 26)),
    Box::new(OverworldJumpLedgePlan::new(D)), // Jump ledge
    Box::new(WalkToPlan::new(21, 36)), // enter Route 1
    Box::new(WalkToPlan::new(14, 18)),
    Box::new(OverworldJumpLedgePlan::new(D)), // Jump ledge
    Box::new(WalkToPlan::new(12, 26)),
    Box::new(OverworldJumpLedgePlan::new(D)), // Jump ledge
    Box::new(WalkToPlan::new(11, 36)), // enter pallet town
    Box::new(WalkToPlan::new(12, 11)), // enter oak's lab // inputs: 14517
    Box::new(WalkToPlan::new(5, 8)), // next to oak
    Box::new(WalkToPlan::new(5, 1)), // next to oak
    Box::new(OverworldTurnPlan::new(D)), // turn towards Oak
    Box::new(OverworldInteractPlan::with(3)), // Talk to Oak
    Box::new(SeqPlan::new(SkipTextsPlan::new(12), HoldTextDisplayOpenPlan::new())), // Oak speech: special pokeball, thank you
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // Oak speech: Gramps
    Box::new(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan::new())), // Oak speech: Gramps
    Box::new(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan::new())), // Oak speech: Have something for you
    Box::new(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan::new())), // Oak speech: hi-tech encyclopedia
    Box::new(SeqPlan::new(SkipTextsPlan::new(3), HoldTextDisplayOpenPlan::new())), // Oak speech: Took Pokedex
    Box::new(SeqPlan::new(SkipTextsPlan::new(10), HoldTextDisplayOpenPlan::new())), // Oak speech: greatest undertaking
    Box::new(SeqPlan::new(SkipTextsPlan::new(8), HoldTextDisplayOpenPlan::new())), // Oak speech: leave it to me
    Box::new(WalkToPlan::new(4, 11).with_move_past_pikachu()), // leave
    Box::new(EdgeWarpPlan::new()), // go outside // inputs: 17373
    Box::new(WalkToPlan::new(10, -1)), // enter Route 1
    Box::new(WalkToPlan::new(11, -1)), // Enter Viridian
    Box::new(WalkToPlan::new(29, 19)), // Enter Mart
    Box::new(OverworldOpenStartMenuPlan::new()), // Open start menu
    Box::new(StartMenuPlan::options_yellow()), // main menu
    Box::new(ChangeOptionsPlan::new().for_yellow()), // set options
    Box::new(StartMenuPlan::close()), // main menu
    Box::new(WalkToPlan::new(3, 5)),
    Box::new(WalkToPlan::new(2, 5)),
    Box::new(OverworldInteractPlan::with(1)), // Mart
    Box::new(TextPlan::new()), // How can I help you
    Box::new(BuySellQuitMenuPlan::buy()),
    Box::new(TextPlan::new()), // Take your time
    Box::new(SeqPlan::new(ListMenuPlan::choose(0), ChooseQuantityMenuPlan::new(3))), // Choose Pokeball x3
    Box::new(SkipTextsPlan::new(1).with_skip_ends(1)), // Item? // That will be
    Box::new(TextPlan::new().with_skip_ends(2)), // Price // Okay?
    Box::new(TwoOptionMenuPlan::yes()), // buy
    Box::new(SkipTextsPlan::new(1)), // Here you go
    Box::new(ListMenuPlan::quit()), // exit buy menu
    Box::new(TextPlan::new()), // Anything else?
    Box::new(BuySellQuitMenuPlan::quit()),
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan)), // Thank you! // inputs: 19588
    Box::new(WalkToPlan::new(3, 6)),
    Box::new(WalkToPlan::new(3, 7)),
    Box::new(EdgeWarpPlan::new()), // go outside
    Box::new(WalkToPlan::new(19, 9)),
    Box::new(SeqPlan::new(SkipTextsPlan::new(7), HoldTextDisplayOpenPlan)), // Catch tutorial
    Box::new(OverworldWaitPlan::new()), // advance map script (ab buttons allowed)
    Box::new(OverworldWaitPlan::with_metric(OverworldInteractionMetric.filter(|r| if let OverworldInteractionResult::WildEncounter { species: Pokemon::Rattata, ..} = r { true } else { false }).into_unit())),
    Box::new(SkipTextsPlan::new(1).with_skip_ends(2)), // Wild Rattata appeared!
    Box::new(TextPlan::new().with_skip_ends(2)),
    Box::new(SkipTextsPlan::new(1)), // Shoot! It was close too
    Box::new(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan)), // Catch tutorial // inputs: 21558
    Box::new(WalkToPlan::new(18, -1)), // Enter Route 2
    Box::new(WalkToPlan::new(4, 54)), // Catch Nidoran
    Box::new(WalkToEncounterPlan::new(8, 46, OverworldInteractionMetric.filter(|v| {
      if let OverworldInteractionResult::WildEncounter { species: Pokemon::NidoranM, level: 6, dvs } = v {
        // log::info!("Nidoran"),
        if dvs.hp() > 9 || dvs.atk < 15 || dvs.def > 9 || dvs.spd < 13 || dvs.spc < 13 { return false; }
        log::info!("Chosen Nidoran DVs: {:?}", dvs);
        true
      } else { false }
    }).into_unit())),
    // Box::new(SkipTextsPlan::new(1).with_skip_ends(2)), // Wild Nidoran appeared!
    // Box::new(TextPlan::new().with_skip_ends(2)), // Go, Pikachu!
    // r.save("multi_yellow_test"),
    // r.load("multi_yellow_test"),
    // Box::new(BattleMenuPlan::items()),
    // Box::new(ListMenuPlan::choose(0)), // Poke Ball
    // Box::new(TextPlan::with_metric(CatchSuccessMetric, false).with_skip_ends(2)),
    // Box::new(SkipTextsPlan::new(1).with_skip_ends(2)), // Nidoran was
    // Box::new(SkipTextsPlan::new(1)), // caught!
    // Box::new(SkipTextsPlan::new(1)), // New dex entry
    // Box::new(SkipTextsPlan::new(1).with_skip_ends(1)), // Nidoran!
    // Box::new(TextScrollWaitPlan::new()), // dex entry
    // Box::new(TextScrollWaitPlan::new()), // Scroll dex text 2
    // Box::new(SkipTextsPlan::new(1)), // Give nickname?
    // Box::new(TextPlan::new().with_skip_ends(2)), // to Nidoran?
    // Box::new(TwoOptionMenuPlan::yes()),
    // Box::new(NamingScreenPlan::with_letter(b'A')),
    // r.save("multi_yellow_nidoran"),
    // r.load("multi_yellow_nidoran"),
    // Box::new(WalkToPlan::new(3, 43)), // Enter Viridian Forest
    // Box::new(WalkToPlan::new(5, 0)), // Enter Viridian Forest
  ])
}
