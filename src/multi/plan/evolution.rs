use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvolutionPlanState  {
  hjoy5_state: HJoy5State,
  evolution_inputs_remaining: u8,
}
impl PartialOrd for EvolutionPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.evolution_inputs_remaining.partial_cmp(&self.evolution_inputs_remaining)
  }
}
impl PartialEq for EvolutionPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress Evolution_CheckForCancel inputs
pub struct EvolutionPlan {
  // instance state
  hjoy5_state: HJoy5State,
  evolution_inputs_remaining: u8,

  // config state
  cancel: bool,
  forced: bool,
}
impl EvolutionPlan {
  pub fn dont_cancel() -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      hjoy5_state: HJoy5State::unknown(),
      evolution_inputs_remaining: 0,

      cancel: false,
      forced: false,
    }
  }
  pub fn cancel() -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      hjoy5_state: HJoy5State::unknown(),
      evolution_inputs_remaining: 0,

      cancel: true,
      forced: false,
    }
  }
  pub fn forced() -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      hjoy5_state: HJoy5State::unknown(),
      evolution_inputs_remaining: 0,

      cancel: false,
      forced: true,
    }
  }
}
impl<R: MultiRom> Plan<R> for EvolutionPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::EvolutionState(EvolutionPlanState { hjoy5_state: self.hjoy5_state.clone(), evolution_inputs_remaining: self.evolution_inputs_remaining })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::EvolutionState(EvolutionPlanState { hjoy5_state, evolution_inputs_remaining }) = state {
      self.hjoy5_state = hjoy5_state.clone();
      self.evolution_inputs_remaining = *evolution_inputs_remaining;
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.hjoy5_state = HJoy5State::from_gb_state(gb, state);
    self.evolution_inputs_remaining = 72;
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { if self.cancel { Input::B } else { Input::empty() } }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    if self.hjoy5_state.get_hjoy5(input).contains(B) {
      if self.cancel || self.forced { Some(B) } else { None }
    } else {
      if self.cancel && self.evolution_inputs_remaining <= 1 { None } else { Some(Input::empty()) }
    }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)> {
    if self.hjoy5_state.get_hjoy5(input).contains(B) {
      if self.cancel {
        gb.restore(s);
        gb.input(input);
        gb.step();
        return Some((gb.save(), Some(())));
      } else if !self.forced { return None; }
    }
    gb.restore(s);
    gb.input(input);
    gb.step();
    self.evolution_inputs_remaining -= 1;
    if self.evolution_inputs_remaining == 0 {
      if self.cancel { None } else { Some((gb.save(), Some(()))) }
    } else {
      let new_state = gb.save();
      self.hjoy5_state = HJoy5State::from_gb(gb);
      Some((new_state, None))
    }
  }
}
