use crate::TokenKind::*;

#[cfg(test)]
mod tests;

mod cursor;
use cursor::Cursor;

fn main() {
    let tokens = tokenize(&"0: -> 1 => 2");

    for token in tokens {
        println!("{:?}", token)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

/// Enum representing common lexeme types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    /// Any whitespace characters sequence.
    Whitespace,
    Literal {
        kind: LiteralKind,
    },
    /// "+"
    Plus,
    /// "-"
    Minus,
    /// "*"
    Star,
    /// "/"
    Slash,
    /// ":"
    Colon,
    /// "="
    Equals,
    /// "->"
    Pipe,
    /// "=>"
    Assignment,
    /// Unknown token, not expected by the lexer, e.g. "№"
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    /// "12_u8", "0o100", "0b120i99"
    Int,
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(mut input: &str) -> impl Iterator<Item = Token> + '_ {
    std::iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }
        let token = first_token(input);
        input = &input[token.len..];
        Some(token)
    })
}

/// Parses the first token from the provided input string.
pub fn first_token(input: &str) -> Token {
    debug_assert!(!input.is_empty());
    Cursor::new(input).advance_token()
}

pub fn is_whitespace(c: char) -> bool {
    match c {
        '\t' | '\n' | 'r' | ' ' => true,
        _ => false,
    }
}

impl Cursor<'_> {
    /// Parses a token from the input string.
    fn advance_token(&mut self) -> Token {
        let first_char = self.bump().unwrap();
        let token_kind = match first_char {
            c if is_whitespace(c) => self.whitespace(),
            c @ '0'..='9' => {
                let kind = self.number();

                TokenKind::Literal { kind }
            }
            '+' => Plus,
            '-' => {
                if self.first() == '>' {
                    self.pipe()
                } else {
                    Minus
                }
            }
            '*' => Star,
            '/' => Slash,
            '=' => {
                if self.first() == '>' {
                    self.assignment()
                } else {
                    Equals
                }
            }
            ':' => Colon,
            _ => Unknown,
        };
        Token::new(token_kind, self.len_consumed())
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    /// Returns amount of eaten symbols.
    fn eat_while<F>(&mut self, mut predicate: F) -> usize
    where
        F: FnMut(char) -> bool,
    {
        let mut eaten: usize = 0;
        while predicate(self.first()) && !self.is_eof() {
            eaten += 1;
            self.bump();
        }

        eaten
    }

    fn whitespace(&mut self) -> TokenKind {
        debug_assert!(is_whitespace(self.prev()));
        self.eat_while(is_whitespace);
        Whitespace
    }

    fn number(&mut self) -> LiteralKind {
        self.eat_digits();
        LiteralKind::Int
    }

    fn pipe(&mut self) -> TokenKind {
        self.bump();
        Pipe
    }

    fn assignment(&mut self) -> TokenKind {
        self.bump();
        Assignment
    }

    fn eat_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '_' => {
                    self.bump();
                }
                '0'..='9' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }
}
