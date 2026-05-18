use super::types::App;

impl App {
    pub fn run(&mut self) {
        let _guard = crate::terminal::TerminalGuard::new();
        let (cols, rows) = crate::terminal::get_size();
        self.terminal_cols = cols;
        self.terminal_rows = rows;
        
        self.init();
        self.layout.compute(&self.doc, cols);

        while self.running {
            crate::view::render_screen(self);
            let key = crate::input::read_key();
            match key {
                crate::input::Key::Unknown => continue,
                _ => crate::commands::dispatch(self, key),
            }
        }
    }
}
