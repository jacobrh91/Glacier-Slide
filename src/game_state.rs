#[derive(Clone, Copy)]
pub struct GameConfig {
    pub cols: usize,
    pub rows: usize,
    pub rock_probability: u8, // as a percentage
    pub minimum_moves_required: u16,
}

pub struct GameState {
    pub config: GameConfig,
    pub levels_solved: u16,
    pub player_focused_view: bool,
    pub debug_mode: bool,
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
                cols: (cols + 2) as usize,
                rows: (rows + 2) as usize,
                rock_probability,
                minimum_moves_required,
            },
            levels_solved: 0,
            player_focused_view,
            debug_mode,
            display_solution: false,
        }
    }
}
