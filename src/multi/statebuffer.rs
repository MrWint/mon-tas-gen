use serde_derive::{Serialize, Deserialize};

use std::cmp::{max, Ordering};
use std::collections::hash_map::{HashMap, Entry};
use std::iter::FromIterator;
use super::*;
use crate::big_array::BigArray;

pub const MULTI_STATE_BUFFER_DEFAULT_MAX_SIZE: usize = 16;

#[derive(Clone, Serialize, Deserialize)]
pub struct MultiStateItem {
  pub gb_state: std::rc::Rc<GbState>,
  pub plan_state: PlanState,

  /// Derived: whether the plan state is considered safe.
  is_safe: bool,
}
impl MultiStateItem {
  pub fn new(gb_state: GbState, plan_state: PlanState, is_safe: bool) -> Self {
    Self {
      gb_state: std::rc::Rc::new(gb_state),
      plan_state,
      is_safe,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct MultiState<const N: usize> {
  /// List of states for the individual instances.
  #[serde(with = "BigArray")]
  pub instances: [MultiStateItem; N],
  /// List of inputs that led to this state.
  pub inputs: InputLog,
}
impl<const N: usize> MultiState<N> {
  pub fn new(instances: [MultiStateItem; N], inputs: InputLog) -> Self {
    for s in instances.iter() {
      assert!(s.gb_state.is_at_input(), "Invalid state added to MultiState!");
    }
    Self {
      instances,
      inputs,
    }
  }
  /// Computes fingerprint of the RNG states of the individual instances.
  fn get_rng_fingerprint(&self) -> u64 {
    let mut rng_fingerprint = 0;
    for s in self.instances.iter() {
      rng_fingerprint = rng_fingerprint * 1_228_782_641 + u64::from(s.gb_state.rng_state);
    }
    rng_fingerprint
  }
  /// Checks whether all instances are at a safe state.
  pub fn is_safe(&self) -> bool {
    self.instances.iter().all(|instance| instance.is_safe)
  }
  /// Checks whether the current state is strictly earlier than the given state.
  pub fn compare_plans(&self, other: &Self) -> Option<Ordering> {
    let mut has_ahead_instance = false;
    let mut has_behind_instance = false;
    for (i, oi) in self.instances.iter().zip(other.instances.iter()) {
      match i.plan_state.partial_cmp(&oi.plan_state)? {
        Ordering::Greater => has_ahead_instance = true,
        Ordering::Less => has_behind_instance = true,
        _ => {}
      }
      if has_ahead_instance && has_behind_instance {
        return None;
      }
    }
    if has_ahead_instance { Some(Ordering::Greater) }
    else if has_behind_instance { Some(Ordering::Less) }
    else { Some(Ordering::Equal) }
  }
  /// Returns the frame at which the next instance can make progress (i.e. both input nybbles are defined).
  pub fn get_next_input_frame(&self) -> u32 {
    self.instances.iter().map(|instance| max(instance.gb_state.get_input_frame_lo(), instance.gb_state.get_input_frame_hi())).min().unwrap()
  }
}

#[derive(Serialize, Deserialize)]
pub struct MultiStateBuffer<const N: usize> {
  states: HashMap<u64, MultiState<N>>,
  // for each state, count number of states more advanced than this, and frame count
  metrics: HashMap<u64, (u32, u32)>,
  max_size: usize,
}
impl<const N: usize> Default for MultiStateBuffer<N> {
  fn default() -> Self { Self::with_max_size(MULTI_STATE_BUFFER_DEFAULT_MAX_SIZE) }
}
impl<const N: usize> MultiStateBuffer<N> {
  pub fn new() -> Self { Default::default() }
  pub fn with_max_size(max_size: usize) -> Self {
    MultiStateBuffer {
      states: HashMap::with_capacity(0), // don't allocate.
      metrics: HashMap::with_capacity(0), // don't allocate.
      max_size,
    }
  }

  pub fn from_iter_sized<I: IntoIterator<Item=MultiState<N>>>(iter: I, max_size: usize) -> Self {
    let mut sb = MultiStateBuffer::with_max_size(max_size);
    sb.add_all(iter);
    sb
  }

  /// Adds a state to the buffer.
  pub fn add_state(&mut self, s: MultiState<N>) {
    if self.states.capacity() == 0 {
      self.states.reserve(self.max_size + 1); // Reserve one additional element to hold excess before pruning.
      self.metrics.reserve(self.max_size + 1); // Reserve one additional element to hold excess before pruning.
    }
    let rng_fingerprint = s.get_rng_fingerprint();
    let frame_count = s.inputs.len_max();
    match self.states.entry(rng_fingerprint) {
      Entry::Occupied(mut entry) => {
        let other_frame_count = entry.get().inputs.len_max();
        if other_frame_count > frame_count {
          log::debug!("Replacing state with RNG fingerprint {:x} ({} -> {} frames)", rng_fingerprint, other_frame_count, frame_count);
          let old_state = entry.insert(s);
          self.metrics_remove(rng_fingerprint, old_state);
          self.metrics_add(rng_fingerprint, frame_count);
        }
      }
      Entry::Vacant(entry) => {
        entry.insert(s);
        self.metrics_add(rng_fingerprint, frame_count);
        self.prune();
      }
    }
  }
  /// Adds multiple states to the buffer.
  pub fn add_all<I: IntoIterator<Item=MultiState<N>>>(&mut self, iter: I) {
    for s in iter.into_iter() { self.add_state(s); }
  }
  fn metrics_add(&mut self, rng_fingerprint: u64, frame_count: u32) {
    let s = self.states.get(&rng_fingerprint).unwrap();

    let mut num_better_states = 0;
    for (rng, os) in self.states.iter() {
      if rng_fingerprint != *rng {
        match s.compare_plans(os) {
          Some(Ordering::Less) => num_better_states += 1,
          Some(Ordering::Greater) => self.metrics.get_mut(rng).unwrap().0 += 1,
          _ => {}
        }
      }
    }
    self.metrics.insert(rng_fingerprint, (num_better_states, frame_count));
  }
  fn metrics_remove(&mut self, rng_fingerprint: u64, s: MultiState<N>) {
    let (old_num_better_states, _) = self.metrics.remove(&rng_fingerprint).unwrap();
    let mut num_better_states = 0;
    for (rng, os) in self.states.iter() {
      if rng_fingerprint != *rng {
        match s.compare_plans(os) {
          Some(Ordering::Less) => num_better_states += 1,
          Some(Ordering::Greater) => self.metrics.get_mut(rng).unwrap().0 -= 1,
          _ => {}
        }
      }
    }
    assert!(old_num_better_states == num_better_states);
  }
  /// Removes states until it doesn't exceed ```max_size``` anymore.
  fn prune(&mut self) {
    assert!(self.states.len() <= self.max_size + 1);
    if self.states.len() > self.max_size {
      let (&tbr_key, (num_better_states, _)) = self.metrics.iter().max_by_key(|e| e.1).unwrap();
      log::debug!("Pruning state with {} better states", num_better_states);
      let old_state = self.states.remove(&tbr_key).unwrap();
      self.metrics_remove(tbr_key, old_state);
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
  pub fn iter<'a>(&'a self) -> std::collections::hash_map::Values<'a, u64, MultiState<N>> {
    self.into_iter()
  }
}
impl<const N: usize> FromIterator<MultiState<N>> for MultiStateBuffer<N> {
  fn from_iter<I: IntoIterator<Item=MultiState<N>>>(iter: I) -> Self {
    let mut sb = MultiStateBuffer::new();
    sb.add_all(iter);
    sb
  }
}

impl<const N: usize> IntoIterator for MultiStateBuffer<N> {
  type Item = MultiState<N>;
  type IntoIter = std::iter::Map<std::collections::hash_map::IntoIter<u64, MultiState<N>>, fn((u64, MultiState<N>)) -> MultiState<N>>;

  fn into_iter(self) -> Self::IntoIter {
    self.states.into_iter().map(|(_, s)| s)
  }
}
impl<'a, const N: usize> IntoIterator for &'a MultiStateBuffer<N> {
  type Item = &'a MultiState<N>;
  type IntoIter = std::collections::hash_map::Values<'a, u64, MultiState<N>>;

  fn into_iter(self) -> Self::IntoIter {
    self.states.values()
  }
}
