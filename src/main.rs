mod type_racer_game;
mod terminal_draw;

use std::io;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use crate::terminal_draw::draw;

fn main() -> io::Result<()>  {
    let mut terminal = ratatui::init();
    let result = load_app(&mut terminal);
    ratatui::restore();
    result
}

fn load_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(draw)?;
        if keep_alive()? {
            break Ok(());
        }
    }
}

fn keep_alive() -> io::Result<bool> {
    if !event::poll(Duration::from_millis(16))? {
        return Ok(false);
    }

    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Esc => {
                // Stop game if started, otherwise close.
                Ok(true)
            },
            KeyCode::Enter => {
                // Start Game
                Ok(false)
            },
            _ => Ok(false),
        },
        _ => Ok(false),
    }
}
