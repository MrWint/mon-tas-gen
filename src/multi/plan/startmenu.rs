use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StartMenuPlanState {
  handle_menu_input_state: HandleMenuInputState,
  distance_to_goal: u8,
}
impl PartialOrd for StartMenuPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.distance_to_goal.partial_cmp(&self.distance_to_goal)
  }
}
impl PartialEq for StartMenuPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

const NUM_ITEMS_WITH_DEX: u8 = 7;
// Plan to progress HandleMenuInput_ inputs, selecting a chosen start menu item
pub struct StartMenuPlan {
  // instance state
  handle_menu_input_state: HandleMenuInputState,

  // config state
  goal_index: u8,
}
impl StartMenuPlan {
  pub fn dex() -> Self { Self::with_index(0) }
  pub fn mon() -> Self { Self::with_index(1) }
  pub fn items() -> Self { Self::with_index(2) }
  pub fn info() -> Self { Self::with_index(3) }
  pub fn save() -> Self { Self::with_index(4) }
  pub fn options() -> Self { Self::with_index(5) }
  pub fn close() -> SeqPlan<Self, StartMenuClosePlan> { SeqPlan::new(Self::with_index(6), StartMenuClosePlan::new()) }
  fn with_index(index: u8) -> Self {
    Self {
      handle_menu_input_state: HandleMenuInputState::unknown(),
      goal_index: index,
    }
  }

