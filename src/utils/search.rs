use crate::app::App;
use regex::Regex;

pub fn do_search(app: &mut App, pattern: &str, dir: i32) {
    if pattern.is_empty() || app.doc.raw_lines.is_empty() {
        return;
    }

    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(_) => {
            app.search_failed = true;
            return;
        }
    };

    let mut current_raw = 0;
    for (i, &display_idx) in app.layout.raw_to_display.iter().enumerate() {
        if display_idx <= app.scroll_y {
            current_raw = i;
        } else {
            break;
        }
    }

    let n = app.doc.raw_lines.len();
    let mut found = None;
    let mut wrapped = false;

    for i in 1..=n {
        let idx = if dir > 0 {
            let next = current_raw + i;
            if next >= n {
                wrapped = true;
                next % n
            } else {
                next
            }
        } else {
            let next = (current_raw as i32 - i as i32).rem_euclid(n as i32) as usize;
            if next > current_raw {
                wrapped = true;
            }
            next
        };

        if re.is_match(&app.doc.raw_lines[idx]) {
            found = Some(idx);
            break;
        }
    }

    if let Some(idx) = found {
        app.scroll_y = app.layout.raw_to_display[idx];
        app.search_wrapped = wrapped;
        app.last_pattern = pattern.to_string();
    } else {
        app.search_failed = true;
    }
}
