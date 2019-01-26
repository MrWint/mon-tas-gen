mod bk2;

use crate::bk2::{Bk2Writer, read_bk2_inputs};
use gambatte::*;
use gambatte::inputs::*;
use montas::ftii;
use montas::gb::*;
#[allow(unused_imports)] use montas::gbexecutor::*;
use montas::rom::*;
use montas::segment::*;
#[allow(unused_imports)] use montas::segment::overworld::gen2;
#[allow(unused_imports)] use montas::util::*;
use montas::sdl::*;
use montas::statebuffer::StateBuffer;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

fn main() {
  // test_gambattesdl();
  // test_gambattedll();
  // if true {return;}

  // let mut gbe = SingleGb::<Crystal>::with_screen();
  // // let mut gbe = GbPool::<Crystal>::no_screen();
  // let s = gbe.get_initial_state();
  // let sb = MoveSegment::new(A).with_max_skips(10).execute_split(&mut gbe, vec![s]).into_state_buffer();
  // let sb = MoveSegment::new(START).with_max_skips(10).execute_split(&mut gbe, sb).into_state_buffer();
  // let sb = MoveSegment::new(D).with_max_skips(10).execute_split(&mut gbe, sb).into_state_buffer();
  // let sb = MoveSegment::new(L|A).with_max_skips(10).execute_split(&mut gbe, sb).into_state_buffer();
  // let sb = MoveSegment::new(B).with_max_skips(10).execute_split(&mut gbe, sb).into_state_buffer();
  // let _sb = MoveSegment::new(A).with_max_skips(10).execute_split(&mut gbe, sb).into_state_buffer();
  // // let sb = gbe.execute_text_segment(sb, 1, A); // choose Boy
  // // let sb = gbe.execute_text_segment(sb, 3, B);
  // // let _sb = gbe.execute_text_segment(sb, 4, A); // time: 10:..
  // if true {return;}


  playback_demos();
  // convert_efl();
  // if true {return;}

  // BlueTestSegment::run();
  // YellowTestSegment::run();
  // CrystalTestSegment::run();
}

#[allow(dead_code)]
fn test_gambatte() {
  let sdl = Sdl::init_sdl(2, 3);
  let mut gambatte1 = Gambatte::create("roms/gbc_bios.bin", "roms/yellow.gbc", false, SdlScreen::new(sdl.clone(), 0));
  let mut gambatte2 = Gambatte::create("roms/gbc_bios.bin", "roms/crystal.gbc", false, SdlScreen::new(sdl, 1));
  for _ in 0..5000 {
    gambatte1.step();
    gambatte2.step();
    sleep(Duration::from_millis(0));
  }
  let state1 = gambatte1.save_state();
  // let state2 = gambatte2.save_state();
  gambatte1.load_state(&state1);
  // gambatte2.load_state(&state2);
  for _ in 0..5000 {
    gambatte1.step();
    gambatte2.step();
    sleep(Duration::from_millis(0));
  }
}

fn test_segment_end<R: Rom, E: GbExecutor<R>>(gbe: &mut E, sb: &StateBuffer, file_name: &str) {
  {
    println!("Creating sample input file...");
    let inputs = gbe.execute(sb, move |gb, s, tx| {
      gb.restore(&s);
      tx.send((gb.create_inputs(), s)).unwrap();
    }).into_map_iter().map(|(i, _)| i).min_by_key(Vec::len).unwrap();
    Bk2Writer::new::<R>().write_bk2(&format!("{}.bk2", file_name), &inputs).unwrap();
  }
  println!("Rendering end states of {}", sb);
  gbe.execute(sb, move |gb, s, tx| {
    gb.restore(&s);
    for _ in 0..10000 {
      gb.input(Input::empty());
      gb.step();
    }
    std::thread::sleep(std::time::Duration::from_millis(100));
    tx.send(((), s)).unwrap();
  }).into_state_buffer_map(0);
}

pub struct BlueTestSegment {}
impl BlueTestSegment {
  #[allow(dead_code)]
  fn run() {
    // let mut gbe = SingleGb::<Blue>::with_screen();
    let mut gbe = GbPool::<Blue>::with_screen();
    let states = vec![gbe.get_initial_state()];
    let _sb = BlueTestSegment{}.execute(&mut gbe, states);
    test_segment_end(&mut gbe, &_sb, "temp/blue_test");
  }
}
impl Segment<Blue> for BlueTestSegment {
  type Key = ();

