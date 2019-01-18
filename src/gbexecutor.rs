use crate::gb::*;
use crate::rom::*;
use crate::sdl::*;
use crate::statebuffer::*;
use gambatte::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::mpsc::{channel, IntoIter, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

pub trait StateKey: Eq + Hash + Debug + Send + 'static {}
impl<T: Eq + Hash + Debug + Send + 'static> StateKey for T {}

pub trait GbExecutor<R: Rom> {
  fn execute<K: StateKey, I: IntoIterator<Item=State>, F: Fn(&mut Gb<R>, State, Sender<(K, State)>) + Send + Sync>(&mut self, states: I, f: F) -> GbResults<(K, State)>;
  fn get_initial_state(&mut self) -> State;
}

pub struct SingleGb<R> {
  gb: Gb<R>,
}
impl<R: Rom> GbExecutor<R> for SingleGb<R> {
  fn execute<K: StateKey, I: IntoIterator<Item=State>, F: Fn(&mut Gb<R>, State, Sender<(K, State)>) + Send + Sync>(&mut self, sb: I, f: F) -> GbResults<(K, State)> {
    let (tx, rx) = channel::<(K, State)>();
    for s in sb {
      f(&mut self.gb, s, tx.clone());
    }
    GbResults { rx }
  }

  fn get_initial_state(&mut self) -> State {
    self.gb.restore_initial_state();
    self.gb.save()
  }
}
impl<R: BasicRomInfo + JoypadAddresses + RngAddresses> SingleGb<R> {
  pub fn with_screen() -> Self {
    let sdl = Sdl::init_sdl(1 /* num screens */, 1 /* scale */);
    SingleGb {
      gb: Gb::<R>::create(false /* equal length frames */, SdlScreen::new(sdl, 0 /* screen */)),
    }
  }
  pub fn no_screen() -> Self {
    SingleGb {
      gb: Gb::<R>::create(false /* equal length frames */, NoScreen {}),
    }
  }
}

pub struct GbPool<R: Rom> {
  jobs: Sender<Box<dyn FnOnce(&mut Gb<R>) + Send>>,
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
    let sdl = if has_screen { Some(Sdl::init_sdl(num_threads as u32 /* num screens */, 1 /* scale */)) } else { None };

    let (tx, rx) = channel::<Box<dyn FnOnce(&mut Gb<R>) + Send>>();

    let job_receiver = Arc::new(Mutex::new(rx));

    // Threadpool threads
    for i in 0..num_threads {
      let job_receiver = Arc::clone(&job_receiver);
      let sdl = sdl.clone();
      thread::spawn(move || {
        let mut gb = if has_screen {
          Gb::<R>::create(false /* equal length frames */, SdlScreen::new(sdl.unwrap(), i as u32 /* screen */))
        } else {
          Gb::<R>::create(false /* equal length frames */, NoScreen {})
        };

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

pub struct GbResults<T> {
    rx: Receiver<T>,
}
impl IntoIterator for GbResults<((), State)> {
  type Item = State;
  existential type IntoIter: Iterator<Item=State>; // = ::std::iter::Map<IntoIter<((), State)>, fn(((), State)) -> State>;

  fn into_iter(self) -> Self::IntoIter {
    self.rx.into_iter().map(|(_, v)| v)
  }
}
impl<K: StateKey> GbResults<(K, State)> {
  #[allow(dead_code)]
  pub fn into_state_buffer_map(self, buffer_size: usize) -> HashMap<K, StateBuffer> {
    let mut result: HashMap<K, StateBuffer> = HashMap::new();
    for (value, s) in self.into_map_iter() {
      result.entry(value).or_insert_with(|| StateBuffer::with_max_size(buffer_size)).add_state(s);
    }
    result
  }
  pub fn into_map_iter(self) -> IntoIter<(K, State)> {
      self.rx.into_iter()
  }
}
impl GbResults<((), State)> {
  #[allow(dead_code)]
  pub fn into_state_buffer(self, buffer_size: usize) -> StateBuffer {
    StateBuffer::from_iter_sized(self, buffer_size)
  }
}

type GbFn<'a, R, K> = dyn Fn(&mut Gb<R>, State, Sender<(K, State)>) + Send + Sync + 'a;
impl<R: Rom> GbExecutor<R> for GbPool<R> {
  fn execute<K: StateKey, I: IntoIterator<Item=State>, F: Fn(&mut Gb<R>, State, Sender<(K, State)>) + Send + Sync>(&mut self, states: I, f: F) -> GbResults<(K, State)> {
    // Wrap functon in an Arc.
    let f: Arc<GbFn<'_, R, K>> = Arc::new(f);
    // Erase lifetime constraints: The resulting iterator must be fully consumed before the life time of F ends (ideally within the same statement) for this to be safe.
    let f: Arc<GbFn<'static, R, K>> = unsafe { ::std::mem::transmute(f) };

    let (tx, rx) = channel::<(K, State)>();

    for s in states.into_iter() {
      let tx = tx.clone();
      let f = Arc::clone(&f);
      let job = Box::new(move |gb: &mut Gb<R>| { f(gb, s, tx) });
      self.jobs.send(job).unwrap();
    }

    GbResults { rx }
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
