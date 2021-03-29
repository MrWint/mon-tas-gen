use serde_derive::{Serialize, Deserialize};

use crate::metric::*;
use crate::metric::battle::*;
use crate::metric::battle::gen1::*;
use crate::multi::*;
use crate::rom::*;
use std::cmp::Ordering;
use std::rc::Rc;
use super::fightturn::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FightTurnITPlanState {
  progress: FightTurnITProgress,
  actual_move_order: MoveOrder,
  after_hit_text_count: u32,
  pub enemy_hp: u16,
  sub_plan: Rc<PlanState>,
}
impl PartialOrd for FightTurnITPlanState {
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
    if self.progress > FightTurnITProgress::BattleMenuSelectMove && other.progress > FightTurnITProgress::BattleMenuSelectMove && self.actual_move_order != other.actual_move_order {
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
impl PartialEq for FightTurnITPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum FightTurnITProgress {
  BattleMenuSelectFight,
  BattleMenuSelectMove,
  FirstAttackAfterHitText,
  FirstAttackAfterEffectText,
  SecondAttackAfterHitText,  
  SecondAttackAfterEffectText,
}

pub struct FightTurnITPlan<R> {
  progress: FightTurnITProgress,

  // Set during turns
  actual_move_order: MoveOrder,
  after_hit_text_count: u32,
  enemy_hp: u16,

  player_attack: AttackDesc,
  enemy_attack: EnemyAttackDesc,
  move_order: Option<MoveOrder>,
  skip_battle_menu: bool,

  battle_menu_plan: Option<Box<dyn Plan<R, Value=()>>>,
  select_move_menu_plan: Option<Box<dyn Plan<R, Value=MoveSelectITResult>>>,
  first_after_hit_text_plan: Option<Box<dyn Plan<R, Value=FirstAfterHitTextITResult>>>,
  after_hit_text_plan: Option<Box<dyn Plan<R, Value=()>>>,
  after_effect_text_plan: Option<Box<dyn Plan<R, Value=()>>>,
  first_after_effect_text_plan: Option<Box<dyn Plan<R, Value=SecondAttackITResult>>>,
}
impl<R> FightTurnITPlan<R> {
  pub fn new(player_attack: AttackDesc, enemy_attack: EnemyAttackDesc, move_order: Option<MoveOrder>) -> Self {
    Self {
      progress: FightTurnITProgress::BattleMenuSelectFight,
      actual_move_order: MoveOrder::PlayerFirst,
      after_hit_text_count: 0,
      enemy_hp: 0,

      player_attack,
      enemy_attack,
      move_order,
      skip_battle_menu: false,

      battle_menu_plan: None,
      select_move_menu_plan: None,
      first_after_hit_text_plan: None,
      after_hit_text_plan: None,
      after_effect_text_plan: None,
      first_after_effect_text_plan: None,
    }
  }
  pub fn skip_battle_menu(self) -> Self { Self { skip_battle_menu: true, ..self } }
}
fn create_first_attack_used_move_metric(actual_move_order: MoveOrder, player_attack: &AttackDesc, enemy_attack: &EnemyAttackDesc, move_info: &MoveInfo) -> UseMoveMetric {
  if actual_move_order == MoveOrder::EnemyFirst { // Enemy attacks first
    match enemy_attack {
      EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
      EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks first"),
      EnemyAttackDesc::Attack(attack_desc) => {
        UseMoveMetric::new(attack_desc, move_info, Some(BattleBeforeMoveMetric::for_player()))
      }
    }
  } else { // Player attacks first
    let before_opp_move_metric = if let EnemyAttackDesc::NoAttack = enemy_attack { None } else {
      Some(BattleBeforeMoveMetric::for_enemy(enemy_attack))
    };
    UseMoveMetric::new(player_attack, move_info, before_opp_move_metric)
  }
}
fn create_second_attack_used_move_metric(actual_move_order: MoveOrder, player_attack: &AttackDesc, enemy_attack: &EnemyAttackDesc, move_info: &MoveInfo) -> UseMoveMetric {
  if actual_move_order == MoveOrder::PlayerFirst { // Enemy attacks second
    match enemy_attack {
      EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
      EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks second"),
      EnemyAttackDesc::Attack(attack_desc) => {
        UseMoveMetric::new(attack_desc, move_info, None)
      }
    }
  } else { // Player attacks second
    UseMoveMetric::new(player_attack, move_info, None)
  }
}
impl<R: MultiRom + HandleMenuInputAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + AIChooseMoveAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses + Gen1FightTurnAddresses + Gen1MoveEffectAddresses> FightTurnITPlan<R> {
  fn create_first_attack_after_hit_text_plan(&mut self) -> Box<dyn Plan<R, Value=FirstAfterHitTextITResult>> {
    assert!(self.after_hit_text_count > 0); // has after hit texts
    Box::new(SkipTextsITPlan::with_metric(self.after_hit_text_count, FirstAfterHitTextITMetric::new(&self)))
  }
  fn create_first_attack_after_effect_text_plan(&mut self) -> Box<dyn Plan<R, Value=SecondAttackITResult>> {
    if self.actual_move_order == MoveOrder::EnemyFirst {
      match &self.enemy_attack {
        EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
        EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks first"),
        EnemyAttackDesc::Attack(attack_desc) => {
          match attack_desc.typ {
            AttackType::Hit { effect, .. } => {
              assert!(effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect); // Has some effect texts
              Box::new(SkipTextsITPlan::with_metric(1, PlayerSecondAttackITMetric::new(&self.player_attack))) // Mon stat up/down/burned/...
            },
            AttackType::HitFailed | AttackType::EffectFailed => {
              Box::new(SkipTextsITPlan::with_metric(1, PlayerSecondAttackITMetric::new(&self.player_attack))) // But it failed
            },
            AttackType::StatUpDown => {
              Box::new(SkipTextsITPlan::with_metric(1, PlayerSecondAttackITMetric::new(&self.player_attack))) // Mon stat up/down
            },
          }
        },
      }
    } else { // Player attacks first
      match self.player_attack.typ {
        AttackType::Hit { effect, .. } => {
          assert!(effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect); // Has some effect texts
          Box::new(SkipTextsITPlan::with_metric(1, EnemySecondAttackITMetric::new(&self.enemy_attack))) // Mon stat up/down/burned/...
        },
        AttackType::HitFailed | AttackType::EffectFailed => {
          if let EnemyAttackDesc::NoAttack = self.enemy_attack { panic!("NoAttack enemy gets to attack after failed hit") }
          Box::new(SkipTextsITPlan::with_metric(1, EnemySecondAttackITMetric::new(&self.enemy_attack))) // But it failed
        },
        AttackType::StatUpDown => {
          if let EnemyAttackDesc::NoAttack = self.enemy_attack { panic!("NoAttack enemy gets to attack after StatUpDown") }
          Box::new(SkipTextsITPlan::with_metric(1, EnemySecondAttackITMetric::new(&self.enemy_attack))) // Mon stat up/down
        },
      }
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
              Box::new(SkipTextsITPlan::with_metric(self.after_hit_text_count, AfterHitTextMetric::new(effect, None)))
            },
            AttackType::HitFailed | AttackType::EffectFailed => panic!("failed hit has no after hit texts"),
            AttackType::StatUpDown => panic!("StatUpDown has no after hit texts"),
          }
        },
      }
    } else { // Player attacks second
      match self.player_attack.typ {
        AttackType::Hit { effect, .. } => {
          Box::new(SkipTextsITPlan::with_metric(self.after_hit_text_count, AfterHitTextMetric::new(effect, None)))
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
              Box::new(SkipTextsITPlan::new(1)) // Mon stat up/down/burned/...
            },
            AttackType::HitFailed | AttackType::EffectFailed => {
              Box::new(SkipTextsITPlan::new(1)) // But it failed
            },
            AttackType::StatUpDown => {
              Box::new(SkipTextsITPlan::new(1)) // Mon stat up/down
            },
          }
        },
      }
    } else { // Player attacks second
      match self.player_attack.typ {
        AttackType::Hit { effect, .. } => {
          assert!(effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect); // Has some effect texts
          Box::new(SkipTextsITPlan::new(1)) // Mon stat up/down/burned/...
        },
        AttackType::HitFailed | AttackType::EffectFailed => {
          Box::new(SkipTextsITPlan::new(1)) // But it failed
        },
        AttackType::StatUpDown => {
          Box::new(SkipTextsITPlan::new(1)) // Mon stat up/down
        },
      }
    }
  }
}
impl<R: MultiRom + HandleMenuInputAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + AIChooseMoveAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses + Gen1FightTurnAddresses + Gen1MoveEffectAddresses> Plan<R> for FightTurnITPlan<R> {
  type Value = ();