  fn execute_split<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<Blue>>(&self, gbe: &mut E, _iter: I) -> HashMap<Self::Key, StateBuffer> {
//     let sb = DelaySegment::new(MoveSegment::with_metric_fn(START, |_gb| Some(()))).execute(gbe, _iter);
//     println!("{}", sb);
//     let sb = MoveSegment::new(A).with_max_skips(10).execute(gbe, sb);
//     println!("{}", sb);
//     let sb = MoveSegment::new(START).with_max_skips(10).execute(gbe, sb);
//     println!("{}", sb);
// //    let sb = MoveSegment::new(A).with_max_skips(10).execute(gbe, sb);
//     let sb = MoveSegment::new(D|A).execute(gbe, sb); // options
//     let sb = MoveSegment::new(L).execute(gbe, sb); // text speed fast
//     let sb = MoveSegment::new(D).execute(gbe, sb); // battle animations...
//     let sb = MoveSegment::new(L).execute(gbe, sb); // ...off
//     let sb = MoveSegment::new(D).execute(gbe, sb); // battle style...
//     let sb = MoveSegment::new(L).execute(gbe, sb); // ...set
//     let sb = MoveSegment::new(B).execute(gbe, sb); // back
//     let sb = MoveSegment::new(A).execute(gbe, sb); // new game
//     println!("{}", sb);
//     let sb = SkipTextsSegment::new(13, B).execute(gbe, sb); // skip texts until player name
//     let sb = MoveSegment::new(D|A).execute(gbe, sb); // Name: Blue
//     let sb = SkipTextsSegment::new(5, B).execute(gbe, sb); // skip texts until rival name
//     let sb = MoveSegment::new(D|A).execute(gbe, sb); // Name: Red
//     let sb = SkipTextsSegment::new(7, B).execute(gbe, sb); // skip texts until game start
//     let sb = TextSegment::new(A).execute(gbe, sb); // ...awaits, let's go
//     println!("{}", sb);
//     sb.save("blue_test");
    let sb = StateBuffer::load("blue_test");
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);

    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    let sb = IdentifyInputSegment::new().execute(gbe, sb);
    // let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);

    // let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);

    // let sb = MoveSegment::with_metric(A, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // sb.save("blue_test");
    // let sb = StateBuffer::load("blue_test");
    // let sb = SkipTextsSegment::new(6, B).execute(gbe, sb); // it's dangerous to go outside, take this
    // let sb = TextSegment::new(A).expect_conflicting_inputs().execute(gbe, sb); // come with me
    // let sb = MoveSegment::new(B).execute(gbe, sb); // come with me
    // let sb = IdentifyInputSegment::new().execute(gbe, sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
    // let sb = IdentifyInputSegment::new().execute(gbe, sb);
    sleep(Duration::from_millis(1000));

    let mut result = HashMap::new();
    result.insert((), sb);
    result
  }
}

pub struct YellowTestSegment {}
impl YellowTestSegment {
  #[allow(dead_code)]
  fn run() {
    // let mut gbe = SingleGb::<Yellow>::with_screen();
    let mut gbe = GbPool::<Yellow>::with_screen();
    let states = vec![gbe.get_initial_state()];
    let _sb = YellowTestSegment{}.execute(&mut gbe, states);
    test_segment_end(&mut gbe, &_sb, "temp/yellow_test");
  }
}
impl Segment<Yellow> for YellowTestSegment {
  type Key = ();

