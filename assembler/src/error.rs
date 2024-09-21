use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ParseError {
    line: u32,
    kind: Kind,
}

#[derive(Debug)]
pub enum Kind {
    NotFoundOperation,
    TooManyOperations,
    TooManyTokens,
    TooFewTokens,
    TooManyTokensBeforeOperation,
    TooManyTokensAfterOperation,
}

impl ParseError {
    pub fn new(line: u32, kind: Kind) -> Self {
        ParseError { line, kind }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self.kind {
            Kind::NotFoundOperation => "Operation not found",
            Kind::TooManyOperations => "Too many operations found",
            _ => "Something gone wrong",
        };

        write!(f, "{} (at line {}).", msg, self.line)
    }
}

impl Error for ParseError {}
