mod gb;
use std::{collections::HashSet, mem::MaybeUninit};

pub use gb::*;
mod input;
pub use input::*;
mod plan;
pub use plan::*;
mod statebuffer;
pub use statebuffer::*;
mod run;
pub use run::*;

use crate::bk2::*;
use crate::rom::*;
use gambatte::*;

pub trait IMultiGbExecutor {
  fn save(&self) -> MultiStateItem;
  fn load_plan(&mut self, plan_state: &PlanState, gb_state: &GbState);
  fn canonicalize_input(&self, state: &GbState, input: Input) -> Option<Input>;
  fn execute_input(&mut self, state: &GbState, input: Input) -> Option<(MultiStateItem, bool)>;
  fn debug_identify_input(&mut self, state: &GbState, instance: usize);
  fn debug_render_end_states(&mut self, state: &GbState);
}
pub struct MultiGbExecutor<R: MultiRom> {
  gb: Gb<R>,
  plan: ListPlan<R>,
}
impl<R: MultiRom> MultiGbExecutor<R> {
  pub fn new(mut gb: Gb<R>, mut plan: ListPlan<R>) -> Self {
    let initial_state = &gb.save();
    plan.initialize(&mut gb, initial_state);
    gb.restore(&initial_state);
    Self {
      gb,
      plan,
    }
  }
}

impl<R: MultiRom + InputIdentificationAddresses> IMultiGbExecutor for MultiGbExecutor<R> {
  fn save(&self) -> MultiStateItem {
    MultiStateItem::new(self.gb.save(), self.plan.save(), self.plan.is_safe())
  }
  fn load_plan(&mut self, plan_state: &PlanState, gb_state: &GbState) {
    self.plan.restore(plan_state);
    self.plan.ensure_cur_item_initialized(&mut self.gb, gb_state);
  }
  fn canonicalize_input(&self, state: &GbState, input: Input) -> Option<Input> {
    if (input & (inputs::LO_INPUTS | Input::UP)) == inputs::LO_INPUTS { return None; } // Block all attempts at soft reset inputs (apply most restrictive check from Yellow)
    let input = state.remove_ignored_inputs(input);
    self.plan.canonicalize_input(input)
  }
  fn execute_input(&mut self, state: &GbState, input: Input) -> Option<(MultiStateItem, bool)> {
    if (input & (inputs::LO_INPUTS | Input::UP)) == inputs::LO_INPUTS { return None; } // Block all attempts at soft reset inputs (apply most restrictive check from Yellow)
    let input = state.remove_ignored_inputs(input);
    if let Some((mut gb_state, value)) = self.plan.execute_input(&mut self.gb, state, input) {
      gb_state.blocked_inputs &= self.plan.get_blockable_inputs();

      Some((MultiStateItem::new(gb_state, self.plan.save(), self.plan.is_safe()), value.is_some()))
    } else { None }
  }
  fn debug_identify_input(&mut self, state: &GbState, instance: usize) {
    let frame = state.get_input_frame_lo();
    if let Some(name) = identify_input(&mut self.gb, state) {
      log::info!("Instance {} finished with next input {} at frame {}", instance, name, frame);
    } else {
      log::info!("Instance {} finished with next input not identified at frame {}", instance, frame);
    }
  }
  fn debug_render_end_states(&mut self, state: &GbState) {
    self.gb.restore(state);
    for _ in 0..10 {
      self.gb.input(Input::empty());
      self.gb.step();
      std::thread::sleep(std::time::Duration::from_millis(200));
    }
    for _ in 0..1000 {
      self.gb.input(Input::empty());
      self.gb.step();
    }
    std::thread::sleep(std::time::Duration::from_millis(200));
  }
}

pub trait CollectIntoArray {
  type Item;

  fn collect_into_array<const N: usize>(self) -> [Self::Item; N];
}
impl<I: Iterator> CollectIntoArray for I {
  type Item = I::Item;

