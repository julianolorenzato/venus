use std::collections::{HashMap, HashSet, LinkedList};
use std::fmt::format;
use std::io::{BufRead, BufReader, Cursor, Read};

use common::Word;

use crate::lexer::{self, Line, Program};

#[derive(Debug)]
struct Macro {
    name: String,
    params: Vec<String>,
    body: Vec<Line>,
}

struct Current {
    macro_name: String,
    macro_params: Vec<String>,
    nest_level: u32,
    macro_start: usize
}
pub struct MacroProcessor {
    // change to set after maybe? and move string name to macro struct
    macro_table: HashMap<String, Macro>,
    program: Program,
    current: Current
}

impl MacroProcessor {
    pub fn new(program: Program) -> Self {
        MacroProcessor {
            macro_table: HashMap::<String, Macro>::new(),
            program,
            current: Current {
                macro_name: String::new(),
                macro_params: vec![],
                macro_start: 0,
                nest_level: 0
            }
        }
    }

    pub fn run(&mut self) -> Result<Program, String> {
        // self.find_macro_definitions();

        let mut p = Vec::<Line>::new();

        // let ms = Vec::<LinkedList<Macro>>::new();
        let mut outer_macro_start = 0;
        let mut nested_level: u32 = 0;
        let mut outer_macro_name: String = String::new();
        let mut outer_macro_params: Vec<String> = Vec::<String>::new();

        for (i, line) in self.program.iter().enumerate() {
            println!("{:?}", self.macro_table);

            match line {
                Line::MacroDef(name, params) => {
                    self.define_macro(name.to_string(), params.to_vec(), i);
                    // start of OUTER macro def
                    // if nested_level == 0 {
                    //     outer_macro_name = name.to_string();
                    //     outer_macro_params = params.to_vec();
                    //     outer_macro_start = i;
                    // }

                    // nested_level += 1;
                }
                Line::MacroEnd => {
                    if nested_level == 0 {
                        return Err("unexpected macro end here".to_string());
                    } else {
                        nested_level -= 1;

                        // end of OUTER macro def
                        if nested_level == 0 {
                            let m = Macro {
                                body: self.program[outer_macro_start..i].to_vec(),
                                params: outer_macro_params.clone(),
                                name: self.current.macro_name
                            };

                            self.macro_table.insert(outer_macro_name.clone(), m);
                        }
                    }
                }
                Line::MacroCall(name, args, _) => {
                    if nested_level != 0 {
                        continue;
                    }

                    if let Some(m) = self.macro_table.get(name) {
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
                                        return Err("unexpected macro end here".to_string());
                                    }

                                    nested_level -= 1;

                                    if nested_level == 0 {
                                        let m = Macro {
                                            body: vec![],
                                            params: macro_params.clone(),
                                            name: self.current.macro_name
                                        };

                                        self.macro_table.insert(macro_name.clone(), m);
                                    }
                                }
                                _ => continue,
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
                }
            }
        }

        Ok(p)
    }

    fn define_macro(&mut self, name: String, params: Vec<String>, start: usize) {
        // start of OUTER macro def
        if self.current.nest_level == 0 {
            self.current.macro_name = name;
            self.current.macro_params = params;
        }

        self.current.nest_level += 1;
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
