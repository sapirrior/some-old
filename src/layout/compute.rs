use crate::document::Document;
use super::types::Layout;

impl Layout {
    pub fn compute(&mut self, doc: &Document, cols: u16) {
        self.display_lines.clear();
        self.display_to_raw.clear();
        self.raw_to_display.clear();

        if doc.raw_lines.is_empty() {
            return;
        }

        let margin = (cols as usize * 8) / 100;
        let content_width = if cols as usize > 2 * margin {
            cols as usize - (2 * margin)
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

            let mut start = 0;
            let mut first_chunk = true;
            let chars: Vec<char> = raw.chars().collect();
            let len = chars.len();

            while start < len {
                let current_raw = if first_chunk { raw_num } else { 0 };
                let remaining = len - start;

                if remaining <= content_width {
                    let chunk: String = chars[start..].iter().collect();
                    self.add_line(&chunk, current_raw);
                    break;
                }

                let mut split = start + content_width;
                // Look for space or hyphen to split
                let mut found_split = false;
                for j in (start..=split).rev() {
                    if j < len && (chars[j] == ' ' || chars[j] == '-') {
                        split = j;
                        found_split = true;
                        break;
                    }
                }

                if !found_split {
                    // Hard split
                    split = start + content_width;
                    let chunk: String = chars[start..split].iter().collect();
                    self.add_line(&chunk, current_raw);
                    start = split;
                } else {
                    // Split at space or hyphen
                    let mut split_len = split - start;
                    if chars[split] == '-' {
                        split_len += 1;
                    }
                    
                    let chunk: String = chars[start..start + split_len].iter().collect();
                    self.add_line(&chunk, current_raw);
                    
                    start = split;
                    if chars[split] == ' ' {
                        start += 1;
                    }
                }
                first_chunk = false;
            }
        }
    }
}
