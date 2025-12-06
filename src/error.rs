use std::fmt;

// TODO: Add line/column info to error types.

#[derive(Debug)]
pub enum Error {
    LexerError(String),
    ParserError(String),
    RuntimeError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LexerError(msg) => write!(f, "LexerError: {msg}"),
            Error::ParserError(msg) => write!(f, "ParserError: {msg}"),
            Error::RuntimeError(msg) => write!(f, "RuntimeError: {msg}"),
        }
    }
}

impl std::error::Error for Error {}
