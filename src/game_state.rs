#[derive(Clone, Copy)]
pub struct GameConfig {
    pub cols: usize,
    pub rows: usize,
    pub rock_probability: u8, // as a percentage
    pub minimum_moves_required: u8,
}

pub struct GameState {
    pub config: GameConfig,
    pub levels_solved: u16,
    pub player_focused_view: bool,
}

impl GameState {
    pub fn new(
        cols: usize,
        rows: usize,
        rock_probability: u8,
        minimum_moves_required: u8,
        player_focused_view: bool,
    ) -> Self {
        GameState {
            config: GameConfig {
                cols,
                rows,
                rock_probability,
                minimum_moves_required,
            },
            levels_solved: 0,
            player_focused_view,
        }
    }
}
