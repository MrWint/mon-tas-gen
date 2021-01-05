use crate::multi::*;
use crate::rom::*;


pub fn identify_input_print<R: Rom + InputIdentificationAddresses>(gb: &mut Gb<R>, s: &GbState) {
  if let Some(name) = identify_input(gb, s) {
    log::info!("identify_input_print: Identified input as {}", name);
  } else {
    log::info!("identify_input_print: Input not identified");
  }
}
pub fn identify_input<R: Rom + InputIdentificationAddresses>(gb: &mut Gb<R>, s: &GbState) -> Option<&'static str> {
  identify_input_with(gb, s, Input::empty())
}
fn identify_input_with<R: Rom + InputIdentificationAddresses>(gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<&'static str> {
  let post_addresses: Vec<_> = R::II_ADDRESSES.iter().map(|(_, _, post, _)| *post).collect();
  gb.restore(s);
  gb.input(input);
  let hit = gb.step_until(&post_addresses);
  R::II_ADDRESSES.iter().find_map(|(_, _, post, name)| if *post == hit { Some(*name) } else { None })
}
pub struct IdentifyInputPlan;
impl<R: MultiRom + InputIdentificationAddresses> Plan<R> for IdentifyInputPlan {
  type Value = ();

  fn save(&self) -> PlanState { PlanState::IdentifyInputState }
  fn restore(&mut self, _state: &PlanState) { }
  fn initialize(&mut self, _gb: &mut Gb<R>, _state: &GbState) { }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() }
  fn canonicalize_input(&self, _input: Input) -> Option<Input> { Some(Input::empty()) }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    if let Some(name) = identify_input_with(gb, s, input) {
      log::info!("IdentifyInputPlan: Identified input as {}", name);
      gb.step();
    } else {
      log::info!("IdentifyInputPlan: Input not identified");
    }
    Some((gb.save(), Some(())))
  }
}
