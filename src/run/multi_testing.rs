use montas::multi::*;
use montas::rom::*;
use montas::sdl::*;

const EQUAL_LENGTH_FRAMES: bool = false;
const RTC_DIVISOR_OFFSET: i32 = 0;

#[allow(dead_code)]
pub fn start() {
  log::set_max_level(log::LevelFilter::Debug);

  let sdl = Sdl::init_sdl(1 /* num screens */, 1 /* scale */);
  let blue_gb = Gb::<Blue>::create(EQUAL_LENGTH_FRAMES, RTC_DIVISOR_OFFSET, SdlScreen::new(sdl, 0));
  let blue_executor = MultiGbExecutor::new(blue_gb, blue_plan());
  let mut r = MultiGbRunner::new(vec![Box::new(blue_executor)]);

  std::thread::sleep(std::time::Duration::from_millis(1000));
  r.step();
  std::thread::sleep(std::time::Duration::from_millis(1000));
  r.step();
  std::thread::sleep(std::time::Duration::from_millis(1000));
  r.step();
  std::thread::sleep(std::time::Duration::from_millis(1000));
  r.step();
  std::thread::sleep(std::time::Duration::from_millis(1000));
  r.step();
  std::thread::sleep(std::time::Duration::from_millis(1000));
  r.step();
  std::thread::sleep(std::time::Duration::from_millis(1000));
}

fn blue_plan() -> ListPlan<Blue> {
  ListPlan::new(vec![
    Box::new(SkipIntroPlan::new().with_auto_pass_after(214)), // Logo
    Box::new(SkipIntroPlan::new().with_auto_pass_after(322)), // Intro cutscene
    Box::new(SkipIntroPlan::new().with_no_up_select_b()), // main menu
    Box::new(IdentifyInputPlan),
  ])
}
