use crate::document::Document;
use crate::layout::Layout;
use super::types::App;

impl App {
    pub fn new(filenames: Vec<String>) -> Self {
        Self {
            doc: Document::new(),
            layout: Layout::new(),
            scroll_y: 0,
            filenames,
            current_file_index: 0,
            last_pattern: String::new(),
            last_search_dir: 1,
            search_failed: false,
            search_wrapped: false,
            show_help: false,
            show_line_numbers: false,
            running: true,
            terminal_cols: 0,
            terminal_rows: 0,
        }
    }

    pub fn init(&mut self) {
        if !self.filenames.is_empty() {
            let filename = self.filenames[self.current_file_index].clone();
            self.doc.load_file(filename);
        }
    }

    pub fn switch_file(&mut self, index: usize) {
        if index < self.filenames.len() {
            self.current_file_index = index;
            self.doc.clear();
            let filename = self.filenames[self.current_file_index].clone();
            self.doc.load_file(filename);
            self.scroll_y = 0;
            self.layout.compute(&self.doc, self.terminal_cols);
        }
    }
}
