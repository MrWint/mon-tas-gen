pub mod gen1;
pub mod gen2;



use std::cmp::{max, min};

use crate::constants::*;
use crate::metric::*;
use num_traits::cast::ToPrimitive;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum MoveOrder {
  PlayerFirst,
  EnemyFirst,
}
#[allow(dead_code)]
pub struct BattleMoveOrderMetric;
impl<R: JoypadAddresses + BattleDetermineMoveOrderAddresses> Metric<R> for BattleMoveOrderMetric {
  type ValueType = MoveOrder;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    let hit = gb.step_until(&[R::MOVE_ORDER_PLAYER_FIRST_ADDRESS, R::MOVE_ORDER_ENEMY_FIRST_ADDRESS]);
    if hit == R::MOVE_ORDER_PLAYER_FIRST_ADDRESS { return Some(MoveOrder::PlayerFirst); }
    if hit == R::MOVE_ORDER_ENEMY_FIRST_ADDRESS { return Some(MoveOrder::EnemyFirst); }
    None
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum BattleObedience {
  Obey,
  Disobey,
}
#[allow(dead_code)]
pub struct BattleObedienceMetric;
impl<R: JoypadAddresses + BattleObedienceAddresses> Metric<R> for BattleObedienceMetric {
  type ValueType = BattleObedience;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if gb.step_until(&[R::CHECK_OBEDIENCE_START_ADDRESS]) == 0 { return None; }
    let hit = gb.step_until(&[R::CHECK_OBEDIENCE_OBEY_ADDRESS, R::CHECK_OBEDIENCE_DISOBEY_ADDRESS]);
    if hit == R::CHECK_OBEDIENCE_OBEY_ADDRESS { Some(BattleObedience::Obey) }
    else if hit == R::CHECK_OBEDIENCE_DISOBEY_ADDRESS { Some(BattleObedience::Disobey) }
    else { None }
  }
}

