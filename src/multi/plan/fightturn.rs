use serde_derive::{Serialize, Deserialize};

use crate::constants::*;
use crate::metric::*;
use crate::metric::battle::*;
use crate::metric::battle::gen1::*;
use crate::multi::*;
use crate::rom::*;
use std::cmp::Ordering;
use std::{ops::RangeInclusive, rc::Rc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FightTurnPlanState {
  progress: FightTurnProgress,
  actual_move_order: MoveOrder,
  after_hit_text_count: u32,
  pub enemy_hp: u16,
  move_info: Option<MoveInfo>,
  sub_plan: Rc<PlanState>,
}
impl PartialOrd for FightTurnPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    let enemy_hp_ord = self.enemy_hp.cmp(&other.enemy_hp);
    let combine_with_enemy_hp = move |partial_ord: Option<Ordering>| {
      match partial_ord {
        None => None,
        Some(Ordering::Equal) => Some(enemy_hp_ord.reverse()),
        Some(Ordering::Less) => if enemy_hp_ord == Ordering::Less { None } else { Some(Ordering::Less) },
        Some(Ordering::Greater) => if enemy_hp_ord == Ordering::Greater { None } else { Some(Ordering::Greater) },
      }
    };
    if self.progress > FightTurnProgress::BattleMenuSelectMove && other.progress > FightTurnProgress::BattleMenuSelectMove && self.actual_move_order != other.actual_move_order {
      // State are incomparable if move order is different.
      return None;
    }
    if self.progress != other.progress {
      combine_with_enemy_hp(self.progress.partial_cmp(&other.progress))
    } else {
      combine_with_enemy_hp(self.sub_plan.partial_cmp(&other.sub_plan))
    }
  }
}
impl PartialEq for FightTurnPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EnemyAttackDesc {
  TrainerAI(TrainerAIAction),
  Attack(AttackDesc),
  NoAttack,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AttackDesc {
  mov: Move,
  typ: AttackType,
}
impl AttackDesc {
  pub fn hit(mov: Move, non_crit_damage: RangeInclusive<u16>) -> Self {
    Self { mov, typ: AttackType::Hit { non_crit_damage, crit_damage: 1..=0, effect: None } }
  }
  pub fn crit(mov: Move, crit_damage: RangeInclusive<u16>) -> Self {
    Self { mov, typ: AttackType::Hit { non_crit_damage: 1..=0, crit_damage, effect: None } }
  }
  pub fn hit_no_side_effect(mov: Move, non_crit_damage: RangeInclusive<u16>) -> Self {
    Self { mov, typ: AttackType::Hit { non_crit_damage, crit_damage: 1..=0, effect: Some(MoveEffectResult::NoEffect) } }
  }
  pub fn crit_no_side_effect(mov: Move, crit_damage: RangeInclusive<u16>) -> Self {
    Self { mov, typ: AttackType::Hit { non_crit_damage: 1..=0, crit_damage, effect: Some(MoveEffectResult::NoEffect) } }
  }
  pub fn crit_with_side_effect(mov: Move, crit_damage: RangeInclusive<u16>, effect: MoveEffectResult) -> Self {
    Self { mov, typ: AttackType::Hit { non_crit_damage: 1..=0, crit_damage, effect: Some(effect) } }
  }
  pub fn hit_or_crit_effect(mov: Move, non_crit_damage: RangeInclusive<u16>, crit_damage: RangeInclusive<u16>, effect: Option<MoveEffectResult>) -> Self {
    Self { mov, typ: AttackType::Hit { non_crit_damage, crit_damage, effect } }
  }
  pub fn hit_failed(mov: Move) -> Self {
    Self { mov, typ: AttackType::HitFailed }
  }
  pub fn stat_up_down(mov: Move) -> Self {
    Self { mov, typ: AttackType::StatUpDown }
  }
  pub fn effect_failed(mov: Move) -> Self {
    Self { mov, typ: AttackType::EffectFailed }
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AttackType {
  Hit { non_crit_damage: RangeInclusive<u16>, crit_damage: RangeInclusive<u16>, effect: Option<MoveEffectResult> },
  HitFailed,
  StatUpDown,
  EffectFailed,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum FightTurnProgress {
  BattleMenuSelectFight,
  BattleMenuSelectMove,
  FirstAttackUsedMoveText,
  FirstAttackAfterHitText,
  FirstAttackAfterEffectText,
  SecondAttackUsedMoveText,
  SecondAttackAfterHitText,  
  SecondAttackAfterEffectText,
}

pub struct FightTurnPlan<R> {
  progress: FightTurnProgress,

  // Set during turns
  actual_move_order: MoveOrder,
  after_hit_text_count: u32,
  enemy_hp: u16,
  move_info: Option<MoveInfo>,

  player_attack: AttackDesc,
  enemy_attack: EnemyAttackDesc,
  move_order: Option<MoveOrder>,

  battle_menu_plan: Option<Box<dyn Plan<R, Value=()>>>,
  select_move_menu_plan: Option<Box<dyn Plan<R, Value=MoveOrder>>>,
  use_move_text_plan: Option<Box<dyn Plan<R, Value=(u16, bool)>>>,
  after_hit_text_plan: Option<Box<dyn Plan<R, Value=()>>>,
  after_effect_text_plan: Option<Box<dyn Plan<R, Value=()>>>,
}
impl<R> FightTurnPlan<R> {
  pub fn new(player_attack: AttackDesc, enemy_attack: EnemyAttackDesc, move_order: Option<MoveOrder>) -> Self {
    Self {
      progress: FightTurnProgress::BattleMenuSelectFight,
      actual_move_order: MoveOrder::PlayerFirst,
      after_hit_text_count: 0,
      enemy_hp: 0,
      move_info: None,

      player_attack,
      enemy_attack,
      move_order,

      battle_menu_plan: None,
      select_move_menu_plan: None,
      use_move_text_plan: None,
      after_hit_text_plan: None,
      after_effect_text_plan: None,
    }
  }
  fn side_effect_text_skips(&self, mov: Move) -> u32 {
    match mov {
      Move::Ember => 0,
      _ => 2,
    }
  }
}
impl<R: MultiRom + HandleMenuInputAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + AIChooseMoveAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses + Gen1FightTurnAddresses + Gen1MoveEffectAddresses + TextAddresses> FightTurnPlan<R> {
  fn create_first_attack_used_move_text_plan(&mut self) -> Box<dyn Plan<R, Value=(u16, bool)>> {
    if self.actual_move_order == MoveOrder::EnemyFirst { // Enemy attacks first
      match &self.enemy_attack {
        EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
        EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks first"),
        EnemyAttackDesc::Attack(attack_desc) => {
          Box::new(TextPlan::with_metric(UseMoveMetric::new(attack_desc, self.move_info.as_ref().unwrap(), Some(BattleBeforeMoveMetric::for_player())), false).with_skip_ends(4)) // enemy mon used move!
        }
      }
    } else { // Player attacks first
      let before_opp_move_metric = if let EnemyAttackDesc::NoAttack = self.enemy_attack { None } else {
        Some(BattleBeforeMoveMetric::for_enemy(&self.enemy_attack))
      };
      Box::new(TextPlan::with_metric(UseMoveMetric::new(&self.player_attack, self.move_info.as_ref().unwrap(), before_opp_move_metric), false).with_skip_ends(4)) // mon used move!
    }
  }
  fn create_first_attack_after_hit_text_plan(&mut self) -> Box<dyn Plan<R, Value=()>> {
    assert!(self.after_hit_text_count > 0); // has after hit texts
    if self.actual_move_order == MoveOrder::EnemyFirst {
      match &self.enemy_attack {
        EnemyAttackDesc::TrainerAI(_) => panic!("TrainerAI has no after hit texts"),
        EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks first"),
        EnemyAttackDesc::Attack(attack_desc) => {
          match attack_desc.typ {
            AttackType::Hit { effect, .. } => {
              Box::new(SkipTextsPlan::with_metric(self.after_hit_text_count, AfterHitTextMetric::new(effect, Some(BattleBeforeMoveMetric::for_player()))))
            },
            AttackType::HitFailed | AttackType::EffectFailed => panic!("failed hit has no after hit texts"),
            AttackType::StatUpDown => panic!("StatUpDown has no after hit texts"),
          }
        },
      }
    } else { // Player attacks first
      match self.player_attack.typ {
        AttackType::Hit { effect, .. } => {
          let before_opp_move_metric = if let EnemyAttackDesc::NoAttack = self.enemy_attack { None } else {
            Some(BattleBeforeMoveMetric::for_enemy(&self.enemy_attack))
          };
          Box::new(SkipTextsPlan::with_metric(self.after_hit_text_count, AfterHitTextMetric::new(effect, before_opp_move_metric)))
        },
        AttackType::HitFailed | AttackType::EffectFailed => panic!("failed hit has no after hit texts"),
        AttackType::StatUpDown => panic!("StatUpDown has no after hit texts"),
      }
    }
  }
  fn create_first_attack_after_effect_text_plan(&mut self) -> Box<dyn Plan<R, Value=()>> {
    if self.actual_move_order == MoveOrder::EnemyFirst {
      match &self.enemy_attack {
        EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
        EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks first"),
        EnemyAttackDesc::Attack(attack_desc) => {
          match attack_desc.typ {
            AttackType::Hit { effect, .. } => {
              assert!(effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect); // Has some effect texts
              let ends_to_be_skipped = self.side_effect_text_skips(attack_desc.mov);
              Box::new(SkipTextsPlan::with_metric(1, BattleBeforeMoveMetric::for_player()).with_skip_ends(ends_to_be_skipped)) // Mon stat up/down/burned/...
            },
            AttackType::HitFailed | AttackType::EffectFailed => {
              Box::new(SkipTextsPlan::with_metric(1, BattleBeforeMoveMetric::for_player())) // But it failed
            },
            AttackType::StatUpDown => {
              Box::new(SkipTextsPlan::with_metric(1, BattleBeforeMoveMetric::for_player()).with_skip_ends(2)) // Mon stat up/down
            },
          }
        },
      }
    } else { // Player attacks first
      match self.player_attack.typ {
        AttackType::Hit { effect, .. } => {
          assert!(effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect); // Has some effect texts
          let ends_to_be_skipped = self.side_effect_text_skips(self.player_attack.mov);
          if let EnemyAttackDesc::NoAttack = self.enemy_attack {
            Box::new(SkipTextsPlan::new(1).with_skip_ends(ends_to_be_skipped)) // Mon stat up/down/burned/...
          } else {
            Box::new(SkipTextsPlan::with_metric(1, BattleBeforeMoveMetric::for_enemy(&self.enemy_attack)).with_skip_ends(ends_to_be_skipped)) // Mon stat up/down/burned/...
          }
        },
        AttackType::HitFailed | AttackType::EffectFailed => {
          if let EnemyAttackDesc::NoAttack = self.enemy_attack { panic!("NoAttack enemy gets to attack after failed hit") }
          Box::new(SkipTextsPlan::with_metric(1, BattleBeforeMoveMetric::for_enemy(&self.enemy_attack))) // But it failed
        },
        AttackType::StatUpDown => {
          if let EnemyAttackDesc::NoAttack = self.enemy_attack { panic!("NoAttack enemy gets to attack after StatUpDown") }
          Box::new(SkipTextsPlan::with_metric(1, BattleBeforeMoveMetric::for_enemy(&self.enemy_attack)).with_skip_ends(2)) // Mon stat up/down
        },
      }
    }
  }

  fn create_second_attack_used_move_text_plan(&mut self) -> Box<dyn Plan<R, Value=(u16, bool)>> {
    if self.actual_move_order == MoveOrder::PlayerFirst { // Enemy attacks second
      match &self.enemy_attack {
        EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
        EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks second"),
        EnemyAttackDesc::Attack(attack_desc) => {
          Box::new(TextPlan::with_metric(UseMoveMetric::new(attack_desc, self.move_info.as_ref().unwrap(), None), false).with_skip_ends(4)) // enemy mon used move!
        }
      }
    } else { // Player attacks second
      Box::new(TextPlan::with_metric(UseMoveMetric::new(&self.player_attack, self.move_info.as_ref().unwrap(), None), false).with_skip_ends(4)) // mon used move!
    }
  }
  fn create_second_attack_after_hit_text_plan(&mut self) -> Box<dyn Plan<R, Value=()>> {
    assert!(self.after_hit_text_count > 0); // has after hit texts
    if self.actual_move_order == MoveOrder::PlayerFirst { // Enemy attacks second
      match &self.enemy_attack {
        EnemyAttackDesc::TrainerAI(_) => panic!("TrainerAI has no after hit texts"),
        EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks second"),
        EnemyAttackDesc::Attack(attack_desc) => {
          match attack_desc.typ {
            AttackType::Hit { effect, .. } => {
              Box::new(SkipTextsPlan::with_metric(self.after_hit_text_count, AfterHitTextMetric::new(effect, None)))
            },
            AttackType::HitFailed | AttackType::EffectFailed => panic!("failed hit has no after hit texts"),
            AttackType::StatUpDown => panic!("StatUpDown has no after hit texts"),
          }
        },
      }
    } else { // Player attacks second
      match self.player_attack.typ {
        AttackType::Hit { effect, .. } => {
          Box::new(SkipTextsPlan::with_metric(self.after_hit_text_count, AfterHitTextMetric::new(effect, None)))
        },
        AttackType::HitFailed | AttackType::EffectFailed => panic!("failed hit has no after hit texts"),
        AttackType::StatUpDown => panic!("StatUpDown has no after hit texts"),
      }
    }
  }
  fn create_second_attack_after_effect_text_plan(&mut self) -> Box<dyn Plan<R, Value=()>> {
    if self.actual_move_order == MoveOrder::PlayerFirst { // Enemy attacks second
      match &self.enemy_attack {
        EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
        EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks first"),
        EnemyAttackDesc::Attack(attack_desc) => {
          match attack_desc.typ {
            AttackType::Hit { effect, .. } => {
              assert!(effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect); // Has some effect texts
              let ends_to_be_skipped = self.side_effect_text_skips(attack_desc.mov);
              Box::new(SkipTextsPlan::new(1).with_skip_ends(ends_to_be_skipped)) // Mon stat up/down/burned/...
            },
            AttackType::HitFailed | AttackType::EffectFailed => {
              Box::new(SkipTextsPlan::new(1)) // But it failed
            },
            AttackType::StatUpDown => {
              Box::new(SkipTextsPlan::new(1).with_skip_ends(2)) // Mon stat up/down
            },
          }
        },
      }
    } else { // Player attacks second
      match self.player_attack.typ {
        AttackType::Hit { effect, .. } => {
          assert!(effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect); // Has some effect texts
          let ends_to_be_skipped = self.side_effect_text_skips(self.player_attack.mov);
          Box::new(SkipTextsPlan::new(1).with_skip_ends(ends_to_be_skipped)) // Mon stat up/down/burned/...
        },
        AttackType::HitFailed | AttackType::EffectFailed => {
          Box::new(SkipTextsPlan::new(1)) // But it failed
        },
        AttackType::StatUpDown => {
          Box::new(SkipTextsPlan::new(1).with_skip_ends(2)) // Mon stat up/down
        },
      }
    }
  }
}
impl<R: MultiRom + HandleMenuInputAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + AIChooseMoveAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses + Gen1FightTurnAddresses + Gen1MoveEffectAddresses + TextAddresses> Plan<R> for FightTurnPlan<R> {
  type Value = ();

  fn save(&self) -> PlanState {
    let progress = self.progress;
    let actual_move_order = self.actual_move_order;
    let after_hit_text_count = self.after_hit_text_count;
    let enemy_hp = self.enemy_hp;
    let move_info = self.move_info.clone();
    let sub_plan_state = match self.progress {
      FightTurnProgress::BattleMenuSelectFight => self.battle_menu_plan.as_ref().unwrap().save(),
      FightTurnProgress::BattleMenuSelectMove => self.select_move_menu_plan.as_ref().unwrap().save(),
      FightTurnProgress::FirstAttackUsedMoveText => self.use_move_text_plan.as_ref().unwrap().save(),
      FightTurnProgress::FirstAttackAfterHitText => self.after_hit_text_plan.as_ref().unwrap().save(),
      FightTurnProgress::FirstAttackAfterEffectText => self.after_effect_text_plan.as_ref().unwrap().save(),
      FightTurnProgress::SecondAttackUsedMoveText => self.use_move_text_plan.as_ref().unwrap().save(),
      FightTurnProgress::SecondAttackAfterHitText => self.after_hit_text_plan.as_ref().unwrap().save(),
      FightTurnProgress::SecondAttackAfterEffectText => self.after_effect_text_plan.as_ref().unwrap().save(),
    };
    PlanState::FightTurnState(FightTurnPlanState { progress, actual_move_order, after_hit_text_count, enemy_hp, move_info, sub_plan: Rc::new(sub_plan_state) })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::FightTurnState(FightTurnPlanState { progress, actual_move_order, after_hit_text_count, enemy_hp, move_info, sub_plan }) = state {
      self.progress = *progress;
      self.actual_move_order = *actual_move_order;
      self.after_hit_text_count = *after_hit_text_count;
      self.enemy_hp = *enemy_hp;
      self.move_info = move_info.clone();
      let sub_plan_state = sub_plan.as_ref();
      match progress {
        FightTurnProgress::BattleMenuSelectFight => {
          self.battle_menu_plan = Some(Box::new(BattleMenuPlan::fight()));
          self.battle_menu_plan.as_mut().unwrap().restore(sub_plan_state);
        },
        FightTurnProgress::BattleMenuSelectMove => {
          self.select_move_menu_plan = Some(Box::new(SelectMoveMenuPlan::with_metric(self.player_attack.mov, MoveSelectMetric::new(&self))));
          self.select_move_menu_plan.as_mut().unwrap().restore(sub_plan_state);
        },
        FightTurnProgress::FirstAttackUsedMoveText => {
          self.use_move_text_plan = Some(self.create_first_attack_used_move_text_plan());
          self.use_move_text_plan.as_mut().unwrap().restore(sub_plan_state);
        },
        FightTurnProgress::FirstAttackAfterHitText => {
          self.after_hit_text_plan = Some(self.create_first_attack_after_hit_text_plan());
          self.after_hit_text_plan.as_mut().unwrap().restore(sub_plan_state);
        },
        FightTurnProgress::FirstAttackAfterEffectText => {
          self.after_effect_text_plan = Some(self.create_first_attack_after_effect_text_plan());
          self.after_effect_text_plan.as_mut().unwrap().restore(sub_plan_state);
        },
        FightTurnProgress::SecondAttackUsedMoveText => {
          self.use_move_text_plan = Some(self.create_second_attack_used_move_text_plan());
          self.use_move_text_plan.as_mut().unwrap().restore(sub_plan_state);
        },
        FightTurnProgress::SecondAttackAfterHitText => {
          self.after_hit_text_plan = Some(self.create_second_attack_after_hit_text_plan());
          self.after_hit_text_plan.as_mut().unwrap().restore(sub_plan_state);
        },
        FightTurnProgress::SecondAttackAfterEffectText => {
          self.after_effect_text_plan = Some(self.create_second_attack_after_effect_text_plan());
          self.after_effect_text_plan.as_mut().unwrap().restore(sub_plan_state);
        },
      }
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    gb.restore(state);
    self.enemy_hp = BattleMonInfoFn::new(Who::Enemy).invoke(gb).hp;
    self.battle_menu_plan = Some(Box::new(BattleMenuPlan::fight()));
    self.battle_menu_plan.as_mut().unwrap().initialize(gb, &state);
    self.progress = FightTurnProgress::BattleMenuSelectFight;
  }
  fn is_safe(&self) -> bool {
    match self.progress {
      FightTurnProgress::BattleMenuSelectFight => self.battle_menu_plan.as_ref().unwrap().is_safe(),
      FightTurnProgress::BattleMenuSelectMove => self.select_move_menu_plan.as_ref().unwrap().is_safe(),
      FightTurnProgress::FirstAttackUsedMoveText => self.use_move_text_plan.as_ref().unwrap().is_safe(),
      FightTurnProgress::FirstAttackAfterHitText => self.after_hit_text_plan.as_ref().unwrap().is_safe(),
      FightTurnProgress::FirstAttackAfterEffectText => self.after_effect_text_plan.as_ref().unwrap().is_safe(),
      FightTurnProgress::SecondAttackUsedMoveText => self.use_move_text_plan.as_ref().unwrap().is_safe(),
      FightTurnProgress::SecondAttackAfterHitText => self.after_hit_text_plan.as_ref().unwrap().is_safe(),
      FightTurnProgress::SecondAttackAfterEffectText => self.after_effect_text_plan.as_ref().unwrap().is_safe(),
    }
  }
  fn get_blockable_inputs(&self) -> Input {
    match self.progress {
      FightTurnProgress::BattleMenuSelectFight => self.battle_menu_plan.as_ref().unwrap().get_blockable_inputs(),
      FightTurnProgress::BattleMenuSelectMove => self.select_move_menu_plan.as_ref().unwrap().get_blockable_inputs(),
      FightTurnProgress::FirstAttackUsedMoveText => self.use_move_text_plan.as_ref().unwrap().get_blockable_inputs(),
      FightTurnProgress::FirstAttackAfterHitText => self.after_hit_text_plan.as_ref().unwrap().get_blockable_inputs(),
      FightTurnProgress::FirstAttackAfterEffectText => self.after_effect_text_plan.as_ref().unwrap().get_blockable_inputs(),
      FightTurnProgress::SecondAttackUsedMoveText => self.use_move_text_plan.as_ref().unwrap().get_blockable_inputs(),
      FightTurnProgress::SecondAttackAfterHitText => self.after_hit_text_plan.as_ref().unwrap().get_blockable_inputs(),
      FightTurnProgress::SecondAttackAfterEffectText => self.after_effect_text_plan.as_ref().unwrap().get_blockable_inputs(),
    }
  }
  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    match self.progress {
      FightTurnProgress::BattleMenuSelectFight => self.battle_menu_plan.as_ref().unwrap().canonicalize_input(input),
      FightTurnProgress::BattleMenuSelectMove => self.select_move_menu_plan.as_ref().unwrap().canonicalize_input(input),
      FightTurnProgress::FirstAttackUsedMoveText => self.use_move_text_plan.as_ref().unwrap().canonicalize_input(input),
      FightTurnProgress::FirstAttackAfterHitText => self.after_hit_text_plan.as_ref().unwrap().canonicalize_input(input),
      FightTurnProgress::FirstAttackAfterEffectText => self.after_effect_text_plan.as_ref().unwrap().canonicalize_input(input),
      FightTurnProgress::SecondAttackUsedMoveText => self.use_move_text_plan.as_ref().unwrap().canonicalize_input(input),
      FightTurnProgress::SecondAttackAfterHitText => self.after_hit_text_plan.as_ref().unwrap().canonicalize_input(input),
      FightTurnProgress::SecondAttackAfterEffectText => self.after_effect_text_plan.as_ref().unwrap().canonicalize_input(input),
    }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    match self.progress {
      FightTurnProgress::BattleMenuSelectFight => {
        match self.battle_menu_plan.as_mut().unwrap().execute_input(gb, state, input) {
          Some((new_state, Some(()))) => {
            self.select_move_menu_plan = Some(Box::new(SelectMoveMenuPlan::with_metric(self.player_attack.mov, MoveSelectMetric::new(&self))));
            self.select_move_menu_plan.as_mut().unwrap().initialize(gb, &new_state);
            self.progress = FightTurnProgress::BattleMenuSelectMove;
            Some((new_state, None))
          },
          Some((new_state, None)) => Some((new_state, None)),
          None => None,
        }
      },
      FightTurnProgress::BattleMenuSelectMove => {
        match self.select_move_menu_plan.as_mut().unwrap().execute_input(gb, state, input) {
          Some((new_state, Some(move_order))) => {
            self.actual_move_order = move_order;
            if self.actual_move_order == MoveOrder::PlayerFirst {
              self.move_info = Some(MoveInfosFn::new(Who::Player).invoke(gb).into_iter().find(|move_info| move_info.mov == self.player_attack.mov).expect("player move not found"));
              self.after_hit_text_count = if self.move_info.as_ref().unwrap().is_effective { 1 } else { 0 };
            } else if let EnemyAttackDesc::Attack(attack_desc) = &self.enemy_attack {
              self.move_info = Some(MoveInfosFn::new(Who::Enemy).invoke(gb).into_iter().find(|move_info| move_info.mov == attack_desc.mov).expect("enemy move not found"));
              self.after_hit_text_count = if self.move_info.as_ref().unwrap().is_effective { 1 } else { 0 };
            }

            self.use_move_text_plan = Some(self.create_first_attack_used_move_text_plan());
            self.use_move_text_plan.as_mut().unwrap().initialize(gb, &new_state);
            self.progress = FightTurnProgress::FirstAttackUsedMoveText;
            Some((new_state, None))
          },
          Some((new_state, None)) => Some((new_state, None)),
          None => None,
        }
      },
      FightTurnProgress::FirstAttackUsedMoveText => {
        match self.use_move_text_plan.as_mut().unwrap().execute_input(gb, state, input) {
          Some((new_state, Some((damage, crit)))) => {
            if crit { self.after_hit_text_count += 1; }
            if self.actual_move_order == MoveOrder::PlayerFirst { self.enemy_hp = self.enemy_hp.saturating_sub(damage) };

            if self.actual_move_order == MoveOrder::EnemyFirst {
              match &self.enemy_attack {
                EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
                EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks first"),
                EnemyAttackDesc::Attack(attack_desc) => {
                  match attack_desc.typ {
                    AttackType::Hit { effect, .. } => {
                      if self.after_hit_text_count > 0 { // has after hit texts
                        self.after_hit_text_plan = Some(self.create_first_attack_after_hit_text_plan());
                        self.after_hit_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                        self.progress = FightTurnProgress::FirstAttackAfterHitText;
                      } else if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
                        self.after_effect_text_plan = Some(self.create_first_attack_after_effect_text_plan());
                        self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                        self.progress = FightTurnProgress::FirstAttackAfterEffectText;
                      } else { // neither hit nor effect texts, start player attack
                        self.move_info = Some(MoveInfosFn::new(Who::Player).invoke(gb).into_iter().find(|move_info| move_info.mov == self.player_attack.mov).expect("player move not found"));
                        self.after_hit_text_count = if self.move_info.as_ref().unwrap().is_effective { 1 } else { 0 };

                        self.use_move_text_plan = Some(self.create_second_attack_used_move_text_plan());
                        self.use_move_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                        self.progress = FightTurnProgress::SecondAttackUsedMoveText;
                      }
                    },
                    AttackType::HitFailed | AttackType::EffectFailed | AttackType::StatUpDown => {
                      self.after_effect_text_plan = Some(self.create_first_attack_after_effect_text_plan());
                      self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                      self.progress = FightTurnProgress::FirstAttackAfterEffectText;
                    },
                  }
                },
              }
            } else { // Player attacks first
              match self.player_attack.typ {
                AttackType::Hit { effect, .. } => {
                  if self.after_hit_text_count > 0 { // has after hit texts
                    self.after_hit_text_plan = Some(self.create_first_attack_after_hit_text_plan());
                    self.after_hit_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                    self.progress = FightTurnProgress::FirstAttackAfterHitText;
                  } else if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
                    self.after_effect_text_plan = Some(self.create_first_attack_after_effect_text_plan());
                    self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                    self.progress = FightTurnProgress::FirstAttackAfterEffectText;
                  } else { // neither hit nor effect texts, start enemy attack (if any)
                    match &self.enemy_attack {
                      EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
                      EnemyAttackDesc::NoAttack => return Some((new_state, Some(()))), // Enemy does not attack, all done
                      EnemyAttackDesc::Attack(attack_desc) => {
                        self.move_info = Some(MoveInfosFn::new(Who::Enemy).invoke(gb).into_iter().find(|move_info| move_info.mov == attack_desc.mov).expect("enemy move not found"));
                        self.after_hit_text_count = if self.move_info.as_ref().unwrap().is_effective { 1 } else { 0 };

                        self.use_move_text_plan = Some(self.create_second_attack_used_move_text_plan());
                        self.use_move_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                        self.progress = FightTurnProgress::SecondAttackUsedMoveText;
                      }
                    }
                  }
                },
                AttackType::HitFailed | AttackType::EffectFailed | AttackType::StatUpDown => {
                  self.after_effect_text_plan = Some(self.create_first_attack_after_effect_text_plan());
                  self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                  self.progress = FightTurnProgress::FirstAttackAfterEffectText;
                },
              }
            }
            Some((new_state, None))
          },
          Some((new_state, None)) => Some((new_state, None)),
          None => None,
        }
      },
      FightTurnProgress::FirstAttackAfterHitText => {
        match self.after_hit_text_plan.as_mut().unwrap().execute_input(gb, state, input) {
          Some((new_state, Some(()))) => {
            if self.actual_move_order == MoveOrder::EnemyFirst {
              match &self.enemy_attack {
                EnemyAttackDesc::TrainerAI(_ai_action) => panic!("TrainerAI does not hit"),
                EnemyAttackDesc::NoAttack => panic!("NoAttack enemy does not hit"),
                EnemyAttackDesc::Attack(attack_desc) => {
                  match attack_desc.typ {
                    AttackType::Hit { effect, .. } => {
                      if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
                        self.after_effect_text_plan = Some(self.create_first_attack_after_effect_text_plan());
                        self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                        self.progress = FightTurnProgress::FirstAttackAfterEffectText;
                      } else { // no effect texts, start player attack
                        self.move_info = Some(MoveInfosFn::new(Who::Player).invoke(gb).into_iter().find(|move_info| move_info.mov == self.player_attack.mov).expect("player move not found"));
                        self.after_hit_text_count = if self.move_info.as_ref().unwrap().is_effective { 1 } else { 0 };

                        self.use_move_text_plan = Some(self.create_second_attack_used_move_text_plan());
                        self.use_move_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                        self.progress = FightTurnProgress::SecondAttackUsedMoveText;
                      }
                    },
                    AttackType::HitFailed | AttackType::EffectFailed => panic!("failed hit has no after hit texts"),
                    AttackType::StatUpDown => panic!("StatUpDown has no after hit texts"),
                  }
                },
              }
            } else { // Player attacks first
              match self.player_attack.typ {
                AttackType::Hit { effect, .. } => {
                  if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
                    self.after_effect_text_plan = Some(self.create_first_attack_after_effect_text_plan());
                    self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                    self.progress = FightTurnProgress::FirstAttackAfterEffectText;
                  } else { // no effect texts, start enemy attack (if any)
                    match &self.enemy_attack {
                      EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
                      EnemyAttackDesc::NoAttack => return Some((new_state, Some(()))), // Enemy does not attack, all done
                      EnemyAttackDesc::Attack(attack_desc) => {
                        self.move_info = Some(MoveInfosFn::new(Who::Enemy).invoke(gb).into_iter().find(|move_info| move_info.mov == attack_desc.mov).expect("enemy move not found"));
                        self.after_hit_text_count = if self.move_info.as_ref().unwrap().is_effective { 1 } else { 0 };

                        self.use_move_text_plan = Some(self.create_second_attack_used_move_text_plan());
                        self.use_move_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                        self.progress = FightTurnProgress::SecondAttackUsedMoveText;
                      }
                    }
                  }
                },
                AttackType::HitFailed | AttackType::EffectFailed => panic!("failed hit has no after hit texts"),
                AttackType::StatUpDown => panic!("StatUpDown has no after hit texts"),
              }
            }
            Some((new_state, None))
          },
          Some((new_state, None)) => Some((new_state, None)),
          None => None,
        }
      },
      FightTurnProgress::FirstAttackAfterEffectText => {
        match self.after_effect_text_plan.as_mut().unwrap().execute_input(gb, state, input) {
          Some((new_state, Some(()))) => {
            if self.actual_move_order == MoveOrder::EnemyFirst {
              self.move_info = Some(MoveInfosFn::new(Who::Player).invoke(gb).into_iter().find(|move_info| move_info.mov == self.player_attack.mov).expect("player move not found"));
              self.after_hit_text_count = if self.move_info.as_ref().unwrap().is_effective { 1 } else { 0 };

              self.use_move_text_plan = Some(self.create_second_attack_used_move_text_plan());
              self.use_move_text_plan.as_mut().unwrap().initialize(gb, &new_state);
              self.progress = FightTurnProgress::SecondAttackUsedMoveText;
            } else { // Player attacks first
              match &self.enemy_attack {
                EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
                EnemyAttackDesc::NoAttack => return Some((new_state, Some(()))), // Enemy does not attack, all done
                EnemyAttackDesc::Attack(attack_desc) => {
                  self.move_info = Some(MoveInfosFn::new(Who::Enemy).invoke(gb).into_iter().find(|move_info| move_info.mov == attack_desc.mov).expect("enemy move not found"));
                  self.after_hit_text_count = if self.move_info.as_ref().unwrap().is_effective { 1 } else { 0 };

                  self.use_move_text_plan = Some(self.create_second_attack_used_move_text_plan());
                  self.use_move_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                  self.progress = FightTurnProgress::SecondAttackUsedMoveText;
                }
              }
            }
            Some((new_state, None))
          },
          Some((new_state, None)) => Some((new_state, None)),
          None => None,
        }
      },
      FightTurnProgress::SecondAttackUsedMoveText => {
        match self.use_move_text_plan.as_mut().unwrap().execute_input(gb, state, input) {
          Some((new_state, Some((damage, crit)))) => {
            if crit { self.after_hit_text_count += 1; }
            if self.actual_move_order == MoveOrder::EnemyFirst { self.enemy_hp = self.enemy_hp.saturating_sub(damage) };

            if self.actual_move_order == MoveOrder::PlayerFirst { // Enemy attacks second
              match &self.enemy_attack {
                EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
                EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks second"),
                EnemyAttackDesc::Attack(attack_desc) => {
                  match attack_desc.typ {
                    AttackType::Hit { effect, .. } => {
                      if self.after_hit_text_count > 0 { // has after hit texts
                        self.after_hit_text_plan = Some(self.create_second_attack_after_hit_text_plan());
                        self.after_hit_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                        self.progress = FightTurnProgress::SecondAttackAfterHitText;
                      } else if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
                        self.after_effect_text_plan = Some(self.create_second_attack_after_effect_text_plan());
                        self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                        self.progress = FightTurnProgress::SecondAttackAfterEffectText;
                      } else { return Some((new_state, Some(()))) } // neither hit nor effect texts, we're done
                    },
                    AttackType::HitFailed | AttackType::EffectFailed | AttackType::StatUpDown => {
                      self.after_effect_text_plan = Some(self.create_second_attack_after_effect_text_plan());
                      self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                      self.progress = FightTurnProgress::SecondAttackAfterEffectText;
                    },
                  }
                },
              }
            } else { // Player attacks second
              match self.player_attack.typ {
                AttackType::Hit { effect, .. } => {
                  if self.after_hit_text_count > 0 { // has after hit texts
                    self.after_hit_text_plan = Some(self.create_second_attack_after_hit_text_plan());
                    self.after_hit_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                    self.progress = FightTurnProgress::SecondAttackAfterHitText;
                  } else if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
                    self.after_effect_text_plan = Some(self.create_second_attack_after_effect_text_plan());
                    self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                    self.progress = FightTurnProgress::SecondAttackAfterEffectText;
                  } else { return Some((new_state, Some(()))) } // neither hit nor effect texts, we're done
                },
                AttackType::HitFailed | AttackType::EffectFailed | AttackType::StatUpDown => {
                  self.after_effect_text_plan = Some(self.create_second_attack_after_effect_text_plan());
                  self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                  self.progress = FightTurnProgress::SecondAttackAfterEffectText;
                },
              }
            }
            Some((new_state, None))
          },
          Some((new_state, None)) => Some((new_state, None)),
          None => None,
        }
      },
      FightTurnProgress::SecondAttackAfterHitText => {
        match self.after_hit_text_plan.as_mut().unwrap().execute_input(gb, state, input) {
          Some((new_state, Some(()))) => {
            if self.actual_move_order == MoveOrder::PlayerFirst { // Enemy attacks second
              match &self.enemy_attack {
                EnemyAttackDesc::TrainerAI(_ai_action) => panic!("TrainerAI does not hit"),
                EnemyAttackDesc::NoAttack => panic!("NoAttack enemy does not hit"),
                EnemyAttackDesc::Attack(attack_desc) => {
                  match attack_desc.typ {
                    AttackType::Hit { effect, .. } => {
                      if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
                        self.after_effect_text_plan = Some(self.create_second_attack_after_effect_text_plan());
                        self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                        self.progress = FightTurnProgress::SecondAttackAfterEffectText;
                      } else { return Some((new_state, Some(()))) } // no effect texts, we're done
                    },
                    AttackType::HitFailed | AttackType::EffectFailed => panic!("failed hit has no after hit texts"),
                    AttackType::StatUpDown => panic!("StatUpDown has no after hit texts"),
                  }
                },
              }
            } else { // Player attacks second
              match self.player_attack.typ {
                AttackType::Hit { effect, .. } => {
                  if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
                    self.after_effect_text_plan = Some(self.create_second_attack_after_effect_text_plan());
                    self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                    self.progress = FightTurnProgress::SecondAttackAfterEffectText;
                  } else { return Some((new_state, Some(()))) } // no effect texts, we're done
                },
                AttackType::HitFailed | AttackType::EffectFailed => panic!("failed hit has no after hit texts"),
                AttackType::StatUpDown => panic!("StatUpDown has no after hit texts"),
              }
            }
            Some((new_state, None))
          },
          Some((new_state, None)) => Some((new_state, None)),
          None => None,
        }
      },
      FightTurnProgress::SecondAttackAfterEffectText => self.after_effect_text_plan.as_mut().unwrap().execute_input(gb, state, input),
    }
  }
}

