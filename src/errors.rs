use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZyraErrors {
    #[error(transparent)]
    LexerErrors(#[from] LexerErrors),

    #[error(transparent)]
    ParserErrors(#[from] ParserErrors),

    #[error(transparent)]
    InterpreterErrors(#[from] InterpreterErrors),
}

#[derive(Debug, Error)]
pub enum LexerErrors {
    #[error("unknow cha: {0}")]
    UnknowChar(char),

    #[error("no chars")]
    NOChars,

    #[error("unterminated string")]
    UnterminatedString,

    #[error("invalid number: {0}")]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Error)]
pub enum ParserErrors {
    #[error("unknow char")]
    UnknowChar(char),
}

#[derive(Debug, Error)]
pub enum InterpreterErrors {
    #[error("unknow char")]
    UnknowChar(char),
}
