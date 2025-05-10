mod assembler;
mod error;
mod macro_processor;
mod parser;

use crate::parser::{decode, Line};
use crate::parser::{encode, Program};
use clap::Parser;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Cursor, Read, Write},
};

#[derive(Parser)]
#[command(author, version, about="Assemble Venus bytecode code generation.", long_about = None)]
struct AssemblerArgs {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long, default_value_t = true)]
    macro_support: bool,
}

fn main() {
    let args = AssemblerArgs::parse();

    let file = File::open(args.filepath).unwrap();
    let reader = BufReader::new(file);

    let mut p: Program = reader
        .lines()
        .enumerate()
        .map(|(i, line)| decode(&line.unwrap(), i as u32).unwrap())
        .collect();

    // TODO: turn program a borrowed reference
    if args.macro_support {
        p = macro_processor::run(p).unwrap();
        p = assembler::run(p);
    } else {
        p = assembler::run(p)
    }

    let mut f = File::create("codegen/result.asm").unwrap();

    let p = p.iter().fold(String::new(), |acc, line| {
        format!("{}{}", acc, encode(line.clone()))
    });

    f.write_all(p.as_bytes()).expect("oops");
}

pub fn gen<R: BufRead>(r: R) -> String {
    let mut p = vec![];

    for (i, line) in r.lines().enumerate() {
        let line = line.unwrap();

        p.push(decode(&line, i as u32).unwrap());
    }

    let program = macro_processor::run(p).unwrap();

    let mut result = String::new();

    for line in program {
        match line {
            Line::Comment(_) => continue,
            Line::Removed => continue,
            _ => result.push_str(&encode(line)),
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mp() {
        let input = "
            MACRO GN &A &B
                MACRO BA &X &Y &A
                    ADD &X
                    ADD &Y
                    ADD &A
                MEND
    
                MACRO BO &X &Y &B
                    SUB &X
                    SUB &Y
                    SUB &B
                MEND
    
                BA &B &A 5
            MEND
    
            GN -5 8
            BO -7 8 4
    
            MULT 9 9";

        let expected = "
            ADD 8
            ADD -5
            ADD 5
            SUB -7
            SUB 8
            SUB 4
            MULT 9 9
        ";
        let result = gen(Cursor::new(input));

        assert_eq!(result, expected)
    }
}
