use std::{collections::HashSet, process, time::Duration};

use crate::{
    board::{point::Point, Board},
    direction::Direction,
    renderer::Renderer,
};

pub struct Solver<'a> {
    board: &'a mut Board,
}

struct Results {
    search_steps: Box<u64>,
    solutions: Vec<String>,
}

impl<'a> Solver<'a> {
    pub fn new(board: &'a mut Board) -> Self {
        Solver { board }
    }

    pub fn solve(self: &mut Self) {
        let cache = HashSet::<Point>::new();
        let mut search_steps = Box::new(0u32);
        let mut solutions = Vec::new();
        self.solve_rec(
            Vec::new(),
            self.board.start.pos.clone(),
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

        println!("Solutions: {}", clean_solutions.len());
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

        if self.board.player_won() {
            solutions.push(prev_moves);
        } else if !cache.contains(&curr_position) {
            cache.insert(self.board.player.pos.clone());
            for direction in self.get_possible_moves(prev_moves.last()) {
                self.reset_player_position(&curr_position);
                let steps = self.board.steps_in_direction(&direction);
                if steps > 0 {
                    **search_steps += 1;
                    let mut updated_moves = prev_moves.clone();
                    updated_moves.push(direction.clone());
                    for _ in 0..steps {
                        // Move player the required number of steps
                        self.board.move_player(direction.clone());
                    }
                    self.solve_rec(
                        updated_moves,
                        self.board.player.pos.clone(),
                        cache.clone(),
                        solutions,
                        search_steps,
                    );
                }
            }
        }
    }

    fn reset_player_position(self: &mut Self, current_position: &Point) {
        if self.board.player.pos != *current_position {
            self.board
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
