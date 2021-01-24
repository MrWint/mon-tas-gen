use crate::gb::*;
use crate::metric::*;
use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;
use std::collections::HashMap;
use std::marker::PhantomData;



pub struct JoypadLowSensitivitySegment<R, M> {
  inputs: &'static [Input],
  metric: M,
  buffer_size: usize,
  _rom: PhantomData<R>,
}
impl<R: JoypadAddresses + RngAddresses, F: Metric<R>, V> JoypadLowSensitivitySegment<R, F> where F: Fn(&mut Gb<R>) -> Option<V> {
  pub fn with_metric_fn(inputs: &'static [Input], f: F) -> Self { Self::with_metric(inputs, f) }
}
impl<R: JoypadAddresses + RngAddresses, M: Metric<R>> JoypadLowSensitivitySegment<R, M> {
  pub fn with_metric(inputs: &'static [Input], metric: M) -> Self {
    Self {
      inputs,
      metric,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      _rom: PhantomData,
    }
  }
}
impl<R> JoypadLowSensitivitySegment<R, NullMetric> {
  pub fn new(inputs: &'static [Input]) -> Self {
    Self {
      inputs,
      metric: NullMetric,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      _rom: PhantomData,
    }
  }
}
impl<R, M> WithOutputBufferSize for JoypadLowSensitivitySegment<R, M> {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + JoypadLowSensitivityAddresses, M: Metric<R>> Segment<R> for JoypadLowSensitivitySegment<R, M> {
  type Key = M::ValueType;

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    gbe.execute(sb, move |gb, s, tx| {
      gb.restore(&s);
      gb.input(Input::empty());
      let hit = gb.step_until(R::JOYPAD_USE_ADDRESSES); // don't check for the metric until after the joypad input has been used.
      assert!(hit != 0); // This is a decision point, there has to be a joypad use.
      let last_inputs = Input::from_bits_truncate(gb.gb.read_memory(R::JOYPAD_LAST_MEM_ADDRESS));
      log::debug!("currently pressed inputs: {:?}", last_inputs);
      for &input in self.inputs {
        gb.restore(&s);
        if input.intersects(last_inputs) {
          // Add wait frame if desired inputs collide with last inputs.
          gb.input(Input::empty());
          gb.step();
        }
        gb.input(input);
        let hit = gb.step_until(R::JOYPAD_USE_ADDRESSES); // don't check for the metric until after the joypad input has been used.
        assert!(hit != 0); // This is a decision point, there has to be a joypad use.
        if let Some(value) = self.metric.evaluate(gb) {
          if gb.skipped_relevant_inputs { // restore state if metric overran next input
            gb.restore(&s);
            gb.input(input);
          }
          if !gb.is_at_input { gb.step(); }
          tx.send(gb.save_with_value(value)).unwrap();
        }
      }
    }).into_state_buffer_map(self.buffer_size)
  }
}
