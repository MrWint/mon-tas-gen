use serde_derive::{Serialize, Deserialize};

use crate::metric::Metric;
use crate::metric::overworld::gen1::*;
use crate::multi::*;
use crate::rom::*;
use gambatte::inputs::*;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalkToEncounterPlanState {
  pos: (usize, usize),
  turnframe_direction: Option<u8>,
  map: Rc<MapState>,
  joypad_overworld_state: JoypadOverworldState,
  dist_to_goal: i32,
  requires_turn: bool,
}
impl PartialOrd for WalkToEncounterPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.dist_to_goal != other.dist_to_goal {
      other.dist_to_goal.partial_cmp(&self.dist_to_goal)
    } else {
      other.requires_turn.partial_cmp(&self.requires_turn)
    }
  }
}
impl PartialEq for WalkToEncounterPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MapState {
  width: usize,
  height: usize,
  distances: Vec<i32>,
}
impl MapState {
  fn from_gb<R: MultiRom + JoypadOverworldAddresses + Gen1OverworldAddresses + Gen1DVAddresses + Gen1MapAddresses>(gb: &mut Gb<R>, dest_x: usize, dest_y: usize) -> Self {
    let map = Map::default().load_gen1_map::<R>(&gb.gb);
    let width = map.width;
    let height = map.height;
    // calculate distances
    let distances = {
      let mut distances = vec![];
      distances.resize(width * height, -1);
      distances[width * dest_y + dest_x] = 0;
      let mut queue = VecDeque::new();
      queue.push_back((dest_x, dest_y));
      while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        for &(dx, dy, from_input) in &[(0, -1, Input::DOWN), (0, 1, Input::UP), (-1, 0, Input::RIGHT), (1, 0, Input::LEFT)] {
          if y as isize + dy < 0 || y as isize + dy >= height as isize { continue; } // out of bounds
          if x as isize + dx < 0 || x as isize + dx >= width as isize { continue; } // out of bounds
          let from_x = (x as isize + dx) as usize;
          let from_y = (y as isize + dy) as usize;
          if distances[width * from_y + from_x] != -1 { continue; } // already calculated
          if !map.tile_allowed_movements[width * from_y + from_x].contains(from_input) { continue; } // step not allowed
          distances[width * from_y + from_x] = distances[width * y + x] + 1;
          queue.push_back((from_x, from_y));
        }
      }
      distances
    };
    MapState {
      width,
      height,
      distances,
    }
  }
}

// Plan to progress JoypadOverworld inputs
pub struct WalkToEncounterPlan<M> {
  // instance state
  joypad_overworld_state: JoypadOverworldState,
  pos_x: usize,
  pos_y: usize,
  turnframe_direction: Option<u8>,
  map: Rc<MapState>,

  // config state
  dest_x: usize,
  dest_y: usize,
  move_past_pikachu: bool,
  metric: M,
}
impl<M>WalkToEncounterPlan<M> {
  pub fn new(dest_x: isize, dest_y: isize, metric: M) -> Self {
    Self {
      // Set instance state to dummy values, will be initialize()'d later.
      joypad_overworld_state: JoypadOverworldState::unknown(),
      pos_x: 0,
      pos_y: 0,
      turnframe_direction: None,
      map: Rc::new(MapState::default()),

      // Default config state.
      dest_x: (dest_x + 6) as usize,
      dest_y: (dest_y + 6) as usize,
      move_past_pikachu: false,
      metric,
    }
  }
  pub fn with_move_past_pikachu(self) -> Self { Self { move_past_pikachu: true, ..self } }
  fn causes_turn(&self, input: Input) -> bool {
    if let Some(turnframe_dir) = self.turnframe_direction {
      turnframe_dir != input_to_dir(input)
    } else { false }
  }
  fn allowed_walk(&self, dir: Input) -> bool {
    let dist = self.map.distances[self.map.width * self.pos_y + self.pos_x];
    let dir_dist = match dir {
      U => self.map.distances[self.map.width * (self.pos_y - 1) + self.pos_x],
      D => self.map.distances[self.map.width * (self.pos_y + 1) + self.pos_x],
      L => self.map.distances[self.map.width * self.pos_y + self.pos_x - 1],
      R => self.map.distances[self.map.width * self.pos_y + self.pos_x + 1],
      _ => panic!("invalid direction {:?}", dir),
    };
    dir_dist >= 0 && dir_dist < dist
  }
  fn requires_turn_frame(&self) -> bool {
    if let Some(turnframe_dir) = self.turnframe_direction {
      let dist = self.map.distances[self.map.width * self.pos_y + self.pos_x] as usize;
      match turnframe_dir {
        8 => self.pos_x != self.dest_x || self.dest_y + dist != self.pos_y,
        4 => self.pos_x != self.dest_x || self.pos_y + dist != self.dest_y,
        2 => self.pos_y != self.dest_y || self.dest_x + dist != self.pos_x,
        1 => self.pos_y != self.dest_y || self.pos_x + dist != self.dest_x,
        _ => true,
      }
    } else { false }
  }
}
impl<R: MultiRom + JoypadOverworldAddresses + Gen1OverworldAddresses + Gen1DVAddresses + Gen1MapAddresses, M: Metric<R>> Plan<R> for WalkToEncounterPlan<M> {
  type Value = M::ValueType;

