use serde_derive::{Serialize, Deserialize};

use crate::rom::*;
use super::*;

use std::cmp::Ordering;
use std::rc::Rc;

pub trait Plan<R: Rom> {
  /// Type of the resulting values of this plan.
  type Value;

  /// Saves current plan state
  fn save(&self) -> PlanState;
  /// Restores saved plan state
  fn restore(&mut self, state: &PlanState);
  /// Checks whether the current state is considered safe, i.e. there is guaranteed to be a sequence of inputs which completes it.
  fn is_safe(&self) -> bool;
  /// Returns the set of buttons which can be blocked from being pressed by the previous input.
  fn get_blockable_inputs(&self) -> Input;
  /// Analyzed the initial state and initialize the internal plan state to allow completion.
  /// Called before any canonicalize_input or execute_input calls are made.
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState);
  /// Normalizes the input to one that has the same effect, used to deduplicate inputs with the same outcome.
  fn canonicalize_input(&self, input: Input) -> Option<Input>;
  /// Executes the given input on the given state. If successful, returns a new state and updates the internal state, potentially finishing and returning a value.
  /// If the plan returns, the internal state is no longer guaranteed to be consistent.
  /// The execution step may skip multiple input uses, as long as all of them happen on the same input frame and use the same given input.
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<Self::Value>)>;
}
pub struct NullPlan;
impl<R: MultiRom> Plan<R> for NullPlan {
  type Value = ();

  fn save(&self) -> PlanState { PlanState::NullState }
  fn restore(&mut self, _state: &PlanState) { }
  fn is_safe(&self) -> bool { true }
  fn get_blockable_inputs(&self) -> Input { Input::empty() }
  fn initialize(&mut self, _gb: &mut Gb<R>, _state: &GbState) { }
  fn canonicalize_input(&self, _input: Input) -> Option<Input> { Some(Input::empty()) }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    gb.restore(state);
    gb.input(input);
    gb.step();
    Some((gb.save(), None))
  }
}

pub struct ListPlan<R: Rom> {
  plans: Vec<Box<dyn Plan<R, Value=()>>>,
  cur_item: usize,
  cur_item_is_initialized: bool,
}
impl<R: Rom> ListPlan<R> {
  pub fn new(plans: Vec<Box<dyn Plan<R, Value=()>>>) -> Self {
    assert!(!plans.is_empty());
    Self {
      plans,
      cur_item: 0,
      cur_item_is_initialized: false,
    }
  }
  fn initialize_cur_item(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.plans[self.cur_item].initialize(gb, state);
    self.cur_item_is_initialized = true;
  }
  pub fn ensure_cur_item_initialized(&mut self, gb: &mut Gb<R>, state: &GbState) {
    if !self.cur_item_is_initialized {
      self.plans[self.cur_item].initialize(gb, state);
      self.cur_item_is_initialized = true;
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListPlanState {
  cur_item: usize,
  sub_plan: Option<Rc<PlanState>>,
}
impl PartialOrd for ListPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.cur_item != other.cur_item {
      self.cur_item.partial_cmp(&other.cur_item)
    } else {
      self.sub_plan.partial_cmp(&other.sub_plan)
    }
}
}
impl PartialEq for ListPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}
impl<R: Rom> Plan<R> for ListPlan<R> {
  type Value = ();

