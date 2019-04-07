use crate::gb::State;
use crate::util::*;
use serde_derive::{Serialize, Deserialize};
use std::cmp::min;
use std::collections::hash_map::{HashMap, Entry};
use std::fmt;
use std::fs::File;
use std::iter::FromIterator;
use std::marker::PhantomData;

pub const STATE_BUFFER_DEFAULT_MAX_SIZE: usize = 16;
pub const STATE_BUFFER_UNBOUNDED_MAX_SIZE: usize = STATE_BUFFER_DEFAULT_MAX_SIZE * 16; // 4096;

/// Collection of ```States``` which are assumed to be at the same logical decision point in the execution.
/// ```StateBuffer```s have a maximum size, and prune excess states if they become too full.
/// The decision which states to prune is made based on the cycle_count and the RNG state of the ```State```,
/// preferring a diverse collection of states with minimal cycles.
#[derive(Serialize, Deserialize)]
pub struct StateBuffer<V = (), S: StateMetric = DSumStateMetric> {
  /// Maps RNG states to stored ```State```s. If two ```State```s have the same RNG state, they are assumed to exhibit identical future behavior.
  states: HashMap<u32, State<V>>,
  metrics: HashMap<u32, (u64, u32)>,
  /// Maximum number of ```State```s which can be stored in this buffer.
  max_size: usize,
  phantom_data: PhantomData<S>,
}

impl<V, S: StateMetric> Default for StateBuffer<V, S> {
  fn default() -> Self { Self::with_max_size(STATE_BUFFER_DEFAULT_MAX_SIZE) }
}
impl <V: serde::Serialize + serde::de::DeserializeOwned, S: StateMetric> StateBuffer<V, S> {
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
impl<V, S: StateMetric> StateBuffer<V, S> {
  pub fn new() -> Self { Default::default() }
  pub fn unbounded() -> Self { Self::with_max_size(STATE_BUFFER_UNBOUNDED_MAX_SIZE) }
  pub fn with_max_size(max_size: usize) -> Self {
    StateBuffer {
      states: HashMap::with_capacity(0), // don't allocate.
      metrics: HashMap::with_capacity(0), // don't allocate.
      max_size,
      phantom_data: PhantomData,
    }
  }

  pub fn from_iter_unbounded<I: IntoIterator<Item=State<V>>>(iter: I) -> Self {
    let mut sb = StateBuffer::unbounded();
    sb.add_all(iter);
    sb
  }
  pub fn from_iter_sized<I: IntoIterator<Item=State<V>>>(iter: I, max_size: usize) -> Self {
    let mut sb = StateBuffer::with_max_size(max_size);
    sb.add_all(iter);
    sb
  }

  /// Adds a state to the buffer.
  pub fn add_state(&mut self, s: State<V>) {
    assert!(s.is_at_input, "Invalid state added to StateBuffer!");
    if self.states.capacity() == 0 {
      self.states.reserve(self.max_size + 1); // Reserve one additional element to hold excess before pruning.
      self.metrics.reserve(self.max_size + 1); // Reserve one additional element to hold excess before pruning.
    }
    let cycle_count = s.cycle_count;
    let rng_state = s.rng_state;
    match self.states.entry(rng_state) {
      Entry::Occupied(mut entry) => {
        if entry.get().cycle_count > s.cycle_count { entry.insert(s); }
        // metrics don't need to be updated.
      }
      Entry::Vacant(entry) => {
        entry.insert(s);
        self.metrics_add(cycle_count, rng_state);
        self.prune();
      }
    }
  }
  fn metrics_add(&mut self, cycle_count: u64, rng_state: u32) {
    let state_metric = S::new(rng_state);
    let mut new_metric_sum = 0;
    for (rng, (_, ref mut metric_sum)) in self.metrics.iter_mut() {
      let metric = state_metric.get_metric(*rng);
      *metric_sum += metric;
      new_metric_sum += metric;
    }
    self.metrics.insert(rng_state, (cycle_count, new_metric_sum));
  }
  fn metrics_remove(&mut self, rng_state: u32) {
    let (_, old_metric_sum) = self.metrics.remove(&rng_state).unwrap();
    let state_metric = S::new(rng_state);
    let mut new_metric_sum = 0;
    for (rng, (_, ref mut metric_sum)) in self.metrics.iter_mut() {
      let metric = state_metric.get_metric(*rng);
      *metric_sum -= metric;
      new_metric_sum += metric;
    }
    assert!(old_metric_sum == new_metric_sum);
  }

