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
    blink_on: bool,
) {
    let root = Layout::vertical([
        Constraint::Length(3), // title
        Constraint::Min(0),    // main content
    ]).split(frame.area());

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

    *button_area_out = Some(button_area);

    render_sentence_and_input(frame, game, left_area, blink_on);

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
    ))).alignment(Alignment::Center)
}

fn render_sentence_and_input(frame: &mut Frame, game: &TypeRacerGame, area: Rect, blink_on: bool) {
    let block = Block::default().title("Sentence").title_alignment(Alignment::Center).borders(Borders::ALL);

    frame.render_widget(block.clone(), area);

    let inner = block.inner(area);

    let rows = Layout::vertical([
        Constraint::Length(1), // sentence
        Constraint::Length(1), // blank line
        Constraint::Length(1), // blank line
        Constraint::Length(1), // input from player
        Constraint::Min(0),
    ]).split(inner);

    let max_cols = inner.width as usize;
    let cursor = game.cursor_index();
    let has_error = game.has_error();
    let mut spans: Vec<Span<'static>> = Vec::new();
    
    for (i, ch) in game.sentence().chars().take(max_cols).enumerate() {
        let mut style = if i < cursor {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else if i == cursor && game.is_started() {
            if has_error {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            }
        } else {
            Style::default().fg(Color::White)
        };

        if i == cursor && game.is_started() && blink_on {
            style = style.add_modifier(Modifier::UNDERLINED);
        }

        spans.push(Span::styled(ch.to_string(), style));
    }

    let sentence_line = Paragraph::new(Line::from(spans)).wrap(Wrap { trim: true });
    frame.render_widget(sentence_line, rows[0]);

    let input_line = truncate_to_cols(game.input(), max_cols);
    let input = Paragraph::new(input_line).style(Style::default().fg(Color::Yellow)).wrap(Wrap { trim: true });

    frame.render_widget(input, rows[3]);
}

fn truncate_to_cols(s: &str, max_cols: usize) -> String {
    if max_cols == 0 {
        return String::new();
    }
    s.chars().take(max_cols).collect()
}

fn timer_text(value: &str) -> Paragraph<'static> {
    let timer_block = Block::default().title("Timer").title_alignment(Alignment::Center).borders(Borders::ALL);

    Paragraph::new(Line::from(Span::styled(
        value.to_string(),
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    ))).alignment(Alignment::Center).block(timer_block)
}

fn button_text(label: &str) -> Paragraph<'static> {
    let start_block = Block::default().borders(Borders::ALL);
    
    Paragraph::new(Line::from(Span::styled(
        format!("[ {label} ]"),
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    ))).alignment(Alignment::Center).block(start_block)
}

fn wpm_text(value: u32) -> Paragraph<'static> {
    let wpm_block = Block::default().title("WPM").title_alignment(Alignment::Center).borders(Borders::ALL);

    Paragraph::new(Line::from(Span::styled(
        value.to_string(), 
        Style::default().add_modifier(Modifier::BOLD)
    ))).alignment(Alignment::Center).block(wpm_block)
}