  fn collect_into_array<const N: usize>(mut self) -> [Self::Item; N] {
    // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
    // safe because the type we are claiming to have initialized here is a
    // bunch of `MaybeUninit`s, which do not require initialization.
    let mut data: [MaybeUninit<Self::Item>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    // Dropping a `MaybeUninit` does nothing. Thus using raw pointer
    // assignment instead of `ptr::write` does not cause the old
    // uninitialized value to be dropped. Also if there is a panic during
    // this loop, we have a memory leak, but there is no memory safety
    // issue.
    for i in 0..N {
      if let Some(item) = self.next() {
        data[i] = MaybeUninit::new(item);
      } else { panic!("Iterator did not have enough elements") }
    }

    // Everything is initialized. Transmute the array to the initialized type.
    unsafe { std::mem::transmute_copy::<_, [Self::Item; N]>(&data) }
  }
}

pub struct MultiGbRunner<const N: usize> {
  instances: [Box<dyn IMultiGbExecutor>; N],
  states: MultiStateBuffer<N>,
  states_unsafe: MultiStateBuffer<N>,
  final_states: MultiStateBuffer<N>,
}
impl<const N: usize> MultiGbRunner<N> {
  pub fn new(instances: [Box<dyn IMultiGbExecutor>; N]) -> Self {
    assert!(!instances.is_empty());
    let initial_state = MultiState::new(instances.iter().map(|instance| instance.save()).collect_into_array(), InputLog::new());
    let mut result = Self {
      instances,
      states: MultiStateBuffer::new(),
      states_unsafe: MultiStateBuffer::new(),
      final_states: MultiStateBuffer::new(),
    };
    result.add_state(initial_state);
    result
  }

  pub fn run(&mut self) {
    while !self.has_finished_states() {
      self.step();
    }
  }

  pub fn run_until(&mut self, end_frame: u32) {
    while !self.has_finished_states() && self.states.iter().map(|s| s.get_next_input_frame()).min().unwrap_or(0) <= end_frame {
      self.step();
    }
  }

