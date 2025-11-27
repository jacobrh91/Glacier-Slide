use std::{
    io::{stdout, Result, Stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{Event, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    execute, queue, style,
    terminal::{self, enable_raw_mode, ClearType},
};

use crate::{
    board::Board,
    game::get_introduction_section,
    game_state::GameState,
    system::{exit_game, respond_to_input},
};

pub struct Renderer<'a> {
    board: &'a mut Board,
    game_state: &'a mut GameState,
    frame_delay_millis: u64,
    initial_render: bool,
    force_rerender: bool,
}

impl<'a> Renderer<'a> {
    pub fn new(
        board: &'a mut Board,
        game_state: &'a mut GameState,
        frame_delay_millis: u64,
    ) -> Self {
        Self {
            board,
            game_state,
            frame_delay_millis,
            initial_render: false,
            force_rerender: false,
        }
    }

    fn key_input_handler(&mut self, event: Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) = event
        {
            match (code, modifiers) {
                // Ctrl-C exits immediately
                (KeyCode::Char('c'), m) if m == KeyModifiers::CONTROL => exit_game(),

                // View toggle: player-focused vs full-board
                (KeyCode::Char('v') | KeyCode::Char('V'), _) => {
                    self.game_state.player_focused_view = !self.game_state.player_focused_view;
                    self.force_rerender = true;
                }

                // Give up: show solution immediately
                (KeyCode::Char('g') | KeyCode::Char('G'), _) => {
                    self.game_state.display_solution = true;
                    self.force_rerender = true;
                }

                // Game-level exit
                (KeyCode::Char('Q'), _) => exit_game(),

                // All other keys go to the board movement logic
                (other, _) => self.board.respond_to_input(other),
            }
        }
    }

    fn step_animation(&mut self) -> bool {
        // Advance one step of any queued slide; returns true if something moved
        self.board.process_move().is_some()
    }

    fn draw_frame(&self, stdout: &mut Stdout) -> Result<()> {
        execute!(stdout, terminal::Clear(ClearType::All))?;
        queue!(stdout, cursor::MoveTo(0, 0))?;

        let mut lines = Vec::<String>::new();

        // Intro
        lines.extend(get_introduction_section());

        // Levels solved
        lines.push(format!("Levels solved: {}", self.game_state.levels_solved));
        lines.push(String::new());

        // Board view (player-focused or full-board)
        lines.extend(self.board.render_board(self.game_state.player_focused_view));

        // Win text
        if self.board.player_won() {
            lines.push("You won! Press 'Space' to play again.".to_string());
        }
        lines.push(String::new());

        let debug = self.game_state.config.debug;
        let player_gave_up = self.game_state.display_solution;

        // Solution section
        if debug || player_gave_up {
            let solution_str = self
                .board
                .solution
                .as_ref()
                .and_then(|s| s.get_solution_string())
                .unwrap_or_else(|| "Unknown".to_string());
            lines.push(format!("Solution: {}", solution_str));
        }

        // Debug-only section
        if debug {
            let pos = self.board.player.0;
            lines.push(format!("Position: {}, {}", pos.col, pos.row));
            lines.push(format!("Move Queue: {:?}", self.board.move_queue));

            let edges_traversed = self
                .board
                .solution
                .as_ref()
                .map(|s| s.edges_traversed.to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            lines.push(format!(
                "Edges traversed to find solution: {}",
                edges_traversed
            ));
        }

        // Render all lines
        for (row_index, line) in lines.iter().enumerate() {
            queue!(
                stdout,
                style::Print(line),
                cursor::MoveTo(0, row_index as u16 + 1)
            )?;
        }

        stdout.flush()?;
        Ok(())
    }

    pub fn render_scene(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = stdout();

        while !self.board.player_has_won {
            // Handle input (non-blocking, propagates I/O errors)
            {
                let mut handler = |event| self.key_input_handler(event);
                respond_to_input(&mut handler)?;
            }

            // Advance sliding animations (if any)
            let advanced_animation = self.step_animation();

            let needs_rerender = self.force_rerender || !self.initial_render || advanced_animation;

            if needs_rerender {
                self.force_rerender = false;
                self.initial_render = true;
                self.draw_frame(&mut stdout)?;
            }

            thread::sleep(Duration::from_millis(self.frame_delay_millis));
        }

        Ok(())
    }
}
