use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::metric::overworld::gen1::*;
use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverworldOpenStartMenuPlanState {
  joypad_overworld_state: JoypadOverworldState,
}
impl PartialOrd for OverworldOpenStartMenuPlanState {
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    Some(Ordering::Equal)
  }
}
impl PartialEq for OverworldOpenStartMenuPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress JoypadOverworld inputs
pub struct OverworldOpenStartMenuPlan {
  // instance state
  joypad_overworld_state: JoypadOverworldState,

  // config state
}
impl OverworldOpenStartMenuPlan {
  pub fn new() -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      joypad_overworld_state: JoypadOverworldState::unknown(),

      // Default config state.
    }
  }
}
impl<R: MultiRom + JoypadOverworldAddresses + Gen1OverworldAddresses + Gen1DVAddresses> Plan<R> for OverworldOpenStartMenuPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::OverworldOpenStartMenuState(OverworldOpenStartMenuPlanState { joypad_overworld_state: self.joypad_overworld_state.clone() })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::OverworldOpenStartMenuState(OverworldOpenStartMenuPlanState { joypad_overworld_state, }) = state {
      self.joypad_overworld_state = joypad_overworld_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.joypad_overworld_state = JoypadOverworldState::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { START }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    let (_, pressed) = self.joypad_overworld_state.get_input(input);
    Some(pressed & START)
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    gb.restore(s);
    gb.input(input);
    match get_overworld_interaction_result(gb) {
      OverworldInteractionResult::DisplayText { id: 0 } => {
        gb.step();
        Some((gb.save(), Some(())))
      },
      OverworldInteractionResult::NoAction | OverworldInteractionResult::Collision => {
        gb.delay_step();
        let new_state = gb.save();
        self.joypad_overworld_state = JoypadOverworldState::from_gb(gb);
        Some((new_state, None))
      }
      _ => None,
    }
  }
}
