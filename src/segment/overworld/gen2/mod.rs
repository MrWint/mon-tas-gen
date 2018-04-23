use gambatte::Input;
use gb::*;
use rom::*;


mod map;
pub use self::map::Map;
mod jumpledgesegment;
pub use self::jumpledgesegment::JumpLedgeSegment;
mod turnsegment;
pub use self::turnsegment::TurnSegment;
mod walkstepsegment;
pub use self::walkstepsegment::WalkStepSegment;
mod walktosegment;
pub use self::walktosegment::WalkToSegment;
mod warpsegment;
pub use self::warpsegment::WarpSegment;



#[derive(Debug, Eq, PartialEq)]
pub enum OverworldInteractionResult {
  NoOverworldInput,
  PlayerEventsNotCalled,
  ScriptRunning(PlayerEventScript),
  SeenByTrainer,
  MapConnection,
  Warped, // includes falling, edge warps
  MapCoordEvent,
  CountStepEvent, // includes hatching
  RandomEncounter { species: u8, level: u8 },
  ReentryScript,
  SceneScript,
  EndBugContest,
  PhoneCall,
  ForcedMovement, // including Whirlpool
  Turned,
  Walked(Input), // including biking, ice, surfing
  JumpedLedge,
  Interact, // includes signs, trainers, tiles, hidden items, ...
  StartMenu,
  SelectMenu,
  NoEvents,
  Unknown,
}

#[derive(Debug, Eq, PartialEq)]
pub enum PlayerEventScript {
  MapScript,
  SeenByTrainer,
  TalkToTrainer,
  ItemBall,
  Connection,
  Warp,
  Fall,
  Whiteout,
  Hatch,
  JoyChangeFacing,
}


fn to_player_event(event: u8) -> PlayerEventScript {
  match event {
    255 => PlayerEventScript::MapScript,
    1 => PlayerEventScript::SeenByTrainer,
    2 => PlayerEventScript::TalkToTrainer,
    3 => PlayerEventScript::ItemBall,
    4 => PlayerEventScript::Connection,
    5 => PlayerEventScript::Warp,
    6 => PlayerEventScript::Fall,
    7 => PlayerEventScript::Whiteout,
    8 => PlayerEventScript::Hatch,
    9 => PlayerEventScript::JoyChangeFacing,
    _ => panic!("got invalid player event {}", event),
  }
}
fn dir_to_input(dir: u8) -> Input {
  match dir {
    255 => Input::empty(),
    0 => Input::DOWN,
    1 => Input::UP,
    2 => Input::LEFT,
    3 => Input::RIGHT,
    _ => panic!("got invalid direction {}", dir),
  }
}

