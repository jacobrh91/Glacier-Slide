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

    // Base config comes from difficulty, or falls back to default.
    let base_config = cli
        .difficulty
        .map(GameConfig::get_config_from_difficulty)
        .unwrap_or_default();

    // CLI flags override the base config.
    let cols = cli.columns.unwrap_or(base_config.cols);
    let rows = cli.rows.unwrap_or(base_config.rows);
    let rock_probability = cli.rock_percentage.unwrap_or(base_config.rock_probability);
    let min_moves = cli
        .moves_required
        .unwrap_or(base_config.minimum_moves_required);

    let game_state = GameState::new(
        cols,
        rows,
        rock_probability,
        min_moves,
        !cli.full_level_view,
        cli.debug,
        cli.board_only,
    );

    if cli.board_only {
        let board = board::Board::generate_solvable_board(&game_state.config);
        print!("{}", board.get_layout_json());
    } else {
        game::start_game(game_state);
    }

    Ok(())
}
