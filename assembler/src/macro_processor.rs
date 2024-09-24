// use std::io::{BufRead, BufReader, Cursor, Read};

// use assembler::Program;
// use common::Word;

// struct MacroProcessor {
//     macros: Vec<Word>,
// }

// impl MacroProcessor {
//     fn new() -> Self {
//         MacroProcessor { macros: vec![] }
//     }

//     fn process<R: Read>(&self, reader: R) -> Program {
//         Some(vec![])
//     }

//     fn run(&self, lines: std::slice::Iter<Word>) -> std::slice::Iter<Word> {
//         let mut a = vec![];

//         for line in lines {
//             a.append(line);
//         }

//         let program = self.process(reader);

//         if let Some(program) = program {
//             Cursor::new(program)
//         } else {
//             panic!("oops")
//         }
//     }
// }

// pub fn run(reader: BufReader) {
//     for line in reader.lines() {
//         println!("{:?}", line);
//     }
// }
