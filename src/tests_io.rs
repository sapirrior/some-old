#[cfg(test)]
mod tests {
    use crate::document::Document;
    use std::sync::atomic::AtomicBool;
    use std::sync::{Arc, mpsc};
    use std::time::Duration;

    #[test]
    fn test_spawn_reader_and_drain() {
        let (tx, rx) = mpsc::channel();
        let stop_signal = Arc::new(AtomicBool::new(false));

        let content = "line 1\nline 2\nline 3";
        let reader = std::io::Cursor::new(content);

        Document::spawn_reader(reader, tx, stop_signal);

        // Wait a bit for the thread to process
        std::thread::sleep(Duration::from_millis(100));

        let mut lines = Vec::new();
        while let Ok(line) = rx.try_recv() {
            lines.push(line);
        }

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line 1");
        assert_eq!(lines[1], "line 2");
        assert_eq!(lines[2], "line 3");
    }
}
