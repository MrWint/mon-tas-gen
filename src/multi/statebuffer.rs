use serde_derive::{Serialize, Deserialize};

use std::{cmp::Ordering, collections::BTreeMap};
use std::collections::hash_map::HashMap;
use super::*;
use crate::big_array::BigArray;

pub const MULTI_STATE_BUFFER_SINGLE_RNG_MAX_SIZE: usize = 4; // how many states with the same RNG value are kept at most.
pub const MULTI_STATE_BUFFER_DEFAULT_MAX_SIZE: usize = 64;
pub const MULTI_STATE_BUFFER_HARD_LIMIT_MAX_SIZE: usize = MULTI_STATE_BUFFER_DEFAULT_MAX_SIZE * 16;

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
  pub fn new_rc(gb_state: std::rc::Rc<GbState>, plan_state: PlanState, is_safe: bool) -> Self {
    Self {
      gb_state,
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
    self.instances.iter().map(|instance| instance.gb_state.get_input_frame_lo()).min().unwrap()
  }

  fn get_blocked_inputs(&self) -> BlockedInputs<N> {
    let mut result = BlockedInputs([Input::empty(); N]);
    for i in 0..N {
      result.0[i] = self.instances[i].gb_state.blocked_inputs;
    }
    result
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockedInputs<const N: usize>(#[serde(with = "BigArray")] [Input; N]);
impl<const N: usize> BlockedInputs<N> {
  fn contains(&self, other: &BlockedInputs<N>) -> bool {
    self.0.iter().zip(other.0.iter()).all(|(&s, &o)| s.contains(o))
  }
}

#[derive(Default, Serialize, Deserialize)]
pub struct StateBuffer<const N: usize> {
  frame_buffers: BTreeMap<u32, MultiStateBuffer<N>>,
}
impl<const N: usize> StateBuffer<N> {
  pub fn new() -> Self {
    Self { frame_buffers: BTreeMap::new() }
  }
  pub fn is_empty(&self) -> bool {
    self.frame_buffers.is_empty()
  }
  pub fn get_min_input_frame(&self) -> Option<u32> {
    self.frame_buffers.iter().next().map(|x| *x.0)
  }
  pub fn get_max_input_frame(&self) -> Option<u32> {
    self.frame_buffers.iter().rev().next().map(|x| *x.0)
  }
  pub fn fetch_min_input_frame_states(&mut self) -> MultiStateBuffer<N> {
    let min_input_frame = self.get_min_input_frame().expect("StateBuffer is empty");
    self.frame_buffers.remove(&min_input_frame).unwrap()
  }
  pub fn count_states(&self) -> usize {
    self.frame_buffers.values().map(|buffer| buffer.len()).sum()
  }
  pub fn add_state(&mut self, s: MultiState<N>) {
    let buffer = self.frame_buffers.entry(s.get_next_input_frame()).or_default();
    buffer.add_state(s);
    // if self.count_states() > MULTI_STATE_BUFFER_HARD_LIMIT_MAX_SIZE {
    //   for (_, buf) in self.frame_buffers.iter_mut() {
    //     if buf.len() > 1 {
    //       buf.prune();
    //       break;
    //     }
    //   }
    // }
  }
  pub fn iter(&self) -> std::iter::Flatten<std::collections::btree_map::Values<'_, u32, MultiStateBuffer<N>>> {
    self.frame_buffers.values().flatten()
  }
}

#[derive(Serialize, Deserialize)]
pub struct MultiStateBuffer<const N: usize> {
  blocked_inputs: HashMap<u64, Vec<BlockedInputs<N>>>,
  states: HashMap<(u64, BlockedInputs<N>), MultiState<N>>,
  // for each state, count number of states more advanced than this, the number of states that are comparable to this, and frame count
  metrics: HashMap<(u64, BlockedInputs<N>), (u32, u32, u32)>,
  max_size: usize,
}
impl<const N: usize> Default for MultiStateBuffer<N> {
  fn default() -> Self { Self::with_max_size(MULTI_STATE_BUFFER_DEFAULT_MAX_SIZE) }
}
impl<const N: usize> MultiStateBuffer<N> {
  fn with_max_size(max_size: usize) -> Self {
    MultiStateBuffer {
      blocked_inputs: HashMap::with_capacity(0), // don't allocate.
      states: HashMap::with_capacity(0), // don't allocate.
      metrics: HashMap::with_capacity(0), // don't allocate.
      max_size,
    }
  }

  /// Adds a state to the buffer.
  fn add_state(&mut self, s: MultiState<N>) {
    if self.states.capacity() == 0 {
      self.states.reserve(self.max_size + 1); // Reserve one additional element to hold excess before pruning.
      self.metrics.reserve(self.max_size + 1); // Reserve one additional element to hold excess before pruning.
    }
    let rng_fingerprint = s.get_rng_fingerprint();
    let blocked_inputs = s.get_blocked_inputs();
    let frame_count = s.get_next_input_frame();
    let num_better_states = self.states.iter().filter(|(_, os)| s.compare_plans(os) == Some(Ordering::Less)).count() as u32;
    let mut num_comparable_states = self.states.iter().filter(|(_, os)| s.compare_plans(os).is_some()).count() as u32;
    let blocked_input_list = self.blocked_inputs.entry(rng_fingerprint).or_insert(Vec::new());
    { // Compare to existing states with the same rng fingerprint
      let mut i = 0;
      while i < blocked_input_list.len() {
        let other_key = (rng_fingerprint, blocked_input_list[i].clone());
        let &(mut other_num_better_states, mut other_num_comparable_states, other_frame_count) = self.metrics.get(&other_key).unwrap();
        // Account for new state as if it were already added to the buffer.
        if s.compare_plans(self.states.get(&other_key).unwrap()) == Some(Ordering::Greater) { other_num_better_states += 1; }
        if s.compare_plans(self.states.get(&other_key).unwrap()).is_some() { other_num_comparable_states += 1; }
        // Try to determine a clearly superior state.
        match num_better_states.cmp(&other_num_better_states).then(num_comparable_states.cmp(&other_num_comparable_states)).then(frame_count.cmp(&other_frame_count)).then_with(|| if blocked_inputs.contains(&blocked_input_list[i]) { Ordering::Greater } else if blocked_input_list[i].contains(&blocked_inputs) { Ordering::Less } else { Ordering::Equal }) {
          Ordering::Less => {
            // Stored state is worse than current state, remove
            let old_blocked_input = blocked_input_list.swap_remove(i);
            log::trace!("For RNG fingerprint {:x}, removing blocked inputs {:?} which are worse than new {:?}", rng_fingerprint, old_blocked_input, blocked_inputs);
            let tbr_key = (rng_fingerprint, old_blocked_input);
            assert!(other_key == tbr_key);
            let old_state = self.states.remove(&tbr_key).unwrap();
            { // Update metrics
              let (old_num_better_states, old_num_comparable_states, _) = self.metrics.remove(&tbr_key).unwrap();
              let mut assert_num_better_states = 0;
              let mut assert_num_comparable_states = 0;
              for (key, os) in self.states.iter() {
                let cmp = old_state.compare_plans(os);
                if cmp.is_some() {
                  self.metrics.get_mut(key).unwrap().1 -= 1;
                  assert_num_comparable_states += 1;
                }
                match cmp {
                  Some(Ordering::Less) => assert_num_better_states += 1,
                  Some(Ordering::Greater) => self.metrics.get_mut(key).unwrap().0 -= 1,
                  _ => {}
                }
              }
              assert!(old_num_better_states == assert_num_better_states);
              assert!(old_num_comparable_states == assert_num_comparable_states);
              // States removed here can't affect the num_better_states of the current item, because otherwise the current item would have been dropped instead by transitivity of the relation.
              assert!(s.compare_plans(&old_state) != Some(Ordering::Less));
              if old_state.compare_plans(&s).is_some() { num_comparable_states -= 1; }
            }
          },
          Ordering::Greater => {
            // current state is worse than stored, drop
            log::trace!("For RNG fingerprint {:x}, dropping new blocked inputs {:?} which are worse than {:?}", rng_fingerprint, blocked_inputs, blocked_input_list[i]);
            return;
          },
          _ => { i += 1; },
        }
      }
    }
    if blocked_input_list.len() >= MULTI_STATE_BUFFER_SINGLE_RNG_MAX_SIZE {
      // There is no room to add this
      log::debug!("Too many states with RNG fingerprint {:x}, dropping state with blocked inputs {:?}", rng_fingerprint, blocked_inputs);
      return;
    }
    blocked_input_list.push(blocked_inputs.clone());
    drop(blocked_input_list); // Stop mut borrow
    let inserted_key = (rng_fingerprint, blocked_inputs);
    { // Update metrics
      let mut assert_num_better_states = 0;
      let mut assert_num_comparable_states = 0;
      for (key, os) in self.states.iter() {
        let cmp = s.compare_plans(os);
        if cmp.is_some() {
          self.metrics.get_mut(key).unwrap().1 += 1;
          assert_num_comparable_states += 1;
        }
        match cmp {
          Some(Ordering::Less) => assert_num_better_states += 1,
          Some(Ordering::Greater) => self.metrics.get_mut(key).unwrap().0 += 1,
          _ => {}
        }
      }
      assert!(num_better_states == assert_num_better_states);
      assert!(num_comparable_states == assert_num_comparable_states);
      self.metrics.insert(inserted_key.clone(), (num_better_states, num_comparable_states, frame_count));
    }
    self.states.insert(inserted_key, s);
    if self.states.len() > self.max_size {
      self.prune();
    }
  }
  /// Removes states until it doesn't exceed ```max_size``` anymore.
  fn prune(&mut self) {
    assert!(self.states.len() <= self.max_size + 1);
    let (tbr_key, (num_better_states, num_comparable_states, _)) = self.metrics.iter().max_by_key(|e| e.1).unwrap();
    let tbr_key = tbr_key.clone();
    log::trace!("Pruning state with {} better states and {} comparable states", num_better_states, num_comparable_states);
    let old_state = self.states.remove(&tbr_key).unwrap();
    { // Update metrics
      let (old_num_better_states, old_num_comparable_states, _) = self.metrics.remove(&tbr_key).unwrap();
      let mut assert_num_better_states = 0;
      let mut assert_num_comparable_states = 0;
      for (key, os) in self.states.iter() {
        let cmp = old_state.compare_plans(os);
        if cmp.is_some() {
          self.metrics.get_mut(key).unwrap().1 -= 1;
          assert_num_comparable_states += 1;
        }
        match cmp {
          Some(Ordering::Less) => assert_num_better_states += 1,
          Some(Ordering::Greater) => self.metrics.get_mut(key).unwrap().0 -= 1,
          _ => {}
        }
      }
      assert!(old_num_better_states == assert_num_better_states);
      assert!(old_num_comparable_states == assert_num_comparable_states);
    }
    let blocked_input_list = self.blocked_inputs.get_mut(&tbr_key.0).unwrap();
    let blocked_input_list_pos = blocked_input_list.iter().position(|x| *x == tbr_key.1).unwrap();
    blocked_input_list.swap_remove(blocked_input_list_pos);
  }

  pub fn len(&self) -> usize {
    self.states.len()
  }
  pub fn iter<'a>(&'a self) -> std::collections::hash_map::Values<'a, (u64, BlockedInputs<N>), MultiState<N>> {
    self.into_iter()
  }
  pub fn into_sorted_iter(self) -> std::iter::Map<std::vec::IntoIter<((u32, u32, u32), MultiState<N>)>, fn(((u32, u32, u32), MultiState<N>)) -> MultiState<N>> {
    let MultiStateBuffer { states, metrics, .. } = self;
    let mut states: Vec<_> = states.into_iter().map(|(key, state)| (*metrics.get(&key).unwrap(), state)).collect();
    states.sort_by_key(|(metric, _)| *metric);
    states.into_iter().map(|(_, s)| s)
  }
}

impl<const N: usize> IntoIterator for MultiStateBuffer<N> {
  type Item = MultiState<N>;
  type IntoIter = std::iter::Map<std::collections::hash_map::IntoIter<(u64, BlockedInputs<N>), MultiState<N>>, fn(((u64, BlockedInputs<N>), MultiState<N>)) -> MultiState<N>>;

  fn into_iter(self) -> Self::IntoIter {
    self.states.into_iter().map(|(_, s)| s)
  }
}
impl<'a, const N: usize> IntoIterator for &'a MultiStateBuffer<N> {
  type Item = &'a MultiState<N>;
  type IntoIter = std::collections::hash_map::Values<'a, (u64, BlockedInputs<N>), MultiState<N>>;

  fn into_iter(self) -> Self::IntoIter {
    self.states.values()
  }
}
