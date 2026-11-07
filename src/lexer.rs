use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    source: String,
    line: i32,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            tokens: Vec::new(),
            source,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> &[Token] {
        let mut it = self.source.chars().peekable();

        while let Some(ch) = it.next() {
            match ch {
                '{' => self
                    .tokens
                    .push(Token::new(TokenType::LCurly, ch.to_string(), self.line)),
                '}' => self
                    .tokens
                    .push(Token::new(TokenType::RCurly, ch.to_string(), self.line)),
                '(' => self
                    .tokens
                    .push(Token::new(TokenType::LParen, ch.to_string(), self.line)),
                ')' => self
                    .tokens
                    .push(Token::new(TokenType::RParen, ch.to_string(), self.line)),
                '[' => self
                    .tokens
                    .push(Token::new(TokenType::LSquar, ch.to_string(), self.line)),
                ']' => self
                    .tokens
                    .push(Token::new(TokenType::RSquar, ch.to_string(), self.line)),
                ',' => self
                    .tokens
                    .push(Token::new(TokenType::Comma, ch.to_string(), self.line)),
                '.' => self
                    .tokens
                    .push(Token::new(TokenType::Dot, ch.to_string(), self.line)),
                '+' => self
                    .tokens
                    .push(Token::new(TokenType::Plus, ch.to_string(), self.line)),
                '-' => {
                    if it.peek() == Some(&'-') {
                        it.next();

                        while let Some(&next_ch) = it.peek() {
                            if next_ch == '\n' {
                                break;
                            }

                            it.next();
                        }
                    } else {
                        self.tokens
                            .push(Token::new(TokenType::Minus, ch.to_string(), self.line))
                    }
                }
                '*' => self
                    .tokens
                    .push(Token::new(TokenType::Star, ch.to_string(), self.line)),
                '/' => self
                    .tokens
                    .push(Token::new(TokenType::Slash, ch.to_string(), self.line)),
                '^' => self
                    .tokens
                    .push(Token::new(TokenType::Caret, ch.to_string(), self.line)),
                '%' => self
                    .tokens
                    .push(Token::new(TokenType::Mod, ch.to_string(), self.line)),
                ':' => self
                    .tokens
                    .push(Token::new(TokenType::Colon, ch.to_string(), self.line)),
                ';' => {
                    self.tokens
                        .push(Token::new(TokenType::Semicolon, ch.to_string(), self.line))
                }
                '?' => self
                    .tokens
                    .push(Token::new(TokenType::Question, ch.to_string(), self.line)),
                '!' => {
                    if it.peek() == Some(&'=') {
                        it.next();

                        self.tokens
                            .push(Token::new(TokenType::NotEq, "!=".to_string(), self.line));
                    } else {
                        self.tokens
                            .push(Token::new(TokenType::Not, ch.to_string(), self.line));
                    }
                }
                '>' => {
                    if it.peek() == Some(&'=') {
                        it.next();

                        self.tokens
                            .push(Token::new(TokenType::GtE, ">=".to_string(), self.line));
                    } else {
                        self.tokens
                            .push(Token::new(TokenType::Gt, ch.to_string(), self.line));
                    }
                }
                '<' => {
                    if it.peek() == Some(&'=') {
                        it.next();

                        self.tokens
                            .push(Token::new(TokenType::LtE, "<=".to_string(), self.line));
                    } else {
                        self.tokens
                            .push(Token::new(TokenType::Lt, ch.to_string(), self.line));
                    }
                }
                '=' => {
                    if it.peek() == Some(&'=') {
                        it.next();

                        self.tokens
                            .push(Token::new(TokenType::Eq, "==".to_string(), self.line));
                    } else {
                        self.tokens
                            .push(Token::new(TokenType::Assign, ch.to_string(), self.line));
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

                        self.tokens
                            .push(Token::new(TokenType::Number, number_str, self.line));
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

                        self.tokens
                            .push(Token::new(TokenType::String, str_value, self.line));
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

                        self.tokens.push(Token::new(ident_type, word, self.line));
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
