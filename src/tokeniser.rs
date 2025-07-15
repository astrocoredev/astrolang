use std::{iter};

pub struct Tokeniser {
    input: String,
}

impl Tokeniser {
    pub fn new(input: String) -> Self {
        Self {
            input
        }
    }
    
    pub fn run(&mut self) -> Vec<Token> {
        let mut iter = self.input.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(chr) = iter.next() {
            match chr {
                chr if chr.is_whitespace() => continue,

                // Handle Strings
                '"' | '\'' => {
                    let token: String = iter::from_fn(
                            || iter.by_ref().next_if(|ch| *ch != chr)
                        )
                        .collect();
                    tokens.push(Token::String(token));
                },

                // Handle Numbers
                chr if chr.is_ascii_digit() => {
                    let item: String = iter::once(chr)
                        .chain(iter::from_fn(
                            || iter.by_ref().next_if(|ch| ch.is_ascii_digit() || *ch == '.')
                        ))
                        .collect();
                    tokens.push(Token::Number(item));
                }

                // Handle Keywords, Types or Identifiers
                chr if chr.is_alphabetic() => {
                    let item: String = iter::once(chr)
                        .chain(iter::from_fn(
                            || iter.by_ref().next_if(|ch| ch.is_alphanumeric() || *ch == '_')
                        ))
                        .collect();

                    let token = match item.as_ref() {
                        // Keywords
                        "let" => Token::Let,
                        "while" => Token::While,
                        "for" => Token::For,
                        "fn" => Token::Fn,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "match" => Token::Match,
                        "return" => Token::Return,

                        "true" => Token::Bool(true),
                        "false" => Token::Bool(false),

                        _ => Token::Identifier(item),
                    };
                    tokens.push(token);
                },

                // Operators
                '+' => tokens.push(Token::Plus),
                '*' => tokens.push(Token::Asterisk),
                '/' => tokens.push(Token::ForSlash),
                ';' => tokens.push(Token::Semicolon),
                ':' => tokens.push(Token::Colon),
                '.' => tokens.push(Token::Period),
                ',' => tokens.push(Token::Comma),
                '!' => tokens.push(Token::LogNot),
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '{' => tokens.push(Token::LeftCurlyBracket),
                '}' => tokens.push(Token::RightCurlyBracket),
                '<' => tokens.push(Token::LeftAngleBracket),
                '>' => tokens.push(Token::RightAngleBracket),
                '[' => tokens.push(Token::LeftSqreBracket),
                ']' => tokens.push(Token::RightSqreBracket),
                '\\' => tokens.push(Token::BackSlash),

                '-' | '=' => {
                    let item: String = iter::once(chr)
                        .chain(iter::from_fn(
                            || iter.by_ref().next_if(|ch| *ch == '>')
                        ))
                        .collect();
                    let token = match item.as_ref() {
                        "-" => Token::Hyphen,
                        "=" => Token::Equals,
                        "=>" => Token::EqualArrow,
                        "->" => Token::HyphenArrow,
                        _ => unreachable!(),
                    };
                    tokens.push(token);
                },
                '|' | '&' => {
                    let item: String = iter::once(chr)
                        .chain(iter::from_fn(
                            || iter.by_ref().next_if(|ch| *ch == chr)
                        ))
                        .collect();
                    let token = match item.as_ref() {
                        "||" => Token::LogOr,
                        "&&" => Token::LogAnd,
                        _ => unreachable!(),
                    };
                    tokens.push(token);
                }
                _ => todo!(),
            }
        }
        tokens
    }
}

#[derive(Debug)]
pub enum Token {
    // Keywords
    Let, While, For,
    Fn, If, Else, Match,
    Return,

    // Identifier
    Identifier(String),

    // Arithmetic Operators
    Plus, Hyphen,
    Asterisk, ForSlash,

    // Syntax Operators
    Equals, Comma,
    EqualArrow, HyphenArrow,
    LogNot, LogAnd, LogOr,
    Semicolon, Colon, Period, BackSlash,
    LeftParen, RightParen,
    LeftCurlyBracket, RightCurlyBracket,
    LeftAngleBracket, RightAngleBracket,
    LeftSqreBracket, RightSqreBracket,

    // Values
    String(String),
    Number(String),
    Bool(bool),
}