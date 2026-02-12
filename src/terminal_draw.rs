use ratatui::layout::{Alignment, Constraint, Layout, Margin, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::type_racer_game::TypeRacerGame;

pub(crate) fn draw(
    frame: &mut Frame,
    game: &TypeRacerGame,
    button_area_out: &mut Option<Rect>,
    timer_text_value: &str,
    wpm_value: u32,
    button_label: &str,
) {
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
    ])
        .split(content_area);

    let left_area = columns[0];
    let right_area = columns[1];

    let right_rows = Layout::vertical([
        Constraint::Length(7),
        Constraint::Length(3),
        Constraint::Length(5),
        Constraint::Min(0),
    ])
        .split(right_area);

    let timer_area = right_rows[0];
    let button_area = right_rows[1];
    let wpm_area = right_rows[2];

    *button_area_out = Some(button_area);

    render_sentence_and_input(frame, game, left_area);

    frame.render_widget(title(), title_area);
    frame.render_widget(
        timer_text(timer_text_value),
        timer_area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
    );
    frame.render_widget(button_text(button_label), button_area);
    frame.render_widget(wpm_text(wpm_value), wpm_area);
}

fn title() -> Paragraph<'static> {
    Paragraph::new(Line::from(Span::styled(
        "TypeRacer",
        Style::default().add_modifier(Modifier::BOLD),
    )))
        .alignment(Alignment::Center)
}

fn render_sentence_and_input(frame: &mut Frame, game: &TypeRacerGame, area: Rect) {
    let block = Block::default()
        .title("Sentence")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    frame.render_widget(block.clone(), area);

    let inner = block.inner(area);

    // Sentence line, then two blank lines, then input line.
    let rows = Layout::vertical([
        Constraint::Length(1), // sentence
        Constraint::Length(1), // blank
        Constraint::Length(1), // blank
        Constraint::Length(1), // input (3 lines below sentence)
        Constraint::Min(0),
    ])
        .split(inner);

    let max_cols = inner.width as usize;

    let target_line = truncate_to_cols(game.sentence(), max_cols);
    let input_line = truncate_to_cols(game.input(), max_cols);

    let target = Paragraph::new(target_line)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .wrap(Wrap { trim: true });

    let input = Paragraph::new(input_line)
        .style(Style::default().fg(Color::Yellow))
        .wrap(Wrap { trim: true });

    frame.render_widget(target, rows[0]);
    frame.render_widget(input, rows[3]);
}

fn truncate_to_cols(s: &str, max_cols: usize) -> String {
    if max_cols == 0 {
        return String::new();
    }
    s.chars().take(max_cols).collect()
}

fn timer_text(value: &str) -> Paragraph<'static> {
    let timer_block = Block::default()
        .title("Timer")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    Paragraph::new(Line::from(Span::styled(
        value.to_string(),
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )))
        .alignment(Alignment::Center)
        .block(timer_block)
}

fn button_text(label: &str) -> Paragraph<'static> {
    let start_block = Block::default().borders(Borders::ALL);
    Paragraph::new(Line::from(Span::styled(
        format!("[ {label} ]"),
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    )))
        .alignment(Alignment::Center)
        .block(start_block)
}

fn wpm_text(value: u32) -> Paragraph<'static> {
    let wpm_block = Block::default()
        .title("WPM")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    Paragraph::new(Line::from(Span::styled(
        value.to_string(),
        Style::default().add_modifier(Modifier::BOLD),
    )))
        .alignment(Alignment::Center)
        .block(wpm_block)
}