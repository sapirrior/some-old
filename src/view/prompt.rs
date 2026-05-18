use crate::app::App;
use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    style::Print,
    QueueableCommand,
};
use std::io::{self, Write};

pub fn draw_prompt(app: &App, prompt_char: char, input: &str) {
    let mut stdout = io::stdout();
    let _ = stdout.queue(cursor::MoveTo(0, app.terminal_rows.saturating_sub(1)));
    let _ = stdout.queue(Clear(ClearType::CurrentLine));
    let _ = stdout.queue(Print(format!("{}{}", prompt_char, input)));
    let _ = stdout.flush().unwrap();
}
