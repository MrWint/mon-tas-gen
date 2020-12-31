use serde_derive::{Serialize, Deserialize};

use crate::rom::*;
use super::*;

use std::cmp::Ordering;
use std::rc::Rc;

pub trait PlanBase {
  /// Saves current plan state
  fn save(&self) -> PlanState;
  /// Restores saved plan state
  fn restore(&mut self, state: &PlanState);
  /// Resets plan state to initial value
  fn reset(&mut self);
  /// Checks whether the current state is considered safe, i.e. there is guaranteed to be a sequence of inputs which completes it.
  fn is_safe(&self) -> bool;
  fn canonicalize_input(&self, input: Input) -> Option<Input>;
}

pub trait Plan<R: Rom>: PlanBase {
  /// Type of the resulting values of this plan.
  type Value;

  /// Returns possible inputs from the current state. Each input description denotes a potentially different acceptable outcome.
  fn get_inputs(&self, gb: &mut Gb<R>, state: &GbState) -> Inputs;
  /// Executes the given input on the given state. If successful, returns a new state and updates the internal state, potentially finishing and returning a value.
  /// If the plan returns, the internal state is no longer guaranteed to be consistent.
  /// The execution step may skip multiple input uses, as long as all of them happen on the same input frame and use the same given input.
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)>;
}
pub struct NullPlan;
impl<R: Rom> Plan<R> for NullPlan {
  type Value = ();

  fn get_inputs(&self, _gb: &mut Gb<R>, _state: &GbState) -> Inputs { Inputs::any() }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    gb.restore(state);
    gb.input(input);
    gb.step();
    Some((gb.save(), None))
  }
}
impl PlanBase for NullPlan {
  fn save(&self) -> PlanState { PlanState::EmptyState }
  fn restore(&mut self, _state: &PlanState) { }
  fn reset(&mut self) { }
  fn is_safe(&self) -> bool { true }
  fn canonicalize_input(&self, _input: Input) -> Option<Input> { Some(Input::empty()) }
}

pub struct ListPlan<R: Rom> {
  plans: Vec<Box<dyn Plan<R, Value=()>>>,
  cur_item: usize,
}
impl<R: Rom> ListPlan<R> {
  pub fn new(plans: Vec<Box<dyn Plan<R, Value=()>>>) -> Self {
    assert!(!plans.is_empty());
    Self {
      plans,
      cur_item: 0,
    }
  }
}
impl<R: Rom> Plan<R> for ListPlan<R> {
  type Value = ();

  fn get_inputs(&self, gb: &mut Gb<R>, state: &GbState) -> Inputs {
    assert!(self.cur_item < self.plans.len());
    self.plans[self.cur_item].get_inputs(gb, state)
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    assert!(self.cur_item < self.plans.len());
    if let Some((new_state, result)) = self.plans[self.cur_item].execute_input(gb, state, input) {
      if result.is_some() {
        self.cur_item += 1;
        if self.cur_item >= self.plans.len() {
          return Some((new_state, Some(())));
        }
        self.plans[self.cur_item].reset();
      }
      Some((new_state, None))
    } else { None }
  }
}
impl<R: Rom> PlanBase for ListPlan<R> {
  fn save(&self) -> PlanState {
    PlanState::ListState {
      cur_item: self.cur_item,
      sub_plan: if self.cur_item >= self.plans.len() { None } else { Some(Rc::new(self.plans[self.cur_item].save())) },
    }
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::ListState { cur_item, sub_plan } = state {
      assert!(*cur_item < self.plans.len());
      self.cur_item = *cur_item;
      if let Some(sub_plan) = sub_plan {
        self.plans[*cur_item].restore(sub_plan);
      } else {
        self.plans[*cur_item].reset();
      }
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn reset(&mut self) {
    self.cur_item = 0;
    self.plans[0].reset();
  }
  fn is_safe(&self) -> bool {
    self.cur_item >= self.plans.len() || self.plans[self.cur_item].is_safe()
  }
  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    assert!(self.cur_item < self.plans.len());
    self.plans[self.cur_item].canonicalize_input(input)
  }
}

#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub enum PlanState {
  EmptyState,
  ListState { cur_item: usize, sub_plan: Option<Rc<PlanState>> },
  SkipIntroState { inputs_until_auto_pass: u32, }
}
impl PartialEq for PlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

impl PartialOrd for PlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self {
      PlanState::EmptyState => {
        if let PlanState::EmptyState = other {
          Some(Ordering::Equal)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::ListState { cur_item, sub_plan } => {
        if let PlanState::ListState { cur_item: other_item, sub_plan: other_plan } = other {
          if cur_item != other_item {
            cur_item.partial_cmp(other_item)
          } else {
            sub_plan.partial_cmp(other_plan)
          }
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
      PlanState::SkipIntroState { inputs_until_auto_pass } => {
        if let PlanState::SkipIntroState { inputs_until_auto_pass: other_inputs_until_auto_pass } = other {
          other_inputs_until_auto_pass.partial_cmp(inputs_until_auto_pass)
        } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); }
      },
    }
  }
}

mod identifyinput;
pub use identifyinput::*;
mod skipintro;
pub use skipintro::*;
