use crate::app::App;
use super::utils::{read_prompt, clamp_scroll};

pub fn sys_colon(app: &mut App) {
    let buf = read_prompt(app, ':');
    if buf.is_empty() { return; }

    match buf.as_str() {
        "n" => if app.current_file_index < app.filenames.len().saturating_sub(1) {
            app.switch_file(app.current_file_index + 1);
        },
        "p" => if app.current_file_index > 0 {
            app.switch_file(app.current_file_index - 1);
        },
        "q" => app.running = false,
        "N" => app.show_line_numbers = !app.show_line_numbers,
        _ => {
            if let Ok(line) = buf.parse::<usize>() {
                if line > 0 && line <= app.doc.line_count() {
                    app.scroll_y = app.layout.raw_to_display[line - 1];
                    clamp_scroll(app);
                }
            }
        }
    }
}
