#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LBracket,
    RBracket,
    Plus,
    Minus,
    Mult,
    Div,
    Num {value: u32}
}

pub struct Lexer<'a> {
    source: &'a str,
    pos: usize,
    line: usize
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token>{
        self.next_token()
    }
}

impl Lexer<'_> {

    pub fn new(source: &str) -> Lexer {
        Lexer { source: source, pos: 0, line: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {

        if !self.has_more() {
            return None;
        } 

        self.skip_whitespace();

        match self.peek() {
            Some(c) if "()+-*/".contains(c) => {
                self.pos += 1;
                Some(match c {
                    '(' => Token::LBracket,
                    ')' => Token::RBracket,
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Mult,
                    '/' => Token::Div,
                    _ => unreachable!(),
                })
            },
            Some(_) => Some(self.parse_number()),
            None => None,
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if !self.has_more() || !self.peek().unwrap().is_whitespace() {
                break;
            }

            if self.peek().unwrap() == '\n' || self.peek().unwrap() == '\r' {
                    self.line += 1;
            }

            self.pos += 1;
        }
    }

    fn parse_number(&mut self) -> Token {
        let start = self.pos;

        if !self.peek().unwrap().is_numeric() {
            panic!("Lexing error! NaN");
        }

        while self.has_more()  && self.peek().unwrap().is_numeric() {
                self.pos += 1;
        } 

        let num_str: String = self.source.chars().skip(start).take(self.pos-start).collect();
        let val = num_str.parse::<u32>().unwrap();

        Token::Num { value: val }
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.pos) 
    }

    fn has_more(&self) -> bool {
        return self.pos < self.source.len();
    }
}

