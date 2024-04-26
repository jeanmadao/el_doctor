use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};

use crate::ui::ui;

pub enum CurrentScreen {
    Main,
    Exiting,
}

pub struct App {
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
        }
    }
    pub fn run_app<B: Backend>(
        terminal: &mut Terminal<B>,
        app: &mut App,
    ) -> io::Result<bool> {
        loop {
            terminal.draw(|f| ui(f, app))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Exiting;
                        }
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => {
                            return Ok(true);
                        }
                        KeyCode::Char('n') | KeyCode::Char('q') => {
                            return Ok(false);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
