mod direction;
mod level_reader;
mod point;
mod solver;

mod tile;
use crate::{game::get_introduction_section, system::exit_game};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use direction::{Direction, Move, Slide};
use solver::Solver;
use time_elapsed;

use crossterm::event::KeyCode;
use level_reader::Level;
use point::Point;
use rand::Rng;

use thousands::Separable;

use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hash, Hasher},
};
use tile::{End, Player, Rock, Start, Tile};

// Board coordinates start at 0, 0 in the top left corner
#[derive(Debug, Clone)]
pub struct Board {
    rows: usize,
    cols: usize,
    pub start: Start,
    pub end: End,
    pub player: Player,
    rocks: Vec<Rock>,
    grid: Vec<Vec<Tile>>,
    pub move_queue: VecDeque<Move>,
    pub player_has_won: bool,
    pub player_focused_view: bool,
}

impl Board {
    pub fn new(
        rows: usize,
        cols: usize,
        start: Point,
        end: Point,
        rocks: Vec<Point>,
        player_focused_view: bool,
    ) -> Self {
        let mut grid = vec![vec![Tile::Ice; cols]; rows];

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

        return Board {
            rows,
            cols,
            start: Start { pos: start.clone() },
            end: End { pos: end },
            player: Player { pos: start },
            rocks: rocks.iter().map(|r| Rock { pos: r.clone() }).collect(),
            grid,
            move_queue: VecDeque::new(),
            player_has_won: false,
            player_focused_view,
        };
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
        (
            possible_values[start_idx].clone(),
            possible_values[end_idx].clone(),
        )
    }

    fn generate_rock(col: usize, row: usize, percent_probability: u8) -> Option<Point> {
        let mut rng = rand::rng();
        let value = rng.random_range(1..101);

        if value <= percent_probability {
            Some(Point { col, row })
        } else {
            None
        }
    }

    fn random(cols: usize, rows: usize, percent_probability: u8) -> Self {
        assert!(cols >= 3 && rows >= 3);
        let (start, end) = Board::get_random_start_and_end(cols, rows);

        let mut rocks = Vec::new();
        let col_right_bound = cols - 2;
        let row_bottom_bound = rows - 2;
        for c in 1..=col_right_bound {
            for r in 1..=row_bottom_bound {
                if let Some(r) = Board::generate_rock(c, r, percent_probability) {
                    rocks.push(r);
                }
            }
        }

        Board::new(rows, cols, start, end, rocks, true)
    }

    pub fn solvable_random(
        cols: usize,
        rows: usize,
        percent_probability: u8,
        minimum_solution_length: u16,
    ) -> Self {
        disable_raw_mode().unwrap();
        for i in get_introduction_section() {
            println!("{}", i);
        }

        let mut value = 1;
        let mut board = Board::random(cols, rows, percent_probability);
        let mut time = time_elapsed::start("level generator");

        let mut solver = Solver::from_board(&board);
        let mut solution = solver.solve();
        let mut denominator: u32 = 1;

        while !solution.is_solvable()
            || solution.solutions[0].chars().count() < minimum_solution_length.into()
        {
            board = Board::random(cols, rows, percent_probability);
            solver = Solver::from_board(&board);
            solution = solver.solve().clone();
            if value % denominator == 0 {
                denominator *= 10;
                time.log_overall(format!(
                    "Boards generated: {:9}",
                    value.separate_with_commas()
                ));
            }
            value += 1;
        }
        enable_raw_mode().unwrap();
        board
    }

    pub fn from_level(l: Level) -> Self {
        Board::new(l.rows, l.cols, l.start, l.end, l.rocks, true)
    }

    // TODO does this need to be public?
    pub fn reset(self: &mut Self) {
        self.update_player_position(self.start.pos.row, self.start.pos.col);
    }

