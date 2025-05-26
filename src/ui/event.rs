use crossterm::event::{self, Event, KeyCode};
use std::{error::Error, time::Duration};

pub fn handle_events(app: &mut super::app::App) -> Result<bool, Box<dyn Error>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Up => app.move_up(),
                KeyCode::Down => app.move_down(),
                KeyCode::Backspace => app.backspace(),
                KeyCode::Char(c) if c.is_ascii_digit() => app.append_digit(c),
                _ => {}
            }
        }
    }
    Ok(false)
}
