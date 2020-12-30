use serde_derive::{Serialize, Deserialize};

use std::collections::{HashMap, HashSet};
use std::ops::BitAnd;

use super::*;

/// Inputs split across two consecutive frames
pub struct SplitInputs {
  input_descs: HashSet<(InputDesc, InputDesc)>,
}
impl SplitInputs {
  pub fn any() -> Self {
    Self {
      input_descs: [(InputDesc::any(), InputDesc::any())].iter().cloned().collect(),
    }
  }
  pub fn combine(&self, other: &Self) -> Self {
    Self {
      input_descs: self.input_descs.iter().flat_map(|&id| other.input_descs.iter().filter_map(move |&oid| Some(((id.0 & oid.0)?, (id.1 & oid.1)?)))).collect()
    }
  }
  pub fn iter(&self) -> std::collections::hash_set::Iter<'_, (InputDesc, InputDesc)> {
    self.input_descs.iter()
  }
}
impl BitAnd<(InputDesc, InputDesc)> for SplitInputs {
  type Output = Self;

  fn bitand(self, rhs: (InputDesc, InputDesc)) -> Self {
    Self {
      input_descs: self.input_descs.iter().filter_map(|&id| Some(((id.0 & rhs.0)?, (id.1 & rhs.1)?))).collect()
    }
  }
}

pub struct Inputs {
  input_descs: HashSet<InputDesc>,
}
impl Inputs {
  pub fn new(input_descs: HashSet<InputDesc>) -> Self {
    Self {
      input_descs,
    }
  }
  pub fn any() -> Self {
    Self {
      input_descs: [InputDesc::any()].iter().cloned().collect(),
    }
  }
  pub fn combine(&self, other: &Self) -> Self {
    Self {
      input_descs: self.input_descs.iter().flat_map(|&id| other.input_descs.iter().filter_map(move |&oid| id & oid)).collect()
    }
  }
  pub fn iter(&self) -> std::collections::hash_set::Iter<'_, InputDesc> {
    self.input_descs.iter()
  }
  pub fn ignore_lo(&self) -> Self {
    Self {
      input_descs: self.input_descs.iter().map(InputDesc::ignore_lo).collect()
    }
  }
  pub fn ignore_hi(&self) -> Self {
    Self {
      input_descs: self.input_descs.iter().map(InputDesc::ignore_hi).collect()
    }
  }
  pub fn split_lo_hi(&self) -> SplitInputs {
    SplitInputs {
      input_descs: self.input_descs.iter().map(InputDesc::split_lo_hi).collect()
    }
  }
  pub fn split_hi_lo(&self) -> SplitInputs {
    SplitInputs {
      input_descs: self.input_descs.iter().map(InputDesc::split_hi_lo).collect()
    }
  }
  pub fn split_only_lo(&self) -> SplitInputs {
    SplitInputs {
      input_descs: self.input_descs.iter().map(|id| (InputDesc::any(), id.ignore_hi())).collect()
    }
  }
  pub fn split_only_hi(&self) -> SplitInputs {
    SplitInputs {
      input_descs: self.input_descs.iter().map(|id| (InputDesc::any(), id.ignore_lo())).collect()
    }
  }
  pub fn split(&self) -> SplitInputs {
    SplitInputs {
      input_descs: self.input_descs.iter().map(|&id| (InputDesc::any(), id)).collect()
    }
  }
}
impl BitAnd<InputDesc> for Inputs {
  type Output = Self;

  fn bitand(self, rhs: InputDesc) -> Self {
    Self {
      input_descs: self.input_descs.iter().filter_map(|&id| id & rhs).collect()
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputDesc {
  input: Input,
  ignore_mask: Input,
}
impl BitAnd for InputDesc {
  type Output = Option<Self>;

  fn bitand(self, other: Self) -> Self::Output {
    if !((self.input ^ other.input) - (self.ignore_mask | other.ignore_mask)).is_empty() {
      // Found conflicting inputs
      return None;
    }
    Some(InputDesc {
      input: (self.input - self.ignore_mask) | (other.input - other.ignore_mask),
      ignore_mask: self.ignore_mask & other.ignore_mask,
    })
  }
}
impl InputDesc {
  pub fn new(input: Input, ignore_mask: Input) -> Self {
    Self {
      input: input - ignore_mask,
      ignore_mask,
    }
  }
  pub fn any() -> Self {
    Self {
      input: Input::empty(),
      ignore_mask: Input::all(),
    }
  }
  pub fn get_input(&self) -> Input {
    assert!((self.input & self.ignore_mask).is_empty()); // ignored inputs should not be set
    self.input
  }
  pub fn ignore_lo(&self) -> Self {
    InputDesc {
      input: self.input - inputs::LO_INPUTS,
      ignore_mask: self.ignore_mask | inputs::LO_INPUTS,
    }
  }
  pub fn ignore_hi(&self) -> Self {
    InputDesc {
      input: self.input - inputs::HI_INPUTS,
      ignore_mask: self.ignore_mask | inputs::HI_INPUTS,
    }
  }
  pub fn split_lo_hi(&self) -> (Self, Self) {
    (self.ignore_hi(), self.ignore_lo())
  }
  pub fn split_hi_lo(&self) -> (Self, Self) {
    (self.ignore_lo(), self.ignore_hi())
  }
}

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