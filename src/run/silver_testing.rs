use crate::run::*;
use gambatte::inputs::*;
use montas::constants::*;
use montas::segment::battle::*;
use montas::segment::battle::gen2::*;
use montas::segment::overworld::gen2::*;

#[allow(dead_code)]
pub fn start() {
  let mut r: GbRunner<Silver, _> = GbRunner::pool_with_screen();

  run(&mut r);

  r.run(IdentifyInputSegment::new());

  r.debug_segment_end("temp/silver_test");
}

fn run<G: GbExecutor<Silver>>(r: &mut GbRunner<Silver, G>) {
  r.run(DelaySegment::new(MoveSegment::with_metric_fn(A, |_gb| Some(()))));
  r.debug_print_states();
  r.run(MoveSegment::new(START).with_max_skips(10));
  r.debug_print_states();
  r.run(MoveSegment::new(D)); // options
  r.run(MoveSegment::new(L|A)); // fast options
  r.run(MoveSegment::new(B)); // back
  r.run(MoveSegment::new(A)); // new game
  r.debug_print_states();
  r.run(SkipTextsSegment::new(3));
  r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // time: 10:..
  r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // what // 10 oclock // ?
  r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // ? // How many minutes?
  r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // whoa // 00 min // ?
  r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // day // overslept
  r.run(SkipTextsSegment::new(17)); // oak speech
  r.run(MoveSegment::new(D)); // Name: Silver
  r.run(MoveSegment::new(A)); // Name: Silver
  r.run(SkipTextsSegment::new(7)); // skip texts until game start
  r.run(TextSegment::new()); // ... seeing you later
  r.save("silver_test");

  r.load("silver_test");
  r.run(TurnSegment::new(R));
  r.run_debug(WalkToSegment::new(7, 0).into(OverworldInteractionResult::Warped));
  r.run(WarpSegment::new());
  r.run(SkipScriptSegment::new());
  r.run(SkipTextsSegment::new(7)); // mom speech
  r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // silver // received // PokeGear // . // Pokemon Gear
  r.run(SkipTextsSegment::new(4)); // mom speech
  r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose Sunday
  r.run(SkipTextsSegment::new(1).with_confirm_input(A).with_skip_ends(1)); // Sunday // is it?
  r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // no DST
  r.run(SkipTextsSegment::new(1).with_confirm_input(A).with_skip_ends(1)); // 10:00 AM // confirm time
  r.run(SkipTextsSegment::new(3)); // mom speech
  r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // know phone
  r.run(SkipTextsSegment::new(5)); // mom speech
  r.run(SkipScriptSegment::new());
  r.run(WalkToSegment::new(7, 7));
  r.run(WarpSegment::new().with_input(D)); r.debug_print_states();
  r.save("silver_left_house");

  r.load("silver_left_house");
  r.run(WalkToSegment::new(6, 3).into(OverworldInteractionResult::Warped));
  r.run(WarpSegment::new()); r.debug_print_states();
  r.run(SkipScriptSegment::new()); r.debug_print_states();
  r.run(SkipTextsSegment::new(16)); // elm speech
  r.run(TurnSegment::new(D)); r.debug_print_states();
  r.run(WalkToSegment::new(7, 4));
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v == &OverworldInteractionResult::NoEvents}).into_unit())); r.debug_print_states();
  r.run(TurnSegment::new(U)); r.debug_print_states();
  r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); if let OverworldInteractionResult::Interact(_) = v { true } else { false }}).into_unit())); r.debug_print_states();
  r.run(MoveSegment::new(B)); // close picture
  r.run(SkipTextsSegment::new(1)); // choose Totodile
  r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose Totodile
  r.save("silver_choose_starter");

  r.load("silver_choose_starter");
  r.run(SkipTextsSegment::new(2).with_buffer_size(8192)); r.debug_print_states(); // elm speech
  r.run(TextSegment::new().with_skip_ends(2).with_buffer_size(8192)); r.debug_print_states(); // Player received // Totodile // !
  r.save("silver_choose_starter_unbounded");

  r.load("silver_choose_starter_unbounded");
  r.run_debug(DelaySegment::new(MoveSegment::with_metric(A | B, Gen2DVMetric {}.filter(|v| {
    if v.atk < 15 || /*v.spc < 15 ||*/ v.spd < 15 { return false; }
    log::debug!("Chosen DVs: {:?}", v); true
  }).into_unit()))); r.debug_print_states();
  r.save("silver_after_choose_starter");

  r.load("silver_after_choose_starter");
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); r.debug_print_states(); // nickname to // Totodile // you
  r.run(SkipTextsSegment::new(1).with_confirm_input(B)); r.debug_print_states(); // no nickname
  r.run(SkipScriptSegment::new()); r.debug_print_states();
  r.run(SkipTextsSegment::new(9)); // elm speech
  r.run(TurnSegment::new(D)); r.debug_print_states();
  r.run(WalkToSegment::new(4, 7));
  r.run(WalkStepSegment::new(D).into(OverworldInteractionResult::MapCoordEvent)); r.debug_print_states();
  r.run(SkipScriptSegment::new()); r.debug_print_states();
  r.run(SkipTextsSegment::new(2)); // aide speech
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player received // potion // .
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // potion // in
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // item pocket // .
  r.run(SkipTextsSegment::new(2)); // aide speech
  r.run(SkipScriptSegment::new()); r.debug_print_states();
  r.run(WalkToSegment::new(4, 11));
  r.run(WarpSegment::new().with_input(D)); r.debug_print_states();
  r.save("silver_test_after_elm");

  r.load("silver_test_after_elm");
  r.run(WalkToSegment::new(-1, 8).into(OverworldInteractionResult::MapConnection));
  r.run(MoveSegment::new(NIL)); r.debug_print_states(); // MapConnection
  r.run(WalkToSegment::new(9, 6));
  r.run(JumpLedgeSegment::new(L)); r.debug_print_states();
  r.run(WalkToSegment::new(-1, 7).into(OverworldInteractionResult::MapConnection));
  r.run(MoveSegment::new(NIL)); r.debug_print_states(); // MapConnection
  r.run(WalkToSegment::new(17, -1).into(OverworldInteractionResult::MapConnection));
  r.run(MoveSegment::new(NIL)); r.debug_print_states(); // MapConnection
  r.run(WalkToSegment::new(17, 5).into(OverworldInteractionResult::Warped));
  r.run(WarpSegment::new()); r.debug_print_states();
  r.save("silver_test_entered_mr_pokemon_house");

  r.load("silver_test_entered_mr_pokemon_house");
  r.run(SkipScriptSegment::new()); r.debug_print_states();
  r.run(SkipTextsSegment::new(2)); // Mr.Pokemon speech
  r.run(SkipScriptSegment::new()); r.debug_print_states();
  r.run(SkipTextsSegment::new(2)); // Mr.Pokemon speech
  r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // // // put // mystery egg // in
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // key poket // .
  r.run(SkipTextsSegment::new(10)); // Mr.Pokemon speech
  r.run(SkipScriptSegment::new()); r.debug_print_states();
  r.run(SkipTextsSegment::new(23)); // Oak speech
  r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // got pokedex // speech
  r.run(SkipTextsSegment::new(5)); // Oak speech
  r.run(SkipScriptSegment::new()); r.debug_print_states();
  r.run(SkipTextsSegment::new(4)); // Mr.Pokemon speech
  r.run(TurnSegment::new(D)); r.debug_print_states();
  r.run(WalkStepSegment::new(D)); r.debug_print_states();
  r.run(WarpSegment::new().with_input(D)); r.debug_print_states();
  r.save("silver_test_after_mr_pokemon_house");

  r.load("silver_test_after_mr_pokemon_house");
  r.run(MoveLoopSegment::new(OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v != &OverworldInteractionResult::CountStepEvent}).into_unit())); r.debug_print_states();
  r.run(SkipTextsSegment::new(4)); // Elm phone call
  r.run(TextSegment::new().with_skip_ends(6)); // Click // ... // ... // ...
  r.run(WalkToSegment::new(7, 54).into(OverworldInteractionResult::MapConnection));
  r.run(MoveSegment::new(NIL)); r.debug_print_states(); // MapConnection
  r.run(WalkToSegment::new(33, 7).into(OverworldInteractionResult::MapCoordEvent));
  r.run(SkipScriptSegment::new()); r.debug_print_states();
  r.save("silver_test_before_rival1");

  r.load("silver_test_before_rival1");
  r.run(SkipTextsSegment::new(7)); // pre-battle texts
  r.run(SkipTextsSegment::new(2)); // trainer wants to battle // trainer sent out
  r.run_debug(TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Growl)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5)); // chikorita // ! // Go // Totodile // !
  // println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  // println!("Player: {:#?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  // println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  // println!("Enemy: {:#?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // Turn 1
  r.run(MoveSegment::new(A)); // Fight
  r.run(MoveSegment::new(NIL)); // neutral
  r.run_debug(
    DelaySegment::new(
      MoveSegment::with_metric(A,
          BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst)
          .and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
      .seq(TextSegment::with_metric(
          Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(FightTurnResult::CriticalHit { damage: 8, }))
          .with_skip_ends(3).with_unbounded_buffer())
      )); // Scratch //// mon // used // move // !
  r.run(TextSegment::new()); // critical hit!
  r.run(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey))); // confirm
  r.run_debug(TextSegment::with_metric(Gen2StatUpDownMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer()); // mon // used // move // !
  r.run(TextSegment::new().with_allowed_end_inputs(A)); // but it failed!
  r.run(DelaySegment::new(MoveSegment::with_metric(B, Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Growl)))); // confirm
  // Turn 2
  r.run(MoveSegment::new(A)); // Fight
  r.run(MoveSegment::new(NIL)); // neutral
  r.run_debug(
    DelaySegment::new(
      MoveSegment::with_metric(A,
          BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst)
          .and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))
      .seq(TextSegment::with_metric(
          Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(FightTurnResult::CriticalHit { damage: 8, }))
          .with_skip_ends(3).with_unbounded_buffer())
      )); // Scratch //// mon // used // move // !
  r.run(TextSegment::new()); // critical hit!
  r.run_debug(
    DelaySegment::new(
      MoveSegment::with_metric(A|B,
          BattleObedienceMetric {}.expect(BattleObedience::Obey))
      .seq(TextSegment::with_metric(
          Gen2StatUpDownMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer())
      )); // confirm //// mon // used // move // !
  r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  // Turn 3
  r.run(MoveSegment::new(A)); // Fight
  r.run(MoveSegment::new(NIL)); // neutral
  r.run_debug(MoveSegment::with_metric(A, BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst)
      .and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))); // Scratch
  r.run_debug(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer()); // mon // used // move // !
  r.save("silver_test_rival1_defeated");

  r.load("silver_test_rival1_defeated");
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // enemy // mon // fainted
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // gained // num XP
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // mon // grew to level // X
  r.run(SkipTextsSegment::new(1)); // ??? was defeated
  r.run(SkipTextsSegment::new(1)); // defeat text
  r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // player got // X for winning

  r.run(SkipScriptSegment::new());
  r.run(SkipTextsSegment::new(5)); // ... ... ... // name is ??? // world's greatest // mon // trainer
  r.run(SkipScriptSegment::new());
  r.run(TurnSegment::new(U));
  r.run_debug(WalkToSegment::new(40, 7).into(OverworldInteractionResult::MapConnection));


  // r.run_debug(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit()));
}