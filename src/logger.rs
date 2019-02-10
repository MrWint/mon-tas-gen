use log::{Log, LevelFilter, Metadata, Record, SetLoggerError};

struct Logger {}
impl Log for Logger {
  fn enabled(&self, _metadata: &Metadata) -> bool { true }
  fn log(&self, record: &Record) {
    println!(
        "{} {:<5} [{}] {}",
        time::strftime("%Y-%m-%d %H:%M:%S", &time::now()).unwrap(),
        record.level().to_string(),
        record.module_path().unwrap_or_default(),
        record.args());
  }
  fn flush(&self) {}
}

pub fn init_with_level(level_filter: LevelFilter) -> Result<(), SetLoggerError> {
  let logger = Logger { };
  log::set_boxed_logger(Box::new(logger))?;
  log::set_max_level(level_filter);
  Ok(())
}
