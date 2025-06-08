mod board;
mod game;
mod game_state;
mod parser;
mod renderer;
mod system;

use std::error::Error;

use clap::Parser;
use game_state::{GameConfig, GameState};
use parser::Args;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();

    // Default arguments derived from difficulty parameter
    // (can be overwritten by passing in additional arguments)
    let config = cli
        .difficulty
        .map(|x| GameConfig::get_config_from_difficulty(x.as_str()))
        .unwrap_or_else(|| GameConfig::default());

    let game_state = GameState::new(
        cli.columns.unwrap_or_else(|| config.cols),
        cli.rows.unwrap_or_else(|| config.rows),
        cli.rock_percentage
            .unwrap_or_else(|| config.rock_probability),
        cli.moves_required
            .unwrap_or_else(|| config.minimum_moves_required),
        !cli.full_level_view,
        cli.debug,
    );
    game::start_game(game_state);
    Ok(())
}
