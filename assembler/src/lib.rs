mod error;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Seek},
};

use common::{
    instructions::{token_to_instr, Instruction},
    pseudo_instructions::token_to_pseudo_instr,
};
use common::{pseudo_instructions::PseudoInstruction, Sizeable};
use common::{NOperands, Word};
use error::{Kind, ParseError};

pub struct Assembler<'a> {
    filepath: &'a str,
    line_counter: u32,
    location_counter: u32,
    symbol_table: HashMap<String, Info>,
    program: Option<Vec<Word>>,
}

impl<'a> Assembler<'a> {
    pub fn new(filepath: &'a str) -> Self {
        Assembler {
            filepath,
            line_counter: 0,
            location_counter: 0,
            symbol_table: HashMap::new(),
            program: None,
        }
    }

    pub fn run(&self) -> Result<Program, ParseError> {
        let mut file = File::open(self.filepath).unwrap();
        let reader = BufReader::new(&file);

        let mut parsed_program: ParsedProgram = vec![];
        for line in reader.lines() {
            let line = line.unwrap();

            let split_line = split_line(line.as_str()).unwrap();

            let parsed_line = parse_line(split_line)?;

            if !is_valid_label(&parsed_line) {
                panic!("invalid label, it should contain only letters")
            }

            if !is_valid_operands(&parsed_line) {
                panic!("invalid amount of operands for this operation")
            }

            parsed_program.push(parsed_line);

            // first pass
        }

        file.rewind().unwrap();

        let reader = BufReader::new(&file);
        // second pass
        for line in reader.lines() {}

        Ok(Vec::<u16>::new())
    }
}

struct Info {}
type Program = Vec<Word>;

type ParsedProgram = Vec<ParsedLine>;

// pub fn run(filepath: &str, tasks: Vec<fn(String)>) -> Result<Program, ParseError> {
//     let symbol_table: HashMap<String, Info> =

// }

// fn first_pass(parsed_line: ParsedLine, symbol_table: &mut SymbolTable) {}

fn split_line(line: &str) -> Result<Vec<&str>, ParseError> {
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
        Err(ParseError::new(0, Kind::TooManyTokens))
    } else {
        Ok(split_line)
    }
}

type ParsedLine = (Option<String>, String, Option<String>, Option<String>);

