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
}

struct Current {
    macro_name: String,
    macro_params: Vec<String>,
    nest_level: u32,
    macro_start: usize,
}
pub struct MacroProcessor {
    // change to set after maybe? and move string name to macro struct
    macro_table: HashMap<String, Macro>,
    program: Program,
    current: Current,
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
                nest_level: 0,
            },
        }
    }

    pub fn run3(&mut self, mut i: usize) -> Result<Program, String> {
        // let mut i = 0;
        while i < self.program.len() {
            match &self.program[i].to_owned() {
                Line::MacroDef(name, params) => {
                    let start = i;
                    let mut level = 0;

                    loop {
                        i += 1;

                        if i > self.program.len() {
                            return Err(format!("MEND not found for {} macro.", name));
                        }

                        match &self.program[i] {
                            Line::MacroDef(_, _) => level += 1,
                            Line::MacroEnd => {
                                if level == 0 {
                                    // increments 1 to skip macro signature line in the body
                                    let body = self.program[start + 1..i].to_vec();

                                    // replace macro definition by removed lines
                                    self.program
                                        .splice(start..=i, vec![Line::Removed; i - start + 1]);

                                    self.macro_table.insert(
                                        name.to_string(),
                                        Macro {
                                            params: params.to_vec(),
                                            body,
                                        },
                                    );

                                    break;
                                }

                                level -= 1;
                            }
                            _ => continue,
                        }
                    }
                }
                Line::MacroEnd => return Err(format!("unexpected MEND here (line {i}).")),
                Line::MacroCall(name, args, _) => {
                    if let Some(m) = self.macro_table.get(name) {
                        // need to adjust args...
                        self.program.splice(i..=i, m.body.clone());
                        self.run3(i)?;
                    } else {
                        return Err(format!("macro {} not defined", name));
                    }
                }
                _ => {}
            }

            i += 1;
        }

        for (i, l) in self.program.iter().enumerate() {
            println!("{}:{:?}", i + 1, l)
        }

        Ok(self.program.clone())
    }

    pub fn run(&mut self) -> Result<Program, String> {
        let mut p = Vec::<Line>::new();

        for (i, line) in self.program.clone().iter().enumerate() {
            println!("{:?}", self.macro_table);

            match line {
                Line::MacroDef(name, params) => {
                    self.def_macro(name.to_string(), params.to_vec(), i)
                }
                Line::MacroEnd => {
                    self.end_macro(i)?;
                }
                Line::MacroCall(name, args, _) => {
                    self.expand_macro(name, &mut p)?;
                }
                other_line => {
                    if self.current.nest_level != 0 {
                        continue;
                    }
                    p.push(other_line.clone())
                }
            }
        }

        Ok(p)
    }

    fn def_macro(&mut self, name: String, params: Vec<String>, start: usize) {
        // start of OUTER macro def
        if self.current.nest_level == 0 {
            self.current.macro_name = name;
            self.current.macro_start = start;
            self.current.macro_params = params;
        }

        self.current.nest_level += 1;
    }

    fn end_macro(&mut self, end: usize) -> Result<(), &str> {
        if self.current.nest_level == 0 {
            return Err("unexpected macro end here");
        } else {
            self.current.nest_level -= 1;

            // end of OUTER macro def
            if self.current.nest_level == 0 {
                let m = Macro {
                    body: self.program[self.current.macro_start..end].to_vec(),
                    params: self.current.macro_params.clone(),
                    // name: self.current.macro_name.to_string(),
                };

                self.macro_table.insert(self.current.macro_name.clone(), m);
            }
        }

        Ok(())
    }

    fn expand_macro(&mut self, name: &str, p: &mut Program) -> Result<(), String> {
        if self.current.nest_level != 0 {
            return Ok(());
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
                                // name: self.current.macro_name.to_string(),
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

        Ok(())
    }
}
