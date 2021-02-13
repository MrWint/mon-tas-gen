#[allow(unused_imports)] use gambatte::inputs::*;
#[allow(unused_imports)] use montas::constants::*;
#[allow(unused_imports)] use montas::metric::*;
#[allow(unused_imports)] use montas::metric::battle::*;
#[allow(unused_imports)] use montas::metric::battle::gen1::*;
use montas::multi::*;
use montas::rom::*;
use montas::sdl::*;

const EQUAL_LENGTH_FRAMES: bool = false;
const RTC_DIVISOR_OFFSET: i32 = 0;

#[allow(dead_code)]
pub fn start() {
  log::set_max_level(log::LevelFilter::Debug);

  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let red_gb = Gb::<Red>::create(EQUAL_LENGTH_FRAMES, RTC_DIVISOR_OFFSET, SdlScreen::new(sdl.clone(), 0));
  let mut r = SingleGbRunner::new(red_gb);

  // r.run(SkipIntroPlan::new().with_auto_pass_after(214)); // Logo
  // r.run(SkipIntroPlan::new().with_auto_pass_after(322)); // Intro cutscene
  // r.run(SkipIntroPlan::new().with_no_up_select_b()); // main menu
  // r.run(MainMenuPlan::new()); // main menu
  // r.run(SkipTextsPlan::new(13)); // oak speech
  // r.run(IntroNameMenuPlan::choose_custom_name()); // own name
  // r.run(NamingScreenPlan::with_letter(b'R'));
  // r.run(SkipTextsPlan::new(5)); // oak speech
  // r.run(IntroNameMenuPlan::choose_custom_name()); // rival name
  // r.run(NamingScreenPlan::with_letter(b'U'));
  // r.run(SkipTextsPlan::new(7)); // oak speech
  // r.run(TextPlan::new()); // ... awaits let's go
  // r.save("multi_red_intro");
  // r.load("multi_red_intro");
  // r.run(OverworldOpenStartMenuPlan::new()); // Open start menu
  // r.run(StartMenuPlan::options()); // main menu
  // r.run(ChangeOptionsPlan::new()); // set options
  // r.run(StartMenuPlan::close()); // main menu
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
  // r.save("multi_red_before_starter");
  // r.load("multi_red_before_starter");
  // r.run(OverworldTurnPlan::new(R)); // turn towards Charmander
  // r.run(OverworldInteractPlan::with(2)); // Interact with Charmander Ball
  // r.run(TextScrollWaitPlan::new()); // Scroll dex text 1
  // r.run(TextScrollWaitPlan::new()); // Scroll dex text 2
  // r.run(SkipTextsPlan::new(1)); // so you want Charmander
  // r.run(TextPlan::new()); // so you want Charmander?
  // r.run(TwoOptionMenuPlan::yes()); // choose Charmander
  // r.run(SkipTextsPlan::new(1)); // looks really energetic
  // r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // received Charmander! Do you want...
  // r.run(TextPlan::new().with_skip_ends(2)); // give nickname?
  // r.run(TwoOptionMenuPlan::yes()); // give nickname
  // r.run(NamingScreenPlan::with_metric(b'A', Gen1DVMetric {}.filter(|v| {
  //     // if v.atk < 15 || v.def < 11 || v.spc < 12 || v.spd < 7 || v.def & 1 == 0 || (v.spd & 1 == 0 && v.spc & 1 == 0) { return false; } // totodile
  //     if v.atk < 15 || v.spc < 15 || v.spd < 15 { return false; } // Charmander DVs
  //     log::info!("Chosen DVs: {:?}", v); true
  //   }).into_unit()));
  // r.run(HoldTextDisplayOpenPlan::new());
  // r.save("multi_red_after_starter"); // DVs: 15 / 10 / 15 / 15
  // r.load("multi_red_after_starter");
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())); // I'll take this one then
  // r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan::new())); // rival received // squirtle // !
  // r.run(WalkToPlan::new(5, 6)); // trigger rival fight
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())); // Rival fight
  // r.run(OverworldWaitPlan::trainer_battle(225)); // Rival fight
  // r.run(StartTrainerBattlePlan::with_pre_battle_texts(0)); // Rival fight
  // r.run(FightKOPlan::new(Move::Scratch, None, EnemyAttackDesc::Attack(AttackDesc::effect_failed(Move::TailWhip))));
  // r.save("multi_red_after_rival1"); // 8558
  // r.load("multi_red_after_rival1");
  // r.run(EndTrainerBattlePlan::with_level_up(3)); // Rival1 fight
  // r.run(OverworldWaitPlan::new()); // advance map script (abSs buttons allowed)
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())); // after rival1 texts
  // r.run(WalkToPlan::new(5, 11)); // Leave lab
  // r.run(EdgeWarpPlan::new()); // go outside // inputs: 9551
  // r.run(WalkToPlan::new(10, -1));
  // r.run(WalkToPlan::new(11, -1)); // Enter Viridian
  // r.run(WalkToPlan::new(29, 19)); // Enter Mart, starts cutscene
  // r.save("multi_red_test");
  // r.load("multi_red_test");
  // r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())); // mart cutscene
  // r.run(OverworldWaitPlan::auto_walk(U)); // mart cutscene
  // r.run(OverworldWaitPlan::auto_walk(U)); // mart cutscene
  // r.run(OverworldWaitPlan::auto_walk(L)); // mart cutscene
  // r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())); // mart cutscene
  // r.run(WalkToPlan::new(3, 6));
  // r.run(WalkToPlan::new(3, 7));
  // r.run(EdgeWarpPlan::new()); // go outside // inputs: 11802
  // r.run(WalkToPlan::new(29, 26));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(21, 36)); // enter Route 1
  // r.run(WalkToPlan::new(14, 18));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(12, 26));
  // r.run(OverworldJumpLedgePlan::new(D)); // Jump ledge
  // r.run(WalkToPlan::new(11, 36)); // enter pallet town
  // r.run(WalkToPlan::new(12, 11)); // enter oak's lab // inputs: 13302
  // r.save("multi_red_test2");
  // r.load("multi_red_test2");
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
  // r.run(EdgeWarpPlan::new()); // go outside // inputs: 15949
  // r.save("multi_red_after_oak_parcel");
  r.load("multi_red_after_oak_parcel");
  r.run(WalkToPlan::new(10, -1)); // enter Route 1
  r.run(WalkToPlan::new(11, -1)); // Enter Viridian
  r.run(WalkToPlan::new(18, -1)); // Enter Route 2
  r.run(WalkToPlan::new(3, 43)); // Enter Viridian Forest
  r.run(WalkToPlan::new(5, 0)); // Enter Viridian Forest
  r.save("multi_red_test");
  // r.load("multi_red_test");


  // r.save("multi_red_test");
  // r.load("multi_red_test");

  // r.debug_print_state_fn(MoveInfosFn::new(Who::Player));
  // r.debug_print_state_fn(BattleMonInfoFn::new(Who::Player));
  // r.debug_print_state_fn(MoveInfosFn::new(Who::Enemy));
  // r.debug_print_state_fn(BattleMonInfoFn::new(Who::Enemy));

  r.debug_segment_end("temp/multi_red");
}
