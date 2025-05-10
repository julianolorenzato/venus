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
