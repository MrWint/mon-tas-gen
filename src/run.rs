use gambatte::Input;
use log::{LevelFilter::*};
use montas::bk2::Bk2Writer;
use montas::gbexecutor::*;
use montas::metric::*;
use montas::rom::*;
use montas::segment::*;
use montas::statebuffer::*;
use montas::util::with_log_level;
use std::marker::PhantomData;

struct GbRunner<R: Rom> {
  gbe: RuntimeGbExecutor<R>,
  sb: StateBuffer,
  _rom: PhantomData<R>,
}

impl<R: Rom> GbRunner<R> {
  #[allow(dead_code)] fn pool_no_screen(inputs: &[Input]) -> Self { GbRunner::new(RuntimeGbExecutor::<R>::pool_no_screen(), inputs) }
  #[allow(dead_code)] fn pool_with_screen(inputs: &[Input]) -> Self { GbRunner::new(RuntimeGbExecutor::<R>::pool_with_screen(), inputs) }
  #[allow(dead_code)] fn single_no_screen(inputs: &[Input]) -> Self { GbRunner::new(RuntimeGbExecutor::<R>::single_no_screen(), inputs) }
  #[allow(dead_code)] fn single_with_screen(inputs: &[Input]) -> Self { GbRunner::new(RuntimeGbExecutor::<R>::single_with_screen(), inputs) }
}

impl<R: Rom> GbRunner<R> {
  fn new(mut gbe: RuntimeGbExecutor<R>, inputs: &[Input]) -> Self {
    let sb: StateBuffer = vec![gbe.get_state_from_inputs(inputs)].into_iter().collect();
    Self {
      gbe,
      sb,
      _rom: PhantomData,
    }
  }

  fn run<S: Segment<R, Key=()>>(&mut self, segment: S) {
    let sb = std::mem::replace(&mut self.sb, StateBuffer::new());
    self.sb = segment.execute(&mut self.gbe, sb);
  }
  #[allow(dead_code)]
  fn run_merge<S: Segment<R>>(&mut self, segment: S) {
    let sb = std::mem::replace(&mut self.sb, StateBuffer::new());
    self.sb = segment.execute_split(&mut self.gbe, sb).merge_state_buffers();
  }
  #[allow(dead_code)]
  fn run_debug<S: Segment<R, Key=()>>(&mut self, segment: S) {
    with_log_level(Debug, || {
      self.run(segment);
    });
  }
  #[allow(dead_code)]
  fn run_merge_debug<S: Segment<R>>(&mut self, segment: S) {
    with_log_level(Debug, || {
      self.run_merge(segment);
    });
  }
  #[allow(dead_code)]
  fn get_state_metric<V: StateValue + PartialEq + std::fmt::Debug, S: StateFn<R, OV=V> + Send + Sync>(&mut self, state_fn: S) -> V {
    self.gbe.execute_state(&self.sb, state_fn).get_value_assert_all_equal()
  }
  #[allow(dead_code)]
  fn get_state_metric_fn<V: StateValue + PartialEq + std::fmt::Debug, S: Fn(&dyn GbI<R>) -> V + Send + Sync>(&mut self, state_fn: S) -> V {
    self.gbe.execute_state_fn(&self.sb, state_fn).get_value_assert_all_equal()
  }

  #[allow(dead_code)]
  fn debug_print_states(&self) {
    println!("{:?}", self.sb);
  }

  #[allow(dead_code)]
  fn debug_write_memory(&mut self, address: u16, value: u8) {
    self.sb = self.gbe.execute(&self.sb, move |gb, s, tx| {
      gb.restore(&s);
      gb.gb.write_memory(address, value);
      tx.send(gb.save()).unwrap();
    }).into_state_buffer(self.sb.get_max_size());
  }

  fn debug_segment_end(&mut self, file_name: &str) {
    let chosen_state = (&self.sb).into_iter().min_by_key(|s| s.inputs.last().map(|(f, _)| f).unwrap_or(&0)).unwrap().clone();
    {
      log::info!("Creating sample input file...");
      let inputs = self.gbe.execute(&[&chosen_state], move |gb, s, tx| {
        gb.restore(&s);
        tx.send(s.replace_value(gb.create_inputs())).unwrap();
      }).into_iter().map(|s| s.value).min_by_key(Vec::len).unwrap();
      Bk2Writer::new::<R>().with_rtc_divisor_offset(RTC_DIVISOR_OFFSET).write_bk2(&format!("{}.bk2", file_name), &inputs).unwrap();
      log::info!("Inputs contain {} delays", chosen_state.num_delays);
    }
    log::info!("Rendering end states of {:?}", self.sb);
    self.gbe.execute(&self.sb, move |gb, s, tx| {
      gb.restore(&s);
      for _ in 0..10 {
        gb.input(Input::empty());
        gb.step();
        std::thread::sleep(std::time::Duration::from_millis(200));
      }
      for _ in 0..1000 {
        gb.input(Input::empty());
        gb.step();
      }
      std::thread::sleep(std::time::Duration::from_millis(200));
      tx.send(s).unwrap();
    }).into_state_buffer_map(0);
  }

  fn save(&self, file_name: &str) {
    self.sb.save(file_name);
    log::info!("saved states to {}", file_name);
  }
  fn load(&mut self, file_name: &str) {
    self.sb = StateBuffer::load(file_name);
  }
}



pub mod blue_glitchless;
pub mod blue_testing;
pub mod crystal_desync;
pub mod crystal_glitchless;
pub mod multi_blue;
pub mod multi_testing;
pub mod silver_testing;
pub mod yellow_testing;
