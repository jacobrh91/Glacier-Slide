use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use crate::{
    board::{point::Point, Board},
    direction::{Direction, Move, Slide},
};

pub struct Solver {
    pub board: Rc<RefCell<Board>>,
    visual_mode: bool, // If true, saves the graph traversal moves.
    pub move_record: VecDeque<Move>,
}

impl Solver {
    pub fn new(board: Rc<RefCell<Board>>) -> Self {
        Solver {
            board,
            visual_mode: false,
            move_record: VecDeque::new(),
        }
    }
    pub fn enable_visual_mode(self: &mut Self) {
        self.visual_mode = true;
        // self.move_record.push_back(Move::Reset);
    }

    pub fn solve(self: &mut Self) {
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
        clean_solutions.sort();
        clean_solutions.reverse();

        println!(
            "Solutions: {}\nSteps searched: {}",
            clean_solutions.len(),
            search_steps
        );
        if clean_solutions.len() > 0 {
            println!("Shortest: {}", clean_solutions.first().unwrap().len());
        }
        println!("\nSolution(s):");
        for s in clean_solutions {
            println!("  {}", s);
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
                    updated_moves.push(direction.clone());
                    // If in visual mode, record the movement.
                    if self.visual_mode {
                        self.move_record
                            .push_back(Move::MovePlayer(Slide::new(steps, direction.clone())));
                    }
                    // Move the player for the solver.
                    for _ in 0..steps {
                        self.board.borrow_mut().move_player(direction.clone());
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
            if self.visual_mode {
                // Record the reset of the player's position
                self.move_record
                    .push_back(Move::Teleport(current_position.clone()));
            }
            self.board
                .borrow_mut()
                .update_player_position(current_position.row, current_position.col);
        }
    }

    fn get_possible_moves(self: &Self, previous_move_opt: Option<&Direction>) -> Vec<Direction> {
        previous_move_opt
            .map(|previous_move| {
                // If previous move exist, next move must be in orthogonal direction.
                if *previous_move == Direction::Up || *previous_move == Direction::Down {
                    vec![Direction::Right, Direction::Left]
                } else {
                    vec![Direction::Up, Direction::Down]
                }
            })
            .unwrap_or_else(|| Vec::from(Direction::all()))
    }
}
