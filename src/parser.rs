use clap::{arg, command, Parser, Subcommand};
use clap_num::number_range;

fn dimension_bounds(s: &str) -> Result<u8, String> {
    number_range(s, 3, 20)
}

fn moves_required(s: &str) -> Result<u16, String> {
    number_range(s, 1, 35)
}

fn rock_percentage(s: &str) -> Result<u8, String> {
    number_range(s, 5, 50)
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Play the game in the terminal.
    Play(LevelArgs),
    /// Generate a solvable board and print it as JSON.
    Generate(LevelArgs),
    /// Run the HTTP server to geneerate solvable boards.
    Serve(ServeArgs),
}

#[derive(Debug, Parser)]
pub struct LevelArgs {
    #[arg(ignore_case = true, value_parser = ["easy", "medium", "hard", "extreme"])]
    pub difficulty: Option<String>,
    #[arg(short, long, value_parser = dimension_bounds)]
    pub columns: Option<u8>,
    #[arg(short, long, value_parser = dimension_bounds)]
    pub rows: Option<u8>,
    #[arg(short, long, value_parser = moves_required, value_name = "MOVES")]
    pub moves_required: Option<u16>,
    #[arg(short = 'p', long, value_parser = rock_percentage, value_name = "PERCENTAGE")]
    pub rock_percentage: Option<u8>,
    #[arg(short = 'v', long)]
    pub full_level_view: bool,
    #[arg(short, long)]
    pub debug: bool,
}

#[derive(Debug, Parser)]
pub struct ServeArgs {
    #[arg(long, default_value = "127.0.0.1:7878")]
    pub bind: String,
}
