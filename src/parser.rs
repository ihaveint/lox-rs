use crate::Lox;
use crate::expression::{Expr, LiteralValue};
use crate::token::{Literal, Token, TokenType};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    lox: &'a mut Lox,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, lox: &'a mut Lox) -> Parser<'a> {
        Parser {
            tokens,
            current: 0,
            lox,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn synchronize(&mut self) {
        self.advance();

        while (!self.is_at_end()) {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            if vec![
                TokenType::Class,
                TokenType::Fun,
                TokenType::Var,
                TokenType::For,
                TokenType::If,
                TokenType::While,
                TokenType::Print,
                TokenType::Return,
            ]
            .contains(&self.peek().token_type)
            {
                return;
            }
            self.advance();
        }
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.comparison()?;

        while (self.matches_any(&[&TokenType::BangEqual, &TokenType::EqualEqual])) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.term()?;

        while self.matches_any(&[
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::Less,
            &TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.matches_any(&[&TokenType::Minus, &TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.matches_any(&[&TokenType::Slash, &TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.matches_any(&[&TokenType::Bang, &TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.matches_any(&[&TokenType::False]) {
            return Ok(Expr::Literal(LiteralValue::False));
        } else if self.matches_any(&[&TokenType::True]) {
            return Ok(Expr::Literal(LiteralValue::True));
        } else if self.matches_any(&[&TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralValue::Nil));
        }

        if self.matches_any(&[&TokenType::Literal]) {
            let literal = self.previous();
            match literal.literal.unwrap() {
                Literal::String(s) => return Ok(Expr::Literal(LiteralValue::String(s))),
                Literal::Number(n) => return Ok(Expr::Literal(LiteralValue::Number(n))),
                _ => {}
            }
        }

        if self.matches_any(&[&TokenType::LeftParen]) {
            let expression = self.expression()?;
            self.consume(&TokenType::RightParen, "Expected ')' after expression.");
            return Ok(Expr::Grouping(Box::new(expression)));
        }

        Err(self.error(&self.peek(), "Expected expression."))
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Token {
        if self.check(token_type) {
            return self.advance();
        }

        panic!("{:?}", self.error(&self.peek(), message))
    }

    fn error(&mut self, token: &Token, message: &str) -> String {
        self.lox.error_parser(token, message);
        return "ParseError".into();
    }

    fn matches_any(&mut self, types: &[&TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn previous(&mut self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == *t;
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::Eof;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }
}
