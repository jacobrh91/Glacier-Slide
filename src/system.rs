use std::{
    io::{stdout, Write},
    process,
    time::Duration,
};

use crossterm::{
    cursor::{MoveTo, Show},
    execute,
    terminal::{disable_raw_mode, Clear, ClearType},
};

pub fn exit_game() {
    // Always restore raw mode first
    let _ = disable_raw_mode();

    // Clean visible screen
    let _ = execute!(
        stdout(),
        Clear(ClearType::All),
        Show,
        crossterm::cursor::MoveTo(0, 0)
    );

    // Ensure everything is flushed
    let _ = stdout().flush();

    // Exit safely
    process::exit(130);
}

pub fn clear_terminal() {
    let mut out = stdout();
    let _ = execute!(out, Clear(ClearType::All), MoveTo(0, 0));
    let _ = out.flush();
}

pub fn respond_to_input<F: FnMut(crossterm::event::Event)>(event_handler: &mut F) {
    // Do not delay when polling. It is simply a way to get
    // the input handling logic to not block the thread.
    if let Ok(input_found) = crossterm::event::poll(Duration::from_millis(0)) {
        if input_found {
            crossterm::event::read().ok().map(event_handler);
        }
    }
}
