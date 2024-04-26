use ratatui::{
    layout::{Constraint, Direction, Layout}, style::{Color, Style}, text::{Line, Span, Text}, widgets::{Block, Borders, Paragraph}, Frame
};

use crate::app::{App, CurrentScreen};



pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "el_doctor",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    let current_screen_text = vec![
        match app.current_screen {
            CurrentScreen::Main => {
                Span::styled(" Main", Style::default().fg(Color::Green))
            }
            CurrentScreen::Exiting => {
                Span::styled(" Exiting", Style::default().fg(Color::LightRed))
            }
        }
        .to_owned(),
    ];

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
             Constraint::Percentage(50),
             Constraint::Percentage(50),])
        .split(chunks[2]);

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "<q> to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "<q> to confirm",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL));

    let mode_footer = Paragraph::new(Line::from(current_screen_text))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);
}
