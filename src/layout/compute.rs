use super::types::Layout;
use crate::document::Document;

impl Layout {
    pub fn compute(&mut self, doc: &Document, cols: u16) {
        self.display_lines.clear();
        self.display_to_raw.clear();
        self.raw_to_display.clear();

        if doc.raw_lines.is_empty() {
            return;
        }

        let margin = (cols as usize * 8) / 100;
        let content_width = if cols as usize > margin * 2 {
            cols as usize - (margin * 2)
        } else {
            1
        };

        for (i, raw) in doc.raw_lines.iter().enumerate() {
            self.raw_to_display.push(self.display_lines.len());
            let raw_num = i + 1;

            if raw.is_empty() {
                self.add_line("", raw_num);
                continue;
            }

            let chars: Vec<char> = raw.chars().collect();
            let len = chars.len();
            let mut start = 0;
            let mut first_chunk = true;

            while start < len {
                let current_raw = if first_chunk { raw_num } else { 0 };
                let remaining = len - start;

                if remaining <= content_width {
                    let chunk: String = chars[start..].iter().collect();
                    self.add_line(&chunk, current_raw);
                    break;
                }

                let mut split = start + content_width;
                if split > len {
                    split = len;
                }

                let mut found_split = false;
                // Look for space or hyphen to split
                for j in (start..split).rev() {
                    if chars[j] == ' ' || chars[j] == '-' {
                        split = j;
                        found_split = true;
                        break;
                    }
                }

                if found_split {
                    let mut chunk_end = split;
                    let mut next_start = split + 1;

                    if chars[split] == '-' {
                        chunk_end = split + 1;
                        next_start = split + 1;
                    }

                    let chunk: String = chars[start..chunk_end].iter().collect();
                    self.add_line(&chunk, current_raw);
                    start = next_start;
                } else {
                    // Hard split
                    let chunk: String = chars[start..split].iter().collect();
                    self.add_line(&chunk, current_raw);
                    start = split;
                }

                first_chunk = false;
            }
        }
    }
}
