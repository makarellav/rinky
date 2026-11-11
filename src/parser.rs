use std::fmt::format;

use crate::token::{Token, TokenType};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];

        self.current += 1;

        return token;
    }

    fn peek(&self) -> Option<&Token> {
        if self.current >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.current])
        }
    }

    fn is_next(&self, expected_type: TokenType) -> bool {
        if self.current + 1 >= self.tokens.len() {
            false
        } else {
            self.tokens[self.current + 1].token_type == expected_type
        }
    }

    fn expect(&mut self, expected_type: TokenType) -> Result<&Token, String> {
        match self.peek() {
            Some(t) if t.token_type == expected_type => {}
            Some(t) => {
                return Err(format!(
                    "line {}: expected {:?}, but found {:?}",
                    t.line, expected_type, t.token_type
                ));
            }
            None => {
                return Err(format!(
                    "line {}: expected {:?}, but got end of an input",
                    self.tokens.last().map_or(1, |t| t.line),
                    expected_type
                ));
            }
        }

        self.current += 1;

        Ok(self.previous())
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }

    fn is_match(&mut self, expected_type: TokenType) -> bool {
        if self.peek().is_some_and(|t| t.token_type == expected_type) {
            self.current += 1;

            true
        } else {
            false
        }
    }

    pub fn parse(&self) {}
}
