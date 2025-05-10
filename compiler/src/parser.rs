use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;
use std::collections::LinkedList;

enum Expr {
    Number(i64),
    Identifier(String),
    List(Vec<Expr>),
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<Expr>,
    },
}

#[derive(Parser)]
#[grammar = "../grammar.pest"]
struct GrammarParser;

pub fn run(str: &str) {
    let instructions: Vec<&str> = vec![];
    let parsed = GrammarParser::parse(Rule::program, str).unwrap();
    println!("{parsed:#?}")

    // match GrammarParser::parse(Rule::program, str) {
    //     Ok(pairs) => {
    //         for (i, pair) in pairs.enumerate() {
    //             println!("{}: {:?}, {:?}, {}\n", i, pair.as_node_tag(), pair.as_rule(), pair.as_str());
    //         }
    //     }
    //     Err(error) => panic!("error"),
    // }
}

// fn build_ast(pairs: Pairs<'_, Rule>) -> Expr {
// if pairs.len() == 1 {
//     match pairs.next().expect("a").as_rule() {
//         Rule::program =>
//     }
// } else {
//     for pair in pairs {

//     }
// }
// if pairs.next()

// for pair in pairs {
//     match pair.as_rule() {
//         Rule::program => {
//             let program_items = vec![];

//             for inner in pair.into_inner() {
//                 let ast = build_ast(inner);
//                 program_items.push(ast);
//             }

//             Expr::List(program_items)
//         }
//         _ => Expr::List(vec![]),
//     }
// }

// Expr::Number(23)
// }
