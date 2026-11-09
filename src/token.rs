#[derive(Debug)]
pub enum TokenType {
    // Single-char tokens
    LParen,
    RParen,
    LCurly,
    RCurly,
    LSquar,
    RSquar,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Mod,
    Colon,
    Semicolon,
    Question,
    Not,
    Gt,
    Lt,
    Assign,
    // Two-char tokens
    Eq,
    NotEq,
    GtE,
    LtE,
    // Literals
    Number,
    String,
    Identifier,
    // Keywords
    If,
    Then,
    Else,
    True,
    False,
    And,
    Or,
    While,
    Do,
    For,
    Func,
    Null,
    End,
    Print,
    PrintLn,
    Ret,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        return Token {
            token_type,
            lexeme,
            line,
        };
    }
}
