#[allow(unused_imports)] use gambatte::inputs::*;
use montas::multi::*;
use montas::rom::*;
use montas::sdl::*;

const EQUAL_LENGTH_FRAMES: bool = false;
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
  // r.run(NamingScreenPlan::with_letter(b'I'));
  // r.run(SkipTextsPlan::new(5)); // oak speech
  // r.run(IntroNameMenuPlan::choose_custom_name()); // rival name
  // r.run(NamingScreenPlan::with_letter(b'U'));
  // r.run(SkipTextsPlan::new(7)); // oak speech
  // r.run(TextPlan::new()); // ... awaits let's go
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(OverworldOpenStartMenuPlan::new()); // Open start menu
  // r.run(StartMenuPlan::options()); // main menu
  // r.run(ChangeOptionsPlan::new()); // set options
  // r.run(StartMenuPlan::close()); // main menu
  // r.run(WalkToPlan::new(7, 1)); // go down stairs
  // r.run(WalkToPlan::new(3, 6)); // go outside
  // r.run(WalkToPlan::new(3, 7)); // go outside
  // r.run(EdgeWarpPlan::new()); // go outside
  // r.save("multi_blue_test2");
  // r.load("multi_blue_test2");
  // r.run(WalkToPlan::new(10, 1)); // trigger oak cutscene
  // r.run(OverworldWaitPlan::new()); // Skip PalletTownScript0
  // r.run(TextPlan::new()); // it's dangerous to go outside, take this
  // r.run(HoldTextDisplayOpenPlan::new()); // close text box
  // r.run(SeqPlan::new(SkipTextsPlan::new(6), HoldTextDisplayOpenPlan::new())); // oak speech
  // r.run(OverworldWaitPlan::new()); // Skip PalletTownScript load
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
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
  // r.save("multi_blue_test");
  // r.load("multi_blue_test");
  // r.run(SkipTextsPlan::new(1).with_skip_ends(3)); // received Squirtle! Do you want...
  // r.run(TextPlan::new().with_skip_ends(2)); // give nickname?
  // r.run(TwoOptionMenuPlan::yes()); // give nickname
  // r.run(NamingScreenPlan::with_metric(b'A', Gen1DVMetric {}.filter(|v| {
  //     // if v.atk < 15 || v.def < 11 || v.spc < 12 || v.spd < 7 || v.def & 1 == 0 || (v.spd & 1 == 0 && v.spc & 1 == 0) { return false; } // totodile
  //     if v.atk < 15 || v.spc < 15 || v.spd < 15 { return false; } // squirtle DVs
  //     log::info!("Chosen DVs: {:?}", v); true
  //   }).into_unit()));
  // r.run(HoldTextDisplayOpenPlan::new());
  // r.save("multi_blue_test2"); // DVs: 15 / 4 / 15 / 15
  r.load("multi_blue_test2");
  r.run(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())); // I'll take this one then
  r.run(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan::new())); // rival received // bulbasaur // !
  r.run(WalkToPlan::new(5, 6)); // trigger rival fight
  r.run(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())); // Rival fight
  r.run(OverworldWaitPlan::trainer_battle(225)); // Rival fight

  r.debug_segment_end("temp/multi_testing");
}
