use std::{error::Error, io};
use crossterm::{event::DisableMouseCapture, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod ui;
use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    // terminal setup
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // application_startup
    let mut app = App::new();
    let _ = App::run_app(&mut terminal, &mut app);

    // terminal restoration
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    // terminal.show_cursor()?;

    Ok(())
}

