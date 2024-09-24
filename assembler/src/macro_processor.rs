use std::io::{BufRead, BufReader, Cursor, Read};

use common::Word;

use crate::lexer::{self, Line, Program};

struct Macro {
    name: String,
    params: Vec<String>,
}

struct MacroProcessor {
    macros: Vec<Macro>,
}

impl MacroProcessor {
    fn new() -> Self {
        MacroProcessor { macros: vec![] }
    }

    fn run(&mut self, program: Program) -> Program {
        let mut p = Vec::<Line>::new();

        for line in program {
            match line {
                Line::MacroSignature(name, params) => {
                    self.macros.push(Macro { name, params });
                }
                l => p.push(l),
            }
        }

        vec![]
    }
}
