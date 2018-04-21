use gambatte::Input;
use gb::*;
use rom::*;
use statebuffer::StateBuffer;
use std::marker::PhantomData;

pub struct SkipTextsSegment<R: JoypadAddresses + RngAddresses + TextAddresses> {
  num_texts: u32,
  confirm_input: Input,
  _rom: PhantomData<R>,
}
impl<R: JoypadAddresses + RngAddresses + TextAddresses> SkipTextsSegment<R> {
  pub fn new(num_texts: u32, confirm_input: Input) -> Self {
    assert!(num_texts > 0);
    assert!(!confirm_input.contains(Input::A) || !confirm_input.contains(Input::B));
    SkipTextsSegment {
      num_texts: num_texts,
      confirm_input: confirm_input,
      _rom: PhantomData,
    }
  }
}
impl<R: JoypadAddresses + RngAddresses + TextAddresses> super::Segment for SkipTextsSegment<R> {
  type Rom = R;

  fn execute<I: IntoIterator<Item=State>>(&self, gb: &mut Gb<R>, iter: I) -> StateBuffer {
    let skip_input = if self.confirm_input.contains(Input::A) { Input::B } else { Input::A };
    let text_segment = super::TextSegment::new(skip_input);
    let confirm_segment = super::MoveSegment::new(self.confirm_input);
    let mut sb = text_segment.execute(gb, iter);
    for _ in 1..self.num_texts {
      sb = confirm_segment.execute(gb, sb);
      sb = text_segment.execute(gb, sb);
    }
    confirm_segment.execute(gb, sb)
  }
}
