use crate::run::*;
use gambatte::inputs::*;
use montas::constants::*;
use montas::segment::battle::*;
use montas::segment::battle::gen2::*;
use montas::segment::overworld::gen2::*;

#[allow(dead_code)]
pub fn start() {
  let mut r: GbRunner<Crystal, _> = GbRunner::pool_with_screen();

  run(&mut r);

  r.run(IdentifyInputSegment::new());

  r.debug_segment_end("temp/crystal_glitchless");
}

fn run<G: GbExecutor<Crystal>>(r: &mut GbRunner<Crystal, G>) {
  r.run(MoveSegment::new(A));
  r.run(MoveSegment::new(START));
  r.run(MoveSegment::new(D)); // options
  r.run(MoveSegment::new(L|A)); // fast options
  r.run(MoveSegment::new(B)); // back
  r.run(MoveSegment::new(A)); // new game
  r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose Boy
  r.run(SkipTextsSegment::new(3));
  r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // time: 10:..
  r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // what // 10 oclock // ?
  r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // ? // How many minutes?
  r.run(SkipTextsSegment::new(1).with_skip_ends(2).with_confirm_input(A)); // whoa // 00 min // ?
  r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // day // overslept
  r.run(SkipTextsSegment::new(16)); // oak speech
  r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // oak speech
  { // Name: J (+50 frames)
    r.run(MoveSegment::new(A)); // custom name
    r.run(MoveSegment::new(D)); // down to J
    r.run(MoveSegment::new(A)); // input J
    r.run(MoveSegment::new(START)); // go to end
    r.run(MoveSegment::new(A)); // confirm
    r.run(VerifyInputSegment::new("NamingScreenJoypadLoop")); // closing takes extra loop though input
  }
  // { // Name: Chris
  //   r.run(MoveSegment::new(D)); // Name: Chris
  //   r.run(MoveSegment::new(A)); // Name: Chris
  // }
  r.run(SkipTextsSegment::new(7)); // skip texts until game start
  r.debug_print_states();
  r.run(TextSegment::new()); // ... seeing you later
  r.debug_print_states();
  r.save("crystal_test");

  r.load("crystal_test");
  r.run(TurnSegment::new(R));
  r.run(WalkToSegment::new(7, 0).into(OverworldInteractionResult::Warped));
  r.run(WarpSegment::new());
  r.run(WalkToSegment::new(7, 2));
  r.run(WalkStepSegment::new(D));
  r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  r.debug_print_states();
  r.run(SkipTextsSegment::new(7)); // mom speech
  r.run(SkipTextsSegment::new(1).with_skip_ends(3)); // received // PokeGear // . // Pokemon Gear
  r.run(SkipTextsSegment::new(4)); // mom speech
  r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose Sunday
  r.run(SkipTextsSegment::new(1).with_confirm_input(A).with_skip_ends(1)); // Sunday // is it?
  r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // no DST
  r.run(SkipTextsSegment::new(1).with_confirm_input(A).with_skip_ends(1)); // 10:00 AM // confirm time
  r.run(SkipTextsSegment::new(3)); // mom speech
  r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // know phone
  r.run(SkipTextsSegment::new(5)); // mom speech
  r.run(TurnSegment::new(R));
  r.debug_print_states();
  r.run(WalkToSegment::new(7, 7));
  r.run(WarpSegment::new().with_input(D));
  r.debug_print_states();
  r.save("crystal_left_house");

  r.load("crystal_left_house");
  r.run(WalkToSegment::new(6, 3).into(OverworldInteractionResult::Warped));
  r.run(WarpSegment::new());
  r.debug_print_states();
  r.run(SkipScriptSegment::new());
  r.debug_print_states();
  r.run(SkipTextsSegment::new(12)); // elm speech
  r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose to help
  r.run(SkipTextsSegment::new(6)); // elm speech
  r.run(SkipScriptSegment::new());
  r.debug_print_states();
  r.run(SkipTextsSegment::new(15)); // elm speech
  r.run(SkipScriptSegment::new());
  r.debug_print_states();
  r.run(SkipTextsSegment::new(5)); // elm speech
  r.run(WalkToSegment::new(7, 4));
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::NoEvents)));
  r.debug_print_states();
  r.run(TurnSegment::new(U));
  r.debug_print_states();
  r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.debug_print().expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  r.debug_print_states();
  r.run(MoveSegment::new(B)); // close picture
  r.run(SkipTextsSegment::new(1)); // choose Totodile
  r.run(SkipTextsSegment::new(1).with_confirm_input(A)); // choose Totodile
  r.save("crystal_choose_starter");

  r.load("crystal_choose_starter");
  r.run(SkipTextsSegment::new(2).with_buffer_size(8192)); // elm speech
  r.debug_print_states();
  r.run(TextSegment::new().with_skip_ends(2).with_buffer_size(8192)); // Player received // Totodile // !
  r.debug_print_states();
  r.save("crystal_choose_starter_unbounded");

  r.load("crystal_choose_starter_unbounded");
  r.run_debug(DelaySegment::new(MoveSegment::with_metric(A | B, Gen2DVMetric {}.filter(|v| {
    if v.atk < 15 || v.def < 15 || v.spc < 15 || v.spd < 15 { return false; }
    log::debug!("Chosen DVs: {:?}", v); true
  }).into_unit())));
  r.debug_print_states();
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // nickname to // Totodile // you
  r.debug_print_states();
  r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // no nickname
  r.debug_print_states();
  r.run(SkipScriptSegment::new());
  r.debug_print_states();
  r.run(SkipTextsSegment::new(11)); // elm speech
  r.run(TurnSegment::new(D));
  r.debug_print_states();
  r.run(WalkToSegment::new(4, 7));
  r.run(WalkStepSegment::new(D).into(OverworldInteractionResult::MapCoordEvent));
  r.debug_print_states();
  r.run(SkipScriptSegment::new());
  r.debug_print_states();
  r.run(SkipTextsSegment::new(2)); // aide speech
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player received // potion // .
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // potion // in
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // item pocket // .
  r.run(SkipTextsSegment::new(2)); // aide speech
  r.run(SkipScriptSegment::new());
  r.debug_print_states();
  r.run(WalkToSegment::new(4, 11));
  r.run(WarpSegment::new().with_input(D));
  r.debug_print_states();
  r.save("crystal_test_after_elm");

  r.load("crystal_test_after_elm");
  r.run(WalkToSegment::new(-1, 8).into(OverworldInteractionResult::MapConnection));
  r.run(MoveSegment::new(NIL)); // MapConnection
  r.debug_print_states();
  r.run(WalkToSegment::new(9, 6));
  r.run(JumpLedgeSegment::new(L));
  r.debug_print_states();
  r.run(WalkToSegment::new(-1, 7).into(OverworldInteractionResult::MapConnection));
  r.run(MoveSegment::new(NIL)); // MapConnection
  r.debug_print_states();
  r.run(WalkToSegment::new(17, -1).into(OverworldInteractionResult::MapConnection));
  r.run(MoveSegment::new(NIL)); // MapConnection
  r.debug_print_states();
  r.run(WalkToSegment::new(17, 5).into(OverworldInteractionResult::Warped));
  r.run(WarpSegment::new());
  r.debug_print_states();
  r.save("crystal_test_entered_mr_pokemon_house");

  r.load("crystal_test_entered_mr_pokemon_house");
  r.run(SkipScriptSegment::new());
  r.debug_print_states();
  r.run(SkipTextsSegment::new(2)); // Mr.Pokemon speech
  r.run(SkipScriptSegment::new());
  r.debug_print_states();
  r.run(SkipTextsSegment::new(2)); // Mr.Pokemon speech
  r.run(SkipTextsSegment::new(1).with_skip_ends(4)); // // // put // mystery egg // in
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // key poket // .
  r.run(SkipTextsSegment::new(10)); // Mr.Pokemon speech
  r.run(SkipScriptSegment::new());
  r.debug_print_states();
  r.run(SkipTextsSegment::new(23)); // Oak speech
  r.run(SkipTextsSegment::new(1).with_skip_ends(1)); // got pokedex // speech
  r.run(SkipTextsSegment::new(5)); // Oak speech
  r.run(SkipScriptSegment::new());
  r.debug_print_states();
  r.run(SkipTextsSegment::new(4)); // Mr.Pokemon speech
  r.run(TurnSegment::new(D));
  r.debug_print_states();
  r.run(WalkStepSegment::new(D));
  r.debug_print_states();
  r.run(WarpSegment::new().with_input(D));
  r.debug_print_states();
  r.save("crystal_test_after_mr_pokemon_house");

  r.load("crystal_test_after_mr_pokemon_house");
  r.run(MoveLoopSegment::new(OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v != &OverworldInteractionResult::CountStepEvent}).into_unit()));
  r.debug_print_states();
  r.run(SkipTextsSegment::new(4)); // Elm phone call
  r.run(TextSegment::new().with_skip_ends(6)); // Click // ... // ... // ...
  r.run(WalkToSegment::new(7, 54).into(OverworldInteractionResult::MapConnection));
  r.run(MoveSegment::new(NIL)); // MapConnection
  r.debug_print_states();
  r.run(WalkToSegment::new(33, 7).into(OverworldInteractionResult::MapCoordEvent));
  r.run(SkipScriptSegment::new());
  r.debug_print_states();
  r.save("crystal_test_before_rival1");

  r.load("crystal_test_before_rival1");
  r.run(SkipTextsSegment::new(7)); // pre-battle texts
  r.run(SkipTextsSegment::new(2)); // trainer wants to battle // trainer sent out
  r.run_debug(TextSegment::with_metric(Gen2AIChooseMoveMetric {}.debug_print().expect(Move::Growl)).with_allowed_end_inputs(Input::all() - A).with_skip_ends(5)); // chikorita // ! // Go // Totodile // !
  println!("Player: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Player)));
  println!("Player: {:#?}", r.get_state_metric(BattleMonInfoFn::new(Who::Player)));
  println!("Enemy: {:?}", r.get_state_metric(MoveInfosFn::new(Who::Enemy)));
  println!("Enemy: {:#?}", r.get_state_metric(BattleMonInfoFn::new(Who::Enemy)));
  // Turn 1
  r.run(MoveSegment::new(A)); // Fight
  r.run(MoveSegment::new(NIL)); // neutral
  r.run_debug(MoveSegment::with_metric(A, BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst)
      .and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))); // Scratch
  r.run_debug(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(FightTurnResult::CriticalHit { damage: 8, })).with_skip_ends(3).with_unbounded_buffer()); // mon // used // move // !
  r.run(TextSegment::new()); // critical hit!
  r.run(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey))); // confirm
  r.run_debug(TextSegment::with_metric(Gen2StatUpDownMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer()); // mon // used // move // !
  r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
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
  r.run(MoveSegment::with_metric(A|B, BattleObedienceMetric {}.expect(BattleObedience::Obey))); // confirm
  r.run_debug(TextSegment::with_metric(Gen2StatUpDownMetric {}.debug_print().expect(FightTurnResult::Failed)).with_skip_ends(3).with_unbounded_buffer()); // mon // used // move // !
  r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // but it failed!
  // Turn 3
  r.run(MoveSegment::new(A)); // Fight
  r.run(MoveSegment::new(NIL)); // neutral
  r.run_debug(MoveSegment::with_metric(A, BattleMoveOrderMetric {}.debug_print().expect(MoveOrder::PlayerFirst)
      .and_then(BattleObedienceMetric {}.debug_print().expect(BattleObedience::Obey)))); // Scratch
  r.run_debug(TextSegment::with_metric(Gen2NormalHitMetric::with_expected_max_damage(5, 8).debug_print().expect(FightTurnResult::Hit { damage: 5, })).with_skip_ends(3).with_unbounded_buffer()); // mon // used // move // !
  r.save("crystal_test_rival1_defeated");

  r.load("crystal_test_rival1_defeated");
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
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::MapConnection))); // MapConnection
  r.debug_print_states();
  r.run_debug(WalkToSegment::new(14, 12));
  r.run(JumpLedgeSegment::new(D));
  r.debug_print_states();
  r.run_debug(WalkToSegment::new(42, 9));
  r.run(JumpLedgeSegment::new(R));
  r.debug_print_states();
  r.run_debug(WalkToSegment::new(60, 9).into(OverworldInteractionResult::MapConnection));
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.expect(OverworldInteractionResult::MapConnection))); // MapConnection
  r.debug_print_states();
  r.run_debug(WalkToSegment::new(6, 3).into(OverworldInteractionResult::Warped));
  r.run(WarpSegment::new());
  r.debug_print_states();
  r.run_debug(WalkToSegment::new(4, 5).into(OverworldInteractionResult::MapCoordEvent));
  r.run(SkipScriptSegment::new());
  r.run(SkipTextsSegment::new(7)); // mon stolen
  r.run(SkipTextsSegment::new(1).with_confirm_input(B)); // mon stolen
  r.run(MoveSegment::new(R)); // B
  r.run(MoveSegment::new(A)); // B
  r.run(MoveSegment::new(START)); // confirm
  r.run(MoveSegment::new(A)); // confirm
  r.run(MoveSegment::new(B)); // ?
  r.run(SkipTextsSegment::new(2)); // so B was his name // thanks
  r.run(SkipScriptSegment::new());
  r.run(WalkStepSegment::new(R));
  r.run(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.filter(|v| {println!("{:?}", v); v == &OverworldInteractionResult::NoEvents}).into_unit()));
  r.debug_print_states();
  r.run(TurnSegment::new(U));
  r.debug_print_states();
  r.run(MoveSegment::with_metric(A, OverworldInteractionMetric {}.expect(OverworldInteractionResult::Interact(InteractType::ObjectScript))));
  r.debug_print_states();
  r.run(SkipTextsSegment::new(5)); // terrible // discovery // give egg
  r.run(SkipScriptSegment::new());
  r.run(SkipTextsSegment::new(1)); // this?
  r.run(SkipScriptSegment::new());
  r.run(SkipTextsSegment::new(21)); // egg // great discovery // what // incredible // potential
  r.run(TurnSegment::new(D));
  r.debug_print_states();
  r.run_debug(WalkToSegment::new(4, 7));
  r.run_debug(WalkToSegment::new(4, 8).into(OverworldInteractionResult::MapCoordEvent));
  r.run(SkipScriptSegment::new());
  r.run(SkipTextsSegment::new(2)); // take balls
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // received // balls // !
  r.run(SkipTextsSegment::new(4)); // take balls
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // player put // ball // in
  r.run(SkipTextsSegment::new(1).with_skip_ends(2)); // the // ball pocket // .
  r.run(SkipScriptSegment::new());
  r.run(WalkToSegment::new(4, 11));
  r.run(WarpSegment::new().with_input(D));
  r.debug_print_states();
  r.save("crystal_test_after_elm2");

  r.load("crystal_test_after_elm2");


  // r.run_debug(MoveSegment::with_metric(NIL, OverworldInteractionMetric {}.debug_print().into_unit()));
}