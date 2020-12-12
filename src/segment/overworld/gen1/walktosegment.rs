use crate::rom::*;
use crate::segment::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;
use log::{debug, log_enabled};
use std::collections::VecDeque;
use std::fmt::Write;
use super::OverworldInteractionResult;

pub struct WalkToSegment {
  dest_x: usize,
  dest_y: usize,
  into_result: OverworldInteractionResult,
  buffer_size: usize,
}
impl WalkToSegment {
  #[allow(dead_code)]
  pub fn new(dest_x: isize, dest_y: isize) -> Self {
    Self {
      dest_x: (dest_x + 6) as usize,
      dest_y: (dest_y + 6) as usize,
      into_result: OverworldInteractionResult::NoAction,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  #[allow(dead_code)]
  pub fn into(self, into_result: OverworldInteractionResult) -> Self { Self { into_result, ..self } }
}
impl WithOutputBufferSize for WalkToSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + Gen1MapAddresses + Gen1OverworldAddresses + Gen1DVAddresses> crate::segment::Segment<R> for WalkToSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let initial_states: Vec<_> = sb.into_iter().collect();
    assert!(!initial_states.is_empty());
    let map = gbe.execute(vec![&initial_states[0]], |gb, s, tx| {
      gb.restore(&s);
      gb.input(Input::empty());
      gb.step();
      tx.send(gb.save_with_value(super::map::Map::default().load_gen1_map(gb))).unwrap();
    }).get_value_assert_all_equal();
    // let map = gbe.execute_state_fn(vec![&initial_states[0]], |gb| {
    //   super::map::Map::default().load_gen1_map(gb)
    // }).into_split_iter().next().unwrap().1;

    debug!("WalkToSegment navigate to ({}, {})", self.dest_x as isize-6, self.dest_y as isize-6);
    if log_enabled!(log::Level::Debug) { debug!("tiles:\n{}", map.tile_string()); }
    if log_enabled!(log::Level::Debug) { debug!("allowed movements:\n{}", map.tile_allowed_movements_string()); }

    // calculate distances
    let distances = {
      let mut distances = vec![];
      distances.resize(map.width * map.height, -1);
      distances[map.width * self.dest_y + self.dest_x] = 0;
      let mut queue = VecDeque::new();
      queue.push_back((self.dest_x, self.dest_y));
      while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        for &(dx, dy, from_input) in &[(0, -1, Input::DOWN), (0, 1, Input::UP), (-1, 0, Input::RIGHT), (1, 0, Input::LEFT)] {
          if y as isize + dy < 0 || y as isize + dy >= map.height as isize { continue; } // out of bounds
          if x as isize + dx < 0 || x as isize + dx >= map.width as isize { continue; } // out of bounds
          let from_x = (x as isize + dx) as usize;
          let from_y = (y as isize + dy) as usize;
          if distances[map.width * from_y + from_x] != -1 { continue; } // already calculated
          if !map.tile_allowed_movements[map.width * from_y + from_x].contains(from_input) { continue; } // step not allowed
          distances[map.width * from_y + from_x] = distances[map.width * y + x] + 1;
          queue.push_back((from_x, from_y));
        }
      }
      distances
    };

    if log_enabled!(log::Level::Debug) {
      let mut buf = String::new();
      for y in 0..map.height {
        for x in 0..map.width {
          write!(&mut buf, " {:2}", distances[map.width * y + x]).unwrap();
        }
        write!(&mut buf, "\n").unwrap();
      }
      debug!("distance map:\n{}", buf);
    }

    // initialize intermediate buffers
    let mut buffers = Vec::<StateBuffer>::new();
    for _ in 0..map.width * map.height { buffers.push(StateBuffer::with_max_size(self.buffer_size)); }
    let mut max_dist = -1;
    for (s, (x, y)) in gbe.execute_state_fn(initial_states, |gb| {
      let x = gb.gb.read_memory(R::PLAYER_X_POS_MEM_ADDRESS) as usize + 6;
      let y = gb.gb.read_memory(R::PLAYER_Y_POS_MEM_ADDRESS) as usize + 6;
      (x, y)
    }).into_split_iter() {
      buffers[map.width * y + x].add_state(s);
      if max_dist < distances[map.width * y + x] { max_dist = distances[map.width * y + x]; }
    }
    assert!(max_dist >= 0, "destination unreachable!");

    let mut cur_dist = max_dist;
    while cur_dist > 0 {
      let into_result = if cur_dist == 1 { &self.into_result } else { &super::OverworldInteractionResult::NoAction };
      for x in 0..map.width {
        for y in 0..map.height {
          if distances[map.width * y + x] == cur_dist {
            let sb = std::mem::replace(&mut buffers[map.width * y + x], StateBuffer::new());
            if sb.is_empty() { continue; }
            debug!("processing states at ({},{}) with dist {}, statebuffer {:?}", x as isize-6, y as isize-6, cur_dist, sb);
            for (s, (nx, ny)) in gbe.execute(sb, |gb, s, tx| {
              for &(dx, dy, input) in &[(0, 1, Input::DOWN), (0, -1, Input::UP), (1, 0, Input::RIGHT), (-1, 0, Input::LEFT)] {
                if !map.tile_allowed_movements[map.width * y + x].contains(input) { continue; } // step not allowed
                if y as isize + dy < 0 || y as isize + dy >= map.height as isize { continue; } // out of bounds
                if x as isize + dx < 0 || x as isize + dx >= map.width as isize { continue; } // out of bounds
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;
                if distances[map.width * ny + nx] != cur_dist - 1 { continue; } // not on any shortest path

                gb.restore(&s);
                gb.input(input);
                if super::walkstepsegment::walk_step_check(gb, &s, input, into_result) {
                  gb.restore(&s);
                  gb.input(input);
                  gb.step();
                  tx.send(gb.save_with_value((nx, ny))).unwrap();
                }
              }
            }).into_split_iter() {
              buffers[map.width * ny + nx].add_state(s);
            }
          }
        }
      }
      cur_dist -= 1;
    }

    let mut result = HashMap::new();
    result.insert((), buffers.into_iter().nth(map.width * self.dest_y + self.dest_x).unwrap());
    result
  }
}
