use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum Line {
    Empty,
    Comment(String),
    Regular(Option<String>, String, Option<String>, Option<String>),
    MacroSignature(String, Vec<String>),
}

pub fn tokenize(line: &str, line_index: u32) -> Result<Line, LexerError> {
    let line = line.trim().to_uppercase();

    if line.is_empty() {
        return Ok(Line::Empty);
    }

    if line.starts_with("*") {
        let comment = String::from(&line[1..]);
        return Ok(Line::Comment(comment));
    }

    if line.starts_with("MACRO") {
        return tokenize_macro_signature(&line, line_index);
    }

    // return Err(LexerError::new(0, LexerErrorKind::InvalidLabel));
    return Ok(Line::Regular(
        None,
        "NOT IMPLEMENTED YET".to_string(),
        None,
        None,
    ));
}

fn tokenize_macro_signature(line: &str, line_index: u32) -> Result<Line, LexerError> {
    let mut tokens = line.split(" ");

    let macro_name = tokens.by_ref().skip(1).next();

    let parameters: Result<Vec<String>, LexerError> = tokens
        .enumerate()
        .map(|(param_index, macro_param)| {
            if macro_param.starts_with("&") {
                Ok(macro_param[1..].to_string())
            } else {
                Err(LexerError::new(
                    line_index,
                    LexerErrorKind::InvalidMacroParam(param_index),
                ))
            }
        })
        .collect();

    if let Some(name) = macro_name {
        Ok(Line::MacroSignature(name.to_string(), parameters?))
    } else {
        Err(LexerError::new(
            line_index,
            LexerErrorKind::MissingMacroName,
        ))
    }
}

fn check_label(token: &str) -> bool {
    token.chars().next().unwrap().is_alphabetic()
}

#[derive(Debug)]
pub struct LexerError {
    line_index: u32,
    kind: LexerErrorKind,
}

#[derive(Debug)]
enum LexerErrorKind {
    InvalidLabel,
    MissingMacroName,
    InvalidMacroParam(usize),
}

impl LexerError {
    fn new(line_index: u32, kind: LexerErrorKind) -> Self {
        LexerError { line_index, kind }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self.kind {
            LexerErrorKind::InvalidLabel => "invalid label found".to_string(),
            LexerErrorKind::MissingMacroName => "macro name not found".to_string(),
            LexerErrorKind::InvalidMacroParam(param_index) => {
                format!("argument {param_index} must have a prefixed '&'")
            }
            _ => "something gone wrong".to_string(),
        };

        writeln!(f, "{} (at line {}).", msg, self.line_index)
    }
}

impl Error for LexerError {}
