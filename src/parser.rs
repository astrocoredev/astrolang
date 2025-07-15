use std::process;

use crate::tokeniser::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn parse(&mut self) -> Expr {
        let mut expressions: Vec<Box<Expr>> = Vec::new();
        while let Some(token) = self.peek() {
            let expr = match token {
                Token::Let => self.parse_variable(),
                Token::Fn => self.parse_function(),
                Token::If => self.parse_if(),
                Token::While => self.parse_while(),
                Token::Match => self.parse_match(),
                Token::For => self.parse_for(),
                Token::Return => self.parse_return(),
                Token::Number(_)
                | Token::String(_)
                | Token::Bool(_)
                => self.parse_value(),
                Token::LeftParen 
                | Token::LeftCurlyBracket
                | Token::LeftSqreBracket
                | Token::LeftAngleBracket
                => {
                    self.next();
                    let exprbr = self.parse();
                    match self.next() {
                        Some(Token::RightParen)
                        | Some(Token::RightCurlyBracket)
                        | Some(Token::RightSqreBracket)
                        | Some(Token::RightAngleBracket)
                        => exprbr,
                        _ => {
                            eprintln!("Expected right bracket");
                            process::exit(0)
                        }
                    }
                }
                _ => todo!(),
            };
            expressions.push(Box::new(expr));
        }
        Expr::Body { expressions }
    }

    fn parse_variable(&mut self) -> Expr {
        todo!()
    }

    fn parse_function(&mut self) -> Expr {
        todo!()
    }

    fn parse_if(&mut self) -> Expr {
        todo!()
    }

    fn parse_while(&mut self) -> Expr {
        todo!()
    }

    fn parse_match(&mut self) -> Expr {
        unimplemented!()
    }

    fn parse_for(&mut self) -> Expr {
        unimplemented!()
    }

    fn parse_return(&mut self) -> Expr {
        todo!()
    }

    fn parse_value(&mut self) -> Expr {
        match self.next().unwrap() {
            Token::Number(val) => Expr::Int(val.parse::<i128>().unwrap()),
            Token::String(val) => Expr::Str(val.clone()),
            Token::Bool(val) => Expr::Bool(*val),
            _ => unreachable!()
        }
    }
}

pub enum Expr {
    // Arithmetic Operators
    Add { left: Box<Expr>, right: Box<Expr> },
    Subtract { left: Box<Expr>, right: Box<Expr> },
    Multiply { left: Box<Expr>, right: Box<Expr> },
    Divide { left: Box<Expr>, right: Box<Expr> },

    // Logical Operators
    Not { expr: Box<Expr> },
    And { left: Box<Expr>, right: Box<Expr> },
    Or  { left: Box<Expr>, right: Box<Expr> },

    // Functions and Variables
    Function { name: String, parameters: Box<Expr>, body: Box<Expr> },
    Variable { name: String, value: Box<Expr> },

    // Control Flow
    If { cond: Box<Expr>, body_true: Box<Expr>, body_false: Box<Expr> },
    While { cond: Box<Expr>, body: Box<Expr> },

    // Body
    Body { expressions: Vec<Box<Expr>> },

    Int(i128),
    Str(String),
    Bool(bool)
}