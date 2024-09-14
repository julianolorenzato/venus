mod cli;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use common::pseudo_instructions::PseudoInstruction;
use common::{
    instructions::{token_to_instr, Instruction},
    pseudo_instructions::token_to_pseudo_instr,
};
use common::{NOperands, Word};

fn main() {
    cli::run();
}

struct Info {}

type SymbolTable = HashMap<String, Info>;

fn run() {
    let symbol_table: SymbolTable = HashMap::new();

    split_line("ADD START 3 * oppasda");
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

type AnalyzedLine<'a, 'b, 'c, 'd> = (
    Option<&'a str>,
    Option<&'b str>,
    Option<&'c str>,
    Option<&'d str>,
);

fn split_line(line: &str) -> Result<Vec<&str>, &str> {
    let line = line.trim();

    // Line has info, need to remove the possible forwards comment
    let line = match line.find('*') {
        Some(pos) => &line[..pos],
        None => line,
    };

    let mut tokens = line.split_whitespace();

    let split_line: Vec<&str> = tokens.by_ref().take(4).collect();

    // need to check for too many tokens error here (as soon as
    // possible) in order to not waste time computing excedent useless data
    if tokens.next().is_some() {
        Err("too many tokens")
    } else {
        Ok(split_line)
    }
}

// This function should return a tuple defining
// what token is what (label, operation, operand1, operand2)
fn analyze_line(line: Vec<&str>) -> Result<AnalyzedLine, &str> {
    match line[..] {
        [] => Err("too few tokens"),
        [a] => match instr_exists(a) {
            true => Ok((None, Some(a), None, None)),
            false => Err("missing operation"),
        },
        [a, b] => match (instr_exists(a), instr_exists(b)) {
            (false, false) => Err("missing operation"),
            (true, true) => Err("too many operations"),
            (true, false) => Ok((None, Some(a), Some(b), None)),
            (false, true) => Ok((Some(a), Some(b), None, None)),
        },
        [a, b, c] => match (instr_exists(a), instr_exists(b), instr_exists(c)) {
            (false, false, false) => Err("missing operation"),
            (true, true, _) => Err("too many operations"),
            (_, true, true) => Err("too many operations"),
            (true, _, true) => Err("too many operations"),
            (false, false, true) => Err("too many tokens before operation"),
            (false, true, false) => Ok((Some(a), Some(b), Some(c), None)),
            (true, false, false) => Ok((None, Some(a), Some(b), Some(c))),
        },
        [a, b, c, d] => match (
            instr_exists(a),
            instr_exists(b),
            instr_exists(c),
            instr_exists(d),
        ) {
            (false, false, false, false) => Err("missing operation"),
            (true, true, _, _) => Err("too many operations"),
            (true, _, true, _) => Err("too many operations"),
            (true, _, _, true) => Err("too many operations"),
            (_, true, _, true) => Err("too many operations"),
            (_, _, true, true) => Err("too many operations"),
            (_, true, true, _) => Err("too many operations"),
            (false, false, false, true) => Err("too many tokens before operation"),
            (false, false, true, false) => Err("too many tokens before operation"),
            (false, true, false, false) => Ok((Some(a), Some(b), Some(c), Some(d))),
            (true, false, false, false) => Err("too many tokens"),
        },
        _ => panic!("invalid state"),
    }
}

fn instr_exists(token: &str) -> bool {
    if let Some(_) = token_to_instr(token) {
        true
    } else if let Some(_) = token_to_pseudo_instr(token) {
        true
    } else {
        false
    }
}

// fn treat_line() {
//     let mut label: Option<&str> = None;
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
    fn split_line_regular() {
        let got = split_line("LOOP JUMP 15");
        let expected = Ok(vec!["LOOP", "JUMP", "15"]);
        assert_eq!(got, expected);
    }

    #[test]
    fn split_line_with_comment() {
        let got = split_line("LOOP JUMP 15 *SOME IRRELEVANT STUFF");
        let expected = Ok(vec!["LOOP", "JUMP", "15"]);
        assert_eq!(got, expected);
    }

    #[test]
    fn split_line_too_many_tokens() {
        let got = split_line("LOOP JUMP 15 48 74");
        let expected = Err::<Vec<&str>, &str>("too many tokens");
        assert_eq!(got, expected);
    }

    #[test]
    fn analyze_line_zero_tokens() {
        let got = analyze_line(vec![]);
        let expected: Result<AnalyzedLine, &str> = Err("too few tokens");
        assert_eq!(got, expected);
    }

    #[test]
    fn analyze_line_one_token() {
        let got = analyze_line(vec!["ADD"]);
        let expected: Result<AnalyzedLine, &str> = Ok((None, Some("ADD"), None, None));
        assert_eq!(got, expected);
    }

    #[test]
    fn analyze_line_one_token_missing_operation() {
        let got = analyze_line(vec!["ZIG"]);
        let expected: Result<AnalyzedLine, &str> = Err("missing operation");
        assert_eq!(got, expected);
    }

    #[test]
    fn analyze_line_two_tokens() {
        let got = analyze_line(vec!["PR", "SUB"]);
        let expected: Result<AnalyzedLine, &str> = Ok((Some("PR"), Some("SUB"), None, None));
        assert_eq!(got, expected);
    }

    #[test]
    fn analyze_line_two_tokens_missing_operation() {
        let got = analyze_line(vec!["PR", "15"]);
        let expected: Result<AnalyzedLine, &str> = Err("missing operation");
        assert_eq!(got, expected);
    }
}
