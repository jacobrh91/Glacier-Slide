use std::io::{self, Result};

use crossterm::event::Event;
use crossterm::event::{Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::terminal::disable_raw_mode;

use crate::board::Board;
use crate::game_state::GameState;
use crate::renderer::Renderer;
use crate::system::{clear_terminal, exit_game, respond_to_input};

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

fn play_next_input_handler(play_again_signal: &mut bool) -> io::Result<()> {
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

    respond_to_input(&mut event_handler)
}

pub fn start_game(mut game_state: GameState) -> Result<()> {
    loop {
        // During puzzle generation, disable raw mode so printing works normally.
        disable_raw_mode()?;
        clear_terminal()?;

        for line in get_introduction_section() {
            println!("{line}");
        }

        // Generate a new solvable board with the current config.
        let mut board = Board::generate_solvable_board(&game_state.config);

        // Run the main interactive loop for this board.
        {
            let mut renderer = Renderer::new(&mut board, &mut game_state, 50);
            renderer.render_scene()?;
        }

        // After the level finishes, update game_state.
        if board.player_has_won {
            game_state.levels_solved += 1;
        }
        game_state.display_solution = false;

        // Ask whether to play another level.
        let mut play_next = false;
        while !play_next {
            play_next_input_handler(&mut play_next)?;
        }
    }
}
