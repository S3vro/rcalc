use crate::lexer::Token;
use std::fmt;

#[derive(Debug)]
pub enum Expr {
    Int(u32),
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>)
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Int(n) => write!(f, "{n}"),
            Expr::Unary(op, expr) => write!(f, "{}({})", op, expr),
            Expr::Binary(op, lhs, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
        };
        write!(f, "{s}")
    }
}

pub struct TokenSource {
    tokens: Vec<Token>, //TODO: How do i make this a reference?
    pos: usize
}

impl TokenSource {
    
    pub fn new(tokens: Vec<Token>) -> TokenSource {
        Self {tokens: tokens, pos:0}
    }

    pub fn peek(&self) -> Token {
        self.expect_has_more();
        self.tokens[self.pos].clone()
    }

    pub fn has_more(&self) -> bool {
        self.pos < self.tokens.len()
    }

    pub fn consume(&mut self) -> Token {
        let token = self.peek();
        self.pos += 1;
        token
    }

    pub fn expect(&mut self, token: Token) {
        let actual = self.tokens[self.pos].clone();
        if actual != token {
            panic!("Expected {token:?}, but found {actual:?}!");
        } 
        self.pos += 1
    }

    fn expect_has_more(&self) {
        if !self.has_more(){
            panic!("reached end of token stream");
        }
    }
}

pub fn parse_expr(token_src: &mut TokenSource) -> Expr {
    let mut lhs = parse_term(token_src);
    loop {
        if token_src.has_more() && (token_src.peek() == Token::Plus || token_src.peek() == Token::Minus) {
            let op = match token_src.peek() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => panic!("Should never be reached!")
            };

            token_src.consume();
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(parse_term(token_src)));
        } else {
            return lhs;
        }
    }
}

fn parse_term(token_src: &mut TokenSource) -> Expr {
    let mut lhs = parse_factor(token_src);
    loop {
        if token_src.has_more() && (token_src.peek() == Token::Mult || token_src.peek() == Token::Div) {
            let op = match token_src.peek() {
                Token::Mult => BinaryOp::Mul,
                Token::Div => BinaryOp::Div,
                _ => panic!("Should never be reached!")
            };

            token_src.consume();
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(parse_factor(token_src)));
        } else {
            return lhs;
        }
    }
}

fn parse_factor(token_src: &mut TokenSource) -> Expr {
    match token_src.peek() {
        Token::LBracket => {
            token_src.consume();
            let exp = parse_expr(token_src);
            token_src.expect(Token::RBracket);
            exp
        },
        Token::Minus => { 
            token_src.consume();
            Expr::Unary(UnaryOp::Neg, Box::new(parse_factor(token_src)))
        },
        Token::Num {value: n} => {
            token_src.consume();
            Expr::Int(n)
        }
        _ => panic!("Unexpected factor {:?}", token_src.peek())
    }
}
