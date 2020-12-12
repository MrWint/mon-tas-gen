use crate::constants::*;
use crate::gb::*;
use crate::rom::*;
use crate::segment::metric::*;
use gambatte::Input;

mod map;
pub use self::map::Map;
mod overworldmovesegment;
pub use self::overworldmovesegment::OverworldMoveSegment;
mod walkstepsegment;
pub use self::walkstepsegment::WalkStepSegment;
mod walktosegment;
pub use self::walktosegment::WalkToSegment;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OverworldInteractionResult {
  NoOverworldInput,
  WarpTo { map: u8, entrance: u8 },
  FlyWarpOrDungeonWarp,
  DisplayText { id: u8 },
  WildEncounter { species: Pokemon, level: u8, dvs: DVs },
  TrainerBattle { species: u8 },
  Turned { direction: Input },
  Collision,
  BlackOut,
  Walked { direction: Input },
  NoAction,
  Unknown,
}

pub fn dir_to_input(dir: u8) -> Input {
  match dir {
    8 => Input::UP,
    4 => Input::DOWN,
    2 => Input::LEFT,
    1 => Input::RIGHT,
    _ => panic!("got invalid direction {}", dir),
  }
}
pub fn input_to_dir(input: Input) -> u8 {
  match input {
    Input::UP => 8,
    Input::DOWN => 4,
    Input::LEFT => 2,
    Input::RIGHT => 1,
    _ => panic!("got invalid direction {:?}", input),
  }
}

pub struct OverworldInteractionMetric {}
impl<R: JoypadAddresses + Gen1OverworldAddresses + Gen1DVAddresses> Metric<R> for OverworldInteractionMetric {
  type ValueType = OverworldInteractionResult;

  fn evaluate(&self, gb: &mut Gb<R>) -> Option<Self::ValueType> {
    Some(get_overworld_interaction_result(gb))
  }
}

pub fn get_overworld_interaction_result<R: JoypadAddresses + Gen1OverworldAddresses + Gen1DVAddresses>(gb: &mut Gb<R>) -> OverworldInteractionResult {
  if !super::super::is_correct_input_use(gb, R::OVERWORLD_BEFORE_JOYPAD_ADDRESS, R::OVERWORLD_JOYPAD_ADDRESS, R::OVERWORLD_AFTER_JOYPAD_ADDRESS) {
    return OverworldInteractionResult::NoOverworldInput;
  }
  let hit = gb.run_until_or_next_input_use(&[
      R::OVERWORLD_WARP_FOUND_ADDRESS,
      R::OVERWORLD_FLY_DUNGEON_WARP_FOUND_ADDRESS,
      R::OVERWORLD_DISPLAY_TEXT_ADDRESS,
      R::OVERWORLD_INIT_BATTLE_COMMON_ADDRESS,
      R::OVERWORLD_TURNING_DONE_ADDRESS,
      R::OVERWORLD_LAND_COLLISION_ADDRESS,
      R::OVERWORLD_WATER_COLLISION_ADDRESS,
      R::OVERWORLD_HANDLE_BLACKOUT_ADDRESS,
      R::OVERWORLD_WALKED_ADDRESS,
      R::OVERWORLD_NO_ACTION_ADDRESS]);
  if hit == R::OVERWORLD_WARP_FOUND_ADDRESS {
    OverworldInteractionResult::WarpTo { map: gb.gb.read_memory(R::OVERWORLD_WARP_MAP_MEM_ADDRESS), entrance: gb.gb.read_memory(R::OVERWORLD_WARP_ENTRANCE_MEM_ADDRESS) }
  } else if hit == R::OVERWORLD_FLY_DUNGEON_WARP_FOUND_ADDRESS {
    OverworldInteractionResult::FlyWarpOrDungeonWarp
  } else if hit == R::OVERWORLD_DISPLAY_TEXT_ADDRESS {
    OverworldInteractionResult::DisplayText { id: gb.gb.read_memory(R::OVERWORLD_DISPLAY_TEXT_ID_MEM_ADDRESS) }
  } else if hit == R::OVERWORLD_INIT_BATTLE_COMMON_ADDRESS {
    let species = gb.gb.read_memory(R::OVERWORLD_BATTLE_SPECIES_MEM_ADDRESS);
    if species < 200 {
      let dvs = Gen1DVMetric{}.evaluate(gb).unwrap();
      OverworldInteractionResult::WildEncounter { species: Pokemon::from_gen1_index(species).unwrap(), level: gb.gb.read_memory(R::OVERWORLD_BATTLE_LEVEL_MEM_ADDRESS), dvs }
    } else {
      OverworldInteractionResult::TrainerBattle { species }
    }
  } else if hit == R::OVERWORLD_TURNING_DONE_ADDRESS {
    OverworldInteractionResult::Turned { direction: dir_to_input(gb.gb.read_memory(R::OVERWORLD_MOVING_DIRECTION_MEM_ADDRESS)) }
  } else if hit == R::OVERWORLD_LAND_COLLISION_ADDRESS {
    // still need to check for warps
    let hit = gb.step_until(&[R::OVERWORLD_WARP_FOUND_ADDRESS, R::OVERWORLD_LAND_COLLISION_NO_WARP_ADDRESS]);
    if hit == R::OVERWORLD_WARP_FOUND_ADDRESS {
      OverworldInteractionResult::WarpTo { map: gb.gb.read_memory(R::OVERWORLD_WARP_MAP_MEM_ADDRESS), entrance: gb.gb.read_memory(R::OVERWORLD_WARP_ENTRANCE_MEM_ADDRESS) }
    } else {
      OverworldInteractionResult::Collision
    }
  } else if hit == R::OVERWORLD_WATER_COLLISION_ADDRESS {
    OverworldInteractionResult::Collision
  } else if hit == R::OVERWORLD_HANDLE_BLACKOUT_ADDRESS {
    OverworldInteractionResult::BlackOut
  } else if hit == R::OVERWORLD_WALKED_ADDRESS {
    OverworldInteractionResult::Walked { direction: dir_to_input(gb.gb.read_memory(R::OVERWORLD_MOVING_DIRECTION_MEM_ADDRESS)) }
  } else if hit == R::OVERWORLD_NO_ACTION_ADDRESS {
    OverworldInteractionResult::NoAction
  } else {
    OverworldInteractionResult::Unknown
  }
}
