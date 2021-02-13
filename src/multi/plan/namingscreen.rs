use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::metric::*;
use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamingScreenPlanState {
  letter_selected: bool,
  delta: (i8, i8),
  pressed_input_state: PressedInputState,
}
impl PartialOrd for NamingScreenPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.letter_selected != other.letter_selected {
      self.letter_selected.partial_cmp(&other.letter_selected)
    } else {
      (other.delta.0.abs() + other.delta.1.abs()).partial_cmp(&(self.delta.0.abs() + self.delta.1.abs()))
    }
  }
}
impl PartialEq for NamingScreenPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress DisplayNamingScreen inputs, entering single-letter names
pub struct NamingScreenPlan<M> {
  // instance state
  pressed_input_state: PressedInputState,
  delta: (i8, i8),
  letter_selected: bool,

  // config state
  initial_delta: (i8, i8),
  metric: M,
}
impl NamingScreenPlan<NullMetric> {
  pub fn with_letter(letter: u8) -> Self { Self::with_metric(letter, NullMetric) }
}
impl<M> NamingScreenPlan<M> {
  pub fn with_metric(letter: u8, metric: M) -> Self {
    assert!(letter >= b'A' && letter <= b'Z');
    let letter_offset = (letter - b'A') as i8;
    let dx = (letter_offset + 4) % 9 - 4;
    let dy = letter_offset / 9;
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      pressed_input_state: PressedInputState::unknown(),
      delta: (dx, dy),
      letter_selected: false,

      // Default config state.
      initial_delta: (dx, dy),
      metric,
    }
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses, M: Metric<R>> Plan<R> for NamingScreenPlan<M> {
  type Value = M::ValueType;

  fn save(&self) -> PlanState {
    PlanState::NamingScreenState(NamingScreenPlanState { letter_selected: self.letter_selected, delta: self.delta, pressed_input_state: self.pressed_input_state })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::NamingScreenState(NamingScreenPlanState { letter_selected, delta, pressed_input_state }) = state {
      self.letter_selected = *letter_selected;
      self.delta = *delta;
      self.pressed_input_state = *pressed_input_state;
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.letter_selected = false;
    self.delta = self.initial_delta;
    self.pressed_input_state = PressedInputState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input {
    if self.letter_selected { START }
    else if self.delta.0 == 0 && self.delta.1 == 0 { A }
    else {
      let mut blocked_inputs = Input::empty();
      if self.delta.0 > 0 { blocked_inputs |= R; }
      if self.delta.0 < 0 { blocked_inputs |= L; }
      if self.delta.1 > 0 { blocked_inputs |= D; }
      if self.delta.1 < 0 { blocked_inputs |= U; }
      blocked_inputs
    }
  }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    let input = self.pressed_input_state.get_pressed_input(input);
    if input.contains(D) {
      if self.letter_selected { Some(Input::empty()) } else if self.delta.1 > 0 { Some(D) } else { None }
    } else if input.contains(U) {
      if self.letter_selected { Some(Input::empty()) } else if self.delta.1 < 0 { Some(U) } else { None }
    } else if input.contains(L) {
      if self.letter_selected { Some(Input::empty()) } else if self.delta.0 < 0 { Some(L) } else { None }
    } else if input.contains(R) {
      if self.letter_selected { Some(Input::empty()) } else if self.delta.0 > 0 { Some(R) } else { None }
    } else if input.contains(START) {
      if self.letter_selected { Some(START) } else { None }
    } else if input.contains(SELECT) {
      if self.letter_selected { Some(Input::empty()) } else { None }
    } else if input.contains(B) {
      if self.letter_selected { None } else { Some(Input::empty()) }
    } else if input.contains(A) {
      if self.letter_selected || self.delta != (0, 0) { None } else { Some(A) }
    } else { Some(Input::empty()) }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<M::ValueType>)> {
    let pressed = self.pressed_input_state.get_pressed_input(input);
    let mut is_done = false;
    let mut delay = false;
    if pressed.contains(D) {
      if self.letter_selected { delay = true; } else if self.delta.1 > 0 { self.delta = (self.delta.0, self.delta.1 - 1) } else { return None; }
    } else if pressed.contains(U) {
      if self.letter_selected { delay = true; } else if self.delta.1 < 0 { self.delta = (self.delta.0, self.delta.1 + 1) } else { return None; }
    } else if pressed.contains(L) {
      if self.letter_selected { delay = true; } else if self.delta.0 < 0 { self.delta = (self.delta.0 + 1, self.delta.1) } else { return None; }
    } else if pressed.contains(R) {
      if self.letter_selected { delay = true; } else if self.delta.0 > 0 { self.delta = (self.delta.0 - 1, self.delta.1) } else { return None; }
    } else if pressed.contains(START) {
      if self.letter_selected { is_done = true; } else { return None; }
    } else if pressed.contains(SELECT) {
      if self.letter_selected { delay = true; } else { return None; }
    } else if pressed.contains(B) {
      if self.letter_selected { return None; } else { delay = true; }
    } else if pressed.contains(A) {
      if self.letter_selected || self.delta != (0, 0) { return None; } else { self.letter_selected = true; }
    } else { delay = true; }
    gb.restore(s);
    gb.input(input);
    if is_done {
      if let Some(metric_value) = self.metric.evaluate(gb) {
        gb.step();
        return Some((gb.save(), Some(metric_value)));
      } else { return None; }
    }
    if delay {
      gb.delay_step();
    } else {
      gb.step();
    }
    let new_state = gb.save();
    self.pressed_input_state = PressedInputState::from_gb(gb);
    Some((new_state, None))
  }
}