  fn save(&self) -> PlanState {
    let progress = self.progress;
    let actual_move_order = self.actual_move_order;
    let after_hit_text_count = self.after_hit_text_count;
    let enemy_hp = self.enemy_hp;
    let sub_plan_state = match self.progress {
      FightTurnITProgress::BattleMenuSelectFight => self.battle_menu_plan.as_ref().unwrap().save(),
      FightTurnITProgress::BattleMenuSelectMove => self.select_move_menu_plan.as_ref().unwrap().save(),
      FightTurnITProgress::FirstAttackAfterHitText => self.first_after_hit_text_plan.as_ref().unwrap().save(),
      FightTurnITProgress::FirstAttackAfterEffectText => self.first_after_effect_text_plan.as_ref().unwrap().save(),
      FightTurnITProgress::SecondAttackAfterHitText => self.after_hit_text_plan.as_ref().unwrap().save(),
      FightTurnITProgress::SecondAttackAfterEffectText => self.after_effect_text_plan.as_ref().unwrap().save(),
    };
    PlanState::FightTurnITState(FightTurnITPlanState { progress, actual_move_order, after_hit_text_count, enemy_hp, sub_plan: Rc::new(sub_plan_state) })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::FightTurnITState(FightTurnITPlanState { progress, actual_move_order, after_hit_text_count, enemy_hp, sub_plan }) = state {
      self.progress = *progress;
      self.actual_move_order = *actual_move_order;
      self.after_hit_text_count = *after_hit_text_count;
      self.enemy_hp = *enemy_hp;
      let sub_plan_state = sub_plan.as_ref();
      match progress {
        FightTurnITProgress::BattleMenuSelectFight => {
          self.battle_menu_plan = Some(Box::new(BattleMenuPlan::fight()));
          self.battle_menu_plan.as_mut().unwrap().restore(sub_plan_state);
        },
        FightTurnITProgress::BattleMenuSelectMove => {
          self.select_move_menu_plan = Some(Box::new(SelectMoveMenuPlan::with_metric(self.player_attack.mov, MoveSelectITMetric::new(&self))));
          self.select_move_menu_plan.as_mut().unwrap().restore(sub_plan_state);
        },
        FightTurnITProgress::FirstAttackAfterHitText => {
          self.first_after_hit_text_plan = Some(self.create_first_attack_after_hit_text_plan());
          self.first_after_hit_text_plan.as_mut().unwrap().restore(sub_plan_state);
        },
        FightTurnITProgress::FirstAttackAfterEffectText => {
          self.first_after_effect_text_plan = Some(self.create_first_attack_after_effect_text_plan());
          self.first_after_effect_text_plan.as_mut().unwrap().restore(sub_plan_state);
        },
        FightTurnITProgress::SecondAttackAfterHitText => {
          self.after_hit_text_plan = Some(self.create_second_attack_after_hit_text_plan());
          self.after_hit_text_plan.as_mut().unwrap().restore(sub_plan_state);
        },
        FightTurnITProgress::SecondAttackAfterEffectText => {
          self.after_effect_text_plan = Some(self.create_second_attack_after_effect_text_plan());
          self.after_effect_text_plan.as_mut().unwrap().restore(sub_plan_state);
        },
      }
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    gb.restore(state);
    self.enemy_hp = BattleMonInfoFn::new(Who::Enemy).invoke(gb).hp;
    if self.skip_battle_menu {
      self.select_move_menu_plan = Some(Box::new(SelectMoveMenuPlan::with_metric(self.player_attack.mov, MoveSelectITMetric::new(&self))));
      self.select_move_menu_plan.as_mut().unwrap().initialize(gb, &state);
      self.progress = FightTurnITProgress::BattleMenuSelectMove;
    } else {
      self.battle_menu_plan = Some(Box::new(BattleMenuPlan::fight()));
      self.battle_menu_plan.as_mut().unwrap().initialize(gb, &state);
      self.progress = FightTurnITProgress::BattleMenuSelectFight;
    }
  }
  fn is_safe(&self) -> bool {
    match self.progress {
      FightTurnITProgress::BattleMenuSelectFight => self.battle_menu_plan.as_ref().unwrap().is_safe(),
      FightTurnITProgress::BattleMenuSelectMove => self.select_move_menu_plan.as_ref().unwrap().is_safe(),
      FightTurnITProgress::FirstAttackAfterHitText => self.first_after_hit_text_plan.as_ref().unwrap().is_safe(),
      FightTurnITProgress::FirstAttackAfterEffectText => self.first_after_effect_text_plan.as_ref().unwrap().is_safe(),
      FightTurnITProgress::SecondAttackAfterHitText => self.after_hit_text_plan.as_ref().unwrap().is_safe(),
      FightTurnITProgress::SecondAttackAfterEffectText => self.after_effect_text_plan.as_ref().unwrap().is_safe(),
    }
  }
  fn get_blockable_inputs(&self) -> Input {
    match self.progress {
      FightTurnITProgress::BattleMenuSelectFight => self.battle_menu_plan.as_ref().unwrap().get_blockable_inputs(),
      FightTurnITProgress::BattleMenuSelectMove => self.select_move_menu_plan.as_ref().unwrap().get_blockable_inputs(),
      FightTurnITProgress::FirstAttackAfterHitText => self.first_after_hit_text_plan.as_ref().unwrap().get_blockable_inputs(),
      FightTurnITProgress::FirstAttackAfterEffectText => self.first_after_effect_text_plan.as_ref().unwrap().get_blockable_inputs(),
      FightTurnITProgress::SecondAttackAfterHitText => self.after_hit_text_plan.as_ref().unwrap().get_blockable_inputs(),
      FightTurnITProgress::SecondAttackAfterEffectText => self.after_effect_text_plan.as_ref().unwrap().get_blockable_inputs(),
    }
  }
  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    match self.progress {
      FightTurnITProgress::BattleMenuSelectFight => self.battle_menu_plan.as_ref().unwrap().canonicalize_input(input),
      FightTurnITProgress::BattleMenuSelectMove => self.select_move_menu_plan.as_ref().unwrap().canonicalize_input(input),
      FightTurnITProgress::FirstAttackAfterHitText => self.first_after_hit_text_plan.as_ref().unwrap().canonicalize_input(input),
      FightTurnITProgress::FirstAttackAfterEffectText => self.first_after_effect_text_plan.as_ref().unwrap().canonicalize_input(input),
      FightTurnITProgress::SecondAttackAfterHitText => self.after_hit_text_plan.as_ref().unwrap().canonicalize_input(input),
      FightTurnITProgress::SecondAttackAfterEffectText => self.after_effect_text_plan.as_ref().unwrap().canonicalize_input(input),
    }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    match self.progress {
      FightTurnITProgress::BattleMenuSelectFight => {
        match self.battle_menu_plan.as_mut().unwrap().execute_input(gb, state, input) {
          Some((new_state, Some(()))) => {
            self.select_move_menu_plan = Some(Box::new(SelectMoveMenuPlan::with_metric(self.player_attack.mov, MoveSelectITMetric::new(&self))));
            self.select_move_menu_plan.as_mut().unwrap().initialize(gb, &new_state);
            self.progress = FightTurnITProgress::BattleMenuSelectMove;
            Some((new_state, None))
          },
          Some((new_state, None)) => Some((new_state, None)),
          None => None,
        }
      },
      FightTurnITProgress::BattleMenuSelectMove => {
        match self.select_move_menu_plan.as_mut().unwrap().execute_input(gb, state, input) {
          Some((new_state, Some(result))) => {
            match result {
              MoveSelectITResult::FirstAfterHitTexts { actual_move_order, enemy_damage, after_hit_text_count } => {
                self.actual_move_order = actual_move_order;
                self.after_hit_text_count = after_hit_text_count;
                self.enemy_hp = self.enemy_hp.saturating_sub(enemy_damage);
                self.first_after_hit_text_plan = Some(self.create_first_attack_after_hit_text_plan());
                self.first_after_hit_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                self.progress = FightTurnITProgress::FirstAttackAfterHitText;
              },
              MoveSelectITResult::FirstAfterEffectTexts { actual_move_order, enemy_damage } => {
                self.actual_move_order = actual_move_order;
                self.enemy_hp = self.enemy_hp.saturating_sub(enemy_damage);
                self.first_after_effect_text_plan = Some(self.create_first_attack_after_effect_text_plan());
                self.first_after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                self.progress = FightTurnITProgress::FirstAttackAfterEffectText;
              },
              MoveSelectITResult::SecondAfterHitTexts { actual_move_order, enemy_damage, after_hit_text_count } => {
                self.actual_move_order = actual_move_order;
                self.after_hit_text_count = after_hit_text_count;
                self.enemy_hp = self.enemy_hp.saturating_sub(enemy_damage);
                self.after_hit_text_plan = Some(self.create_second_attack_after_hit_text_plan());
                self.after_hit_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                self.progress = FightTurnITProgress::SecondAttackAfterHitText;
              },
              MoveSelectITResult::SecondAfterEffectTexts { actual_move_order, enemy_damage } => {
                self.actual_move_order = actual_move_order;
                self.enemy_hp = self.enemy_hp.saturating_sub(enemy_damage);
                self.after_effect_text_plan = Some(self.create_second_attack_after_effect_text_plan());
                self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                self.progress = FightTurnITProgress::SecondAttackAfterEffectText;
              },
              MoveSelectITResult::Done => return Some((new_state, Some(()))),
            }
            Some((new_state, None))
          },
          Some((new_state, None)) => Some((new_state, None)),
          None => None,
        }
      },
      FightTurnITProgress::FirstAttackAfterHitText => {
        match self.first_after_hit_text_plan.as_mut().unwrap().execute_input(gb, state, input) {
          Some((new_state, Some(result))) => {
            match result {
              FirstAfterHitTextITResult::FirstAfterEffectTexts => {
                self.first_after_effect_text_plan = Some(self.create_first_attack_after_effect_text_plan());
                self.first_after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                self.progress = FightTurnITProgress::FirstAttackAfterEffectText;
              },
              FirstAfterHitTextITResult::SecondAfterHitTexts { enemy_damage, after_hit_text_count } => {
                self.after_hit_text_count = after_hit_text_count;
                self.enemy_hp = self.enemy_hp.saturating_sub(enemy_damage);
                self.after_hit_text_plan = Some(self.create_second_attack_after_hit_text_plan());
                self.after_hit_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                self.progress = FightTurnITProgress::SecondAttackAfterHitText;
              },
              FirstAfterHitTextITResult::SecondAfterEffectTexts { enemy_damage } => {
                self.enemy_hp = self.enemy_hp.saturating_sub(enemy_damage);
                self.after_effect_text_plan = Some(self.create_second_attack_after_effect_text_plan());
                self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                self.progress = FightTurnITProgress::SecondAttackAfterEffectText;
              },
              FirstAfterHitTextITResult::Done => return Some((new_state, Some(()))),
            }
            Some((new_state, None))
          },
          Some((new_state, None)) => Some((new_state, None)),
          None => None,
        }
      },
      FightTurnITProgress::FirstAttackAfterEffectText => {
        match self.first_after_effect_text_plan.as_mut().unwrap().execute_input(gb, state, input) {
          Some((new_state, Some(result))) => {
            match result {
              SecondAttackITResult::AfterHitTexts { enemy_damage, after_hit_text_count } => {
                self.after_hit_text_count = after_hit_text_count;
                self.enemy_hp = self.enemy_hp.saturating_sub(enemy_damage);
                self.after_hit_text_plan = Some(self.create_second_attack_after_hit_text_plan());
                self.after_hit_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                self.progress = FightTurnITProgress::SecondAttackAfterHitText;
              },
              SecondAttackITResult::AfterEffectTexts { enemy_damage } => {
                self.enemy_hp = self.enemy_hp.saturating_sub(enemy_damage);
                self.after_effect_text_plan = Some(self.create_second_attack_after_effect_text_plan());
                self.after_effect_text_plan.as_mut().unwrap().initialize(gb, &new_state);
                self.progress = FightTurnITProgress::SecondAttackAfterEffectText;
              },
              SecondAttackITResult::Done => return Some((new_state, Some(()))),
            }
            Some((new_state, None))
          },
          Some((new_state, None)) => Some((new_state, None)),
          None => None,
        }
      },
      FightTurnITProgress::SecondAttackAfterHitText => {
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
                        self.progress = FightTurnITProgress::SecondAttackAfterEffectText;
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
                    self.progress = FightTurnITProgress::SecondAttackAfterEffectText;
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
      FightTurnITProgress::SecondAttackAfterEffectText => self.after_effect_text_plan.as_mut().unwrap().execute_input(gb, state, input),
    }
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum MoveSelectITResult {
  FirstAfterHitTexts { actual_move_order: MoveOrder, enemy_damage: u16, after_hit_text_count: u32 },
  FirstAfterEffectTexts { actual_move_order: MoveOrder, enemy_damage: u16 },
  SecondAfterHitTexts { actual_move_order: MoveOrder, enemy_damage: u16, after_hit_text_count: u32 },
  SecondAfterEffectTexts { actual_move_order: MoveOrder, enemy_damage: u16 },
  Done,
}
struct MoveSelectITMetric {
  move_select_metric: MoveSelectMetric,
  player_attack: AttackDesc,
  enemy_attack: EnemyAttackDesc,
}
impl MoveSelectITMetric {
  fn new<R>(plan: &FightTurnITPlan<R>) -> Self {
    Self {
      move_select_metric: MoveSelectMetric::new(&plan.enemy_attack, plan.move_order),
      player_attack: plan.player_attack.clone(),
      enemy_attack: plan.enemy_attack.clone(),
    }
  }
}
impl<R: MultiRom + AIChooseMoveAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + Gen1FightTurnAddresses + Gen1MoveEffectAddresses> Metric<R> for MoveSelectITMetric {
  type ValueType = MoveSelectITResult;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    let actual_move_order = self.move_select_metric.evaluate(gb)?;
    let move_info = if actual_move_order == MoveOrder::PlayerFirst {
      MoveInfosFn::new(Who::Player).invoke(gb).into_iter().find(|move_info| move_info.mov == self.player_attack.mov).expect("player move not found")
    } else if let EnemyAttackDesc::Attack(attack_desc) = &self.enemy_attack {
      MoveInfosFn::new(Who::Enemy).invoke(gb).into_iter().find(|move_info| move_info.mov == attack_desc.mov).expect("enemy move not found")
    } else {
      panic!("Enemy attacks with no move defined!");
    };
    let mut after_hit_text_count = if move_info.is_effective { 1 } else { 0 };

    let (damage, crit) = create_first_attack_used_move_metric(actual_move_order, &self.player_attack, &self.enemy_attack, &move_info).evaluate(gb)?;
    if crit { after_hit_text_count += 1; }

    if actual_move_order == MoveOrder::EnemyFirst {
      match &self.enemy_attack {
        EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
        EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks first"),
        EnemyAttackDesc::Attack(attack_desc) => {
          match attack_desc.typ {
            AttackType::Hit { effect, .. } => {
              if after_hit_text_count > 0 { // has after hit texts
                return Some(MoveSelectITResult::FirstAfterHitTexts { actual_move_order, enemy_damage: 0, after_hit_text_count });
              } else if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
                return Some(MoveSelectITResult::FirstAfterEffectTexts { actual_move_order, enemy_damage: 0 });
              } else { // neither hit nor effect texts, start player attack
                let second_move_info = MoveInfosFn::new(Who::Player).invoke(gb).into_iter().find(|move_info| move_info.mov == self.player_attack.mov).expect("player move not found");
                after_hit_text_count = if second_move_info.is_effective { 1 } else { 0 };

                let (damage, crit) = create_second_attack_used_move_metric(actual_move_order, &self.player_attack, &self.enemy_attack, &second_move_info).evaluate(gb)?;
                if crit { after_hit_text_count += 1; }

                match self.player_attack.typ {
                  AttackType::Hit { effect, .. } => {
                    if after_hit_text_count > 0 { // has after hit texts
                      return Some(MoveSelectITResult::SecondAfterHitTexts { actual_move_order, enemy_damage: damage, after_hit_text_count });
                    } else if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
                      return Some(MoveSelectITResult::SecondAfterEffectTexts { actual_move_order, enemy_damage: damage });
                    } else { // neither hit nor effect texts, we're done
                      return Some(MoveSelectITResult::Done);
                    }
                  },
                  AttackType::HitFailed | AttackType::EffectFailed | AttackType::StatUpDown => {
                    return Some(MoveSelectITResult::SecondAfterEffectTexts { actual_move_order, enemy_damage: damage });
                  },
                }
              }
            },
            AttackType::HitFailed | AttackType::EffectFailed | AttackType::StatUpDown => {
              return Some(MoveSelectITResult::FirstAfterEffectTexts { actual_move_order, enemy_damage: 0 });
            },
          }
        },
      }
    } else { // Player attacks first
      match self.player_attack.typ {
        AttackType::Hit { effect, .. } => {
          if after_hit_text_count > 0 { // has after hit texts
            return Some(MoveSelectITResult::FirstAfterHitTexts { actual_move_order, enemy_damage: damage, after_hit_text_count });
          } else if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
            return Some(MoveSelectITResult::FirstAfterEffectTexts { actual_move_order, enemy_damage: damage });
          } else { // neither hit nor effect texts, start enemy attack (if any)
            match &self.enemy_attack {
              EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
              EnemyAttackDesc::NoAttack => return Some(MoveSelectITResult::Done), // Enemy does not attack, all done
              EnemyAttackDesc::Attack(attack_desc) => {
                let second_move_info = MoveInfosFn::new(Who::Enemy).invoke(gb).into_iter().find(|move_info| move_info.mov == attack_desc.mov).expect("enemy move not found");
                after_hit_text_count = if second_move_info.is_effective { 1 } else { 0 };

                let (_, crit) = create_second_attack_used_move_metric(actual_move_order, &self.player_attack, &self.enemy_attack, &second_move_info).evaluate(gb)?;
                if crit { after_hit_text_count += 1; }

                match attack_desc.typ {
                  AttackType::Hit { effect, .. } => {
                    if after_hit_text_count > 0 { // has after hit texts
                      return Some(MoveSelectITResult::SecondAfterHitTexts { actual_move_order, enemy_damage: damage, after_hit_text_count });
                    } else if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
                      return Some(MoveSelectITResult::SecondAfterEffectTexts { actual_move_order, enemy_damage: damage });
                    } else { // neither hit nor effect texts, we're done
                    return Some(MoveSelectITResult::Done);
                    }
                  },
                  AttackType::HitFailed | AttackType::EffectFailed | AttackType::StatUpDown => {
                    return Some(MoveSelectITResult::SecondAfterEffectTexts { actual_move_order, enemy_damage: damage });
                  },
                }
              }
            }
          }
        },
        AttackType::HitFailed | AttackType::EffectFailed | AttackType::StatUpDown => {
          return Some(MoveSelectITResult::FirstAfterEffectTexts { actual_move_order, enemy_damage: damage });
        },
      }
    }
  }
}




