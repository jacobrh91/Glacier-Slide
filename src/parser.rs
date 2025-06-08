use clap::{arg, builder::PossibleValuesParser, command, Parser};
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

fn difficulty_value_parser() -> PossibleValuesParser {
    clap::builder::PossibleValuesParser::new(["easy", "medium", "hard", "extreme"])
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(ignore_case = true, value_parser = difficulty_value_parser())]
    pub difficulty: Option<String>,
    #[arg(short, long, value_parser = dimension_bounds)]
    pub columns: Option<u8>,
    #[arg(short, long, value_parser = dimension_bounds)]
    pub rows: Option<u8>,
    #[arg(short, long, value_parser = moves_required, value_name = "MOVES")]
    pub moves_required: Option<u16>,
    #[arg(short = 'p', long, value_parser = rock_percentage, value_name = "PERCENTAGE")]
    pub rock_percentage: Option<u8>,
    #[arg(short = 'v', long, default_value_t = false)]
    pub full_level_view: bool,
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}
