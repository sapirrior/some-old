use crate::app::App;
use crate::utils;
use super::utils::read_prompt;

pub fn search_forward(app: &mut App) {
    let pattern = read_prompt(app, '/');
    if !pattern.is_empty() {
        app.last_search_dir = 1;
        utils::do_search(app, &pattern, 1);
    }
}

pub fn search_backward(app: &mut App) {
    let pattern = read_prompt(app, '?');
    if !pattern.is_empty() {
        app.last_search_dir = -1;
        utils::do_search(app, &pattern, -1);
    }
}

pub fn search_next(app: &mut App) {
    if !app.last_pattern.is_empty() {
        let pattern = app.last_pattern.clone();
        utils::do_search(app, &pattern, app.last_search_dir);
    }
}

pub fn search_prev(app: &mut App) {
    if !app.last_pattern.is_empty() {
        let pattern = app.last_pattern.clone();
        utils::do_search(app, &pattern, -app.last_search_dir);
    }
}
