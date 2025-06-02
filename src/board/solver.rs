use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use crate::board::{point::Point, Board};

use super::direction::Direction;

pub struct Solver {
    pub board: Rc<RefCell<Board>>,
}

#[derive(Clone)]
pub struct Solution {
    pub solutions: Vec<String>,
}

impl Solution {
    pub fn is_solvable(self: &Self) -> bool {
        self.solutions.len() > 0
    }
}

impl Solver {
    pub fn new(board: Rc<RefCell<Board>>) -> Self {
        Solver { board }
    }

    pub fn from_board(board: &Board) -> Self {
        Solver::new(Rc::new(RefCell::new(board.clone())))
    }

    pub fn solve(self: &mut Self) -> Solution {
        let cache = HashSet::<Point>::new();
        let mut search_steps = Box::new(0u32);
        let mut solutions = Vec::new();

        let curr_position = self.board.borrow().start.pos.clone();

        self.solve_rec(
            Vec::new(),
            curr_position,
            cache,
            &mut solutions,
            &mut search_steps,
        );
        let mut clean_solutions: Vec<String> = solutions
            .into_iter()
            .map(|s| Direction::to_string(s))
            .collect::<Vec<_>>();
        clean_solutions.sort_by(|a, b| b.chars().count().cmp(&a.chars().count()));
        clean_solutions.reverse();

        Solution {
            solutions: clean_solutions,
        }
    }

    fn solve_rec(
        self: &mut Self,
        prev_moves: Vec<Direction>,
        curr_position: Point,
        mut cache: HashSet<Point>,
        solutions: &mut Vec<Vec<Direction>>,
        search_steps: &mut Box<u32>,
    ) {
        self.reset_player_position(&curr_position);

        if self.board.borrow().player_won() {
            solutions.push(prev_moves);
        } else if !cache.contains(&curr_position) {
            cache.insert(self.board.borrow().player.pos.clone());
            for direction in self.get_possible_moves(prev_moves.last()) {
                self.reset_player_position(&curr_position);
                let steps = self.board.borrow().steps_in_direction(&direction);
                if steps > 0 {
                    **search_steps += 1;
                    let mut updated_moves = prev_moves.clone();
                    updated_moves.push(direction);
                    // Move the player for the solver.
                    for _ in 0..steps {
                        self.board.borrow_mut().move_player(direction);
                    }
                    let curr_position = self.board.borrow().player.pos.clone();
                    self.solve_rec(
                        updated_moves,
                        curr_position,
                        cache.clone(),
                        solutions,
                        search_steps,
                    );
                }
            }
        }
    }

    fn reset_player_position(self: &mut Self, current_position: &Point) {
        if self.board.borrow().player.pos != *current_position {
            self.board
                .borrow_mut()
                .update_player_position(current_position.row, current_position.col);
        }
    }

    fn get_possible_moves(self: &Self, previous_move_opt: Option<&Direction>) -> Vec<Direction> {
        previous_move_opt
            .map(|previous_move| {
                // If the previous move exists, the next move must be in an orthogonal direction.
                if *previous_move == Direction::Up || *previous_move == Direction::Down {
                    vec![Direction::Right, Direction::Left]
                } else {
                    vec![Direction::Up, Direction::Down]
                }
            })
            .unwrap_or_else(|| Vec::from(Direction::all()))
    }
}
