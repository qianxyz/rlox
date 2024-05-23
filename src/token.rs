#[derive(Debug, Clone)]
#[rustfmt::skip]
pub enum TokenType {
    // single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // one or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // literals
    Identifier(String),
    String(String),
    Number(f32),

    // keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            type_,
            lexeme,
            line,
        }
    }
}
