use crate::error::{Kind, ParseError};
use common::{
    instructions::token_to_instr, pseudo_instructions::token_to_pseudo_instr, NOperands, Sizeable,
};

pub type ParsedProgram = Vec<ParsedLine>;

pub type ParsedLine = (Option<String>, String, Option<String>, Option<String>);

pub fn split_line(line: &str, line_index: u32) -> Result<Vec<&str>, ParseError> {
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
        Err(ParseError::new(line_index, Kind::TooManyTokens))
    } else {
        Ok(split_line)
    }
}

// This function should return a tuple defining
// what token is what (label, operation, operand1, operand2)
pub fn parse_line(line: Vec<&str>, line_index: u32) -> Result<ParsedLine, ParseError> {
    match &line[..] {
        [] => Err(ParseError::new(line_index, Kind::TooFewTokens)),
        [a] => match is_valid_operation(a) {
            true => Ok((None, a.to_string(), None, None)),
            false => Err(ParseError::new(line_index, Kind::NotFoundOperation)),
        },
        [a, b] => match (is_valid_operation(a), is_valid_operation(b)) {
            (false, false) => Err(ParseError::new(line_index, Kind::NotFoundOperation)),
            (true, true) => Err(ParseError::new(line_index, Kind::TooManyOperations)),
            (true, false) => Ok((None, a.to_string(), Some(b.to_string()), None)),
            (false, true) => Ok((Some(a.to_string()), b.to_string(), None, None)),
        },
        [a, b, c] => match (
            is_valid_operation(a),
            is_valid_operation(b),
            is_valid_operation(c),
        ) {
            (false, false, false) => Err(ParseError::new(line_index, Kind::NotFoundOperation)),
            (true, true, _) => Err(ParseError::new(line_index, Kind::TooManyOperations)),
            (_, true, true) => Err(ParseError::new(line_index, Kind::TooManyOperations)),
            (true, _, true) => Err(ParseError::new(line_index, Kind::TooManyOperations)),
            (false, false, true) => Err(ParseError::new(
                line_index,
                Kind::TooManyTokensBeforeOperation,
            )),
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
            (false, false, false, false) => {
                Err(ParseError::new(line_index, Kind::NotFoundOperation))
            }
            (true, true, _, _) => Err(ParseError::new(line_index, Kind::TooManyOperations)),
            (true, _, true, _) => Err(ParseError::new(line_index, Kind::TooManyOperations)),
            (true, _, _, true) => Err(ParseError::new(line_index, Kind::TooManyOperations)),
            (_, true, _, true) => Err(ParseError::new(line_index, Kind::TooManyOperations)),
            (_, _, true, true) => Err(ParseError::new(line_index, Kind::TooManyOperations)),
            (_, true, true, _) => Err(ParseError::new(line_index, Kind::TooManyOperations)),
            (false, false, false, true) => Err(ParseError::new(
                line_index,
                Kind::TooManyTokensBeforeOperation,
            )),
            (false, false, true, false) => Err(ParseError::new(
                line_index,
                Kind::TooManyTokensBeforeOperation,
            )),
            (false, true, false, false) => Ok((
                Some(a.to_string()),
                b.to_string(),
                Some(c.to_string()),
                Some(d.to_string()),
            )),
            (true, false, false, false) => Err(ParseError::new(
                line_index,
                Kind::TooManyTokensAfterOperation,
            )),
        },
        _ => panic!("invalid state"),
    }
}

pub fn is_valid_label(line: &ParsedLine) -> bool {
    if let Some(token) = &line.0 {
        token.chars().next().unwrap().is_alphabetic()
    } else {
        // if there is no label, consider it valid
        true
    }
}

pub fn is_valid_operation(token: &str) -> bool {
    if let Some(_) = token_to_instr(token) {
        true
    } else if let Some(_) = token_to_pseudo_instr(token) {
        true
    } else {
        false
    }
}