    fn create_arrows(self: &Self, start_position: bool, p: Point) -> String {
        if p.col == 0 {
            if start_position {
                String::from("▷ ")
            } else {
                String::from("◁ ")
            }
        } else if p.col == self.cols - 1 {
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
        } else {
            if start_position {
                String::from("△ ")
            } else {
                String::from("▽ ")
            }
        }
    }

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    pub fn render_board(self: &Self) -> Vec<String> {
        if self.player_focused_view {
            self.render_player_focused_board()
        } else {
            self.render_full_board()
        }
    }

    fn render_player_focused_board(self: &Self) -> Vec<String> {
        let depth = 4;

        let mut result = Vec::new();
        let c_left = self.player.pos.col as isize - depth;
        let c_right = self.player.pos.col as isize + depth;
        let r_top = self.player.pos.row as isize - depth;
        let r_bottom = self.player.pos.row as isize + depth;

        for r in r_top..=r_bottom {
            let mut row_str = String::from("");
            for c in c_left..=c_right {
                if c < 0 || c as usize >= self.cols || r < 0 || r as usize >= self.rows {
                    // deterministic, but unpredictable.
                    let rock_positions: Vec<Point> = self.rocks.iter().map(|r| r.pos).collect();
                    let v = Board::calculate_hash(&((c, r), rock_positions));

                    if v % 10 < 8 {
                        row_str.push_str("██");
                    } else {
                        row_str.push_str("  ");
                    }
                } else {
                    let tile_str = match self.grid[r as usize][c as usize] {
                        Tile::Wall | Tile::Rock => String::from("██"),
                        Tile::Start => self.create_arrows(
                            true,
                            Point {
                                col: c as usize,
                                row: r as usize,
                            },
                        ),
                        Tile::End => self.create_arrows(
                            false,
                            Point {
                                col: c as usize,
                                row: r as usize,
                            },
                        ),
                        Tile::Player => String::from("🟥"),
                        Tile::Ice => String::from("  "),
                    };
                    row_str.push_str(&tile_str);
                }
            }
            result.push(row_str);
        }
        result
    }

    fn render_full_board(self: &Self) -> Vec<String> {
        let mut result = Vec::new();
        for r in 0..self.rows {
            let mut row_str = String::from("");
            for c in 0..self.cols {
                let tile_str = match self.grid[r][c] {
                    Tile::Wall => String::from("██"),
                    Tile::Rock => String::from("██"),
                    Tile::Start => self.create_arrows(true, Point { col: c, row: r }),
                    Tile::End => self.create_arrows(false, Point { col: c, row: r }),
                    Tile::Player => String::from("🟥"),
                    Tile::Ice => String::from("  "),
                };
                row_str.push_str(&tile_str);
            }
            result.push(row_str);
        }
        result
    }

    pub fn steps_in_direction(self: &Self, direction: &Direction) -> u8 {
        let mut curr_pos = self.player.pos.clone();
        let mut steps: u8 = 0;
        let mut stop = false;
        match direction {
            Direction::Up => {
                while !stop && curr_pos.row != 0 {
                    match self.grid[curr_pos.row - 1][curr_pos.col] {
                        Tile::Wall | Tile::Rock => stop = true,
                        _ => {
                            curr_pos.row -= 1;
                            steps += 1;
                        }
                    }
                }
            }
            Direction::Down => {
                while !stop && curr_pos.row != self.rows - 1 {
                    match self.grid[curr_pos.row + 1][curr_pos.col] {
                        Tile::Wall | Tile::Rock => stop = true,
                        _ => {
                            curr_pos.row += 1;
                            steps += 1;
                        }
                    }
                }
            }
            Direction::Left => {
                while !stop && curr_pos.col != 0 {
                    match self.grid[curr_pos.row][curr_pos.col - 1] {
                        Tile::Wall | Tile::Rock => stop = true,
                        _ => {
                            curr_pos.col -= 1;
                            steps += 1;
                        }
                    }
                }
            }
            Direction::Right => {
                while !stop && curr_pos.col != self.cols - 1 {
                    match self.grid[curr_pos.row][curr_pos.col + 1] {
                        Tile::Wall | Tile::Rock => stop = true,
                        _ => {
                            curr_pos.col += 1;
                            steps += 1;
                        }
                    }
                }
            }
        }
        steps
    }

