mod board;
mod game;
mod game_state;
mod parser;
mod renderer;
mod system;

use std::error::Error;

use clap::Parser;
use game_state::GameState;
use parser::Args;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();

    let game_state = GameState::new(
        cli.columns,
        cli.rows,
        cli.rock_percentage,
        cli.moves_required,
        !cli.full_level_view,
        cli.debug_mode,
    );
    game::start_game(game_state);
    Ok(())
}
