use crate::board::Board;
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
  
  Use WASD or the arrow keys to move.
  Press 'SPACE' to restart.
  Press 'V' or 'v' to change the view.
  Press 'Q' or 'Ctrl-C' to exit.

";
    let parts: Vec<String> = intro.split("\n").map(|x| String::from(x)).collect();
    parts
}

fn play_next_input_handler(play_again_signal: &mut bool) {
    // Poll without a delay.
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

pub fn start_game() {
    clear_terminal();
    let mut boards_solved: u16 = 0;
    // Generate next puzzle

    loop {
        let mut board = Board::solvable_random(12, 12, 15, 5);
        play_board(&mut board, boards_solved);

        let mut play_next_game = false;

        while !play_next_game {
            play_next_input_handler(&mut play_next_game)
        }

        // clear_terminal();
        boards_solved += 1;
    }
}

fn play_board(board: &mut Board, boards_solved: u16) {
    clear_terminal();

    let rc_board = Rc::new(RefCell::new(board));
    let render_callback = || {
        let mut output = get_introduction_section();

        let board_section = rc_board.borrow().render_board();
        output.extend(board_section);
        let pos = rc_board.borrow().player.pos;
        let boards_solved: String = format!("Levels solved: {}", boards_solved);
        let current_position: String = format!("Position: {}, {}", pos.col, pos.row);
        let move_queue: String = format!("Move Queue: {:?}", rc_board.borrow().move_queue);

        let mut winning_text = String::from("");
        // If the player has won, display this text.
        if rc_board.borrow().player_won() {
            winning_text = String::from("You won! Press 'Space' to play again.");
        }
        output.push(winning_text);
        output.push(String::from("\n"));
        output.push(boards_solved);
        output.push(current_position);
        output.push(move_queue);
        output
    };

    let input_handler = |key_code: KeyCode| {
        let mut board_mref = rc_board.borrow_mut();
        board_mref.respond_to_input(key_code)
    };

    let player_moves_iterator = iter::from_fn(|| {
        let board_ref = rc_board.clone();
        let mut board_mref = board_ref.borrow_mut();
        board_mref.process_move()
    });

    let is_game_over = || rc_board.borrow().player_has_won;

    let mut renderer = Renderer::new(
        render_callback,
        input_handler,
        player_moves_iterator,
        is_game_over,
        50,
    );

    renderer.render_scene();
}