pub struct AIChooseMoveMetric;
impl<R: JoypadAddresses + AIChooseMoveAddresses> Metric<R> for AIChooseMoveMetric {
  type ValueType = Move;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if gb.step_until(&[R::AFTER_AI_CHOOSE_MOVE_ADDRESS]) == 0 { return None; }
    Move::from_index(gb.gb().read_memory(R::CUR_ENEMY_MOVE_MEM_ADDRESS))
  }
}
pub struct ExpectedAIChooseMoveMetric {
  pub expected_move: Option<Move>,
}
impl<R: Rom + AIChooseMoveAddresses> Metric<R> for ExpectedAIChooseMoveMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    AIChooseMoveMetric {}.filter(|&m| {
      log::trace!("Enemy selected move: {:?}", m);
      if let Some(mov) = self.expected_move {
        m == mov
      } else {
        ![Move::QuickAttack, Move::MachPunch, Move::ExtremeSpeed, Move::Endure, Move::Protect, Move::Detect].contains(&m)
      }
    }).into_unit().evaluate(gb)
  }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Who {
  Enemy,
  Player,
}
impl Who {
  #[inline]
  pub fn choose<V>(self, player: V, enemy: V) -> V {
    match self {
      Who::Player => player,
      Who::Enemy => enemy,
    }
  }
  pub fn opp(self) -> Who {
    match self {
      Who::Player => Who::Enemy,
      Who::Enemy => Who::Player,
    }
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct MoveInfo {
  pub mov: Move,
  pub power: u8,
  pub typ: Type,
  pub min_damage: u16,
  pub max_damage: u16,
  pub min_crit_damage: u16,
  pub max_crit_damage: u16,
  pub is_effective: bool,
}

fn truncate_hl_bc(atk: u16, def: u16) -> (u8, u8) {
  if atk >= 0x100 || def >= 0x100 {
    (max(atk >> 2, 1) as u8, max(def >> 2, 1) as u8)
  } else { (atk as u8, def as u8) }
}

fn read_type_matchups<R: Rom + BattleMovesInfoAddresses>(gb: &dyn GbI<R>) -> Vec<(Type, Type, u32)> {
  let mut matchups = vec![];
  let mut current_matchup_address = R::TYPE_MATCHUPS_ADDRESS;
  loop {
    let at = gb.gb().read_rom(current_matchup_address + 0);
    if at == 0xFE { current_matchup_address += 1; continue; }
    if at == 0xFF  { break; }
    let attack_type = Type::from_index(at).expect(&format!("unknown type {}", at));
    let defense_type = Type::from_index(gb.gb().read_rom(current_matchup_address + 1)).unwrap();
    let effectiveness = u32::from(gb.gb().read_rom(current_matchup_address + 2));
    matchups.push((attack_type, defense_type, effectiveness));
    current_matchup_address += 3;
  }
  matchups
}

pub struct MoveInfosFn { who: Who }
impl MoveInfosFn {
  pub fn new(who: Who) -> Self {
    let res = Self { who };
    res
  }
}
impl<R: Rom + BattleMovesInfoAddresses + BattleMonInfoAddresses> StateFn<R> for MoveInfosFn {
  type OV = Vec<MoveInfo>;

  fn invoke(&self, gb: &dyn GbI<R>) -> Vec<MoveInfo> {
    let self_info = BattleMonInfoFn::new(self.who).invoke(gb);
    let opp_info = BattleMonInfoFn::new(self.who.opp()).invoke(gb);

    self_info.moves.iter().map(|&(mov, _)| {
      let move_base_address = R::MOVES_ADDRESS + (mov.to_i32().unwrap() - 1) * R::MOVES_ENTRY_LENGTH;
      assert!(gb.gb().read_rom(move_base_address) == mov.to_u8().unwrap());
      let power = gb.gb().read_rom(move_base_address + 2);
      let typ = Type::from_index(gb.gb().read_rom(move_base_address + 3)).unwrap();

      let (atk, def) = truncate_hl_bc(self_info.stats.atk_for(typ), opp_info.stats.def_for(typ));
      let (crit_atk, crit_def) = if R::is_gen2() && self_info.stat_levels.atk_for(typ) > opp_info.stat_levels.def_for(typ) { (atk, def) }
      else { truncate_hl_bc(self_info.orig_stats.atk_for(typ), opp_info.orig_stats.def_for(typ)) };
      let level = u32::from(self_info.level);
      let crit_level = if R::is_gen1() { 2 * level } else { level }; // in gen1, level is doubled for crits

      let calc_damage = |lvl: u32, atk: u8, mut def: u8, crit_multiplier: u32| {
        if power == 0 { return (0, false); }

        if mov == Move::SelfDestruct || mov == Move::Explosion { def = max(def >> 1, 1); } // Explosion moves halve enemy defense

        let mut damage = (lvl * 2 / 5 + 2) * u32::from(power) * u32::from(atk) / u32::from(def) / 50;
        if self_info.held_item.into_iter().any(|item| item.held_boost_type().into_iter().any(|t| t == typ)) {
          damage = damage * 110 / 100;
        }
        damage = min(damage * crit_multiplier, 997) + 2;

        if mov == Move::Struggle { return (damage, false); } // no stab or type effectiveness when using Struggle

        // apply gen2 badge type boosts
        if R::is_gen2() && self.who == Who::Player {
          let badges = gb.gb().read_memory_word_le(R::GEN2_BADGES_MEM_ADDRESS);
          if typ.get_gen2_type_boost_badge().into_iter().any(|badge| badges & badge != 0) {
            damage += damage / 8;
          }
        }

        // apply STAB boost
        if self_info.types.contains(&typ) {
          damage += damage / 2;
        }

        // apply type effectiveness
        let mut is_effective = false;
        let mut total_effectivity = 10;
        for (at, dt, eff) in read_type_matchups::<R>(gb) {
          if typ == at && opp_info.types.contains(&dt) {
            if R::is_gen1() { is_effective = true; }
            total_effectivity = total_effectivity * eff / 10;
            damage = if damage == 0 || eff == 0 { 0 } else { max(damage * eff / 10, 1) };
          }
        }
        if R::is_gen2() && total_effectivity != 10 { is_effective = true; }

        (damage, is_effective)
      };
      let (damage, is_effective) = calc_damage(level, atk, def, 1);
      let (crit_damage, _) = calc_damage(crit_level, crit_atk, crit_def, if R::is_gen2() { 2 } else { 1 }); // in gen2, damage is doubled for crits

      let min_damage = if damage > 1 { damage * 217 / 255 } else { damage } as u16;
      let max_damage = damage as u16;
      let min_crit_damage = if crit_damage > 1 { crit_damage * 217 / 255 } else {crit_damage } as u16;
      let max_crit_damage = crit_damage as u16;
      MoveInfo { mov, power, typ, min_damage, max_damage, min_crit_damage, max_crit_damage, is_effective }
    }).collect()
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Stats {
  pub atk: u16,
  pub def: u16,
  pub spd: u16,
  pub spc_atk: u16,
  pub spc_def: u16,
}
impl Stats {
  #[inline] pub fn atk_for(&self, typ: Type) -> u16 {
    if typ.is_special_type() { self.spc_atk } else { self.atk }
  }
  #[inline] pub fn def_for(&self, typ: Type) -> u16 {
    if typ.is_special_type() { self.spc_def } else { self.def }
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct StatLevels {
  pub atk: u8,
  pub def: u8,
  pub spd: u8,
  pub spc_atk: u8,
  pub spc_def: u8,
  pub accuracy: u8,
  pub evasion: u8,
}
impl StatLevels {
  #[inline] pub fn atk_for(&self, typ: Type) -> u8 {
    if typ.is_special_type() { self.spc_atk } else { self.atk }
  }
  #[inline] pub fn def_for(&self, typ: Type) -> u8 {
    if typ.is_special_type() { self.spc_def } else { self.def }
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct BattleMonInfo {
  pub species: Pokemon,
  pub held_item: Option<Item>,
  pub moves: Vec<(Move, u8)>,
  pub dvs: DVs,
  pub happiness: u8,
  pub level: u8,
  pub status: Status,
  pub hp: u16,
  pub max_hp: u16,
  pub stats: Stats,
  pub types: Vec<Type>,

  pub orig_stats: Stats,
  pub stat_levels: StatLevels
}

#[allow(dead_code)]
pub struct BattleMonInfoFn { who: Who }
impl BattleMonInfoFn {
  pub fn new(who: Who) -> Self { Self { who } }
}
impl<R: Rom + BattleMonInfoAddresses> StateFn<R> for BattleMonInfoFn {
  type OV = BattleMonInfo;

  fn invoke(&self, gb: &dyn GbI<R>) -> BattleMonInfo {
    let mon_struct_base_mem_address = self.who.choose(R::BATTLE_MON_STRUCT_MEM_ADDRESS, R::ENEMY_MON_STRUCT_MEM_ADDRESS);
    let species = Pokemon::from_index::<R>(gb.gb().read_memory(mon_struct_base_mem_address + 0)).unwrap();

    if R::is_gen1() {
      let hp = gb.gb().read_memory_word_be(mon_struct_base_mem_address + 1);
      // 3 is BoxLevel or PartyPos
      let status = Status::from_u8(gb.gb().read_memory(mon_struct_base_mem_address + 4));
      let types = (0..2).filter_map(|i| Type::from_index(gb.gb().read_memory(mon_struct_base_mem_address + 5 + i))).collect();
      // 7 is catch rate
      let moves = (0..4).filter_map(|i| Move::from_index(gb.gb().read_memory(mon_struct_base_mem_address + 8 + i)).map(|m| (m, gb.gb().read_memory(mon_struct_base_mem_address + 25 + i)))).collect();
      let dvs = DVs::from_u16_be(gb.gb().read_memory_word_be(mon_struct_base_mem_address + 12));
      let level = gb.gb().read_memory(mon_struct_base_mem_address + 14);
      let max_hp = gb.gb().read_memory_word_be(mon_struct_base_mem_address + 15);
      let stats = Stats {
        atk: gb.gb().read_memory_word_be(mon_struct_base_mem_address + 17),
        def: gb.gb().read_memory_word_be(mon_struct_base_mem_address + 19),
        spd: gb.gb().read_memory_word_be(mon_struct_base_mem_address + 21),
        spc_atk: gb.gb().read_memory_word_be(mon_struct_base_mem_address + 23),
        spc_def: gb.gb().read_memory_word_be(mon_struct_base_mem_address + 23),
      };
      // 25-28 are PPs

      let orig_stats = match self.who {
        Who::Player => {
          let stats_base_mem_address = R::GEN1_PARTY_MON_STATS_BASE_MEM_ADDRESS + R::GEN1_PARTY_MON_STRUCT_LEN * u16::from(gb.gb().read_memory(R::GEN1_PLAYER_MON_NUMBER_MEM_ADDRESS));
          let atk = gb.gb().read_memory_word_be(stats_base_mem_address + 0);
          let def = gb.gb().read_memory_word_be(stats_base_mem_address + 2);
          let spd = gb.gb().read_memory_word_be(stats_base_mem_address + 4);
          let spc = gb.gb().read_memory_word_be(stats_base_mem_address + 6);
          Stats { atk, def, spd, spc_atk: spc, spc_def: spc }
        },
        Who::Enemy => gen1_calc_enemy_stats(gb, species, level, dvs),
      };

      let stat_levels = {
        let stat_levels_base_mem_address = self.who.choose(R::BATTLE_MON_STAT_LEVELS_MEM_ADDRESS, R::ENEMY_MON_STAT_LEVELS_MEM_ADDRESS);
        let atk = gb.gb().read_memory(stat_levels_base_mem_address + 0);
        let def = gb.gb().read_memory(stat_levels_base_mem_address + 1);
        let spd = gb.gb().read_memory(stat_levels_base_mem_address + 2);
        let spc = gb.gb().read_memory(stat_levels_base_mem_address + 3);
        let accuracy = gb.gb().read_memory(stat_levels_base_mem_address + 4);
        let evasion = gb.gb().read_memory(stat_levels_base_mem_address + 5);
        StatLevels { atk, def, spd, spc_atk: spc, spc_def: spc, accuracy, evasion }
      };

      let held_item = None;
      let happiness = 0;

      BattleMonInfo { species, held_item, moves, dvs, happiness, level, status, hp, max_hp, stats, types, orig_stats, stat_levels }
    } else {
      let held_item = Item::from_gen2_index(gb.gb().read_memory(mon_struct_base_mem_address + 1));
      let moves = (0..4).filter_map(|i| Move::from_index(gb.gb().read_memory(mon_struct_base_mem_address + 2 + i)).map(|m| (m, gb.gb().read_memory(mon_struct_base_mem_address + 8 + i)))).collect();
      let dvs = DVs::from_u16_be(gb.gb().read_memory_word_be(mon_struct_base_mem_address + 6));
      // 8-11 are PPs
      let happiness = gb.gb().read_memory(mon_struct_base_mem_address + 12);
      let level = gb.gb().read_memory(mon_struct_base_mem_address + 13);
      let status = Status::from_u8(gb.gb().read_memory(mon_struct_base_mem_address + 14));
      // 15 is unused
      let hp = gb.gb().read_memory_word_be(mon_struct_base_mem_address + 16);
      let max_hp = gb.gb().read_memory_word_be(mon_struct_base_mem_address + 18);
      let stats = Stats {
        atk: gb.gb().read_memory_word_be(mon_struct_base_mem_address + 20),
        def: gb.gb().read_memory_word_be(mon_struct_base_mem_address + 22),
        spd: gb.gb().read_memory_word_be(mon_struct_base_mem_address + 24),
        spc_atk: gb.gb().read_memory_word_be(mon_struct_base_mem_address + 26),
        spc_def: gb.gb().read_memory_word_be(mon_struct_base_mem_address + 28),
      };
      let types = (0..2).filter_map(|i| Type::from_index(gb.gb().read_memory(mon_struct_base_mem_address + 30 + i))).collect();

      let orig_stats = {
        let stats_base_mem_address = self.who.choose(R::GEN2_BATTLE_MON_ORIG_STATS_MEM_ADDRESS, R::GEN2_ENEMY_MON_ORIG_STATS_MEM_ADDRESS);
        let atk = gb.gb().read_memory_word_be(stats_base_mem_address + 0);
        let def = gb.gb().read_memory_word_be(stats_base_mem_address + 2);
        let spd = gb.gb().read_memory_word_be(stats_base_mem_address + 4);
        let spc_atk = gb.gb().read_memory_word_be(stats_base_mem_address + 6);
        let spc_def = gb.gb().read_memory_word_be(stats_base_mem_address + 8);
        Stats { atk, def, spd, spc_atk, spc_def }
      };

      let stat_levels = {
        let stat_levels_base_mem_address = self.who.choose(R::BATTLE_MON_STAT_LEVELS_MEM_ADDRESS, R::ENEMY_MON_STAT_LEVELS_MEM_ADDRESS);
        let atk = gb.gb().read_memory(stat_levels_base_mem_address + 0);
        let def = gb.gb().read_memory(stat_levels_base_mem_address + 1);
        let spd = gb.gb().read_memory(stat_levels_base_mem_address + 2);
        let spc_atk = gb.gb().read_memory(stat_levels_base_mem_address + 3);
        let spc_def = gb.gb().read_memory(stat_levels_base_mem_address + 4);
        let accuracy = gb.gb().read_memory(stat_levels_base_mem_address + 5);
        let evasion = gb.gb().read_memory(stat_levels_base_mem_address + 6);
        StatLevels { atk, def, spd, spc_atk, spc_def, accuracy, evasion }
      };

      BattleMonInfo { species, held_item, moves, dvs, happiness, level, status, hp, max_hp, stats, types, orig_stats, stat_levels }
    }
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct BaseStats {
  pub hp: u8,
  pub atk: u8,
  pub def: u8,
  pub spd: u8,
  pub spc: u8,
}
fn gen1_get_base_stats<R: BattleMonInfoAddresses>(gb: &dyn GbI<R>, species: Pokemon) -> BaseStats {
  if species == Pokemon::Mew {
    BaseStats {
      hp: 100,
      atk: 100,
      def: 100,
      spd: 100,
      spc: 100,
    }
  } else {
    let base_stats_address = R::GEN1_BASE_STATS_BASE_ADDRESS + (species.to_i32().unwrap() - 1) * R::GEN1_BASE_STATS_LEN + 1;
    BaseStats {
      hp: gb.gb().read_rom(base_stats_address + 0),
      atk: gb.gb().read_rom(base_stats_address + 1),
      def: gb.gb().read_rom(base_stats_address + 2),
      spd: gb.gb().read_rom(base_stats_address + 3),
      spc: gb.gb().read_rom(base_stats_address + 4),
    }
  }
}

/// ignores stat exp, suitable for enemy mons
fn gen1_calc_enemy_stats<R: BattleMonInfoAddresses>(gb: &dyn GbI<R>, species: Pokemon, level: u8, dvs: DVs) -> Stats {
  let base_stats = gen1_get_base_stats(gb, species);
  Stats {
    atk: min(((u16::from(base_stats.atk) + u16::from(dvs.atk)) * 2 * u16::from(level)) / 100 + 5, 999),
    def: min(((u16::from(base_stats.def) + u16::from(dvs.def)) * 2 * u16::from(level)) / 100 + 5, 999),
    spd: min(((u16::from(base_stats.spd) + u16::from(dvs.spd)) * 2 * u16::from(level)) / 100 + 5, 999),
    spc_atk: min(((u16::from(base_stats.spc) + u16::from(dvs.spc)) * 2 * u16::from(level)) / 100 + 5, 999),
    spc_def: min(((u16::from(base_stats.spc) + u16::from(dvs.spc)) * 2 * u16::from(level)) / 100 + 5, 999),
  }
}

pub struct CatchSuccessMetric {}
impl<R: JoypadAddresses + BattleCatchMonAddresses> Metric<R> for CatchSuccessMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if gb.step_until(&[R::CATCH_SUCCESS_ADDRESS, R::CATCH_FAIL_ADDRESS]) == R::CATCH_SUCCESS_ADDRESS {
      Some(())
    } else {
      None
    }
  }
}
