use thiserror::Error;

use crate::token::{Token, TokenType};

/// A scanner that reads source code and produces a list of tokens or errors.
/// NOTE: Only ASCII characters are supported.
pub struct Scanner {
    /// The source code to scan.
    source: String,
    /// The output list of tokens.
    tokens: Vec<Token>,
    /// The list of scanner errors.
    errors: Vec<ScannerError>,
    /// The start of the current lexeme being scanned.
    start: usize,
    /// The current character being scanned.
    current: usize,
    /// The current line number.
    line: usize,
}

#[derive(Error, Debug)]
pub enum ScannerErrorType {
    #[error("unexpected character '{}'", *.0 as char)]
    UnexpectedCharacter(u8),
    #[error("unterminated string")]
    UnterminatedString,
}

#[derive(Error, Debug)]
#[error("[line {line}] Error: {error}")]
pub struct ScannerError {
    error: ScannerErrorType,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Entry point for scanning.
    pub fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<ScannerError>> {
        while !self.is_at_end() {
            // current parse point is the start of the next lexeme
            self.start = self.current;
            self.scan_token();
        }

        if self.errors.is_empty() {
            // add EOF token
            self.tokens
                .push(Token::new(TokenType::Eof, String::new(), self.line));
            Ok(self.tokens)
        } else {
            Err(self.errors)
        }
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
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() || c == b'_' => self.identifier(),

            _ => self.add_error(ScannerErrorType::UnexpectedCharacter(c)),
        };
    }

    /// Consumes the current character and returns it.
    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source.as_bytes()[self.current - 1]
    }

    fn add_token(&mut self, type_: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(type_, text.to_string(), self.line))
    }

    fn add_error(&mut self, error: ScannerErrorType) {
        self.errors.push(ScannerError {
            error,
            line: self.line,
        });
    }

    /// Consumes the current character if it matches the expected character.
    fn match_(&mut self, expected: u8) -> bool {
        match self.source.as_bytes().get(self.current) {
            None => false,
            Some(&c) if c != expected => false,
            _ => {
                self.current += 1;
                true
            }
        }
    }

    /// Returns the current character without consuming it.
    fn peek(&self) -> u8 {
        match self.source.as_bytes().get(self.current) {
            None => b'\0',
            Some(&c) => c,
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
            self.add_error(ScannerErrorType::UnterminatedString);
        }

        // consume the ending `"`
        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String(value.to_string()));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // fractional part
        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            // consume the `.`
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number: f32 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number(number));
    }

    /// Returns the next character without consuming it.
    fn peek_next(&self) -> u8 {
        match self.source.as_bytes().get(self.current + 1) {
            None => b'\0',
            Some(&c) => c,
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == b'_' {
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
            _ => TokenType::Identifier(text.to_string()),
        };

        self.add_token(type_);
    }
}
