use core::fmt;
use std::{error::Error, io::Empty};

use common::{instructions::token_to_instr, pseudo_instructions::token_to_pseudo_instr};

#[derive(Debug, Clone)]
pub enum Line {
    Empty,
    Comment(String),
    Regular(Option<String>, String, Option<String>, Option<String>),
    MacroDef(String, Vec<String>),
    MacroEnd,
    MacroCall(String, Vec<String>, WhyMacroCall),
    Removed
}

pub type Program = Vec<Line>;

pub fn decode(line: &str, line_index: u32) -> Result<Line, LexerError> {
    let line = line.trim();

    if line.is_empty() {
        return Ok(Line::Empty);
    }

    if line.starts_with("*") {
        let comment = String::from(&line[1..]);
        return Ok(Line::Comment(comment));
    }

    if line.to_uppercase().starts_with("MACRO") {
        return decode_macro_signature(&line, line_index);
    }

    if line.to_uppercase().starts_with("MEND") {
        return if line.to_uppercase() == "MEND" {
            Ok(Line::MacroEnd)
        } else {
            Err(LexerError::new(
                line_index,
                LexerErrorKind::TooManyTokensAfterMacroEnd,
            ))
        };
    }

    return parse_line(line.split(" ").collect(), line_index);
}

fn decode_macro_signature(line: &str, line_index: u32) -> Result<Line, LexerError> {
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
        Ok(Line::MacroDef(name.to_string(), parameters?))
    } else {
        Err(LexerError::new(
            line_index,
            LexerErrorKind::MissingMacroName,
        ))
    }
}

pub fn encode(line: Line) -> String {
    match line {
        Line::Empty => String::from("\n"),
        Line::Comment(mut comment) => {
            comment.insert(0, '*');
            comment.push('\n');
            comment
        },
        Line::MacroDef(mut name, params) => {
            for param in params {
                name.push(' ');
                name.push('&');
                name.push_str(&param);
            }

            name.push('\n');
            name
        },
        Line::MacroEnd => String::from("MEND\n"),
        Line::MacroCall(mut name, args, _) => {
            for arg in args {
                name.push(' ');
                name.push_str(&arg);
            }

            name.push('\n');
            name
        },
        Line::Regular(label, operation, operand1, operand2) => {
            let mut encoded = String::new();

            if let Some(label) = label {
                encoded.push_str(&label);
                encoded.push(' ');
            }

            encoded.push_str(&operation);

            if let Some(op1) = operand1 {
                encoded.push(' ');
                encoded.push_str(&op1);
            }

            if let Some(op2) = operand2 {
                encoded.push(' ');
                encoded.push_str(&op2);
            }

            encoded.push('\n');
            encoded
        },
        Line::Removed => String::new()
    }
}

// is used? must be
fn check_name(token: &str) -> bool {
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
    TooManyTokensAfterMacroEnd,
    TooFewTokens,
}

#[derive(Debug, Clone)]
pub enum WhyMacroCall {
    NotFoundOperation,
    TooManyOperations,
    TooManyTokens,
    TooManyTokensBeforeOperation,
    TooManyTokensAfterOperation,
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
            LexerErrorKind::TooManyTokensAfterMacroEnd => {
                "too many tokens found after MEND".to_string()
            }
            _ => "something gone wrong".to_string(),
        };

        writeln!(f, "{} (at line {}).", msg, self.line_index)
    }
}

impl Error for LexerError {}

pub fn is_valid_operation(token: &str) -> bool {
    if let Some(_) = token_to_instr(token) {
        true
    } else if let Some(_) = token_to_pseudo_instr(token) {
        true
    } else {
        false
    }
}


