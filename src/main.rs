mod board;
mod direction;
mod renderer;
mod solver;

use board::{Board, Point};
use renderer::Renderer;
use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

fn main() {
    let board: Board = Board::new(
        5,
        5,
        Point { row: 1, col: 0 },
        Point { row: 4, col: 3 },
        Vec::<Point>::with_capacity(9),
    );
    let rc_board = Rc::new(RefCell::new(board));

    let render_callback = || rc_board.borrow().render_board();

    let input_handler = |c: char| {
        let mut board_mref = rc_board.borrow_mut();
        board_mref.respond_to_input(c)
    };

    let player_moves_iterator = iter::from_fn(|| {
        let board_ref = rc_board.clone();
        let mut board_mref = board_ref.borrow_mut();
        board_mref.move_queue.pop_front().map(|direction| {
            board_mref.move_player(direction);
        })
    });

    let mut renderer = Renderer::new(render_callback, input_handler, player_moves_iterator, 50);

    renderer.render_scene();
}
