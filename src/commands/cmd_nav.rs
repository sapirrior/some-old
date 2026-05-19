use super::cmd_utils::clamp_scroll;
use crate::app::App;

pub fn nav_up(app: &mut App) {
    app.follow_mode = false;
    if app.scroll_y > 0 {
        app.scroll_y -= 1;
    }
}

pub fn nav_down(app: &mut App) {
    app.follow_mode = false;
    app.scroll_y += 1;
    clamp_scroll(app);
}

pub fn nav_page_up(app: &mut App) {
    app.follow_mode = false;
    let view_h = app.terminal_rows.saturating_sub(1) as usize;
    app.scroll_y = app.scroll_y.saturating_sub(view_h);
}

pub fn nav_page_down(app: &mut App) {
    app.follow_mode = false;
    let view_h = app.terminal_rows.saturating_sub(1) as usize;
    app.scroll_y += view_h;
    clamp_scroll(app);
}

pub fn nav_half_up(app: &mut App) {
    app.follow_mode = false;
    let view_h = app.terminal_rows.saturating_sub(1) as usize;
    app.scroll_y = app.scroll_y.saturating_sub(view_h / 2);
}

pub fn nav_half_down(app: &mut App) {
    app.follow_mode = false;
    let view_h = app.terminal_rows.saturating_sub(1) as usize;
    app.scroll_y += view_h / 2;
    clamp_scroll(app);
}

pub fn nav_home(app: &mut App) {
    app.follow_mode = false;
    app.scroll_y = 0;
}

pub fn nav_end(app: &mut App) {
    app.follow_mode = false;
    app.scroll_y = app.layout.count();
    clamp_scroll(app);
}

pub fn toggle_follow(app: &mut App) {
    app.follow_mode = !app.follow_mode;
    if app.follow_mode {
        let view_h = app.terminal_rows.saturating_sub(1) as usize;
        app.scroll_y = app.layout.display_lines.len().saturating_sub(view_h);
    }
}
