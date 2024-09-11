use std::{
    collections::HashMap,
    env::current_exe,
    fs::File,
    io::{BufRead, BufReader},
    str::SplitWhitespace,
    string::{FromUtf8Error, ParseError},
};

use crate::common::{instructions::Instruction, Word};

pub mod cli;

struct Info {}

type SymbolTable = HashMap<String, Info>;

fn run() {
    let symbol_table: SymbolTable = HashMap::new();

    parse_line("ADD START 3 * oppasda");
}

fn first_pass(symbol_table: &mut SymbolTable) {}

fn read_line(file: &mut File) {
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    match reader.read_line(&mut line) {
        Ok(_) => (),
        Err(_) => panic!("something goes wrong during reading line"),
    }
}

#[derive(Debug, PartialEq)]
enum ParsedLine<'a> {
    Regular(Option<&'a str>, Instruction, Option<Word>, Option<Word>),
    Comment,
}

enum ParseState {
    Start,
    Label,
    Operation,
    Operand1,
    Operand2,
    Comment,
    End,
}

#[derive(Clone, Copy)]
enum OpSize {
    Unkown,
    Zero,
    One,
    Two,
    Three,
}

fn parse_line(line: &str) -> Result<ParsedLine, &str> {
    if line.starts_with("*") {
        return Ok(ParsedLine::Comment);
    }

    let mut label: Option<&str> = None;
    let mut tokens = line.split_whitespace();
    let mut state = ParseState::Start;
    let mut operation_size = OpSize::Unkown;

    // maybe use tuples (operation_size, tokens.next()) in all match arms to got a cleaner code
    loop {
        match state {
            ParseState::Start => {
                if let Some(token) = tokens.next() {
                    if let Some(size) = check_pseudo_instruction(token) {
                        // Pseudo Instruction
                        state = ParseState::Operation;
                        operation_size = size;
                    } else {
                        // Label
                        state = ParseState::Label;
                        label = Some(token);
                    }
                } else {
                    return Err("empty line");
                }
            }
            ParseState::Label => {
                if let Some(token) = tokens.next() {
                    if let Some(size) = check_pseudo_instruction(token) {
                        // Pseudo Instruction
                        state = ParseState::Operation;
                        operation_size = size;
                    } else {
                        return Err("operation value is invalid");
                    }
                } else {
                    return Err("found label but missing instruction");
                }
            }
            ParseState::Operation => match (operation_size, tokens.next()) {
                (OpSize::Zero, Some(token)) => {
                    if token.starts_with("*") {
                        state = ParseState::Comment
                    } else {
                        return Err("too much tokens for this operation");
                    }
                }
                (OpSize::Zero, None) => state = ParseState::End,
                (OpSize::One, None) => state = ParseState::End,
                (_, _) => state = ParseState::Operand1,
            },
            ParseState::Operand1 => match (operation_size, tokens.next()) {
                (OpSize::One, Some(token)) => {
                    if token.starts_with("*") {
                        state = ParseState::Comment
                    } else {
                        return Err("too much tokens for this operation");
                    }
                }
                (OpSize::One, None) => state = ParseState::End,
                (OpSize::Two, None) => state = ParseState::End,
                (_, _) => state = ParseState::Operand2,
            },
            ParseState::Operand2 => match (operation_size, tokens.next()) {
                // (OpSize::Two) =>
            },
            ParseState::Comment => {}
            ParseState::End => {
                if let None = tokens.next() {
                    // return Ok(ParsedLine::Regular(label, None, 3, 3));
                } else {
                    return Err("too many tokens in the line");
                }
            }
        }
    }
}

// fn extract_next_token(tokens: SplitWhitespace<'_>) -> Result<&str, &str> {

// }
// fn check_valid_states(state: ParseState, op_size: OpSize) -> Result<(), &str> {
//     match (state, op_size) {
//         (ParseState::Operand1, OpSize::Zero) => Err("too many operands"),
//         (ParseState::Operand2, OpSize::Zero) => Err("too many operands"),
//         (ParseState::Operand2, OpSize::One) => Err("too many operands"),
//         _ => Ok(()),
//     }
// }

fn check_pseudo_instruction(token: &str) -> Option<OpSize> {
    match token {
        "CONST" => Some(OpSize::Zero),
        "SPACE" => Some(OpSize::One),
        "INTDEF" => Some(OpSize::Two),
        "INTUSE" => Some(OpSize::Three),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let got = parse_line("LOOP JUMP 15");

        let expected: Result<ParsedLine, &str> = Ok(ParsedLine::Regular(
            Some("LOOP"),
            Instruction::JUMP,
            Some(15),
            None,
        ));

        assert_eq!(got, expected);
    }
}
