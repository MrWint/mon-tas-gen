use serde_derive::{Serialize, Deserialize};
use std::cmp::Ordering;

use crate::metric::*;
use crate::metric::overworld::gen1::*;
use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverworldPushBoulderPlanState {
  joypad_overworld_state: JoypadOverworldState,
  push_boulder_result: PushBoulderResult,
}
impl PartialOrd for OverworldPushBoulderPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.push_boulder_result.partial_cmp(&other.push_boulder_result)
  }
}
impl PartialEq for OverworldPushBoulderPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress JoypadOverworld inputs
pub struct OverworldPushBoulderPlan {
  // instance state
  joypad_overworld_state: JoypadOverworldState,
  push_boulder_result: PushBoulderResult,

  // config state
  direction: Input,
}
impl OverworldPushBoulderPlan {
  pub fn new(direction: Input) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      joypad_overworld_state: JoypadOverworldState::unknown(),
      push_boulder_result: PushBoulderResult::NotTriedPushing,

      // Default config state.
      direction,
    }
  }
}
impl<R: MultiRom + JoypadOverworldAddresses + Gen1OverworldAddresses + Gen1DVAddresses + PushBoulderAddresses> Plan<R> for OverworldPushBoulderPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::OverworldPushBoulderState(OverworldPushBoulderPlanState { push_boulder_result: self.push_boulder_result, joypad_overworld_state: self.joypad_overworld_state.clone() })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::OverworldPushBoulderState(OverworldPushBoulderPlanState { push_boulder_result, joypad_overworld_state, }) = state {
      self.joypad_overworld_state = joypad_overworld_state.clone();
      self.push_boulder_result = *push_boulder_result;
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.joypad_overworld_state = JoypadOverworldState::from_gb_state(gb, state);
    self.push_boulder_result = PushBoulderResult::NotTriedPushing;
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
        assert!(dir == direction);
        self.push_boulder_result = get_push_boulder_result(gb);
        if self.push_boulder_result == PushBoulderResult::BoulderDustScheduled {
          gb.step();
          return Some((gb.save(), Some(())));
        }
        gb.delay_step();
        let new_state = gb.save();
        self.joypad_overworld_state = JoypadOverworldState::from_gb(gb);
        Some((new_state, None))
      },
      OverworldInteractionResult::Collision => {
        self.push_boulder_result = get_push_boulder_result(gb);
        if self.push_boulder_result == PushBoulderResult::BoulderDustScheduled {
          gb.step();
          return Some((gb.save(), Some(())));
        }
        gb.delay_step();
        let new_state = gb.save();
        self.joypad_overworld_state = JoypadOverworldState::from_gb(gb);
        Some((new_state, None))
      }
      OverworldInteractionResult::NoAction => {
        self.push_boulder_result = get_push_boulder_result(gb);
        if self.push_boulder_result == PushBoulderResult::BoulderDustScheduled {
          gb.step();
          return Some((gb.save(), Some(())));
        }
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PushBoulderResult {
  NotTriedPushing,
  TriedPushing,
  BoulderDustScheduled,
}

pub fn get_push_boulder_result<R: JoypadAddresses + PushBoulderAddresses>(gb: &mut dyn GbI<R>) -> PushBoulderResult {
  assert!(gb.step_until(&[R::AFTER_TRY_PUSHING_BOULDER_ADDRESS]) != 0);
  let flags = gb.gb().read_memory(R::BOULDER_FLAGS_MEM_ADDRESS);
  if flags & 0b0000_0010 != 0 { PushBoulderResult::BoulderDustScheduled }
  else if flags & 0b0100_0000 != 0 { PushBoulderResult::TriedPushing }
  else { PushBoulderResult::NotTriedPushing }
}