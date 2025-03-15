use crossterm::{execute, terminal};

use crossterm::event::{
    poll, read, Event, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, queue, style};
use std::io::{stdout, Stdout, Write};
use std::iter::Iterator;
use std::process;
use std::time::Duration;

pub struct Renderer<S, T, U>
where
    S: Fn() -> Vec<String>,
    T: FnMut(char) -> (), // responds to input by changing state
    U: Iterator,          // in practice, responds to actions queued by T
{
    sout: Stdout,
    render_function: S,
    input_handler: T,
    change_iterator: U,
    initial_render: bool,
}

impl<S: Fn() -> Vec<String>, T: FnMut(char) -> (), U: Iterator> Renderer<S, T, U> {
    pub fn new(render_function: S, input_handler: T, iterator: U) -> Renderer<S, T, U> {
        Renderer {
            sout: stdout(),
            render_function: render_function,
            input_handler: input_handler,
            change_iterator: iterator,
            initial_render: false,
        }
    }

    fn key_input_handler(self: &mut Self, event: Event) {
        if let Key(key_event) = event {
            if let KeyEvent {
                code: KeyCode::Char(c),
                modifiers,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } = key_event
            {
                if c == 'c' && modifiers == KeyModifiers::CONTROL {
                    disable_raw_mode().unwrap();
                    process::exit(130);
                } else {
                    (self.input_handler)(c);
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
        loop {
            match poll(Duration::from_millis(100)) {
                Ok(input_found) => {
                    if input_found {
                        read().ok().map(|event| {
                            self.key_input_handler(event);
                        });
                    }
                }
                Err(_) => (),
            };
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
        }
    }
}
