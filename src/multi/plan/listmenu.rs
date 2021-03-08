use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListMenuPlanState {
  handle_menu_input_state: HandleMenuInputState,
  list_scroll_offset: u8,
  list_count: u8,
  goal_dist: u8
}
impl PartialOrd for ListMenuPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.goal_dist.partial_cmp(&self.goal_dist)
  }
}
impl PartialEq for ListMenuPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress HandleMenuInput_ inputs, selecting an item in the a DisplayListMenuID list
pub struct ListMenuPlan {
  // instance state
  handle_menu_input_state: HandleMenuInputState,
  list_scroll_offset: u8,
  list_count: u8,

  // config state
  goal_list_item: u8,
  use_select: bool,
  quit: bool,
}
impl ListMenuPlan {
  pub fn quit() -> Self {
    Self {
      handle_menu_input_state: HandleMenuInputState::unknown(),
      list_scroll_offset: 0,
      list_count: 0,

      goal_list_item: 0,
      use_select: false,
      quit: true,
    }
  }
  pub fn choose(goal_list_item: u8) -> Self {
    Self {
      handle_menu_input_state: HandleMenuInputState::unknown(),
      list_scroll_offset: 0,
      list_count: 0,

      goal_list_item,
      use_select: false,
      quit: false,
    }
  }
  pub fn swap(goal_list_item: u8) -> Self {
    Self {
      handle_menu_input_state: HandleMenuInputState::unknown(),
      list_scroll_offset: 0,
      list_count: 0,

      goal_list_item,
      use_select: true,
      quit: false,
    }
  }
  fn goal_dist(&self, current_item: u8) -> u8 {
    if self.quit { return 0; }
    let current_list_item = current_item + self.list_scroll_offset;
    if current_list_item > self.goal_list_item { current_list_item - self.goal_list_item } else { self.goal_list_item - current_list_item }
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses + HandleMenuInputAddresses + InputIdentificationAddresses> Plan<R> for ListMenuPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::ListMenuState(ListMenuPlanState { handle_menu_input_state: self.handle_menu_input_state.clone(), list_scroll_offset: self.list_scroll_offset, list_count: self.list_count, goal_dist: self.goal_dist(self.handle_menu_input_state.current_item) })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::ListMenuState(ListMenuPlanState { handle_menu_input_state, list_scroll_offset, list_count, goal_dist: _ }) = state {
      self.handle_menu_input_state = handle_menu_input_state.clone();
      self.list_scroll_offset = *list_scroll_offset;
      self.list_count = *list_count;
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.handle_menu_input_state = HandleMenuInputState::from_gb_state(gb, state);
    self.list_scroll_offset = gb.gb.read_memory(R::LIST_SCROLL_OFFSET_MEM_ADDRESS);
    self.list_count = gb.gb.read_memory(R::LIST_COUNT_MEM_ADDRESS);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { if self.quit { B } else if self.use_select { SELECT } else { A } }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    match self.handle_menu_input_state.get_result(input) {
      HandleMenuInputResult::DoNothing => Some(Input::empty()),
      HandleMenuInputResult::ScrollTo { current_item } => {
        if self.quit || self.goal_dist(current_item) > self.goal_dist(self.handle_menu_input_state.current_item) { return None; } // Never scroll away from goal item
        Some(input & (U|D))
      },
      HandleMenuInputResult::Exit { current_item, input: exit_input } => {
        if exit_input.intersects(A) {
          if self.quit || self.use_select || current_item + self.list_scroll_offset != self.goal_list_item { None } else { Some(A) }
        } else if exit_input.intersects(B) {
          if !self.quit { None } else { Some(B) }
        } else if exit_input.intersects(SELECT) {
          if self.quit || !self.use_select || current_item + self.list_scroll_offset != self.goal_list_item { None } else { Some(SELECT) }
        } else if exit_input.intersects(D) {
          if self.list_count < self.list_scroll_offset + 3 { return Some(Input::empty()); } // scroll down not possible
          if self.quit || current_item + self.list_scroll_offset >= self.goal_list_item { return None; } // Don't scroll the wrong direction
          Some(D)
        } else { // up
          if self.list_scroll_offset == 0 { return Some(Input::empty()); } // scroll up not possible
          if self.quit || current_item + self.list_scroll_offset <= self.goal_list_item { return None; } // Don't scroll the wrong direction
          Some(U)
        }
      }
    }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)> {
    match self.handle_menu_input_state.get_result(input) {
      HandleMenuInputResult::DoNothing => {
        gb.restore(s);
        while gb.get_input_frame_lo() == s.get_input_frame_lo() && gb.get_input_frame_hi() == s.get_input_frame_hi() {
          gb.input(input);
          gb.delay_step();
        }
        let new_state = gb.save();
        self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
        self.list_scroll_offset = gb.gb.read_memory(R::LIST_SCROLL_OFFSET_MEM_ADDRESS);
        self.list_count = gb.gb.read_memory(R::LIST_COUNT_MEM_ADDRESS);
        Some((new_state, None))
      },
      HandleMenuInputResult::ScrollTo { current_item } => {
        if self.quit || self.goal_dist(current_item) > self.goal_dist(self.handle_menu_input_state.current_item) { return None; } // Never scroll away from goal item
        gb.restore(s);
        gb.input(input);
        if self.goal_dist(current_item) < self.goal_dist(self.handle_menu_input_state.current_item) { gb.step(); } else { gb.delay_step(); }
        
        let new_state = gb.save();
        self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
        self.list_scroll_offset = gb.gb.read_memory(R::LIST_SCROLL_OFFSET_MEM_ADDRESS);
        self.list_count = gb.gb.read_memory(R::LIST_COUNT_MEM_ADDRESS);
        Some((new_state, None))
      },
      HandleMenuInputResult::Exit { current_item, input: exit_input } => {
        if exit_input.intersects(A) {
          if self.quit || self.use_select || current_item + self.list_scroll_offset != self.goal_list_item { None } else {
            gb.restore(s);
            gb.input(input);
            gb.step();
            Some((gb.save(), Some(())))
          }
        } else if exit_input.intersects(B) {
          if !self.quit { None } else {
            gb.restore(s);
            gb.input(input);
            gb.step();
            Some((gb.save(), Some(())))
          }
        } else if exit_input.intersects(SELECT) {
          if self.quit || !self.use_select || current_item + self.list_scroll_offset != self.goal_list_item { None } else {
            gb.restore(s);
            gb.input(input);
            gb.step();
            Some((gb.save(), Some(())))
          }
        } else if exit_input.intersects(D) {
          if self.list_count < self.list_scroll_offset + 3 { // scroll down not possible
            gb.restore(s);
            gb.input(input);
            gb.delay_step();
            let new_state = gb.save();
            self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
            self.list_scroll_offset = gb.gb.read_memory(R::LIST_SCROLL_OFFSET_MEM_ADDRESS);
            self.list_count = gb.gb.read_memory(R::LIST_COUNT_MEM_ADDRESS);
            Some((new_state, None))
          } else if self.quit || current_item + self.list_scroll_offset >= self.goal_list_item { None } else { // Don't scroll the wrong direction
            gb.restore(s);
            gb.input(input);
            gb.step();
            let new_state = gb.save();
            self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
            self.list_scroll_offset = gb.gb.read_memory(R::LIST_SCROLL_OFFSET_MEM_ADDRESS);
            self.list_count = gb.gb.read_memory(R::LIST_COUNT_MEM_ADDRESS);
            Some((new_state, None))
          }
        } else { // up
          if self.list_scroll_offset == 0 { // scroll up not possible
            gb.restore(s);
            gb.input(input);
            gb.delay_step();
            let new_state = gb.save();
            self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
            self.list_scroll_offset = gb.gb.read_memory(R::LIST_SCROLL_OFFSET_MEM_ADDRESS);
            self.list_count = gb.gb.read_memory(R::LIST_COUNT_MEM_ADDRESS);
            Some((new_state, None))
          } else if self.quit || current_item + self.list_scroll_offset <= self.goal_list_item { None } else { // Don't scroll the wrong direction
            gb.restore(s);
            gb.input(input);
            gb.step();
            let new_state = gb.save();
            self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
            self.list_scroll_offset = gb.gb.read_memory(R::LIST_SCROLL_OFFSET_MEM_ADDRESS);
            self.list_count = gb.gb.read_memory(R::LIST_COUNT_MEM_ADDRESS);
            Some((new_state, None))
          }
        }
      },
    }
  }
}
