use std::collections::HashSet;

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

// Plan to progress CheckForUserInterruption inputs
pub struct SkipIntroPlan {
  // instance state
  inputs_until_auto_pass: u32,

  // config state
  initial_inputs_until_auto_pass: u32,
  allow_up_select_b: bool,
}
impl SkipIntroPlan {
  pub fn new() -> Self {
    Self {
      inputs_until_auto_pass: std::u32::MAX,
      initial_inputs_until_auto_pass: std::u32::MAX,
      allow_up_select_b: true,
    }
  }
  pub fn with_auto_pass_after(self, inputs_until_auto_pass: u32) -> Self { Self { inputs_until_auto_pass, initial_inputs_until_auto_pass: inputs_until_auto_pass, ..self } }
  pub fn with_no_up_select_b(self) -> Self { Self { allow_up_select_b: false, ..self } }
}
impl PlanBase for SkipIntroPlan {
  fn save(&self) -> PlanState {
    PlanState::SkipIntroState { inputs_until_auto_pass: self.inputs_until_auto_pass }
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::SkipIntroState { inputs_until_auto_pass } = state {
      self.inputs_until_auto_pass = *inputs_until_auto_pass;
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn reset(&mut self) { self.inputs_until_auto_pass = self.initial_inputs_until_auto_pass; }
  fn is_safe(&self) -> bool { true }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    if !self.allow_up_select_b && input.contains(U | SELECT | B) {
      // U|SELECT|B is not allowed, pressing it would enter the clear save dialog.
      return None;
    }
    if input == U | SELECT | B {
      return Some(input);
    }
    Some(input & (A | START))
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses> Plan<R> for SkipIntroPlan {
  type Value = ();

  fn get_inputs(&self, _gb: &mut Gb<R>, _s: &GbState) -> Inputs {
    let mut inputs = HashSet::new();

    if self.allow_up_select_b {
      // Propose U|SELECT|B to progress (other inputs not allowed).
      inputs.insert(InputDesc::new(U | SELECT | B, Input::empty()));
    }

    inputs.insert(InputDesc::any()); // Any input is fine, even if it doesn't progress.
    Inputs::new(inputs)
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
    let h_joy5 = HJoy5Metric.evaluate(gb).expect("Not at JoypadLowSensitivity input.");
    gb.step();
    if h_joy5.intersects(A | START) || self.inputs_until_auto_pass <= 1 {
      Some((gb.save(), Some(())))
    } else {
      self.inputs_until_auto_pass -= 1;
      Some((gb.save(), None))
    }
  }
}
