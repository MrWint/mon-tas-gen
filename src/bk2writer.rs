use gambatte::*;
use montas::rom::*;
use std::io::Write;
use zip::write::FileOptions;

pub(crate) enum Bk2Writer {}
impl Bk2Writer {
  pub(crate) fn write_bk2<R: BasicRomInfo>(file_name: &str, inputs: &Vec<Input>) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(file_name);
    let file = std::fs::File::create(&path).unwrap();

    let mut zip = zip::ZipWriter::new(file);

    zip.start_file("Comments.txt", FileOptions::default())?;
    zip.write_all(b"\r\n")?;

    zip.start_file("Subtitles.txt", FileOptions::default())?;
    zip.write_all(b"\r\n")?;

    zip.start_file("SyncSettings.json", FileOptions::default())?;
    zip.write_all(br#"{"o":{"$type":"BizHawk.Emulation.Cores.Nintendo.Gameboy.Gameboy+GambatteSyncSettings, BizHawk.Emulation.Cores","ConsoleMode":2,"GBACGB":true,"MulticartCompat":false,"RealTimeRTC":false,"RTCInitialTime":0,"EqualLengthFrames":false,"InitialDiv":0}}"#)?;
    zip.write_all(b"\r\n")?;

    zip.start_file("Header.txt", FileOptions::default())?;
    zip.write_all(b"MovieVersion BizHawk v2.0.0\r\n")?;
    zip.write_all(b"Author MrWint\r\n")?;
    zip.write_all(b"emuVersion Version 2.3.0\r\n")?;
    zip.write_all(b"Platform GB\r\n")?;
    zip.write_all(format!("GameName {}\r\n", R::GAME_NAME).as_bytes())?;
    zip.write_all(format!("SHA1 {}\r\n", R::SHA1).as_bytes())?;
    zip.write_all(format!("BoardName {}\r\n", R::BOARD_NAME).as_bytes())?;
    zip.write_all(b"GBC_Firmware_World 1293D68BF9643BC4F36954C1E80E38F39864528D\r\n")?;
    zip.write_all(b"IsCGBMode 1\r\n")?;
    zip.write_all(b"Core Gambatte\r\n")?;

    zip.start_file("Input Log.txt", FileOptions::default())?;
    for i in inputs {
      zip.write_all(format!("|{}{}{}{}{}{}{}{}.|\n",
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