use crate::{
    error::ParserError,
    parser::{Line, Program},
};
use std::{collections::{HashMap, LinkedList}, thread, time::Duration};

pub fn run(p: Program) -> Result<Program, String> {
    let mut mp = MacroProcessor::new(p);
    mp.run(0, &mut LinkedList::new())
}

#[derive(Debug)]
struct Macro {
    // name: String,
    params: Vec<String>,
    body: Vec<Line>,
}

impl Macro {
    fn new(params: Vec<String>, body: Vec<Line>) -> Macro {
        Macro { params, body }
    }

    fn apply(&self) -> Vec<Line> {
        let code = vec![];

        // for line in &self.body {
        //     match line {
        //         Line::Regular(_, b, c, d) => {
        //             Macro::check_ampersand(a);
        //             Macro::check_ampersand(a);
        //             Macro::check_ampersand(a);
        //             Macro::check_ampersand(a);
        //         }
        //         _ => continue,
        //     }
        // }

        code
    }
}

fn check_params(token: &mut String, params: Vec<String>) {
    // if params.contains(token) {
    //     // token = &String::from("A");
    //     token.clear();
    //     token.push_str(string);
    // }
    // if token.starts_with("&") {
    //     token.drain(..1);
    // }
}

struct MacroProcessor {
    // change to set after maybe? and move string name to macro struct
    macro_table: HashMap<String, Macro>,
    program: Program,
    params_stack: LinkedList<Vec<String>>
}

impl MacroProcessor {
    fn new(program: Program) -> Self {
        MacroProcessor {
            macro_table: HashMap::<String, Macro>::new(),
            program,
            params_stack: LinkedList::new()
        }
    }

    fn run(
        &mut self,
        mut i: usize,
        params_stack: &mut LinkedList<Vec<String>>,
    ) -> Result<Program, String> {
        // let mut i = 0;
        while i < self.program.len() {
            // thread::sleep(Duration::from_secs(1));
            // println!("{i}");
            // println!("{:?}", self.program[i]);
            // println!("{:#?}", self.params_stack);
            match &self.program[i].to_owned() {
                Line::MacroDef(name, params) => {
                    let start = i;
                    let mut level = 0;
                    // let mut p_stack: LinkedList<Vec<String>> = LinkedList::new();
                    self.params_stack.push_front(params.to_vec());

                    loop {
                        i += 1;
                        // println!("{i}");
                        // println!("{:?}", self.program[i]);
                        // println!("{:#?}", self.params_stack);

                        if i > self.program.len() {
                            return Err(format!("MEND not found for {} macro.", name));
                        }

                        match &self.program[i] {
                            Line::MacroDef(_, params) => {
                                level += 1;
                                self.params_stack.push_front(params.to_vec());
                                // p_stack.push_front(params.to_vec());
                            }
                            Line::MacroEnd => {
                                self.params_stack.pop_front();
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
                        self.run(i, params_stack)?;
                    } else {
                        return Err(format!("macro {} not defined", name));
                    }
                }
                _ => {}
            }

            i += 1;
        }

        Ok(self.program.clone())
    }
}
