#[derive(Clone, Copy)]
pub struct GameConfig {
    pub cols: u8,
    pub rows: u8,
    pub rock_probability: u8, // as a percentage
    pub minimum_moves_required: u16,
    pub debug: bool,
}

impl GameConfig {
    pub fn default() -> Self {
        GameConfig {
            cols: 7,
            rows: 7,
            minimum_moves_required: 7,
            rock_probability: 15,
            debug: false,
        }
    }

    pub fn new(columns_and_rows: u8, minimum_moves_required: u16, rock_probability: u8) -> Self {
        GameConfig {
            cols: columns_and_rows,
            rows: columns_and_rows,
            minimum_moves_required,
            rock_probability,
            debug: false,
        }
    }
    pub fn get_config_from_difficulty(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "easy" => GameConfig::new(7, 7, 15),
            "medium" => GameConfig::new(12, 11, 15),
            "hard" => GameConfig::new(17, 18, 10),
            "extreme" => GameConfig::new(20, 25, 12),
            _ => {
                // This should not be possible given the argument parser should already have guaranteed this string is valid.
                panic!("Unknown difficulty argument '{}'. Choose from 'easy', 'medium', 'hard', or 'extreme'.", s)
            }
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
        debug_mode: bool,
    ) -> Self {
        GameState {
            config: GameConfig {
                // Add 2 to the column and row bounds to add the top/bottom or left/right borders to the column/row count.
                cols: cols + 2,
                rows: rows + 2,
                rock_probability,
                debug: debug_mode,
                minimum_moves_required,
            },
            levels_solved: 0,
            player_focused_view,
            display_solution: false,
        }
    }
}
