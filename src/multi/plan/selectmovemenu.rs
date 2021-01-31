use crate::constants::*;
use crate::metric::*;
use crate::metric::battle::*;
use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

// Plan to progress HandleMenuInput_ inputs, selecting a chosen battle move
pub struct SelectMoveMenuPlan<M> {
  // instance state
  handle_menu_input_state: HandleMenuInputState,
  move_index: u8,
  num_moves: u8,

  // config state
  mov: Move,
  metric: M,
}
impl SelectMoveMenuPlan<NullMetric> {
  pub fn new(mov: Move) -> Self { Self::with_metric(mov, NullMetric) }
}
impl<M> SelectMoveMenuPlan<M> {
  pub fn with_metric(mov: Move, metric: M) -> Self {
    Self {
      handle_menu_input_state: HandleMenuInputState::unknown(),
      move_index: 0,
      num_moves: 0,

      mov,
      metric,
    }
  }
  #[inline]
  fn get_effective_index(&self) -> u8 {
    self.handle_menu_input_state.current_item - 1
  }
  fn distance_down(&self) -> u8 {
    (self.move_index + self.num_moves - self.get_effective_index()) % self.num_moves
  }
  fn distance_up(&self) -> u8 {
    (self.get_effective_index() + self.num_moves - self.move_index) % self.num_moves
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses + HandleMenuInputAddresses + InputIdentificationAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses, M: Metric<R>> Plan<R> for SelectMoveMenuPlan<M> {
  type Value = M::ValueType;

  fn save(&self) -> PlanState {
    PlanState::SelectMoveMenuState { handle_menu_input_state: self.handle_menu_input_state.clone(), move_index: self.move_index, num_moves: self.num_moves, distance_to_goal: std::cmp::min(self.distance_up(), self.distance_down()) }
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::SelectMoveMenuState { handle_menu_input_state, move_index, num_moves, distance_to_goal: _ } = state {
      self.handle_menu_input_state = handle_menu_input_state.clone();
      self.move_index = *move_index;
      self.num_moves = *num_moves;
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.handle_menu_input_state = HandleMenuInputState::from_gb_state(gb, state);
    let move_infos = MoveInfosFn::new(Who::Player).invoke(gb);
    self.move_index = move_infos.iter().position(|move_info| move_info.mov == self.mov).expect("move not found") as u8;
    self.num_moves = move_infos.len() as u8;
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input {
    if self.get_effective_index() == self.move_index {
      return A;
    }

    let mut blocked_inputs = Input::empty();
    if self.distance_up() <= self.num_moves / 2 {
      blocked_inputs |= U;
    }
    if self.distance_down() <= self.num_moves / 2 {
      blocked_inputs |= D;
    }
    blocked_inputs
  }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    match self.handle_menu_input_state.get_result(input) {
      HandleMenuInputResult::DoNothing => Some(Input::empty()),
      HandleMenuInputResult::ScrollTo { current_item } => {
        assert!(current_item == self.handle_menu_input_state.current_item); // All up/down movements in the move select menu are watched and lead to an exit
        Some(Input::empty())
      },
      HandleMenuInputResult::Exit { current_item: _, input: exit_input } => {
        if exit_input.intersects(U) {
          let dist_up = self.distance_up();
          if dist_up > 0 && dist_up <= self.num_moves / 2 { Some(U) } else { None }
        } else if exit_input.intersects(D) {
          let dist_down = self.distance_down();
          if dist_down > 0 && dist_down <= self.num_moves / 2 { Some(D) } else { None }
        } else {
          if self.get_effective_index() == self.move_index && !exit_input.intersects(SELECT | B) {
            Some(A)
          } else { None }
        }
      },
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
        Some((new_state, None))
      },
      HandleMenuInputResult::ScrollTo { current_item } => {
        assert!(current_item == self.handle_menu_input_state.current_item); // All up/down movements in the move select menu are watched and lead to an exit
        gb.restore(s);
        gb.input(input);
        gb.delay_step();
        let new_state = gb.save();
        self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
        Some((new_state, None))
      },
      HandleMenuInputResult::Exit { current_item: _, input: exit_input } => {
        if exit_input.intersects(U) {
          let dist_up = self.distance_up();
          if dist_up > 0 && dist_up <= self.num_moves / 2 {
            gb.restore(s);
            gb.input(input);
            gb.step();
            let new_state = gb.save();
            self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
            Some((new_state, None))
          } else { None }
        } else if exit_input.intersects(D) {
          let dist_down = self.distance_down();
          if dist_down > 0 && dist_down <= self.num_moves / 2 { 
            gb.restore(s);
            gb.input(input);
            gb.step();
            let new_state = gb.save();
            self.handle_menu_input_state = HandleMenuInputState::from_gb(gb);
            Some((new_state, None))
          } else { None }
        } else {
          if self.get_effective_index() == self.move_index && !exit_input.intersects(SELECT | B) {
            gb.restore(s);
            gb.input(input);
            if let Some(metric_value) = self.metric.evaluate(gb) {
              if !gb.is_at_input() { gb.step(); }
              Some((gb.save(), Some(metric_value)))
            } else { None }
          } else { None }
        }
      },
    }
  }
}
