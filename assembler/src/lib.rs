use std::{
    collections::HashMap,
    env::current_exe,
    fs::File,
    io::{BufRead, BufReader, Empty},
    str::SplitWhitespace,
    string::{FromUtf8Error, ParseError},
};

use common::pseudo_instructions::token_to_pseudo_instr;
use common::{instructions::Instruction, Sizeable};
use common::{NOperands, Word};

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
enum ParsedLine<'a, 'b, 'c, 'd> {
    Regular(Option<&'a str>, &'b str, Option<&'c str>, Option<&'d str>),
    Empty,
}

enum State {
    Start,
    Label,
    Operation,
    Operand1,
    // Operand2,
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
    let line = line.trim();

    // Line is empty or is a comment
    if line == "" || line.starts_with("*") {
        return Ok(ParsedLine::Empty);
    }

    // Line has info
    let line = match line.find('*') {
        Some(pos) => &line[..pos],
        None => line,
    };

    return Ok(ParsedLine::Regular(Some("a"), "b", Some("c"), Some("d")));
}

// fn treat_line() {
//     let mut label: Option<&str> = None;
//     let mut tokens = line.split_whitespace();
//     let mut curr_state = State::Start;
//     let mut n_operands: Option<NOperands> = None;
//     let mut operand1: Option<Word>;
//     let mut operand2: Option<Word>;

//     // maybe use tuples (operation_size, tokens.next()) in all match arms to got a cleaner code
//     loop {
//         match (curr_state, tokens.next(), n_operands) {
//             (State::Start, Some(token), None) => {
//                 if let Some(pseudo_instruction) = token_to_pseudo_instr(token) {
//                     // Pseudo Instruction
//                     n_operands = Some(pseudo_instruction.n_operands());
//                     curr_state = State::Operation;
//                 } else {
//                     // Label
//                     curr_state = State::Label;
//                     label = Some(token);
//                 }
//             }
//             (State::Start, None, _) => panic!("invalid state"),
//             (State::Start, _, None) => panic!("invalid state"),
//             (State::Label, None, _) => return Err("found label but missing instruction"),
//             (State::Label, Some(token), _) => {
//                 if let Some(pseudo_instruction) = token_to_pseudo_instr(token) {
//                     n_operands = Some(pseudo_instruction.n_operands());
//                     curr_state = State::Operation;
//                 } else {
//                     return Err("invalid operation");
//                 }
//             },
//             (State::Operation, None, Some(NOperands::Zero)) => curr_state = State::End,
//             (State::Operation, Some(_), Some(NOperands::Zero)) => return Err("too many operands for this operation"),
//             (State::Operation, Some(token), Some(NOperands::One)) => {
//                 operand1 = match token.parse() {
//                     Ok(v) => v,
//                     Err(_) => return Err("invalid")
//                 };
//                 curr_state = State::End
//             },
//             (State::Operation, Some(token), Some(NOperands::Two))
//         }
//     }
// }

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

// fn check_pseudo_instruction(token: &str) -> Option<OpSize> {
//     match token {
//         "CONST" => Some(OpSize::Zero),
//         "SPACE" => Some(OpSize::One),
//         "INTDEF" => Some(OpSize::Two),
//         "INTUSE" => Some(OpSize::Three),
//         _ => None,
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let got = parse_line("LOOP JUMP 15");

        // let expected: Result<ParsedLine, &str> = Ok(ParsedLine::Regular(
        //     Some("LOOP"),
        //     Instruction::JUMP,
        //     Some(15),
        //     None,
        // ));

        // assert_eq!(got, expected);
    }
}
