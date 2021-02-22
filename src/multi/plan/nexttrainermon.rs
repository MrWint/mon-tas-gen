use crate::multi::*;
use crate::rom::*;

pub struct NextTrainerMonPlan;

impl NextTrainerMonPlan {
  pub fn with_level_up<R: MultiRom + TextAddresses>() -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // enemy // mon // fainted
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // gained // num XP
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // grew to level // X
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(3))); // trainer // sent out // mon
    ListPlan::new(plans)
  }
  pub fn with_learn_move<R: MultiRom + TextAddresses>() -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // enemy // mon // fainted
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // gained // num XP
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // grew to level // X
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(3))); // mon // learned // move
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(3))); // trainer // sent out // mon
    ListPlan::new(plans)
  }
}
