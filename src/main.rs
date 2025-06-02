mod board;
mod game;
mod renderer;
mod system;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    game::start_game();
    Ok(())
}