  fn save(&self) -> PlanState {
    PlanState::ListState(ListPlanState {
      cur_item: self.cur_item,
      sub_plan: if self.cur_item >= self.plans.len() { None } else { Some(Rc::new(self.plans[self.cur_item].save())) },
    })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::ListState(ListPlanState { cur_item, sub_plan }) = state {
      assert!(*cur_item < self.plans.len());
      self.cur_item = *cur_item;
      if let Some(sub_plan) = sub_plan {
        self.plans[*cur_item].restore(sub_plan);
        self.cur_item_is_initialized = true;
      } else {
        self.cur_item_is_initialized = false;
      }
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.cur_item = 0;
    self.initialize_cur_item(gb, state);
  }
  fn is_safe(&self) -> bool {
    self.cur_item >= self.plans.len() || self.plans[self.cur_item].is_safe()
  }
  fn get_blockable_inputs(&self) -> Input {
    if self.cur_item < self.plans.len() { self.plans[self.cur_item].get_blockable_inputs() } else { Input::all() }
  }
  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    assert!(self.cur_item_is_initialized);
    self.plans[self.cur_item].canonicalize_input(input)
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<()>)> {
    assert!(self.cur_item < self.plans.len());
    self.ensure_cur_item_initialized(gb, state);
    if let Some((new_state, result)) = self.plans[self.cur_item].execute_input(gb, state, input) {
      if result.is_some() {
        self.cur_item += 1;
        if self.cur_item >= self.plans.len() {
          self.cur_item_is_initialized = false;
          return Some((new_state, Some(())));
        }
        self.initialize_cur_item(gb, &new_state);
      }
      Some((new_state, None))
    } else { None }
  }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeqPlanState {
  p_done: bool,
  sub_plan: Rc<PlanState>,
}
impl PartialOrd for SeqPlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.p_done != other.p_done {
      self.p_done.partial_cmp(&other.p_done)
    } else {
      self.sub_plan.partial_cmp(&other.sub_plan)
    }
}
}
impl PartialEq for SeqPlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}
pub struct SeqPlan<P, Q> {
  p: P,
  q: Q,
  p_done: bool,
}
impl<P, Q> SeqPlan<P, Q> {
  pub fn new(p: P, q: Q) -> Self {
    Self { p, q, p_done: false, }
  }
}
impl<R: Rom,P: Plan<R, Value=()>, Q: Plan<R>> Plan<R> for SeqPlan<P, Q> {
  type Value = Q::Value;

