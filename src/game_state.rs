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
    pub fn new(columns_and_rows: u8, minimum_moves_required: u16, rock_probability: u8) -> Self {
        GameConfig {
            cols: columns_and_rows,
            rows: columns_and_rows,
            minimum_moves_required,
            rock_probability,
            debug: false,
            board_only: false,
        }
    }
    pub fn get_config_from_difficulty(s: &str) -> Result<Self, String> {
        match s.trim().to_lowercase().as_str() {
            "easy" => Ok(GameConfig::new(7, 7, 15)),
            "medium" => Ok(GameConfig::new(12, 11, 15)),
            "hard" => Ok(GameConfig::new(17, 18, 10)),
            "extreme" => Ok(GameConfig::new(20, 25, 12)),
            other => Err(format!(
                "Unknown difficulty '{}'. Expected one of: easy, medium, hard, or extreme.",
                other
            )),
        }
    }
}

pub struct GameState {
    pub config: GameConfig,
    pub levels_solved: u16,
    pub player_focused_view: bool,
    pub display_solution: bool,
}

impl GameState {
    pub fn new(
        cols: u8,
        rows: u8,
        rock_probability: u8,
        minimum_moves_required: u16,
        player_focused_view: bool,
        debug: bool,
        board_only: bool,
    ) -> Self {
        GameState {
            config: GameConfig {
                // Add 2 to the column and row bounds to add the top/bottom or left/right borders to the column/row count.
                cols: cols + 2,
                rows: rows + 2,
                rock_probability,
                minimum_moves_required,
                debug,
                board_only,
            },
            levels_solved: 0,
            player_focused_view,
            display_solution: false,
        }
    }

    pub fn from(level: LevelArgs, board_only: bool) -> Result<Self, String> {
        // Base config comes from difficulty, or falls back to default.
        let base_config = level
            .difficulty
            .as_deref()
            .map(GameConfig::get_config_from_difficulty)
            .transpose()?
            .unwrap_or_default();

        // CLI flags override the base config.
        let cols = level.columns.unwrap_or(base_config.cols);
        let rows = level.rows.unwrap_or(base_config.rows);
        let rock_probability = level
            .rock_percentage
            .unwrap_or(base_config.rock_probability);
        let min_moves = level
            .moves_required
            .unwrap_or(base_config.minimum_moves_required);

        let game_state = GameState::new(
            cols,
            rows,
            rock_probability,
            min_moves,
            !level.full_level_view,
            level.debug,
            board_only,
        );
        Ok(game_state)
    }
}
