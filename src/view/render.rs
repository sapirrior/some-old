use crate::app::App;
use crossterm::{
    QueueableCommand, cursor,
    style::{self, Attribute, Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use regex::Regex;
use std::io::{self, Write};

pub fn render_screen(app: &App) {
    let mut stdout = io::stdout();

    if app.show_help {
        render_help(app, &mut stdout);
        let _ = stdout.flush();
        return;
    }

    let _ = stdout.queue(cursor::MoveTo(0, 0));
    let _ = stdout.queue(style::SetAttribute(Attribute::Reset));

    let margin = (app.terminal_cols as usize * 8) / 100;
    let view_height = if app.terminal_rows > 1 {
        app.terminal_rows as usize - 1
    } else {
        0
    };

    let mut scroll_y = app.scroll_y;
    if app.layout.count() > 0 {
        if scroll_y >= app.layout.count() {
            scroll_y = app.layout.count().saturating_sub(1);
        }
    } else {
        scroll_y = 0;
    }

    let regex = if !app.last_pattern.is_empty() {
        Regex::new(&app.last_pattern).ok()
    } else {
        None
    };

    for i in 0..view_height {
        let line_idx = scroll_y + i;
        let _ = stdout.queue(cursor::MoveTo(0, i as u16));
        let _ = stdout.queue(Clear(ClearType::CurrentLine));

        if line_idx < app.layout.count() {
            if app.show_line_numbers {
                let raw_line = app.layout.display_to_raw[line_idx];
                if raw_line > 0 {
                    let _ = stdout.queue(SetForegroundColor(Color::AnsiValue(242)));
                    let line_num_str =
                        format!("{:>width$} ", raw_line, width = margin.saturating_sub(1));
                    let _ = stdout.queue(Print(line_num_str));
                    let _ = stdout.queue(style::SetAttribute(Attribute::Reset));
                } else {
                    let _ = stdout.queue(Print(" ".repeat(margin)));
                }
            } else {
                let _ = stdout.queue(Print(" ".repeat(margin)));
            }

            let line = &app.layout.display_lines[line_idx];
            if let Some(re) = &regex {
                let mut last_end = 0;
                for mat in re.find_iter(line) {
                    let _ = stdout.queue(Print(&line[last_end..mat.start()]));
                    let _ = stdout.queue(style::SetAttribute(Attribute::Reverse));
                    let _ = stdout.queue(Print(&line[mat.start()..mat.end()]));
                    let _ = stdout.queue(style::SetAttribute(Attribute::Reset));
                    last_end = mat.end();
                }
                let _ = stdout.queue(Print(&line[last_end..]));
            } else {
                let _ = stdout.queue(Print(line));
            }
        }
    }

    // Status bar
    let _ = stdout.queue(cursor::MoveTo(0, app.terminal_rows.saturating_sub(1)));
    let _ = stdout.queue(Clear(ClearType::CurrentLine));

    if let Some(ref msg) = app.error_message {
        let _ = stdout.queue(SetForegroundColor(Color::Red));
        let _ = stdout.queue(style::SetAttribute(Attribute::Reverse));
        let _ = stdout.queue(Print(msg));
        let _ = stdout.queue(style::SetAttribute(Attribute::Reset));
    } else if app.search_failed {
        let _ = stdout.queue(style::SetAttribute(Attribute::Reverse));
        let _ = stdout.queue(Print("Pattern not found (press any key)"));
        let _ = stdout.queue(style::SetAttribute(Attribute::Reset));
    } else if app.search_wrapped {
        let _ = stdout.queue(style::SetAttribute(Attribute::Reverse));
        let _ = stdout.queue(Print("Search wrapped (press any key)"));
        let _ = stdout.queue(style::SetAttribute(Attribute::Reset));
    } else {
        let current_last_line = scroll_y + view_height;
        if current_last_line >= app.layout.count() && !app.is_loading {
            let _ = stdout.queue(style::SetAttribute(Attribute::Reverse));
            let _ = stdout.queue(Print("(END)"));
            let _ = stdout.queue(style::SetAttribute(Attribute::Reset));
        } else {
            let _ = stdout.queue(Print(":"));
        }

        if app.is_loading {
            let _ = stdout.queue(Print(" [Loading...]"));
        }
        if app.follow_mode {
            let _ = stdout.queue(SetForegroundColor(Color::Yellow));
            let _ = stdout.queue(Print(" [Follow]"));
            let _ = stdout.queue(style::SetAttribute(Attribute::Reset));
        }
    }

    let _ = stdout.flush();
}

const HELP_LINES: &[&str] = &[
    "j, Down       Scroll down one line",
    "k, Up         Scroll up one line",
    "d             Scroll down half a page",
    "u             Scroll up half a page",
    "f, PageDown   Scroll down one page",
    "b, PageUp     Scroll up one page",
    "g, Home       Go to top of document",
    "G, End        Go to end of document",
    "F             Toggle follow mode (like tail -f)",
    "/             Search forward for a pattern",
    "?             Search backward for a pattern",
    "n             Repeat last search",
    "N             Repeat last search in reverse direction",
    ":n            Next file",
    ":p            Previous file",
    ":N            Toggle line numbers",
    ":<num>        Jump to line <num>",
    "h             Show this help",
    "q             Quit",
    "Esc           Clear search highlights / Return from help",
];

fn render_help(app: &App, stdout: &mut io::Stdout) {
    let _ = stdout.queue(style::SetAttribute(Attribute::Reset));
    let _ = stdout.queue(Clear(ClearType::All));
    let _ = stdout.queue(cursor::MoveTo(0, 0));

    let title = format!("--- Some-old v{} Help ---", env!("CARGO_PKG_VERSION"));
    let title_x = (app.terminal_cols as usize).saturating_sub(title.len()) / 2;
    let _ = stdout.queue(cursor::MoveTo(title_x as u16, 0));
    let _ = stdout.queue(Print(title));

    for (i, line) in HELP_LINES.iter().enumerate() {
        if i + 2 >= app.terminal_rows as usize {
            break;
        }
        let _ = stdout.queue(cursor::MoveTo(0, (i + 2) as u16));
        let _ = stdout.queue(Print(*line));
    }
}