  fn save(&self) -> PlanState {
    PlanState::SeqState(SeqPlanState {
      p_done: self.p_done,
      sub_plan: Rc::new(if self.p_done { self.q.save() } else { self.p.save() }),
    })
  }
  fn restore(&mut self, state: &PlanState) {
    if let PlanState::SeqState(SeqPlanState { p_done, sub_plan }) = state {
      self.p_done = *p_done;
      if self.p_done { self.q.restore(sub_plan) } else { self.p.restore(sub_plan) };
    } else { panic!("Loading incompatible plan state {:?}", state); }
  }
  fn initialize(&mut self, gb: &mut Gb<R>, state: &GbState) {
    self.p_done = false;
    self.p.initialize(gb, state);
  }
  fn is_safe(&self) -> bool {
    if self.p_done { self.q.is_safe() } else { self.p.is_safe() }
  }
  fn get_blockable_inputs(&self) -> Input {
    if self.p_done { self.q.get_blockable_inputs() } else { self.p.get_blockable_inputs() }
  }
  fn canonicalize_input(&self, input: Input) -> Option<Input> {
    if self.p_done { self.q.canonicalize_input(input) } else { self.p.canonicalize_input(input) }
  }
  fn execute_input(&mut self, gb: &mut Gb<R>, state: &GbState, input: Input) -> Option<(GbState, Option<Q::Value>)> {
    if self.p_done {
      self.q.execute_input(gb, state, input)
    } else {
      if let Some((new_state, result)) = self.p.execute_input(gb, state, input) {
        if result.is_some() {
          self.p_done = true;
          self.q.initialize(gb, &new_state);
        }
        Some((new_state, None))
      } else { None }
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PlanState {
  NullState,
  BattleMenuState(BattleMenuPlanState),
  BikeShopMenuState(BikeShopMenuPlanState),
  BillsPCMenuState(BillsPCMenuPlanState),
  BuySellQuitMenuState(BuySellQuitMenuPlanState),
  ChangeOptionsState(ChangeOptionsPlanState),
  ChooseQuantityMenuState(ChooseQuantityMenuPlanState),
  DepositWithdrawMenuState(DepositWithdrawMenuPlanState),
  EdgeWarpState(EdgeWarpPlanState),
  EvolutionState(EvolutionPlanState),
  FightKOState(FightKOPlanState),
  FightKOITState(FightKOITPlanState),
  FightTurnState(FightTurnPlanState),
  FightTurnITState(FightTurnITPlanState),
  FlyToState(FlyToPlanState),
  HoldTextDisplayOpenState(HoldTextDisplayOpenPlanState),
  IdentifyInputState(IdentifyInputPlanState),
  IntroNameMenuState(IntroNameMenuPlanState),
  ItemUseTossMenuState(ItemUseTossMenuPlanState),
  ListState(ListPlanState),
  ListMenuState(ListMenuPlanState),
  MainMenuState(MainMenuPlanState),
  NamingScreenState(NamingScreenPlanState),
  OverrideMoveMenuState(OverrideMoveMenuPlanState),
  OverworldEncounterState(OverworldEncounterPlanState),
  OverworldInteractState(OverworldInteractPlanState),
  OverworldJumpLedgeState(OverworldJumpLedgePlanState),
  OverworldOpenStartMenuState(OverworldOpenStartMenuPlanState),
  OverworldPushBoulderState(OverworldPushBoulderPlanState),
  OverworldTurnState(OverworldTurnPlanState),
  OverworldWaitState(OverworldWaitPlanState),
  PartyMenuState(PartyMenuPlanState),
  PartyMonMenuState(PartyMonMenuPlanState),
  PCMainMenuState(PCMainMenuPlanState),
  SelectMoveMenuState(SelectMoveMenuPlanState),
  SeqState(SeqPlanState),
  SkipIntroState(SkipIntroPlanState),
  SkipTextsState(SkipTextsPlanState),
  SkipTextsITState(SkipTextsITPlanState),
  SkipYellowIntroState(SkipYellowIntroPlanState),
  SkipYellowTitleState(SkipYellowTitlePlanState),
  StartMenuState(StartMenuPlanState),
  StartMenuCloseState(StartMenuClosePlanState),
  TextState(TextPlanState),
  TextCommandPauseState(TextCommandPausePlanState),
  TextScrollWaitState(TextScrollWaitPlanState),
  TwoOptionMenuState(TwoOptionMenuPlanState),
  VendingMachineMenuState(VendingMachineMenuPlanState),
  WalkToState(WalkToPlanState),
  WalkToEncounterState(WalkToEncounterPlanState),
}
impl PartialEq for PlanState {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

impl PartialOrd for PlanState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self {
      PlanState::NullState => if let PlanState::NullState = other { Some(Ordering::Equal) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::BattleMenuState(state) => if let PlanState::BattleMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::BikeShopMenuState(state) => if let PlanState::BikeShopMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::BillsPCMenuState(state) => if let PlanState::BillsPCMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::BuySellQuitMenuState(state) => if let PlanState::BuySellQuitMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::ChangeOptionsState(state) => if let PlanState::ChangeOptionsState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::ChooseQuantityMenuState(state) => if let PlanState::ChooseQuantityMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::DepositWithdrawMenuState(state) => if let PlanState::DepositWithdrawMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::EdgeWarpState(state) => if let PlanState::EdgeWarpState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::EvolutionState(state) => if let PlanState::EvolutionState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::FightKOState(state) => if let PlanState::FightKOState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::FightKOITState(state) => if let PlanState::FightKOITState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::FightTurnState(state) => if let PlanState::FightTurnState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::FightTurnITState(state) => if let PlanState::FightTurnITState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::FlyToState(state) => if let PlanState::FlyToState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::HoldTextDisplayOpenState(state) => if let PlanState::HoldTextDisplayOpenState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::IdentifyInputState(state) => if let PlanState::IdentifyInputState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::ItemUseTossMenuState(state) => if let PlanState::ItemUseTossMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::ListState(state) => if let PlanState::ListState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::ListMenuState(state) => if let PlanState::ListMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::IntroNameMenuState(state) => if let PlanState::IntroNameMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::MainMenuState(state) => if let PlanState::MainMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::NamingScreenState(state) => if let PlanState::NamingScreenState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::OverrideMoveMenuState(state) => if let PlanState::OverrideMoveMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::OverworldEncounterState(state) => if let PlanState::OverworldEncounterState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::OverworldInteractState(state) => if let PlanState::OverworldInteractState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::OverworldJumpLedgeState(state) => if let PlanState::OverworldJumpLedgeState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::OverworldOpenStartMenuState(state) => if let PlanState::OverworldOpenStartMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::OverworldPushBoulderState(state) => if let PlanState::OverworldPushBoulderState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::OverworldTurnState(state) => if let PlanState::OverworldTurnState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::OverworldWaitState(state) => if let PlanState::OverworldWaitState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::PartyMenuState(state) => if let PlanState::PartyMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::PartyMonMenuState(state) => if let PlanState::PartyMonMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::PCMainMenuState(state) => if let PlanState::PCMainMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::SelectMoveMenuState(state) => if let PlanState::SelectMoveMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::SeqState(state) => if let PlanState::SeqState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::SkipIntroState(state) => if let PlanState::SkipIntroState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::SkipTextsState(state) => if let PlanState::SkipTextsState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::SkipTextsITState(state) => if let PlanState::SkipTextsITState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::SkipYellowIntroState(state) => if let PlanState::SkipYellowIntroState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::SkipYellowTitleState(state) => if let PlanState::SkipYellowTitleState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::StartMenuState(state) => if let PlanState::StartMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::StartMenuCloseState(state) => if let PlanState::StartMenuCloseState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::TextState(state) => if let PlanState::TextState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::TextCommandPauseState(state) => if let PlanState::TextCommandPauseState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::TextScrollWaitState(state) => if let PlanState::TextScrollWaitState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::TwoOptionMenuState(state) => if let PlanState::TwoOptionMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::VendingMachineMenuState(state) => if let PlanState::VendingMachineMenuState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::WalkToState(state) => if let PlanState::WalkToState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
      PlanState::WalkToEncounterState(state) => if let PlanState::WalkToEncounterState(other_state) = other { state.partial_cmp(other_state) } else { panic!("Comparing invalid plan states {:?} and {:?}", self, other); },
    }
  }
}

mod battlemenu;
pub use battlemenu::*;
mod bikeshopmenu;
pub use bikeshopmenu::*;
mod billspcmenu;
pub use billspcmenu::*;
mod buysellquitmenu;
pub use buysellquitmenu::*;
mod changeoptions;
pub use changeoptions::*;
mod choosequantitymenu;
pub use choosequantitymenu::*;
mod depositwithdrawmenu;
pub use depositwithdrawmenu::*;
mod edgewarp;
pub use edgewarp::*;
mod endtrainerbattle;
pub use endtrainerbattle::*;
mod evolution;
pub use evolution::*;
mod fightko;
pub use fightko::*;
mod fightkoit;
pub use fightkoit::*;
mod fightturn;
pub use fightturn::*;
mod fightturnit;
pub use fightturnit::*;
mod flyto;
pub use flyto::*;
mod holdtextdisplayopen;
pub use holdtextdisplayopen::*;
mod identifyinput;
pub use identifyinput::*;
mod intronamemenu;
pub use intronamemenu::*;
mod itemusetossmenu;
pub use itemusetossmenu::*;
mod joypad;
pub use joypad::*;
mod listmenu;
pub use listmenu::*;
mod mainmenu;
pub use mainmenu::*;
mod namingscreen;
pub use namingscreen::*;
mod nexttrainermon;
pub use nexttrainermon::*;
mod overridemove;
pub use overridemove::*;
mod overworldencounter;
pub use overworldencounter::*;
mod overworldinteract;
pub use overworldinteract::*;
mod overworldjumpledge;
pub use overworldjumpledge::*;
mod overworldopenstartmenu;
pub use overworldopenstartmenu::*;
mod overworldpushboulder;
pub use overworldpushboulder::*;
mod overworldturn;
pub use overworldturn::*;
mod overworldwait;
pub use overworldwait::*;
mod partymenu;
pub use partymenu::*;
mod partymonmenu;
pub use partymonmenu::*;
mod pcmainmenu;
pub use pcmainmenu::*;
mod selectmovemenu;
pub use selectmovemenu::*;
mod skipintro;
pub use skipintro::*;
mod skiptexts;
pub use skiptexts::*;
mod skiptextsit;
pub use skiptextsit::*;
mod skipyellowintro;
pub use skipyellowintro::*;
mod skipyellowtitle;
pub use skipyellowtitle::*;
mod startmenu;
pub use startmenu::*;
mod starttrainerbattle;
pub use starttrainerbattle::*;
mod text;
pub use text::*;
mod textcommandpause;
pub use textcommandpause::*;
mod textscrollwait;
pub use textscrollwait::*;
mod twooptionmenu;
pub use twooptionmenu::*;
mod vendingmachinemenu;
pub use vendingmachinemenu::*;
mod walkto;
pub use walkto::*;
mod walktoencounter;
pub use walktoencounter::*;