  /// Adds multiple states to the buffer.
  pub fn add_all<I: IntoIterator<Item=State<V>>>(&mut self, iter: I) {
    for s in iter.into_iter() { self.add_state(s); }
  }
  /// Removes states until it doesn't exceed ```max_size``` anymore.
  /// It successively removes the state with the highest cycle count, or if there are ties the state which is most similar to other states.
  /// The similarity of states is estimated by their D-Sum, where the similarity score for a state s0 is sum_s(sqrt(dsum_difference(s0,s))).
  fn prune(&mut self) {
    while self.states.len() > self.max_size {
      let (&tbr_key, _) = self.metrics.iter().min_by(|(_, (cycle_1, sum_1)), (_, (cycle_2, sum_2))| cycle_2.cmp(cycle_1).then(sum_1.cmp(sum_2))).unwrap();
      let result = self.states.remove(&tbr_key);
      assert!(result.is_some());
      self.metrics_remove(tbr_key);
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
  pub fn get_max_size(&self) -> usize {
    self.max_size
  }
}

impl<V, S: StateMetric> FromIterator<State<V>> for StateBuffer<V, S> {
  fn from_iter<I: IntoIterator<Item=State<V>>>(iter: I) -> Self {
    let mut sb = StateBuffer::new();
    sb.add_all(iter);
    sb
  }
}

impl<V, S: StateMetric> IntoIterator for StateBuffer<V, S> {
  type Item = State<V>;
  #[allow(clippy::type_complexity)]
  type IntoIter = std::iter::Map<std::collections::hash_map::IntoIter<u32, State<V>>, fn((u32, State<V>)) -> State<V>>;

  fn into_iter(self) -> Self::IntoIter {
    self.states.into_iter().map(|(_, s)| s)
  }
}
impl<'a, V, S: StateMetric> IntoIterator for &'a StateBuffer<V, S> {
  type Item = &'a State<V>;
  type IntoIter = std::collections::hash_map::Values<'a, u32, State<V>>;

  fn into_iter(self) -> Self::IntoIter {
    self.states.values()
  }
}

impl<V, S: StateMetric> fmt::Display for StateBuffer<V, S> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let max_cycle_count = self.states.values().map(|s| s.cycle_count).max().unwrap_or(0);
    let min_cycle_count = self.states.values().map(|s| s.cycle_count).min().unwrap_or(0);
    let max_dsum = self.states.values().map(|s| s.get_d_sum()).max().unwrap_or(0);
    let min_dsum = self.states.values().map(|s| s.get_d_sum()).min().unwrap_or(0);
    let max_div = self.states.values().map(|s| s.get_div_state()).max().unwrap_or(0);
    let min_div = self.states.values().map(|s| s.get_div_state()).min().unwrap_or(0);

    write!(f, "StateBuffer len {}, times {}-{}, dsums {:#x}-{:#x}, divs {:#x}-{:#x}, dsum graph: ", self.states.len(), to_human_readable_time(min_cycle_count), to_human_readable_time(max_cycle_count), min_dsum, max_dsum, min_div, max_div)?;
    for i in 0..=255 {
      write!(f, "{}", if self.states.values().map(|s| s.get_d_sum()).any(|v| v == i) { '|' } else { '.' })?;
    }
    Ok(())
  }
}


pub trait StateMetric {
  fn new(rng_state: u32) -> Self;
  fn get_metric(&self, rng_state: u32) -> u32;
}
pub struct DSumStateMetric(u8);
impl DSumStateMetric {
  #[inline] fn dsum(rng_state: u32) -> u8 { (rng_state + (rng_state >> 8)) as u8 }
}
impl StateMetric for DSumStateMetric {
  fn new(rng_state: u32) -> Self {
    DSumStateMetric(Self::dsum(rng_state))
  }
  fn get_metric(&self, rng_state: u32) -> u32 {
    let dsum = Self::dsum(rng_state);
    let dsum_difference: u8 = min(self.0.wrapping_sub(dsum), dsum.wrapping_sub(self.0));
    f64::from(u32::from(dsum_difference) << 24).sqrt() as u32
  }
}
pub struct DivStateStateMetric(u16);
impl DivStateStateMetric {
  #[inline] fn div_state(rng_state: u32) -> u16 { (rng_state >> 16) as u16 }
}
impl StateMetric for DivStateStateMetric {
  fn new(rng_state: u32) -> Self {
    DivStateStateMetric(Self::div_state(rng_state))
  }
  fn get_metric(&self, rng_state: u32) -> u32 {
    let div_state = Self::div_state(rng_state);
    let div_state_difference: u16 = min(div_state.wrapping_sub(self.0) & 0x3fff, self.0.wrapping_sub(div_state) & 0x3fff);
    f64::from(u32::from(div_state_difference) << 18).sqrt() as u32
  }
}
