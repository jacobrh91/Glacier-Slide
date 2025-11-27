mod direction;
mod point;
mod solution;
mod tile;

use crate::{
    game_state::{GameConfig, GameState},
    system::exit_game,
};

use crossterm::event::KeyCode;
use direction::{Direction, Move, Slide};
use point::Point;
use rand::Rng;
use serde::{Serialize, Serializer};
use solution::Solution;
use thousands::Separable;
use tile::{End, Player, Rock, Start, Tile};
use time_elapsed::{self, TimeElapsed};

use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

fn grid_as_strings<S>(grid: &[Vec<Tile>], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let rows: Vec<String> = grid
        .iter()
        .map(|row| row.iter().map(Tile::as_char).collect())
        .collect();

    rows.serialize(serializer)
}

#[derive(Serialize)]
pub struct BoardLayout {
    rows: usize, // Value includes the left and right border columns
    cols: usize, // Value includes the top and bottom border rows
    start: Start,
    end: End,
    rocks: Vec<Rock>,
    // Including the grid is actually redundant (because the same info can be
    // derived in the other fields), but it provides a clean, human-readable
    // layout of what the level looks like.
    #[serde(serialize_with = "grid_as_strings")]
    grid: Vec<Vec<Tile>>,
}

// Board coordinates start at 0, 0 in the top left corner
pub struct Board {
    layout: BoardLayout,
    pub player: Player,
    pub move_queue: VecDeque<Move>,
    pub player_has_won: bool,
    bot_is_solving: bool,
    pub solution: Option<Solution>,
    pub game_state: Option<Rc<RefCell<GameState>>>,
}

impl Board {
    pub fn new(rows: usize, cols: usize, start: Point, end: Point, rocks: Vec<Point>) -> Self {
        let mut grid = vec![vec![Tile::Ice; cols]; rows];

        #[allow(clippy::needless_range_loop)]
        for r in 0..rows {
            for c in 0..cols {
                // if first or last row or first or last column
                if r == 0 || r == rows - 1 || c == 0 || c == cols - 1 {
                    grid[r][c] = Tile::Wall;
                } else {
                    grid[r][c] = Tile::Ice;
                }
            }
        }

        // Set player (always at start during initialization) and end positions.
        grid[start.row][start.col] = Tile::Player;
        grid[end.row][end.col] = Tile::End;

        // Set rocks
        for rock in &rocks {
            grid[rock.row][rock.col] = Tile::Rock;
        }

        Board {
            layout: BoardLayout {
                rows,
                cols,
                start: Start(start),
                end: End(end),
                rocks: rocks.iter().map(|r| Rock(*r)).collect(),
                grid,
            },
            player: Player(start),
            move_queue: VecDeque::new(),
            player_has_won: false,
            bot_is_solving: false,
            solution: None,
            game_state: None,
        }
    }

    pub fn get_layout_json(&self) -> String {
        serde_json::to_string_pretty(&self.layout).expect("Failed to serialize to JSON")
    }

    // Game state is not needed when generating solvable boards, but
    //   is needed when the level is being played.
    pub fn attach_game_state(&mut self, game_state: Rc<RefCell<GameState>>) {
        self.game_state = Some(game_state);
    }

    fn get_random_start_and_end(cols: usize, rows: usize) -> (Point, Point) {
        assert!(cols >= 3 && rows >= 3);
        let total_possible = (2 * (cols - 2)) + (2 * (rows - 2));
        let mut possible_values = Vec::with_capacity(total_possible);

        // Get top and bottom borders (not including the corner)
        let max_col = cols - 1;
        for c in 0..max_col {
            possible_values.push(Point { col: c, row: 0 });
            possible_values.push(Point {
                col: c,
                row: rows - 1,
            });
        }
        // Get left and right borders (not including the corner)
        let max_row = rows - 1;
        for r in 0..max_row {
            possible_values.push(Point { col: 0, row: r });
            possible_values.push(Point {
                col: cols - 1,
                row: r,
            });
        }

        let mut rng = rand::rng();

        let start_idx = rng.random_range(0..total_possible);
        let mut end_idx = rng.random_range(0..total_possible);
        while start_idx == end_idx {
            end_idx = rng.random_range(0..total_possible);
        }

        (possible_values[start_idx], possible_values[end_idx])
    }

