extern crate bincode;
#[macro_use] extern crate bitflags;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate flate2;

use gambatte::*;
use gambatte::inputs::*;
use gb::*;
use rom::*;
use segment::*;
use statebuffer::StateBuffer;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

mod ftii;
mod gambatte;
mod gb;
mod rom;
mod segment;
mod statebuffer;

fn main() {
  Gambatte::init_screens(1 /* num screens */, 3 /* scale */);
  let mut gb = Gb::<Blue>::create(Gambatte::create_on_screen(0 /* screen */, false /* equal length frames */));
  {
    let states = vec![gb.save()];
    let sb = BlueTestSegment{}.execute(&mut gb, states);

    println!("Rendering end states of {}", sb);
    for s in &sb {
      gb.restore(s);
      for _ in 0..1000 {
        gb.input(Input::empty());
        gb.step();
      }
      std::thread::sleep(std::time::Duration::from_millis(100));
    }
    // for _ in 0..1000 {
    //   pgb.input(NIL);
    //   pgb.step();
    //   std::thread::sleep(std::time::Duration::from_millis(1));
    // }
    // save_inputs("temp/test.txt", pgb.create_inputs());
  }
}

pub struct BlueTestSegment {}
impl Segment for BlueTestSegment {
  type Rom = Blue;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<Blue>, iter: I) -> StateBuffer {
    let sb = DelaySegment::new(MoveSegment::with_check(START, |_gb| true)).execute(gb, iter);
    println!("{}", sb);
    let sb = MoveSegment::new(A).with_max_skips(10).execute(gb, sb);
    println!("{}", sb);
    let sb = MoveSegment::new(START).with_max_skips(10).execute(gb, sb);
    println!("{}", sb);
//    let sb = MoveSegment::new(A).with_max_skips(10).execute(gb, sb);
    let sb = MoveSegment::new(D|A).execute(gb, sb); // options
    let sb = MoveSegment::new(L).execute(gb, sb); // text speed fast
    let sb = MoveSegment::new(D).execute(gb, sb); // battle animations...
    let sb = MoveSegment::new(L).execute(gb, sb); // ...off
    let sb = MoveSegment::new(D).execute(gb, sb); // battle style...
    let sb = MoveSegment::new(L).execute(gb, sb); // ...set
    let sb = MoveSegment::new(B).execute(gb, sb); // back
    let sb = MoveSegment::new(A).execute(gb, sb); // new game
    println!("{}", sb);
    let sb = SkipTextsSegment::new(13, B).execute(gb, sb); // skip texts until player name
    let sb = MoveSegment::new(D|A).execute(gb, sb); // Name: Blue
    let sb = SkipTextsSegment::new(5, B).execute(gb, sb); // skip texts until rival name
    let sb = MoveSegment::new(D|A).execute(gb, sb); // Name: Red
    let sb = SkipTextsSegment::new(7, B).execute(gb, sb); // skip texts until game start
    let sb = TextSegment::new(A).execute(gb, sb); // ...awaits, let's go
    println!("{}", sb);
    let sb = MoveSegment::with_check(R, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(R, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(R, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(R, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);

    let sb = MoveSegment::with_check(D, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(D, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(D, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(D, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(D, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(D, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(L, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(L, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(L, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(L, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(D, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(D, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);

    let sb = MoveSegment::with_check(R, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(R, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(R, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(R, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(R, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(R, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);

    let sb = MoveSegment::with_check(A, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    sb.save("blue_test");
    let sb = StateBuffer::load("blue_test");
    let sb = SkipTextsSegment::new(6, B).execute(gb, sb); // it's dangerous to go outside, take this
    let sb = TextSegment::new(A).expect_conflicting_inputs().execute(gb, sb); // come with me
    let sb = MoveSegment::new(B).execute(gb, sb); // come with me
    let sb = IdentifyInputSegment::new().execute(gb, sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = MoveSegment::with_check(U, |gb| {println!("{:?}", segment::overworld::get_overworld_interaction_result(gb)); true}).execute(gb, sb); println!("{}", sb);
    let sb = IdentifyInputSegment::new().execute(gb, sb);
    sleep(Duration::from_millis(1000));
    sb
  }
}

pub struct YellowTestSegment {}
impl Segment for YellowTestSegment {
  type Rom = Yellow;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<Yellow>, iter: I) -> StateBuffer {
    let sb = DelaySegment::new(MoveSegment::with_check(START, |_gb| true)).execute(gb, iter);
    println!("{}", sb);
    let sb = MoveSegment::new(A).with_max_skips(10).execute(gb, sb);
    println!("{}", sb);
    let sb = MoveSegment::new(START).with_max_skips(10).execute(gb, sb);
    println!("{}", sb);
  //  let sb = MoveSegment::new(A).with_max_skips(10).execute(gb, sb);
    let sb = MoveSegment::new(D).execute(gb, sb); // options
    let sb = MoveSegment::new(L|A).execute(gb, sb); // fast options
    let sb = MoveSegment::new(B).execute(gb, sb); // back
    let sb = MoveSegment::new(A).execute(gb, sb); // new game
    println!("{}", sb);
    let sb = SkipTextsSegment::new(13, B).execute(gb, sb); // skip texts until player name
    let sb = MoveSegment::new(D).execute(gb, sb); // Name: Yellow
    let sb = MoveSegment::new(A).execute(gb, sb); // Name: Yellow
    let sb = SkipTextsSegment::new(5, B).execute(gb, sb); // skip texts until rival name
    let sb = MoveSegment::new(D).execute(gb, sb); // Name: Blue
    let sb = MoveSegment::new(A).execute(gb, sb); // Name: Blue
    let sb = SkipTextsSegment::new(7, B).execute(gb, sb); // skip texts until game start
    let sb = TextSegment::new(A).execute(gb, sb); // ...awaits, let's go
    println!("{}", sb);
    sleep(Duration::from_millis(1000));
    sb
  }
}

pub struct SilverTestSegment {}
impl Segment for SilverTestSegment {
  type Rom = Silver;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<Silver>, iter: I) -> StateBuffer {
    let sb = DelaySegment::new(MoveSegment::with_check(A, |_gb| true)).execute(gb, iter);
    println!("{}", sb);
    let sb = MoveSegment::new(START).with_max_skips(10).execute(gb, sb);
    println!("{}", sb);
    let sb = MoveSegment::new(D).execute(gb, sb); // options
    let sb = MoveSegment::new(L|A).execute(gb, sb); // fast options
    let sb = MoveSegment::new(B).execute(gb, sb); // back
    let sb = MoveSegment::new(A).execute(gb, sb); // new game
    println!("{}", sb);
    let sb = SkipTextsSegment::new(3, B).execute(gb, sb);
    let sb = SkipTextsSegment::new(4, A).execute(gb, sb); // time: 10:..
    let sb = TextSegment::new(A).execute(gb, sb); // overslept
    let sb = MoveSegment::new(B).execute(gb, sb); // overslept
    let sb = SkipTextsSegment::new(17, B).execute(gb, sb); // oak speech
    let sb = MoveSegment::new(D).execute(gb, sb); // Name: Silver
    let sb = MoveSegment::new(A).execute(gb, sb); // Name: Silver
    let sb = SkipTextsSegment::new(7, B).execute(gb, sb); // skip texts until game start
    let sb = TextSegment::new(A).execute(gb, sb); // ... seeing you later
    let sb = IdentifyInputSegment::new().execute(gb, sb);
    println!("{}", sb);
    sleep(Duration::from_millis(1000));
    sb
  }
}

pub struct CrystalTestSegment {}
impl Segment for CrystalTestSegment {
  type Rom = Crystal;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<Crystal>, iter: I) -> StateBuffer {
    let sb = DelaySegment::new(MoveSegment::with_check(A, |_gb| true)).execute(gb, iter);
    println!("{}", sb);
    let sb = MoveSegment::new(START).with_max_skips(10).execute(gb, sb);
    println!("{}", sb);
    let sb = MoveSegment::new(D).execute(gb, sb); // options
    let sb = MoveSegment::new(L|A).execute(gb, sb); // fast options
    let sb = MoveSegment::new(B).execute(gb, sb); // back
    let sb = MoveSegment::new(A).execute(gb, sb); // new game
    println!("{}", sb);
    let sb = SkipTextsSegment::new(1, A).execute(gb, sb); // choose Boy
    let sb = SkipTextsSegment::new(3, B).execute(gb, sb);
    let sb = SkipTextsSegment::new(4, A).execute(gb, sb); // time: 10:..
    let sb = TextSegment::new(A).expect_conflicting_inputs().execute(gb, sb); // overslept
    let sb = MoveSegment::new(B).execute(gb, sb); // overslept
    let sb = SkipTextsSegment::new(17, B).execute(gb, sb); // oak speech
    let sb = MoveSegment::new(D).execute(gb, sb); // Name: Chris
    let sb = MoveSegment::new(A).execute(gb, sb); // Name: Chris
    let sb = SkipTextsSegment::new(7, B).execute(gb, sb); // skip texts until game start
    let sb = TextSegment::new(A).execute(gb, sb); // ... seeing you later
    let sb = IdentifyInputSegment::new().execute(gb, sb);
    println!("{}", sb);
    sleep(Duration::from_millis(1000));
    sb
  }
}





#[allow(dead_code)]
fn convert_efl() {
  let (hi_inputs, lo_inputs) = {
    let mut gb = Gambatte::create_on_screen(0 /* screen */, false /* equal length frames */);
    gb.load_gbc_bios("roms/gbc_bios.bin");
    gb.load_rom("roms/blue.gb");
    ftii::to_ftii::<Blue>(gb, load_inputs("temp/blue_NSC.txt"))
  };

  let inputs = {
    let mut gb = Gambatte::create_on_screen(0 /* screen */, true /* equal length frames */);
    gb.load_gbc_bios("roms/gbc_bios.bin");
    gb.load_rom("roms/blue.gb");
    ftii::from_ftii::<Blue>(gb, hi_inputs, lo_inputs)
  };
  save_inputs("temp/blue_NSC_efl.txt", inputs);
}

fn load_inputs(file_name: &str) -> Vec<Input> {
  let mut result: Vec<Input> = vec![];
  let f = File::open(file_name).expect("file not found");
  let file = BufReader::new(&f);
  for line in file.lines() {
    let l = line.unwrap();
    let mut input = Input::empty();
    if l.contains("D") { input |= Input::DOWN; }
    if l.contains("U") { input |= Input::UP; }
    if l.contains("L") { input |= Input::LEFT; }
    if l.contains("R") { input |= Input::RIGHT; }
    if l.contains("S") { input |= Input::START; }
    if l.contains("s") { input |= Input::SELECT; }
    if l.contains("B") { input |= Input::B; }
    if l.contains("A") { input |= Input::A; }
    result.push(input);
  }
  result
}

fn save_inputs(file_name: &str, inputs: Vec<Input>) {
  let mut f = File::create(file_name).expect("creating file failed");
  for i in inputs {
    f.write_all(format!("|{}{}{}{}{}{}{}{}.|\n",
        if i.contains(Input::UP) {"U"} else {"."},
        if i.contains(Input::DOWN) {"D"} else {"."},
        if i.contains(Input::LEFT) {"L"} else {"."},
        if i.contains(Input::RIGHT) {"R"} else {"."},
        if i.contains(Input::START) {"S"} else {"."},
        if i.contains(Input::SELECT) {"s"} else {"."},
        if i.contains(Input::B) {"B"} else {"."},
        if i.contains(Input::A) {"A"} else {"."}).as_bytes()).unwrap();
  }
}
