mod macro_processor;
mod lexer;

use clap::Parser;
use lexer::Line;
use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Parser)]
#[command(author, version, about="Assemble Venus mnemonic code.", long_about = None)]
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

    let mut program = Vec::<Line>::new();

    for (i, line) in reader.lines().enumerate() {
        let line_index = i as u32;

        match lexer::decode(&line.unwrap(), line_index) {
            Ok(line) => {
                // println!("{:?}", line);
                program.push(line);
            },
            Err(err) => println!("{}", err)
        };
    }
    
    let mut mp = macro_processor::MacroProcessor::new(program);

    let p = mp.run();

    println!("{:#?}", p);

    // let mut asm = Assembler::new(&args.filepath);

    // if args.macro_support {
    //     reader = macro_processor::run(file);
    //     file = macro_processor::run(r);
    // }

    // assembler::run(reader);

    // match asm.run() {
    //     Ok(program) => println!("{:?}", program),
    //     Err(err) => panic!("{err}"),
    // }
}
