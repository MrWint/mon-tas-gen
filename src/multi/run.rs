use crate::metric::overworld::gen1::*;
use super::*;

pub struct SingleGbRunner<R> {
  gb: Gb<R>,
  states: MultiStateBuffer<1>,
  states_unsafe: MultiStateBuffer<1>,
  final_states: MultiStateBuffer<1>,
}
impl<R: MultiRom + Gen1OverworldAddresses + Gen1DVAddresses> SingleGbRunner<R> {
  pub fn new(gb: Gb<R>) -> Self {
    let initial_state = MultiState::new([MultiStateItem::new(gb.save(), PlanState::NullState, true)], InputLog::new());
    let mut result = Self {
      gb,
      states: MultiStateBuffer::new(),
      states_unsafe: MultiStateBuffer::new(),
      final_states: MultiStateBuffer::new(),
    };
    result.add_state(initial_state);
    result
  }

  pub fn run<P: Plan<R>>(&mut self, mut plan: P) {
    assert!(self.states_unsafe.is_empty() && self.final_states.is_empty());
    // Initialize states in buffer with new plan
    let old_states = std::mem::take(&mut self.states);
    for s in old_states.into_iter() {
      let MultiState { instances: [instance], inputs } = s;
      plan.initialize(&mut self.gb, &instance.gb_state);
      self.add_state(MultiState::new([MultiStateItem::new_rc(instance.gb_state, plan.save(), plan.is_safe())], inputs));
    }
    // Execute plan until all states are through
    while !self.states.is_empty() || !self.states_unsafe.is_empty() {
      if let Some(max_final_frame) = self.final_states.iter().map(|s| s.get_next_input_frame()).max() {
        let states = if self.states.is_empty() { &mut self.states_unsafe } else { &mut self.states };
        let min_frame = states.iter().map(|s| s.get_next_input_frame()).min().expect("Can't step: state buffer is empty");
        if min_frame > max_final_frame {
          log::info!("Discarding all remaining states, max final frame {} is smaller than next active frame {}", max_final_frame, min_frame);
          self.states = MultiStateBuffer::new();
          self.states_unsafe = MultiStateBuffer::new();
          break;
        }
      }
      self.step(&mut plan);
    }
    std::mem::swap(&mut self.states, &mut self.final_states);
  }

