use gambatte::*;
use gb::*;
use rom::*;
use statebuffer::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub trait StateKey: Eq + Hash + Debug + Send + 'static {}
impl<T: Eq + Hash + Debug + Send + 'static> StateKey for T {}

pub trait GbExecutor<R: Rom> {
  fn execute<K: StateKey, I: IntoIterator<Item=State>, F: Fn(&mut Gb<R>, State, Sender<(K, State)>) + Send + Sync>(&mut self, states: I, f: F) -> SdlUpdatingIter<(K, State)>;
  fn get_initial_state(&mut self) -> State;
}

pub struct SingleGb<R> {
  gb: Gb<R>,
}
impl<R: Rom> GbExecutor<R> for SingleGb<R> {
  fn execute<K: StateKey, I: IntoIterator<Item=State>, F: Fn(&mut Gb<R>, State, Sender<(K, State)>) + Send + Sync>(&mut self, sb: I, f: F) -> SdlUpdatingIter<(K, State)> {
    let (tx, rx) = channel::<(K, State)>();
    for s in sb {
      f(&mut self.gb, s, tx.clone());
    }
    SdlUpdatingIter { rx }
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

pub struct GbPool<R: Rom> {
  jobs: Sender<Box<FnOnce(&mut Gb<R>) + Send>>,
}

impl<R: Rom> GbPool<R> {
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

    let (tx, rx) = channel::<Box<FnOnce(&mut Gb<R>) + Send>>();

    let job_receiver = Arc::new(Mutex::new(rx));

    // Threadpool threads
    for i in 0..num_threads {
      let job_receiver = Arc::clone(&job_receiver);
      thread::spawn(move || {
        let mut gb = Gb::<R>::create(Gambatte::create_on_screen(if has_screen {i as i32} else {-1} /* screen */, false /* equal length frames */));

        loop {
          let message = job_receiver.lock().expect("Worker thread unable to lock job_receiver").recv();
          match message {
            Ok(job) => job(&mut gb),
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
    rx: Receiver<T>,
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
impl<K: StateKey> SdlUpdatingIter<(K, State)> {
  #[allow(dead_code)]
  pub fn into_state_buffer_map(self, buffer_size: usize) -> HashMap<K, StateBuffer> {
    let mut result: HashMap<K, StateBuffer> = HashMap::new();
    for (value, s) in self {
      result.entry(value).or_insert_with(|| StateBuffer::with_max_size(buffer_size)).add_state(s);
    }
    result
  }
}
impl SdlUpdatingIter<((), State)> {
  #[allow(dead_code)]
  pub fn into_state_buffer(self, buffer_size: usize) -> StateBuffer {
    let mut result = StateBuffer::with_max_size(buffer_size);
    for (_, s) in self {
      result.add_state(s);
    }
    result
  }
}

type GbFn<'a, R, K> = Fn(&mut Gb<R>, State, Sender<(K, State)>) + Send + Sync + 'a;
impl<R: Rom> GbExecutor<R> for GbPool<R> {
  fn execute<K: StateKey, I: IntoIterator<Item=State>, F: Fn(&mut Gb<R>, State, Sender<(K, State)>) + Send + Sync>(&mut self, states: I, f: F) -> SdlUpdatingIter<(K, State)> {
    // Wrap functon in an Arc.
    let f: Arc<GbFn<R, K>> = Arc::new(f);
    // Erase lifetime constraints: The resulting iterator must be fully consumed before the life time of F ends (ideally within the same statement) for this to be safe.
    let f: Arc<GbFn<'static, R, K>> = unsafe { ::std::mem::transmute(f) };

    let (tx, rx) = channel::<(K, State)>();

    for s in states.into_iter() {
      let tx = tx.clone();
      let f = Arc::clone(&f);
      let job = Box::new(move |gb: &mut Gb<R>| { f(gb, s, tx) });
      self.jobs.send(job).unwrap();
    }

    SdlUpdatingIter { rx }
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