  /// Progress all states with the fewest number of frames, prioritizing unsafe states.
  pub fn step(&mut self) {
    let states = if self.states_unsafe.is_empty() { &mut self.states } else { &mut self.states_unsafe };
    let min_frame = states.iter().map(|s| s.get_next_input_frame()).min().expect("Can't step: state buffer is empty");
    let old_states = std::mem::take(states);
    drop(states); // Stop the mut borrow.
    let num_processed_states = old_states.iter().filter(|s| s.get_next_input_frame() == min_frame).count();
    log::debug!("performing step at frame {} with {} safe and {} unsafe states, moving {} of {} states", min_frame, self.states.len(), self.states_unsafe.len(), num_processed_states, old_states.len());
    for state in old_states.into_iter() {
      if state.get_next_input_frame() == min_frame {
        self.step_state(state);
      } else {
        self.add_state(state);
      }
    }
  }
  fn step_state(&mut self, s: MultiState<N>) {
    // Choose input frame to fill
    let input_frame = s.get_next_input_frame();
    assert!(s.inputs.len_hi() <= input_frame); // Hi nybble is still undecided for this frame.
    assert!(s.inputs.len_lo() <= input_frame + 1); // Lo nybble is still undecided for the next frame at least (may be determined for current frame if a -1/0 case became a 0/0 last iteration).
    // Start with any input
    let mut input_iter = InputIterator::new();
    let mut prev_hi_determined = false;
    let mut lo_determined = false;
    let mut set_prev_hi = false;
    let mut set_hi = false;

    // Apply any already decided hi nybble of the previous frame
    if let Some(hi_input) = s.inputs.get_input_hi(input_frame - 1) {
      prev_hi_determined = true;
      input_iter.prev_input |= hi_input;
    }
    if let Some(lo_input) = s.inputs.get_input_lo(input_frame) {
      lo_determined = true;
      input_iter.cur_input |= lo_input;
    }

    // Check which nybbles are affected
    for i in 0..N {
      let input_frame_lo = s.instances[i].gb_state.get_input_frame_lo();
      let input_frame_hi = s.instances[i].gb_state.get_input_frame_hi();
      assert!(input_frame_lo >= input_frame_hi); // Assumption based on which order the nybbles are read in.
      assert!(input_frame_lo <= input_frame_hi + 1); // Nybble reads can't be more than one frame apart.
      assert!(input_frame_lo >= input_frame); // No instance is left behind
      assert!(!lo_determined || input_frame_hi >= input_frame); // -1/0 case is impossible if current lo is already determined, these would have been processed in the last step.
      if input_frame_hi < input_frame { // -1/0 case
        if !prev_hi_determined { set_prev_hi = true; }
      } else if input_frame_lo == input_frame { // 0/0 case
        set_hi = true;
      }
      if input_frame_lo == input_frame || input_frame_hi == input_frame { // If this instance affects the input frame
        self.instances[i].load_plan(&s.instances[i].plan_state, &s.instances[i].gb_state); // pre-load all plan instances, they will be used multiple times later.
      }
    }
    assert!(!lo_determined || (set_hi && !set_prev_hi)); // No -1/0 cases exist if lo is already determined
    input_iter.use_lo = !lo_determined;
    input_iter.use_hi = set_hi;
    input_iter.use_prev_hi = set_prev_hi;

    let mut processed_canonical_inputs = HashSet::new();
    // Go through reasonable input combinations
    'next_input: for (prev_input, cur_input) in input_iter {
      if set_hi { // If hi bit is being set, check for possible conflicts with 0/1 cases
        let hi = cur_input & inputs::HI_INPUTS;
        for i in 0..N {
          let input_frame_lo = s.instances[i].gb_state.get_input_frame_lo();
          let input_frame_hi = s.instances[i].gb_state.get_input_frame_hi();
          if input_frame_hi == input_frame && input_frame_lo > input_frame { // 0/1 case
            // Plan is guaranteed to be already loaded.
            if (0..16).map(Input::from_bits_truncate).find_map(|lo| self.instances[i].canonicalize_input(&s.instances[i].gb_state, hi | lo)).is_none() {
              continue 'next_input; // There is no hope for these inputs.
            }
          }
        }
      }
      // Determine canonical inputs
      let mut canonical_inputs = [Input::empty(); N];
      for i in 0..N {
        let input_frame_lo = s.instances[i].gb_state.get_input_frame_lo();
        let input_frame_hi = s.instances[i].gb_state.get_input_frame_hi();
        self.instances[i].load_plan(&s.instances[i].plan_state, &s.instances[i].gb_state); // reload plan.
        canonical_inputs[i] = if input_frame_lo == input_frame {  // -1/0 or 0/0 case
          let instance_input = if input_frame_hi < input_frame { // -1/0 case
            (prev_input & inputs::HI_INPUTS) | (cur_input & inputs::LO_INPUTS)
          } else { // 0/0 case
            cur_input
          };
          // Plan is guaranteed to be already loaded.
          if let Some(canonical_input) = self.instances[i].canonicalize_input(&s.instances[i].gb_state, instance_input) {
            canonical_input
          } else {
            continue 'next_input; // There is no hope for these inputs.
          }
        } else {
          Input::empty() // Ignore unused instances
        };
      }
      if !processed_canonical_inputs.insert(canonical_inputs) {
        continue 'next_input; // Combination of canonical states was already processed.
      }
      log::trace!("performing inputs {:?} and {:?}", prev_input, cur_input);
      let mut multi_state_items: [MaybeUninit<MultiStateItem>; N] = unsafe { MaybeUninit::uninit().assume_init() };
      let mut is_done = false;
      for i in 0..N {
        let mut cur_instance = s.instances[i].clone();
        loop {
          let input_frame_lo = cur_instance.gb_state.get_input_frame_lo();
          let input_frame_hi = cur_instance.gb_state.get_input_frame_hi();
          if input_frame_lo > input_frame { break; } // not -1/0 nor 0/0 case
          if !set_hi && input_frame_hi == input_frame { break; } // not -1/0 case, cur hi will not be set so we're done
          // Next input use is fully defined now, execute and move on
          let instance_input = if input_frame_hi < input_frame { // -1/0 case
            (prev_input & inputs::HI_INPUTS) | (cur_input & inputs::LO_INPUTS)
          } else { // 0/0 case
            cur_input
          };
          if let Some((multi_state_item, instance_is_done)) = self.instances[i].execute_input(&cur_instance.gb_state, instance_input) {
            cur_instance = multi_state_item;
            if instance_is_done {
              is_done = true;
              self.instances[i].debug_identify_input(&cur_instance.gb_state, i);
              assert!(cur_instance.gb_state.get_input_frame_lo() > input_frame_lo || (!set_hi && cur_instance.gb_state.get_input_frame_hi() == input_frame), "Unsavable final state with multiple input uses in its input frame.");
              break;
            }
          } else {
            // Drop allocated items to prevent memory leak.
            for elem in &mut multi_state_items[0..i] {
              unsafe { std::ptr::drop_in_place(elem.as_mut_ptr()); }
            }
            continue 'next_input;
          }
        }
        multi_state_items[i] = MaybeUninit::new(cur_instance);
      }
      let mut new_inputs = s.inputs.clone();
      if set_prev_hi {
        new_inputs.set_input_hi(input_frame - 1, prev_input);
      }
      if !lo_determined {
        new_inputs.set_input_lo(input_frame, cur_input);
      }
      if set_hi {
        new_inputs.set_input_hi(input_frame, cur_input);
      }
      let multi_state = MultiState::new(unsafe { std::mem::transmute_copy(&multi_state_items) }, new_inputs);
      if is_done {
        self.final_states.add_state(multi_state);
      } else {
        self.add_state(multi_state);
      }
    }
  }
  fn add_state(&mut self, state: MultiState<N>) {
    if state.is_safe() {
      self.states.add_state(state);
    } else {
      self.states_unsafe.add_state(state);
    }
  }

