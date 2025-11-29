mod board;
mod web_server;

mod game;
mod game_state;
mod parser;
mod renderer;
mod system;

use std::error::Error;

use clap::Parser;
use game_state::{GameConfig, GameState};
use parser::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let mut cli = Args::parse();

    if cli.server_mode {
        ///////////////////////////////////////////////////////////////////////
        // If running in server mode, expose an endpoint that returns levels //
        //   based on a single "difficulty" parameter.                       //
        ///////////////////////////////////////////////////////////////////////
        cli.board_only = true;
        web_server::start_web_server(cli.bind).await?;
    } else {
        //////////////////////////////////////////////
        // Otherwise, play the game in the terminal //
        //////////////////////////////////////////////

        // Base config comes from difficulty, or falls back to default.
        let base_config = cli
            .difficulty
            .as_deref()
            .map(GameConfig::get_config_from_difficulty)
            .transpose()?
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
            // Do not play the game, just return the puzzle to STDOUT as JSON.
            let board = board::Board::generate_solvable_board(&game_state.config, None);
            print!("{}", board.get_layout_json());
        } else {
            game::start_game(game_state)?;
        }
    }
    Ok(())
}
