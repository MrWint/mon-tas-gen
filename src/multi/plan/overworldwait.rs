use crate::metric::overworld::gen1::*;
use crate::multi::*;
use crate::rom::*;

// Plan to progress JoypadOverworld inputs, waiting one input (with result NoAction)
pub struct OverworldWaitPlan {
  // instance state

  // config state
  expected_result: OverworldInteractionResult,
}
impl OverworldWaitPlan {
  pub fn new() -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.

      // Default config state.
      expected_result: OverworldInteractionResult::NoAction,
    }
  }
  pub fn auto_walk(direction: Input) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.

      // Default config state.
      expected_result: OverworldInteractionResult::Walked { direction },
    }
  }
  pub fn trainer_battle(id: u8) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.

      // Default config state.
      expected_result: OverworldInteractionResult::TrainerBattle { species: id },
    }
  }
}
impl<R: MultiRom + JoypadOverworldAddresses + Gen1OverworldAddresses + Gen1DVAddresses> Plan<R> for OverworldWaitPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::OverworldWaitState
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::OverworldWaitState = state {
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, _gb: &mut Gb<R>, _state: &GbState) {}
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() }

  fn canonicalize_input(&self, _input: Input) -> Option<Input> { Some(Input::empty()) }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    gb.restore(s);
    gb.input(input);
    if get_overworld_interaction_result(gb) == self.expected_result {
      gb.step();
      Some((gb.save(), Some(())))
    } else { None }
  }
}
