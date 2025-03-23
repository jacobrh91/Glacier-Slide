pub mod level_reader;
pub mod point;
mod tile;

use crate::direction::{Direction, Move, Slide};

use crossterm::event::KeyCode;
use level_reader::Level;

use point::Point;
use std::collections::VecDeque;
use tile::{End, Player, Rock, Start, Tile};

// Board coordinates start at 0, 0 in the top left corner
#[derive(Debug)]
pub struct Board {
    rows: usize,
    cols: usize,
    pub start: Start,
    pub end: End,
    pub player: Player,
    rocks: Vec<Rock>,
    grid: Vec<Vec<Tile>>,
    pub move_queue: VecDeque<Move>,
    debug_mode: bool,
}

impl Board {
    pub fn new(rows: usize, cols: usize, start: Point, end: Point, rocks: Vec<Point>) -> Self {
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
            debug_mode: false,
        };
    }

    pub fn from_level(l: Level) -> Self {
        Board::new(l.rows, l.cols, l.start, l.end, l.rocks)
    }

    pub fn enable_debug_mode(self: &mut Self) {
        self.debug_mode = true;
    }

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

    pub fn render_board(self: &Self) -> Vec<String> {
        let mut result = Vec::new();
        for r in 0..self.rows {
            let mut row_str = String::from("");
            for c in 0..self.cols {
                let tile_str = match self.grid[r][c] {
                    Tile::Wall => {
                        if !self.debug_mode {
                            String::from("██")
                        } else {
                            if r == 0 || r == self.rows - 1 {
                                format!(" {:1}", c % 10).to_string()
                            } else if c == 0 || c == self.cols - 1 {
                                format!(" {:1}", r % 10).to_string()
                            } else {
                                String::from("██")
                            }
                        }
                    }
                    Tile::Rock => String::from("██"),
                    Tile::Start => self.create_arrows(true, Point { col: c, row: r }),
                    Tile::End => self.create_arrows(false, Point { col: c, row: r }),
                    Tile::Player => String::from("🟥"), // ◖◗
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
                Some(Move::MovePlayer(Slide::new(steps, direction.clone())))
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn respond_to_input(self: &mut Self, key_code: KeyCode) {
        let move_opt: Option<Move> = match key_code {
            KeyCode::Char('w') | KeyCode::Up => self.create_slide_move(&Direction::Up),
            KeyCode::Char('s') | KeyCode::Down => self.create_slide_move(&Direction::Down),
            KeyCode::Char('a') | KeyCode::Left => self.create_slide_move(&Direction::Left),
            KeyCode::Char('d') | KeyCode::Right => self.create_slide_move(&Direction::Right),
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
                Move::Teleport(p) => self.update_player_position(p.row, p.col),
            })
    }

    // fn add_rock(self: &mut Self, p: Point) -> bool {
    //     match self.grid[p.row][p.col] {
    //         Tile::Ice => {
    //             self.grid[p.row][p.col] = Tile::Rock;
    //             true
    //         }
    //         _ => false,
    //     }
    // }
}
