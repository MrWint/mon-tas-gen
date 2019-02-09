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

pub trait StateValue: Send + 'static {}
impl<T: Send + 'static> StateValue for T {}
pub trait StateKey: Eq + Hash + Debug + StateValue {}
impl<T: Eq + Hash + Debug + StateValue> StateKey for T {}

pub trait GbExecutor<R: Rom> {
  fn execute<IV: StateValue, S: StateRef<IV>, OV: StateValue, I: IntoIterator<Item=S>, F: Fn(&mut Gb<R>, State<IV>, Sender<State<OV>>) + Send + Sync>(&mut self, states: I, f: F) -> GbResults<State<OV>>;
  fn get_initial_state(&mut self) -> State;
  fn execute_state_fn<IV: StateValue, S: StateRef<IV>, OV: StateValue, I: IntoIterator<Item=S>, F: Fn(&mut Gb<R>) -> OV + Send + Sync>(&mut self, states: I, f: F) -> std::iter::Map<std::sync::mpsc::IntoIter<State<OV>>, fn(State<OV>) -> (State, OV)> {
    self.execute(states, |gb, s, tx| {
      gb.restore(&s);
      tx.send(s.replace_value(f(gb))).unwrap();
    }).into_split_iter()
  }
}

pub struct SingleGb<R> {
  gb: Gb<R>,
}
impl<R: Rom> GbExecutor<R> for SingleGb<R> {
  fn execute<IV: StateValue, S: StateRef<IV>, OV: StateValue, I: IntoIterator<Item=S>, F: Fn(&mut Gb<R>, State<IV>, Sender<State<OV>>) + Send + Sync>(&mut self, sb: I, f: F) -> GbResults<State<OV>> {
    let (tx, rx) = channel::<State<OV>>();
    for s in sb {
      f(&mut self.gb, s.to_state(), tx.clone());
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

trait FnBox<R>: Send {
  fn call_box(self: Box<Self>, gb: &mut Gb<R>);
}
impl<R, F: FnOnce(&mut Gb<R>) + Send> FnBox<R> for F {
  fn call_box(self: Box<F>, gb: &mut Gb<R>) {
    (*self)(gb)
  }
}

pub struct GbPool<R: Rom> {
  jobs: Sender<Box<FnBox<R>>>,
}

const STACK_SIZE: usize = 2 * 1024 * 1024;

impl<R: Rom> GbPool<R> {
  pub fn with_screen() -> GbPool<R> {
    Self::new(true)
  }
  pub fn no_screen() -> GbPool<R> {
    Self::new(false)
  }

  fn new(has_screen: bool) -> GbPool<R> {
    let num_threads = num_cpus::get();
    let sdl = if has_screen { Some(Sdl::init_sdl(num_threads as u32 /* num screens */, 1 /* scale */)) } else { None };

    let (tx, rx) = channel::<Box<FnBox<R>>>();

    let job_receiver = Arc::new(Mutex::new(rx));

    // Threadpool threads
    for i in 0..num_threads {
      let job_receiver = Arc::clone(&job_receiver);
      let sdl = sdl.clone();
      thread::Builder::new()
          .name(format!("GbPool worker {}", i))
          .stack_size(STACK_SIZE)
          .spawn(move || {
            let mut gb = if has_screen {
              Gb::<R>::create(false /* equal length frames */, SdlScreen::new(sdl.unwrap(), i as u32 /* screen */))
            } else {
              Gb::<R>::create(false /* equal length frames */, NoScreen {})
            };

            loop {
              let message = job_receiver.lock().expect("Worker thread unable to lock job_receiver").recv();
              match message {
                Ok(job) => job.call_box(&mut gb),
                Err(..) => break, // The Pool was dropped.
              };
            }
          })
          .unwrap();
    }

    GbPool {
      jobs: tx,
    }
  }
}

pub struct GbResults<T> {
    rx: Receiver<T>,
}
impl<K> IntoIterator for GbResults<State<K>> {
  type Item = State<K>;
  type IntoIter = IntoIter<State<K>>;

  fn into_iter(self) -> Self::IntoIter {
    self.rx.into_iter()
  }
}
impl<K> GbResults<State<K>> {
  pub fn into_split_iter(self) -> std::iter::Map<std::sync::mpsc::IntoIter<State<K>>, fn(State<K>) -> (State, K)> {
    self.rx.into_iter().map(State::split_state_and_value)
  }
  pub fn into_state_buffer(self, buffer_size: usize) -> StateBuffer<K> {
    StateBuffer::from_iter_sized(self, buffer_size)
  }
}
impl<K: Eq + Hash> GbResults<State<K>> {
  pub fn into_state_buffer_map(self, buffer_size: usize) -> HashMap<K, StateBuffer> {
    let mut result: HashMap<K, StateBuffer> = HashMap::new();
    for (s, value) in self.into_split_iter() {
      result.entry(value).or_insert_with(|| StateBuffer::with_max_size(buffer_size)).add_state(s);
    }
    result
  }
}

type GbFn<'a, R, IV, OV> = dyn Fn(&mut Gb<R>, State<IV>, Sender<State<OV>>) + Send + Sync + 'a;
impl<R: Rom> GbExecutor<R> for GbPool<R> {
  fn execute<IV: StateValue, S: StateRef<IV>, OV: StateValue, I: IntoIterator<Item=S>, F: Fn(&mut Gb<R>, State<IV>, Sender<State<OV>>) + Send + Sync>(&mut self, states: I, f: F) -> GbResults<State<OV>> {
    // Wrap functon in an Arc.
    let f: Arc<GbFn<'_, R, IV, OV>> = Arc::new(f);
    // Erase lifetime constraints: The resulting iterator must be fully consumed before the life time of F ends (ideally within the same statement) for this to be safe.
    let f: Arc<GbFn<'static, R, IV, OV>> = unsafe { std::mem::transmute(f) };

    let (tx, rx) = channel::<State<OV>>();

    for s in states.into_iter() {
      let tx = tx.clone();
      let f = Arc::clone(&f);
      let s = s.to_state();
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
