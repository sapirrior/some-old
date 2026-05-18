pub enum Key {
    Char(char),
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Esc,
    Backspace,
    Enter,
    Resize(u16, u16),
    Unknown,
}