// This function should return a tuple defining
// what token is what (label, operation, operand1, operand2)
fn parse_line(line: Vec<&str>) -> Result<ParsedLine, ParseError> {
    match &line[..] {
        [] => Err(ParseError::new(0, Kind::TooFewTokens)),
        [a] => match is_valid_operation(a) {
            true => Ok((None, a.to_string(), None, None)),
            false => Err(ParseError::new(0, Kind::NotFoundOperation)),
        },
        [a, b] => match (is_valid_operation(a), is_valid_operation(b)) {
            (false, false) => Err(ParseError::new(0, Kind::NotFoundOperation)),
            (true, true) => Err(ParseError::new(0, Kind::TooManyOperations)),
            (true, false) => Ok((None, a.to_string(), Some(b.to_string()), None)),
            (false, true) => Ok((Some(a.to_string()), b.to_string(), None, None)),
        },
        [a, b, c] => match (
            is_valid_operation(a),
            is_valid_operation(b),
            is_valid_operation(c),
        ) {
            (false, false, false) => Err(ParseError::new(0, Kind::NotFoundOperation)),
            (true, true, _) => Err(ParseError::new(0, Kind::TooManyOperations)),
            (_, true, true) => Err(ParseError::new(0, Kind::TooManyOperations)),
            (true, _, true) => Err(ParseError::new(0, Kind::TooManyOperations)),
            (false, false, true) => Err(ParseError::new(0, Kind::TooManyTokensBeforeOperation)),
            (false, true, false) => Ok((
                Some(a.to_string()),
                b.to_string(),
                Some(c.to_string()),
                None,
            )),
            (true, false, false) => Ok((
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
            (false, false, false, false) => Err(ParseError::new(0, Kind::NotFoundOperation)),
            (true, true, _, _) => Err(ParseError::new(0, Kind::TooManyOperations)),
            (true, _, true, _) => Err(ParseError::new(0, Kind::TooManyOperations)),
            (true, _, _, true) => Err(ParseError::new(0, Kind::TooManyOperations)),
            (_, true, _, true) => Err(ParseError::new(0, Kind::TooManyOperations)),
            (_, _, true, true) => Err(ParseError::new(0, Kind::TooManyOperations)),
            (_, true, true, _) => Err(ParseError::new(0, Kind::TooManyOperations)),
            (false, false, false, true) => {
                Err(ParseError::new(0, Kind::TooManyTokensBeforeOperation))
            }
            (false, false, true, false) => {
                Err(ParseError::new(0, Kind::TooManyTokensBeforeOperation))
            }
            (false, true, false, false) => Ok((
                Some(a.to_string()),
                b.to_string(),
                Some(c.to_string()),
                Some(d.to_string()),
            )),
            (true, false, false, false) => {
                Err(ParseError::new(0, Kind::TooManyTokensAfterOperation))
            }
        },
        _ => panic!("invalid state"),
    }
}

fn is_valid_label(line: &ParsedLine) -> bool {
    if let Some(token) = &line.0 {
        token.chars().next().unwrap().is_alphabetic()
    } else {
        // if there is no label, consider it valid
        true
    }
}

fn is_valid_operation(token: &str) -> bool {
    if let Some(_) = token_to_instr(token) {
        true
    } else if let Some(_) = token_to_pseudo_instr(token) {
        true
    } else {
        false
    }
}

fn is_valid_operands(line: &ParsedLine) -> bool {
    let (_, operation_token, operand1_token, operand2_token) = line;

    let n_operands: NOperands;

    if let Some(operation) = token_to_instr(operation_token.as_str()) {
        n_operands = operation.n_operands();
    } else if let Some(operation) = token_to_pseudo_instr(operation_token.as_str()) {
        n_operands = operation.n_operands();
    } else {
        panic!(
            "invalid state, operation validity should already be checked, maybe I should change it"
        )
    }

    match (n_operands, operand1_token, operand2_token) {
        (NOperands::Zero, None, None) => true,
        (NOperands::One, Some(_), None) => true,
        (NOperands::One, None, Some(_)) => true,
        (NOperands::Two, Some(_), Some(_)) => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_line_test() {
        let test_cases = vec![
            (
                "regular case",
                "LOOP JUMP 15",
                Ok(vec!["LOOP", "JUMP", "15"]),
            ),
            (
                "comment case",
                "LOOP JUMP 15 *SOME IRRELEVANT STUFF",
                Ok(vec!["LOOP", "JUMP", "15"]),
            ),
            (
                "more than 4 tokens case",
                "LOOP JUMP 15 48 74",
                Err("too many tokens"),
            ),
        ];

        for (description, input, expected_output) in test_cases {
            let output = split_line(input);
            // assert_eq!(output, expected_output, "{}", description);
        }
    }

    #[test]
    fn parse_line_test() {
        let test_cases = vec![
            ("when 0 tokens were supplied", vec![], Err("too few tokens")),
            (
                "when 1 token was supplied and it is an operation",
                vec!["ADD"],
                Ok((None, "ADD", None, None)),
            ),
            (
                "when 1 token was supplied and it is not an operation",
                vec!["ZIG"],
                Err("missing operation"),
            ),
            (
                "when 2 tokens were supplied and the first is an operation",
                vec!["ADD", "BOO"],
                Ok((None, "ADD", Some("BOO"), None)),
            ),
            (
                "when 2 tokens were supplied and the second is an operation",
                vec!["SIG", "SUB"],
                Ok((Some("SIG"), "SUB", None, None)),
            ),
            (
                "when 2 tokens were supplied and both are operations",
                vec!["JUMP", "ADD"],
                Err("too many operations"),
            ),
            (
                "when 2 tokens were supplied and none of them are operations",
                vec!["SIG", "FOO"],
                Err("missing operation"),
            ),
            (
                "when 3 tokens were supplied and the first is an operation",
                vec!["ADD", "15", "BAR"],
                Ok((None, "ADD", Some("15"), Some("BAR"))),
            ),
            (
                "when 3 tokens were supplied and the second is an operation",
                vec!["SIG", "JUMP", "89"],
                Ok((Some("SIG"), "JUMP", Some("89"), None)),
            ),
            (
                "when 3 tokens were supplied and the third is an operation",
                vec!["SIG", "FOO", "JUMP"],
                Err("too many tokens before operation"),
            ),
            (
                "when 3 tokens were supplied and there are multiple operations",
                vec!["SUB", "FOO", "JUMP"],
                Err("too many operations"),
            ),
            (
                "when 3 tokens were supplied and none of them are operations",
                vec!["FOO", "BAR", "BAZ"],
                Err("missing operation"),
            ),
            (
                "when 4 tokens were supplied and the second is an operation",
                vec!["ZIG", "JUMP", "DOL", "77"],
                Ok((Some("ZIG"), "JUMP", Some("DOL"), Some("77"))),
            ),
            (
                "when 4 tokens were supplied and the third is an operation",
                vec!["ZIG", "FOO", "JUMP", "55"],
                Err("too many tokens before operation"),
            ),
            (
                "when 4 tokens were supplied and the fourth is an operation",
                vec!["ZIG", "FOO", "55", "JUMP"],
                Err("too many tokens before operation"),
            ),
            (
                "when 4 tokens were supplied and the first is an operation",
                vec!["JUMP", "78", "DOL", "45"],
                Err("too many tokens after operation"),
            ),
            (
                "when 4 tokens were supplied and there are multiple operations",
                vec!["SIG", "SUB", "JUMP", "45"],
                Err("too many operations"),
            ),
        ];

        for (description, input, expected_output) in test_cases {
            let output = parse_line(input);
            // assert_eq!(output, expected_output, "{}", description);
        }
    }
}