    pub fn update_player_position(self: &mut Self, new_row: usize, new_col: usize) {
        // Cloning because this after moving the player, we need to know how to restore the previous tile.
        let prev_pos = self.player.pos.clone();

        self.player.pos.row = new_row;
        self.player.pos.col = new_col;
        self.grid[new_row][new_col] = Tile::Player;

        if self.player_won() {
            self.player_has_won = true;
        }

        // Clean up the position where the player used to be.
        if prev_pos == self.start.pos {
            self.grid[prev_pos.row][prev_pos.col] = Tile::Start;
        } else if prev_pos == self.end.pos {
            self.grid[prev_pos.row][prev_pos.col] = Tile::End;
        } else {
            self.grid[prev_pos.row][prev_pos.col] = Tile::Ice;
        }
    }

    fn create_slide_move(self: &Self, direction: &Direction) -> Option<Move> {
        // If the queue is not empty, the player is still moving
        if self.move_queue.is_empty() {
            let steps = self.steps_in_direction(&direction);
            if steps > 0 {
                Some(Move::MovePlayer(Slide::new(steps, *direction)))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn respond_to_input(self: &mut Self, key_code: KeyCode) {
        if !self.player_has_won {
            let move_opt: Option<Move> = match key_code {
                KeyCode::Char('w') | KeyCode::Up => self.create_slide_move(&Direction::Up),
                KeyCode::Char('s') | KeyCode::Down => self.create_slide_move(&Direction::Down),
                KeyCode::Char('a') | KeyCode::Left => self.create_slide_move(&Direction::Left),
                KeyCode::Char('d') | KeyCode::Right => self.create_slide_move(&Direction::Right),
                KeyCode::Char('v') | KeyCode::Char('V') => Some(Move::ChangeView),
                KeyCode::Char('Q') => Some(Move::Exit),
                KeyCode::Char('\u{0020}') => Some(Move::Reset),
                _ => None,
            };
            move_opt.map(|r#move| {
                if let Move::Reset = r#move {
                    self.move_queue.clear();
                    if self.player.pos != self.start.pos {
                        // Only queue the reset move if the player is not in the start position.
                        self.move_queue.push_back(r#move);
                    }
                } else {
                    self.move_queue.push_back(r#move);
                }
            });
        }
    }

    pub fn move_player(self: &mut Self, dir: Direction) {
        let (new_row, new_col) = match dir {
            Direction::Up => (self.player.pos.row - 1, self.player.pos.col),
            Direction::Down => (self.player.pos.row + 1, self.player.pos.col),
            Direction::Left => (self.player.pos.row, self.player.pos.col - 1),
            Direction::Right => (self.player.pos.row, self.player.pos.col + 1),
        };
        self.update_player_position(new_row, new_col)
    }

    pub fn player_won(self: &Self) -> bool {
        self.player.pos == self.end.pos
    }

    pub fn process_move(self: &mut Self) -> Option<()> {
        /* Pop the move queue, and respond to the move. This method is intended to be called
          within a callback function in the renderer.
        */
        self.move_queue
            .pop_front()
            .map(|curr_move| match curr_move {
                Move::MovePlayer(mut slide) => {
                    // If the number of steps is greater than 1, modify the Slide object,
                    // and put it back on the front of the queue
                    if slide.steps > 1 {
                        slide.steps -= 1;
                        self.move_queue.push_front(Move::MovePlayer(slide.clone()));
                    }
                    self.move_player(slide.direction)
                }
                Move::Reset => self.reset(),
                Move::ChangeView => self.player_focused_view = !self.player_focused_view,
                Move::Exit => exit_game(),
            })
    }
}
