use crate::app::App;
use crate::input::Key;

pub fn read_prompt(app: &App, prompt_char: char) -> String {
    let mut input = String::new();
    loop {
        crate::view::draw_prompt(app, prompt_char, &input);
        match crate::input::read_key() {
            Key::Enter => break,
            Key::Esc => { input.clear(); break; },
            Key::Backspace => { input.pop(); },
            Key::Char(c) => { input.push(c); },
            Key::Unknown => continue,
            _ => {}
        }
    }
    input
}

pub fn clamp_scroll(app: &mut App) {
    let view_h = app.terminal_rows.saturating_sub(1) as usize;
    let layout_count = app.layout.count();
    if app.scroll_y + view_h > layout_count {
        app.scroll_y = layout_count.saturating_sub(view_h);
    }
}
