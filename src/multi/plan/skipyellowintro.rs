use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkipYellowIntroPlanState {
  inputs_until_auto_pass: u32,
  pressed_state: PressedInputState,
}
impl PartialOrd for SkipYellowIntroPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.inputs_until_auto_pass.partial_cmp(&self.inputs_until_auto_pass)
  }
}
impl PartialEq for SkipYellowIntroPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress PlayIntroScene inputs
pub struct SkipYellowIntroPlan {
  // instance state
  inputs_until_auto_pass: u32,
  pressed_state: PressedInputState,

  // config state
  initial_inputs_until_auto_pass: u32,
}
impl SkipYellowIntroPlan {
  pub fn new() -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      inputs_until_auto_pass: std::u32::MAX,
      pressed_state: PressedInputState::unknown(),

      // Default config state.
      initial_inputs_until_auto_pass: std::u32::MAX,
    }
  }
  pub fn with_auto_pass_after(self, initial_inputs_until_auto_pass: u32) -> Self { Self { initial_inputs_until_auto_pass, ..self } }
}
impl<R: Rom + JoypadLowSensitivityAddresses> Plan<R> for SkipYellowIntroPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::SkipYellowIntroState(SkipYellowIntroPlanState { inputs_until_auto_pass: self.inputs_until_auto_pass, pressed_state: self.pressed_state })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::SkipYellowIntroState(SkipYellowIntroPlanState { inputs_until_auto_pass, pressed_state }) = state {
      self.inputs_until_auto_pass = *inputs_until_auto_pass;
      self.pressed_state = *pressed_state;
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.inputs_until_auto_pass = self.initial_inputs_until_auto_pass;
    self.pressed_state = PressedInputState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { A | B | START }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    let pressed = self.pressed_state.get_pressed_input(input);
    if pressed.intersects(A | B | START) {
      return Some(A | B | START);
    }
    Some(Input::empty())
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    gb.restore(s);
    gb.input(input);
    let pressed = self.pressed_state.get_pressed_input(input);
    if pressed.intersects(A | B | START) || self.inputs_until_auto_pass <= 1 {
      gb.step();
      Some((gb.save(), Some(())))
    } else {
      if true { return None; } // Yellow intro causes Joypad interrupts, don't allow it to continue.
      // Stay in intro, update instance state.
      gb.delay_step();
      let new_state = gb.save();
      self.inputs_until_auto_pass -= 1;
      self.pressed_state = PressedInputState::from_gb(gb);
      Some((new_state, None))
    }
  }
}
