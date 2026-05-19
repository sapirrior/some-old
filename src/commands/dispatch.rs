use super::{cmd_nav, cmd_search, cmd_sys};
use crate::app::App;
use crate::input::Key;

pub fn dispatch(app: &mut App, key: Key) {
    if app.search_failed {
        app.search_failed = false;
    }
    if app.search_wrapped {
        app.search_wrapped = false;
    }

    // Clear error message on next key press, unless it's a resize event
    if let Key::Resize(_, _) = key {
    } else {
        app.error_message = None;
    }

    if app.show_help {
        match key {
            Key::Char('h') | Key::Esc => app.show_help = false,
            Key::Char('q') => app.running = false,
            _ => {}
        }
        return;
    }

    match key {
        Key::Up | Key::Char('k') => cmd_nav::nav_up(app),
        Key::Down | Key::Char('j') => cmd_nav::nav_down(app),
        Key::PageUp | Key::Char('b') => cmd_nav::nav_page_up(app),
        Key::PageDown | Key::Char('f') => cmd_nav::nav_page_down(app),
        Key::Char('u') => cmd_nav::nav_half_up(app),
        Key::Char('d') => cmd_nav::nav_half_down(app),
        Key::Home | Key::Char('g') => cmd_nav::nav_home(app),
        Key::End | Key::Char('G') => cmd_nav::nav_end(app),
        Key::Char('F') => cmd_nav::toggle_follow(app),
        Key::Char('/') => cmd_search::search_forward(app),
        Key::Char('?') => cmd_search::search_backward(app),
        Key::Char('n') => cmd_search::search_next(app),
        Key::Char('N') => cmd_search::search_prev(app),
        Key::Char(':') => cmd_sys::sys_colon(app),
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