  fn execute_split<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<Yellow>>(&self, gbe: &mut E, _iter: I) -> HashMap<Self::Key, StateBuffer> {
  //   let sb = MoveSegment::new(START).with_max_skips(15).with_buffer_size(4096).execute(gbe, _iter);
  //   println!("{}", sb);
  //   let sb = MoveSegment::new(A).with_max_skips(15).with_buffer_size(4096).execute(gbe, sb);
  //   println!("{}", sb);
  //   let sb = MoveSegment::new(START).with_max_skips(15).with_buffer_size(4096).execute(gbe, sb);
  //   println!("{}", sb);
  //   let sb = DelaySegment::new(MoveSegment::with_metric(A, TrainerIDMetric{}.filter(|v| { v == &0x26F1 }).into_unit())).with_debug_output(true).execute(gbe, sb); // new game
  //   sb.save("yellow_after_tid");
  //   let sb = StateBuffer::load("yellow_after_tid");
  //   println!("{}", sb);
  //   let sb = SkipTextsSegment::new(13, B).execute(gbe, sb); // skip texts until player name
  //   let sb = MoveSegment::new(D).execute(gbe, sb); // Name: Yellow
  //   let sb = MoveSegment::new(A).execute(gbe, sb); // Name: Yellow
  //   let sb = SkipTextsSegment::new(5, B).execute(gbe, sb); // skip texts until rival name
  //   let sb = MoveSegment::new(D).execute(gbe, sb); // Name: Blue
  //   let sb = MoveSegment::new(A).execute(gbe, sb); // Name: Blue
  //   let sb = SkipTextsSegment::new(7, B).execute(gbe, sb); // skip texts until game start
  //   let sb = TextSegment::new(A).execute(gbe, sb); // ...awaits, let's go
  //   println!("{}", sb);
  //   let sb = MoveSegment::new(START).execute(gbe, sb);
  //   let sb = MoveSegment::new(U).execute(gbe, sb);
  //   let sb = MoveSegment::new(NIL).execute(gbe, sb);
  //   let sb = MoveSegment::new(U).execute(gbe, sb);
  //   let sb = MoveSegment::new(L|A).execute(gbe, sb);
  //   let sb = MoveSegment::new(B).execute(gbe, sb);
  //   let sb = MoveSegment::new(START).execute(gbe, sb);
  //   sb.save("yellow_after_intro_256_div");
  //   let sb = StateBuffer::load("yellow_after_intro_256_div");
  //   let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  // let sb = MoveSegment::new(NIL).execute(gbe, sb);
  //   let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);

  //   let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(L, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);

  //   let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   sb.save("yellow_grass_256_wait1_");
  //   let sb = StateBuffer::load("yellow_grass_256_wait1_");
  //   let sb = SkipTextsSegment::new(3, B).execute(gbe, sb); // oak in grass
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = SkipTextsSegment::new(3, B).execute(gbe, sb); // oak vs pikachu, catch pikachu
  //   let sb = SkipTextsSegment::new(7, B).execute(gbe, sb); // oak in grass
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = SkipTextsSegment::new(15, B).execute(gbe, sb); // oak speech
  //   let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(D, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(R, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(U, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(A, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   sb.save("yellow_before_collect_pikachu_256_wait1_");
  //   let sb = StateBuffer::load("yellow_before_collect_pikachu_256_wait1_");
  //   let sb = MoveSegment::new(NIL).execute(gbe, sb); // Release A button to start sequence
  //   let sb = SkipTextsSegment::new(10, B).execute(gbe, sb); // Rival snatches
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = MoveSegment::with_metric(NIL, FnMetric::new(|gb| {println!("{:?}", overworld::gen1::get_overworld_interaction_result(gb)); Some(())})).execute(gbe, sb); println!("{}", sb);
  //   let sb = SkipTextsSegment::new(4, B).execute(gbe, sb); // get Pikachu
  //   sb.save("yellow_before_collect_pikachu_2_256_wait1_");
    let sb = StateBuffer::load("yellow_before_collect_pikachu_2_256_wait1_");
    let sb = SkipTextsSegment::new(2, B).with_buffer_size(4096).execute(gbe, sb); // get Pikachu
    let sb = TextSegment::new(A).with_buffer_size(4096).execute(gbe, sb); // Nickname?
    let sb = DelaySegment::new(MoveSegment::with_metric(B, Gen1DVMetric {}.filter(|v| {
      let dvs = u16::from(v.atk) << 12 | u16::from(v.def) << 8 | u16::from(v.spd) << 4 | u16::from(v.spc);
      let div = ((dvs >> 8) as u8).wrapping_sub(dvs as u8);
      if ::rand::random::<u8>() < 0x04 {
        print!("{:X}", div);
        // print!("/{:X}", v.div_state >> 6);
        // print!("/{}", to_human_readable_time(v.cycle_count));
        print!(" ");
      }
      if v.atk != 7 { return false; }
      if v.spc != 8 && v.spc != 9 { return false; }
      if v.def != 0 && v.def != 1 { return false; }
      if v.spd != 12 && v.spd != 13 { return false; }
      println!("Chosen DVs: {:?}", v); true
    }).into_unit())).with_debug_output(true).execute(gbe, sb);// println!("{}", sb);
    sb.save("yellow_after_collect_pikachu_256_div");
    let sb = StateBuffer::load("yellow_after_collect_pikachu_256_div");
    let sb = IdentifyInputSegment::new().execute(gbe, sb);
    sleep(Duration::from_millis(1000));

    let mut result = HashMap::new();
    result.insert((), sb);
    result
  }
}