#[derive(Debug, Eq, Hash, PartialEq)]
enum FirstAfterHitTextITResult {
  FirstAfterEffectTexts,
  SecondAfterHitTexts { enemy_damage: u16, after_hit_text_count: u32 },
  SecondAfterEffectTexts { enemy_damage: u16 },
  Done,
}
struct FirstAfterHitTextITMetric {
  player_attack: AttackDesc,
  enemy_attack: EnemyAttackDesc,
  actual_move_order: MoveOrder,
}
impl FirstAfterHitTextITMetric {
  fn new<R>(plan: &FightTurnITPlan<R>) -> Self {
    Self {
      player_attack: plan.player_attack.clone(),
      enemy_attack: plan.enemy_attack.clone(),
      actual_move_order: plan.actual_move_order,
    }
  }
}
impl<R: MultiRom + AIChooseMoveAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + Gen1FightTurnAddresses + Gen1MoveEffectAddresses> Metric<R> for FirstAfterHitTextITMetric {
  type ValueType = FirstAfterHitTextITResult;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {

    if self.actual_move_order == MoveOrder::EnemyFirst {
      match &self.enemy_attack {
        EnemyAttackDesc::TrainerAI(_) => panic!("TrainerAI has no after hit texts"),
        EnemyAttackDesc::NoAttack => panic!("NoAttack enemy attacks first"),
        EnemyAttackDesc::Attack(attack_desc) => {
          match attack_desc.typ {
            AttackType::Hit { effect, .. } => {
              if let Some(effect) = effect {
                if MoveEffectMetric.expect(effect).evaluate(gb).is_none() { return None; }
                if effect != MoveEffectResult::NoEffect { return Some(FirstAfterHitTextITResult::FirstAfterEffectTexts); }
              }
              match PlayerSecondAttackITMetric::new(&self.player_attack).evaluate(gb)? {
                SecondAttackITResult::AfterHitTexts { enemy_damage, after_hit_text_count} => Some(FirstAfterHitTextITResult::SecondAfterHitTexts { enemy_damage, after_hit_text_count }),
                SecondAttackITResult::AfterEffectTexts { enemy_damage } => Some(FirstAfterHitTextITResult::SecondAfterEffectTexts { enemy_damage }),
                SecondAttackITResult::Done => Some(FirstAfterHitTextITResult::Done),
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
          if let Some(effect) = effect {
            if MoveEffectMetric.expect(effect).evaluate(gb).is_none() { return None; }
            if effect != MoveEffectResult::NoEffect { return Some(FirstAfterHitTextITResult::FirstAfterEffectTexts); }
          }
          match EnemySecondAttackITMetric::new(&self.enemy_attack).evaluate(gb)? {
            SecondAttackITResult::AfterHitTexts { enemy_damage, after_hit_text_count} => Some(FirstAfterHitTextITResult::SecondAfterHitTexts { enemy_damage, after_hit_text_count }),
            SecondAttackITResult::AfterEffectTexts { enemy_damage } => Some(FirstAfterHitTextITResult::SecondAfterEffectTexts { enemy_damage }),
            SecondAttackITResult::Done => Some(FirstAfterHitTextITResult::Done),
          }
        },
        AttackType::HitFailed | AttackType::EffectFailed => panic!("failed hit has no after hit texts"),
        AttackType::StatUpDown => panic!("StatUpDown has no after hit texts"),
      }
    }
  }
}




#[derive(Debug, Eq, Hash, PartialEq)]
enum SecondAttackITResult {
  AfterHitTexts { enemy_damage: u16, after_hit_text_count: u32 },
  AfterEffectTexts { enemy_damage: u16 },
  Done,
}
struct PlayerSecondAttackITMetric {
  player_attack: AttackDesc,
}
impl PlayerSecondAttackITMetric {
  fn new(player_attack: &AttackDesc) -> Self {
    Self {
      player_attack: player_attack.clone(),
    }
  }
}
impl<R: MultiRom + AIChooseMoveAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + Gen1FightTurnAddresses + Gen1MoveEffectAddresses> Metric<R> for PlayerSecondAttackITMetric {
  type ValueType = SecondAttackITResult;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    if BattleBeforeMoveMetric::for_player().evaluate(gb).is_none() { return None; }

    let second_move_info = MoveInfosFn::new(Who::Player).invoke(gb).into_iter().find(|move_info| move_info.mov == self.player_attack.mov).expect("player move not found");
    let mut after_hit_text_count = if second_move_info.is_effective { 1 } else { 0 };

    let (damage, crit) = UseMoveMetric::new(&self.player_attack, &second_move_info, None).evaluate(gb)?;
    if crit { after_hit_text_count += 1; }

    match self.player_attack.typ {
      AttackType::Hit { effect, .. } => {
        if after_hit_text_count > 0 { // has after hit texts
          return Some(SecondAttackITResult::AfterHitTexts { enemy_damage: damage, after_hit_text_count });
        } else if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
          return Some(SecondAttackITResult::AfterEffectTexts { enemy_damage: damage });
        } else { // neither hit nor effect texts, we're done
          return Some(SecondAttackITResult::Done);
        }
      },
      AttackType::HitFailed | AttackType::EffectFailed | AttackType::StatUpDown => {
        return Some(SecondAttackITResult::AfterEffectTexts { enemy_damage: damage });
      },
    }
  }
}

struct EnemySecondAttackITMetric {
  enemy_attack: EnemyAttackDesc,
}
impl EnemySecondAttackITMetric {
  fn new(enemy_attack: &EnemyAttackDesc) -> Self {
    Self {
      enemy_attack: enemy_attack.clone(),
    }
  }
}
impl<R: MultiRom + AIChooseMoveAddresses + BattleDetermineMoveOrderAddresses + BattleObedienceAddresses + Gen1TrainerAIAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses + Gen1FightTurnAddresses + Gen1MoveEffectAddresses> Metric<R> for EnemySecondAttackITMetric {
  type ValueType = SecondAttackITResult;

