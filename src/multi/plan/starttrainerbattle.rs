use crate::multi::*;
use crate::rom::*;

pub struct StartTrainerBattlePlan;

impl StartTrainerBattlePlan {
  pub fn with_pre_battle_texts<R: MultiRom + TextAddresses>(num_pre_battle_texts: u32) -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    if num_pre_battle_texts > 0 {
      plans.push(Box::new(SkipTextsPlan::new(num_pre_battle_texts)));
      plans.push(Box::new(HoldTextDisplayOpenPlan::new()));
    }
    plans.push(Box::new(SkipTextsPlan::new(1).with_skip_ends(1))); // trainer // wants to fight
    plans.push(Box::new(TextPlan::new().with_skip_ends(6))); // trainer // sent out // mon // go // mon // !
    ListPlan::new(plans)
  }
  pub fn with_pre_battle_texts_it<R: MultiRom + TextAddresses>(num_pre_battle_texts: u32) -> ListPlan<R> {
    let mut plans: Vec<Box<dyn Plan<R, Value=()>>> = vec![];
    if num_pre_battle_texts > 0 {
      plans.push(Box::new(SkipTextsITPlan::new(num_pre_battle_texts)));
      plans.push(Box::new(HoldTextDisplayOpenPlan::new()));
    }
    plans.push(Box::new(SkipTextsITPlan::new(1))); // trainer // wants to fight
    ListPlan::new(plans)
  }
}