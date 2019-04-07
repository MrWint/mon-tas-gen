use crate::gb::*;
use crate::rom::*;
use gambatte::Input;
use std::fmt::Write;

#[derive(Debug, Default, Hash, Eq, PartialEq)]
pub struct Map {
  allow_water_tiles: bool,
  pub width: usize,
  pub height: usize,
  tile_collision: Vec<u8>,
  pub tile_allowed_movements: Vec<Input>,
}

impl Map {
  #[allow(dead_code)]
  pub fn with_water_tiles(self) -> Self { Self { allow_water_tiles: true, ..self } }

  #[allow(clippy::cyclomatic_complexity)]
  pub fn load_gen2_map<T: JoypadAddresses + RngAddresses + Gen2MapAddresses>(mut self, gb: &Gb<T>) -> Self {
    let map_block_width = gb.gb.read_memory(T::MAP_WIDTH_MEM_ADDRESS) as usize + 6;
    let map_block_height = gb.gb.read_memory(T::MAP_HEIGHT_MEM_ADDRESS) as usize + 6;
    self.width = map_block_width * 2;
    self.height = map_block_height * 2;

    // get tile collisions
    {
      let mut blocks = vec![];
      // get blocks
      for offset in 0..(map_block_height as u16 * map_block_width as u16) {
        blocks.push(gb.gb.read_memory(T::OVERWORLD_MAP_MEM_ADDRESS + offset));
      }

      let tileset_collision_data_base_address = (i32::from(gb.gb.read_memory(T::TILESET_COLLISION_BANK_MEM_ADDRESS)) << 16) + i32::from(gb.gb.read_memory_word_le(T::TILESET_COLLISION_PTR_MEM_ADDRESS));
      for y in 0..self.height {
        for x in 0..self.width {
          let block = blocks[map_block_width * (y >> 1) + (x >> 1)];
          let tile = if block == 0 { 0xff } else {
            gb.gb.read_rom(tileset_collision_data_base_address + i32::from(block) * 4 + (y as i32 & 1)*2 + (x as i32 & 1))
          };
          self.tile_collision.push(tile);
        }
      }
    }

    // get allowed movements
    for y in 0..self.height {
      for x in 0..self.width {
        let mut allowed_inputs = Input::empty();
        for &(input, dx, dy, forbidden_cur_tiles, forbidden_new_tiles) in &[
            (Input::UP, 0, -1, &[0xb1,0xb5,0xb7,0xc1,0xc5,0xc7], &[0xb3,0xb4,0xb5,0xc3,0xc4,0xc5]),
            (Input::DOWN, 0, 1, &[0xb0,0xb4,0xb6,0xc0,0xc4,0xc6], &[0xb2,0xb6,0xb7,0xc2,0xc6,0xc7]),
            (Input::LEFT, -1, 0, &[0xb2,0xb6,0xb7,0xc2,0xc6,0xc7], &[0xb0,0xb4,0xb6,0xc0,0xc4,0xc6]),
            (Input::RIGHT, 1, 0, &[0xb3,0xb4,0xb5,0xc3,0xc4,0xc5], &[0xb1,0xb5,0xb7,0xc1,0xc5,0xc7])] {
          if y as isize + dy < 0 || y as isize + dy >= self.height as isize { continue; } // out of bounds
          if x as isize + dx < 0 || x as isize + dx >= self.width as isize { continue; } // out of bounds
          let nx = (x as isize + dx) as usize;
          let ny = (y as isize + dy) as usize;
          if forbidden_cur_tiles.contains(&self.tile_collision[self.width * y + x]) { continue; } // direction forbidden
          if forbidden_new_tiles.contains(&self.tile_collision[self.width * ny + nx]) { continue; } // direction forbidden
          let new_tile_collision_type = gb.gb.read_rom(T::TILE_COLLISION_TABLE_ADDRESS + i32::from(self.tile_collision[self.width * ny + nx])) & 0xf;
          if new_tile_collision_type != 0 && (new_tile_collision_type != 1 || !self.allow_water_tiles) { continue; } // target tile not passable
          allowed_inputs |= input;
        }
        self.tile_allowed_movements.push(allowed_inputs);
      }
    }

    // iterate map objects
    for i in 1..16 {
      let object_base_address = T::MAP_OBJECTS_MEM_ADDRESS + i * 16;
      if gb.gb.read_memory(object_base_address + 1) == 0 { continue; } // no sprite
      let event_flag_index = gb.gb.read_memory_word_le(object_base_address + 0xc);
      if gb.gb.read_memory(T::EVENT_FLAGS_MEM_ADDRESS + event_flag_index/8) & (1 << (event_flag_index % 8)) != 0 { continue; } // object hidden
      let x = gb.gb.read_memory(object_base_address + 3) + 2;
      let y = gb.gb.read_memory(object_base_address + 2) + 2;
      let movement = gb.gb.read_memory(object_base_address + 4);
      if [0x01, 0x03, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x16, 0x17, 0x18, 0x19, 0x1e, 0x1f].contains(&movement) { // is stationary, prevent moving into it
        for &(dx, dy, block_input) in &[
            (0, 1, Input::UP),
            (0, -1, Input::DOWN),
            (1, 0, Input::LEFT),
            (-1, 0, Input::RIGHT)] {
          if y as isize + dy < 0 || y as isize + dy >= self.height as isize { continue; } // out of bounds
          if x as isize + dx < 0 || x as isize + dx >= self.width as isize { continue; } // out of bounds
          let nx = (x as isize + dx) as usize;
          let ny = (y as isize + dy) as usize;
          self.tile_allowed_movements[self.width * ny + nx] -= block_input;
        }
        if gb.gb.read_memory(object_base_address + 8) & 0xf == 2 && [0x06, 0x07, 0x08, 0x09].contains(&movement) { // is stationary trainer
          let script_pointer = gb.gb.read_memory_word_le(object_base_address + 0xa);
          let map_scripts_bank = gb.gb.read_memory(T::MAP_SCRIPTS_BANK_MEM_ADDRESS);
          let script_pointer_rom_address = (i32::from(map_scripts_bank) << 16) + i32::from(script_pointer);
          let event_flag_index = gb.gb.read_rom_word_le(script_pointer_rom_address);
          if gb.gb.read_memory(T::EVENT_FLAGS_MEM_ADDRESS + event_flag_index/8) & (1 << (event_flag_index % 8)) == 0 { // not already fought
            let range = gb.gb.read_memory(object_base_address + 9);
            let (dx, dy) = match movement {
              6 => (0, 1),
              7 => (0, -1),
              8 => (-1, 0),
              9 => (1, 0),
              _ => panic!("got invalid movement {}", movement),
            };
            let mut nx = x as usize;
            let mut ny = y as usize;
            for _ in 1..=range {
              if ny as isize + dy < 0 || ny as isize + dy >= self.height as isize { break; } // out of bounds
              if nx as isize + dx < 0 || nx as isize + dx >= self.width as isize { break; } // out of bounds
              nx = (nx as isize + dx) as usize;
              ny = (ny as isize + dy) as usize;
              self.tile_allowed_movements[self.width * ny + nx] = Input::empty(); // inside trainer's vision, can enter but not leave (without fighting)
            }
          }
        }
      }
    }
    self
  }

  pub fn tile_collision_string(&self) -> String {
    let mut buf = String::new();
    for y in 0..self.height {
      for x in 0..self.width {
        write!(&mut buf, " {:02x}", self.tile_collision[self.width * y + x]).unwrap();
      }
      write!(&mut buf, "\n").unwrap();
    }
    buf
  }
  pub fn tile_allowed_movements_string(&self) -> String {
    let mut buf = String::new();
    for y in 0..self.height {
      for x in 0..self.width {
        write!(&mut buf, " {:02x}", self.tile_allowed_movements[self.width * y + x].bits()).unwrap();
      }
      write!(&mut buf, "\n").unwrap();
    }
    buf
  }
}