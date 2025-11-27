use crossterm::{
    cursor,
    event::{Event, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    execute, queue, style,
    terminal::{self, enable_raw_mode, ClearType},
};
use std::{
    io::{stdout, Result, Stdout, Write},
    iter::Iterator,
    thread,
    time::Duration,
};

use crate::system::{exit_game, respond_to_input};

pub struct Renderer<RenderFn, InputFn, MoveIterator, GameOver>
where
    RenderFn: Fn() -> Vec<String>,
    InputFn: FnMut(KeyCode),
    MoveIterator: Iterator,
    GameOver: Fn() -> bool,
{
    render_function: RenderFn,
    input_handler: InputFn,
    change_iterator: MoveIterator,
    game_over_function: GameOver,
    initial_render: bool,
    frame_delay_millis: u64,
}

impl<RenderFn, InputFn, MoveIterator, GameOver> Renderer<RenderFn, InputFn, MoveIterator, GameOver>
where
    RenderFn: Fn() -> Vec<String>,
    InputFn: FnMut(KeyCode),
    MoveIterator: Iterator,
    GameOver: Fn() -> bool,
{
    pub fn new(
        render_function: RenderFn,
        input_handler: InputFn,
        change_iterator: MoveIterator,
        game_over_function: GameOver,
        frame_delay_millis: u64,
    ) -> Self {
        Self {
            render_function,
            input_handler,
            change_iterator,
            game_over_function,
            initial_render: false,
            frame_delay_millis,
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
                (KeyCode::Char('c'), m) if m == KeyModifiers::CONTROL => exit_game(),
                (key_code, _) => (self.input_handler)(key_code),
            }
        }
    }

    fn render_next_frame(&mut self) -> bool {
        // call move iterator and return true if another frame should be rendered.
        self.change_iterator.next().is_some()
    }

    fn draw_frame(&self, stdout: &mut Stdout) -> Result<()> {
        execute!(stdout, terminal::Clear(ClearType::All))?;
        queue!(stdout, cursor::MoveTo(0, 0))?;

        let scene = (self.render_function)();

        for (row_index, line) in scene.iter().enumerate() {
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

        while !(self.game_over_function)() {
            let mut event_handler = |event: Event| self.key_input_handler(event);
            respond_to_input(&mut event_handler)?;

            if !self.initial_render || self.render_next_frame() {
                self.initial_render = true;
                self.draw_frame(&mut stdout)?;
            }
            thread::sleep(Duration::from_millis(self.frame_delay_millis));
        }
        Ok(())
    }
}
