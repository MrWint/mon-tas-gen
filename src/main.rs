mod bk2;
mod logger;

use crate::bk2::{Bk2Writer, read_bk2_inputs};
use gambatte::*;
use gambatte::inputs::*;
use log::{LevelFilter::*};
use montas::constants::*;
use montas::ftii;
use montas::gb::*;
#[allow(unused_imports)] use montas::gbexecutor::*;
use montas::rom::*;
use montas::segment::*;
#[allow(unused_imports)] use montas::segment::overworld::gen2;
#[allow(unused_imports)] use montas::util::*;
use montas::sdl::*;
use montas::statebuffer::StateBuffer;
use montas::util::with_log_level;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

fn main() {
  crate::logger::init_with_level(Info).unwrap();

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


  // playback_test();
  // playback_demos();
  // convert_efl();

  // BlueTestSegment::run();
  // YellowTestSegment::run();
  SilverTestSegment::run();
  // CrystalTestSegment::run();
}

fn test_segment_end<R: Rom, E: GbExecutor<R>>(gbe: &mut E, sb: &StateBuffer, file_name: &str) {
  let chosen_state = sb.into_iter().next().unwrap().clone();
  {
    log::info!("Creating sample input file...");
    let inputs = gbe.execute(&[&chosen_state], move |gb, s, tx| {
      gb.restore(&s);
      tx.send(s.replace_value(gb.create_inputs())).unwrap();
    }).into_iter().map(|s| s.value).min_by_key(Vec::len).unwrap();
    Bk2Writer::new::<R>().write_bk2(&format!("{}.bk2", file_name), &inputs).unwrap();
  }
  log::info!("Rendering end states of {}", sb);
  gbe.execute(sb, move |gb, s, tx| {
    gb.restore(&s);
    for _ in 0..10 {
      gb.input(Input::empty());
      gb.step();
      std::thread::sleep(std::time::Duration::from_millis(200));
    }
    for _ in 0..1000 {
      gb.input(Input::empty());
      gb.step();
    }
    std::thread::sleep(std::time::Duration::from_millis(200));
    tx.send(s).unwrap();
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
    let sb = StateBuffer::<()>::load("blue_test");
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
    // let sb: StateBuffer = StateBuffer::load("yellow_before_collect_pikachu_2_256_wait1_");
    // let sb = SkipTextsSegment::new(2, B).with_buffer_size(4096).execute(gbe, sb); // get Pikachu
    // let sb = TextSegment::new(A).with_buffer_size(4096).execute(gbe, sb); // Nickname?
    // let sb = with_log_level(Debug, || { DelaySegment::new(MoveSegment::with_metric(B, Gen1DVMetric {}.filter(|v| {
    //   let dvs = u16::from(v.atk) << 12 | u16::from(v.def) << 8 | u16::from(v.spd) << 4 | u16::from(v.spc);
    //   let div = ((dvs >> 8) as u8).wrapping_sub(dvs as u8);
    //   if ::rand::random::<u8>() < 0x04 {
    //     print!("{:X}", div);
    //     // print!("/{:X}", v.div_state >> 6);
    //     // print!("/{}", to_human_readable_time(v.cycle_count));
    //     print!(" ");
    //   }
    //   if v.atk != 7 { return false; }
    //   if v.spc != 8 && v.spc != 9 { return false; }
    //   if v.def != 0 && v.def != 1 { return false; }
    //   if v.spd != 12 && v.spd != 13 { return false; }
    //   println!("Chosen DVs: {:?}", v); true
    // }).into_unit())).execute(gbe, sb) });
    // sb.save("yellow_after_collect_pikachu_256_div");
    let sb = StateBuffer::<()>::load("yellow_after_collect_pikachu_256_div");
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
    // let sb = DelaySegment::new(MoveSegment::with_metric_fn(A, |_gb| Some(()))).execute(gbe, _iter);
    // println!("{}", sb);
    // let sb = MoveSegment::new(START).with_max_skips(10).execute(gbe, sb);
    // println!("{}", sb);
    // let sb = MoveSegment::new(D).execute(gbe, sb); // options
    // let sb = MoveSegment::new(L|A).execute(gbe, sb); // fast options
    // let sb = MoveSegment::new(B).execute(gbe, sb); // back
    // let sb = MoveSegment::new(A).execute(gbe, sb); // new game
    // println!("{}", sb);
    // let sb = SkipTextsSegment::new(3).execute(gbe, sb);
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).execute(gbe, sb); // time: 10:..
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A).execute(gbe, sb); // what // 10 oclock // ?
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).execute(gbe, sb); // ? // How many minutes?
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A).execute(gbe, sb); // whoa // 00 min // ?
    // let sb = SkipTextsSegment::new(1).with_skip_ends(1).execute(gbe, sb); // day // overslept
    // let sb = SkipTextsSegment::new(17).execute(gbe, sb); // oak speech
    // let sb = MoveSegment::new(D).execute(gbe, sb); // Name: Silver
    // let sb = MoveSegment::new(A).execute(gbe, sb); // Name: Silver
    // let sb = SkipTextsSegment::new(7).execute(gbe, sb); // skip texts until game start
    // let sb = TextSegment::new().execute(gbe, sb); // ... seeing you later
    // sb.save("silver_test");
    // let sb: StateBuffer = StateBuffer::load("silver_test");
    // let sb = gen2::TurnSegment::new(R).execute(gbe, sb);
    // let sb = with_log_level(Debug, || { gen2::WalkToSegment::new(7, 0).into(gen2::OverworldInteractionResult::Warped).execute(gbe, sb) });
    // let sb = gen2::WarpSegment::new().execute(gbe, sb);
    // let sb = gen2::SkipScriptSegment::new().execute(gbe, sb);
    // let sb = SkipTextsSegment::new(7).execute(gbe, sb); // mom speech
    // let sb = SkipTextsSegment::new(1).with_skip_ends(4).execute(gbe, sb); // silver // received // PokeGear // . // Pokemon Gear
    // let sb = SkipTextsSegment::new(4).execute(gbe, sb); // mom speech
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).execute(gbe, sb); // choose Sunday
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).with_skip_ends(1).execute(gbe, sb); // Sunday // is it?
    // let sb = SkipTextsSegment::new(1).with_confirm_input(B).execute(gbe, sb); // no DST
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).with_skip_ends(1).execute(gbe, sb); // 10:00 AM // confirm time
    // let sb = SkipTextsSegment::new(3).execute(gbe, sb); // mom speech
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).execute(gbe, sb); // know phone
    // let sb = SkipTextsSegment::new(5).execute(gbe, sb); // mom speech
    // let sb = gen2::SkipScriptSegment::new().execute(gbe, sb);
    // let sb = gen2::WalkToSegment::new(7, 7).execute(gbe, sb);
    // let sb = gen2::WarpSegment::new().with_input(D).execute(gbe, sb); println!("{}", sb);
    // sb.save("silver_left_house");
    // let sb: StateBuffer = StateBuffer::load("silver_left_house");
    // let sb = gen2::WalkToSegment::new(6, 3).into(gen2::OverworldInteractionResult::Warped).execute(gbe, sb);
    // let sb = gen2::WarpSegment::new().execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(16).execute(gbe, sb); // elm speech
    // let sb = gen2::TurnSegment::new(D).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(7, 4).execute(gbe, sb);
    // let sb = MoveSegment::with_metric(NIL, gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v == &gen2::OverworldInteractionResult::NoEvents}).into_unit()).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::TurnSegment::new(U).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(A, gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); if let gen2::OverworldInteractionResult::Interact(_) = v { true } else { false }}).into_unit()).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::new(B).execute(gbe, sb); // close picture
    // let sb = SkipTextsSegment::new(1).execute(gbe, sb); // choose Totodile
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).execute(gbe, sb); // choose Totodile
    // sb.save("silver_choose_starter");
    // let sb: StateBuffer = StateBuffer::load("silver_choose_starter");
    // let sb = SkipTextsSegment::new(2).with_buffer_size(8192).execute(gbe, sb); println!("{}", sb); // elm speech
    // let sb = TextSegment::new().with_skip_ends(2).with_buffer_size(8192).execute(gbe, sb); println!("{}", sb); // Player received // Totodile // !
    // sb.save("silver_choose_starter_unbounded");
    // let sb: StateBuffer = StateBuffer::load("silver_choose_starter_unbounded");
    // let sb = with_log_level(Debug, || { DelaySegment::new(MoveSegment::with_metric(A | B, Gen2DVMetric {}.filter(|v| {
    //   if v.atk < 15 || /*v.spc < 15 ||*/ v.spd < 15 { return false; }
    //   log::debug!("Chosen DVs: {:?}", v); true
    // }).into_unit())).execute(gbe, sb) }); println!("{}", sb);
    // sb.save("silver_after_choose_starter");
    // let sb: StateBuffer = StateBuffer::load("silver_after_choose_starter");
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); println!("{}", sb); // nickname to // Totodile // you
    // let sb = SkipTextsSegment::new(1).with_confirm_input(B).execute(gbe, sb); println!("{}", sb); // no nickname
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(9).execute(gbe, sb); // elm speech
    // let sb = gen2::TurnSegment::new(D).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(4, 7).execute(gbe, sb);
    // let sb = gen2::WalkStepSegment::new(D).into(gen2::OverworldInteractionResult::MapCoordEvent).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(2).execute(gbe, sb); // aide speech
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // player received // potion // .
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // player put // potion // in
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // the // item pocket // .
    // let sb = SkipTextsSegment::new(2).execute(gbe, sb); // aide speech
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(4, 11).execute(gbe, sb);
    // let sb = gen2::WarpSegment::new().with_input(D).execute(gbe, sb); println!("{}", sb);
    // sb.save("silver_test_after_elm");
    // let sb: StateBuffer = StateBuffer::load("silver_test_after_elm");
    // let sb = gen2::WalkToSegment::new(-1, 8).into(gen2::OverworldInteractionResult::MapConnection).execute(gbe, sb);
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); println!("{}", sb); // MapConnection
    // let sb = gen2::WalkToSegment::new(9, 6).execute(gbe, sb);
    // let sb = gen2::JumpLedgeSegment::new(L).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(-1, 7).into(gen2::OverworldInteractionResult::MapConnection).execute(gbe, sb);
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); println!("{}", sb); // MapConnection
    // let sb = gen2::WalkToSegment::new(17, -1).into(gen2::OverworldInteractionResult::MapConnection).execute(gbe, sb);
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); println!("{}", sb); // MapConnection
    // let sb = gen2::WalkToSegment::new(17, 5).into(gen2::OverworldInteractionResult::Warped).execute(gbe, sb);
    // let sb = gen2::WarpSegment::new().execute(gbe, sb); println!("{}", sb);
    // sb.save("silver_test_entered_mr_pokemon_house");
    // let sb: StateBuffer = StateBuffer::load("silver_test_entered_mr_pokemon_house");
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(2).execute(gbe, sb); // Mr.Pokemon speech
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(2).execute(gbe, sb); // Mr.Pokemon speech
    // let sb = SkipTextsSegment::new(1).with_skip_ends(4).execute(gbe, sb); // // // put // mystery egg // in
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // key poket // .
    // let sb = SkipTextsSegment::new(10).execute(gbe, sb); // Mr.Pokemon speech
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(23).execute(gbe, sb); // Oak speech
    // let sb = SkipTextsSegment::new(1).with_skip_ends(1).execute(gbe, sb); // got pokedex // speech
    // let sb = SkipTextsSegment::new(5).execute(gbe, sb); // Oak speech
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(4).execute(gbe, sb); // Mr.Pokemon speech
    // let sb = gen2::TurnSegment::new(D).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::WalkStepSegment::new(D).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::WarpSegment::new().with_input(D).execute(gbe, sb); println!("{}", sb);
    // sb.save("silver_test_after_mr_pokemon_house");
    // let sb: StateBuffer = StateBuffer::load("silver_test_after_mr_pokemon_house");
    // let sb = MoveLoopSegment::new(gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v != &gen2::OverworldInteractionResult::CountStepEvent}).into_unit()).execute(gbe, sb); println!("{}", sb);
    // let sb = SkipTextsSegment::new(4).execute(gbe, sb); // Elm phone call
    // let sb = TextSegment::new().with_skip_ends(6).execute(gbe, sb); // Click // ... // ... // ...
    // let sb = gen2::WalkToSegment::new(7, 54).into(gen2::OverworldInteractionResult::MapConnection).execute(gbe, sb);
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); println!("{}", sb); // MapConnection
    // let sb = gen2::WalkToSegment::new(33, 7).into(gen2::OverworldInteractionResult::MapCoordEvent).execute(gbe, sb);
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // sb.save("silver_test_before_rival1");
    // let sb: StateBuffer = StateBuffer::load("silver_test_before_rival1");
    // let sb = SkipTextsSegment::new(7).execute(gbe, sb); // pre-battle texts
    // let sb = SkipTextsSegment::new(2).execute(gbe, sb); // trainer wants to battle // trainer sent out
    // let sb = with_log_level(Debug, || { TextSegment::with_metric(montas::segment::battle::gen2::Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Growl)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5).execute(gbe, sb) }); // chikorita // ! // Go // Totodile // !
    // println!("Player: {:?}", gbe.execute_state(&sb, montas::segment::battle::MoveInfosFn::new(montas::segment::battle::Who::Player)).get_value_assert_all_equal());
    // println!("Player: {:#?}", gbe.execute_state(&sb, montas::segment::battle::BattleMonInfoFn::new(montas::segment::battle::Who::Player)).get_value_assert_all_equal());
    // println!("Enemy: {:?}", gbe.execute_state(&sb, montas::segment::battle::MoveInfosFn::new(montas::segment::battle::Who::Enemy)).get_value_assert_all_equal());
    // println!("Enemy: {:#?}", gbe.execute_state(&sb, montas::segment::battle::BattleMonInfoFn::new(montas::segment::battle::Who::Enemy)).get_value_assert_all_equal());
    // // Turn 1
    // let sb = MoveSegment::new(A).execute(gbe, sb); // Fight
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); // neutral
    // let sb = with_log_level(Debug, || {
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         montas::segment::battle::BattleMoveOrderMetric {}.debug_print().expect(montas::segment::battle::MoveOrder::PlayerFirst)
    //         .and_then(montas::segment::battle::BattleObedienceMetric {}.debug_print().expect(montas::segment::battle::BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(
    //         montas::segment::battle::gen2::Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(montas::segment::battle::gen2::FightTurnResult::CriticalHit { damage: 8, }))
    //         .with_skip_ends(3).with_unbounded_buffer())
    //     ).execute(gbe, sb) }); // Scratch //// mon // used // move // !
    // let sb = TextSegment::new().execute(gbe, sb); // critical hit!
    // let sb = MoveSegment::with_metric(A|B, montas::segment::battle::BattleObedienceMetric {}.expect(montas::segment::battle::BattleObedience::Obey)).execute(gbe, sb); // confirm
    // let sb = with_log_level(Debug, || { TextSegment::with_metric(montas::segment::battle::gen2::Gen2StatUpDownMetric {}.debug_print().expect(montas::segment::battle::gen2::FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer().execute(gbe, sb) }); // mon // used // move // !
    // let sb = TextSegment::new().with_allowed_end_inputs(A).execute(gbe, sb); // but it failed!
    // let sb = DelaySegment::new(MoveSegment::with_metric(B, montas::segment::battle::gen2::Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Growl))).execute(gbe, sb); // confirm
    // // Turn 2
    // let sb = MoveSegment::new(A).execute(gbe, sb); // Fight
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); // neutral
    // let sb = with_log_level(Debug, || {
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         montas::segment::battle::BattleMoveOrderMetric {}.debug_print().expect(montas::segment::battle::MoveOrder::PlayerFirst)
    //         .and_then(montas::segment::battle::BattleObedienceMetric {}.debug_print().expect(montas::segment::battle::BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(
    //         montas::segment::battle::gen2::Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(montas::segment::battle::gen2::FightTurnResult::CriticalHit { damage: 8, }))
    //         .with_skip_ends(3).with_unbounded_buffer())
    //     ).execute(gbe, sb) }); // Scratch //// mon // used // move // !
    // let sb = TextSegment::new().execute(gbe, sb); // critical hit!
    // let sb = with_log_level(Debug, || {
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A|B,
    //         montas::segment::battle::BattleObedienceMetric {}.expect(montas::segment::battle::BattleObedience::Obey))
    //     .seq(TextSegment::with_metric(
    //         montas::segment::battle::gen2::Gen2StatUpDownMetric {}.debug_print().expect(montas::segment::battle::gen2::FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
    //     ).execute(gbe, sb) }); // confirm //// mon // used // move // !
    // let sb = SkipTextsSegment::new(1).with_confirm_input(B).execute(gbe, sb); // but it failed!
    // // Turn 3
    // let sb = MoveSegment::new(A).execute(gbe, sb); // Fight
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); // neutral
    // let sb = with_log_level(Debug, || { MoveSegment::with_metric(A, montas::segment::battle::BattleMoveOrderMetric {}.debug_print().expect(montas::segment::battle::MoveOrder::PlayerFirst)
    //     .and_then(montas::segment::battle::BattleObedienceMetric {}.debug_print().expect(montas::segment::battle::BattleObedience::Obey))).execute(gbe, sb) }); // Scratch
    // let sb = with_log_level(Debug, || { TextSegment::with_metric(montas::segment::battle::gen2::Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(montas::segment::battle::gen2::FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer().execute(gbe, sb) }); // mon // used // move // !

    // sb.save("silver_test_rival1_defeated");
    let sb: StateBuffer = StateBuffer::load("silver_test_rival1_defeated");

    let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // enemy // mon // fainted
    let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // mon // gained // num XP
    let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // mon // grew to level // X
    let sb = SkipTextsSegment::new(1).execute(gbe, sb); // ??? was defeated
    let sb = SkipTextsSegment::new(1).execute(gbe, sb); // defeat text
    let sb = SkipTextsSegment::new(1).with_skip_ends(1).execute(gbe, sb); // player got // X for winning

    let sb = gen2::SkipScriptSegment::new().execute(gbe, sb);
    let sb = SkipTextsSegment::new(5).execute(gbe, sb); // ... ... ... // name is ??? // world's greatest // mon // trainer
    let sb = gen2::SkipScriptSegment::new().execute(gbe, sb);
    let sb = gen2::TurnSegment::new(U).execute(gbe, sb);
    let sb = with_log_level(Debug, || { gen2::WalkToSegment::new(40, 7).into(gen2::OverworldInteractionResult::MapConnection).execute(gbe, sb) });

    // let sb = with_log_level(Debug, || { TextSegment::new().with_skip_ends(0).execute(gbe, sb) }); // 
    // let sb = with_log_level(Debug, || { MoveSegment::with_metric(NIL, gen2::OverworldInteractionMetric {}.debug_print().into_unit()).execute(gbe, sb) }); println!("{}", sb);
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
    // let sb = MoveSegment::new(A).execute(gbe, _iter);
    // let sb = MoveSegment::new(START).execute(gbe, sb);
    // let sb = MoveSegment::new(D).execute(gbe, sb); // options
    // let sb = MoveSegment::new(L|A).execute(gbe, sb); // fast options
    // let sb = MoveSegment::new(B).execute(gbe, sb); // back
    // let sb = MoveSegment::new(A).execute(gbe, sb); // new game
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).execute(gbe, sb); // choose Boy
    // let sb = SkipTextsSegment::new(3).execute(gbe, sb);
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).execute(gbe, sb); // time: 10:..
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A).execute(gbe, sb); // what // 10 oclock // ?
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).execute(gbe, sb); // ? // How many minutes?
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A).execute(gbe, sb); // whoa // 00 min // ?
    // let sb = SkipTextsSegment::new(1).with_skip_ends(1).execute(gbe, sb); // day // overslept
    // let sb = SkipTextsSegment::new(17).execute(gbe, sb); // oak speech
    // let sb = MoveSegment::new(D).execute(gbe, sb); // Name: Chris
    // let sb = MoveSegment::new(A).execute(gbe, sb); // Name: Chris
    // let sb = SkipTextsSegment::new(7).execute(gbe, sb); // skip texts until game start
    // let sb = TextSegment::new().execute(gbe, sb); // ... seeing you later
    // sb.save("crystal_test");
    // let sb: StateBuffer = StateBuffer::load("crystal_test");
    // let sb = gen2::TurnSegment::new(R).execute(gbe, sb);
    // let sb = gen2::WalkToSegment::new(7, 0).into(gen2::OverworldInteractionResult::Warped).execute(gbe, sb);
    // let sb = gen2::WarpSegment::new().execute(gbe, sb);
    // let sb = gen2::WalkToSegment::new(7, 2).execute(gbe, sb);
    // let sb = gen2::WalkStepSegment::new(D).execute(gbe, sb);
    // let sb = MoveSegment::with_metric(A, gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); if let gen2::OverworldInteractionResult::Interact(_) = v { true } else { false }}).into_unit()).execute(gbe, sb); println!("{}", sb);
    // let sb = SkipTextsSegment::new(7).execute(gbe, sb); // mom speech
    // let sb = SkipTextsSegment::new(1).with_skip_ends(3).execute(gbe, sb); // received // PokeGear // . // Pokemon Gear
    // let sb = SkipTextsSegment::new(4).execute(gbe, sb); // mom speech
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).execute(gbe, sb); // choose Sunday
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).with_skip_ends(1).execute(gbe, sb); // Sunday // is it?
    // let sb = SkipTextsSegment::new(1).with_confirm_input(B).execute(gbe, sb); // no DST
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).with_skip_ends(1).execute(gbe, sb); // 10:00 AM // confirm time
    // let sb = SkipTextsSegment::new(3).execute(gbe, sb); // mom speech
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).execute(gbe, sb); // know phone
    // let sb = SkipTextsSegment::new(5).execute(gbe, sb); // mom speech
    // let sb = gen2::TurnSegment::new(R).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(7, 7).execute(gbe, sb);
    // let sb = gen2::WarpSegment::new().with_input(D).execute(gbe, sb); println!("{}", sb);
    // sb.save("crystal_left_house");
    // let sb: StateBuffer = StateBuffer::load("crystal_left_house");
    // let sb = gen2::WalkToSegment::new(6, 3).into(gen2::OverworldInteractionResult::Warped).execute(gbe, sb);
    // let sb = gen2::WarpSegment::new().execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(12).execute(gbe, sb); // elm speech
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).execute(gbe, sb); // choose to help
    // let sb = SkipTextsSegment::new(6).execute(gbe, sb); // elm speech
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(15).execute(gbe, sb); // elm speech
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(5).execute(gbe, sb); // elm speech
    // let sb = gen2::WalkToSegment::new(7, 4).execute(gbe, sb);
    // let sb = MoveSegment::with_metric(NIL, gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v == &gen2::OverworldInteractionResult::NoEvents}).into_unit()).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::TurnSegment::new(U).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(A, gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); if let gen2::OverworldInteractionResult::Interact(_) = v { true } else { false }}).into_unit()).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::new(B).execute(gbe, sb); // close picture
    // let sb = SkipTextsSegment::new(1).execute(gbe, sb); // choose Totodile
    // let sb = SkipTextsSegment::new(1).with_confirm_input(A).execute(gbe, sb); // choose Totodile
    // sb.save("crystal_choose_starter");
    // let sb: StateBuffer = StateBuffer::load("crystal_choose_starter");
    // let sb = SkipTextsSegment::new(2).with_buffer_size(8192).execute(gbe, sb); println!("{}", sb); // elm speech
    // let sb = TextSegment::new().with_skip_ends(2).with_buffer_size(8192).execute(gbe, sb); println!("{}", sb); // Player received // Totodile // !
    // sb.save("crystal_choose_starter_unbounded");
    // let sb: StateBuffer = StateBuffer::load("crystal_choose_starter_unbounded");
    // let sb = with_log_level(Debug, || { DelaySegment::new(MoveSegment::with_metric(A | B, Gen2DVMetric {}.filter(|v| {
    //   if v.atk < 15 || v.def < 15 || v.spc < 15 || v.spd < 15 { return false; }
    //   log::debug!("Chosen DVs: {:?}", v); true
    // }).into_unit())).execute(gbe, sb) }); println!("{}", sb);
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); println!("{}", sb); // nickname to // Totodile // you
    // let sb = SkipTextsSegment::new(1).with_confirm_input(B).execute(gbe, sb); println!("{}", sb); // no nickname
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(11).execute(gbe, sb); // elm speech
    // let sb = gen2::TurnSegment::new(D).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(4, 7).execute(gbe, sb);
    // let sb = gen2::WalkStepSegment::new(D).into(gen2::OverworldInteractionResult::MapCoordEvent).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(2).execute(gbe, sb); // aide speech
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // player received // potion // .
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // player put // potion // in
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // the // item pocket // .
    // let sb = SkipTextsSegment::new(2).execute(gbe, sb); // aide speech
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(4, 11).execute(gbe, sb);
    // let sb = gen2::WarpSegment::new().with_input(D).execute(gbe, sb); println!("{}", sb);
    // sb.save("crystal_test_after_elm");
    // let sb: StateBuffer = StateBuffer::load("crystal_test_after_elm");
    // let sb = gen2::WalkToSegment::new(-1, 8).into(gen2::OverworldInteractionResult::MapConnection).execute(gbe, sb);
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); println!("{}", sb); // MapConnection
    // let sb = gen2::WalkToSegment::new(9, 6).execute(gbe, sb);
    // let sb = gen2::JumpLedgeSegment::new(L).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::WalkToSegment::new(-1, 7).into(gen2::OverworldInteractionResult::MapConnection).execute(gbe, sb);
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); println!("{}", sb); // MapConnection
    // let sb = gen2::WalkToSegment::new(17, -1).into(gen2::OverworldInteractionResult::MapConnection).execute(gbe, sb);
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); println!("{}", sb); // MapConnection
    // let sb = gen2::WalkToSegment::new(17, 5).into(gen2::OverworldInteractionResult::Warped).execute(gbe, sb);
    // let sb = gen2::WarpSegment::new().execute(gbe, sb); println!("{}", sb);
    // sb.save("crystal_test_entered_mr_pokemon_house");
    // let sb: StateBuffer = StateBuffer::load("crystal_test_entered_mr_pokemon_house");
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(2).execute(gbe, sb); // Mr.Pokemon speech
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(2).execute(gbe, sb); // Mr.Pokemon speech
    // let sb = SkipTextsSegment::new(1).with_skip_ends(4).execute(gbe, sb); // // // put // mystery egg // in
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // key poket // .
    // let sb = SkipTextsSegment::new(10).execute(gbe, sb); // Mr.Pokemon speech
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(23).execute(gbe, sb); // Oak speech
    // let sb = SkipTextsSegment::new(1).with_skip_ends(1).execute(gbe, sb); // got pokedex // speech
    // let sb = SkipTextsSegment::new(5).execute(gbe, sb); // Oak speech
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // let sb = SkipTextsSegment::new(4).execute(gbe, sb); // Mr.Pokemon speech
    // let sb = gen2::TurnSegment::new(D).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::WalkStepSegment::new(D).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::WarpSegment::new().with_input(D).execute(gbe, sb); println!("{}", sb);
    // sb.save("crystal_test_after_mr_pokemon_house");
    // let sb: StateBuffer = StateBuffer::load("crystal_test_after_mr_pokemon_house");
    // let sb = MoveLoopSegment::new(gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v != &gen2::OverworldInteractionResult::CountStepEvent}).into_unit()).execute(gbe, sb); println!("{}", sb);
    // let sb = SkipTextsSegment::new(4).execute(gbe, sb); // Elm phone call
    // let sb = TextSegment::new().with_skip_ends(6).execute(gbe, sb); // Click // ... // ... // ...
    // let sb = gen2::WalkToSegment::new(7, 54).into(gen2::OverworldInteractionResult::MapConnection).execute(gbe, sb);
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); println!("{}", sb); // MapConnection
    // let sb = gen2::WalkToSegment::new(33, 7).into(gen2::OverworldInteractionResult::MapCoordEvent).execute(gbe, sb);
    // let sb = gen2::SkipScriptSegment::new().execute_split(gbe, sb).merge_state_buffers(); println!("{}", sb);
    // sb.save("crystal_test_before_rival1");
    // let sb: StateBuffer = StateBuffer::load("crystal_test_before_rival1");
    // let sb = SkipTextsSegment::new(7).execute(gbe, sb); // pre-battle texts
    // let sb = SkipTextsSegment::new(2).execute(gbe, sb); // trainer wants to battle // trainer sent out
    // let sb = with_log_level(Debug, || { TextSegment::with_metric(montas::segment::battle::gen2::Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Growl)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5).execute(gbe, sb) }); // chikorita // ! // Go // Totodile // !
    // println!("Player: {:?}", gbe.execute_state(&sb, montas::segment::battle::MoveInfosFn::new(montas::segment::battle::Who::Player)).get_value_assert_all_equal());
    // println!("Player: {:#?}", gbe.execute_state(&sb, montas::segment::battle::BattleMonInfoFn::new(montas::segment::battle::Who::Player)).get_value_assert_all_equal());
    // println!("Enemy: {:?}", gbe.execute_state(&sb, montas::segment::battle::MoveInfosFn::new(montas::segment::battle::Who::Enemy)).get_value_assert_all_equal());
    // println!("Enemy: {:#?}", gbe.execute_state(&sb, montas::segment::battle::BattleMonInfoFn::new(montas::segment::battle::Who::Enemy)).get_value_assert_all_equal());
    // // Turn 1
    // let sb = MoveSegment::new(A).execute(gbe, sb); // Fight
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); // neutral
    // let sb = with_log_level(Debug, || { MoveSegment::with_metric(A, montas::segment::battle::BattleMoveOrderMetric {}.debug_print().expect(montas::segment::battle::MoveOrder::PlayerFirst)
    //     .and_then(montas::segment::battle::BattleObedienceMetric {}.debug_print().expect(montas::segment::battle::BattleObedience::Obey))).execute(gbe, sb) }); // Scratch
    // let sb = with_log_level(Debug, || { TextSegment::with_metric(montas::segment::battle::gen2::Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(montas::segment::battle::gen2::FightTurnResult::CriticalHit { damage: 8, })).with_skip_ends(3).with_unbounded_buffer().execute(gbe, sb) }); // mon // used // move // !
    // let sb = TextSegment::new().execute(gbe, sb); // critical hit!
    // let sb = MoveSegment::with_metric(A|B, montas::segment::battle::BattleObedienceMetric {}.expect(montas::segment::battle::BattleObedience::Obey)).execute(gbe, sb); // confirm
    // let sb = with_log_level(Debug, || { TextSegment::with_metric(montas::segment::battle::gen2::Gen2StatUpDownMetric {}.debug_print().expect(montas::segment::battle::gen2::FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer().execute(gbe, sb) }); // mon // used // move // !
    // let sb = SkipTextsSegment::new(1).with_confirm_input(B).execute(gbe, sb); // but it failed!
    // // Turn 2
    // let sb = MoveSegment::new(A).execute(gbe, sb); // Fight
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); // neutral
    // let sb = with_log_level(Debug, || {
    //   DelaySegment::new(
    //     MoveSegment::with_metric(A,
    //         montas::segment::battle::BattleMoveOrderMetric {}.debug_print().expect(montas::segment::battle::MoveOrder::PlayerFirst)
    //         .and_then(montas::segment::battle::BattleObedienceMetric {}.debug_print().expect(montas::segment::battle::BattleObedience::Obey)))
    //     .seq(TextSegment::with_metric(
    //         montas::segment::battle::gen2::Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(montas::segment::battle::gen2::FightTurnResult::CriticalHit { damage: 8, }))
    //         .with_skip_ends(3).with_unbounded_buffer())
    //     ).execute(gbe, sb) }); // Scratch //// mon // used // move // !
    // let sb = TextSegment::new().execute(gbe, sb); // critical hit!
    // let sb = MoveSegment::with_metric(A|B, montas::segment::battle::BattleObedienceMetric {}.expect(montas::segment::battle::BattleObedience::Obey)).execute(gbe, sb); // confirm
    // let sb = with_log_level(Debug, || { TextSegment::with_metric(montas::segment::battle::gen2::Gen2StatUpDownMetric {}.debug_print().expect(montas::segment::battle::gen2::FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer().execute(gbe, sb) }); // mon // used // move // !
    // let sb = SkipTextsSegment::new(1).with_confirm_input(B).execute(gbe, sb); // but it failed!
    // // Turn 3
    // let sb = MoveSegment::new(A).execute(gbe, sb); // Fight
    // let sb = MoveSegment::new(NIL).execute(gbe, sb); // neutral
    // let sb = with_log_level(Debug, || { MoveSegment::with_metric(A, montas::segment::battle::BattleMoveOrderMetric {}.debug_print().expect(montas::segment::battle::MoveOrder::PlayerFirst)
    //     .and_then(montas::segment::battle::BattleObedienceMetric {}.debug_print().expect(montas::segment::battle::BattleObedience::Obey))).execute(gbe, sb) }); // Scratch
    // let sb = with_log_level(Debug, || { TextSegment::with_metric(montas::segment::battle::gen2::Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(montas::segment::battle::gen2::FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer().execute(gbe, sb) }); // mon // used // move // !

    // sb.save("crystal_test_rival1_defeated");
    // let sb: StateBuffer = StateBuffer::load("crystal_test_rival1_defeated");

    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // enemy // mon // fainted
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // mon // gained // num XP
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // mon // grew to level // X
    // let sb = SkipTextsSegment::new(1).execute(gbe, sb); // ??? was defeated
    // let sb = SkipTextsSegment::new(1).execute(gbe, sb); // defeat text
    // let sb = SkipTextsSegment::new(1).with_skip_ends(1).execute(gbe, sb); // player got // X for winning

    // let sb = gen2::SkipScriptSegment::new().execute(gbe, sb);
    // let sb = SkipTextsSegment::new(5).execute(gbe, sb); // ... ... ... // name is ??? // world's greatest // mon // trainer
    // let sb = gen2::SkipScriptSegment::new().execute(gbe, sb);
    // let sb = gen2::TurnSegment::new(U).execute(gbe, sb);
    // let sb = with_log_level(Debug, || { gen2::WalkToSegment::new(40, 7).into(gen2::OverworldInteractionResult::MapConnection).execute(gbe, sb) });
    // let sb = MoveSegment::with_metric(NIL, gen2::OverworldInteractionMetric {}.expect(gen2::OverworldInteractionResult::MapConnection)).execute(gbe, sb); println!("{}", sb); // MapConnection
    // let sb = with_log_level(Debug, || { gen2::WalkToSegment::new(14, 12).execute(gbe, sb) });
    // let sb = gen2::JumpLedgeSegment::new(D).execute(gbe, sb); println!("{}", sb);
    // let sb = with_log_level(Debug, || { gen2::WalkToSegment::new(42, 9).execute(gbe, sb) });
    // let sb = gen2::JumpLedgeSegment::new(R).execute(gbe, sb); println!("{}", sb);
    // let sb = with_log_level(Debug, || { gen2::WalkToSegment::new(60, 9).into(gen2::OverworldInteractionResult::MapConnection).execute(gbe, sb) });
    // let sb = MoveSegment::with_metric(NIL, gen2::OverworldInteractionMetric {}.expect(gen2::OverworldInteractionResult::MapConnection)).execute(gbe, sb); println!("{}", sb); // MapConnection
    // let sb = with_log_level(Debug, || { gen2::WalkToSegment::new(6, 3).into(gen2::OverworldInteractionResult::Warped).execute(gbe, sb) });
    // let sb = gen2::WarpSegment::new().execute(gbe, sb); println!("{}", sb);
    // let sb = with_log_level(Debug, || { gen2::WalkToSegment::new(4, 5).into(gen2::OverworldInteractionResult::MapCoordEvent).execute(gbe, sb) });
    // let sb = gen2::SkipScriptSegment::new().execute(gbe, sb);
    // let sb = SkipTextsSegment::new(7).execute(gbe, sb); // mon stolen
    // let sb = SkipTextsSegment::new(1).with_confirm_input(B).execute(gbe, sb); // mon stolen
    // let sb = MoveSegment::new(R).execute(gbe, sb); // B
    // let sb = MoveSegment::new(A).execute(gbe, sb); // B
    // let sb = MoveSegment::new(START).execute(gbe, sb); // confirm
    // let sb = MoveSegment::new(A).execute(gbe, sb); // confirm
    // let sb = MoveSegment::new(B).execute(gbe, sb); // ?
    // let sb = SkipTextsSegment::new(2).execute(gbe, sb); // so B was his name // thanks
    // let sb = gen2::SkipScriptSegment::new().execute(gbe, sb);
    // let sb = gen2::WalkStepSegment::new(R).execute(gbe, sb);
    // let sb = MoveSegment::with_metric(NIL, gen2::OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v == &gen2::OverworldInteractionResult::NoEvents}).into_unit()).execute(gbe, sb); println!("{}", sb);
    // let sb = gen2::TurnSegment::new(U).execute(gbe, sb); println!("{}", sb);
    // let sb = MoveSegment::with_metric(A, gen2::OverworldInteractionMetric {}.expect(gen2::OverworldInteractionResult::Interact(gen2::InteractType::ObjectScript))).execute(gbe, sb); println!("{}", sb);
    // let sb = SkipTextsSegment::new(5).execute(gbe, sb); // terrible // discovery // give egg
    // let sb = gen2::SkipScriptSegment::new().execute(gbe, sb);
    // let sb = SkipTextsSegment::new(1).execute(gbe, sb); // this?
    // let sb = gen2::SkipScriptSegment::new().execute(gbe, sb);
    // let sb = SkipTextsSegment::new(21).execute(gbe, sb); // egg // great discovery // what // incredible // potential
    // let sb = gen2::TurnSegment::new(D).execute(gbe, sb); println!("{}", sb);
    // let sb = with_log_level(Debug, || { gen2::WalkToSegment::new(4, 7).execute(gbe, sb) });
    // let sb = with_log_level(Debug, || { gen2::WalkToSegment::new(4, 8).into(gen2::OverworldInteractionResult::MapCoordEvent).execute(gbe, sb) });
    // let sb = gen2::SkipScriptSegment::new().execute(gbe, sb);
    // let sb = SkipTextsSegment::new(2).execute(gbe, sb); // take balls
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // received // balls // !
    // let sb = SkipTextsSegment::new(4).execute(gbe, sb); // take balls
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // player put // ball // in
    // let sb = SkipTextsSegment::new(1).with_skip_ends(2).execute(gbe, sb); // the // ball pocket // .
    // let sb = gen2::SkipScriptSegment::new().execute(gbe, sb);
    // let sb = gen2::WalkToSegment::new(4, 11).execute(gbe, sb);
    // let sb = gen2::WarpSegment::new().with_input(D).execute(gbe, sb); println!("{}", sb);
    // sb.save("crystal_test_after_elm2");
    let sb: StateBuffer = StateBuffer::load("crystal_test_after_elm2");

    // let sb = with_log_level(Debug, || { TextSegment::new().with_skip_ends(0).execute(gbe, sb) }); // 
    let sb = with_log_level(Debug, || { MoveSegment::with_metric(NIL, gen2::OverworldInteractionMetric {}.debug_print().into_unit()).execute(gbe, sb) }); println!("{}", sb);
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
  let mut gb3 = Gambatte::create("roms/gbc_bios.bin", "roms/gold.gbc", false /* equal length frames */, SdlScreen::new(sdl.clone(), 2 /* screen */));
  let mut gb4 = Gambatte::create("roms/gbc_bios.bin", "roms/crystal.gbc", false /* equal length frames */, SdlScreen::new(sdl.clone(), 3 /* screen */));
  let inputs1 = read_bk2_inputs("temp/blue_demo.bk2").unwrap();
  let inputs2 = read_bk2_inputs("temp/tikevin83-pokemonyellow-consoleverified.bk2").unwrap();
  let inputs3 = read_bk2_inputs("temp/gold_demo.bk2").unwrap();
  let inputs4 = read_bk2_inputs("temp/crystal_demo.bk2").unwrap();
  for i in 0..1_000_000 {
    let mut has_input = false;
    gb1.set_input(if inputs1.len() > i {has_input = true; inputs1[i]} else {Input::from_bits_truncate(1 << (rand::random::<u8>() & 7))}); gb1.step();
    gb2.set_input(if inputs2.len() > i {has_input = true; inputs2[i]} else {Input::from_bits_truncate(1 << (rand::random::<u8>() & 7))}); gb2.step();
    gb3.set_input(if inputs3.len() > i {has_input = true; inputs3[i]} else {Input::from_bits_truncate(1 << (rand::random::<u8>() & 7))}); gb3.step();
    gb4.set_input(if inputs4.len() > i {has_input = true; inputs4[i]} else {Input::from_bits_truncate(1 << (rand::random::<u8>() & 7))}); gb4.step();
    if !has_input { break; }
    if i == 8711 { gb1.write_memory(0xd179, 0x10); } // give XP
    if i == 11152 { gb3.write_memory(0xda32, 0x10); } // give XP
    if i == 47000 { gb4.write_memory(0xdce7, 0x20); } // give XP
  }
}

#[allow(dead_code)]
fn playback_test() {
  let sdl = Sdl::init_sdl(1 /* num screens */, 3 /* scale */);
  let mut gb = Gambatte::create("roms/gbc_bios.bin", "roms/silver.gbc", false /* equal length frames */, SdlScreen::new(sdl.clone(), 0 /* screen */));
  let inputs = read_bk2_inputs("temp/silver_test.bk2").unwrap();
  for input in inputs {
    gb.set_input(input); gb.step();
    std::thread::sleep(std::time::Duration::from_millis(1));
  }
}
