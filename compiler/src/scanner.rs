use core::panic;

use itertools::{multipeek, Itertools, MultiPeek, PeekNth, PeekingNext};

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Keyword {
    Define,
    DefineP,
    DoubleArrow,
    Fun,
    Import,
    True,
    False,
    Quote,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    DoubleEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Not,
    Or,
    And,
    If,
    Print,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    Identifier(String),
    Keyword(Keyword),
    Integer(i32),
    Float(f64),
    String(String),
    Space,
    EOF,
}

// pub static symbol_table = HashMap::new();

pub struct Scanner<I>
where
    I: Iterator<Item = char>,
{
    input: MultiPeek<I>,
    // Atualizar a tablela de simbolos é opcional
    // symbol_table: HashMap<String, String>,
    curr_line_idx: usize,
    curr_pos_idx: usize,
}

#[derive(Debug)]
pub struct LexicalError {
    pub line: usize,
    pub pos: usize,
    pub description: &'static str,
}

impl LexicalError {
    fn new(line: usize, pos: usize, desc: &'static str) -> LexicalError {
        LexicalError {
            line,
            pos,
            description: desc,
        }
    }
}

impl<I> Scanner<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(mut input: I) -> Self {
        return Scanner {
            input: multipeek(input),
            // symbol_table: HashMap::new(),
            curr_line_idx: 0,
            curr_pos_idx: 0,
        };
    }

    pub fn next_token(&mut self) -> Result<Token, LexicalError> {
        if let Some(c) = self.advance() {
            // Por enqaunto só estou dando match específico em keywords que nao começam
            // com character alfabético, e deixando a função identifier_or_keyword
            // lidar com o resto das keywords, mas a função match_exact serviria
            // para lidar com qualquer lexeme que tenha caractéres previamente
            // conhecidos, isso inclui keywords.
            match c {
                '(' => Ok(Token::LeftParen),
                ')' => Ok(Token::RightParen),
                ' ' | '\t' | '\n' => Ok(self.space()),
                '+' => Ok(Token::Keyword(Keyword::Plus)),
                '-' if self.match_exact('-', "->>") => Ok(Token::Keyword(Keyword::DoubleArrow)),
                '-' => Ok(Token::Keyword(Keyword::Minus)),
                '*' => Ok(Token::Keyword(Keyword::Star)),
                '/' => Ok(Token::Keyword(Keyword::Slash)),
                '%' => Ok(Token::Keyword(Keyword::Percent)),
                '\'' => Ok(Token::Keyword(Keyword::Quote)),
                '=' if self.match_exact('=', "==") => Ok(Token::Keyword(Keyword::DoubleEqual)),
                '!' if self.match_exact('!', "!=") => Ok(Token::Keyword(Keyword::BangEqual)),
                '<' if self.match_exact('<', "<=") => Ok(Token::Keyword(Keyword::LessEqual)),
                '<' => Ok(Token::Keyword(Keyword::Less)),
                '>' if self.match_exact('>', ">=") => Ok(Token::Keyword(Keyword::GreaterEqual)),
                '>' => Ok(Token::Keyword(Keyword::Greater)),
                c if c.is_ascii_digit() => self.integer_or_float(c),
                c if c.is_ascii_alphabetic() => Ok(self.identifier_or_keyword(c)),
                _ => Err(LexicalError::new(
                    self.curr_line_idx,
                    self.curr_pos_idx,
                    "unrecognized token",
                )),
            }
        } else {
            Ok(Token::EOF)
        }
    }

    fn advance(&mut self) -> Option<char> {
        let next_char = self.input.next();

        match next_char {
            Some(c) if c == '\n' => {
                self.curr_pos_idx = 0;
                self.curr_line_idx += 1;
            }
            Some(_) => {
                self.curr_pos_idx += 1;
            }
            None => {}
        }

        next_char
    }

    fn match_exact(&mut self, first_char: char, pattern: &str) -> bool {
        if pattern.chars().next() != Some(first_char) {
            panic!("first argument of 'match_exact' must be the first char of the second argument");
        }

        let mut lexeme = vec![first_char];
        let mut advance_count = 0;

        // skip the first one because we already have the first_char
        for c in pattern.chars().skip(1) {
            match self.input.peek().copied() {
                Some(ch) if ch == c => {
                    advance_count += 1;
                    lexeme.push(ch);
                }
                _ => {
                    break;
                }
            }
        }

        if lexeme == pattern.chars().collect::<Vec<char>>() {
            if advance_count > 0 {
                // should decrement 1 because nth(0) advances 1 position
                //
                // TODO! we should advance the lexer advance_count times
                // instead of doing this:
                self.input.nth(advance_count - 1);
            }
            true
        } else {
            false
        }
    }

    fn integer_or_float(&mut self, first_char: char) -> Result<Token, LexicalError> {
        let mut digit_sequence = vec![first_char];
        let mut dot = false;

        // talvez escrever uma função para abstrair essa sequencia
        // de while let, match, break etc...
        while let Some(c) = self.input.peek() {
            match c {
                c if c.is_ascii_digit() => {
                    digit_sequence.push(*c);
                    self.input.next();
                }
                c if *c == '.' && !dot => {
                    // we just consume the dot if it is followed by
                    // an ascii digit
                    if let Some(forward_c) = self.input.peek() {
                        if forward_c.is_ascii_digit() {
                            self.advance();
                            digit_sequence.push('.');
                            dot = true;
                        }

                        // we dont care about double peeking here, because
                        // its the last time we will peek before advance
                        // the lexer in the outer function
                    }
                }
                _ => {
                    break;
                }
            }
        }

        if dot {
            let float: f64 = digit_sequence.iter().collect::<String>().parse().unwrap();
            Ok(Token::Float(float))
        } else {
            let integer: i32 = digit_sequence.iter().collect::<String>().parse().unwrap();
            Ok(Token::Integer(integer))
        }
    }

    fn identifier_or_keyword(&mut self, first_char: char) -> Token {
        let mut lexeme = vec![first_char];

        while let Some(c) = self.input.peek() {
            match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' | '-' => {
                    lexeme.push(self.advance().unwrap());
                }
                _ => {
                    break;
                }
            }
        }

        let lexeme: String = lexeme.iter().collect();

        match lexeme.as_str() {
            "define" => Token::Keyword(Keyword::Define),
            "define-p" => Token::Keyword(Keyword::DefineP),
            "fun" => Token::Keyword(Keyword::Fun),
            "import" => Token::Keyword(Keyword::Import),
            "true" => Token::Keyword(Keyword::True),
            "false" => Token::Keyword(Keyword::False),
            "or" => Token::Keyword(Keyword::Or),
            "and" => Token::Keyword(Keyword::And),
            "not" => Token::Keyword(Keyword::Not),
            "if" => Token::Keyword(Keyword::If),
            "print" => Token::Keyword(Keyword::Print),
            _ => Token::Identifier(lexeme),
        }
    }

    fn space(&mut self) -> Token {
        while let Some(c) = self.input.peek() {
            match c {
                ' ' | '\t' | '\n' => {
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }

        Token::Space
    }

    fn string(&mut self) -> Result<Token, LexicalError> {
        let str = String::new();
        while let Some(c) = self.input.peek() {
            match c {
                '"' => {
                    self.advance();
                    if let Some(c) = self.input.peek() {
                        if *c == '"' {
                            return Err(LexicalError::new(
                                self.curr_line_idx,
                                self.curr_pos_idx,
                                "character '\"' after string.",
                            ));
                        }
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }
        Ok(Token::String("aaa".to_owned()))
    }
}

// impl<I> Iterator for Scanner<I>
// where
//     I: Iterator<Item = char>,
// {
//     type Item = Token;

//     fn next(&mut self) -> Option<Self::Item> {
//         let token = self.next_token();

//         if token == Token::EOF {
//             None
//         } else {
//             Some(token)
//         }
//     }
// }
