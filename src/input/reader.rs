use super::keys::Key;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

pub fn read_key() -> Key {
    loop {
        match event::read() {
            Ok(event) => {
                if let Some(key) = parse_event(event) {
                    return key;
                }
            }
            Err(_) => return Key::Unknown,
        }
    }
}

pub fn poll_key(timeout: Duration) -> Key {
    if let Ok(true) = event::poll(timeout)
        && let Ok(event) = event::read()
        && let Some(key) = parse_event(event)
    {
        return key;
    }
    Key::Unknown
}

fn parse_event(event: Event) -> Option<Key> {
    match event {
        Event::Key(KeyEvent { code, .. }) => match code {
            KeyCode::Char(c) => Some(Key::Char(c)),
            KeyCode::Up => Some(Key::Up),
            KeyCode::Down => Some(Key::Down),
            KeyCode::PageUp => Some(Key::PageUp),
            KeyCode::PageDown => Some(Key::PageDown),
            KeyCode::Home => Some(Key::Home),
            KeyCode::End => Some(Key::End),
            KeyCode::Esc => Some(Key::Esc),
            KeyCode::Backspace => Some(Key::Backspace),
            KeyCode::Enter => Some(Key::Enter),
            _ => Some(Key::Unknown),
        },
        Event::Resize(mut cols, mut rows) => {
            // Drain subsequent resize events to prevent layout thrashing
            while let Ok(true) = event::poll(Duration::from_millis(10)) {
                if let Ok(Event::Resize(c, r)) = event::read() {
                    cols = c;
                    rows = r;
                } else {
                    break;
                }
            }
            Some(Key::Resize(cols, rows))
        }
        _ => None,
    }
}
