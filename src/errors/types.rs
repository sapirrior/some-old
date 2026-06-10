use std::fmt;
use std::io;

#[derive(Debug)]
pub enum SomeError {
    Io(io::Error),
    Terminal(String),
}

impl fmt::Display for SomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O Error: {}", err),
            Self::Terminal(msg) => write!(f, "Terminal Error: {}", msg),
        }
    }
}

impl std::error::Error for SomeError {}

impl From<io::Error> for SomeError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
