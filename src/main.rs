mod board;
mod direction;
mod game;
mod renderer;
mod solver;
mod visual_solver;

use board::Board;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let level_data = read_level_data("pokemon")?;
    // let mut board = Board::from_level(level_data);
    let mut board = Board::solvable_random(15, 15, 15, 10);
    let arg_opt = env::args().last();

    if arg_opt.map(|arg| arg == "game").unwrap_or_else(|| false) {
        game::start_game(&mut board)
    } else {
        // let rc_board = Rc::new(RefCell::new(board));
        // let mut solver = Solver::new(rc_board);
        // solver.solve();
        // Ok(())
        visual_solver::render_steps()
    }
}
