use crate::parser::LevelArgs;

#[derive(Clone, Debug)]
pub struct GameConfig {
    pub cols: u8,
    pub rows: u8,
    pub rock_probability: u8, // as a percentage
    pub minimum_moves_required: u16,
    pub debug: bool,
    pub board_only: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            cols: 7,
            rows: 7,
            minimum_moves_required: 7,
            rock_probability: 15,
            debug: false,
            board_only: false,
        }
    }
}

impl GameConfig {
    fn new_square(size: u8, minimum_moves_required: u16, rock_probability: u8) -> Self {
        GameConfig {
            cols: size,
            rows: size,
            minimum_moves_required,
            rock_probability,
            debug: false,
            board_only: false,
        }
    }

    pub fn get_config_from_difficulty(difficulty: &str) -> Result<Self, String> {
        match difficulty.trim().to_ascii_lowercase().as_str() {
            "easy" => Ok(GameConfig::new_square(7, 7, 15)),
            "medium" => Ok(GameConfig::new_square(12, 11, 15)),
            "hard" => Ok(GameConfig::new_square(17, 18, 10)),
            "extreme" => Ok(GameConfig::new_square(20, 25, 12)),
            other => Err(format!(
                "Unknown difficulty '{}'. Expected one of: easy, medium, hard, or extreme.",
                other
            )),
        }
    }

    pub fn from_level_args(level: &LevelArgs, board_only: bool) -> Result<Self, String> {
        // Base config comes from difficulty, or falls back to default.
        let base = level
            .difficulty
            .as_deref()
            .map(GameConfig::get_config_from_difficulty)
            .transpose()?
            .unwrap_or_default();

        Ok(GameConfig {
            // Add 2 to the column and row counts to account for borders.
            cols: level.columns.unwrap_or(base.cols) + 2,
            rows: level.rows.unwrap_or(base.rows) + 2,
            rock_probability: level.rock_percentage.unwrap_or(base.rock_probability),
            minimum_moves_required: level.moves_required.unwrap_or(base.minimum_moves_required),
            debug: level.debug,
            board_only,
        })
    }
}

pub struct GameState {
    pub config: GameConfig,
    pub levels_solved: u16,
    pub player_focused_view: bool,
    pub display_solution: bool,
}

impl GameState {
    pub fn new(config: GameConfig, player_focused_view: bool) -> Self {
        GameState {
            config,
            levels_solved: 0,
            player_focused_view,
            display_solution: false,
        }
    }

    pub fn from(level: LevelArgs, board_only: bool) -> Result<Self, String> {
        let config = GameConfig::from_level_args(&level, board_only)?;
        let player_focused_view = !level.full_level_view;
        Ok(GameState::new(config, player_focused_view))
    }
}
