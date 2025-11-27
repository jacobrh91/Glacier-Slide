use crate::board::Board;
use crate::game_state::GameState;
use crate::renderer::Renderer;
use crate::system::{clear_terminal, exit_game, respond_to_input};

use crossterm::event::{
    Event, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

pub fn get_introduction_section() -> Vec<String> {
    const INTRO: &str = r"Welcome to
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

    INTRO.lines().map(str::to_owned).collect()
}

fn play_next_input_handler(play_again_signal: &mut bool) {
    let mut event_handler = |event: Event| {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) = event
        {
            match (code, modifiers) {
                (KeyCode::Char('c'), m) if m == KeyModifiers::CONTROL => exit_game(),
                (KeyCode::Char('Q'), _) => exit_game(),
                (KeyCode::Char(' '), _) => *play_again_signal = true,
                _ => {}
            }
        }
    };

    respond_to_input(&mut event_handler);
}

pub fn start_game(game_state: GameState) {
    let game_config = game_state.config.clone();
    let rc_game_state = Rc::new(RefCell::new(game_state));

    loop {
        // Disable raw mode so puzzle generation and intro text print nicely.
        let _ = disable_raw_mode();
        clear_terminal();

        for line in get_introduction_section() {
            println!("{line}");
        }

        let mut board = Board::generate_solvable_board(&game_config);

        // Now that we have a board, prepare to play it.
        let _ = enable_raw_mode();
        board.attach_game_state(Rc::clone(&rc_game_state));

        play_board(&mut board);

        // After the level has been solved, reset the show-solution state.
        rc_game_state.borrow_mut().display_solution = false;

        let mut play_next_game = false;
        while !play_next_game {
            play_next_input_handler(&mut play_next_game);
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
        let borrowed_board = interior_mut_board.borrow();
        let game_state_opt = borrowed_board.game_state.as_ref();

        let mut output = Vec::<String>::new();

        //////////////////
        // Intro section
        //////////////////
        output.extend(get_introduction_section());

        /////////////////
        // Solved count
        /////////////////
        let solved_count = game_state_opt
            .map(|state| state.borrow().levels_solved)
            .unwrap_or(0);
        output.push(format!("Levels solved: {}", solved_count));
        output.push(String::new());

        //////////////////
        // Board section
        //////////////////
        output.extend(borrowed_board.render_board());

        if borrowed_board.player_won() {
            output.push("You won! Press 'Space' to play again.".to_string());
        }
        output.push(String::new());

        let (debug, player_gave_up) = game_state_opt
            .map(|state| {
                let game_state = state.borrow();
                (game_state.config.debug, game_state.display_solution)
            })
            .unwrap_or((false, false));

        ///////////////////////////////////////////////////////////////
        // Solution section (either for player-gave-up or debug mode)
        ///////////////////////////////////////////////////////////////
        if debug || player_gave_up {
            let solution_str = borrowed_board
                .solution
                .as_ref()
                .and_then(|solution| solution.get_solution_string())
                .unwrap_or_else(|| "Unknown".to_string());
            output.push(format!("Solution: {}", solution_str));
        }

        ////////////////////////////
        // Debug mode only section
        ////////////////////////////
        if debug {
            let pos = borrowed_board.player.0;
            output.push(format!("Position: {}, {}", pos.col, pos.row));

            let move_queue = format!("Move Queue: {:?}", borrowed_board.move_queue);
            output.push(move_queue);

            let edges_traversed = borrowed_board
                .solution
                .as_ref()
                .map(|solution| solution.edges_traversed.to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            output.push(format!(
                "Edges traversed to find solution: {}",
                edges_traversed
            ));
        }

        output
    };

    let input_handler = |key_code: KeyCode| {
        let mut board_mref = interior_mut_board.borrow_mut();
        board_mref.respond_to_input(key_code)
    };

    let board_ref_for_moves = Rc::clone(&interior_mut_board);
    let player_moves_iterator = iter::from_fn(move || {
        let mut board_ref = board_ref_for_moves.borrow_mut();
        board_ref.process_move()
    });

    let is_game_over = || interior_mut_board.borrow().player_has_won;

    let mut renderer = Renderer::new(
        render_callback,
        input_handler,
        player_moves_iterator,
        is_game_over,
        50,
    );

    renderer.render_scene();
}
