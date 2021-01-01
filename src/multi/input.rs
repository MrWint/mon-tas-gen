use serde_derive::{Serialize, Deserialize};

use std::collections::HashMap;

use super::*;


#[derive(Clone, Serialize, Deserialize)]
pub struct InputLog {
  inputs: HashMap<u32, Input>,
  input_len: [u32; 2], // [lo, hi]
}
impl InputLog {
  pub fn new() -> InputLog {
    InputLog {
      inputs: HashMap::new(),
      input_len: [0, 0],
    }
  }
  #[inline]
  pub fn len_max(&self) -> u32 {
    std::cmp::max(self.input_len[0], self.input_len[1])
  }
  #[inline]
  pub fn len_min(&self) -> u32 {
    std::cmp::min(self.input_len[0], self.input_len[1])
  }
  #[inline]
  pub fn len_lo(&self) -> u32 {
    self.input_len[0]
  }
  #[inline]
  pub fn len_hi(&self) -> u32 {
    self.input_len[1]
  }
  pub fn get_input_lo(&self, i: u32) -> Option<Input> {
    if self.len_lo() <= i {
      None
    } else {
      Some(*self.inputs.get(&i).unwrap_or(&Input::empty()) & inputs::LO_INPUTS)
    }
  }
  pub fn get_input_hi(&self, i: u32) -> Option<Input> {
    if self.len_hi() <= i {
      None
    } else {
      Some(*self.inputs.get(&i).unwrap_or(&Input::empty()) & inputs::HI_INPUTS)
    }
  }
  pub fn set_input_lo(&mut self, i: u32, input: Input) {
    if self.len_lo() <= i {
      *self.inputs.entry(i).or_insert(Input::empty()) |= input & inputs::LO_INPUTS;
      self.input_len[0] = i + 1;
    } else {
      assert!(*self.inputs.get(&i).unwrap_or(&Input::empty()) & inputs::LO_INPUTS == input & inputs::LO_INPUTS);
    }
  }
  pub fn set_input_hi(&mut self, i: u32, input: Input) {
    if self.len_hi() <= i {
      *self.inputs.entry(i).or_insert(Input::empty()) |= input & inputs::HI_INPUTS;
      self.input_len[1] = i + 1;
    } else {
      assert!(*self.inputs.get(&i).unwrap_or(&Input::empty()) & inputs::HI_INPUTS == input & inputs::HI_INPUTS);
    }
  }
  #[allow(dead_code)]
  pub fn create_inputs(&mut self) -> Vec<Input> {
    let input_len = self.inputs.iter().filter_map(|(f, i)| if i.is_empty() { None } else { Some(*f) }).max().unwrap_or(0) as usize + 1;
    let mut result = Vec::with_capacity(input_len);
    result.resize(input_len, Input::empty());
    for (frame, input) in self.inputs.iter() {
      if !input.is_empty() {
        result[*frame as usize] = *input;
      }
    }
    log::info!("creating inputs done: #inputs: {}", result.len());
    result
  }
}