pub struct Layout {
    pub display_lines: Vec<String>,
    pub display_to_raw: Vec<usize>, // 1-indexed raw line number, 0 for continuation
    pub raw_to_display: Vec<usize>, // maps raw line index to starting display line index
}

impl Layout {
    pub fn new() -> Self {
        Self {
            display_lines: Vec::new(),
            display_to_raw: Vec::new(),
            raw_to_display: Vec::new(),
        }
    }
}
