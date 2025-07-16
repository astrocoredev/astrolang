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