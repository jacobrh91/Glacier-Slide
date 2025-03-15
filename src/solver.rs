// /*

// Solve recursively
//   - player position
//   - rock position
//   - previous moves
//   - winning path
//   - need a cache table to keep track of where the player has been
// Need to reposition player as you recurse

// Save at each state? Or only when solution is found?

// {
//   "arena": {
//     ...
//   },
//   "rock_positions": {
//     // record solutions found along the way up to N rocks.

//   }

//   "solutions": [
//     {
//       "rock_positions",
//       "path",

//     }
//   ],

// }

// Save structure
//   - separated by start, end, and rock positions.
//   - Should probably store attempts too. As with bigger maps, we don't want to recompute
//     anything we've already explored

// */
// use crate::{board::Direction, Board, Point};
// // use board::Point;
// use std::{thread, time::Duration};

// struct Arena {
//     max_rocks: u8,
//     rows: usize,
//     cols: usize,
//     start_pos: Point,
//     end_pos: Point,
// }

// fn rec_search(b: Board, prev_positions: Vec<Point>, prev_steps: Vec<Direction>) {
//     if prev_positions.contains(&b.player.pos) {
//         // Back to location we have already been. Dead end.
//         ()
//     } else if b.player_won() {
//         println!("Solution {:?}. Rocks: {:?}", prev_steps, b.rocks);
//         thread::sleep(Duration::from_millis(100000));
//     }

//     for i in Direction::iter() {
//         if b.can_player_move(&i) {
//             let prev_positions_updated = prev_positions.clone();
//             prev_positions_updated.push(b.player.pos.clone());
//             let prev_steps_updated = prev_steps.clone();
//             prev_steps_updated.push(i.clone());
//             b.move_player(&i);
//             rec_search(b.clone(), prev_positions_updated, prev_steps_updated);
//         }
//     }
// }

// pub fn find_solutions(arena: Arena) -> &str {
//     b = Board::new(
//         arena.rows,
//         arena.cols,
//         arena.start_pos,
//         arena.end_pos,
//         Vec::<Point>::new(),
//     );
// }
