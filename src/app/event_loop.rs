use super::types::App;
use std::time::Duration;

impl App {
    pub fn run(&mut self) {
        self.init();

        let _guard = match crate::terminal::TerminalGuard::new() {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Initialization error: {}", e);
                return;
            }
        };

        let (cols, rows) = match crate::terminal::get_size() {
            Ok(s) => s,
            Err(e) => {
                self.error_message = Some(e.to_string());
                (80, 24) // Fallback size
            }
        };
        self.terminal_cols = cols;
        self.terminal_rows = rows;

        self.layout.compute(&self.doc, cols);

        while self.running {
            self.drain_lines();
            crate::view::render_screen(self);

            let key = if self.is_loading || self.follow_mode {
                crate::input::poll_key(Duration::from_millis(10))
            } else {
                crate::input::read_key()
            };

            match key {
                crate::input::Key::Unknown => continue,
                _ => crate::commands::dispatch(self, key),
            }
        }
    }

    fn drain_lines(&mut self) {
        let mut new_data = false;
        let mut disconnected = false;

        if let Some(ref rx) = self.line_receiver {
            loop {
                match rx.try_recv() {
                    Ok(line) => {
                        self.doc.raw_lines.push(line);
                        new_data = true;
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => break,
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        disconnected = true;
                        break;
                    }
                }
            }
        }

        if new_data {
            self.layout.compute(&self.doc, self.terminal_cols);
            if self.follow_mode {
                self.scroll_y = self
                    .layout
                    .display_lines
                    .len()
                    .saturating_sub(self.terminal_rows.saturating_sub(1) as usize);
            }
        }

        if disconnected {
            self.is_loading = false;
            self.line_receiver = None;
        }
    }
}
