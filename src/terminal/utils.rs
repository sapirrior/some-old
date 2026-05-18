use crossterm::terminal;

pub fn get_size() -> (u16, u16) {
    terminal::size().expect("Failed to get terminal size")
}
