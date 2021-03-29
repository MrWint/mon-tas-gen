use crate::{metric::*, multi::*};
use crate::rom::*;

pub struct NextTrainerMonPlan;

impl NextTrainerMonPlan {
  pub fn new<R: MultiRom + TextAddresses>() -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // enemy // mon // fainted
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // gained // num XP
    plans.push(Box::new(TextPlan::new().with_skip_ends(3))); // trainer // sent out // mon
    ListPlan::new(plans)
  }
  pub fn new_it<R: MultiRom>() -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsITPlan::new(1))); // enemy // mon // fainted
    plans.push(Box::new(SkipTextsITPlan::new(1))); // mon // gained // num XP
    ListPlan::new(plans)
  }
  pub fn with_metric<R: MultiRom + TextAddresses, M: 'static + Metric<R, ValueType=()>>(metric: M) -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // enemy // mon // fainted
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // gained // num XP
    plans.push(Box::new(TextPlan::with_metric(metric, false).with_skip_ends(3))); // trainer // sent out // mon
    ListPlan::new(plans)
  }
  pub fn with_level_up<R: MultiRom + TextAddresses>() -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // enemy // mon // fainted
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // gained // num XP
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // grew to level // X
    plans.push(Box::new(TextPlan::new().with_skip_ends(3))); // trainer // sent out // mon
    ListPlan::new(plans)
  }
  pub fn with_level_up_it<R: MultiRom>() -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsITPlan::new(1))); // enemy // mon // fainted
    plans.push(Box::new(SkipTextsITPlan::new(1))); // mon // gained // num XP
    plans.push(Box::new(SkipTextsITPlan::new(1))); // mon // grew to level // X
    ListPlan::new(plans)
  }
  pub fn with_level_up_and_metric<R: MultiRom + TextAddresses, M: 'static + Metric<R, ValueType=()>>(metric: M) -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // enemy // mon // fainted
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // gained // num XP
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // grew to level // X
    plans.push(Box::new(TextPlan::with_metric(metric, false).with_skip_ends(3))); // trainer // sent out // mon
    ListPlan::new(plans)
  }
  pub fn with_learn_move<R: MultiRom + TextAddresses>() -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // enemy // mon // fainted
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // gained // num XP
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // grew to level // X
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(3))); // mon // learned // move
    plans.push(Box::new(TextPlan::new().with_skip_ends(3))); // trainer // sent out // mon
    ListPlan::new(plans)
  }
  pub fn with_skip_learning_move<R: MultiRom + TextAddresses + HandleMenuInputAddresses>() -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // enemy // mon // fainted
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // gained // num XP
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // grew to level // X
    plans.push(Box::new(OverrideMovePlan::skip()));
    plans.push(Box::new(TextPlan::new().with_skip_ends(3))); // trainer // sent out // mon
    ListPlan::new(plans)
  }
}
