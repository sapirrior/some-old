pub struct Document {
    pub raw_lines: Vec<String>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            raw_lines: Vec::new(),
        }
    }
}