  fn evaluate(&self, gb: &mut dyn GbI<R>) -> Option<Self::ValueType> {
    match &self.enemy_attack {
      EnemyAttackDesc::TrainerAI(_ai_action) => unimplemented!("TrainerAI handling not implemented"),
      EnemyAttackDesc::NoAttack => return Some(SecondAttackITResult::Done), // Enemy does not attack, all done
      EnemyAttackDesc::Attack(attack_desc) => {
        if BattleBeforeMoveMetric::for_enemy(&self.enemy_attack).evaluate(gb).is_none() { return None; }

        let second_move_info = MoveInfosFn::new(Who::Enemy).invoke(gb).into_iter().find(|move_info| move_info.mov == attack_desc.mov).expect("enemy move not found");
        let mut after_hit_text_count = if second_move_info.is_effective { 1 } else { 0 };

        let (_, crit) = UseMoveMetric::new(attack_desc, &second_move_info, None).evaluate(gb)?;
        if crit { after_hit_text_count += 1; }

        match attack_desc.typ {
          AttackType::Hit { effect, .. } => {
            if after_hit_text_count > 0 { // has after hit texts
              return Some(SecondAttackITResult::AfterHitTexts { enemy_damage: 0, after_hit_text_count });
            } else if effect.unwrap_or(MoveEffectResult::NoEffect) != MoveEffectResult::NoEffect { // Has some effect texts
              return Some(SecondAttackITResult::AfterEffectTexts { enemy_damage: 0 });
            } else { // neither hit nor effect texts, we're done
              return Some(SecondAttackITResult::Done);
            }
          },
          AttackType::HitFailed | AttackType::EffectFailed | AttackType::StatUpDown => {
            return Some(SecondAttackITResult::AfterEffectTexts { enemy_damage: 0 });
          },
        }
      }
    }
  }
}