// maybe should change Line to have PseudoInstrCall and MachineInstrCall, and find each in this function instead of just calling it 'Regular', and the second field of each should be of the proper type (PseudoInstr or MachineInstr)
// maybe instead Ok(Regular...) go to function to define which type of instruction (pseudo or machine)
pub fn parse_line(line: Vec<&str>, line_index: u32) -> Result<Line, LexerError> {
    match &line[..] {
        [] => Err(LexerError::new(line_index, LexerErrorKind::TooFewTokens)),
        [a] => match is_valid_operation(a) {
            true => Ok(Line::Regular(None, a.to_string(), None, None)),
            false => Ok(Line::MacroCall(
                a.to_string(),
                vec![],
                WhyMacroCall::NotFoundOperation,
            )),
        },
        [a, b] => match (is_valid_operation(a), is_valid_operation(b)) {
            (false, false) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string()],
                WhyMacroCall::NotFoundOperation,
            )),
            (true, true) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string()],
                WhyMacroCall::TooManyOperations,
            )),
            (true, false) => Ok(Line::Regular(
                None,
                a.to_string(),
                Some(b.to_string()),
                None,
            )),
            (false, true) => Ok(Line::Regular(
                Some(a.to_string()),
                b.to_string(),
                None,
                None,
            )),
        },
        [a, b, c] => match (
            is_valid_operation(a),
            is_valid_operation(b),
            is_valid_operation(c),
        ) {
            (false, false, false) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string()],
                WhyMacroCall::NotFoundOperation,
            )),
            (true, true, _) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string()],
                WhyMacroCall::TooManyOperations,
            )),
            (_, true, true) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string()],
                WhyMacroCall::TooManyOperations,
            )),
            (true, _, true) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string()],
                WhyMacroCall::TooManyOperations,
            )),
            (false, false, true) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string()],
                WhyMacroCall::TooManyTokensBeforeOperation,
            )),
            (false, true, false) => Ok(Line::Regular(
                Some(a.to_string()),
                b.to_string(),
                Some(c.to_string()),
                None,
            )),
            (true, false, false) => Ok(Line::Regular(
                None,
                a.to_string(),
                Some(b.to_string()),
                Some(c.to_string()),
            )),
        },
        [a, b, c, d] => match (
            is_valid_operation(a),
            is_valid_operation(b),
            is_valid_operation(c),
            is_valid_operation(d),
        ) {
            (false, false, false, false) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string(), d.to_string()],
                WhyMacroCall::NotFoundOperation,
            )),
            (true, true, _, _) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string(), d.to_string()],
                WhyMacroCall::TooManyOperations,
            )),
            (true, _, true, _) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string(), d.to_string()],
                WhyMacroCall::TooManyOperations,
            )),
            (true, _, _, true) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string(), d.to_string()],
                WhyMacroCall::TooManyOperations,
            )),
            (_, true, _, true) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string(), d.to_string()],
                WhyMacroCall::TooManyOperations,
            )),
            (_, _, true, true) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string(), d.to_string()],
                WhyMacroCall::TooManyOperations,
            )),
            (_, true, true, _) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string(), d.to_string()],
                WhyMacroCall::TooManyOperations,
            )),
            (false, false, false, true) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string(), d.to_string()],
                WhyMacroCall::TooManyTokensBeforeOperation,
            )),
            (false, false, true, false) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string(), d.to_string()],
                WhyMacroCall::TooManyTokensBeforeOperation,
            )),
            (false, true, false, false) => Ok(Line::Regular(
                Some(a.to_string()),
                b.to_string(),
                Some(c.to_string()),
                Some(d.to_string()),
            )),
            (true, false, false, false) => Ok(Line::MacroCall(
                a.to_string(),
                vec![b.to_string(), c.to_string(), d.to_string()],
                WhyMacroCall::TooManyTokensAfterOperation,
            )),
        },
        // ugly... I should refac this case
        l => Ok(Line::MacroCall(
            l[0].to_string(),
            l[1..].iter().map(|x| x.to_string()).collect(),
            WhyMacroCall::TooManyTokens,
        )),
    }
}
