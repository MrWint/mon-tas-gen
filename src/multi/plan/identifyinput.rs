use crate::multi::*;
use crate::rom::*;


pub struct IdentifyInputPlan;
impl PlanBase for IdentifyInputPlan {
  fn save(&self) -> PlanState { PlanState::EmptyState }
  fn restore(&mut self, _state: &PlanState) { }
  fn reset(&mut self) { }
  fn is_safe(&self) -> bool { true }
  fn canonicalize_input(&self, _input: Input) -> Option<Input> { Some(Input::empty()) }
}
impl<R: Rom + InputIdentificationAddresses> Plan<R> for IdentifyInputPlan {
  type Value = ();

  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    let post_addresses: Vec<_> = R::II_ADDRESSES.iter().map(|(_, _, post, _)| *post).collect();
    gb.restore(s);
    gb.input(input);
    let hit = gb.step_until(&post_addresses);
    if let Some(name) = R::II_ADDRESSES.iter().find_map(|(_, _, post, name)| if *post == hit { Some(name) } else { None }) {
      log::info!("IdentifyInput: Identified input as {}", name);
      gb.step();
    } else {
      log::info!("IdentifyInput: Input not identified");
    }
    Some((gb.save(), Some(())))
  }
}
