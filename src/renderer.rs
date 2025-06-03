use crossterm::{execute, terminal};

use crossterm::event::{
    Event, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
};
use crossterm::terminal::enable_raw_mode;
use crossterm::{cursor, queue, style};
use std::io::{stdout, Stdout, Write};
use std::iter::Iterator;
use std::thread;
use std::time::Duration;

use crate::system::{exit_game, respond_to_input};

pub struct Renderer<RenderFn, InputFn, MoveIterator, GameOver>
where
    RenderFn: FnMut() -> Vec<String>,
    InputFn: FnMut(KeyCode) -> (),
    MoveIterator: Iterator,
    GameOver: Fn() -> bool,
{
    sout: Stdout,
    render_function: RenderFn,
    input_handler: InputFn,
    change_iterator: MoveIterator,
    game_over_function: GameOver,
    initial_render: bool,
    frame_delay_millis: u64,
}

impl<RenderFn, InputFn, MoveIterator, GameOver> Renderer<RenderFn, InputFn, MoveIterator, GameOver>
where
    RenderFn: FnMut() -> Vec<String>,
    InputFn: FnMut(KeyCode) -> (),
    MoveIterator: Iterator,
    GameOver: Fn() -> bool,
{
    pub fn new(
        render_function: RenderFn,
        input_handler: InputFn,
        change_iterator: MoveIterator,
        game_over_function: GameOver,
        frame_delay_millis: u64,
    ) -> Renderer<RenderFn, InputFn, MoveIterator, GameOver> {
        Renderer {
            sout: stdout(),
            render_function,
            input_handler,
            change_iterator,
            game_over_function,
            initial_render: false,
            frame_delay_millis,
        }
    }

    fn key_input_handler(self: &mut Self, event: Event) {
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
                    keycode => (self.input_handler)(keycode),
                }
            }
        }
    }

    fn render_next_frame(self: &mut Self) -> bool {
        /*
        call move iterator, return true if another frame should be rendered
         */
        self.change_iterator.next().is_some()
    }

    pub fn render_scene(self: &mut Self) {
        enable_raw_mode().unwrap();

        while !(self.game_over_function)() {
            let mut event_handler = |event: Event| (self.key_input_handler(event));
            respond_to_input(&mut event_handler);

            if !self.initial_render || self.render_next_frame() {
                if !self.initial_render {
                    self.initial_render = true;
                }
                execute!(self.sout, terminal::Clear(terminal::ClearType::All)).unwrap();
                queue!(self.sout, cursor::MoveTo(0, 0)).unwrap();
                let scene: Vec<String> = (self.render_function)();

                for (idx, row) in scene.iter().enumerate() {
                    queue!(
                        self.sout,
                        style::Print(row),                 // Print row
                        cursor::MoveTo(0, idx as u16 + 1)  // Move to next row
                    )
                    .unwrap();
                }
                self.sout.flush().unwrap();
            }
            thread::sleep(Duration::from_millis(self.frame_delay_millis));
        }
    }
}
