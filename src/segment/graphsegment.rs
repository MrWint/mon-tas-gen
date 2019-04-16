use crate::gbexecutor::*;
use crate::rom::*;
use crate::segment::*;
use log::*;
use std::collections::BTreeMap;
use std::marker::PhantomData;

pub trait GraphState: StateKey + Ord {
  fn is_terminal_state(&self) -> bool;
}

pub trait Graph<R: Rom> {
  type State: GraphState;

  fn initial_state(&self) -> Self::State;

  fn get_segments(&self, state: Self::State) -> Vec<Box<dyn Segment<R, Key=Self::State>>>;
}

#[allow(dead_code)]
pub struct GraphSegment<R: Rom, G: Graph<R>> {
  graph: G,
  buffer_size: usize,
  _rom: PhantomData<R>,
}
impl<R: Rom, G: Graph<R>> WithOutputBufferSize for GraphSegment<R, G> {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}
impl<R: Rom, G: Graph<R>> GraphSegment<R, G> {
  #[allow(dead_code)]
  pub fn new(graph: G) -> Self {
    Self {
      graph,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
      _rom: PhantomData,
    }
  }
}

impl<R: Rom, G: Graph<R>> Segment<R> for GraphSegment<R, G> {
  type Key = G::State;

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let mut result: HashMap<G::State, StateBuffer> = HashMap::new();

    let mut active_states: BTreeMap<G::State, StateBuffer> = BTreeMap::new();
    active_states.insert(self.graph.initial_state(), sb);
    while !active_states.is_empty() {
      let (min_state, sb) = {
        let mut iter = active_states.into_iter();
        let min = iter.next().unwrap();
        active_states = iter.collect();
        min
      };
      debug!("GraphSegment at state {:?}", min_state);
      for segment in self.graph.get_segments(min_state) {
        for (new_state, new_sb) in segment.execute_split(gbe, sb.clone()) {
          if new_state.is_terminal_state() {
            result.entry(new_state).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_all(new_sb);
          } else {
            active_states.entry(new_state).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_all(new_sb);
          }
        }
      }
    }
    result
  }
}
