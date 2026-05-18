use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use super::types::Document;

impl Document {
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) {
        let file = File::open(&path).unwrap_or_else(|_| {
            panic!("Could not open file: {}", path.as_ref().display());
        });
        self.load_stream(file);
    }

    pub fn load_stream<R: io::Read>(&mut self, reader: R) {
        let buf_reader = BufReader::new(reader);
        for line in buf_reader.lines() {
            let line = line.expect("Failed to read line from stream");
            self.raw_lines.push(line);
        }
    }
}
