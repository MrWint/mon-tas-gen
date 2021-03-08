use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::metric::*;
use crate::metric::overworld::gen1::*;
use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverworldEncounterPlanState {
  joypad_overworld_state: JoypadOverworldState,
}
impl PartialOrd for OverworldEncounterPlanState {
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    Some(Ordering::Equal)
  }
}
impl PartialEq for OverworldEncounterPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress JoypadOverworld inputs
pub struct OverworldEncounterPlan<M> {
  // instance state
  joypad_overworld_state: JoypadOverworldState,

  // config state
  direction: Input,
  metric: M,
}
impl<M> OverworldEncounterPlan<M> {
  pub fn new(direction: Input, metric: M) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      joypad_overworld_state: JoypadOverworldState::unknown(),

      // Default config state.
      direction,
      metric,
    }
  }
}
impl<R: MultiRom + JoypadOverworldAddresses + Gen1OverworldAddresses + Gen1DVAddresses, M: Metric<R>> Plan<R> for OverworldEncounterPlan<M> {
  type Value = M::ValueType;

  fn save(&self) -> PlanState {
    PlanState::OverworldEncounterState(OverworldEncounterPlanState { joypad_overworld_state: self.joypad_overworld_state.clone() })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::OverworldEncounterState(OverworldEncounterPlanState { joypad_overworld_state, }) = state {
      self.joypad_overworld_state = joypad_overworld_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.joypad_overworld_state = JoypadOverworldState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    let (held, pressed) = self.joypad_overworld_state.get_input(input);
    if pressed.intersects(START) { return None; } // Opening start menu is not allowed
    if pressed.intersects(A) { return Some(A); } // Allow pressing A to delay
    let dir = effective_direction(held);
    if dir == self.direction { return Some(self.direction); }
    if dir.is_empty() { return Some(Input::empty()); }
    None
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)> {
    let (held, _) = self.joypad_overworld_state.get_input(input);
    let dir = effective_direction(held);
    gb.restore(s);
    gb.input(input);
    match get_overworld_interaction_result(gb) {
      OverworldInteractionResult::WildEncounter { .. } => {
        gb.restore(s);
        gb.input(input);
        if let Some(metric_value) = self.metric.evaluate(gb) {
          if !gb.is_at_input() { gb.step(); }
          Some((gb.save(), Some(metric_value)))
        } else { None }
      },
      OverworldInteractionResult::Turned { direction } => {
        assert!(dir == direction);
        gb.delay_step();
        let new_state = gb.save();
        self.joypad_overworld_state = JoypadOverworldState::from_gb(gb);
        Some((new_state, None))
      },
      OverworldInteractionResult::Collision => {
        gb.delay_step();
        let new_state = gb.save();
        self.joypad_overworld_state = JoypadOverworldState::from_gb(gb);
        Some((new_state, None))
      }
      OverworldInteractionResult::NoAction => {
        gb.delay_step();
        let new_state = gb.save();
        self.joypad_overworld_state = JoypadOverworldState::from_gb(gb);
        Some((new_state, None))
      }
      _ => None,
    }
  }
}

fn effective_direction(input: Input) -> Input {
  if input.intersects(D) { D }
  else if input.intersects(U) { U }
  else if input.intersects(L) { L }
  else if input.intersects(R) { R }
  else { Input::empty() }
}
