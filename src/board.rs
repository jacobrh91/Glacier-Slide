mod tile;

use crate::direction::Direction;

use std::collections::VecDeque;
//use tile::{End, Player, Rock, Start, Tile};
use tile::{End, Player, Start, Tile};

// use std::iter::Iterator;
// use std::time::Duration;
// use std::{thread, time};
#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

// pub struct MoveIterator<F> {
//     queue: VecDeque<F>,
// }

// impl<F> Iterator for MoveIterator<F>
// where
//     F: FnMut() -> (),
// {
//     type Item = ();
//     fn next(&mut self) -> Option<Self::Item> {
//         self.queue.front_mut().map(|x| x())
//     }
// }

// impl<F: FnMut() -> ()> MoveIterator<F> {
//     pub fn new() -> MoveIterator<F> {
//         MoveIterator {
//             queue: VecDeque::new(),
//         }
//     }
// }

// Board coordinates start at 0, 0 in the top left corner
#[derive(Debug)]
pub struct Board {
    rows: usize,
    cols: usize,
    start: Start,
    end: End,
    pub player: Player,
    // rocks: Vec<Rock>,
    grid: Vec<Vec<Tile>>,
    pub move_queue: VecDeque<Direction>,
}

impl<'a> Board {
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
            // rocks: rocks.iter().map(|r| Rock { pos: r.clone() }).collect(),
            grid,
            move_queue: VecDeque::new(),
        };
    }

    fn create_arrows(self: &Self, start_position: bool, p: Point) -> &str {
        if p.col == 0 {
            if start_position {
                "▷ "
            } else {
                " ▷"
            }
        } else if p.col == self.cols - 1 {
            if start_position {
                " ◁"
            } else {
                "◁ "
            }
        } else if p.row == 0 {
            if start_position {
                "▽▽"
            } else {
                "△△"
            }
        } else {
            if start_position {
                "△△"
            } else {
                "▽▽"
            }
        }
    }

    pub fn render_board(self: &Self) -> Vec<String> {
        let mut result = Vec::new();
        for r in 0..self.rows {
            let mut row_str = String::from("");
            for c in 0..self.cols {
                let tile_str = match self.grid[r][c] {
                    Tile::Wall => "██",
                    Tile::Rock => "██",
                    Tile::Start => self.create_arrows(true, Point { col: c, row: r }),
                    Tile::End => self.create_arrows(false, Point { col: c, row: r }),
                    Tile::Player => "🟥", // ◖◗
                    Tile::Ice => "  ",
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
        };
        steps
    }

    fn update_player_position(self: &mut Self, new_row: usize, new_col: usize) {
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

    pub fn respond_to_input(self: &'a mut Self, c: char) {
        let direction_opt = match c {
            'w' => Some(Direction::Up),
            's' => Some(Direction::Down),
            'a' => Some(Direction::Left),
            'd' => Some(Direction::Right),
            _ => None,
        };
        if let Some(direction) = direction_opt {
            let steps = self.steps_in_direction(&direction);
            for _ in 0..steps {
                self.move_queue.push_back(direction.clone());
            }
        }
    }

    pub fn move_player(self: &'a mut Self, dir: Direction) {
        match dir {
            Direction::Up => {
                self.update_player_position(self.player.pos.row - 1, self.player.pos.col)
            }
            Direction::Down => {
                self.update_player_position(self.player.pos.row + 1, self.player.pos.col)
            }
            Direction::Left => {
                self.update_player_position(self.player.pos.row, self.player.pos.col - 1)
            }
            Direction::Right => {
                self.update_player_position(self.player.pos.row, self.player.pos.col + 1)
            }
        };
    }

    // pub fn player_won(self: &Self) -> bool {
    //     self.player.pos == self.end.pos
    // }

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
