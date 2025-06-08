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
    let config_from_difficulty = cli.difficulty.map(GameConfig::get_config_from_difficulty);

    let config = config_from_difficulty.unwrap_or(GameConfig::default());

    let cols = cli.columns.unwrap_or(config.cols);
    let rows = cli.rows.unwrap_or(config.rows);
    let rock_probability = cli.rock_percentage.unwrap_or(config.rock_probability);
    let min_moves = cli.moves_required.unwrap_or(config.minimum_moves_required);

    let game_state = GameState::new(
        cols,
        rows,
        rock_probability,
        min_moves,
        !cli.full_level_view,
        cli.debug,
    );
    game::start_game(game_state);
    Ok(())
}
