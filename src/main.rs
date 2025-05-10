use std::{
    char,
    io::{self, BufRead, BufReader},
};

fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut repl_lexer = lexer::Lexer::new(reader);

    loop {
        let token = lexer.next();
    }
}

fn repl() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();

    loop {
        line.clear();
        print!("> ");
        io::Write::flush(&mut io::stdout()).unwrap();

        if handle.read_line(&mut line).unwrap() == 0 {
            break;
        }

        let chars = line.chars();

        let repl_lexer = lexer::Lexer::new(chars);
    }
}
