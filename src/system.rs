use std::{
    io::{stdout, Write},
    process,
    time::Duration,
};

use crossterm::{
    cursor, queue,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};

pub fn exit_game() {
    // Without this, the terminal gets messed up after the program ends.
    disable_raw_mode().unwrap();
    process::exit(130);
}

pub fn clear_terminal() {
    enable_raw_mode().unwrap();
    queue!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    queue!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    stdout().flush().unwrap();
    disable_raw_mode().unwrap();
}

pub fn respond_to_input<EventHandler: FnMut(crossterm::event::Event) -> ()>(
    event_handler: &mut EventHandler,
) {
    // Do not delay when polling. It is simply a way to get
    // the input handling logic to not block the thread.
    match crossterm::event::poll(Duration::from_millis(0)) {
        Ok(input_found) => {
            if input_found {
                crossterm::event::read()
                    .ok()
                    .map(|event| event_handler(event));
            }
        }
        Err(_) => (),
    };
}
