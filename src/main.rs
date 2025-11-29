mod board;
mod web_server;

mod game;
mod game_state;
mod parser;
mod renderer;
mod system;

use std::error::Error;

use clap::Parser;
use game_state::GameState;
use parser::{Args, Command, ServeArgs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let cli = Args::parse();

    match cli.command {
        Command::Serve(ServeArgs { bind }) => {
            web_server::start_web_server(bind).await?;
        }
        Command::Play(level_args) => {
            let game_state = GameState::from(level_args, false)?;
            game::start_game(game_state)?;
        }
        Command::Generate(level_args) => {
            let game_state = GameState::from(level_args, true)?;
            // Do not play the game, just return the puzzle to STDOUT as JSON.
            let board = board::Board::generate_solvable_board(&game_state.config, None);
            print!("{}", board.get_layout_json());
        }
    }
    Ok(())
}