struct MoveSelectMetric {
  expected_ai_move: Option<Move>,
  expected_move_order: Option<MoveOrder>,
  before_move_metric: BattleBeforeMoveMetric,
}
impl MoveSelectMetric {
  fn new<R>(plan: &FightTurnPlan<R>) -> Self {
    let expected_ai_move = match &plan.enemy_attack {
      EnemyAttackDesc::Attack(attack) => Some(attack.mov),
      _ => None,
    };
    let expected_move_order = if let EnemyAttackDesc::NoAttack = plan.enemy_attack {
      assert!(plan.move_order != Some(MoveOrder::EnemyFirst), "Enemy can't go first: No attack specified");
      Some(MoveOrder::PlayerFirst)
    } else { plan.move_order };
    Self {
      expected_ai_move,
      expected_move_order,
      before_move_metric: BattleBeforeMoveMetric::for_enemy(&plan.enemy_attack), // may be changed to player later
    }
  }
}
impl<R: MultiRom + AIChooseMoveAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses> Metric<R> for MoveSelectMetric {
  type ValueType = MoveOrder;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    let ai_move = AIChooseMoveMetric.evaluate(gb)?;
    if let Some(expected_ai_move) = self.expected_ai_move {
      if expected_ai_move != ai_move { return None; }
    }
    let move_order = BattleMoveOrderMetric.evaluate(gb)?;
    if let Some(expected_move_order) = self.expected_move_order {
      if expected_move_order != move_order { return None; }
    }
    self.before_move_metric.with_who(move_order.first()).evaluate(gb).and(Some(move_order))
  }
}

