use crate::constants::*;
use crate::metric::battle::*;
use crate::rom::*;
use crate::segment::*;
use crate::segment::battle::gen2::*;
use crate::statebuffer::StateBuffer;
use std::collections::BTreeMap;

pub struct KOSegment {
  mov: Move,
  enemy_attack: EnemyAttack,
  has_effect: bool,
  berry_recovery_after_turn: Option<u16>,
  burn_damage: bool,
  crit_last: bool,
  buffer_size: usize,
}
impl KOSegment {
  #[allow(dead_code)]
  pub fn new(mov: Move, enemy_attack: EnemyAttack) -> Self {
    Self {
      mov,
      enemy_attack,
      has_effect: false,
      berry_recovery_after_turn: None,
      burn_damage: false,
      crit_last: false,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  pub fn has_effect(self) -> Self { Self { has_effect: true, ..self } }
  pub fn crit_last(self) -> Self { Self { crit_last: true, ..self } }
  pub fn with_berry_recovery_after_turn(self, turn: u16) -> Self { Self { berry_recovery_after_turn: Some(turn), ..self } }
  pub fn with_burn_damage(self) -> Self { Self { burn_damage: true, ..self } }
}
impl WithOutputBufferSize for KOSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + Gen2FightTurnAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + TextAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + AIChooseMoveAddresses + Gen2BattleSpiteAddresses> Segment<R> for KOSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let move_infos = gbe.execute_state(&sb, MoveInfosFn::new(Who::Player)).get_value_assert_all_equal();
    let move_info = move_infos.into_iter().find(|move_info| move_info.mov == self.mov).expect("move not found");
    assert!(move_info.max_damage > 0, "selected move does not do any damage");
    assert!(move_info.max_crit_damage > move_info.max_damage);

    let player_mon_info = gbe.execute_state(&sb, BattleMonInfoFn::new(Who::Player)).get_value_assert_all_equal();
    let enemy_mon_info = gbe.execute_state(&sb, BattleMonInfoFn::new(Who::Enemy)).get_value_assert_all_equal();

    let burn_damage = if self.burn_damage { (enemy_mon_info.max_hp + 4 ) / 8} else { 0 };

    let player_moves_first = player_mon_info.stats.spd >= enemy_mon_info.stats.spd;
    let num_total_turns = (enemy_mon_info.hp + move_info.max_crit_damage + burn_damage - 1) / (move_info.max_crit_damage + burn_damage);

    let mut active_states = BTreeMap::<u16, StateBuffer>::new();
    active_states.insert(enemy_mon_info.hp, sb);
    let mut goal_buffer = StateBuffer::with_max_size(self.buffer_size);

    while !active_states.is_empty() {
      let enemy_hp: u16 = *active_states.keys().next_back().unwrap();
      let num_turns = (enemy_hp + move_info.max_crit_damage + burn_damage - 1) / (move_info.max_crit_damage + burn_damage);
      let mut max_total_damage = num_turns * (move_info.max_crit_damage + burn_damage);
      if player_moves_first && max_total_damage - burn_damage >= enemy_hp {
        max_total_damage -= burn_damage; // if half-move is possible, go for it
      } else if !player_moves_first && max_total_damage - move_info.max_crit_damage >= enemy_hp {
        max_total_damage -= move_info.max_crit_damage; // if half-move is possible, go for it
      }
      let num_non_crits = (max_total_damage - enemy_hp) / (move_info.max_crit_damage - move_info.max_damage);
      let overkill_damage = (max_total_damage - enemy_hp) % (move_info.max_crit_damage - move_info.max_damage);
      let num_crits = num_turns - num_non_crits;
      let do_berry_recovery = if let Some(turn) = self.berry_recovery_after_turn {
        num_total_turns - num_turns + 1 == turn
      } else { false };

      let sb = active_states.remove(&enemy_hp).unwrap();
      log::info!("KOSegment {} states with {} hp remaining, needs {} turns ({}@{}/{}@{}) overkill {}", sb.len(), enemy_hp, num_turns, num_non_crits, move_info.max_damage, num_crits, move_info.max_crit_damage, overkill_damage);

      let expected_followup_move = if num_turns > 2 || !player_moves_first { Some(self.enemy_attack.mov) } else { None };

      if num_non_crits > 0 {
        let min_acceptable_damage = move_info.max_damage.saturating_sub(overkill_damage);
        for (damage, sb) in FightTurnSegment::new(self.mov, false, min_acceptable_damage, self.enemy_attack).with_has_effect(self.has_effect).with_burn_damage(self.burn_damage).with_expected_enemy_followup_move(expected_followup_move).with_berry_recovery(do_berry_recovery).execute_split(gbe, sb.clone()) {
          let remaining_enemy_hp = (enemy_hp - damage).saturating_sub(burn_damage);
          if remaining_enemy_hp == 0 {
            goal_buffer.add_all(sb);
          } else {
            active_states.entry(remaining_enemy_hp).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_all(sb);
          }
        }
      }
      if num_crits > 0 && (!self.crit_last || num_crits > 1 || num_non_crits == 0) {
        let min_acceptable_damage = move_info.max_crit_damage.saturating_sub(overkill_damage);
        for (damage, sb) in FightTurnSegment::new(self.mov, true, min_acceptable_damage, self.enemy_attack).with_has_effect(self.has_effect).with_burn_damage(self.burn_damage).with_expected_enemy_followup_move(expected_followup_move).with_berry_recovery(do_berry_recovery).execute_split(gbe, sb.clone()) {
          let remaining_enemy_hp = (enemy_hp - damage).saturating_sub(burn_damage);
          if remaining_enemy_hp == 0 {
            goal_buffer.add_all(sb);
          } else {
            active_states.entry(remaining_enemy_hp).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_all(sb);
          }
        }
      }
    }

    Some(((), goal_buffer)).into_iter().collect()
  }
}
