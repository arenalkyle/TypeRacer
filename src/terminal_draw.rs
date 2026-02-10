use ratatui::Frame;
use ratatui::prelude::{Line, Modifier, Span, Style, Color};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::layout::{Alignment, Constraint, Layout, Margin};

pub(crate) fn draw(frame: &mut Frame) {

    let root = Layout::vertical([
        Constraint::Length(3), // title
        Constraint::Min(0),    // main content
    ])
        .split(frame.area());

    let title_area = root[0];
    let content_area = root[1];

    let columns = Layout::horizontal([
        Constraint::Percentage(70), // left
        Constraint::Percentage(30), // right
    ]).split(content_area);

    let left_area = columns[0];
    let right_area = columns[1];

    let right_rows = Layout::vertical([
        Constraint::Length(7),
        Constraint::Length(3),
        Constraint::Length(5),
        Constraint::Min(0),
    ]).split(right_area);

    let timer_area = right_rows[0];
    let button_area = right_rows[1];
    let wpm_area = right_rows[2];

    frame.render_widget(sentence_text(), left_area);
    frame.render_widget(title(), title_area);
    frame.render_widget(timer_text(), timer_area.inner(Margin { vertical: 1, horizontal: 1 }));
    frame.render_widget(button_text(), button_area);
    frame.render_widget(wpm_text(), wpm_area);

}

fn title() -> Paragraph<'static> {
    let title = Paragraph::new(Line::from(Span::styled(
        "TypeRacer",
        Style::default().add_modifier(Modifier::BOLD),
    )))
        .alignment(Alignment::Center);
    title
}

fn sentence_text() -> Paragraph<'static> {
    let sentence_block = Block::default()
        .title("Sentence")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let sentence_text = Paragraph::new("Put your sentence here...")
        .wrap(Wrap { trim: true })
        .block(sentence_block);
    sentence_text
}

fn timer_text() -> Paragraph<'static> {
    let timer_block = Block::default().title("Timer").title_alignment(Alignment::Center).borders(Borders::ALL);
    let timer_text = Paragraph::new(Line::from(Span::styled(
        "0:30",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )))
        .alignment(Alignment::Center)
        .block(timer_block);
    timer_text
}

fn button_text() -> Paragraph<'static> {
    let start_block = Block::default().borders(Borders::ALL);
    let start_text = Paragraph::new(Line::from(Span::styled(
        "[ Start ]",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )))
        .alignment(Alignment::Center)
        .block(start_block);
    start_text
}

fn wpm_text() -> Paragraph<'static> {
    let wpm_block = Block::default().title("WPM").title_alignment(Alignment::Center).borders(Borders::ALL);
    let wpm_text = Paragraph::new(Line::from(Span::styled(
        "0",
        Style::default().add_modifier(Modifier::BOLD),
    )))
        .alignment(Alignment::Center)
        .block(wpm_block);
    wpm_text
}