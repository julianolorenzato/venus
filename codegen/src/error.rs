use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ParserError {
    line_index: u32,
    kind: ParserErrorKind,
}

#[derive(Debug)]
pub enum ParserErrorKind {
    InvalidLabel,
    MissingMacroName,
    InvalidMacroParam(usize),
    TooManyTokensAfterMacroEnd,
    TooFewTokens,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhyMacroCall {
    NotFoundOperation,
    TooManyOperations,
    TooManyTokens,
    TooManyTokensBeforeOperation,
    TooManyTokensAfterOperation,
}

impl ParserError {
    pub fn new(line_index: u32, kind: ParserErrorKind) -> Self {
        ParserError { line_index, kind }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self.kind {
            ParserErrorKind::InvalidLabel => "invalid label found".to_string(),
            ParserErrorKind::MissingMacroName => "macro name not found".to_string(),
            ParserErrorKind::InvalidMacroParam(param_index) => {
                format!("argument {param_index} must have a prefixed '&'")
            }
            ParserErrorKind::TooManyTokensAfterMacroEnd => {
                "too many tokens found after MEND".to_string()
            }
            _ => "something gone wrong".to_string(),
        };

        writeln!(f, "{} (at line {}).", msg, self.line_index)
    }
}

impl Error for ParserError {}