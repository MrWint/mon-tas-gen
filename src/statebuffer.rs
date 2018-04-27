use gb::State;
use std::cmp::{max,min};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::iter::FromIterator;

pub const STATE_BUFFER_DEFAULT_MAX_SIZE: usize = 10;
pub const STATE_BUFFER_UNBOUNDED_MAX_SIZE: usize = 256;

#[derive(Serialize, Deserialize)]
pub struct StateBuffer {
  states: HashMap<u32, State>,
  max_size: usize,
}

impl StateBuffer {
  pub fn new() -> Self {
    StateBuffer {
      states: HashMap::new(),
      max_size: STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn with_max_size(max_size: usize) -> Self {
    StateBuffer {
      states: HashMap::new(),
      max_size: max_size,
    }
  }
  pub fn unbounded() -> Self {
    StateBuffer {
      states: HashMap::new(),
      max_size: STATE_BUFFER_UNBOUNDED_MAX_SIZE,
    }
  }
  #[allow(dead_code)]
  pub fn from_iter_unbounded<I: IntoIterator<Item=State>>(iter: I) -> Self {
    let mut sb = StateBuffer::unbounded();
    sb.add_all(iter);
    sb
  }
  #[allow(dead_code)]
  pub fn from_iter_sized<I: IntoIterator<Item=State>>(iter: I, max_size: usize) -> Self {
    let mut sb = StateBuffer::with_max_size(max_size);
    sb.add_all(iter);
    sb
  }

  pub fn add_state(&mut self, s: State) {
    assert!(s.is_at_input, "Invalid state added to StateBuffer!");
    if let Some(old_s) = self.states.get(&s.rng_state) {
      if old_s.frame <= s.frame { return; }
    }
    self.states.insert(s.rng_state, s);
    self.prune();
  }
  pub fn add_all<I: IntoIterator<Item=State>>(&mut self, iter: I) {
    for s in iter.into_iter() { self.add_state(s); }
  }
  fn prune(&mut self) {
    while self.states.len() > self.max_size {
      let mut tbr_key = 0;
      let mut tbr_key_frame = 0;
      let mut tbr_key_metric = 0;
      for s in self.states.values() {
        if s.frame < tbr_key_frame { continue; }
        let s_metric = StateBuffer::get_metric(s);
        let mut metric_diff_sum = 0;
        for s2 in self.states.values() {
          let s2_metric = StateBuffer::get_metric(s2);
          let mx = max(s_metric, s2_metric);
          let mn = min(s_metric, s2_metric);
          let diff = ((min(mx - mn, mn + 0x100 - mx) as f64).sqrt() * 256f64) as u32;
          metric_diff_sum += diff;
        }
        if s.frame > tbr_key_frame || (s.frame == tbr_key_frame && metric_diff_sum < tbr_key_metric) {
          tbr_key = s.rng_state;
          tbr_key_frame = s.frame;
          tbr_key_metric = metric_diff_sum;
        }
      }
      self.states.remove(&tbr_key);
    }
  }
  fn get_metric(s: &State) -> u32 {
    return ((s.rng_state >> 0) + (s.rng_state >> 8)) & 0xff; // DSum
  }

  pub fn is_empty(&self) -> bool {
    self.states.is_empty()
  }
  pub fn len(&self) -> usize {
    self.states.len()
  }
  pub fn is_full(&self) -> bool {
    self.states.len() >= self.max_size
  }

  pub fn save(&self, file_name: &str) {
    let file_path = format!("saves/{}.gz", file_name);
    let f = File::create(file_path).unwrap();
    let f = ::flate2::write::GzEncoder::new(f, ::flate2::Compression::best());
    ::bincode::serialize_into(f, &self).expect("saving statebuffer failed")
  }
  pub fn load(file_name: &str) -> Self {
    let file_path = format!("saves/{}.gz", file_name);
    let f = File::open(file_path).expect("file not found");
    let f = ::flate2::read::GzDecoder::new(f);
    ::bincode::deserialize_from(f).expect("loading statebuffer failed")
  }
}

impl FromIterator<State> for StateBuffer {
  fn from_iter<I: IntoIterator<Item=State>>(iter: I) -> Self {
    let mut sb = StateBuffer::new();
    sb.add_all(iter);
    sb
  }
}

fn _into_iterator_get_value_fn(pair: (u32, State)) -> State { pair.1 }
impl IntoIterator for StateBuffer {
  type Item = State;
  type IntoIter = ::std::iter::Map<::std::collections::hash_map::IntoIter<u32, State>, fn((u32, State)) -> State>;

  fn into_iter(self) -> Self::IntoIter {
    self.states.into_iter().map(_into_iterator_get_value_fn)
  }
}
impl<'a> IntoIterator for &'a StateBuffer {
  type Item = &'a State;
  type IntoIter = ::std::collections::hash_map::Values<'a, u32, State>;

  fn into_iter(self) -> Self::IntoIter {
    self.states.values()
  }
}

impl fmt::Display for StateBuffer {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let max_frame = self.states.values().map(|s| s.frame).max().unwrap_or(0);
    let min_frame = self.states.values().map(|s| s.frame).min().unwrap_or(0);
    let max_dsum = self.states.values().map(|s| StateBuffer::get_metric(s)).max().unwrap_or(0);
    let min_dsum = self.states.values().map(|s| StateBuffer::get_metric(s)).min().unwrap_or(0);

    write!(f, "StateBuffer len {}, frames {}-{}, dsums {:#x}-{:#x}", self.states.len(), min_frame, max_frame, min_dsum, max_dsum)
  }
}