struct BattleBeforeMoveMetric {
  who: Who,
  expected_trainer_ai: TrainerAIAction,
}
impl BattleBeforeMoveMetric {
  fn for_enemy(enemy_attack: &EnemyAttackDesc) -> Self {
    let expected_trainer_ai = match enemy_attack {
      EnemyAttackDesc::TrainerAI(ai_action) => *ai_action,
      _ => TrainerAIAction::NoAction,
    };
    Self {
      who: Who::Enemy,
      expected_trainer_ai,
    }
  }
  fn for_player() -> Self {
    Self {
      who: Who::Player,
      expected_trainer_ai: TrainerAIAction::NoAction,
    }
  }
  fn with_who(&self, who: Who) -> Self { Self {who, ..*self} }
}
impl<R: MultiRom + BattleObedienceAddresses + Gen1TrainerAIAddresses> Metric<R> for BattleBeforeMoveMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    match self.who {
      Who::Player => {
        BattleObedienceMetric.expect(BattleObedience::Obey).evaluate(gb)
      },
      Who::Enemy => {
        TrainerAIMetric.expect(self.expected_trainer_ai).evaluate(gb)
      },
    }
  }
}


struct UseMoveMetric {
  typ: AttackType,
  max_damage: u16,
  max_crit_damage: u16,
  is_effective: bool,
  before_opp_move_metric: Option<BattleBeforeMoveMetric>,
}
impl UseMoveMetric {
  fn new(attack: &AttackDesc, move_info: &MoveInfo, before_opp_move_metric: Option<BattleBeforeMoveMetric>) -> Self {
    assert!(attack.mov == move_info.mov);
    Self {
      typ: attack.typ.clone(),
      max_damage: move_info.max_damage,
      max_crit_damage: move_info.max_crit_damage,
      is_effective: move_info.is_effective,
      before_opp_move_metric,
    }
  }
}
impl<R: MultiRom + Gen1FightTurnAddresses + Gen1MoveEffectAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses> Metric<R> for UseMoveMetric {
  type ValueType = (u16, bool);

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    match &self.typ {
      AttackType::Hit { non_crit_damage, crit_damage, effect } => {
        match Gen1NormalHitMetric::with_expected_max_damage(self.max_damage, self.max_crit_damage).evaluate(gb)? {
          FightTurnResult::Hit { damage } => {
            if !non_crit_damage.contains(&damage) { return None; }
            // If neither effective nor critical, need to check for move effect right away. Otherwise, check happens after texts.
            if !self.is_effective {
              if let Some(effect) = effect {
                if MoveEffectMetric.expect(*effect).evaluate(gb).is_none() { return None; }
                // If no move effect either, need to check opponent before move metric (if any).
                if *effect == MoveEffectResult::NoEffect && self.before_opp_move_metric.is_some() && self.before_opp_move_metric.as_ref().unwrap().evaluate(gb).is_none() { return None; }
              } else {
                // If no move effect either, need to check opponent before move metric (if any).
                if self.before_opp_move_metric.is_some() && self.before_opp_move_metric.as_ref().unwrap().evaluate(gb).is_none() { return None; }
              }
            }
            Some((damage, false))
          },
          FightTurnResult::CriticalHit { damage } => {
            if crit_damage.contains(&damage) { Some((damage, true)) } else { None }
          },
          _ => None,
        }
      },
      AttackType::HitFailed => Gen1NormalHitMetric::with_expected_max_damage(self.max_damage, self.max_crit_damage).expect(FightTurnResult::Failed).evaluate(gb).and(Some((0, false))),
      AttackType::StatUpDown => MoveEffectMetric.expect(MoveEffectResult::Success).evaluate(gb).and(Some((0, false))),
      AttackType::EffectFailed => MoveEffectMetric.expect(MoveEffectResult::Failed).evaluate(gb).and(Some((0, false))),
    }
  }
}


struct AfterHitTextMetric {
  effect: Option<MoveEffectResult>,
  before_opp_move_metric: Option<BattleBeforeMoveMetric>,
}
impl AfterHitTextMetric {
  fn new(effect: Option<MoveEffectResult>, before_opp_move_metric: Option<BattleBeforeMoveMetric>) -> Self {
    Self {
      effect,
      before_opp_move_metric,
    }
  }
}
impl<R: MultiRom + Gen1FightTurnAddresses + Gen1MoveEffectAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses> Metric<R> for AfterHitTextMetric {
  type ValueType = ();

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if let Some(effect) = self.effect {
      if MoveEffectMetric.expect(effect).evaluate(gb).is_none() { return None; }
      // If no move effect either, need to check opponent before move metric (if any).
      if effect == MoveEffectResult::NoEffect && self.before_opp_move_metric.is_some() && self.before_opp_move_metric.as_ref().unwrap().evaluate(gb).is_none() { return None; }
    } else {
      // If no move effect either, need to check opponent before move metric (if any).
      if self.before_opp_move_metric.is_some() && self.before_opp_move_metric.as_ref().unwrap().evaluate(gb).is_none() { return None; }
    }
    Some(())
  }
}
