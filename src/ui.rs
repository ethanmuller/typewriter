use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout}, 
    style::{Color, Style}, 
    text::{Line, Span}, 
    widgets::{Block, Padding, Paragraph}, 
    Frame
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let size = frame.size();

    // Render a full-screen block with a white background
    frame.render_widget(
        Block::default()
            .style(Style::default().bg(Color::White)),
        size,
    );

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(size);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(1),
            Constraint::Length(32),
            Constraint::Min(1),
        ])
        .split(layout[1]);

    if app.show_hint {
        frame.render_widget(
            Paragraph::new("Please write continuously and freely.\nWhen you are done, press ESC to exit.")
            .alignment(Alignment::Left)
            .block(
                Block::new()
                .padding(Padding::new(4, 2, 2, 4)),
            )
            .style(Style::default().fg(Color::DarkGray).bg(Color::White)),
            layout[0],
        );
    }

    let history = Span::styled(app.history.clone(), Style::default().fg(Color::Gray));
    let printed = Span::styled(app.printed.clone(), Style::default().fg(Color::DarkGray));
    let input = Span::styled(app.input.clone(), Style::default().fg(Color::Black));

    let text: Vec<Line<'_>> = vec![history.into(), printed.into(), input.into()];
    let widget = Paragraph::new(text)
        .alignment(Alignment::Left)
        .block(Block::default())
        .style(Style::default().fg(Color::DarkGray).bg(Color::White));

    frame.render_widget(widget, cols[1]);
}
