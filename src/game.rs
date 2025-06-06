use crate::board::Board;
use crate::game_state::GameState;
use crate::renderer::Renderer;
use crate::system::{clear_terminal, exit_game, respond_to_input};
use crossterm::event::KeyModifiers;
use crossterm::event::{Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyEventState};

use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

pub fn get_introduction_section() -> Vec<String> {
    let intro = r"Welcome to
   ______ __              _                _____  __ _      __    
  / ____// /____ _ _____ (_)___   _____   / ___/ / /(_)____/ /___ 
 / / __ / // __ `// ___// // _ \ / ___/   \__ \ / // // __  // _ \
/ /_/ // // /_/ // /__ / //  __// /      ___/ // // // /_/ //  __/
\____//_/ \__,_/ \___//_/ \___//_/      /____//_//_/ \__,_/ \___/ 
  
  Use 'WASD' or the arrow keys to move.
  Press 'SPACE' to restart.
  Press 'V' or 'v' to change the view.
  Press 'G' or 'g' to give up and show the solution.
  Press 'Q' or 'Ctrl-C' to exit.
";
    let parts: Vec<String> = intro.split("\n").map(|x| String::from(x)).collect();
    parts
}

fn play_next_input_handler(play_again_signal: &mut bool) {
    let mut event_handler = |event: crossterm::event::Event| {
        if let Key(key_event) = event {
            if let KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } = key_event
            {
                match code {
                    KeyCode::Char(c) if c == 'c' && modifiers == KeyModifiers::CONTROL => {
                        exit_game();
                    }
                    KeyCode::Char('Q') => exit_game(),
                    KeyCode::Char('\u{0020}') => *play_again_signal = true,
                    _ => (),
                }
            }
        }
    };
    respond_to_input(&mut event_handler);
}

pub fn start_game(game_state: GameState) {
    clear_terminal();
    let game_config = game_state.config.clone();

    let rc_game_state = Rc::new(RefCell::new(game_state));

    loop {
        let mut board = Board::generate_solvable_board(game_config);
        board.attach_game_state(Rc::clone(&rc_game_state));

        play_board(&mut board);

        // After the level has been solved, reset the show solution state.
        rc_game_state.borrow_mut().display_solution = false;
        let mut play_next_game = false;
        while !play_next_game {
            play_next_input_handler(&mut play_next_game)
        }
    }
}

fn play_board(board: &mut Board) {
    clear_terminal();

    // Although the render function only needs read access to the board,
    //   the input handler and move iterator both need write access.
    // So, use the interior mutability pattern. https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
    let interior_mut_board = Rc::new(RefCell::new(board));

    let render_callback = || {
        let mut output = Vec::<String>::new();

        let borrowed_board = interior_mut_board.borrow();
        let game_state_opt = borrowed_board.game_state.as_ref();

        //////////////////
        // Intro section
        //////////////////
        let intro = get_introduction_section();
        output.extend(intro);

        /////////////////
        // Solved count
        /////////////////
        let solved_count: u16 = game_state_opt
            .map(|x| x.borrow().levels_solved)
            .unwrap_or_else(|| 0);
        let boards_solved: String = format!("Levels solved: {}", solved_count);

        output.push(boards_solved);
        output.push(String::from("\n"));

        //////////////////
        // Board section
        //////////////////
        let board_section = borrowed_board.render_board();
        output.extend(board_section);

        let mut winning_text = String::from("");
        if borrowed_board.player_won() {
            winning_text = String::from("You won! Press 'Space' to play again.");
        }
        output.push(winning_text);
        output.push(String::from("\n"));

        let (debug_mode, player_gave_up) = game_state_opt
            .map(|x| {
                let game_state = x.borrow();
                (game_state.debug_mode, game_state.display_solution)
            })
            .unwrap_or_else(|| (false, false));

        ///////////////////////////////////////////////////////////////
        // Solution section (either for player-gave-up or debug mode)
        ///////////////////////////////////////////////////////////////
        if debug_mode || player_gave_up {
            let solution_str = borrowed_board
                .get_solution_string()
                .unwrap_or_else(|| String::from("Unknown"));
            let solution: String = format!("Solution: {}", solution_str);
            output.push(solution);
        }

        ////////////////////////////
        // Debug mode only section
        ////////////////////////////
        if debug_mode {
            let pos = borrowed_board.player.pos;
            let current_position: String = format!("Position: {}, {}", pos.col, pos.row);
            output.push(current_position);
            let move_queue: String =
                format!("Move Queue: {:?}", interior_mut_board.borrow().move_queue);
            output.push(move_queue);
            // let edges_traversed = borrowed_board
            //     .solution
            //     .as_ref()
            //     .map(|x| x.edges_traversed.to_string())
            //     .unwrap_or_else(|| String::from("Unknown"));
            // output.push(format!(
            //     "Edges traversed to find solution: {}",
            //     edges_traversed
            // ))
        }
        output
    };

    let input_handler = |key_code: KeyCode| {
        let mut board_mref = interior_mut_board.borrow_mut();
        board_mref.respond_to_input(key_code)
    };

    let player_moves_iterator = iter::from_fn(|| {
        let board_ref = Rc::clone(&interior_mut_board);
        let mut board_mref = board_ref.borrow_mut();
        board_mref.process_move()
    });

    let is_game_over = || interior_mut_board.borrow().player_has_won;

    let mut renderer = Renderer::new(
        render_callback,
        input_handler,
        player_moves_iterator,
        is_game_over,
        50,
    );

    renderer.render_scene()
}
