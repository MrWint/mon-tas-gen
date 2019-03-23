use crate::rom::*;
use gambatte::*;

pub fn from_ftii<R: JoypadAddresses>(mut gb: Gambatte, hi_inputs: Vec<i32>, lo_inputs: Vec<i32>) -> Vec<Input> {
  let mut inputs:  Vec<Input> = vec![];

  let mut frame_start = gb.save_state();
  let mut hi_index = 0;
  let mut frame_start_hi_index = 0;
  let mut lo_index = 0;
  let mut frame_start_lo_index = 0;
  let mut frame_hi_input = -1;
  let mut frame_lo_input = -1;
  let mut frame_input = Input::empty();
  gb.set_input(frame_input);
  let mut hit = gb.step_until(&[R::JOYPAD_READ_HI_ADDRESS, R::JOYPAD_READ_LO_ADDRESS]);

  loop {
    if hit == None {
      inputs.push(frame_input);
      if hi_index >= hi_inputs.len() && lo_index >= lo_inputs.len() { break; }
      frame_start = gb.save_state();
      frame_start_hi_index = hi_index;
      frame_start_lo_index = lo_index;
      frame_hi_input = -1;
      frame_lo_input = -1;
      frame_input = Input::empty();
      gb.set_input(frame_input);
      hit = gb.step_until(&[R::JOYPAD_READ_HI_ADDRESS, R::JOYPAD_READ_LO_ADDRESS]);
    } else if hit == Some(R::JOYPAD_READ_HI_ADDRESS) {
      if *hi_inputs.get(hi_index).unwrap_or(&-1) != -1 && frame_hi_input != hi_inputs[hi_index] {
        if frame_hi_input != -1 { panic!("Can't create inputs: found input conflict between {:#x} and {:#x} at frame {}", frame_hi_input, hi_inputs[hi_index], inputs.len()); }
        else if frame_hi_input == -1 {
          frame_hi_input = hi_inputs[hi_index];
          frame_input = Input::from_bits_truncate((if frame_hi_input >= 0 { frame_hi_input } else { 0 } | if frame_lo_input >= 0 { frame_lo_input } else { 0 }) as u8);
          hi_index = frame_start_hi_index;
          lo_index = frame_start_lo_index;
          gb.load_state(&frame_start); // reset to start of frame and try again
          gb.set_input(frame_input);
          hit = gb.step_until(&[R::JOYPAD_READ_HI_ADDRESS, R::JOYPAD_READ_LO_ADDRESS]);
        }
      } else {
        hi_index += 1;
        hit = gb.step_until(&[R::JOYPAD_READ_LO_ADDRESS]);
      }
    } else if hit == Some(R::JOYPAD_READ_LO_ADDRESS) {
      if *lo_inputs.get(lo_index).unwrap_or(&-1) != -1 && frame_lo_input != lo_inputs[lo_index] {
        if frame_lo_input != -1 { panic!("Can't create inputs: found input conflict between {:#x} and {:#x} at frame {}", frame_lo_input, lo_inputs[lo_index], inputs.len()); }
        else if frame_lo_input == -1 {
          frame_lo_input = lo_inputs[lo_index];
          frame_input = Input::from_bits_truncate((if frame_hi_input >= 0 { frame_hi_input } else { 0 } | if frame_lo_input >= 0 { frame_lo_input } else { 0 }) as u8);
          hi_index = frame_start_hi_index;
          lo_index = frame_start_lo_index;
          gb.load_state(&frame_start); // reset to start of frame and try again
          gb.set_input(frame_input);
          hit = gb.step_until(&[R::JOYPAD_READ_HI_ADDRESS, R::JOYPAD_READ_LO_ADDRESS]);
        }
      } else {
        lo_index += 1;
        hit = gb.step_until(&[R::JOYPAD_READ_HI_ADDRESS]);
      }
    } else { panic!("Illegal return from step_until: {:#x}", hit.unwrap()); }
  }
  while inputs.last().unwrap_or(&Input::A).is_empty() { inputs.pop(); }
  log::info!("conversion done: inputs: {}", inputs.len());
  inputs
}

