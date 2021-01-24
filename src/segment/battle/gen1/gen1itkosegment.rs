use crate::metric::battle::gen1::*;
use crate::rom::*;
use crate::segment::*;
use crate::segment::battle::*;
use crate::segment::battle::gen1::*;
use crate::statebuffer::StateBuffer;
use std::collections::BTreeMap;

pub struct Gen1ITKOSegment {
  mov: Move,
  expected_effect: Option<MoveEffectResult>,
  enemy_attack: EnemyAttack,
  buffer_size: usize,
}
impl Gen1ITKOSegment {
  #[allow(dead_code)]
  pub fn new(mov: Move, enemy_attack: EnemyAttack) -> Self {
    Self {
      mov,
      expected_effect: None,
      enemy_attack,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
  #[allow(dead_code)]
  pub fn with_expected_effect(self, expected_effect: MoveEffectResult) -> Self { Self { expected_effect: Some(expected_effect), ..self } }
}
impl WithOutputBufferSize for Gen1ITKOSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + Gen1FightTurnAddresses + Gen1MoveEffectAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + AIChooseMoveAddresses + JoypadLowSensitivityAddresses + Gen1TrainerAIAddresses> Segment<R> for Gen1ITKOSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let move_infos = gbe.execute_state(&sb, MoveInfosFn::new(Who::Player)).get_value_assert_all_equal();
    let move_info = move_infos.into_iter().find(|move_info| move_info.mov == self.mov).expect("move not found");
    assert!(move_info.max_damage > 0, "selected move does not do any damage");
    let best_damage = max(move_info.max_damage, move_info.max_crit_damage);

    let enemy_mon_info = gbe.execute_state(&sb, BattleMonInfoFn::new(Who::Enemy)).get_value_assert_all_equal();

    let mut active_states = BTreeMap::<u16, StateBuffer>::new();
    active_states.insert(enemy_mon_info.hp, sb);
    let mut goal_buffer = StateBuffer::with_max_size(self.buffer_size);

    while !active_states.is_empty() {
      let enemy_hp: u16 = *active_states.keys().next_back().unwrap();
      let num_turns = (enemy_hp + best_damage - 1) / best_damage;
      let max_total_damage = num_turns * best_damage;
      let num_non_crits = if best_damage == move_info.max_damage { num_turns } else { min(num_turns, (max_total_damage - enemy_hp) / (best_damage - move_info.max_damage)) };
      let num_crits = num_turns - num_non_crits;
      let overkill_damage = num_non_crits * move_info.max_damage + num_crits * move_info.max_crit_damage - enemy_hp;

      let sb = active_states.remove(&enemy_hp).unwrap();
      log::info!("Gen1ITKOSegment {} states with {} hp remaining, needs {} turns ({}@{}/{}@{}) overkill {}", sb.len(), enemy_hp, num_turns, num_non_crits, move_info.max_damage, num_crits, move_info.max_crit_damage, overkill_damage);

      if num_non_crits > 0 {
        let min_acceptable_damage = move_info.max_damage.saturating_sub(overkill_damage);
        for (damage, sb) in Gen1ITFightTurnSegment::new(self.mov, false, min_acceptable_damage, self.enemy_attack).with_expected_effect(if num_turns == 1 { None } else { self.expected_effect }).execute_split(gbe, sb.clone()) {
          let remaining_enemy_hp = enemy_hp - damage;
          if remaining_enemy_hp == 0 {
            goal_buffer.add_all(sb);
          } else {
            active_states.entry(remaining_enemy_hp).or_insert_with(|| StateBuffer::with_max_size(self.buffer_size)).add_all(sb);
          }
        }
      }
      if num_crits > 0 {
        let min_acceptable_damage = move_info.max_crit_damage.saturating_sub(overkill_damage);
        for (damage, sb) in Gen1ITFightTurnSegment::new(self.mov, true, min_acceptable_damage, self.enemy_attack).with_expected_effect(if num_turns == 1 { None } else { self.expected_effect }).execute_split(gbe, sb.clone()) {
          let remaining_enemy_hp = enemy_hp - damage;
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
