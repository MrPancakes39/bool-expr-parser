use std::iter::Peekable;
use std::{fmt::Display, str::Chars};

// ALLOWED CHARACTERS: '(', ')', '&', '|', '!', 't', 'f', and ','
#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    TRUE,
    FALSE,

    NOT,
    OR,
    AND,
    LPAREN,
    RPAREN,
    COMMA,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::ILLEGAL => write!(f, "ILLEGAL"),
            Token::EOF => write!(f, "EOF"),

            Token::TRUE => write!(f, "TRUE"),
            Token::FALSE => write!(f, "FALSE"),

            Token::NOT => write!(f, "NOT"),
            Token::OR => write!(f, "OR"),
            Token::AND => write!(f, "AND"),
            Token::LPAREN => write!(f, "LPAREN"),
            Token::RPAREN => write!(f, "RPAREN"),
            Token::COMMA => write!(f, "COMMA"),
        }
    }
}

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char,             // current char under examination
}

impl<'lex> Lexer<'lex> {
    pub fn new(input: &'lex str) -> Self {
        let mut lex = Self {
            iter: input.chars().peekable(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lex.read_char();

        lex
    }

    pub fn read_char(&mut self) {
        self.ch = match self.iter.next() {
            Some(c) => c,
            None => '\0',
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            't' => Token::TRUE,
            'f' => Token::FALSE,
            '!' => Token::NOT,
            '|' => Token::OR,
            '&' => Token::AND,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            ',' => Token::COMMA,
            '\0' => Token::EOF,
            _ => Token::ILLEGAL,
        };
        if tok == Token::ILLEGAL {
            panic!(
                "Illegal character '{}' at position {}",
                self.ch, self.position
            );
        }

        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "|(!(&(|(f))),f)";
        let mut lexer = Lexer::new(input);

        let tokens = vec![
            Token::OR,
            Token::LPAREN,
            Token::NOT,
            Token::LPAREN,
            Token::AND,
            Token::LPAREN,
            Token::OR,
            Token::LPAREN,
            Token::FALSE,
            Token::RPAREN,
            Token::RPAREN,
            Token::RPAREN,
            Token::COMMA,
            Token::FALSE,
            Token::RPAREN,
            Token::EOF,
        ];

        for token in tokens {
            let next_token = lexer.next_token();
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
    }
}
