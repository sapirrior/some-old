use crossterm::event::{self, Event, KeyCode, KeyEvent};
use super::keys::Key;

pub fn read_key() -> Key {
    if event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) {
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                KeyCode::Char(c) => Key::Char(c),
                KeyCode::Up => Key::Up,
                KeyCode::Down => Key::Down,
                KeyCode::PageUp => Key::PageUp,
                KeyCode::PageDown => Key::PageDown,
                KeyCode::Home => Key::Home,
                KeyCode::End => Key::End,
                KeyCode::Esc => Key::Esc,
                KeyCode::Backspace => Key::Backspace,
                KeyCode::Enter => Key::Enter,
                _ => Key::Unknown,
            },
            Ok(Event::Resize(cols, rows)) => Key::Resize(cols, rows),
            _ => Key::Unknown,
        }
    } else {
        Key::Unknown
    }
}
