use gb::State;
use std::cmp::min;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::iter::FromIterator;
use util::*;

pub const STATE_BUFFER_DEFAULT_MAX_SIZE: usize = 10;
pub const STATE_BUFFER_UNBOUNDED_MAX_SIZE: usize = 256;

/// Collection of ```States``` which are assumed to be at the same logical decision point in the execution.
/// ```StateBuffer```s have a maximum size, and prune excess states if they become too full.
/// The decision which states to prune is made based on the cycle_count and the RNG state of the ```State```.
#[derive(Serialize, Deserialize)]
pub struct StateBuffer {
  /// Maps RNG states to stored ```State```s. If two ```State```s have the same RNG state, they are assumed to exhibit identical future behavior.
  states: HashMap<u32, State>,
  /// Maximum number of ```State```s which can be stored in this buffer.
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

  /// Adds a state to the buffer.
  pub fn add_state(&mut self, s: State) {
    assert!(s.is_at_input, "Invalid state added to StateBuffer!");
    if let Some(old_s) = self.states.get(&s.rng_state) {
      if old_s.cycle_count <= s.cycle_count { return; }
    }
    self.states.insert(s.rng_state, s);
    self.prune();
  }
  /// Adds multiple states to the buffer.
  pub fn add_all<I: IntoIterator<Item=State>>(&mut self, iter: I) {
    for s in iter.into_iter() { self.add_state(s); }
  }
  /// Removes states until it doesn't exceed ```max_size``` anymore.
  /// It successively removes the state with the highest cycle count, or if there are ties the state which is most similar to other states.
  /// The similarity of states is estimated by their D-Sum, where the similarity score for a state s0 is sum_s(sqrt(dsum_difference(s0,s))).
  fn prune(&mut self) {
    while self.states.len() > self.max_size {
      let mut tbr_key = 0;
      let mut tbr_key_metric = ::std::f64::INFINITY;
      let max_cycle_count = self.states.values().map(|s| s.cycle_count).max().unwrap();
      for s in self.states.values() {
        if s.cycle_count < max_cycle_count { continue; }
        let s_dsum = s.get_d_sum();
        let mut dsum_difference_metric = 0.0;
        for s2 in self.states.values() {
          let s2_dsum = s2.get_d_sum();
          let dsum_difference: u8 = min(s_dsum.wrapping_sub(s2_dsum), s2_dsum.wrapping_sub(s_dsum));
          dsum_difference_metric += (dsum_difference as f64).sqrt();
        }
        if dsum_difference_metric < tbr_key_metric {
          tbr_key = s.rng_state;
          tbr_key_metric = dsum_difference_metric;
        }
      }
      self.states.remove(&tbr_key);
    }
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
    let max_cycle_count = self.states.values().map(|s| s.cycle_count).max().unwrap_or(0);
    let min_cycle_count = self.states.values().map(|s| s.cycle_count).min().unwrap_or(0);
    let max_dsum = self.states.values().map(|s| s.get_d_sum()).max().unwrap_or(0);
    let min_dsum = self.states.values().map(|s| s.get_d_sum()).min().unwrap_or(0);

    write!(f, "StateBuffer len {}, times {}-{}, dsums {:#x}-{:#x}", self.states.len(), to_human_readable_time(min_cycle_count), to_human_readable_time(max_cycle_count), min_dsum, max_dsum)
  }
}
