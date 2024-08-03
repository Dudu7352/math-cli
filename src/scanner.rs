use crate::{
    error::TokenScanError,
    tokens::{Operator, Token},
};
use std::collections::LinkedList;

enum TokenScanResult {
    Token(Token),
    Whitespace,
    EmptySource,
    Error(TokenScanError),
}

pub struct TokenScanner {
    source: Vec<char>,
    head: usize,
    tail: usize,
    tokens: LinkedList<Token>,
}

impl<'a> TokenScanner {
    pub fn new(source: Vec<char>) -> Self {
        Self {
            source,
            head: 0,
            tail: 0,
            tokens: LinkedList::new(),
        }
    }

    pub fn scan_tokens(mut self) -> Result<LinkedList<Token>, TokenScanError> {
        loop {
            match self.scan_token() {
                TokenScanResult::Token(token) => self.tokens.push_back(token),
                TokenScanResult::Whitespace => continue,
                TokenScanResult::EmptySource => break,
                TokenScanResult::Error(error) => return Err(error),
            }
        }
        return Ok(self.tokens);
    }

    fn scan_token(&mut self) -> TokenScanResult {
        match self.advance() {
            Some(chr) => match chr {
                '+' => self.consume_and_return_token(Token::Op(Operator::Plus)),
                '-' => self.handle_minus(),
                '/' => self.consume_and_return_token(Token::Op(Operator::Slash)),
                '*' => self.consume_and_return_token(Token::Op(Operator::Star)),
                '(' => self.consume_and_return_token(Token::Op(Operator::LParen)),
                ')' => self.consume_and_return_token(Token::Op(Operator::RParen)),
                '0'..='9' => match self.try_consume_number() {
                    Ok(token) => TokenScanResult::Token(token),
                    Err(err) => TokenScanResult::Error(err),
                },
                _ => {
                    if chr.is_whitespace() {
                        self.consume();
                        TokenScanResult::Whitespace
                    } else {
                        TokenScanResult::Error(TokenScanError::InvalidCharacter)
                    }
                }
            },
            None => TokenScanResult::EmptySource,
        }
    }

    fn handle_minus(&'a mut self) -> TokenScanResult {
        if self.previous_token_not_number() {
            self.head += 1;
            return match self.try_consume_number() {
                Ok(token) => TokenScanResult::Token(token),
                Err(error) => TokenScanResult::Error(error),
            };
        }
        self.consume_and_return_token(Token::Op(Operator::Minus))
    }

    fn previous_token_not_number(&self) -> bool {
        self.peek_last_token().map_or(true, |token| {
            if let Token::Number(_) = token {
                false
            } else {
                true
            }
        })
    }

    fn peek_last_token(&'a self) -> Option<&'a Token> {
        self.tokens.cursor_back().current()
    }

    fn try_consume_number(&mut self) -> Result<Token, TokenScanError> {
        self.advance_until_not_numeric();
        if *self.peek().unwrap_or(&' ') == '.' {
            if !self.peek_next().unwrap_or(&' ').is_numeric() {
                return Err(TokenScanError::InvalidNumberLiteral);
            }
            self.advance();
            self.advance_until_not_numeric();
        }
        let num_literal = self.consume();
        return num_literal
            .parse::<f32>()
            .map(|number| Token::Number(number))
            .map_err(|err| {
                println!("{:?}", err);
                TokenScanError::InvalidNumberLiteral
            });
    }

    fn advance_until_not_numeric(&mut self) {
        loop {
            match self.peek() {
                Some(chr) => {
                    if !chr.is_numeric() {
                        break;
                    }
                    self.head += 1;
                }
                None => break,
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.head == self.source.len() {
            return None;
        }
        self.head += 1;
        return Some(self.source[self.head - 1]);
    }

    fn peek(&self) -> Option<&char> {
        self.source.get(self.head)
    }

    fn peek_next(&self) -> Option<&char> {
        self.source.get(self.head + 1)
    }

    fn consume(&mut self) -> String {
        let result = self.source[self.tail..self.head].into_iter().collect();
        self.tail = self.head;
        result
    }

    fn consume_and_return_token(&mut self, token: Token) -> TokenScanResult {
        self.consume();
        TokenScanResult::Token(token)
    }
}
