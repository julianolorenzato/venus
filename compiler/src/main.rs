mod builtins;
mod parser;
mod scanner;

use std::io::{self, Write};

use scanner::{Scanner, Token};

fn main() {
    loop {
        print!(">> ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read line");
            continue;
        }

        let input = input.trim();

        if input == "\\q" {
            break;
        }

        let mut line_scanner = Scanner::new(input.chars());

        loop {
            let mut tokens: Vec<Token> = vec![];
            match line_scanner.next_token() {
                Ok(Token::EOF) => {
                    break;
                }
                Ok(token) => {
                    println!("{:?}", token);
                    tokens.push(token);
                }
                Err(error) => {
                    println!(
                        "{} at line {} position {}.",
                        error.description, error.line, error.pos
                    );
                }
            }
        }
    }
}
