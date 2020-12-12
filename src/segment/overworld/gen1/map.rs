use crate::gb::*;
use crate::rom::*;
use gambatte::Input;
use std::{fmt::Write, cmp::min};

#[derive(Debug, Default, Hash, Eq, PartialEq)]
pub struct Map {
  pub width: usize,
  pub height: usize,
  tile: Vec<u8>,
  pub tile_allowed_movements: Vec<Input>,
}

impl Map {
  #[allow(clippy::cyclomatic_complexity)]
  pub fn load_gen1_map<T: JoypadAddresses + RngAddresses + Gen1MapAddresses>(mut self, gb: &Gb<T>) -> Self {
    let surfing = gb.gb.read_memory(T::SURF_STATE_MEM_ADDRESS) == 2;

    let tile_pair_collisions = self.parse_tile_pair_collisions(gb, if surfing { T::TILE_PAIR_COLLISIONS_WATER_ADDRESS } else { T::TILE_PAIR_COLLISIONS_LAND_ADDRESS });
    let passable_tiles = self.parse_passable_tile_list(gb);

    let tileset_blocks_ptr = i32::from(gb.gb.read_memory(T::MAP_TILESET_BANK_MEM_ADDRESS)) * 0x1_0000 + i32::from(gb.gb.read_memory_word_le(T::MAP_TILESET_BLOCKS_PTR_MEM_ADDRESS));
    let cur_tileset = gb.gb.read_memory(T::MAP_TILESET_MEM_ADDRESS);

    let map_index = gb.gb.read_memory(T::MAP_INDEX_MEM_ADDRESS);
    let map_header_bank = gb.gb.read_rom(T::MAP_HEADER_BANKS_ADDRESS + i32::from(map_index));
    let text_ptr_base_address = i32::from(map_header_bank) * 0x1_0000 + i32::from(gb.gb.read_memory_word_le(T::MAP_TEXT_PTR_MEM_ADDRESS));

    let map_block_width = gb.gb.read_memory(T::MAP_WIDTH_MEM_ADDRESS) as usize + 6;
    let map_block_height = gb.gb.read_memory(T::MAP_HEIGHT_MEM_ADDRESS) as usize + 6;
    self.width = map_block_width * 2;
    self.height = map_block_height * 2;

    // get tiles
    {
      let mut blocks = vec![];
      // get blocks
      for offset in 0..(map_block_height as u16 * map_block_width as u16) {
        blocks.push(gb.gb.read_memory(T::OVERWORLD_MAP_MEM_ADDRESS + offset));
      }

      for y in 0..self.height {
        for x in 0..self.width {
          let block = blocks[map_block_width * (y >> 1) + (x >> 1)];
          let tile = gb.gb.read_rom(tileset_blocks_ptr + i32::from(block)*0x10 + (x as i32 & 1)*2 + (y as i32 & 1)*8+4);
          self.tile.push(tile);
        }
      }
    }

    // get allowed movements
    for y in 0..self.height {
      for x in 0..self.width {
        let mut allowed_inputs = Input::empty();
        for &(input, dx, dy) in &[(Input::UP, 0, -1), (Input::DOWN, 0, 1), (Input::LEFT, -1, 0), (Input::RIGHT, 1, 0)] {
          if y as isize + dy < 0 || y as isize + dy >= self.height as isize { continue; } // out of bounds
          if x as isize + dx < 0 || x as isize + dx >= self.width as isize { continue; } // out of bounds
          let nx = (x as isize + dx) as usize;
          let ny = (y as isize + dy) as usize;
          let old_tile = self.tile[self.width * y + x];
          let new_tile = self.tile[self.width * ny + nx];
          if tile_pair_collisions.contains(&(cur_tileset, old_tile, new_tile)) || tile_pair_collisions.contains(&(cur_tileset, new_tile, old_tile)) { continue; } // tile pair forbidden
          if surfing && (new_tile == 0x14 || new_tile == 0x32 || new_tile == 0x48) { allowed_inputs |= input; } // water tiles allowed
          else if !passable_tiles.contains(&new_tile) { continue; } // destination tile not passable
          allowed_inputs |= input;
        }
        self.tile_allowed_movements.push(allowed_inputs);
      }
    }

    // iterate map objects
    for i in 1..16 {
      let object_base_address = T::MAP_SPRITE_STATE_DATA_2_MEM_ADDRESS + i * 16;
      if gb.gb.read_memory(object_base_address + 4) == 0 { continue; } // no sprite
      if self.is_sprite_hidden(gb, i as u8) { continue; } // object hidden
      let y = gb.gb.read_memory(object_base_address + 4) + 2;
      let x = gb.gb.read_memory(object_base_address + 5) + 2;
      let movement = gb.gb.read_memory(object_base_address + 6);
      let mov2 = gb.gb.read_memory(T::MAP_SPRITE_DATA_MEM_ADDRESS + (i-1) * 2 + 0);
      let text_id = gb.gb.read_memory(T::MAP_SPRITE_DATA_MEM_ADDRESS + (i-1) * 2 + 1);
      if movement == 0xff { // is stationary, prevent moving into it
          log::info!("stationary sprite at ({},{}) i={}, text_id={}", x-6, y-6, i, text_id);
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
        // if text_id == 0 { continue; } // no text_id
        let text_address = i32::from(gb.gb.read_rom_word_le(text_ptr_base_address + 2*(i32::from(text_id)-1)));
        let text_address = if text_address < 0x4000 { text_address } else { i32::from(map_header_bank) * 0x1_0000 + text_address };
        let trainer_header_address = {
          log::info!("text address for i={}: {:x} (base address: {:x}, text_id: {})", i, text_address, text_ptr_base_address, text_id);
          if gb.gb.read_rom(text_address) != 0x08 || gb.gb.read_rom(text_address + 1) != 0x21 { None }
          else {
            let text_address_2 = if gb.gb.read_rom(text_address + 4) == 0x18 {
              // follow rlative jump
              text_address + 6 + i32::from(gb.gb.read_rom(text_address + 5))
            } else {
              text_address + 4
            };
            if gb.gb.read_rom(text_address_2) == 0xcd && gb.gb.read_rom_word_le(text_address_2 + 1) == T::TALK_TO_TRAINER_FUNCTION_ADDRESS as u16 {
              Some(i32::from(map_header_bank) * 0x1_0000 + i32::from(gb.gb.read_rom_word_le(text_address + 2)))
            } else { None }
          }
        };
        if let Some(trainer_header_address) = trainer_header_address { // is stationary trainer
          let flag_bit = gb.gb.read_rom(trainer_header_address);
          let mut range = gb.gb.read_rom(trainer_header_address + 1) / 0x10;
          if mov2 == 0xd0 { range = min(range, 3); } // limit engage dist, 4+ won't engage when moving down
          let flag_byte = gb.gb.read_rom_word_le(trainer_header_address + 2);
          let already_fought = gb.gb.read_memory(flag_byte + u16::from(flag_bit/8)) & (1 << (flag_bit%8)) != 0;
          // log::info!("trainer trainer_header_address {:#x} flag_bit {} flag_byte {:#x} already_fought {} (byte {:x})", trainer_header_address, flag_bit, flag_byte, already_fought, gb.gb.read_memory(flag_byte + u16::from(flag_bit/8)));
          if !already_fought { // not already fought
            let (dx, dy) = match mov2 {
              0xd0 => (0, 1),
              0xd1 => (0, -1),
              0xd2 => (-1, 0),
              0xd3 => (1, 0),
              0xff => (0, 0), // trainer with no vision
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

  fn is_sprite_hidden<T: JoypadAddresses + RngAddresses + Gen1MapAddresses>(&self, gb: &Gb<T>, sprite_index: u8) -> bool {
    let mut base_address = T::MAP_MISSABLE_OBJECT_LIST_MEM_ADDRESS;
    let bit_index = loop {
      let spr_index = gb.gb.read_memory(base_address);
      if spr_index == 0xff { return false; }
      if sprite_index == spr_index { break gb.gb.read_memory(base_address + 1); }
      base_address += 2;
    };
    gb.gb.read_memory(T::MAP_MISSABLE_OBJECT_FLAGS_MEM_ADDRESS + u16::from(bit_index)/8) & (1 << (bit_index%8)) != 0
	}

  fn parse_tile_pair_collisions<T: JoypadAddresses + RngAddresses + Gen1MapAddresses>(&self, gb: &Gb<T>, mut base_address: i32) -> Vec<(u8, u8, u8)> {
    let mut res = vec![];
    loop {
      let tileset = gb.gb.read_rom(base_address);
      if tileset == 0xff { return res; }
      res.push((tileset, gb.gb.read_rom(base_address + 1), gb.gb.read_rom(base_address + 2)));
      base_address += 3;
    }
  }

  fn parse_passable_tile_list<T: JoypadAddresses + RngAddresses + Gen1MapAddresses>(&self, gb: &Gb<T>) -> Vec<u8> {
    let mut res = vec![];
    let mut base_address = T::MAP_TILESET_COLLISION_PTR_BANK_OFFSET + i32::from(gb.gb.read_memory_word_le(T::MAP_TILESET_COLLISION_PTR_MEM_ADDRESS));
    loop {
      let tile = gb.gb.read_rom(base_address);
      if tile == 0xff { return res; }
      res.push(tile);
      base_address += 1;
    }
  }

  pub fn tile_string(&self) -> String {
    let mut buf = String::new();
    for y in 0..self.height {
      for x in 0..self.width {
        write!(&mut buf, " {:02x}", self.tile[self.width * y + x]).unwrap();
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
