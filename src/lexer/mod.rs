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
        if self.pos >= self.source.len() {
            return None;
        } 

        self.skip_whitespace();

        let token = match self.peek() {
                '(' => {self.pos += 1; Token::LBracket},
                ')' => {self.pos += 1; Token::RBracket},
                '+' => {self.pos += 1; Token::Plus},
                '-' => {self.pos += 1; Token::Minus},
                '*' => {self.pos += 1; Token::Mult},
                '/' => {self.pos += 1; Token::Div},
                _ => self.parse_number(),
        };

        Some(token)
    }

    fn skip_whitespace(&mut self) {
        loop {
            if !Lexer::is_whitespace(self.peek()) {
                break;
            }

            if self.peek() == '\n' || self.peek() == '\r' {
                    self.line += 1;
            }

            self.pos += 1;
        }
    }

    fn parse_number(&mut self) -> Token {
        let start = self.pos;

        if !Lexer::is_numeric(self.peek()) {
            panic!("Lexing error! NaN");
        }

        while self.has_more()  && Lexer::is_numeric(self.peek()) {
                self.pos += 1;
        } 

        let num_str: String = self.source.chars().skip(start).take(self.pos-start).collect();
        let val = num_str.parse::<u32>().unwrap();

        Token::Num { value: val }
    }

    fn peek(&self) -> char {
        if let Some(c) = self.source.chars().nth(self.pos) {
            c
        } else {
            panic!("Could not peek no chars found");
        }
    }

    fn has_more(&self) -> bool {
        return self.pos < self.source.len();
    }


    fn is_numeric(c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_whitespace(c:char) -> bool {
        return c == ' ' || c == '\n' || c == '\r' || c == '\t';
    }
}

