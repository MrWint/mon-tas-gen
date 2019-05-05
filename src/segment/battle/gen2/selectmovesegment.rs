use crate::rom::*;
use crate::segment::*;
use crate::segment::battle::*;
use crate::statebuffer::StateBuffer;
use gambatte::Input;

pub struct SelectMoveSegment {
  mov: Move,
  buffer_size: usize,
}
impl SelectMoveSegment {
  #[allow(dead_code)]
  pub fn new(mov: Move) -> Self {
    Self {
      mov,
      buffer_size: crate::statebuffer::STATE_BUFFER_DEFAULT_MAX_SIZE,
    }
  }
}
impl WithOutputBufferSize for SelectMoveSegment {
  fn with_buffer_size(self, buffer_size: usize) -> Self { Self { buffer_size, ..self } }
}

impl<R: Rom + Gen2FightTurnAddresses + BattleMovesInfoAddresses + BattleMonInfoAddresses> Segment<R> for SelectMoveSegment {
  type Key = ();

  fn execute_split(&self, gbe: &mut RuntimeGbExecutor<R>, sb: StateBuffer) -> HashMap<Self::Key, StateBuffer> {
    let sb = MoveSegment::new(Input::A).with_buffer_size(self.buffer_size).execute(gbe, sb); // Fight

    let move_infos = gbe.execute_state(&sb, MoveInfosFn::new(Who::Player)).get_value_assert_all_equal();
    let move_index = move_infos.iter().position(|move_info| move_info.mov == self.mov).expect("move not found");
    let num_moves = move_infos.len();
    gbe.execute(sb, move |gb, s, tx| {
      gb.restore(&s);
      let cursor_index = usize::from(gb.gb.read_memory(R::CUR_MOVE_INDEX_MEM_ADDRESS));
      let num_up_moves = (cursor_index + num_moves - move_index) % num_moves;
      let num_down_moves = (move_index + num_moves - cursor_index) % num_moves;
      if cursor_index == move_index {
        gb.input(Input::empty());
      } else if num_up_moves < num_down_moves {
        assert!(num_up_moves == 1);
        gb.input(Input::UP);
      } else if num_down_moves == 2 {
        gb.input(Input::DOWN);
        gb.step();
        gb.input(Input::DOWN);
      } else {
        assert!(num_down_moves == 1);
        gb.input(Input::DOWN);
      }
      gb.step();
      tx.send(gb.save()).unwrap();
    }).into_state_buffer_map(self.buffer_size)
  }
}