  /// Progress all states with the fewest number of frames, prioritizing unsafe states.
  fn step<P: Plan<R>>(&mut self, plan: &mut P) {
    let states = if self.states_unsafe.is_empty() { &mut self.states } else { &mut self.states_unsafe };
    let min_frame = states.iter().map(|s| s.get_next_input_frame()).min().expect("Can't step: state buffer is empty");
    let old_states = std::mem::take(states);
    drop(states); // Stop the mut borrow.
    let num_processed_states = old_states.iter().filter(|s| s.get_next_input_frame() == min_frame).count();
    log::debug!("performing step at frame {} with {} safe and {} unsafe states, moving {} of {} states", min_frame, self.states.len(), self.states_unsafe.len(), num_processed_states, old_states.len());
    for state in old_states.into_iter() {
      if state.get_next_input_frame() == min_frame {
        self.step_state(state, plan);
      } else {
        self.add_state(state);
      }
    }
  }
  fn step_state<P: Plan<R>>(&mut self, s: MultiState<1>, plan: &mut P) {
    let MultiState { instances: [instance], inputs } = s;
    // Choose input frame to fill
    let input_frame = instance.gb_state.get_input_frame_lo();
    let input_frame_lo = instance.gb_state.get_input_frame_lo();
    let input_frame_hi = instance.gb_state.get_input_frame_hi();
    assert!(inputs.len_hi() <= input_frame); // Hi nybble is still undecided for this frame.
    assert!(inputs.len_lo() <= input_frame + 1); // Lo nybble is still undecided for the next frame at least (may be determined for current frame if a -1/0 case became a 0/0 last iteration).

    // Check which nybbles are affected
    assert!(input_frame_lo >= input_frame_hi); // Assumption based on which order the nybbles are read in.
    assert!(input_frame_lo <= input_frame_hi + 1); // Nybble reads can't be more than one frame apart.

    let mut processed_canonical_inputs = HashSet::new();
    // Go through reasonable input combinations
    'next_input: for input_num in 0..=255 {
      let input = Input::from_bits_truncate(input_num);
      if input == inputs::LO_INPUTS { continue 'next_input; } // Block all attempts at soft reset inputs
      if let Some(hi_input) = inputs.get_input_hi(input_frame_hi) {
        if input & inputs::HI_INPUTS != hi_input { continue 'next_input; }
      }
      if let Some(lo_input) = inputs.get_input_lo(input_frame_lo) {
        if input & inputs::LO_INPUTS != lo_input { continue 'next_input; }
      }
      let input = instance.gb_state.remove_ignored_inputs(input);
      // Determine canonical inputs
      if let PlanState::NullState = instance.plan_state {
        plan.initialize(&mut self.gb, &instance.gb_state);
      } else {
        plan.restore(&instance.plan_state);
      }
      let canonical_input = if let Some(canonical_input) = plan.canonicalize_input(input) {
        canonical_input
      } else {
        continue 'next_input; // There is no hope for these inputs.
      };
      if !processed_canonical_inputs.insert(canonical_input) {
        continue 'next_input; // Combination of canonical states was already processed.
      }

      log::trace!("performing input {:?}", input);
      let mut is_done = false;
      let mut cur_gb_state = instance.gb_state.clone();
      while cur_gb_state.get_input_frame_lo() == input_frame_lo && cur_gb_state.get_input_frame_hi() == input_frame_hi {
        // Next input use is fully defined now, execute and move on
        if let Some((mut new_gb_state, instance_is_done)) = plan.execute_input(&mut self.gb, &cur_gb_state, input) {
          if instance_is_done.is_none() {
            new_gb_state.blocked_inputs &= plan.get_blockable_inputs();
          } else {
            assert!(new_gb_state.get_input_frame_lo() > input_frame_lo || new_gb_state.get_input_frame_hi() > input_frame_hi, "Unsavable final state with multiple input uses in its input frame.");
            cur_gb_state = std::rc::Rc::new(new_gb_state);
            is_done = true;
            break;
          }
          cur_gb_state = std::rc::Rc::new(new_gb_state);
        } else {
          continue 'next_input;
        }
      }
      let new_multi_state_item = if is_done {
        MultiStateItem::new_rc(cur_gb_state, PlanState::NullState, true)
      } else {
        MultiStateItem::new_rc(cur_gb_state, plan.save(), plan.is_safe())
      };
      let mut new_inputs = inputs.clone();
      new_inputs.set_input_hi(input_frame_hi, input);
      new_inputs.set_input_lo(input_frame_lo, input);
      let multi_state = MultiState::new([new_multi_state_item], new_inputs);
      if is_done {
        self.final_states.add_state(multi_state);
      } else {
        self.add_state(multi_state);
      }
    }
  }
  fn add_state(&mut self, state: MultiState<1>) {
    if state.is_safe() {
      self.states.add_state(state);
    } else {
      self.states_unsafe.add_state(state);
    }
  }
  
  pub fn save(&self, file_name: &str) {
    assert!(self.states_unsafe.is_empty() && self.final_states.is_empty());
    let file_path = format!("saves/{}.gz", file_name);
    let f = std::fs::File::create(file_path).unwrap();
    let mut f = ::flate2::write::GzEncoder::new(f, ::flate2::Compression::best());
    ::bincode::serialize_into(&mut f, &self.states).expect("saving statebuffer failed");
  }
  pub fn load(&mut self, file_name: &str) {
    let file_path = format!("saves/{}.gz", file_name);
    let f = std::fs::File::open(file_path).expect("file not found");
    let mut f = ::flate2::read::GzDecoder::new(f);
    self.states = ::bincode::deserialize_from(&mut f).expect("loading statebuffer failed");
    self.states_unsafe = MultiStateBuffer::new();
    self.final_states = MultiStateBuffer::new();
  }

  fn debug_identify_input(&mut self, state: &GbState) {
    let ignored_inputs = Input::all() - state.remove_ignored_inputs(Input::all());
    let ignored_input_text = if !ignored_inputs.is_empty() {
      format!("with Ignored inputs: {:?}", ignored_inputs)
    } else { String::new() };
    let frame = state.get_input_frame_lo();
    if let Some(name) = identify_input(&mut self.gb, state) {
      let additional_info = if name == "JoypadOverworld" {
        self.gb.restore(state);
        self.gb.input(Input::empty());
        format!(" ({:?})", get_overworld_interaction_result(&mut self.gb))
      } else { String::new() };
      log::info!("Next input {}{} at frame {} {}", name, additional_info, frame, ignored_input_text);
    } else {
      log::info!("Next input not identified at frame {} {}", frame, ignored_input_text);
    }
  }

  pub fn debug_segment_end(&mut self, file_name: &str) {
    {
      let chosen_state = self.states.iter().min_by_key(|s| s.inputs.len_max()).unwrap();
      let inputs = chosen_state.inputs.create_inputs();
      log::info!("Creating sample input file {} with {} inputs", file_name, inputs.len());
      Bk2Writer::new::<Blue>().write_bk2(&format!("{}.bk2", file_name), &inputs).unwrap();
    }
    log::info!("Rendering end states");
    let states: Vec<_> = self.states.iter().map(|s| s.instances[0].gb_state.clone()).collect();
    for s in states {
      self.debug_identify_input(&s);
      self.gb.restore(&s);
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
}