pub struct SilverTestSegment {}
impl SilverTestSegment {
  #[allow(dead_code)]
  fn run() {
    // let mut gbe = SingleGb::<Silver>::with_screen();
    let mut gbe = GbPool::<Silver>::with_screen();
    let states = vec![gbe.get_initial_state()];
    let _sb = SilverTestSegment{}.execute(&mut gbe, states);
    test_segment_end(&mut gbe, &_sb, "temp/silver_test");
  }
}
impl Segment<Silver> for SilverTestSegment {
  type Key = ();

  fn execute_split<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<Silver>>(&self, gbe: &mut E, _iter: I) -> HashMap<Self::Key, StateBuffer> {
    let sb = DelaySegment::new(MoveSegment::with_metric_fn(A, |_gb| Some(()))).execute(gbe, _iter);
    println!("{}", sb);
    let sb = MoveSegment::new(START).with_max_skips(10).execute(gbe, sb);
    println!("{}", sb);
    let sb = MoveSegment::new(D).execute(gbe, sb); // options
    let sb = MoveSegment::new(L|A).execute(gbe, sb); // fast options
    let sb = MoveSegment::new(B).execute(gbe, sb); // back
    let sb = MoveSegment::new(A).execute(gbe, sb); // new game
    println!("{}", sb);
    let sb = SkipTextsSegment::new(3, B).execute(gbe, sb);
    let sb = SkipTextsSegment::new(4, A).execute(gbe, sb); // time: 10:..
    let sb = TextSegment::new(A).execute(gbe, sb); // overslept
    let sb = MoveSegment::new(B).execute(gbe, sb); // overslept
    let sb = SkipTextsSegment::new(17, B).execute(gbe, sb); // oak speech
    let sb = MoveSegment::new(D).execute(gbe, sb); // Name: Silver
    let sb = MoveSegment::new(A).execute(gbe, sb); // Name: Silver
    let sb = SkipTextsSegment::new(7, B).execute(gbe, sb); // skip texts until game start
    let sb = TextSegment::new(A).execute(gbe, sb); // ... seeing you later
    let sb = IdentifyInputSegment::new().execute(gbe, sb);
    println!("{}", sb);
    sleep(Duration::from_millis(1000));

    let mut result = HashMap::new();
    result.insert((), sb);
    result
  }
}

pub struct CrystalTestSegment {}
impl CrystalTestSegment {
  #[allow(dead_code)]
  fn run() {
    // let mut gbe = SingleGb::<Crystal>::with_screen();
    let mut gbe = GbPool::<Crystal>::with_screen();
    let states = vec![gbe.get_initial_state()];
    let _sb = CrystalTestSegment{}.execute(&mut gbe, states);
    test_segment_end(&mut gbe, &_sb, "temp/crystal_test");
  }
}
impl Segment<Crystal> for CrystalTestSegment {
  type Key = ();

