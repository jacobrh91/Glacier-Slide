/*

-c, --columns  3-20 (default: 7)
-r, --rows     3-20 (default: 7)
-m, --moves-required 1-15. (default: 5)

-v, --full_level_view bool (default: false)
-p, --rock-percentage 10-50 (default: 15)


*/
use clap::{arg, command, Parser};
use clap_num::number_range;

fn dimension_bounds(s: &str) -> Result<u8, String> {
    number_range(s, 3, 20)
}

fn moves_required(s: &str) -> Result<u8, String> {
    number_range(s, 1, 15)
}

fn rock_percentage(s: &str) -> Result<u8, String> {
    number_range(s, 10, 50)
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 7, value_parser = dimension_bounds)]
    pub columns: u8,
    #[arg(short, long, default_value_t = 7, value_parser = dimension_bounds)]
    pub rows: u8,
    #[arg(short, long, default_value_t = 5, value_parser = moves_required, value_name = "MOVES")]
    pub moves_required: u8,
    #[arg(short = 'v', long, default_value_t = false)]
    pub full_level_view: bool,
    #[arg(short = 'p', long, default_value_t = 15, value_parser = rock_percentage, value_name = "PERCENTAGE")]
    pub rock_percentage: u8,
}
