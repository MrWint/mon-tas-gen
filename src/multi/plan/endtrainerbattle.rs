use crate::multi::*;
use crate::rom::*;

pub struct EndTrainerBattlePlan;

impl EndTrainerBattlePlan {
  pub fn with_level_up<R: MultiRom + TextAddresses>(num_defeat_texts: u32) -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // enemy // mon // fainted
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // gained // num XP
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // grew to level // X
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // I defeated U
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // First defeat text
    if num_defeat_texts > 1 { plans.push(Box::new(SkipTextsPlan::new(num_defeat_texts - 1))); } // Additional defeat texts
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // player got // X for winning
    ListPlan::new(plans)
  }

  pub fn with_learn_move<R: MultiRom + TextAddresses>(num_defeat_texts: u32) -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // enemy // mon // fainted
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // gained // num XP
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // mon // grew to level // X
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(3))); // mon // learned // move
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // I defeated U
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // First defeat text
    if num_defeat_texts > 1 { plans.push(Box::new(SkipTextsPlan::new(num_defeat_texts - 1))); } // Additional defeat texts
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(2))); // player got // X for winning
    ListPlan::new(plans)
  }
}
