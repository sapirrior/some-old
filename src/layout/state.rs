use super::types::Layout;

impl Layout {
    pub fn add_line(&mut self, text: &str, raw_num: usize) {
        self.display_lines.push(text.to_string());
        self.display_to_raw.push(raw_num);
    }

    pub fn count(&self) -> usize {
        self.display_lines.len()
    }
}
