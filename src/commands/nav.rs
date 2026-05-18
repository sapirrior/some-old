use crate::app::App;
use super::utils::clamp_scroll;

pub fn nav_up(app: &mut App) {
    if app.scroll_y > 0 { app.scroll_y -= 1; }
}

pub fn nav_down(app: &mut App) {
    app.scroll_y += 1;
    clamp_scroll(app);
}

pub fn nav_page_up(app: &mut App) {
    let view_h = app.terminal_rows.saturating_sub(1) as usize;
    app.scroll_y = app.scroll_y.saturating_sub(view_h);
}

pub fn nav_page_down(app: &mut App) {
    let view_h = app.terminal_rows.saturating_sub(1) as usize;
    app.scroll_y += view_h;
    clamp_scroll(app);
}

pub fn nav_half_up(app: &mut App) {
    let view_h = app.terminal_rows.saturating_sub(1) as usize;
    app.scroll_y = app.scroll_y.saturating_sub(view_h / 2);
}

pub fn nav_half_down(app: &mut App) {
    let view_h = app.terminal_rows.saturating_sub(1) as usize;
    app.scroll_y += view_h / 2;
    clamp_scroll(app);
}

pub fn nav_home(app: &mut App) {
    app.scroll_y = 0;
}

pub fn nav_end(app: &mut App) {
    app.scroll_y = app.layout.count();
    clamp_scroll(app);
}
