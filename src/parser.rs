use std::collections::LinkedList;

use crate::{
    error::ParseError,
    tokens::{Operator, Token},
};

#[derive(PartialEq)]
enum ParseResult {
    Ok,
    Error(ParseError),
    NoTokens,
}

pub struct ExpressionParser {
    tokens: LinkedList<Token>,
    operators: LinkedList<Operator>,
    numbers: LinkedList<f32>,
}

impl ExpressionParser {
    pub fn new(tokens: LinkedList<Token>) -> Self {
        Self {
            tokens,
            operators: LinkedList::new(),
            numbers: LinkedList::new(),
        }
    }

    pub fn parse(&mut self) -> Result<f32, ParseError> {
        loop {
            match self.parse_next_token() {
                ParseResult::Ok => continue,
                ParseResult::Error(error) => return Err(error),
                ParseResult::NoTokens => break,
            }
        }
        if let Some(error) = self.empty_operation_stack() {
            return Err(error);
        }
        let result = self.numbers.pop_back().unwrap();
        if !self.numbers.is_empty() {
            return Err(ParseError::StackNotEmpty);
        }
        return Ok(result);
    }

    fn empty_operation_stack(&mut self) -> Option<ParseError> {
        while !self.operators.is_empty() {
            if let ParseResult::Error(err) = self.perform_operation() {
                return Some(err);
            }
        }
        None
    }

    fn parse_next_token(&mut self) -> ParseResult {
        let opt_token = self.tokens.pop_front();
        if opt_token.is_none() {
            return ParseResult::NoTokens;
        }
        match opt_token.unwrap() {
            Token::Op(operator) => self.handle_operator(operator),
            Token::Number(number) => {
                self.numbers.push_back(number);
                ParseResult::Ok
            }
        }
    }

    fn handle_operator(&mut self, operator: Operator) -> ParseResult {
        if operator == Operator::LParen {
            self.operators.push_back(operator);
            return ParseResult::Ok;
        }
        if operator == Operator::RParen {
            return self.perform_all_until_brace();
        }
        match self.peek_last_operator() {
            Some(peeked_operator) => {
                if peeked_operator < &operator {
                    self.operators.push_back(operator);
                    ParseResult::Ok
                } else {
                    self.perform_and_push_new_if_ok(operator)
                }
            }
            None => {
                self.operators.push_back(operator);
                ParseResult::Ok
            }
        }
    }

    fn perform_all_until_brace(&mut self) -> ParseResult {
        loop {
            match self.peek_last_operator() {
                Some(operator) => {
                    if Operator::LParen == *operator {
                        self.operators.pop_back();
                        return ParseResult::Ok;
                    }
                    self.perform_operation();
                }
                None => return ParseResult::Error(ParseError::NoOperator),
            }
        }
    }

    fn perform_and_push_new_if_ok(&mut self, new_operator: Operator) -> ParseResult {
        let result = self.perform_operation();
        if ParseResult::Ok != result {
            return result;
        }
        self.operators.push_back(new_operator);
        ParseResult::Ok
    }

    fn perform_operation(&mut self) -> ParseResult {
        let operator = self.operators.pop_back().unwrap();
        let second = self.numbers.pop_back();
        let first = self.numbers.pop_back();
        if first.is_none() || second.is_none() {
            return ParseResult::Error(ParseError::NoNumber);
        }
        let result = match operator {
            Operator::Plus => first.unwrap() + second.unwrap(),
            Operator::Minus => first.unwrap() - second.unwrap(),
            Operator::Star => first.unwrap() * second.unwrap(),
            Operator::Slash => first.unwrap() / second.unwrap(),
            _ => return ParseResult::Error(ParseError::IncorrectOperator),
        };
        self.numbers.push_back(result);
        ParseResult::Ok
    }

    fn peek_last_operator(&self) -> Option<&Operator> {
        self.operators.cursor_back().current()
    }
}
