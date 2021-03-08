use serde_derive::{Serialize, Deserialize};

use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;
use std::cmp::Ordering;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeOptionsPlanState {
  progress: ChangeOptionsProgress,
  hjoy5_state: HJoy5State,
}
impl PartialOrd for ChangeOptionsPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.progress.partial_cmp(&other.progress)
  }
}
impl PartialEq for ChangeOptionsPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChangeOptionsProgress {
  ChangeTextSpeed,
  MoveToAnimations,
  ChangeAnimations,
  MoveToBattleStyle,
  ChangeBattleStyle,
  Close,
}
// Plan to progress DisplayOptionMenu inputs
pub struct ChangeOptionsPlan {
  // instance state
  hjoy5_state: HJoy5State,
  progress: ChangeOptionsProgress,

  // config state
  is_yellow: bool,
}
impl ChangeOptionsPlan {
  pub fn new() -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      hjoy5_state: HJoy5State::unknown(),
      progress: ChangeOptionsProgress::ChangeTextSpeed,

      // Default config state.
      is_yellow: false,
    }
  }
  pub fn for_yellow(self) -> Self { Self { is_yellow: true, ..self } }
}
impl<R: MultiRom> Plan<R> for ChangeOptionsPlan {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::ChangeOptionsState(ChangeOptionsPlanState { progress: self.progress, hjoy5_state: self.hjoy5_state.clone() })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::ChangeOptionsState(ChangeOptionsPlanState { progress, hjoy5_state, }) = state {
      self.progress = *progress;
      self.hjoy5_state = hjoy5_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.hjoy5_state = HJoy5State::from_gb_state(gb, state);
    self.progress = if self.is_yellow { ChangeOptionsProgress::Close } else { ChangeOptionsProgress::ChangeTextSpeed };
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input {
    match self.progress {
      ChangeOptionsProgress::ChangeTextSpeed => { L },
      ChangeOptionsProgress::MoveToAnimations => { D },
      ChangeOptionsProgress::ChangeAnimations => { L | R },
      ChangeOptionsProgress::MoveToBattleStyle => { D },
      ChangeOptionsProgress::ChangeBattleStyle => { L | R },
      ChangeOptionsProgress::Close => { B | START },
    }
  }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    let hjoy5 = self.hjoy5_state.get_hjoy5(input);
    if hjoy5.intersects(B | START) {
      if self.progress == ChangeOptionsProgress::Close { Some(hjoy5 & (B | START)) } else { None }
    } else if hjoy5.intersects(A) {
      if self.progress == ChangeOptionsProgress::Close { None } else { Some(Input::empty()) }
    } else if hjoy5.intersects(D) {
      if self.progress == ChangeOptionsProgress::Close { Some(Input::empty()) }
      else if self.progress == ChangeOptionsProgress::MoveToAnimations || self.progress == ChangeOptionsProgress::MoveToBattleStyle { Some(D) }
      else { None }
    } else if hjoy5.intersects(U) {
      if self.progress == ChangeOptionsProgress::Close { Some(Input::empty()) }
      else { None }
    } else if hjoy5.intersects(L) {
      if self.progress == ChangeOptionsProgress::ChangeTextSpeed || self.progress == ChangeOptionsProgress::ChangeAnimations || self.progress == ChangeOptionsProgress::ChangeBattleStyle { Some(L) } else { None }
    } else if hjoy5.intersects(R) {
      if self.progress == ChangeOptionsProgress::ChangeAnimations || self.progress == ChangeOptionsProgress::ChangeBattleStyle { Some(R) } else { None }
    } else { Some(Input::empty()) }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    fn transition<R: MultiRom>(this: &mut ChangeOptionsPlan, gb: &mut Gb<R>, s: &GbState, input: Input, new_progress: ChangeOptionsProgress) -> Option<(GbState, Option<()>)> {
      gb.restore(s);
      gb.input(input);
      gb.step();
      let new_state = gb.save();
      this.hjoy5_state = HJoy5State::from_gb(gb);
      this.progress = new_progress;
      Some((new_state, None))
    }
    fn delay<R: MultiRom>(this: &mut ChangeOptionsPlan, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
      gb.restore(s);
      while gb.get_input_frame_lo() == s.get_input_frame_lo() && gb.get_input_frame_hi() == s.get_input_frame_hi() {
        gb.input(input);
        gb.delay_step();
      }
      let new_state = gb.save();
      this.hjoy5_state = HJoy5State::from_gb(gb);
      Some((new_state, None))
    }

    let hjoy5 = self.hjoy5_state.get_hjoy5(input);
    if hjoy5.intersects(B | START) {
      if self.progress == ChangeOptionsProgress::Close {
        gb.restore(s);
        gb.input(input);
        gb.step();
        Some((gb.save(), Some(())))
      } else { None }
    } else if hjoy5.intersects(A) {
      if self.progress == ChangeOptionsProgress::Close { None } else { delay(self, gb, s, input) }
    } else if hjoy5.intersects(D) {
      match self.progress {
        ChangeOptionsProgress::Close => delay(self, gb, s, input),
        ChangeOptionsProgress::MoveToAnimations => transition(self, gb, s, input, ChangeOptionsProgress::ChangeAnimations),
        ChangeOptionsProgress::MoveToBattleStyle => transition(self, gb, s, input, ChangeOptionsProgress::ChangeBattleStyle),
        _ => None,
      }
    } else if hjoy5.intersects(U) {
      match self.progress {
        ChangeOptionsProgress::Close => delay(self, gb, s, input),
        _ => None,
      }
    } else if hjoy5.intersects(L) {
      match self.progress {
        ChangeOptionsProgress::ChangeTextSpeed => transition(self, gb, s, input, ChangeOptionsProgress::MoveToAnimations),
        ChangeOptionsProgress::ChangeAnimations => transition(self, gb, s, input, ChangeOptionsProgress::MoveToBattleStyle),
        ChangeOptionsProgress::ChangeBattleStyle => transition(self, gb, s, input, ChangeOptionsProgress::Close),
        _ => None,
      }
    } else if hjoy5.intersects(R) {
      match self.progress {
        ChangeOptionsProgress::ChangeAnimations => transition(self, gb, s, input, ChangeOptionsProgress::MoveToBattleStyle),
        ChangeOptionsProgress::ChangeBattleStyle => transition(self, gb, s, input, ChangeOptionsProgress::Close),
        _ => None,
      }
    } else {
      delay(self, gb, s, input)
    }
  }
}
