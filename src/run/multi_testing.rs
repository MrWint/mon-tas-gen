use gambatte::inputs::*;
use montas::metric::*;
use montas::multi::*;
use montas::rom::*;
use montas::sdl::*;

const EQUAL_LENGTH_FRAMES: bool = false;
const RTC_DIVISOR_OFFSET: i32 = 0;

#[allow(dead_code)]
pub fn start() {
  log::set_max_level(log::LevelFilter::Debug);

  let sdl = Sdl::init_sdl(2 /* num screens */, 1 /* scale */);
  let blue_gb = Gb::<Blue>::create(EQUAL_LENGTH_FRAMES, RTC_DIVISOR_OFFSET, SdlScreen::new(sdl.clone(), 0));
  let _blue_executor = MultiGbExecutor::new(blue_gb, blue_plan());
  let red_gb = Gb::<Red>::create(EQUAL_LENGTH_FRAMES, RTC_DIVISOR_OFFSET, SdlScreen::new(sdl, 1));
  let _red_executor = MultiGbExecutor::new(red_gb, red_plan());
  let mut r = MultiGbRunner::new([
    Box::new(_blue_executor),
    Box::new(_red_executor),
  ]);

  // r.load("multi_test2");
  r.run();
  r.save("multi_test");
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
    Box::new(NamingScreenPlan::with_letter(b'I')),
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
    Box::new(NamingScreenPlan::with_metric(b'A', Gen1DVMetric {}.filter(|v| {
        // if v.atk < 15 || v.def < 11 || v.spc < 12 || v.spd < 7 || v.def & 1 == 0 || (v.spd & 1 == 0 && v.spc & 1 == 0) { return false, } // totodile
        // if v.atk < 15 || v.spc < 15 || v.spd < 15 { return false; } // squirtle DVs
        log::info!("Chosen DVs: {:?}", v); true
      }).into_unit())),
    Box::new(HoldTextDisplayOpenPlan::new()),
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // I'll take this one then
    Box::new(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan::new())), // rival received // bulbasaur // !
    Box::new(WalkToPlan::new(5, 6)), // trigger rival fight
    Box::new(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())), // Rival fight
    Box::new(OverworldWaitPlan::trainer_battle(225)), // Rival fight
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
    Box::new(NamingScreenPlan::with_letter(b'I')),
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
    Box::new(NamingScreenPlan::with_metric(b'A', Gen1DVMetric {}.filter(|v| {
        // if v.atk < 15 || v.def < 11 || v.spc < 12 || v.spd < 7 || v.def & 1 == 0 || (v.spd & 1 == 0 && v.spc & 1 == 0) { return false, } // totodile
        // if v.atk < 15 || v.spc < 15 || v.spd < 15 { return false; } // squirtle DVs
        log::info!("Chosen DVs: {:?}", v); true
      }).into_unit())),
    Box::new(HoldTextDisplayOpenPlan::new()),
    Box::new(SeqPlan::new(SkipTextsPlan::new(1), HoldTextDisplayOpenPlan::new())), // I'll take this one then
    Box::new(SeqPlan::new(SkipTextsPlan::new(1).with_skip_ends(2), HoldTextDisplayOpenPlan::new())), // rival received // bulbasaur // !
    Box::new(WalkToPlan::new(5, 6)), // trigger rival fight
    Box::new(SeqPlan::new(SkipTextsPlan::new(4), HoldTextDisplayOpenPlan::new())), // Rival fight
    Box::new(OverworldWaitPlan::trainer_battle(225)), // Rival fight
  ])
}