    fn generate_rock(col: usize, row: usize, percent_probability: u8) -> Option<Point> {
        let mut rng = rand::rng();
        let value = rng.random_range(1..=100);

        if value <= percent_probability {
            Some(Point { col, row })
        } else {
            None
        }
    }

    fn generate_random_board(game_config: &GameConfig) -> Self {
        assert!(game_config.cols >= 3 && game_config.rows >= 3);

        let cols = game_config.cols as usize;
        let rows = game_config.rows as usize;

        let (start, end) = Board::get_random_start_and_end(cols, rows);

        let mut rocks = Vec::new();
        for col in 1..cols - 1 {
            for row in 1..rows - 1 {
                if let Some(r) = Board::generate_rock(col, row, game_config.rock_probability) {
                    rocks.push(r);
                }
            }
        }

        Board::new(rows, cols, start, end, rocks)
    }

    pub fn generate_solvable_board(game_config: &GameConfig) -> Self {
        let mut time: Option<TimeElapsed> = None;
        let mut board_count: u32 = 1;
        let mut denominator: u32 = 1;

        if game_config.debug {
            time = Some(time_elapsed::start("level generator"));
        } else if !game_config.board_only {
            println!("Generating level...");
        }

        let mut board;

        loop {
            if board_count > 1_000_000 {
                if !game_config.board_only {
                    println!("Could not find solvable level after 1,000,000 attempts. Adjust board parameters.");
                }
                exit_game();
            }
            board = Board::generate_random_board(game_config);

            if game_config.debug && board_count % denominator == 0 {
                denominator *= 10;

                if let Some(t) = time.as_mut() {
                    t.log_overall(format!(
                        "Boards generated: {:9}",
                        board_count.separate_with_commas()
                    ));
                }
            }

            board_count += 1;

            let max_depth = game_config.minimum_moves_required + 2;
            board.solve(max_depth);

            let solution_found = board
                .solution
                .as_ref()
                .and_then(|s| s.steps.as_ref())
                .map(|steps| steps.len() >= game_config.minimum_moves_required as usize)
                .unwrap_or(false);

            if solution_found {
                break;
            }
        }

        if game_config.board_only {
            // After solving, replace the player's tile with Start in the layout,
            // so JSON shows 'S' instead of 'P' in level layout.
            let start: Point = board.layout.start.0;
            board.layout.grid[start.row][start.col] = Tile::Start;
        }

        board
    }

    fn create_arrows(&self, start_position: bool, p: Point) -> String {
        if p.col == 0 {
            if start_position {
                String::from("▷ ")
            } else {
                String::from("◁ ")
            }
        } else if p.col == self.layout.cols - 1 {
            if start_position {
                String::from(" ◁")
            } else {
                String::from(" ▷")
            }
        } else if p.row == 0 {
            if start_position {
                String::from("▽ ")
            } else {
                String::from("△ ")
            }
        } else if start_position {
            String::from("△ ")
        } else {
            String::from("▽ ")
        }
    }

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    pub fn render_board(&self) -> Vec<String> {
        let player_focused_view = self
            .game_state
            .as_ref()
            .map(|x| x.borrow().player_focused_view)
            .unwrap_or(true);

        if player_focused_view {
            self.render_player_focused_board()
        } else {
            self.render_full_board()
        }
    }

    fn render_tile_at(&self, col: isize, row: isize, inside_bounds: bool) -> String {
        if !inside_bounds {
            // Deriving the border noise from a hash of the rock data is deterministic, but unpredictable.
            let rock_positions: Vec<Point> = self.layout.rocks.iter().map(|r| r.0).collect();
            let v = Board::calculate_hash(&((col, row), rock_positions));
            return if v % 10 < 8 {
                String::from("██")
            } else {
                String::from("  ")
            };
        }

        let col_usize = col as usize;
        let row_usize = row as usize;

        match self.layout.grid[row_usize][col_usize] {
            Tile::Wall | Tile::Rock => String::from("██"),
            Tile::Player => String::from("🟥"),
            Tile::Ice => String::from("  "),
            Tile::Start => self.create_arrows(
                true,
                Point {
                    col: col_usize,
                    row: row_usize,
                },
            ),
            Tile::End => self.create_arrows(
                false,
                Point {
                    col: col_usize,
                    row: row_usize,
                },
            ),
        }
    }

