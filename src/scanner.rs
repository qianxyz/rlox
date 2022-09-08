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
            c if Self::is_digit(c) => self.number(),
            c if Self::is_alphabetic(c) => self.identifier(),

            _ => todo!("unexpected character"),
        };
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source.as_bytes()[self.current - 1]
    }

    fn add_token(&mut self, type_: TokenType) {
        let text = String::from_utf8(
            self.source.as_bytes()[self.start..self.current].to_vec(),
        )
        .unwrap();
        self.tokens.push(Token::new(type_, text, self.line))
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

        let value = String::from_utf8(
            self.source.as_bytes()[self.start + 1..self.current - 1].to_vec(),
        )
        .unwrap();
        self.add_token(TokenType::String { literal: value })
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        // fractional part
        if self.peek() == b'.' && Self::is_digit(self.peek_next()) {
            // consume the `.`
            self.advance();
            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(TokenType::Number {
            literal: String::from_utf8(
                self.source.as_bytes()[self.start..self.current].to_vec(),
            )
            .unwrap()
            .parse()
            .unwrap(),
        })
    }

    fn is_digit(c: u8) -> bool {
        b'0' <= c && c <= b'9'
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current + 1]
        }
    }

    fn is_alphabetic(c: u8) -> bool {
        (b'a' <= c && c <= b'z') || (b'A' <= c && c <= b'Z') || c == b'_'
    }

    fn is_alphanumeric(c: u8) -> bool {
        Self::is_digit(c) || Self::is_alphabetic(c)
    }

    fn identifier(&mut self) {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = String::from_utf8(
            self.source.as_bytes()[self.start..self.current].to_vec(),
        )
        .unwrap();
        let type_ = match text.as_str() {
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