  pub fn has_finished_states(&self) -> bool {
    !self.final_states.is_empty()
  }
  
  pub fn save(&self, file_name: &str) {
    let file_path = format!("saves/{}.gz", file_name);
    let f = std::fs::File::create(file_path).unwrap();
    let mut f = ::flate2::write::GzEncoder::new(f, ::flate2::Compression::best());
    ::bincode::serialize_into(&mut f, &self.states).expect("saving statebuffer failed");
    ::bincode::serialize_into(&mut f, &self.states_unsafe).expect("saving statebuffer failed");
    ::bincode::serialize_into(&mut f, &self.final_states).expect("saving statebuffer failed");
  }
  pub fn load(&mut self, file_name: &str) {
    let file_path = format!("saves/{}.gz", file_name);
    let f = std::fs::File::open(file_path).expect("file not found");
    let mut f = ::flate2::read::GzDecoder::new(f);
    self.states = ::bincode::deserialize_from(&mut f).expect("loading statebuffer failed");
    self.states_unsafe = ::bincode::deserialize_from(&mut f).expect("loading statebuffer failed");
    self.final_states = MultiStateBuffer::default();
    let stored_final_states: MultiStateBuffer<N> = ::bincode::deserialize_from(&mut f).expect("loading statebuffer failed");

    // Final states are not final anymore after the plans have been extended.
    for state in stored_final_states.into_iter() {
      self.add_state(state);
    }
  }

  pub fn debug_segment_end(&mut self, file_name: &str) {
    {
      let chosen_state = self.final_states.iter().min_by_key(|s| s.inputs.len_max()).unwrap();
      let inputs = chosen_state.inputs.create_inputs();
      log::info!("Creating sample input file {} with {} inputs", file_name, inputs.len());
      Bk2Writer::new::<Blue>().write_bk2(&format!("{}.bk2", file_name), &inputs).unwrap();
    }
    log::info!("Rendering end states");
    for s in self.final_states.iter() {
      for i in 0..N {
        self.instances[i].debug_render_end_states(&s.instances[i].gb_state);
      }
    }
  }
}


struct InputIterator {
  use_prev_hi: bool,
  use_lo: bool,
  use_hi: bool,
  prev_input: Input,
  cur_input: Input,
  done: bool,
}
impl InputIterator {
  fn new() -> Self {
    InputIterator {
      use_prev_hi: false,
      use_lo: true,
      use_hi: false,
      prev_input: Input::empty(),
      cur_input: Input::empty(),
      done: false,
    }
  }
}
impl Iterator for InputIterator {
  type Item = (Input, Input);

  fn next(&mut self) -> Option<(Input, Input)> {
    if self.done { return None; }
    let result = Some((self.prev_input, self.cur_input));
    if !self.use_hi && self.cur_input.contains(inputs::LO_INPUTS) { // !use_hi implies use_lo
      self.cur_input -= inputs::LO_INPUTS;
    } else {
      let increment = if self.use_lo { 1 } else { 16 };
      self.cur_input = Input::from_bits_truncate(self.cur_input.bits().wrapping_add(increment));
      if self.cur_input.bits() >= increment { return result; }
    }
    if self.use_prev_hi {
      self.prev_input = Input::from_bits_truncate(self.prev_input.bits().wrapping_add(16));
      self.done = self.prev_input.bits() < 16;
    } else {
      self.done = true;
    }
    result
  }
}
