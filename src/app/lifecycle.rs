use super::types::App;
use crate::document::Document;
use crate::layout::Layout;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;

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
            follow_mode: false,
            is_loading: false,
            terminal_cols: 0,
            terminal_rows: 0,
            error_message: None,
            line_receiver: None,
            stop_io: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn init(&mut self) {
        if !self.filenames.is_empty() {
            let filename = self.filenames[self.current_file_index].clone();
            self.start_loading_file(filename);
        } else {
            use std::io::IsTerminal;
            let stdin = std::io::stdin();
            if !stdin.is_terminal() {
                self.start_loading_stdin();
            }
        }
    }

    fn start_loading_file(&mut self, filename: String) {
        use std::fs::File;
        match File::open(&filename) {
            Ok(file) => {
                self.stop_io.store(true, Ordering::SeqCst);
                self.stop_io = Arc::new(AtomicBool::new(false));
                let (tx, rx) = mpsc::channel();
                self.line_receiver = Some(rx);
                self.is_loading = true;
                Document::spawn_reader(file, tx, Arc::clone(&self.stop_io));
            }
            Err(e) => {
                self.error_message = Some(format!("Error opening {}: {}", filename, e));
            }
        }
    }

    fn start_loading_stdin(&mut self) {
        self.stop_io.store(true, Ordering::SeqCst);
        self.stop_io = Arc::new(AtomicBool::new(false));
        let (tx, rx) = mpsc::channel();
        self.line_receiver = Some(rx);
        self.is_loading = true;
        Document::spawn_reader(std::io::stdin(), tx, Arc::clone(&self.stop_io));
    }

    pub fn switch_file(&mut self, index: usize) {
        if index < self.filenames.len() {
            self.current_file_index = index;
            self.doc.clear();
            self.scroll_y = 0;
            let filename = self.filenames[self.current_file_index].clone();
            self.start_loading_file(filename);
            self.layout.compute(&self.doc, self.terminal_cols);
        }
    }
}
