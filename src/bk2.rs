use gambatte::*;
use crate::rom::*;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use zip::write::FileOptions;

pub struct Bk2Writer {
  author: String,
  game_name: String,
  sha1: String,
  board_name: String,
  equal_length_frames: bool,
  rtc_divisor_offset: i32,
}
impl Bk2Writer {
  pub fn new<R: BasicRomInfo>() -> Self {
    Bk2Writer {
      author: "MrWint".to_owned(),
      game_name: R::GAME_NAME.to_owned(),
      sha1: R::SHA1.to_owned(),
      board_name: R::BOARD_NAME.to_owned(),
      equal_length_frames: false,
      rtc_divisor_offset: 0,
    }
  }
  pub fn with_equal_length_frames(self, equal_length_frames: bool) -> Self {
    Bk2Writer { equal_length_frames, ..self }
  }
  pub fn with_rtc_divisor_offset(self, rtc_divisor_offset: i32) -> Self {
    Bk2Writer { rtc_divisor_offset, ..self }
  }

  pub fn write_bk2(&self, file_name: &str, inputs: &[Input]) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(file_name);
    let file = std::fs::File::create(&path).unwrap();

    let mut zip = zip::ZipWriter::new(file);

    zip.start_file("Comments.txt", FileOptions::default())?;
    zip.write_all(b"\r\n")?;

    zip.start_file("Subtitles.txt", FileOptions::default())?;
    zip.write_all(b"\r\n")?;

    zip.start_file("SyncSettings.json", FileOptions::default())?;
    zip.write_all(br#"{"o":{"$type":"BizHawk.Emulation.Cores.Nintendo.Gameboy.Gameboy+GambatteSyncSettings, BizHawk.Emulation.Cores","ConsoleMode":2,"GBACGB":true,"MulticartCompat":false,"RealTimeRTC":false,"EqualLengthFrames":"#)?;
    zip.write_all(if self.equal_length_frames { b"true" } else { b"false" })?;
    zip.write_all(br#","RTCDivisorOffset":"#)?;
    zip.write_all(format!("{}", self.rtc_divisor_offset).as_bytes())?;
    zip.write_all(br#"}}"#)?;
    zip.write_all(b"\r\n")?;

    zip.start_file("Header.txt", FileOptions::default())?;
    zip.write_all(b"MovieVersion BizHawk v2.0.0\r\n")?;
    zip.write_all(format!("Author {}\r\n", self.author).as_bytes())?;
    zip.write_all(b"emuVersion Version 2.3.0\r\n")?;
    zip.write_all(b"Platform GB\r\n")?;
    zip.write_all(format!("GameName {}\r\n", self.game_name).as_bytes())?;
    zip.write_all(format!("SHA1 {}\r\n", self.sha1).as_bytes())?;
    zip.write_all(format!("BoardName {}\r\n", self.board_name).as_bytes())?;
    zip.write_all(b"GBC_Firmware_World 1293D68BF9643BC4F36954C1E80E38F39864528D\r\n")?;
    zip.write_all(b"IsCGBMode 1\r\n")?;
    zip.write_all(b"Core Gambatte\r\n")?;

    zip.start_file("Input Log.txt", FileOptions::default())?;
    zip.write_all(b"[Input]\r\n")?;
    zip.write_all(b"LogKey:#Up|Down|Left|Right|Start|Select|B|A|Power|\r\n")?;
    for i in inputs {
      zip.write_all(format!("|{}{}{}{}{}{}{}{}.|\r\n",
          if i.contains(Input::UP) {"U"} else {"."},
          if i.contains(Input::DOWN) {"D"} else {"."},
          if i.contains(Input::LEFT) {"L"} else {"."},
          if i.contains(Input::RIGHT) {"R"} else {"."},
          if i.contains(Input::START) {"S"} else {"."},
          if i.contains(Input::SELECT) {"s"} else {"."},
          if i.contains(Input::B) {"B"} else {"."},
          if i.contains(Input::A) {"A"} else {"."}).as_bytes())?;
    }

    zip.finish()?;
    Ok(())
  }
}

pub fn read_bk2_inputs(file_name: &str) -> zip::result::ZipResult<Vec<Input>> {
  let path = std::path::Path::new(file_name);
  let file = std::fs::File::open(&path).unwrap();
  let mut archive = zip::ZipArchive::new(file)?;

  let mut result: Vec<Input> = vec![];
  let file = archive.by_name("Input Log.txt")?;
  let file = BufReader::new(file);
  for line in file.lines() {
    let l = line.unwrap();
    if !l.starts_with('|') { continue; }
    let mut input = Input::empty();
    if l.contains('D') { input |= Input::DOWN; }
    if l.contains('U') { input |= Input::UP; }
    if l.contains('L') { input |= Input::LEFT; }
    if l.contains('R') { input |= Input::RIGHT; }
    if l.contains('S') { input |= Input::START; }
    if l.contains('s') { input |= Input::SELECT; }
    if l.contains('B') { input |= Input::B; }
    if l.contains('A') { input |= Input::A; }
    result.push(input);
  }
  Ok(result)
}
