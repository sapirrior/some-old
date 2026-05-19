use std::io::{self, BufRead, BufReader};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::thread;

use super::types::Document;

impl Document {
    pub fn spawn_reader<R: io::Read + Send + 'static>(
        reader: R,
        tx: Sender<String>,
        stop_signal: Arc<AtomicBool>,
    ) {
        thread::spawn(move || {
            let buf_reader = BufReader::new(reader);
            for line in buf_reader.lines() {
                if stop_signal.load(Ordering::Relaxed) {
                    break;
                }
                if let Ok(l) = line
                    && tx.send(l).is_err()
                {
                    break;
                }
            }
        });
    }
}
