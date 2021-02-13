use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkipIntroPlanState {
  inputs_until_auto_pass: u32,
  hjoy5_state: HJoy5State,
}
impl PartialOrd for SkipIntroPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.inputs_until_auto_pass.partial_cmp(&self.inputs_until_auto_pass)
  }
}
impl PartialEq for SkipIntroPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress CheckForUserInterruption inputs
pub struct SkipIntroPlan {
  // instance state
  inputs_until_auto_pass: u32,
  hjoy5_state: HJoy5State,

  // config state
  initial_inputs_until_auto_pass: u32,
  allow_up_select_b: bool,
}
impl SkipIntroPlan {
  pub fn new() -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      inputs_until_auto_pass: std::u32::MAX,
      hjoy5_state: HJoy5State::unknown(),

      // Default config state.
      initial_inputs_until_auto_pass: std::u32::MAX,
      allow_up_select_b: true,
    }
  }
  pub fn with_auto_pass_after(self, initial_inputs_until_auto_pass: u32) -> Self { Self { initial_inputs_until_auto_pass, ..self } }
  pub fn with_no_up_select_b(self) -> Self { Self { allow_up_select_b: false, ..self } }
}
impl<R: Rom + JoypadLowSensitivityAddresses> Plan<R> for SkipIntroPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::SkipIntroState(SkipIntroPlanState { inputs_until_auto_pass: self.inputs_until_auto_pass, hjoy5_state: self.hjoy5_state })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::SkipIntroState(SkipIntroPlanState { inputs_until_auto_pass, hjoy5_state }) = state {
      self.inputs_until_auto_pass = *inputs_until_auto_pass;
      self.hjoy5_state = *hjoy5_state;
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.inputs_until_auto_pass = self.initial_inputs_until_auto_pass;
    self.hjoy5_state = HJoy5State::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { A | START }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    if !self.allow_up_select_b && input.contains(U | SELECT | B) {
      // U|SELECT|B is not allowed, pressing it would enter the clear save dialog.
      return None;
    }
    if input == U | SELECT | B {
      return Some(input);
    }
    let hjoy5 = self.hjoy5_state.get_hjoy5(input);
    Some(hjoy5 & (A | START))
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    if !self.allow_up_select_b && input.contains(U | SELECT | B) {
      // U|SELECT|B is not allowed, pressing it would enter the clear save dialog.
      return None;
    }
    gb.restore(s);
    gb.input(input);
    if input == U | SELECT | B {
      gb.step();
      return Some((gb.save(), Some(())));
    }
    let hjoy5 = self.hjoy5_state.get_hjoy5(input);
    if hjoy5.intersects(A | START) || self.inputs_until_auto_pass <= 1 {
      gb.step();
      Some((gb.save(), Some(())))
    } else {
      // Stay in intro, update instance state.
      gb.delay_step();
      let new_state = gb.save();
      self.inputs_until_auto_pass -= 1;
      self.hjoy5_state = HJoy5State::from_gb(gb);
      Some((new_state, None))
    }
  }
}
