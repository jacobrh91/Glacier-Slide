use crossterm::{execute, terminal};

use crossterm::event::{
    poll, read, Event, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, queue, style};
use std::io::{stdout, Stdout, Write};
use std::iter::Iterator;
use std::process;
use std::thread;
use std::time::Duration;
pub struct Renderer<RenderFn, InputFn, MoveIterator>
where
    RenderFn: Fn() -> Vec<String>,
    InputFn: FnMut(KeyCode) -> (),
    MoveIterator: Iterator,
{
    sout: Stdout,
    render_function: RenderFn,
    input_handler: InputFn,
    change_iterator: MoveIterator,
    initial_render: bool,
    frame_delay_millis: u64,
}

impl<RenderFn, InputFn, MoveIterator> Renderer<RenderFn, InputFn, MoveIterator>
where
    RenderFn: Fn() -> Vec<String>,
    InputFn: FnMut(KeyCode) -> (),
    MoveIterator: Iterator,
{
    pub fn new(
        render_function: RenderFn,
        input_handler: InputFn,
        change_iterator: MoveIterator,
        frame_delay_millis: u64,
    ) -> Renderer<RenderFn, InputFn, MoveIterator> {
        Renderer {
            sout: stdout(),
            render_function,
            input_handler,
            change_iterator,
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
                        disable_raw_mode().unwrap();
                        process::exit(130);
                    }
                    keycode => (self.input_handler)(keycode),
                }
                // if let KeyCode::Char(c) = code {

                // }
            }
            // {
            //     if c == 'c' && modifiers == KeyModifiers::CONTROL {
            //       KeyCode::
            //         disable_raw_mode().unwrap();
            //         process::exit(130);
            //     } else {
            //         (self.input_handler)(c);
            //     }
            // }
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
            // Do not delay when polling. It is simply a way to get
            // the input handling logic to not block the thread.
            match poll(Duration::from_millis(0)) {
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
            thread::sleep(Duration::from_millis(self.frame_delay_millis));
        }
    }
}
