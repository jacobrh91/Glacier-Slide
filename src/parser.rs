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
    /// Run the HTTP server to generate solvable boards.
    Serve(ServeArgs),
}

#[derive(Debug, Parser)]
pub struct LevelArgs {
    /// Puzzle difficulty
    #[arg(ignore_case = true, value_parser = ["easy", "medium", "hard", "extreme"])]
    pub difficulty: Option<String>,
    /// Number of columns
    #[arg(short, long, value_parser = dimension_bounds)]
    pub columns: Option<u8>,
    /// Number of rows
    #[arg(short, long, value_parser = dimension_bounds)]
    pub rows: Option<u8>,
    /// Minimum moves required to win
    #[arg(short, long, value_parser = moves_required, value_name = "MOVES")]
    pub moves_required: Option<u16>,
    /// Percent of tiles that are rocks
    #[arg(short = 'p', long, value_parser = rock_percentage, value_name = "PERCENTAGE")]
    pub rock_percentage: Option<u8>,
    /// Toggle between views
    #[arg(short = 'v', long)]
    pub full_level_view: bool,
    /// Enable debug mode
    #[arg(short, long)]
    pub debug: bool,
}

#[derive(Debug, Parser)]
pub struct ServeArgs {
    /// The address and port the server will bind to
    #[arg(long, default_value = "127.0.0.1:7878")]
    pub bind: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    //////////////////////
    // Bounds functions //
    //////////////////////
    #[test]
    fn dimension_bounds_accepts_in_range_values() {
        assert_eq!(dimension_bounds("3").unwrap(), 3); // lower bound
        assert_eq!(dimension_bounds("10").unwrap(), 10); // middle value
        assert_eq!(dimension_bounds("20").unwrap(), 20); // upper bound
    }

    #[test]
    fn dimension_bounds_rejects_out_of_range_values() {
        assert!(dimension_bounds("2").is_err()); // too low
        assert!(dimension_bounds("21").is_err()); // too high
    }

    #[test]
    fn moves_required_accepts_in_range_values() {
        assert_eq!(moves_required("1").unwrap(), 1); // lower bound
        assert_eq!(moves_required("10").unwrap(), 10); // middle value
        assert_eq!(moves_required("35").unwrap(), 35); // upper bound
    }

    #[test]
    fn moves_required_rejects_out_of_range_values() {
        assert!(moves_required("0").is_err()); // too low
        assert!(moves_required("36").is_err()); // too high
    }

    #[test]
    fn rock_percentage_accepts_in_range_values() {
        assert_eq!(rock_percentage("5").unwrap(), 5); // lower bound
        assert_eq!(rock_percentage("25").unwrap(), 25); // middle value
        assert_eq!(rock_percentage("50").unwrap(), 50); // upper bound
    }

    #[test]
    fn rock_percentage_rejects_out_of_range_values() {
        assert!(rock_percentage("4").is_err()); // too low
        assert!(rock_percentage("51").is_err()); // too high
    }

    /////////////////////
    // Play subcommand //
    /////////////////////
    #[test]
    fn parse_play_with_all_defaults() {
        let args = Args::parse_from(["./program", "play"]);

        match args.command {
            Command::Play(level) => {
                assert!(level.difficulty.is_none());
                assert!(level.columns.is_none());
                assert!(level.rows.is_none());
                assert!(level.moves_required.is_none());
                assert!(level.rock_percentage.is_none());
                assert!(!level.full_level_view);
                assert!(!level.debug);
            }
            _ => panic!("Expected Play command"),
        }
    }

    #[test]
    fn parse_play_with_difficulty() {
        let diff_str = "hard";
        let args = Args::parse_from(["./program", "play", diff_str]);

        match args.command {
            Command::Play(level) => {
                assert_eq!(level.difficulty.as_deref(), Some(diff_str));
            }
            _ => panic!("Expected Play command"),
        }
    }

    #[test]
    fn parse_play_with_difficulty_case_insensitive() {
        let diff_str = "hARd";
        let args = Args::parse_from(["./program", "play", diff_str]);

        match args.command {
            Command::Play(level) => {
                assert_eq!(level.difficulty.as_deref(), Some(diff_str));
            }
            _ => panic!("Expected Play command"),
        }
    }

    #[test]
    fn parse_play_rejects_invalid_difficulty() {
        // "insane" is not a valid difficulty
        let res = Args::try_parse_from(["./program", "play", "insane"]);
        assert!(res.is_err());
    }

    #[test]
    fn parse_play_with_all_numeric_options_and_flags() {
        let args = Args::parse_from([
            "./program",
            "play",
            "easy",
            "--columns",
            "10",
            "--rows",
            "12",
            "--moves-required",
            "15",
            "--rock-percentage",
            "30",
            "-v",
            "-d",
        ]);

        match args.command {
            Command::Play(level) => {
                assert_eq!(level.difficulty.as_deref(), Some("easy"));
                assert_eq!(level.columns, Some(10));
                assert_eq!(level.rows, Some(12));
                assert_eq!(level.moves_required, Some(15));
                assert_eq!(level.rock_percentage, Some(30));
                assert!(level.full_level_view);
                assert!(level.debug);
            }
            _ => panic!("Expected Play command"),
        }
    }

    #[test]
    fn parse_play_rejects_out_of_range_rock_percentage() {
        // Below minimum (5)
        let res = Args::try_parse_from(["./program", "play", "easy", "--rock-percentage", "1"]);
        assert!(res.is_err());

        // Above maximum (50)
        let res = Args::try_parse_from(["./program", "play", "easy", "--rock-percentage", "99"]);
        assert!(res.is_err());
    }

    /////////////////////////
    // Generate subcommand //
    /////////////////////////
    #[test]
    fn parse_generate_with_some_options() {
        let args = Args::parse_from([
            "./program",
            "generate",
            "medium",
            "-c",
            "8",
            "-r",
            "9",
            "-m",
            "5",
            "-p",
            "10",
        ]);

        match args.command {
            Command::Generate(level) => {
                assert_eq!(level.difficulty.as_deref(), Some("medium"));
                assert_eq!(level.columns, Some(8));
                assert_eq!(level.rows, Some(9));
                assert_eq!(level.moves_required, Some(5));
                assert_eq!(level.rock_percentage, Some(10));
            }
            _ => panic!("Expected Generate command"),
        }
    }

    #[test]
    fn parse_generate_rejects_out_of_range_rock_percentage() {
        // Below minimum (5)
        let res = Args::try_parse_from(["./program", "generate", "easy", "--rock-percentage", "1"]);
        assert!(res.is_err());

        // Above maximum (50)
        let res =
            Args::try_parse_from(["./program", "generate", "easy", "--rock-percentage", "99"]);
        assert!(res.is_err());
    }

    //////////////////////
    // Serve subcommand //
    //////////////////////
    #[test]
    fn parse_serve_uses_default_bind() {
        let args = Args::parse_from(["./program", "serve"]);

        match args.command {
            Command::Serve(serve_args) => {
                assert_eq!(serve_args.bind, "127.0.0.1:7878");
            }
            _ => panic!("Expected Serve command"),
        }
    }

    #[test]
    fn parse_serve_accepts_custom_bind() {
        let args = Args::parse_from(["./program", "serve", "--bind", "0.0.0.0:9999"]);

        match args.command {
            Command::Serve(serve_args) => {
                assert_eq!(serve_args.bind, "0.0.0.0:9999");
            }
            _ => panic!("Expected Serve command"),
        }
    }
}
