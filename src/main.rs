mod terminal_draw;
mod type_racer_game;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{event, execute};
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind,
};
use ratatui::layout::Rect;
use ratatui::DefaultTerminal;

use crate::terminal_draw::draw;
use crate::type_racer_game::TypeRacerGame;

const ROUND_SECS: u64 = 30;

fn main() -> io::Result<()> {
    let mut app = App::new();
    execute!(io::stdout(), EnableMouseCapture)?;
    let result = app.run();
    execute!(io::stdout(), DisableMouseCapture)?;
    ratatui::restore();
    result
}

struct App {
    game: TypeRacerGame,
    terminal: DefaultTerminal,
    should_quit: bool,
    button_area: Option<Rect>,

    deadline: Option<Instant>,
    remaining_secs: u64,
    last_wpm: u32,

    blink_on: bool,
    next_blink_at: Instant,
}

impl App {
    pub fn new() -> Self {
        Self {
            game: TypeRacerGame::new(),
            terminal: ratatui::init(),
            should_quit: false,
            button_area: None,

            deadline: None,
            remaining_secs: ROUND_SECS,
            last_wpm: 0,

            blink_on: true,
            next_blink_at: Instant::now() + Duration::from_millis(500),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        while !self.should_quit {
            self.update_blink();

            let timer_text = format_timer(self.remaining_secs);
            let button_label = if self.game.is_started() { "Stop" } else { "Start" };

            self.terminal.draw(|frame| {
                draw(
                    frame,
                    &self.game,
                    &mut self.button_area,
                    &timer_text,
                    self.last_wpm,
                    button_label,
                    self.blink_on,
                )
            })?;

            self.should_quit = self.keep_alive()?;
        }
        Ok(())
    }

    fn update_blink(&mut self) {
        let now = Instant::now();
        if now >= self.next_blink_at {
            self.blink_on = !self.blink_on;
            self.next_blink_at = now + Duration::from_millis(500);
        }
    }

    fn start_round(&mut self) {
        self.game.start();
        self.deadline = Some(Instant::now() + Duration::from_secs(ROUND_SECS));
        self.remaining_secs = ROUND_SECS;
        self.last_wpm = 0;
    }

    fn stop_round(&mut self) {
        self.game.stop();

        self.deadline = None;
        self.remaining_secs = ROUND_SECS;

        if let Some(wpm) = self.game.calculate_wpm() {
            self.last_wpm = wpm;
        }
    }

    fn update_timer(&mut self) {
        if !self.game.is_started() {
            return;
        }
        let Some(deadline) = self.deadline else {
            return;
        };

        let now = Instant::now();
        if now >= deadline {
            self.remaining_secs = 0;
            self.stop_round(); // force stop at 0
            return;
        }

        self.remaining_secs = deadline.duration_since(now).as_secs().min(ROUND_SECS);
    }

    fn keep_alive(&mut self) -> io::Result<bool> {
        let had_event = event::poll(Duration::from_millis(16))?;
        if had_event {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if self.game.is_started() {
                        match key.code {
                            KeyCode::Char(c) => {
                                if !key.modifiers.contains(event::KeyModifiers::CONTROL)
                                    && !key.modifiers.contains(event::KeyModifiers::ALT)
                                {
                                    self.game.push_char(c);
                                }
                            }
                            KeyCode::Backspace => self.game.backspace(),
                            KeyCode::Tab => self.game.push_char('\t'),
                            _ => {}
                        }

                        // Stop game when sentence matches
                        if self.game.input() == self.game.sentence() {
                            self.stop_round();
                        }

                        // Esc stops the game
                        if matches!(key.code, KeyCode::Esc) && self.game.is_started() {
                            self.stop_round();
                        }
                    } else {
                        match key.code {
                            KeyCode::Esc => return Ok(true),
                            KeyCode::Enter => self.start_round(),
                            _ => {}
                        }
                    }
                }

                Event::Mouse(mouse) => {
                    if matches!(mouse.kind, MouseEventKind::Down(_)) {
                        if let Some(area) = self.button_area {
                            let x = mouse.column;
                            let y = mouse.row;

                            let inside = x >= area.x
                                && x < area.x + area.width
                                && y >= area.y
                                && y < area.y + area.height;

                            if inside {
                                if self.game.is_started() {
                                    self.stop_round();
                                } else {
                                    self.start_round();
                                }
                            }
                        }
                    }
                }

                _ => {}
            }
        }

        self.update_timer();
        Ok(false)
    }
}

fn format_timer(secs: u64) -> String {
    let m = secs / 60;
    let s = secs % 60;
    format!("{m}:{s:02}")
}