use serde_derive::{Serialize, Deserialize};

use crate::constants::*;
use crate::metric::*;
use crate::metric::battle::*;
use crate::metric::battle::gen1::*;
use crate::multi::*;
use crate::rom::*;
use std::{cmp::{Ordering, max, min}, ops::RangeInclusive, rc::Rc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FightKOPlanState {
  num_turns: u16,
  move_order: MoveOrder,
  non_crit_damage: RangeInclusive<u16>,
  crit_damage: RangeInclusive<u16>,
  inner_plan: Rc<PlanState>,
}
impl PartialOrd for FightKOPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.num_turns != other.num_turns {
      let enemy_hp = if let PlanState::FightTurnState(FightTurnPlanState { enemy_hp, .. }) = *self.inner_plan { enemy_hp } else { panic!("unexpected inner plan state") };
      let other_enemy_hp = if let PlanState::FightTurnState(FightTurnPlanState { enemy_hp, .. }) = *other.inner_plan { enemy_hp } else { panic!("unexpected inner plan state") };
      if self.num_turns < other.num_turns {
        if enemy_hp > other_enemy_hp { None } else { Some(Ordering::Greater) }
      } else {
        if enemy_hp < other_enemy_hp { None } else { Some(Ordering::Less) }
      }
    } else {
      self.inner_plan.partial_cmp(&other.inner_plan)
    }
  }
}
impl PartialEq for FightKOPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

pub struct FightKOPlan<R> {
  num_turns: u16,
  move_order: MoveOrder,
  non_crit_damage: RangeInclusive<u16>,
  crit_damage: RangeInclusive<u16>,

  fight_turn_plan: Option<FightTurnPlan<R>>,

  mov: Move,
  effect: Option<MoveEffectResult>,
  enemy_attack: EnemyAttackDesc,
  skip_battle_menu: bool,
}
impl<R> FightKOPlan<R> {
  pub fn new(mov: Move, effect: Option<MoveEffectResult>, enemy_attack: EnemyAttackDesc) -> Self {
    Self {
      num_turns: 0,
      move_order: MoveOrder::PlayerFirst,
      non_crit_damage: 1..=0,
      crit_damage: 1..=0,
    
      fight_turn_plan: None,

      mov,
      effect,
      enemy_attack,
      skip_battle_menu: false,
    }
  }
  pub fn skip_battle_menu(self) -> Self { Self { skip_battle_menu: true, ..self } }
}
impl<R: MultiRom + HandleMenuInputAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + AIChooseMoveAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses + Gen1FightTurnAddresses + Gen1MoveEffectAddresses + TextAddresses> FightKOPlan<R> {
  fn create_fight_turn_plan(&mut self) -> FightTurnPlan<R> {
    let last_move = self.num_turns == 1;
    let effect = if last_move { None } else { self.effect };
    let enemy_attack = if last_move && self.move_order == MoveOrder::PlayerFirst { EnemyAttackDesc::NoAttack } else { self.enemy_attack.clone() };
    let move_order = if last_move { Some(self.move_order) } else { None };
    FightTurnPlan::new(AttackDesc::hit_or_crit_effect(self.mov, self.non_crit_damage.clone(), self.crit_damage.clone(), effect), enemy_attack, move_order)
  }
  fn initialize_variables(&mut self, gb: &mut Gb<R>, state: &GbState) {
    gb.restore(state);
    let move_info = MoveInfosFn::new(Who::Player).invoke(gb).into_iter().find(|move_info| move_info.mov == self.mov).expect("move not found");
    assert!(move_info.max_damage > 0, "selected move does not do any damage");
    let best_damage = max(move_info.max_damage, move_info.max_crit_damage);

    let player_mon_info = BattleMonInfoFn::new(Who::Player).invoke(gb);
    let enemy_mon_info = BattleMonInfoFn::new(Who::Enemy).invoke(gb);
    let enemy_hp = enemy_mon_info.hp;

    assert!(enemy_hp > 0);
    self.num_turns = (enemy_hp + best_damage - 1) / best_damage;
    let max_total_damage = self.num_turns * best_damage;
    let num_non_crits = if best_damage == move_info.max_damage { self.num_turns } else { min(self.num_turns, (max_total_damage - enemy_hp) / (best_damage - move_info.max_damage)) };
    let num_crits = self.num_turns - num_non_crits;
    let overkill_damage = num_non_crits * move_info.max_damage + num_crits * move_info.max_crit_damage - enemy_hp;

    log::trace!("KOPlan with {} hp remaining, needs {} turns ({}@{}/{}@{}) overkill {}", enemy_hp, self.num_turns, num_non_crits, move_info.max_damage, num_crits, move_info.max_crit_damage, overkill_damage);

    self.move_order = if player_mon_info.stats.spd >= enemy_mon_info.stats.spd { MoveOrder::PlayerFirst } else { MoveOrder::EnemyFirst };
    self.non_crit_damage = if num_non_crits == 0 { 1..=0 } else { (move_info.max_damage.saturating_sub(overkill_damage))..=move_info.max_damage };
    self.crit_damage = if num_crits == 0 { 1..=0 } else { (move_info.max_crit_damage.saturating_sub(overkill_damage))..=move_info.max_crit_damage };
  }
}
impl<R: MultiRom + HandleMenuInputAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + AIChooseMoveAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses + Gen1FightTurnAddresses + Gen1MoveEffectAddresses + TextAddresses> Plan<R> for FightKOPlan<R> {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::FightKOState(FightKOPlanState { num_turns: self.num_turns, move_order: self.move_order, non_crit_damage: self.non_crit_damage.clone(), crit_damage: self.crit_damage.clone(), inner_plan: Rc::new(self.fight_turn_plan.as_ref().unwrap().save()) })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::FightKOState(FightKOPlanState { num_turns, move_order, non_crit_damage, crit_damage, inner_plan }) = state {
      self.num_turns = *num_turns;
      self.move_order = *move_order;
      self.non_crit_damage = non_crit_damage.clone();
      self.crit_damage = crit_damage.clone();
      self.fight_turn_plan = Some(self.create_fight_turn_plan());
      self.fight_turn_plan.as_mut().unwrap().restore(inner_plan);
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.initialize_variables(gb, state);
    let mut plan = self.create_fight_turn_plan();
    if self.skip_battle_menu { plan = plan.skip_battle_menu(); }
    plan.initialize(gb, &state);
    self.fight_turn_plan = Some(plan);
  }
  fn is_safe(&self) -> bool {
    self.fight_turn_plan.as_ref().unwrap().is_safe()
  }
  fn get_blockable_inputs(&self) -> Input {
    self.fight_turn_plan.as_ref().unwrap().get_blockable_inputs()
  }
  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    self.fight_turn_plan.as_ref().unwrap().canonicalize_input(input)
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    match self.fight_turn_plan.as_mut().unwrap().execute_input(gb, state, input) {
      Some((new_state, Some(()))) => {
        if self.num_turns == 1 { return Some((new_state, Some(()))); }
        self.initialize_variables(gb, &new_state);
        self.fight_turn_plan = Some(self.create_fight_turn_plan());
        self.fight_turn_plan.as_mut().unwrap().initialize(gb, &new_state);
        Some((new_state, None))
      },
      Some((new_state, None)) => Some((new_state, None)),
      None => None,
    }
  }
}