  #[inline]
  fn is_dex_missing(&self) -> bool {
    self.handle_menu_input_state.max_item == 6
  }
  #[inline]
  fn get_num_items(&self) -> u8 {
    self.handle_menu_input_state.max_item // this is actually 1 more than the max item to simulate wrapping
  }
  #[inline]
  fn get_effective_index(&self) -> u8 {
    // Adjust for missing dex
    self.handle_menu_input_state.current_item + if self.is_dex_missing() { 1 } else { 0 }
  }
  #[inline]
  fn is_close(&self) -> bool {
    self.goal_index == 6
  }
  fn distance_down(&self) -> u8 {
    let num_items = self.get_num_items();
    (self.goal_index + num_items - self.get_effective_index()) % num_items
  }
  fn distance_up(&self) -> u8 {
    let effective_index = self.get_effective_index();
    if self.goal_index <= effective_index {
      effective_index - self.goal_index
    } else {
      self.handle_menu_input_state.current_item + NUM_ITEMS_WITH_DEX - self.goal_index
    }
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses + HandleMenuInputAddresses + InputIdentificationAddresses> Plan<R> for StartMenuPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::StartMenuState(StartMenuPlanState { handle_menu_input_state: self.handle_menu_input_state.clone(), distance_to_goal: std::cmp::min(self.distance_up(), self.distance_down()) })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::StartMenuState(StartMenuPlanState { handle_menu_input_state, distance_to_goal: _ }) = state {
      self.handle_menu_input_state = handle_menu_input_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.handle_menu_input_state = HandleMenuInputState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input {
    if self.is_close() {
      if self.get_effective_index() == self.goal_index { A | B | START } else { B |START }
    } else if self.get_effective_index() == self.goal_index {
      A
    } else {
      let mut blocked_inputs = Input::empty();
      if self.distance_down() <= 3 { blocked_inputs |= D; }
      if self.distance_up() <= 3 { blocked_inputs |= U; }
      blocked_inputs
    }
  }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    match self.handle_menu_input_state.get_result(input) {
      HandleMenuInputResult::DoNothing => Some(Input::empty()),
      HandleMenuInputResult::ScrollTo { current_item } => {
        assert!(current_item == self.handle_menu_input_state.current_item); // All up/down movements in the start menu are watched and lead to an exit
        Some(Input::empty())
      },
      HandleMenuInputResult::Exit { current_item: _, input: exit_input } => {
        if self.is_close() {
          if exit_input.intersects(U | D) { Some(Input::empty()) }
          else if exit_input.intersects(B | START) { Some(exit_input & (B | START)) }
          else if self.get_effective_index() == 6 { Some(A) }
          else { None }
        } else if exit_input.intersects(U) {
          let dist_up = self.distance_up();
          if dist_up > 0 && dist_up <= 3 { Some(U) } else { None }
        } else if exit_input.intersects(D) {
          let dist_down = self.distance_down();
          if dist_down > 0 && dist_down <= 3 { Some(D) } else { None }
        } else {
          if self.get_effective_index() == self.goal_index && !exit_input.intersects(B | START) {
            Some(A)
          } else { None }
        }
      },
    }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    match self.handle_menu_input_state.get_result(input) {
      HandleMenuInputResult::DoNothing => {
        gb.restore(s);
        while gb.get_input_frame_lo() == s.get_input_frame_lo() && gb.get_input_frame_hi() == s.get_input_frame_hi() {
          gb.input(input);
          gb.delay_step();
        }
        let new_state = gb.save();
        self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
        Some((new_state, None))
      },
      HandleMenuInputResult::ScrollTo { current_item } => {
        assert!(current_item == self.handle_menu_input_state.current_item); // All up/down movements in the start menu are watched and lead to an exit
        gb.restore(s);
        gb.input(input);
        gb.delay_step();
        let new_state = gb.save();
        self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
        Some((new_state, None))
      },
      HandleMenuInputResult::Exit { current_item: _, input: exit_input } => {
        if self.is_close() {
          if exit_input.intersects(U | D) {
            gb.restore(s);
            gb.input(input);
            gb.delay_step();
            let new_state = gb.save();
            self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
            Some((new_state, None))
          } else if exit_input.intersects(B | START) || self.get_effective_index() == 6 {
            gb.restore(s);
            gb.input(input);
            gb.step();
            Some((gb.save(), Some(())))
          } else { None }
        } else if exit_input.intersects(U) {
          let dist_up = self.distance_up();
          if dist_up > 0 && dist_up <= 3 {
            gb.restore(s);
            gb.input(input);
            gb.step();
            let new_state = gb.save();
            self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
            Some((new_state, None))
          } else { None }
        } else if exit_input.intersects(D) {
          let dist_down = self.distance_down();
          if dist_down > 0 && dist_down <= 3 { 
            gb.restore(s);
            gb.input(input);
            gb.step();
            let new_state = gb.save();
            self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
            Some((new_state, None))
          } else { None }
        } else {
          if self.get_effective_index() == self.goal_index && !exit_input.intersects(B | START) {
            gb.restore(s);
            gb.input(input);
            gb.step();
            Some((gb.save(), Some(())))
          } else { None }
        }
      },
    }
  }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StartMenuClosePlanState {
  pressed_input_state: PressedInputState,
}
impl PartialOrd for StartMenuClosePlanState {
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    Some(Ordering::Equal)
  }
}
impl PartialEq for StartMenuClosePlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

pub struct StartMenuClosePlan {
  // instance state
  pressed_input_state: PressedInputState,
}
impl StartMenuClosePlan {
  pub fn new() -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      pressed_input_state: PressedInputState::unknown(),
    }
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses> Plan<R> for StartMenuClosePlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::StartMenuCloseState(StartMenuClosePlanState { pressed_input_state: self.pressed_input_state })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::StartMenuCloseState(StartMenuClosePlanState { pressed_input_state }) = state {
      self.pressed_input_state = *pressed_input_state;
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.pressed_input_state = PressedInputState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() } // blocking A can only be beneficial

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    Some(self.pressed_input_state.get_pressed_input(input) & A)
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    let pressed = self.pressed_input_state.get_pressed_input(input);
    gb.restore(s);
    gb.input(input);
    if pressed.intersects(A) {
      gb.delay_step();
      let new_state = gb.save();
      self.pressed_input_state = PressedInputState::from_gb(gb);
      Some((new_state, None))
    } else {
      gb.step();
      Some((gb.save(), Some(())))
    }
  }
}
