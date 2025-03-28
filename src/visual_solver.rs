use crate::board::level_reader::read_level_data;
use crate::board::Board;
use crate::renderer::Renderer;
use crate::solver::Solver;
use crossterm::event::KeyCode;
use std::cell::RefCell;
use std::error::Error;
use std::iter;
use std::rc::Rc;

pub fn render_steps() -> Result<(), Box<dyn Error>> {
    let level_data = read_level_data("pokemon")?;
    let board = Board::from_level(level_data);
    let rc_board = Rc::new(RefCell::new(board));

    let render_callback = || {
        let mut output = rc_board.borrow().render_board();
        let added = format!(
            "Move Queue Length: {:?}",
            rc_board.borrow().move_queue.len()
        );
        output.push(added);
        output
    };

    let input_handler = |_: KeyCode| {};

    let player_moves_iterator = iter::from_fn(|| {
        let board_ref = rc_board.clone();
        let mut board_mref = board_ref.borrow_mut();
        board_mref.process_move()
    });

    let mut solver = Solver::new(rc_board.clone());
    // Enabling visual mode means the solver saved the moves it used to find the solutions.
    solver.enable_visual_mode();
    solver.solve();
    // Load the board with the solver moves.
    rc_board.borrow_mut().move_queue = solver.move_record;

    let frame_delay = 100;
    let mut renderer = Renderer::new(
        render_callback,
        input_handler,
        player_moves_iterator,
        frame_delay,
    );

    renderer.render_scene();

    Ok(())
}
