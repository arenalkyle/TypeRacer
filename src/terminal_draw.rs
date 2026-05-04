use ratatui::layout::{Alignment, Constraint, Layout, Margin, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::type_racer_game::TypeRacerGame;

pub struct TerminalDraw;

impl TerminalDraw {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(
        &self,
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

        self.render_sentence_and_input(frame, game, left_area, blink_on);

        frame.render_widget(self.title(), title_area);
        frame.render_widget(
            self.timer_text(timer_text_value),
            timer_area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
        );
        frame.render_widget(self.button_text(button_label), button_area);
        frame.render_widget(self.wpm_text(wpm_value), wpm_area);
    }

    fn title(&self) -> Paragraph<'static> {
        Paragraph::new(Line::from(Span::styled(
            "TypeRacer",
            Style::default().add_modifier(Modifier::BOLD),
        ))).alignment(Alignment::Center)
    }

    fn render_sentence_and_input(&self, frame: &mut Frame, game: &TypeRacerGame, area: Rect, blink_on: bool) {
        let block = Block::default().title("Sentence").title_alignment(Alignment::Center).borders(Borders::ALL);

        frame.render_widget(block.clone(), area);

        let inner = block.inner(area);
        let width = inner.width as usize;

        let sentence_lines = self.wrapped_line_count(game.sentence(), width).max(1);
        let input_lines = self.wrapped_line_count(game.input(), width).max(1);

        let rows = Layout::vertical([
            Constraint::Length(sentence_lines as u16), // sentence
            Constraint::Length(1), // blank line
            Constraint::Length(input_lines as u16), // input from player
            Constraint::Min(0),
        ])
            .split(inner);

        let cursor = game.cursor_index();
        let has_error = game.has_error();
        let mut spans: Vec<Span<'static>> = Vec::new();

        for (i, ch) in game.sentence().chars().enumerate() {
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

        let sentence_line = Paragraph::new(Line::from(spans))
            .wrap(Wrap { trim: false });
        frame.render_widget(sentence_line, rows[0]);

        let input = Paragraph::new(game.input().to_string())
            .style(Style::default().fg(Color::Yellow))
            .wrap(Wrap { trim: false });

        frame.render_widget(input, rows[2]);
    }

    fn wrapped_line_count(&self, text: &str, width: usize) -> usize {
        if width == 0 {
            return 1;
        }

        let mut line_count = 0usize;

        for raw_line in text.lines() {
            let len = raw_line.chars().count();
            let wrapped = (len + width.saturating_sub(1)) / width;
            line_count += wrapped.max(1);
        }

        line_count.max(1)
    }

    fn timer_text(&self, value: &str) -> Paragraph<'static> {
        let timer_block = Block::default().title("Timer").title_alignment(Alignment::Center).borders(Borders::ALL);

        Paragraph::new(Line::from(Span::styled(
            value.to_string(),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ))).alignment(Alignment::Center).block(timer_block)
    }

    fn button_text(&self, label: &str) -> Paragraph<'static> {
        let start_block = Block::default().borders(Borders::ALL);

        Paragraph::new(Line::from(Span::styled(
            format!("[ {label} ]"),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ))).alignment(Alignment::Center).block(start_block)
    }

    fn wpm_text(&self, value: u32) -> Paragraph<'static> {
        let wpm_block = Block::default().title("WPM").title_alignment(Alignment::Center).borders(Borders::ALL);

        Paragraph::new(Line::from(Span::styled(
            value.to_string(),
            Style::default().add_modifier(Modifier::BOLD)
        ))).alignment(Alignment::Center).block(wpm_block)
    }
}