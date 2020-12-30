use super::*;

/// Determines the result of the JoypadLowSensitivity output hJoy5 given the input.
/// Assumes to be called directly after gb.input().
#[allow(dead_code)]
pub struct HJoy5Metric;
impl<R: Rom + JoypadLowSensitivityAddresses> Metric<R> for HJoy5Metric {
  type ValueType = Input;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    let input = Input::from_bits_truncate(gb.gb.read_memory(R::JOYPAD_INPUT_MEM_ADDRESS));
    let last_input = Input::from_bits_truncate(gb.gb.read_memory(R::JOYPAD_LAST_MEM_ADDRESS));
    let pressed_input = input - last_input;
    let h_joy7 = gb.gb.read_memory(R::JOYPAD_HJOY7_MEM_ADDRESS);
    let h_joy5 = if h_joy7 > 0 { input } else { pressed_input };
    if !pressed_input.is_empty() { return Some(h_joy5); }
    let hit = gb.step_until(&[R::JOYPAD_FRAME_COUNTER_CHECK_ADDRESS]);
    if hit != R::JOYPAD_FRAME_COUNTER_CHECK_ADDRESS { return None; }
    let frame_counter = gb.gb.read_memory(R::JOYPAD_FRAME_COUNTER_MEM_ADDRESS);
    if frame_counter > 0 { return Some(Input::empty()); }
    if !input.contains(Input::A | Input::B) { return Some(h_joy5); }
    let h_joy6 = gb.gb.read_memory(R::JOYPAD_HJOY6_MEM_ADDRESS);
    if h_joy6 > 0 { Some(h_joy5) } else { Some(Input::empty()) }
  }
}
