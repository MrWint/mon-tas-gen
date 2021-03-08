use serde_derive::{Serialize, Deserialize};
use std::cmp::{Ordering, min};

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChooseQuantityMenuPlanState {
  pressed_state: PressedInputState,
  current_quantity: u8,
  max_quantity: u8,

  goal_dist: u8,
}
impl PartialOrd for ChooseQuantityMenuPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.goal_dist.partial_cmp(&self.goal_dist)
  }
}
impl PartialEq for ChooseQuantityMenuPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress HandleMenuInput_ inputs, selecting an item in the a DisplayChooseQuantityMenuID list
pub struct ChooseQuantityMenuPlan {
  // instance state
  pressed_state: PressedInputState,
  current_quantity: u8,
  max_quantity: u8,

  // config state
  goal_quantity: u8,
}
impl ChooseQuantityMenuPlan {
  pub fn new(goal_quantity: u8) -> Self {
    Self {
      pressed_state: PressedInputState::unknown(),
      current_quantity: 0,
      max_quantity: 0,

      goal_quantity,
    }
  }
  fn goal_dist(&self, current_quantity: u8) -> u8 {
    min(self.goal_dist_up(current_quantity), self.goal_dist_down(current_quantity))
  }
  fn goal_dist_up(&self, current_quantity: u8) -> u8 {
    if current_quantity <= self.goal_quantity {
      self.goal_quantity - current_quantity
    } else {
      self.goal_quantity + self.max_quantity - current_quantity
    }
  }
  fn goal_dist_down(&self, current_quantity: u8) -> u8 {
    if current_quantity >= self.goal_quantity {
      current_quantity - self.goal_quantity
    } else {
      current_quantity + self.max_quantity - self.goal_quantity
    }
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses + HandleMenuInputAddresses + InputIdentificationAddresses> Plan<R> for ChooseQuantityMenuPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::ChooseQuantityMenuState(ChooseQuantityMenuPlanState { pressed_state: self.pressed_state.clone(), current_quantity: self.current_quantity, max_quantity: self.max_quantity, goal_dist: self.goal_dist(self.current_quantity) })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::ChooseQuantityMenuState(ChooseQuantityMenuPlanState { pressed_state, current_quantity, max_quantity, goal_dist: _ }) = state {
      self.pressed_state = pressed_state.clone();
      self.current_quantity = *current_quantity;
      self.max_quantity = *max_quantity;
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.pressed_state = PressedInputState::from_gb_state(gb, state);
    self.current_quantity = gb.gb.read_memory(R::MENU_ITEM_QUANTITY_MEM_ADDRESS);
    self.max_quantity = gb.gb.read_memory(R::MENU_MAX_ITEM_QUANTITY_MEM_ADDRESS);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input {
    if self.current_quantity == self.goal_quantity { A }
    else {
      let mut input = Input::empty();
      if self.goal_dist_down(self.current_quantity) <= self.goal_dist_up(self.current_quantity) { input |= D }
      if self.goal_dist_down(self.current_quantity) >= self.goal_dist_up(self.current_quantity) { input |= U }
      input
    }
  }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    let pressed = self.pressed_state.get_pressed_input(input);
    if pressed.intersects(A) {
      if self.current_quantity != self.goal_quantity { None } else { Some(A) }
    } else if pressed.intersects(B) {
      None
    } else if pressed.intersects(U) {
      if self.current_quantity == self.goal_quantity || self.goal_dist_down(self.current_quantity) < self.goal_dist_up(self.current_quantity) { None } else { Some(U) }
    } else if pressed.intersects(D) {
      if self.current_quantity == self.goal_quantity || self.goal_dist_down(self.current_quantity) > self.goal_dist_up(self.current_quantity) { None } else { Some(D) }
    } else {
      Some(Input::empty())
    }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)> {
    let pressed = self.pressed_state.get_pressed_input(input);
    if pressed.intersects(A) {
      if self.current_quantity != self.goal_quantity { None } else {
        gb.restore(s);
        gb.input(input);
        gb.step();
        Some((gb.save(), Some(())))
      }
    } else if pressed.intersects(B) {
      None
    } else if pressed.intersects(U) {
      if self.current_quantity == self.goal_quantity || self.goal_dist_down(self.current_quantity) < self.goal_dist_up(self.current_quantity) { None } else {
        gb.restore(s);
        gb.input(input);
        gb.step();
        let new_state = gb.save();
        self.pressed_state = PressedInputState::from_gb(gb);
        self.current_quantity = gb.gb.read_memory(R::MENU_ITEM_QUANTITY_MEM_ADDRESS);
        self.max_quantity = gb.gb.read_memory(R::MENU_MAX_ITEM_QUANTITY_MEM_ADDRESS);
        Some((new_state, None))
      }
    } else if pressed.intersects(D) {
      if self.current_quantity == self.goal_quantity || self.goal_dist_down(self.current_quantity) > self.goal_dist_up(self.current_quantity) { None } else {
        gb.restore(s);
        gb.input(input);
        gb.step();
        let new_state = gb.save();
        self.pressed_state = PressedInputState::from_gb(gb);
        self.current_quantity = gb.gb.read_memory(R::MENU_ITEM_QUANTITY_MEM_ADDRESS);
        self.max_quantity = gb.gb.read_memory(R::MENU_MAX_ITEM_QUANTITY_MEM_ADDRESS);
        Some((new_state, None))
      }
    } else {
      gb.restore(s);
      while gb.get_input_frame_lo() == s.get_input_frame_lo() && gb.get_input_frame_hi() == s.get_input_frame_hi() {
        gb.input(input);
        gb.delay_step();
      }
      let new_state = gb.save();
      self.pressed_state = PressedInputState::from_gb(gb);
      self.current_quantity = gb.gb.read_memory(R::MENU_ITEM_QUANTITY_MEM_ADDRESS);
      self.max_quantity = gb.gb.read_memory(R::MENU_MAX_ITEM_QUANTITY_MEM_ADDRESS);
      Some((new_state, None))
    }
  }
}
