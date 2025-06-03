mod board;
mod game;
mod game_state;
mod renderer;
mod system;

use std::error::Error;

use game_state::GameState;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments

    let game_state = GameState::new(
        7, 7, 15, // as a percentage
        3, true,
    );
    game::start_game(game_state);
    Ok(())
}