  fn execute_split<S: StateRef, I: IntoIterator<Item=S>, E: GbExecutor<Crystal>>(&self, gbe: &mut E, _iter: I) -> HashMap<Self::Key, StateBuffer> {
    let sb = DelaySegment::new(MoveSegment::with_metric(A, NullMetric {})).with_debug_output(true).execute(gbe, _iter);
    println!("{}", sb);
    let sb = MoveSegment::new(START).with_max_skips(10).execute(gbe, sb);
    println!("{}", sb);
    let sb = MoveSegment::new(D).execute(gbe, sb); // options
    let sb = MoveSegment::new(L|A).execute(gbe, sb); // fast options
    let sb = MoveSegment::new(B).execute(gbe, sb); // back
    let sb = MoveSegment::new(A).execute(gbe, sb); // new game
    println!("{}", sb);
    let sb = SkipTextsSegment::new(1, A).execute(gbe, sb); // choose Boy
    let sb = SkipTextsSegment::new(3, B).execute(gbe, sb);
    let sb = SkipTextsSegment::new(4, A).execute(gbe, sb); // time: 10:..
    let sb = TextSegment::new(A).expect_conflicting_inputs().execute(gbe, sb); // overslept
    let sb = MoveSegment::new(B).execute(gbe, sb); // overslept
    let sb = SkipTextsSegment::new(17, B).execute(gbe, sb); // oak speech
    let sb = MoveSegment::new(D).execute(gbe, sb); // Name: Chris
    let sb = MoveSegment::new(A).execute(gbe, sb); // Name: Chris
    let sb = SkipTextsSegment::new(7, B).execute(gbe, sb); // skip texts until game start
    let sb = TextSegment::new(A).execute(gbe, sb); // ... seeing you later
    println!("{}", sb);
    sb.save("crystal_test");
    let sb = StateBuffer::load("crystal_test");
    let sb = gen2::TurnSegment::new(R).execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = gen2::WalkToSegment::new(7, 0).into(gen2::OverworldInteractionResult::Warped).with_debug_output(true).execute(gbe, sb);
    let sb = gen2::WarpSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = gen2::WalkToSegment::new(7, 2).with_debug_output(true).execute(gbe, sb);
    let sb = gen2::WalkStepSegment::new(D).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::with_metric(A, gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); if let gen2::OverworldInteractionResult::Interact(_) = v { true } else { false }}).into_unit()).execute(gbe, sb); println!("{}", sb);
    let sb = SkipTextsSegment::new(12, B).execute(gbe, sb); // mom speech
    let sb = SkipTextsSegment::new(2, A).execute(gbe, sb); // choose Sunday
    let sb = SkipTextsSegment::new(1, B).execute(gbe, sb); // no DST
    let sb = SkipTextsSegment::new(1, A).execute(gbe, sb); // confirm time
    let sb = SkipTextsSegment::new(3, B).execute(gbe, sb); // mom speech
    let sb = SkipTextsSegment::new(1, A).execute(gbe, sb); // know phone
    let sb = SkipTextsSegment::new(5, B).execute(gbe, sb); // mom speech
    let sb = gen2::TurnSegment::new(R).execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = gen2::WalkToSegment::new(7, 7).with_debug_output(true).execute(gbe, sb);
    let sb = gen2::WarpSegment::new().with_input(D).execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    sb.save("crystal_left_house");
    let sb = StateBuffer::load("crystal_left_house");
    // let sb = gen2::TurnSegment::new(L).execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = gen2::WalkToSegment::new(6, 3).into(gen2::OverworldInteractionResult::Warped).with_debug_output(true).execute(gbe, sb);
    let sb = gen2::WarpSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = SkipTextsSegment::new(12, B).execute(gbe, sb); // elm speech
    let sb = SkipTextsSegment::new(1, A).execute(gbe, sb); // choose to help
    let sb = SkipTextsSegment::new(6, B).execute(gbe, sb); // elm speech
    let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = SkipTextsSegment::new(15, B).execute(gbe, sb); // elm speech
    let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = SkipTextsSegment::new(5, B).execute(gbe, sb); // elm speech
    let sb = gen2::WalkToSegment::new(7, 4).with_debug_output(true).execute(gbe, sb);
    let sb = MoveSegment::with_metric(NIL, gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v == &gen2::OverworldInteractionResult::NoEvents}).into_unit()).execute(gbe, sb); println!("{}", sb);
    let sb = gen2::TurnSegment::new(U).execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = MoveSegment::with_metric(A, gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); if let gen2::OverworldInteractionResult::Interact(_) = v { true } else { false }}).into_unit()).execute(gbe, sb); println!("{}", sb);
    let sb = MoveSegment::new(B).execute(gbe, sb); // close picture
    let sb = SkipTextsSegment::new(2, A).execute(gbe, sb); // choose Totodile
    sb.save("crystal_choose_starter");
    let sb = StateBuffer::load("crystal_choose_starter");
    let sb = SkipTextsSegment::new(2, B).with_buffer_size(4096).execute(gbe, sb); println!("{}", sb); // elm speech
    let sb = TextSegment::new(A).with_buffer_size(4096).execute(gbe, sb); println!("{}", sb); // elm speech
    sb.save("crystal_choose_starter_unbounded");
    let sb = StateBuffer::load("crystal_choose_starter_unbounded");
    let sb = DelaySegment::new(MoveSegment::with_metric(B, Gen2DVMetric {}.filter(|v| {
      if v.atk < 15 || v.spc < 15 || v.spd < 15 { return false; }
      println!("Chosen DVs: {:?}", v); true
    }).into_unit())).with_debug_output(true).execute(gbe, sb); println!("{}", sb);
    let sb = SkipTextsSegment::new(2, B).execute(gbe, sb); println!("{}", sb); // no nickname
    let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = SkipTextsSegment::new(11, B).execute(gbe, sb); // elm speech
    let sb = gen2::TurnSegment::new(D).execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = gen2::WalkToSegment::new(4, 7).with_debug_output(true).execute(gbe, sb);
    let sb = gen2::WalkStepSegment::new(D).into(gen2::OverworldInteractionResult::MapCoordEvent).execute(gbe, sb); println!("{}", sb);
    let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = SkipTextsSegment::new(7, B).execute(gbe, sb); // aide speech
    let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = gen2::WalkToSegment::new(4, 11).with_debug_output(true).execute(gbe, sb);
    let sb = gen2::WarpSegment::new().with_input(D).execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    sb.save("crystal_test_after_elm");
    let sb = StateBuffer::load("crystal_test_after_elm");
    let sb = gen2::WalkToSegment::new(-1, 8).into(gen2::OverworldInteractionResult::MapConnection).with_debug_output(true).execute(gbe, sb);
    let sb = MoveSegment::new(NIL).execute(gbe, sb); println!("{}", sb); // MapConnection
    let sb = gen2::WalkToSegment::new(9, 6).with_debug_output(true).execute(gbe, sb);
    let sb = gen2::JumpLedgeSegment::new(L).execute(gbe, sb); println!("{}", sb);
    let sb = gen2::WalkToSegment::new(-1, 7).into(gen2::OverworldInteractionResult::MapConnection).with_debug_output(true).execute(gbe, sb);
    let sb = MoveSegment::new(NIL).execute(gbe, sb); println!("{}", sb); // MapConnection
    let sb = gen2::WalkToSegment::new(17, -1).into(gen2::OverworldInteractionResult::MapConnection).with_debug_output(true).execute(gbe, sb);
    let sb = MoveSegment::new(NIL).execute(gbe, sb); println!("{}", sb); // MapConnection
    let sb = gen2::WalkToSegment::new(17, 5).into(gen2::OverworldInteractionResult::Warped).with_debug_output(true).execute(gbe, sb);
    let sb = gen2::WarpSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    sb.save("crystal_test_entered_mr_pokemon_house");
    let sb = StateBuffer::load("crystal_test_entered_mr_pokemon_house");
    let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = SkipTextsSegment::new(2, B).execute(gbe, sb); // Mr.Pokemon speech
    let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = SkipTextsSegment::new(14, B).execute(gbe, sb); // Mr.Pokemon speech
    let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = SkipTextsSegment::new(29, B).execute(gbe, sb); // Oak speech
    let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = SkipTextsSegment::new(4, B).execute(gbe, sb); // Mr.Pokemon speech
    let sb = gen2::TurnSegment::new(D).execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    let sb = gen2::WalkStepSegment::new(D).execute(gbe, sb); println!("{}", sb);
    let sb = gen2::WarpSegment::new().with_input(D).execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    sb.save("crystal_test_after_mr_pokemon_house");
    let sb = StateBuffer::load("crystal_test_after_mr_pokemon_house");
    let sb = MoveLoopSegment::new(gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v != &gen2::OverworldInteractionResult::CountStepEvent}).into_unit()).execute(gbe, sb); println!("{}", sb);
    let sb = SkipTextsSegment::new(4, B).execute(gbe, sb); // Elm phone call
    let sb = TextSegment::new(A).execute(gbe, sb); // Elm phone call ends
    let sb = gen2::WalkToSegment::new(7, 54).into(gen2::OverworldInteractionResult::MapConnection).with_debug_output(true).execute(gbe, sb);
    let sb = MoveSegment::new(NIL).execute(gbe, sb); println!("{}", sb); // MapConnection
    let sb = gen2::WalkToSegment::new(33, 7).into(gen2::OverworldInteractionResult::MapCoordEvent).with_debug_output(true).execute(gbe, sb);
    let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    sb.save("crystal_test_before_rival1");
    let sb = StateBuffer::load("crystal_test_before_rival1");
    let sb = SkipTextsSegment::new(7, B).execute(gbe, sb); // pre-battle texts
    let sb = SkipTextsSegment::new(1, B).execute(gbe, sb); // trainer wants to battle
    let sb = SkipTextsSegment::new(1, B).execute(gbe, sb); // trainer sent out ...
    // let sb = TextSegment::new(A).execute(gbe, sb); // mon! / Go! mon!

    let sb = IdentifyInputSegment::new().execute(gbe, sb);
    sleep(Duration::from_millis(1000));

    let mut result = HashMap::new();
    result.insert((), sb);
    result
  }
}





