mod error;
mod parser;
mod lexer;

use common::{instructions::token_to_instr, pseudo_instructions::token_to_pseudo_instr};
use common::{NOperands, Sizeable, Word};
use error::{Kind, ParseError};
use parser::{ParsedLine, ParsedProgram};
use std::io::Read;
use std::slice::Iter;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Seek},
};

pub struct Assembler<'a> {
    source: Iter<'a, ParsedLine>,
    line_counter: u32,
    location_counter: u32,
    symbol_table: HashMap<String, Info>,
    program: Option<Vec<Word>>,
}

enum AllocationMode {
    Absolut,
    Relative,
}

struct Info {
    allocation_mode: AllocationMode,
    address: Option<u32>,
}

impl<'a> Assembler<'a> {
    // pub fn new<R: Read>(source: Iter<ParsedLine>) -> Self {
    //     Assembler {
    //         source,
    //         line_counter: 0,
    //         location_counter: 0,
    //         symbol_table: HashMap::new(),
    //         program: None,
    //     }
    // }

    // pub fn run(&mut self) -> Result<Program, ParseError> {
    //     let reader = BufReader::new(&self.file);
    //     let mut parsed_program: ParsedProgram = vec![];

    //     for line in reader.lines() {
    //         self.line_counter += 1;

    //         let line = line.unwrap();

    //         let split_line = parser::split_line(&line, self.line_counter).unwrap();

    //         let parsed_line = parser::parse_line(split_line, self.line_counter).unwrap();

    //         if !parser::is_valid_label(&parsed_line) {
    //             panic!("invalid label, it should contain only letters")
    //         }

    //         if !parser::is_valid_operands(&parsed_line) {
    //             panic!("invalid amount of operands for this operation")
    //         }

    //         parsed_program.push(parsed_line);
    //     }

    //     self.line_counter = 0;
    //     self.file.rewind().unwrap();

    //     Ok(Some(Vec::<u16>::new()))
    // }

    fn first_pass(&mut self, p: ParsedProgram) {
        for (label, _, operand1, operand2) in p {
            if let Some(label) = label {
                let info = Info {
                    allocation_mode: AllocationMode::Relative,
                    address: Some(self.location_counter),
                };

                self.symbol_table.insert(label, info);
            }

            if let Some(operand1) = operand1 {
                if let Some(literal) = extract_literal(&operand1) {
                    // TODO: literals
                } else {
                    let info = Info {
                        allocation_mode: AllocationMode::Relative,
                        address: None,
                    };

                    self.symbol_table.insert(operand1, info);
                }
            }
        }
    }

    fn second_pass(&mut self, p: ParsedProgram) {}
}

// pub fn run<R: Read>(source: R) -> Program {
//     let line_counter = 0;
//     let location_counter = 0;
//     let symbol_table: HashMap<String, Info> = HashMap::new();
//     let program: Program = None;

//     program
// }

fn extract_literal(operand: &str) -> Option<Word> {
    if operand.starts_with('@') {
        Some(operand[1..].parse().unwrap())
    } else {
        None
    }
}
