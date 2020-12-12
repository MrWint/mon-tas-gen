use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct Gen1ITSkipTextsSegment {
  num_texts: usize,
  confirm_input: Option<Input>,
  buffer_size: usize,
}
impl WithOutputBufferSize for Gen1ITSkipTextsSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl Gen1ITSkipTextsSegment {
  pub fn new(num_texts: usize) -> Self {
    assert!(num_texts > 0);
    Gen1ITSkipTextsSegment {
      num_texts,
      confirm_input: None,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  /// How often is an "end" of the text expected (can happen when special characters are printed).
  pub fn with_skip_ends(self, _ends_to_be_skipped: u32) -> Self { self }
  /// Which button is used to confirm after the end of each TextSegment.
  pub fn with_confirm_input(self, confirm_input: Input) -> Self { Self { confirm_input: Some(confirm_input), ..self } }
}

impl<R: Rom + JoypadLowSensitivityAddresses> Segment<R> for Gen1ITSkipTextsSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, mut sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    assert!(self.confirm_input.is_none() || self.confirm_input.unwrap() == Input::A || self.confirm_input.unwrap() == Input::B);
    let confirm_input_a = self.confirm_input.unwrap_or(Input::empty()).contains(Input::A);
    let beat_input = if confirm_input_a { &[Input::A, Input::B] } else { &[Input::B, Input::A] };
    let offbeat_input = if confirm_input_a { &[Input::B, Input::A] } else { &[Input::A, Input::B] };
    for i in 1..self.num_texts {
      sb = JoypadLowSensitivitySegment::new(if (self.num_texts - i) & 1 == 0 { beat_input } else { offbeat_input }).with_buffer_size(self.buffer_size).execute(gbe, sb);
    }
    let confirm_input: &[Input] = match self.confirm_input {
      None => &[Input::B, Input::A],
      Some(Input::A) => &[Input::A],
      Some(Input::B) => &[Input::B],
      _ => panic!("unknown confirm input {:?}", self.confirm_input),
    };
    JoypadLowSensitivitySegment::new(confirm_input).with_buffer_size(self.buffer_size).execute_split(gbe, sb)
  }
}