  fn save(&self) -> PlanState {
    PlanState::WalkToEncounterState(WalkToEncounterPlanState { pos: (self.pos_x, self.pos_y), turnframe_direction: self.turnframe_direction, map: self.map.clone(), joypad_overworld_state: self.joypad_overworld_state.clone(), dist_to_goal: self.map.distances[self.map.width * self.pos_y + self.pos_x], requires_turn: self.requires_turn_frame() })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::WalkToEncounterState(WalkToEncounterPlanState { pos, turnframe_direction, map, joypad_overworld_state, dist_to_goal: _, requires_turn: _, }) = state {
      self.pos_x = pos.0;
      self.pos_y = pos.1;
      self.turnframe_direction = *turnframe_direction;
      self.map = map.clone();
      self.joypad_overworld_state = joypad_overworld_state.clone();
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    gb.restore(state);
    if self.map.width == 0 { // map not initialized yet
      self.map = Rc::new(MapState::from_gb(gb, self.dest_x, self.dest_y));
    }
    self.pos_x = gb.gb.read_memory(R::PLAYER_X_POS_MEM_ADDRESS) as usize + 6;
    self.pos_y = gb.gb.read_memory(R::PLAYER_Y_POS_MEM_ADDRESS) as usize + 6;
    assert!(self.map.distances[self.map.width * self.pos_y + self.pos_x] >= 0, "Destination unreachable!");
    self.turnframe_direction = if gb.gb.read_memory(R::OVERWORLD_TURNFRAME_CHECK_MEM_ADDRESS) == 0 {
      None
    } else {
      Some(gb.gb.read_memory(R::OVERWORLD_TURNFRAME_DIRECTION_MEM_ADDRESS))
    };
    self.joypad_overworld_state = JoypadOverworldState::from_gb(gb);
  }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() }

  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    let (held, pressed) = self.joypad_overworld_state.get_input(input);
    if pressed.intersects(START) { return None; } // Opening start menu is not allowed
    if pressed.intersects(A) { return Some(A); } // Allow pressing A to delay
    let move_past_pikachu_mask = if self.move_past_pikachu { B } else { NIL };
    for &dir in &[D, U, L ,R] {
      if held.intersects(dir) {
        return if self.causes_turn(dir) || self.allowed_walk(dir) { Some(dir | (held & move_past_pikachu_mask)) } else { None };
      }
    }
    Some(Input::empty())
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, s: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)> {
    gb.restore(s);
    gb.input(input);
    match get_overworld_interaction_result(gb) {
      OverworldInteractionResult::Walked { direction } => {
        if !self.allowed_walk(direction) { return None; }
        match direction {
          U => self.pos_y -= 1,
          D => self.pos_y += 1,
          L => self.pos_x -= 1,
          R => self.pos_x += 1,
          _ => panic!("invalid direction {:?}", direction),
        }
        assert!(self.map.distances[self.map.width * self.pos_y + self.pos_x] >= 0);
        if (self.pos_x, self.pos_y) == (self.dest_x, self.dest_y) {
          return None; // reached goal without any encounter
        }
        gb.step();
        let new_state = gb.save();
        // Does not change turnframe state
        self.joypad_overworld_state = JoypadOverworldState::from_gb(gb);
        Some((new_state, None))
      },
      OverworldInteractionResult::Turned { direction: _ } => {
        if self.requires_turn_frame() {
          gb.step();
        } else {
          gb.delay_step();
        }
        let new_state = gb.save();
        self.turnframe_direction = None; // Turned
        self.joypad_overworld_state = JoypadOverworldState::from_gb(gb);
        Some((new_state, None))
      },
      OverworldInteractionResult::NoAction | OverworldInteractionResult::Collision => {
        gb.delay_step();
        let new_state = gb.save();
        self.turnframe_direction = if gb.gb.read_memory(R::OVERWORLD_TURNFRAME_CHECK_MEM_ADDRESS) == 0 {
          None
        } else {
          Some(gb.gb.read_memory(R::OVERWORLD_TURNFRAME_DIRECTION_MEM_ADDRESS))
        };
        self.joypad_overworld_state = JoypadOverworldState::from_gb(gb);
        Some((new_state, None))
      },
      OverworldInteractionResult::WildEncounter { .. } => {
        gb.restore(s);
        gb.input(input);
        if let Some(metric_value) = self.metric.evaluate(gb) {
          if !gb.is_at_input() { gb.step(); }
          Some((gb.save(), Some(metric_value)))
        } else { None }
      },
      _ => None,
    }
  }
}
