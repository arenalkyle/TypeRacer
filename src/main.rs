mod type_racer_game;
mod terminal_draw;

use std::io;
use std::time::Duration;
use crossterm::{event, execute};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind};
use ratatui::DefaultTerminal;
use ratatui::layout::Rect;
use crate::terminal_draw::draw;
use crate::type_racer_game::TypeRacerGame;

fn main() -> io::Result<()>  {
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
    button_area: Option<Rect>
}

impl App {
    pub fn new() -> Self{
        Self {
            game: TypeRacerGame::new(),
            terminal: ratatui::init(),
            should_quit: false,
            button_area: None
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        while !self.should_quit {
            self.terminal.draw(draw)?;
            self.keep_alive()?;
        }
        Ok(())
    }

    fn keep_alive(&mut self) -> io::Result<bool> {
        if !event::poll(Duration::from_millis(16))? {
            return Ok(false);
        }

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Esc => {
                    if self.game.is_started() {
                        self.game.stop();
                        Ok(false)
                    } else {
                        Ok(true)
                    }
                },
                KeyCode::Enter => {
                    if self.game.is_started() {
                        self.game.stop();
                    } else {
                        self.game.start();
                    }
                    Ok(false)
                },
                _ => Ok(false),
            },
            Event::Mouse(mouse) => {
                if matches!(mouse.kind, MouseEventKind::Down(_)) {
                    if let Some(area) = self.button_area {
                        let x = mouse.column;
                        let y = mouse.row;

                        let inside = x>= area.x
                            && x < area.x + area.width
                            && y >= area.y
                            && y < area.y + area.height;

                        if inside {
                            if self.game.is_started() {
                                self.game.stop();
                            } else {
                                self.game.start();
                            }
                        }
                    }
                }
                Ok(false)
            }
            _ => Ok(false)
        }
    }
}
