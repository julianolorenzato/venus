use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub struct ParseError {
    line_index: u32,
    kind: Kind,
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    NotFoundOperation,
    TooManyOperations,
    TooManyTokens,
    TooFewTokens,
    TooManyTokensBeforeOperation,
    TooManyTokensAfterOperation,
}

impl ParseError {
    pub fn new(line_index: u32, kind: Kind) -> Self {
        ParseError { line_index, kind }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self.kind {
            Kind::NotFoundOperation => "Operation not found",
            Kind::TooManyOperations => "Too many operations found",
            _ => "Something gone wrong",
        };

        write!(f, "{} (at line {}).", msg, self.line_index)
    }
}

impl Error for ParseError {}
