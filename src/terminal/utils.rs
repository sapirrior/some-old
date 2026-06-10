use crate::errors::SomeError;
use crossterm::terminal;

pub fn get_size() -> Result<(u16, u16), SomeError> {
    terminal::size()
        .map_err(|e| SomeError::Terminal(format!("Failed to get terminal size: {}", e)))
}