    fn render_player_focused_board(&self) -> Vec<String> {
        let depth = 4;
        let mut result = Vec::new();

        let col_center = self.player.0.col as isize;
        let row_center = self.player.0.row as isize;

        let col_min = col_center - depth;
        let col_max = col_center + depth;
        let row_min = row_center - depth;
        let row_max = row_center + depth;

        for row in row_min..=row_max {
            let mut row_str = String::new();
            for col in col_min..=col_max {
                let inside_bounds = col >= 0
                    && row >= 0
                    && (col as usize) < self.layout.cols
                    && (row as usize) < self.layout.rows;

                row_str.push_str(&self.render_tile_at(col, row, inside_bounds));
            }
            result.push(row_str);
        }

        result
    }

    fn render_full_board(&self) -> Vec<String> {
        let mut result = Vec::new();

        for row in 0..self.layout.rows {
            let mut row_str = String::new();

            for col in 0..self.layout.cols {
                row_str.push_str(&self.render_tile_at(
                    col as isize,
                    row as isize,
                    true, // always inside bounds in full-board mode
                ));
            }
            result.push(row_str);
        }
        result
    }

    fn steps_in_direction(&self, direction: &Direction) -> u8 {
        let (col_change, row_change): (isize, isize) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let mut current_position = self.player.0;
        let mut steps: u8 = 0;

        loop {
            let next_col = current_position.col as isize + col_change;
            let next_row = current_position.row as isize + row_change;

            // Check level bounds
            if next_col < 0
                || next_row < 0
                || next_col as usize >= self.layout.cols
                || next_row as usize >= self.layout.rows
            {
                break;
            }

            // Check whether a wall or rock should block the players movement
            match self.layout.grid[next_row as usize][next_col as usize] {
                Tile::Wall | Tile::Rock => break,
                _ => {
                    current_position.col = next_col as usize;
                    current_position.row = next_row as usize;
                    steps += 1;
                }
            }
        }

        steps
    }

    fn update_player_position(&mut self, new_row: usize, new_col: usize) {
        if self.player.0.row != new_row || self.player.0.col != new_col {
            // Cloning because after moving the player, we need to know how to restore the previous tile.
            let prev_pos = self.player.0;

            self.player.0.row = new_row;
            self.player.0.col = new_col;
            self.layout.grid[new_row][new_col] = Tile::Player;

            if self.player_won() && !self.bot_is_solving {
                self.player_has_won = true;
                if let Some(game_state_ref) = self.game_state.as_mut() {
                    game_state_ref.borrow_mut().levels_solved += 1;
                }
            }

            // Clean up the position where the player used to be.
            if prev_pos == self.layout.start.0 {
                self.layout.grid[prev_pos.row][prev_pos.col] = Tile::Start;
            } else if prev_pos == self.layout.end.0 {
                self.layout.grid[prev_pos.row][prev_pos.col] = Tile::End;
            } else {
                self.layout.grid[prev_pos.row][prev_pos.col] = Tile::Ice;
            }
        }
    }

