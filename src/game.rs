use crate::board::Board;
use crate::direction::Move;
use crate::renderer::Renderer;
use crossterm::event::KeyCode;
use std::cell::RefCell;
use std::error::Error;
use std::iter;
use std::rc::Rc;

pub fn start_game(board: &mut Board) -> Result<(), Box<dyn Error>> {
    board.enable_debug_mode();
    let rc_board = Rc::new(RefCell::new(board));

    let render_callback = || rc_board.borrow().render_board();

    let input_handler = |key_code: KeyCode| {
        let mut board_mref = rc_board.borrow_mut();
        board_mref.respond_to_input(key_code)
    };

    let player_moves_iterator = iter::from_fn(|| {
        let board_ref = rc_board.clone();
        let mut board_mref = board_ref.borrow_mut();
        board_mref
            .move_queue
            .pop_front()
            .map(|curr_move| match curr_move {
                Move::Direction(direction) => board_mref.move_player(direction),
                Move::Reset => board_mref.reset(),
            })
    });

    let mut renderer = Renderer::new(render_callback, input_handler, player_moves_iterator, 50);

    renderer.render_scene();

    Ok(())
}
