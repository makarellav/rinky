use std::fmt::Error;

use crate::token::{self, Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    source: Vec<char>,
    line: usize,
    current: usize,
    start: usize,
}

impl Lexer {
    pub fn new(source: Vec<char>) -> Self {
        Lexer {
            tokens: Vec::new(),
            source,
            line: 1,
            current: 0,
            start: 0,
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];

        self.current += 1;

        return ch;
    }

    fn add_token(&mut self, token_type: TokenType) {
        let ch: String = self.source[self.start..self.current].iter().collect();

        self.tokens.push(Token::new(token_type, ch, self.line));
    }

    fn add_token_range(&mut self, token_type: TokenType, start: usize, end: usize) {
        let word: String = self.source[start..end].iter().collect();

        self.tokens.push(Token::new(token_type, word, self.line));
    }

    fn peek(&self) -> Option<char> {
        if self.current >= self.source.len() {
            return None;
        }

        Some(self.source[self.current])
    }

    fn look_ahead(&self, n: usize) -> Option<char> {
        if self.current + n >= self.source.len() {
            return None;
        }

        Some(self.source[self.current + n])
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.peek() != Some(expected) {
            return false;
        }

        self.current += 1;

        true
    }

    fn handle_comment(&mut self) {
        while matches!(self.peek(), Some(c) if c != '\n') {
            self.advance();
        }
    }

    fn handle_number(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.advance();
        }

        if self.peek() == Some('.') && self.look_ahead(1).is_some_and(|c| c.is_ascii_digit()) {
            self.advance();

            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.advance();
            }

            self.add_token(TokenType::Float);
        } else {
            self.add_token(TokenType::Integer);
        }
    }

    fn handle_string(&mut self, closing: char) -> Result<(), String> {
        let start_line = self.line;

        while matches!(self.peek(), Some(c) if c != closing) {
            if self.peek() == Some('\n') {
                self.line += 1;
            }

            self.advance();
        }

        if self.peek().is_none() {
            return Err(format!("line {start_line}: unterminated string"));
        }

        self.advance();

        self.add_token_range(TokenType::String, self.start + 1, self.current - 1);

        Ok(())
    }

    fn handle_identifier(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c == '_') {
            self.advance();
        }

        let word: String = self.source[self.start..self.current].iter().collect();

        let ident_type = self.lookup_ident_type(&word);

        self.add_token(ident_type);
    }

    pub fn tokenize(&mut self) -> Result<&[Token], String> {
        while self.current < self.source.len() {
            self.start = self.current;

            let ch = self.advance();

            match ch {
                '{' => self.add_token(TokenType::LCurly),
                '}' => self.add_token(TokenType::RCurly),
                '(' => self.add_token(TokenType::LParen),
                ')' => self.add_token(TokenType::RParen),
                '[' => self.add_token(TokenType::LSquar),
                ']' => self.add_token(TokenType::RSquar),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '+' => self.add_token(TokenType::Plus),
                '-' => {
                    if self.is_match('-') {
                        self.handle_comment();
                    } else {
                        self.add_token(TokenType::Minus)
                    }
                }
                '*' => self.add_token(TokenType::Star),
                '/' => self.add_token(TokenType::Slash),
                '^' => self.add_token(TokenType::Caret),
                '%' => self.add_token(TokenType::Mod),
                ':' => self.add_token(TokenType::Colon),
                ';' => self.add_token(TokenType::Semicolon),
                '?' => self.add_token(TokenType::Question),
                '!' => {
                    if self.is_match('=') {
                        self.add_token(TokenType::NotEq);
                    } else {
                        self.add_token(TokenType::Not);
                    }
                }
                '>' => {
                    if self.is_match('=') {
                        self.add_token(TokenType::GtE);
                    } else {
                        self.add_token(TokenType::Gt);
                    }
                }
                '<' => {
                    if self.is_match('=') {
                        self.add_token(TokenType::LtE);
                    } else {
                        self.add_token(TokenType::Lt);
                    }
                }
                '=' => {
                    if self.is_match('=') {
                        self.add_token(TokenType::Eq);
                    } else {
                        self.add_token(TokenType::Assign);
                    }
                }
                '\n' => self.line += 1,
                ' ' | '\r' | '\t' => {}
                _ => {
                    if ch.is_ascii_digit() {
                        self.handle_number();
                    } else if ch == '"' || ch == '\'' {
                        self.handle_string(ch)?;
                    } else if ch.is_alphanumeric() || ch == '_' {
                        self.handle_identifier();
                    } else {
                        return Err(format!("line {}: unknown character {ch}", self.line));
                    }
                }
            }
        }

        Ok(&self.tokens)
    }

    fn lookup_ident_type(&self, word: &str) -> TokenType {
        match word {
            "if" => TokenType::If,
            "then" => TokenType::Then,
            "else" => TokenType::Else,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "or" => TokenType::Or,
            "and" => TokenType::And,
            "while" => TokenType::While,
            "do" => TokenType::Do,
            "for" => TokenType::For,
            "func" => TokenType::Func,
            "null" => TokenType::Null,
            "end" => TokenType::End,
            "print" => TokenType::Print,
            "println" => TokenType::PrintLn,
            "ret" => TokenType::Ret,
            _ => TokenType::Identifier,
        }
    }
}
