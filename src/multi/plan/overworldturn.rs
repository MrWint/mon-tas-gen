use crate::metric::overworld::gen1::*;
use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

// Plan to progress JoypadOverworld inputs
pub struct OverworldTurnPlan {
  // instance state
  joypad_overworld_state: JoypadOverworldState,

  // config state
  direction: Input,
}
impl OverworldTurnPlan {
  pub fn new(direction: Input) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      joypad_overworld_state: JoypadOverworldState::unknown(),

      // Default config state.
      direction,
    }
  }
}
impl<R: MultiRom + JoypadOverworldAddresses + Gen1OverworldAddresses + Gen1DVAddresses> Plan<R> for OverworldTurnPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::OverworldTurnState { joypad_overworld_state: self.joypad_overworld_state.clone() }
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::OverworldTurnState { joypad_overworld_state, } = state {
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
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    let (held, _) = self.joypad_overworld_state.get_input(input);
    let dir = effective_direction(held);
    gb.restore(s);
    gb.input(input);
    match get_overworld_interaction_result(gb) {
      OverworldInteractionResult::Turned { direction } => {
        assert!(dir ==  direction);
        if direction == self.direction {
          gb.step();
          return Some((gb.save(), Some(())));
        }
        gb.delay_step();
        let new_state = gb.save();
        self.joypad_overworld_state = JoypadOverworldState::from_gb(gb);
        Some((new_state, None))
      },
      OverworldInteractionResult::Collision => {
        if dir == self.direction {
          gb.step();
          return Some((gb.save(), Some(())));
        }
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
