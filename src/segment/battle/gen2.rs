use crate::constants::*;
use crate::gb::*;
use crate::rom::*;
use crate::segment::Metric;

#[allow(dead_code)]
pub struct Gen2AIChooseMoveMetric {}
impl<R: JoypadAddresses + Gen2AIChooseMoveAddresses> Metric<R> for Gen2AIChooseMoveMetric {
  type ValueType = Move;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    if gb.run_until_or_next_input_use(&[R::AFTER_AI_CHOOSE_MOVE_ADDRESS]) == 0 { return None; }
    Some(Move::from_index(gb.gb.read_memory(R::CUR_ENEMY_MOVE_MEM_ADDRESS)).unwrap())
  }
}
