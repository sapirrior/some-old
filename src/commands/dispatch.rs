use crate::app::App;
use crate::input::Key;
use super::{nav, search, sys};

pub fn dispatch(app: &mut App, key: Key) {
    if app.search_failed { app.search_failed = false; }
    if app.search_wrapped { app.search_wrapped = false; }

    if app.show_help {
        match key {
            Key::Char('h') | Key::Esc => app.show_help = false,
            Key::Char('q') => app.running = false,
            _ => {}
        }
        return;
    }

    match key {
        Key::Up | Key::Char('k') => nav::nav_up(app),
        Key::Down | Key::Char('j') => nav::nav_down(app),
        Key::PageUp | Key::Char('b') => nav::nav_page_up(app),
        Key::PageDown | Key::Char(' ') | Key::Char('f') => nav::nav_page_down(app),
        Key::Char('u') => nav::nav_half_up(app),
        Key::Char('d') => nav::nav_half_down(app),
        Key::Home | Key::Char('g') | Key::Char('<') => nav::nav_home(app),
        Key::End | Key::Char('G') | Key::Char('>') => nav::nav_end(app),
        Key::Char('/') => search::search_forward(app),
        Key::Char('?') => search::search_backward(app),
        Key::Char('n') => search::search_next(app),
        Key::Char('N') => search::search_prev(app),
        Key::Char(':') => sys::sys_colon(app),
        Key::Char('h') => app.show_help = true,
        Key::Esc => app.last_pattern.clear(),
        Key::Char('q') => app.running = false,
        Key::Resize(cols, rows) => {
            app.terminal_cols = cols;
            app.terminal_rows = rows;
            app.layout.compute(&app.doc, cols);
        }
        _ => {}
    }
}
