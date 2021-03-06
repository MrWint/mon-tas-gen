
pub(crate) fn to_human_readable_time(cycle_counter: u64) -> String {
  let num_hours = cycle_counter / (1 << 21) / 3600;
  let num_minutes = (cycle_counter / (1 << 21) / 60) % 60;
  let num_seconds = (cycle_counter / (1 << 21)) % 60;
  let num_millis = (cycle_counter * 1000 / (1 << 21)) % 1000;

  let num_frames = cycle_counter / 35112;
  let num_sub_frames = cycle_counter % 35112;
  format!("{:02}:{:02}:{:02}.{:03} ({}:{:05})", num_hours, num_minutes, num_seconds, num_millis, num_frames, num_sub_frames)
}

pub fn with_log_level<V, F: FnOnce() -> V>(log_level: log::LevelFilter, f: F) -> V {
  let old_log_level = log::max_level();
  log::set_max_level(log_level);
  let value = f();
  log::set_max_level(old_log_level);
  value
}