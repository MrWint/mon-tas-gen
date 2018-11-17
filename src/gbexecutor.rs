use gambatte::*;
use gb::*;
use rom::*;
use segment::*;
use statebuffer::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub trait GbExecutor<R: JoypadAddresses + RngAddresses + TextAddresses> {
  fn execute<O: Send + 'static, I: IntoIterator<Item=State>, F: Fn(&mut Gb<R>, State, Sender<O>) + Send + Clone + 'static>(&mut self, states: I, f: F) -> SdlUpdatingIter<O>;
  fn get_initial_state(&mut self) -> State;

  fn execute_fn_sized<I: IntoIterator<Item=State>, F: Fn(&mut Gb<R>) + Send + Clone + 'static>(&mut self, sb: I, f: F, buffer_size: usize) -> StateBuffer {
    
    StateBuffer::from_iter_sized(self.execute(sb, move |gb, s, tx| {
      gb.restore(&s);
      f(gb);
      tx.send(gb.save()).unwrap();
    }), buffer_size)
  }

  fn execute_split_fn_sized<K: Eq + Hash + Debug + Send + 'static, I: IntoIterator<Item=State>, F: Fn(&mut Gb<R>, State, Sender<(K, State)>) + Send + Clone + 'static>(&mut self, sb: I, f: F, buffer_size: usize) -> HashMap<K, StateBuffer> {
    let mut result: HashMap<K, StateBuffer> = HashMap::new();
    for (value, s) in self.execute(sb, f) {
      result.entry(value).or_insert(StateBuffer::with_max_size(buffer_size)).add_state(s);
    }
    result
  }

  fn execute_segment<I: IntoIterator<Item=State>>(&mut self, sb: I, input: Input) -> StateBuffer {
    self.execute(sb, move |gb, s, tx| {
      tx.send(MoveSegment::new(input).with_max_skips(10).execute(gb, vec![s])).unwrap();
    }).flatten().collect()
  }

  fn execute_text_segment<I: IntoIterator<Item=State>>(&mut self, sb: I, num: u32, input: Input) -> StateBuffer {
    self.execute(sb, move |gb, s, tx| {
      tx.send(SkipTextsSegment::new(num, input).execute(gb, vec![s])).unwrap();
    }).flatten().collect()
  }
}

pub struct SingleGb<R> {
  gb: Gb<R>,
}
impl<R: JoypadAddresses + RngAddresses + TextAddresses> GbExecutor<R> for SingleGb<R> {
  fn execute<O: Send + 'static, I: IntoIterator<Item=State>, F: Fn(&mut Gb<R>, State, Sender<O>) + Send + Clone + 'static>(&mut self, states: I, f: F) -> SdlUpdatingIter<O> {
    let (tx, rx) = channel::<O>();

    for s in states {
      f(&mut self.gb, s, tx.clone());
    }

    SdlUpdatingIter { rx: rx }
  }

  fn get_initial_state(&mut self) -> State {
    self.gb.restore_initial_state();
    self.gb.save()
  }
}
impl<R: BasicRomInfo + JoypadAddresses + RngAddresses> SingleGb<R> {
  pub fn with_screen() -> Self {
    Gambatte::init_screens(1 /* num screens */, 1 /* scale */);
    SingleGb {
      gb: Gb::<R>::create(Gambatte::create_on_screen(0 /* screen */, false /* equal length frames */)),
    }
  }
  pub fn no_screen() -> Self {
    SingleGb {
      gb: Gb::<R>::create(Gambatte::create_on_screen(-1 /* screen */, false /* equal length frames */)),
    }
  }
}

trait FnBox<R>: Send {
  fn call_box(self: Box<Self>, gb: &mut Gb<R>);
}
impl<R, F: FnOnce(&mut Gb<R>) + Send> FnBox<R> for F {
  fn call_box(self: Box<F>, gb: &mut Gb<R>) {
    (*self)(gb)
  }
}

pub struct GbPool<R: 'static> {
  jobs: Sender<Box<FnBox<R>>>,
}

impl<R: BasicRomInfo + JoypadAddresses> GbPool<R> {
  pub fn with_screen() -> GbPool<R> {
    Self::new(true)
  }
  pub fn no_screen() -> GbPool<R> {
    Self::new(false)
  }

  fn new(has_screen: bool) -> GbPool<R> {
    let num_threads = ::num_cpus::get();
    if has_screen {
      Gambatte::init_screens(num_threads as u32 /* num screens */, 1 /* scale */);
    }

    let (tx, rx) = channel::<Box<FnBox<R>>>();

    let job_receiver = Arc::new(Mutex::new(rx));

    // Threadpool threads
    for i in 0..num_threads {
      let job_receiver = Arc::clone(&job_receiver);
      thread::spawn(move || {
        let mut gb = Gb::<R>::create(Gambatte::create_on_screen(if has_screen {i as i32} else {-1} /* screen */, false /* equal length frames */));

        loop {
          let message = job_receiver.lock().expect("Worker thread unable to lock job_receiver").recv();
          match message {
            Ok(job) => job.call_box(&mut gb),
            Err(..) => break, // The Pool was dropped.
          };
        }
      });
    }

    GbPool {
      jobs: tx,
    }
  }
}

pub struct SdlUpdatingIter<T> {
    rx: Receiver<T>
}
impl<T> Iterator for SdlUpdatingIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
      loop {
        match self.rx.recv_timeout(Duration::from_millis(10)) {
          Ok(item) => return Some(item),
          Err(::std::sync::mpsc::RecvTimeoutError::Disconnected) => return None,
          Err(::std::sync::mpsc::RecvTimeoutError::Timeout) => Gambatte::handle_sdl_events(),
        }
      }
  }
}

impl<R: BasicRomInfo + JoypadAddresses + RngAddresses + TextAddresses> GbExecutor<R> for GbPool<R> {
  fn execute<O: Send + 'static, I: IntoIterator<Item=State>, F: Fn(&mut Gb<R>, State, Sender<O>) + Send + Clone + 'static>(&mut self, states: I, f: F) -> SdlUpdatingIter<O> {
    let (tx, rx) = channel::<O>();

    for s in states.into_iter() {
      let tx = tx.clone();
      let f = f.clone();
      let job = Box::new(move |gb: &mut Gb<R>| {
        f(gb, s, tx)
      });
      self.jobs.send(job).unwrap();
    }

    SdlUpdatingIter { rx: rx }
  }

  fn get_initial_state(&mut self) -> State {
    let (tx, rx) = channel::<State>();

    let job = Box::new(move |gb: &mut Gb<R>| {
      gb.restore_initial_state();
      tx.send(gb.save()).unwrap();
    });
    self.jobs.send(job).unwrap();

    rx.recv().unwrap()
  }
}
