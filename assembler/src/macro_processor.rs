use std::collections::{HashMap, HashSet, LinkedList};
use std::fmt::format;
use std::io::{BufRead, BufReader, Cursor, Read};

use common::Word;

use crate::lexer::{self, Line, Program};

#[derive(Debug)]
struct Macro {
    // name: String,
    params: Vec<String>,
    body: Vec<Line>,
    macro_level: u32, // inner_macros: Vec<Macro>
}

pub struct MacroProcessor {
    // change to set after maybe?
    macros: HashMap<String, Macro>,
    program: Program,
    macro_level: u32,
}

impl MacroProcessor {
    pub fn new(program: Program) -> Self {
        MacroProcessor {
            macros: HashMap::<String, Macro>::new(),
            program,
            macro_level: 0,
        }
    }

    pub fn run(&mut self) -> Result<Program, String> {
        // self.find_macro_definitions();

        let mut p = Vec::<Line>::new();

        // let ms = Vec::<LinkedList<Macro>>::new();
        let mut nested_level: u32 = 0;
        let mut outer_macro_name: String = String::new();
        let mut outer_macro_params: Vec<String> = Vec::<String>::new();

        for line in &self.program {
            println!("{:?}", self.macros);

            match line {
                Line::MacroDef(name, params) => {
                    // start of OUTER macro def
                    if nested_level == 0 {
                        outer_macro_name = name.to_string();
                        outer_macro_params = params.to_vec();
                    }

                    nested_level += 1;
                }
                Line::MacroEnd => {
                    if nested_level == 0 {
                        return Err("unexpected macro end here".to_string())
                    } else {   
                        nested_level -= 1;

                        // end of OUTER macro def
                        if nested_level == 0 {
                            let m = Macro{
                                body: vec![],
                                params: outer_macro_params.clone(),
                                macro_level: 0
                            };

                            self.macros.insert(outer_macro_name.clone(), m);
                        }
                    }
                }
                Line::MacroCall(name, args, _) => {
                    if nested_level != 0 {
                        continue;
                    }

                    if let Some(m) = self.macros.get(name) {
                        // not handling args yet
                        p.extend_from_slice(&m.body);

                        let mut nested_level: u32 = 0;
                        let mut macro_name: String = String::new();
                        let mut macro_params: Vec<String> = Vec::<String>::new();

                        for line in m.body.clone() {
                            match line {
                                Line::MacroDef(name, params) => {
                                    if nested_level == 0 {
                                        macro_name = name.to_string();
                                        macro_params = params.to_vec();
                                    }

                                    nested_level += 1;
                                }
                                Line::MacroEnd => {
                                    if nested_level == 0 {
                                        return Err("unexpected macro end here".to_string())
                                    }

                                    nested_level -= 1;

                                    if nested_level == 0 {
                                        let m = Macro{
                                            body: vec![],
                                            params: macro_params.clone(),
                                            macro_level: 0
                                        };

                                        self.macros.insert(macro_name.clone(), m);
                                    }
                                }
                                _ => continue
                            }
                        }
                    } else {
                        return Err(format!("macro {} not defined.", name)); // refac to put line
                    }
                }
                other_line => {
                    if nested_level != 0 {
                        continue;
                    }
                    p.push(other_line.clone())
                },
            }
        }

        Ok(p)
    }

    // macro definition and macro signature is the same, need to choose only one name to it
    fn find_macro_definitions(&mut self) {
        // let first_macro_def = self
        //     .program
        //     .iter()
        //     .enumerate()
        //     .find(|(_, line)| matches!(line, Line::MacroSignature(_, _)));

        // if let Some((i, Line::MacroSignature(name, args))) = first_macro_def {
        //     self.macro_body(i, name.to_string(), args.to_vec());
        // }

        for (i, line) in self.program.iter().enumerate() {
            if let Line::MacroDef(name, params) = line {
                // self.macro_body(i);

                // self.find_macro_definitions();
                // let m = Macro {
                //     // name: name.to_string(),
                //     params: params.to_vec(),
                //     body:
                // };

                // self.macros.push(m);
            }
        }
    }

    fn macro_body(&mut self, mut macro_start: usize, name: String, params: Vec<String>) {
        // let mut content = Vec::<Line>::new();

        for line in &self.program[macro_start..] {
            macro_start += 1;

            match line {
                Line::MacroEnd => {
                    // let m = Macro {
                    //     name: name.clone(),
                    //     params: params.clone(),
                    //     body: &self.program[2..3],
                    // };

                    // self.macros.push(m);
                }
                _ => continue,
            }

            // content.push(line.clone());
        }
    }
}
