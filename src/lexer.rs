use crate::token::{Token, TokenType};

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

    pub fn tokenize(&mut self) -> &[Token] {
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
                        let mut number_str = String::from(ch);

                        while let Some(&next_ch) = it.peek() {
                            if next_ch.is_ascii_digit() {
                                number_str.push(it.next().unwrap());
                            } else {
                                break;
                            }
                        }

                        if it.peek() == Some(&'.') {
                            let mut it_clone = it.clone();

                            it_clone.next();

                            if let Some(&post_dot_ch) = it_clone.peek() {
                                if post_dot_ch.is_ascii_digit() {
                                    // push the dot
                                    number_str.push(it.next().unwrap());

                                    while let Some(&next_ch) = it.peek() {
                                        if next_ch.is_ascii_digit() {
                                            number_str.push(it.next().unwrap());
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        self.add_token(TokenType::Number);
                    } else if ch == '"' || ch == '\'' {
                        // TODO: handle unterminated strings
                        let mut str_value = String::new();

                        while let Some(&next_ch) = it.peek() {
                            if next_ch == ch {
                                it.next();

                                break;
                            }

                            if next_ch == '\n' {
                                self.line += 1;
                            }

                            str_value.push(it.next().unwrap());
                        }

                        self.add_token(TokenType::String);
                    } else if ch.is_alphanumeric() || ch == '_' {
                        let mut word = String::from(ch);

                        while let Some(&next_ch) = it.peek() {
                            if next_ch.is_alphanumeric() || next_ch == '_' {
                                word.push(it.next().unwrap());
                            } else {
                                break;
                            }
                        }

                        let ident_type = self.lookup_ident_type(&word);

                        self.add_token(ident_type);
                    }
                }
            }
        }

        &self.tokens
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
