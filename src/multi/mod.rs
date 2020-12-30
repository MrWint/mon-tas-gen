mod gb;
pub use gb::*;
mod input;
pub use input::*;
mod metric;
pub use metric::*;
mod plan;
pub use plan::*;
mod statebuffer;
pub use statebuffer::*;

use crate::rom::*;
use gambatte::*;

pub trait IMultiGbExecutor {
  fn save(&self) -> MultiStateItem;
  fn load_plan(&mut self, state: &PlanState);
  fn get_inputs(&mut self, state: &GbState) -> Inputs;
  fn execute_input(&mut self, state: &GbState, input: Input) -> Option<MultiStateItem>;
}
pub struct MultiGbExecutor<R: Rom> {
  gb: Gb<R>,
  plan: ListPlan<R>,
}
impl<R: Rom> MultiGbExecutor<R> {
  pub fn new(gb: Gb<R>, mut plan: ListPlan<R>) -> Self {
    plan.append_sentinel(); // Make sure the plan never ends.
    plan.reset();
    Self {
      gb,
      plan,
    }
  }
}

impl<R: Rom> IMultiGbExecutor for MultiGbExecutor<R> {
  fn save(&self) -> MultiStateItem {
    MultiStateItem::new(self.gb.save(), self.plan.save(), self.plan.is_safe())
  }
  fn load_plan(&mut self, state: &PlanState) {
    self.plan.restore(state);
  }
  fn get_inputs(&mut self, state: &GbState) -> Inputs {
    self.plan.get_inputs(&mut self.gb, state)
  }
  fn execute_input(&mut self, state: &GbState, input: Input) -> Option<MultiStateItem> {
    let input = state.remove_ignored_inputs(input);
    if let Some((gb_state, value)) = self.plan.execute_input(&mut self.gb, state, input) {
      assert!(value.is_none()); // End sentinel guarantees that the plan never completes.
      Some(MultiStateItem::new(gb_state, self.plan.save(), self.plan.is_safe()))
    } else { None }
  }
}

pub struct MultiGbRunner {
  instances: Vec<Box<dyn IMultiGbExecutor>>,
  states: MultiStateBuffer,
  states_unsafe: MultiStateBuffer,
}
impl MultiGbRunner {
  pub fn new(instances: Vec<Box<dyn IMultiGbExecutor>>) -> Self {
    assert!(!instances.is_empty());
    let initial_state = MultiState::new(instances.iter().map(|instance| instance.save()).collect(), InputLog::new());
    let mut result = Self {
      instances,
      states: MultiStateBuffer::new(),
      states_unsafe: MultiStateBuffer::new(),
    };
    result.add_state(initial_state);
    result
  }