pub fn to_ftii<R: JoypadAddresses>(mut gb: Gambatte, inputs: Vec<Input>) -> (Vec<i32>, Vec<i32>) {

  let mut cur_frame = 0;
  let mut hi_inputs: Vec<i32> = vec![];
  let mut last_hi_inputs_used = true;
  let mut lo_inputs: Vec<i32> = vec![];
  let mut last_lo_inputs_used = true;
  let mut frame_input = if cur_frame < inputs.len() { inputs[cur_frame] } else { Input::empty() };
  gb.set_input(frame_input);
  let mut hit = gb.step_until(&[&[R::JOYPAD_READ_HI_ADDRESS, R::JOYPAD_READ_LO_ADDRESS], R::JOYPAD_USE_ADDRESSES].concat());

  let mut frame_read_count = 0;
  let mut multiple_reads_frames = vec![];
  let mut usage_without_read_frame_count = 0;
  log::info!("converting input size {} ", inputs.len());
  loop {
    if hit == None {
      if frame_read_count > 1 { multiple_reads_frames.push(cur_frame); }
      frame_read_count = 0;
      cur_frame += 1;
      frame_input = if cur_frame < inputs.len() { inputs[cur_frame] } else { Input::empty() };
      gb.set_input(frame_input);
      hit = gb.step_until(&[&[R::JOYPAD_READ_HI_ADDRESS, R::JOYPAD_READ_LO_ADDRESS], R::JOYPAD_USE_ADDRESSES].concat());
    } else if hit == Some(R::JOYPAD_READ_HI_ADDRESS) {
      if !last_hi_inputs_used { // never read, can't affect things, change to -1
        hi_inputs.pop();
        hi_inputs.push(-1);
      }
      hi_inputs.push(i32::from(frame_input.bits() & 0xf0));
      last_hi_inputs_used = false;
      hit = gb.step_until(&[&[R::JOYPAD_READ_LO_ADDRESS], R::JOYPAD_USE_ADDRESSES].concat());
    } else if hit == Some(R::JOYPAD_READ_LO_ADDRESS) {
      frame_read_count += 1;
      if !last_lo_inputs_used { // never read, can't affect things, change to -1
        lo_inputs.pop();
        lo_inputs.push(-1);
      }
      lo_inputs.push(i32::from(frame_input.bits() & 0x0f));
      last_lo_inputs_used = false;
      hit = gb.step_until(&[&[R::JOYPAD_READ_HI_ADDRESS], R::JOYPAD_USE_ADDRESSES].concat());
    } else if R::JOYPAD_USE_ADDRESSES.contains(&hit.unwrap()) {
      if frame_read_count == 0 { usage_without_read_frame_count += 1; }
      if cur_frame >= inputs.len() { break; }
      last_hi_inputs_used = true;
      last_lo_inputs_used = true;
      hit = gb.step_until(&[R::JOYPAD_READ_HI_ADDRESS, R::JOYPAD_READ_LO_ADDRESS]);
    } else { panic!("Illegal return from step_until: {:#x}", hit.unwrap()); }
  }
  // truncate unnecessary empty inputs at the end
  while *lo_inputs.last().unwrap_or(&1) <= 0 { lo_inputs.pop(); }
  while *hi_inputs.last().unwrap_or(&1) <= 0 { hi_inputs.pop(); }

  log::info!("conversion done: hi: {} lo: {}", hi_inputs.len(), lo_inputs.len());
  log::info!("#frames which used a value not read in their frame: {}", usage_without_read_frame_count);
  log::info!("frames which more than 1 read in their frame: {} ({:?})", multiple_reads_frames.len(), multiple_reads_frames);
  log::info!("relevant inputs: hi: {} lo: {}", hi_inputs.len() - hi_inputs.iter().filter(|&n| *n == -1).count(), lo_inputs.len() - lo_inputs.iter().filter(|&n| *n == -1).count());
  (hi_inputs, lo_inputs)
}