    fn create_slide_move(&self, direction: &Direction) -> Option<Move> {
        // If the queue is not empty, the player is still moving
        if self.move_queue.is_empty() {
            let steps = self.steps_in_direction(direction);
            if steps > 0 {
                Some(Move::SlidePlayer(Slide::new(steps, *direction)))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn respond_to_input(&mut self, key_code: KeyCode) {
        if !self.player_has_won {
            let move_opt: Option<Move> = match key_code {
                KeyCode::Char('w') | KeyCode::Up => self.create_slide_move(&Direction::Up),
                KeyCode::Char('s') | KeyCode::Down => self.create_slide_move(&Direction::Down),
                KeyCode::Char('a') | KeyCode::Left => self.create_slide_move(&Direction::Left),
                KeyCode::Char('d') | KeyCode::Right => self.create_slide_move(&Direction::Right),
                KeyCode::Char('v') | KeyCode::Char('V') => Some(Move::ChangeView),
                KeyCode::Char('g') | KeyCode::Char('G') => Some(Move::ShowSolution),
                KeyCode::Char('Q') => Some(Move::Exit),
                KeyCode::Char('\u{0020}') => Some(Move::Reset),
                _ => None,
            };
            if let Some(r#move) = move_opt {
                if let Move::Reset = r#move {
                    self.move_queue.clear();
                    if self.player.0 != self.layout.start.0 {
                        // Only queue the reset move if the player is not in the start position.
                        self.move_queue.push_back(r#move);
                    }
                } else {
                    self.move_queue.push_back(r#move);
                }
            }
        }
    }

    pub fn move_player(&mut self, dir: Direction) {
        let (new_row, new_col) = match dir {
            Direction::Up => (self.player.0.row - 1, self.player.0.col),
            Direction::Down => (self.player.0.row + 1, self.player.0.col),
            Direction::Left => (self.player.0.row, self.player.0.col - 1),
            Direction::Right => (self.player.0.row, self.player.0.col + 1),
        };
        self.update_player_position(new_row, new_col)
    }

    pub fn player_won(&self) -> bool {
        self.player.0 == self.layout.end.0
    }

    pub fn process_move(&mut self) -> Option<()> {
        /* Pop the move queue, and respond to the move. This method is intended to be called
          within a callback function in the renderer.
        */
        self.move_queue
            .pop_front()
            .map(|curr_move| match curr_move {
                Move::SlidePlayer(mut slide) => {
                    // If the number of steps is greater than 1, modify the Slide object,
                    // and put it back on the front of the queue
                    if slide.steps > 1 {
                        slide.steps -= 1;
                        self.move_queue.push_front(Move::SlidePlayer(slide.clone()));
                    }
                    self.move_player(slide.direction)
                }
                Move::Reset => {
                    self.update_player_position(self.layout.start.0.row, self.layout.start.0.col)
                }
                Move::ShowSolution => {
                    if let Some(game_state) = &self.game_state {
                        let mut game_state_ref = game_state.borrow_mut();
                        game_state_ref.display_solution = true;
                    }
                }
                Move::ChangeView => {
                    if let Some(game_state) = &self.game_state {
                        let mut game_state_ref = game_state.borrow_mut();
                        game_state_ref.player_focused_view = !game_state_ref.player_focused_view;
                    }
                }
                Move::Exit => exit_game(),
            })
    }

    fn solve(&mut self, max_depth: u16) {
        let mut visited = HashSet::<Point>::new();
        let mut solution = Solution::new();

        self.bot_is_solving = true;

        // Breadth-first search guarantees the first solution we find is the shortest (if there is a solution).
        let mut bfs_queue = VecDeque::new();
        bfs_queue.push_back((Vec::<Direction>::new(), self.layout.start.0));

        while let Some((parent_prev, parent_pos)) = bfs_queue.pop_front() {
            if parent_pos == self.layout.end.0 {
                solution.steps = Some(parent_prev);
                break;
            } else if parent_prev.len() > max_depth as usize {
                break;
            } else if !visited.contains(&parent_pos) {
                visited.insert(parent_pos);
                solution.edges_traversed += 1;

                // Find possible next moves
                for direction in self.get_possible_moves(parent_prev.last()) {
                    // Reset the player to the parent position
                    self.update_player_position(parent_pos.row, parent_pos.col);
                    let steps = self.steps_in_direction(&direction);
                    if steps > 0 {
                        let mut child_moves = parent_prev.clone();
                        child_moves.push(direction);
                        for _ in 0..steps {
                            self.move_player(direction);
                        }
                        let child_position = self.player.0;
                        bfs_queue.push_back((child_moves, child_position));
                    }
                }
            }
        }
        // After solving (or giving up due to the search depth), return the player to the start
        self.update_player_position(self.layout.start.0.row, self.layout.start.0.col);
        self.bot_is_solving = false;
        self.player_has_won = false;

        self.solution = Some(solution);
    }

    fn get_possible_moves(&self, previous_move: Option<&Direction>) -> Vec<Direction> {
        match previous_move {
            Some(Direction::Up) | Some(Direction::Down) => {
                vec![Direction::Right, Direction::Left]
            }
            Some(Direction::Left) | Some(Direction::Right) => {
                vec![Direction::Up, Direction::Down]
            }
            None => Direction::ALL.to_vec(),
        }
    }
}