  /// Progress all states with the fewest number of frames, prioritizing unsafe states.
  pub fn step(&mut self) {
    log::debug!("Step with buffer {} safe and {} unsafe states", self.states.len(), self.states_unsafe.len());
    let states = if self.states_unsafe.is_empty() { &mut self.states } else { &mut self.states_unsafe };
    let min_frame = states.iter().map(|s| s.get_next_input_frame()).min().expect("Can't step: state buffer is empty");
    log::debug!("performing step at frame {}", min_frame);
    let old_states = std::mem::take(states);
    drop(states); // Stop the mut borrow.
    for state in old_states.into_iter() {
      if state.get_next_input_frame() == min_frame {
        self.step_state(state);
      } else {
        self.add_state(state);
      }
    }
  }
  fn step_state(&mut self, s: MultiState) {
    assert_eq!(self.instances.len(), s.instances.len());
    // Choose input frame to fill
    let input_frame = s.get_next_input_frame();
    // Start with any input
    let mut combined_inputs = SplitInputs::any();
    let mut use_lo = false;
    let mut use_hi = false;
    assert!(s.inputs.len_lo() <= input_frame && s.inputs.len_hi() <= input_frame); // Both nybbles are still undecided for this frame.

    // Apply any already decided nybble of the previous frame
    let mut prev_frame_set_inputs = InputDesc::any();
    if let Some(lo_input) = s.inputs.get_input_lo(input_frame - 1) {
      prev_frame_set_inputs = (prev_frame_set_inputs & InputDesc::new(lo_input, inputs::HI_INPUTS)).unwrap();
    }
    if let Some(hi_input) = s.inputs.get_input_hi(input_frame - 1) {
      prev_frame_set_inputs = (prev_frame_set_inputs & InputDesc::new(hi_input, inputs::LO_INPUTS)).unwrap();
    }
    combined_inputs = combined_inputs & (prev_frame_set_inputs, InputDesc::any());

    // Apply any restrictions based on the plans
    for i in 0..s.instances.len() {
      let input_frame_lo = s.instances[i].gb_state.get_input_frame_lo();
      let input_frame_hi = s.instances[i].gb_state.get_input_frame_hi();
      assert!(input_frame_lo >= input_frame || input_frame_hi >= input_frame); // No instance is left behind
      if input_frame_lo == input_frame || input_frame_hi == input_frame { // If this instance affects the input frame
        self.instances[i].load_plan(&s.instances[i].plan_state);
        let inputs = self.instances[i].get_inputs(&s.instances[i].gb_state);
        let mut split_inputs = if input_frame_lo < input_frame {
          assert_eq!(input_frame_lo + 1, input_frame);
          use_hi = true;
          inputs.split_lo_hi()
        } else if input_frame_lo > input_frame {
          assert_eq!(input_frame_lo - 1, input_frame);
          inputs.split_only_hi()
        } else if input_frame_hi < input_frame {
          assert_eq!(input_frame_hi + 1, input_frame);
          use_lo = true;
          inputs.split_hi_lo()
        } else if input_frame_hi > input_frame {
          assert_eq!(input_frame_hi - 1, input_frame);
          inputs.split_only_lo()
        } else {
          use_lo = true;
          use_hi = true;
          inputs.split()
        };
        // Restrict past inputs if they are already decided.
        split_inputs = split_inputs & (prev_frame_set_inputs, InputDesc::any());
        combined_inputs = combined_inputs.combine(&split_inputs);
      }
    }
    // Go through any input that fulfills all conditions
    'next_input: for (prev_input, cur_input) in combined_inputs.iter().map(|(p, c)| (p.get_input(), c.get_input())) {
      log::debug!("performing inputs {:?} and {:?}", prev_input, cur_input);
      let mut multi_state_items = vec![];
      for i in 0..s.instances.len() {
        let input_frame_lo = s.instances[i].gb_state.get_input_frame_lo();
        let input_frame_hi = s.instances[i].gb_state.get_input_frame_hi();
        if input_frame_lo <= input_frame && input_frame_hi <= input_frame {
          // Next input use is fully defined now, execute and move on
          let instance_input = if input_frame_lo < input_frame {
            assert_eq!(input_frame_lo + 1, input_frame);
            (cur_input & inputs::HI_INPUTS) | (prev_input & inputs::LO_INPUTS)
          } else if input_frame_hi < input_frame {
            assert_eq!(input_frame_hi + 1, input_frame);
            (cur_input & inputs::LO_INPUTS) | (prev_input & inputs::HI_INPUTS)
          } else {
            cur_input
          };
          if instance_input == inputs::LO_INPUTS { continue 'next_input; } // Don't use reset inputs

          self.instances[i].load_plan(&s.instances[i].plan_state); // reload plan (may have been altered in previous loop iterations)
          if let Some(multi_state_item) = self.instances[i].execute_input(&s.instances[i].gb_state, instance_input) {
            multi_state_items.push(multi_state_item);
          } else { continue 'next_input; }
        } else {
          multi_state_items.push(s.instances[i].clone());
        }
      }
      assert_eq!(multi_state_items.len(), s.instances.len());
      let mut new_inputs = s.inputs.clone();
      new_inputs.set_input_lo(input_frame - 1, prev_input);
      new_inputs.set_input_hi(input_frame - 1, prev_input);
      if use_lo {
        new_inputs.set_input_lo(input_frame, cur_input);
      }
      if use_hi {
        new_inputs.set_input_hi(input_frame, cur_input);
      }
      self.add_state(MultiState::new(multi_state_items, new_inputs))
    }
  }
  fn add_state(&mut self, state: MultiState) {
    if state.is_safe() {
      self.states.add_state(state);
    } else {
      self.states_unsafe.add_state(state);
    }
  }
  
  pub fn save(&self, _file_name: &str) {
    todo!()
  }
  pub fn load(&mut self, _file_name: &str) {
    todo!()
  }
}