#[allow(dead_code)]
fn convert_efl() {
  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let (hi_inputs, lo_inputs) = {
    let gb = Gambatte::create("roms/gbc_bios.bin", "roms/crystal.gbc", false /* equal length frames */, SdlScreen::new(sdl.clone(), 0 /* screen */));
    ftii::to_ftii::<Crystal>(gb, read_bk2_inputs("temp/crystal_test.bk2").unwrap())
  };

  let inputs = {
    let gb = Gambatte::create("roms/gbc_bios.bin", "roms/crystal.gbc", true /* equal length frames */, SdlScreen::new(sdl, 0 /* screen */));
    ftii::from_ftii::<Crystal>(gb, hi_inputs, lo_inputs)
  };
  Bk2Writer::new::<Crystal>().with_equal_length_frames(true).write_bk2("temp/crystal_test_efl.bk2", &inputs).unwrap();
}

#[allow(dead_code)]
fn playback_inputs() {
  let inputs = read_bk2_inputs("temp/tikevin83-pokemonyellow-consoleverified.bk2").unwrap();
  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let mut gb = Gambatte::create("roms/gbc_bios.bin", "roms/yellow.gbc", false /* equal length frames */, SdlScreen::new(sdl, 0 /* screen */));
  for input in inputs {
    gb.set_input(input);
    gb.step();
    // sleep(Duration::from_millis(1));
  }
}

