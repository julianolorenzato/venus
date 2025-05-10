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
    Equal,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Not,
    Or,
    And,
    If,
    Else,
    Loop,
    Print,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    Identifier(String),
    Keyword(Keyword),
    Number(f64),
    String(String),
    Space,
    EOF,
}

// pub static symbol_table = HashMap::new();

pub struct Scanner<I>
where
    I: Iterator<Item = char>,
{
    // input: std::iter::Peekable<I>,
    input: MultiPeek<I>,
    current_char: Option<char>,
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
            current_char: None,
            curr_line_idx: 0,
            curr_pos_idx: 0,
        };
    }

    pub fn next_token(&mut self) -> Result<Token, LexicalError> {
        if let Some(c) = self.advance() {
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
                self.input.nth(advance_count - 1);
            }
            true
        } else {
            false
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
            // "->>" => Token::Keyword(Keyword::DoubleArrow),
            "import" => Token::Keyword(Keyword::Import),

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

    fn check_true(&mut self) -> bool {
        for ch in "rue".chars() {
            if let Some(c) = self.input.peek() {
                if *c != ch {
                    return false;
                }
            } else {
                return false;
            }
        }

        for _ in 0..4 {
            self.input.next();
        }

        true
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
