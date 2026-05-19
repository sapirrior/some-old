use crate::errors::InklessError;
use crossterm::terminal;

pub fn get_size() -> Result<(u16, u16), InklessError> {
    terminal::size()
        .map_err(|e| InklessError::Terminal(format!("Failed to get terminal size: {}", e)))
}