pub fn get_overworld_interaction_result<R: JoypadAddresses + Gen2MapEventsAddresses>(gb: &mut Gb<R>) -> OverworldInteractionResult {
  if !super::super::is_correct_input_use(gb, R::OVERWORLD_BEFORE_JOYPAD_ADDRESS, R::OVERWORLD_JOYPAD_ADDRESS, R::OVERWORLD_AFTER_JOYPAD_ADDRESS) {
    return OverworldInteractionResult::NoOverworldInput;
  }
  if gb.step_until(&[R::PLAYER_EVENTS_ADDRESS]) == 0 { return OverworldInteractionResult::PlayerEventsNotCalled; }
  let script_running = gb.gb.read_memory(R::PLAYER_SCRIPT_RUNNING_MEM_ADDRESS);
  if script_running != 0 { return OverworldInteractionResult::ScriptRunning(to_player_event(script_running)); }

  let hit = gb.step_until(&[ // maybe use run_until_or_next_input_use
      R::PLAYER_EVENTS_SEEN_BY_TRAINER_ADDRESS,
      R::PLAYER_EVENTS_MAP_CONNECTION_ADDRESS,
      R::PLAYER_EVENTS_WARP_ADDRESS,
      R::PLAYER_EVENTS_FALL_ADDRESS,
      R::PLAYER_EVENTS_MAP_COORD_EVENT_ADDRESS,
      R::PLAYER_EVENTS_COUNT_STEP_EVENT_ADDRESS,
      R::PLAYER_EVENTS_HATCH_ADDRESS,
      R::PLAYER_EVENTS_RANDOM_ENCOUNTER_ADDRESS,
      R::PLAYER_EVENTS_REENTRY_SCRIPT_ADDRESS,
      R::PLAYER_EVENTS_SCENE_SCRIPT_ADDRESS,
      R::PLAYER_EVENTS_END_BUG_CONTEST_ADDRESS,
      R::PLAYER_EVENTS_PHONE_CALL_ADDRESS,
      R::PLAYER_EVENTS_WHIRLPOOL_FORCED_MOVEMENT_ADDRESS,
      R::PLAYER_EVENTS_FORCED_MOVEMENT_ADDRESS,
      R::PLAYER_EVENTS_TURNING_ADDRESS,
      R::PLAYER_EVENTS_STEP_WALK_ADDRESS,
      R::PLAYER_EVENTS_STEP_BIKE_ADDRESS,
      R::PLAYER_EVENTS_STEP_BIKE_UPHILL_ADDRESS,
      R::PLAYER_EVENTS_STEP_ICE_ADDRESS,
      R::PLAYER_EVENTS_STEP_SURF_ADDRESS,
      R::PLAYER_EVENTS_STEP_OUT_OF_WATER_ADDRESS,
      R::PLAYER_EVENTS_JUMP_LEDGE_ADDRESS,
      R::PLAYER_EVENTS_EDGE_WARP_ADDRESS,
      R::PLAYER_EVENTS_INTERACT_OBJECT_SCRIPT_ADDRESS,
      R::PLAYER_EVENTS_INTERACT_OBJECT_ITEMBALL_ADDRESS,
      R::PLAYER_EVENTS_INTERACT_OBJECT_TRAINER_ADDRESS,
      R::PLAYER_EVENTS_INTERACT_BG_READ_ADDRESS,
      R::PLAYER_EVENTS_INTERACT_BG_HIDDEN_ITEM_ADDRESS,
      R::PLAYER_EVENTS_INTERACT_BG_THENREAD_ADDRESS,
      R::PLAYER_EVENTS_INTERACT_TILE_COLLISION_ADDRESS,
      R::PLAYER_EVENTS_START_MENU_ADDRESS,
      R::PLAYER_EVENTS_SELECT_MENU_ADDRESS,
      R::PLAYER_EVENTS_NO_EVENTS_ADDRESS,
  ]);
  let result = if hit == R::PLAYER_EVENTS_SEEN_BY_TRAINER_ADDRESS {
    OverworldInteractionResult::SeenByTrainer
  } else if hit == R::PLAYER_EVENTS_MAP_CONNECTION_ADDRESS {
    OverworldInteractionResult::MapConnection
  } else if hit == R::PLAYER_EVENTS_WARP_ADDRESS {
    OverworldInteractionResult::Warped
  } else if hit == R::PLAYER_EVENTS_FALL_ADDRESS {
    OverworldInteractionResult::Warped
  } else if hit == R::PLAYER_EVENTS_MAP_COORD_EVENT_ADDRESS {
    OverworldInteractionResult::MapCoordEvent
  } else if hit == R::PLAYER_EVENTS_COUNT_STEP_EVENT_ADDRESS {
    OverworldInteractionResult::CountStepEvent
  } else if hit == R::PLAYER_EVENTS_HATCH_ADDRESS {
    OverworldInteractionResult::CountStepEvent
  } else if hit == R::PLAYER_EVENTS_RANDOM_ENCOUNTER_ADDRESS {
    let species = gb.gb.read_memory(R::PLAYER_EVENTS_RANDOM_ENCOUNTER_SPECIES_MEM_ADDRESS);
    let level = gb.gb.read_memory(R::PLAYER_EVENTS_RANDOM_ENCOUNTER_LEVEL_MEM_ADDRESS);
    OverworldInteractionResult::RandomEncounter { species: species, level: level }
  } else if hit == R::PLAYER_EVENTS_REENTRY_SCRIPT_ADDRESS {
    OverworldInteractionResult::ReentryScript
  } else if hit == R::PLAYER_EVENTS_SCENE_SCRIPT_ADDRESS {
    OverworldInteractionResult::SceneScript
  } else if hit == R::PLAYER_EVENTS_END_BUG_CONTEST_ADDRESS {
    OverworldInteractionResult::EndBugContest
  } else if hit == R::PLAYER_EVENTS_PHONE_CALL_ADDRESS {
    OverworldInteractionResult::PhoneCall
  } else if hit == R::PLAYER_EVENTS_WHIRLPOOL_FORCED_MOVEMENT_ADDRESS {
    OverworldInteractionResult::ForcedMovement
  } else if hit == R::PLAYER_EVENTS_FORCED_MOVEMENT_ADDRESS {
    OverworldInteractionResult::ForcedMovement
  } else if hit == R::PLAYER_EVENTS_TURNING_ADDRESS {
    OverworldInteractionResult::Turned
  } else if hit == R::PLAYER_EVENTS_STEP_WALK_ADDRESS {
    let dir = gb.gb.read_memory(R::PLAYER_EVENTS_WALKING_DIRECTION_MEM_ADDRESS);
    OverworldInteractionResult::Walked(dir_to_input(dir))
  } else if hit == R::PLAYER_EVENTS_STEP_BIKE_ADDRESS {
    let dir = gb.gb.read_memory(R::PLAYER_EVENTS_WALKING_DIRECTION_MEM_ADDRESS);
    OverworldInteractionResult::Walked(dir_to_input(dir))
  } else if hit == R::PLAYER_EVENTS_STEP_BIKE_UPHILL_ADDRESS {
    let dir = gb.gb.read_memory(R::PLAYER_EVENTS_WALKING_DIRECTION_MEM_ADDRESS);
    OverworldInteractionResult::Walked(dir_to_input(dir))
  } else if hit == R::PLAYER_EVENTS_STEP_ICE_ADDRESS {
    let dir = gb.gb.read_memory(R::PLAYER_EVENTS_WALKING_DIRECTION_MEM_ADDRESS);
    OverworldInteractionResult::Walked(dir_to_input(dir))
  } else if hit == R::PLAYER_EVENTS_STEP_SURF_ADDRESS {
    let dir = gb.gb.read_memory(R::PLAYER_EVENTS_WALKING_DIRECTION_MEM_ADDRESS);
    OverworldInteractionResult::Walked(dir_to_input(dir))
  } else if hit == R::PLAYER_EVENTS_STEP_OUT_OF_WATER_ADDRESS {
    let dir = gb.gb.read_memory(R::PLAYER_EVENTS_WALKING_DIRECTION_MEM_ADDRESS);
    OverworldInteractionResult::Walked(dir_to_input(dir))
  } else if hit == R::PLAYER_EVENTS_JUMP_LEDGE_ADDRESS {
    OverworldInteractionResult::JumpedLedge
  } else if hit == R::PLAYER_EVENTS_EDGE_WARP_ADDRESS {
    OverworldInteractionResult::Warped
  } else if hit == R::PLAYER_EVENTS_INTERACT_OBJECT_SCRIPT_ADDRESS {
    OverworldInteractionResult::Interact
  } else if hit == R::PLAYER_EVENTS_INTERACT_OBJECT_ITEMBALL_ADDRESS {
    OverworldInteractionResult::Interact
  } else if hit == R::PLAYER_EVENTS_INTERACT_OBJECT_TRAINER_ADDRESS {
    OverworldInteractionResult::Interact
  } else if hit == R::PLAYER_EVENTS_INTERACT_BG_READ_ADDRESS {
    OverworldInteractionResult::Interact
  } else if hit == R::PLAYER_EVENTS_INTERACT_BG_HIDDEN_ITEM_ADDRESS {
    OverworldInteractionResult::Interact
  } else if hit == R::PLAYER_EVENTS_INTERACT_BG_THENREAD_ADDRESS {
    OverworldInteractionResult::Interact
  } else if hit == R::PLAYER_EVENTS_INTERACT_TILE_COLLISION_ADDRESS {
    OverworldInteractionResult::Interact
  } else if hit == R::PLAYER_EVENTS_START_MENU_ADDRESS {
    OverworldInteractionResult::StartMenu
  } else if hit == R::PLAYER_EVENTS_SELECT_MENU_ADDRESS {
    OverworldInteractionResult::SelectMenu
  } else if hit == R::PLAYER_EVENTS_NO_EVENTS_ADDRESS {
    OverworldInteractionResult::NoEvents
  } else {
    OverworldInteractionResult::Unknown
  };
  if let OverworldInteractionResult::Walked(dir) = result {
    let hit = gb.step_until(&[ // maybe use run_until_or_next_input_use
        R::PLAYER_EVENTS_INTERACT_OBJECT_SCRIPT_ADDRESS,
        R::PLAYER_EVENTS_INTERACT_OBJECT_ITEMBALL_ADDRESS,
        R::PLAYER_EVENTS_INTERACT_OBJECT_TRAINER_ADDRESS,
        R::PLAYER_EVENTS_INTERACT_BG_READ_ADDRESS,
        R::PLAYER_EVENTS_INTERACT_BG_HIDDEN_ITEM_ADDRESS,
        R::PLAYER_EVENTS_INTERACT_BG_THENREAD_ADDRESS,
        R::PLAYER_EVENTS_INTERACT_TILE_COLLISION_ADDRESS,
        R::PLAYER_EVENTS_START_MENU_ADDRESS,
        R::PLAYER_EVENTS_SELECT_MENU_ADDRESS,
        R::PLAYER_EVENTS_NO_EVENTS_ADDRESS,
    ]);
    if hit == R::PLAYER_EVENTS_INTERACT_OBJECT_SCRIPT_ADDRESS {
      OverworldInteractionResult::Interact
    } else if hit == R::PLAYER_EVENTS_INTERACT_OBJECT_ITEMBALL_ADDRESS {
      OverworldInteractionResult::Interact
    } else if hit == R::PLAYER_EVENTS_INTERACT_OBJECT_TRAINER_ADDRESS {
      OverworldInteractionResult::Interact
    } else if hit == R::PLAYER_EVENTS_INTERACT_BG_READ_ADDRESS {
      OverworldInteractionResult::Interact
    } else if hit == R::PLAYER_EVENTS_INTERACT_BG_HIDDEN_ITEM_ADDRESS {
      OverworldInteractionResult::Interact
    } else if hit == R::PLAYER_EVENTS_INTERACT_BG_THENREAD_ADDRESS {
      OverworldInteractionResult::Interact
    } else if hit == R::PLAYER_EVENTS_INTERACT_TILE_COLLISION_ADDRESS {
      OverworldInteractionResult::Interact
    } else if hit == R::PLAYER_EVENTS_START_MENU_ADDRESS {
      OverworldInteractionResult::StartMenu
    } else if hit == R::PLAYER_EVENTS_SELECT_MENU_ADDRESS {
      OverworldInteractionResult::SelectMenu
    } else if hit == R::PLAYER_EVENTS_NO_EVENTS_ADDRESS {
      if dir.is_empty() { OverworldInteractionResult::NoEvents } else { OverworldInteractionResult::Walked(dir) }
    } else {
      OverworldInteractionResult::Unknown
    }
  } else { result }
}