pub fn is_valid_operands(line: &ParsedLine) -> bool {
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
                ("LOOP JUMP 15", 14),
                Ok(vec!["LOOP", "JUMP", "15"]),
            ),
            (
                "comment case",
                ("LOOP JUMP 15 *SOME IRRELEVANT STUFF", 14),
                Ok(vec!["LOOP", "JUMP", "15"]),
            ),
            (
                "more than 4 tokens case",
                ("LOOP JUMP 15 48 74", 14),
                Err(ParseError::new(14, Kind::TooManyTokens)),
            ),
        ];

        for (description, (line, line_index), expected_output) in test_cases {
            let output = split_line(line, line_index);
            assert_eq!(output, expected_output, "{}", description);
        }
    }

    #[test]
    fn parse_line_test() {
        let test_cases = vec![
            (
                "when 0 tokens were supplied",
                (vec![], 23),
                Err(ParseError::new(23, Kind::TooFewTokens)),
            ),
            (
                "when 1 token was supplied and it is an operation",
                (vec!["ADD"], 23),
                Ok((None, "ADD".to_string(), None, None)),
            ),
            (
                "when 1 token was supplied and it is not an operation",
                (vec!["ZIG"], 23),
                Err(ParseError::new(23, Kind::NotFoundOperation)),
            ),
            (
                "when 2 tokens were supplied and the first is an operation",
                (vec!["ADD", "BOO"], 23),
                Ok((None, "ADD".to_string(), Some("BOO".to_string()), None)),
            ),
            (
                "when 2 tokens were supplied and the second is an operation",
                (vec!["SIG", "SUB"], 23),
                Ok((Some("SIG".to_string()), "SUB".to_string(), None, None)),
            ),
            (
                "when 2 tokens were supplied and both are operations",
                (vec!["JUMP", "ADD"], 23),
                Err(ParseError::new(23, Kind::TooManyOperations)),
            ),
            (
                "when 2 tokens were supplied and none of them are operations",
                (vec!["SIG", "FOO"], 23),
                Err(ParseError::new(23, Kind::NotFoundOperation)),
            ),
            (
                "when 3 tokens were supplied and the first is an operation",
                (vec!["ADD", "15", "BAR"], 23),
                Ok((
                    None,
                    "ADD".to_string(),
                    Some("15".to_string()),
                    Some("BAR".to_string()),
                )),
            ),
            (
                "when 3 tokens were supplied and the second is an operation",
                (vec!["SIG", "JUMP", "89"], 23),
                Ok((
                    Some("SIG".to_string()),
                    "JUMP".to_string(),
                    Some("89".to_string()),
                    None,
                )),
            ),
            (
                "when 3 tokens were supplied and the third is an operation",
                (vec!["SIG", "FOO", "JUMP"], 23),
                Err(ParseError::new(23, Kind::TooManyTokensBeforeOperation)),
            ),
            (
                "when 3 tokens were supplied and there are multiple operations",
                (vec!["SUB", "FOO", "JUMP"], 23),
                Err(ParseError::new(23, Kind::TooManyOperations)),
            ),
            (
                "when 3 tokens were supplied and none of them are operations",
                (vec!["FOO", "BAR", "BAZ"], 23),
                Err(ParseError::new(23, Kind::NotFoundOperation)),
            ),
            (
                "when 4 tokens were supplied and the second is an operation",
                (vec!["ZIG", "JUMP", "DOL", "77"], 23),
                Ok((
                    Some("ZIG".to_string()),
                    "JUMP".to_string(),
                    Some("DOL".to_string()),
                    Some("77".to_string()),
                )),
            ),
            (
                "when 4 tokens were supplied and the third is an operation",
                (vec!["ZIG", "FOO", "JUMP", "55"], 23),
                Err(ParseError::new(23, Kind::TooManyTokensBeforeOperation)),
            ),
            (
                "when 4 tokens were supplied and the fourth is an operation",
                (vec!["ZIG", "FOO", "55", "JUMP"], 23),
                Err(ParseError::new(23, Kind::TooManyTokensBeforeOperation)),
            ),
            (
                "when 4 tokens were supplied and the first is an operation",
                (vec!["JUMP", "78", "DOL", "45"], 23),
                Err(ParseError::new(23, Kind::TooManyTokensAfterOperation)),
            ),
            (
                "when 4 tokens were supplied and there are multiple operations",
                (vec!["SIG", "SUB", "JUMP", "45"], 23),
                Err(ParseError::new(23, Kind::TooManyOperations)),
            ),
        ];

        for (description, (line, line_index), expected_output) in test_cases {
            let output = parse_line(line, line_index);
            assert_eq!(output, expected_output, "{}", description);
        }
    }
}
