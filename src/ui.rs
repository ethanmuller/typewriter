use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout}, 
    style::{Color, Style}, 
    text::{Line, Span}, 
    widgets::{Block, Padding, Paragraph}, 
    Frame
};

use crate::app::App;

fn render_hint(app: &mut App, frame: &mut Frame) {
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

    frame.render_widget(
        Paragraph::new("Each line will be printed onto paper as you type.\nPlease write continuously and freely.\nWhen you are done, press ESC to exit.")
            .alignment(Alignment::Left)
            .block(
                Block::new()
                    .padding(Padding::new(4, 2, 2, 4)),
            )
            .style(Style::default().fg(Color::DarkGray).bg(Color::White)),
        layout[0],
    );

    let three = Span::styled(app.three.clone(), Style::default().fg(Color::Gray));
    let two = Span::styled(app.two.clone(), Style::default().fg(Color::DarkGray));
    let one = Span::styled(app.one.clone(), Style::default().fg(Color::Black));

    let text: Vec<Line<'_>> = vec![three.into(), two.into(), one.into()];
    let widget = Paragraph::new(text)
        .alignment(Alignment::Left)
        .block(Block::default())
        .style(Style::default().fg(Color::DarkGray).bg(Color::White));

    frame.render_widget(widget, cols[1]);
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // Render the hint section
    render_hint(app, frame);
}
