use std::io::{self, stdout, Write};

use crossterm::{
    cursor::{MoveTo, Show},
    event::{poll, read, Event},
    execute,
    terminal::{disable_raw_mode, Clear, ClearType},
};

use std::process;

// Clear the current screen buffer and move the cursor to (0, 0).
pub fn clear_terminal() -> io::Result<()> {
    let mut out = stdout();
    execute!(out, Clear(ClearType::All), MoveTo(0, 0))?;
    out.flush()?;
    Ok(())
}

// This function never returns.
pub fn exit_game() -> ! {
    // Best-effort to restore terminal. Ignore errors because we're exiting anyway.
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), Show, Clear(ClearType::All), MoveTo(0, 0));

    process::exit(130);
}

// Non-blocking input polling helper that propagates I/O errors.
pub fn respond_to_input<F: FnMut(Event)>(event_handler: &mut F) -> io::Result<()> {
    if poll(std::time::Duration::from_millis(0))? {
        let ev = read()?;
        event_handler(ev);
    }
    Ok(())
}
