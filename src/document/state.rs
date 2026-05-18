use super::types::Document;

impl Document {
    pub fn clear(&mut self) {
        self.raw_lines.clear();
    }

    pub fn line_count(&self) -> usize {
        self.raw_lines.len()
    }
}
