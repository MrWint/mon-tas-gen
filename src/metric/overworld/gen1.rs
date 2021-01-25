use crate::constants::*;
use crate::metric::*;
use gambatte::Input;

mod map;
pub use map::Map;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OverworldInteractionResult {
  NoOverworldInput,
  WarpTo { map: u8, entrance: u8 },
  FlyWarpOrDungeonWarp,
  DisplayText { id: u8 },
  WildEncounter { species: Pokemon, level: u8, dvs: DVs },
  TrainerBattle { species: u8 },
  Turned { direction: Input },
  JumpLedge,
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

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    Some(get_overworld_interaction_result(gb))
  }
}

pub fn get_overworld_interaction_result<R: JoypadAddresses + Gen1OverworldAddresses + Gen1DVAddresses>(gb: &mut dyn GbI<R>) -> OverworldInteractionResult {
  if gb.step_until(&[R::OVERWORLD_AFTER_JOYPAD_ADDRESS]) == 0 {
    return OverworldInteractionResult::NoOverworldInput;
  }
  let mut hit = gb.step_until(&[
      R::OVERWORLD_WARP_FOUND_ADDRESS, // WarpFound1 or WarpFound2
      R::OVERWORLD_FLY_DUNGEON_WARP_FOUND_ADDRESS, // HandleFlyWarpOrDungeonWarp
      R::OVERWORLD_A_BUTTON_Z_CHECK_1, // .displayDialogue - 12
      R::OVERWORLD_DISPLAY_TEXT_ADDRESS,
      R::OVERWORLD_DISPLAY_TEXT_FAILED_ADDRESS, // .noDirectionButtonsPressed - 3
      R::OVERWORLD_INIT_BATTLE_COMMON_ADDRESS, // InitBattleCommon
      R::OVERWORLD_NEW_BATTLE_NO_BATTLE, // .newBattle + 3
      R::OVERWORLD_TURNING_DONE_ADDRESS,
      R::OVERWORLD_JUMP_LEDGE_ADDRESS, // HandleLedges.foundMatch + 4
      R::OVERWORLD_LAND_COLLISION_ADDRESS,
      R::OVERWORLD_WATER_COLLISION_ADDRESS,
      R::OVERWORLD_WALKED_PRE_ADDRESS, // .notSafariZone
      R::OVERWORLD_NO_ACTION_ADDRESS]);
  
  if hit == R::OVERWORLD_A_BUTTON_Z_CHECK_1 {
    if gb.gb().read_registers().z_flag() { return OverworldInteractionResult::NoAction; }
    assert!(gb.step_until(&[R::OVERWORLD_A_BUTTON_Z_CHECK_2]) != 0);
    if gb.gb().read_registers().z_flag() { return OverworldInteractionResult::NoAction; }
    hit = gb.step_until(&[R::OVERWORLD_DISPLAY_TEXT_ADDRESS, R::OVERWORLD_INIT_BATTLE_COMMON_ADDRESS, R::OVERWORLD_NEW_BATTLE_NO_BATTLE, R::OVERWORLD_DISPLAY_TEXT_FAILED_ADDRESS]);
  }

  if hit == R::OVERWORLD_WARP_FOUND_ADDRESS {
    OverworldInteractionResult::WarpTo { map: gb.gb().read_memory(R::OVERWORLD_WARP_MAP_MEM_ADDRESS), entrance: gb.gb().read_memory(R::OVERWORLD_WARP_ENTRANCE_MEM_ADDRESS) }
  } else if hit == R::OVERWORLD_FLY_DUNGEON_WARP_FOUND_ADDRESS {
    OverworldInteractionResult::FlyWarpOrDungeonWarp
  } else if hit == R::OVERWORLD_DISPLAY_TEXT_ADDRESS {
    OverworldInteractionResult::DisplayText { id: gb.gb().read_memory(R::OVERWORLD_DISPLAY_TEXT_ID_MEM_ADDRESS) }
  } else if hit == R::OVERWORLD_INIT_BATTLE_COMMON_ADDRESS {
    get_overworld_interaction_battle_result(gb)
  } else if hit == R::OVERWORLD_TURNING_DONE_ADDRESS {
    OverworldInteractionResult::Turned { direction: dir_to_input(gb.gb().read_memory(R::OVERWORLD_MOVING_DIRECTION_MEM_ADDRESS)) }
  } else if hit == R::OVERWORLD_JUMP_LEDGE_ADDRESS {
    OverworldInteractionResult::JumpLedge
  } else if hit == R::OVERWORLD_LAND_COLLISION_ADDRESS {
    // still need to check for warps
    let hit = gb.step_until(&[R::OVERWORLD_WARP_FOUND_ADDRESS, R::OVERWORLD_LAND_COLLISION_NO_WARP_ADDRESS]);
    if hit == R::OVERWORLD_WARP_FOUND_ADDRESS {
      OverworldInteractionResult::WarpTo { map: gb.gb().read_memory(R::OVERWORLD_WARP_MAP_MEM_ADDRESS), entrance: gb.gb().read_memory(R::OVERWORLD_WARP_ENTRANCE_MEM_ADDRESS) }
    } else {
      OverworldInteractionResult::Collision
    }
  } else if hit == R::OVERWORLD_WATER_COLLISION_ADDRESS {
    OverworldInteractionResult::Collision
  } else if hit == R::OVERWORLD_WALKED_PRE_ADDRESS {
    let hit = gb.step_until(&[R::OVERWORLD_HANDLE_BLACKOUT_ADDRESS, R::OVERWORLD_WALKED_ADDRESS, R::OVERWORLD_INIT_BATTLE_COMMON_ADDRESS]);
    if hit == R::OVERWORLD_HANDLE_BLACKOUT_ADDRESS {
      OverworldInteractionResult::BlackOut
    } else if hit == R::OVERWORLD_WALKED_ADDRESS {
      OverworldInteractionResult::Walked { direction: dir_to_input(gb.gb().read_memory(R::OVERWORLD_MOVING_DIRECTION_MEM_ADDRESS)) }
    } else {
      assert!(hit == R::OVERWORLD_INIT_BATTLE_COMMON_ADDRESS);
      get_overworld_interaction_battle_result(gb)
    }
  } else if hit == R::OVERWORLD_NO_ACTION_ADDRESS || hit == R::OVERWORLD_NEW_BATTLE_NO_BATTLE || hit == R::OVERWORLD_DISPLAY_TEXT_FAILED_ADDRESS {
    OverworldInteractionResult::NoAction
  } else {
    OverworldInteractionResult::Unknown
  }
}

fn get_overworld_interaction_battle_result<R: JoypadAddresses + Gen1OverworldAddresses + Gen1DVAddresses>(gb: &mut dyn GbI<R>) -> OverworldInteractionResult {
  let species = gb.gb().read_memory(R::OVERWORLD_BATTLE_SPECIES_MEM_ADDRESS);
  if species < 200 {
    let dvs = Gen1DVMetric{}.evaluate(gb).unwrap();
    OverworldInteractionResult::WildEncounter { species: Pokemon::from_gen1_index(species).unwrap(), level: gb.gb().read_memory(R::OVERWORLD_BATTLE_LEVEL_MEM_ADDRESS), dvs }
  } else {
    OverworldInteractionResult::TrainerBattle { species }
  }
}