#[allow(dead_code)]
fn playback_demos() {
  let sdl = Sdl::init_sdl(4 /* num screens */, 3 /* scale */);
  let mut gb1 = Gambatte::create("roms/gbc_bios.bin", "roms/blue.gb", false /* equal length frames */, SdlScreen::new(sdl.clone(), 0 /* screen */));
  let mut gb2 = Gambatte::create("roms/gbc_bios.bin", "roms/yellow.gbc", false /* equal length frames */, SdlScreen::new(sdl.clone(), 1 /* screen */));
  let mut gb3 = Gambatte::create("roms/gbc_bios.bin", "roms/silver.gbc", false /* equal length frames */, SdlScreen::new(sdl.clone(), 2 /* screen */));
  let mut gb4 = Gambatte::create("roms/gbc_bios.bin", "roms/crystal.gbc", false /* equal length frames */, SdlScreen::new(sdl.clone(), 3 /* screen */));
  let inputs1 = read_bk2_inputs("temp/blue_demo.bk2").unwrap();
  let inputs2 = read_bk2_inputs("temp/tikevin83-pokemonyellow-consoleverified.bk2").unwrap();
  let inputs3 = vec![NIL; 1000];
  let inputs4 = read_bk2_inputs("temp/crystal_demo.bk2").unwrap();
  for i in 0..1_000_000 {
    let mut has_input = false;
    gb1.set_input(if inputs1.len() > i {has_input = true; inputs1[i]} else {Input::from_bits_truncate(1 << (rand::random::<u8>() & 7))}); gb1.step();
    gb2.set_input(if inputs2.len() > i {has_input = true; inputs2[i]} else {Input::from_bits_truncate(1 << (rand::random::<u8>() & 7))}); gb2.step();
    gb3.set_input(if inputs3.len() > i {has_input = true; inputs3[i]} else {Input::from_bits_truncate(1 << (rand::random::<u8>() & 7))}); gb3.step();
    gb4.set_input(if inputs4.len() > i {has_input = true; inputs4[i]} else {Input::from_bits_truncate(1 << (rand::random::<u8>() & 7))}); gb4.step();
    if !has_input { break; }
    if i == 8711 { gb1.write_memory(0xd179, 0x10); } // give XP
    if i == 47000 { gb4.write_memory(0xdce7, 0x20); } // give XP
  }
}
