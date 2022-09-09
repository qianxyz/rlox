use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &[Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), self.line));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b';' => self.add_token(TokenType::Semicolon),
            b'*' => self.add_token(TokenType::Star),

            b'!' => {
                if self.match_(b'=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            b'=' => {
                if self.match_(b'=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            b'<' => {
                if self.match_(b'=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            b'>' => {
                if self.match_(b'=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }

            b'/' => {
                if self.match_(b'/') {
                    // line of comment
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            b' ' | b'\r' | b'\t' => (),
            b'\n' => self.line += 1,

            b'"' => self.string(),
            c if c.is_digit() => self.number(),
            c if c.is_alphabetic() => self.identifier(),

            _ => todo!("unexpected character"),
        };
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source.as_bytes()[self.current - 1]
    }

    fn add_token(&mut self, type_: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(type_, text.to_string(), self.line))
    }

    fn match_(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            false
        } else if self.source.as_bytes()[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current]
        }
    }

    fn string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            todo!("unterminated string")
        }

        // consume the ending `"`
        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String {
            literal: value.to_string(),
        })
    }

    fn number(&mut self) {
        while self.peek().is_digit() {
            self.advance();
        }

        // fractional part
        if self.peek() == b'.' && self.peek_next().is_digit() {
            // consume the `.`
            self.advance();
            while self.peek().is_digit() {
                self.advance();
            }
        }

        let number: f32 =
            self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number { literal: number })
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current + 1]
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let type_ = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.add_token(type_);
    }
}

trait AsciiExt_ {
    fn is_digit(&self) -> bool;
    fn is_alphabetic(&self) -> bool;
    fn is_alphanumeric(&self) -> bool;
}

impl AsciiExt_ for u8 {
    fn is_digit(&self) -> bool {
        (*self as char).is_digit(10)
    }

    fn is_alphabetic(&self) -> bool {
        (*self as char).is_alphabetic() || *self == b'_'
    }

    fn is_alphanumeric(&self) -> bool {
        (*self as char).is_alphanumeric() || *self == b'_'
    }
}
