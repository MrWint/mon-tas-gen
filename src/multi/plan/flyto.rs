use serde_derive::{Serialize, Deserialize};
use std::cmp::{Ordering, min};

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlyToPlanState {
  hjoy5_state: HJoy5State,
  distance_up: u8,
  distance_down: u8,
}
impl PartialOrd for FlyToPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    min(other.distance_up, other.distance_down).partial_cmp(&min(self.distance_up, self.distance_down))
  }
}
impl PartialEq for FlyToPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

// Plan to progress LoadTownMap_Fly inputs
pub struct FlyToPlan {
  // instance state
  hjoy5_state: HJoy5State,
  distance_up: u8,
  distance_down: u8,

  // config state
  goal_index: u8,
}
impl FlyToPlan {
  pub fn to_pallet_town() -> Self { Self::to(0) }
  pub fn to_viridian_city() -> Self { Self::to(1) }
  pub fn to_pewter_city() -> Self { Self::to(2) }
  pub fn to_cerulean_city() -> Self { Self::to(3) }
  pub fn to_lavender_town() -> Self { Self::to(4) }
  pub fn to_vermilion_city() -> Self { Self::to(5) }
  pub fn to_celadon_city() -> Self { Self::to(6) }
  pub fn to_fuchsia_city() -> Self { Self::to(7) }
  pub fn to_cinnabar_island() -> Self { Self::to(8) }
  pub fn to_indigo_plateau() -> Self { Self::to(9) }
  pub fn to_saffron_city() -> Self { Self::to(10) }
  fn to(goal_index: u8) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      hjoy5_state: HJoy5State::unknown(),
      distance_up: 0,
      distance_down: 0,
    
      // Default config state.
      goal_index,
    }
  }
}
impl<R: Rom + JoypadLowSensitivityAddresses + FlyLocationsAddresses> Plan<R> for FlyToPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::FlyToState(FlyToPlanState { distance_up: self.distance_up, distance_down: self.distance_down, hjoy5_state: self.hjoy5_state })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::FlyToState(FlyToPlanState { distance_up, distance_down , hjoy5_state }) = state {
      self.distance_up = *distance_up;
      self.distance_down = *distance_down;
      self.hjoy5_state = *hjoy5_state;
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    assert!(gb.gb.read_memory(R::FLY_LOCATIONS_MEM_ADDRESS + u16::from(self.goal_index)) < 0xFE);
    if self.goal_index == 0 {
      self.distance_up = 0;
      self.distance_down = 0;
    } else {
      self.distance_up = 1 + (1..self.goal_index).map(|i| gb.gb.read_memory(R::FLY_LOCATIONS_MEM_ADDRESS + u16::from(i))).filter(|v| v < &0xFE).count() as u8;
      self.distance_down = 1 + ((self.goal_index + 1)..11).map(|i| gb.gb.read_memory(R::FLY_LOCATIONS_MEM_ADDRESS + u16::from(i))).filter(|v| v < &0xFE).count() as u8;
    }
    self.hjoy5_state = HJoy5State::from_gb_state(gb, state);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input {
    if self.distance_up == 0 || self.distance_down == 0 { A }
    else if self.distance_down < self.distance_up { D }
    else if self.distance_down > self.distance_up { U }
    else { U | D }
  }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    let hjoy5 = self.hjoy5_state.get_hjoy5(input);
    if hjoy5.intersects(A) {
      if self.distance_up == 0 || self.distance_down == 0 { Some(A) } else { None }
    } else if hjoy5.intersects(U) {
      if self.distance_down >= self.distance_up { Some(U) } else { None }
    } else if hjoy5.intersects(D) {
      if self.distance_down <= self.distance_up { Some(D) } else { None }
    } else if hjoy5.intersects(B) { None }
    else { Some(Input::empty()) }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    let hjoy5 = self.hjoy5_state.get_hjoy5(input);
    if hjoy5.intersects(A) {
      if self.distance_up == 0 || self.distance_down == 0 {
        gb.restore(s);
        gb.input(input);
        gb.step();
        return Some((gb.save(), Some(())));
      } else { None }
    } else if hjoy5.intersects(U) {
      if self.distance_down >= self.distance_up {
        gb.restore(s);
        gb.input(input);
        gb.step();
        let new_state = gb.save();
        self.hjoy5_state = HJoy5State::from_gb(gb);
        self.distance_up -= 1;
        Some((new_state, None))
      } else { None }
    } else if hjoy5.intersects(D) {
      if self.distance_down <= self.distance_up {
        gb.restore(s);
        gb.input(input);
        gb.step();
        let new_state = gb.save();
        self.hjoy5_state = HJoy5State::from_gb(gb);
        self.distance_down -= 1;
        Some((new_state, None))
      } else { None }
    } else if hjoy5.intersects(B) { None }
    else {
      gb.restore(s);
      gb.input(input);
      gb.delay_step();
      let new_state = gb.save();
      self.hjoy5_state = HJoy5State::from_gb(gb);
      Some((new_state, None))
    }
  }
}
