use crate::document::Document;
use crate::layout::Layout;

pub struct App {
    pub doc: Document,
    pub layout: Layout,
    pub scroll_y: usize,
    pub filenames: Vec<String>,
    pub current_file_index: usize,
    pub last_pattern: String,
    pub last_search_dir: i32, // 1 for forward, -1 for backward
    pub search_failed: bool,
    pub search_wrapped: bool,
    pub show_help: bool,
    pub show_line_numbers: bool,
    pub running: bool,
    pub terminal_cols: u16,
    pub terminal_rows: u16,
}
