use crate::token::{Literal, Token, TokenType};
use crate::expression::{Expr, LiteralValue};

struct Parser{
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser{
            tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while (self.matches_any(&[&TokenType::BangEqual, &TokenType::EqualEqual])){
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.matches_any(&[&TokenType::Greater, &TokenType::GreaterEqual, &TokenType::Less, &TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return expr
    }

    fn term(&mut self) -> Expr{
        let mut expr = self.factor();
        while self.matches_any(&[&TokenType::Minus, &TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return expr
    }

    fn factor(&mut self) -> Expr{
        let mut expr = self.unary();

        while self.matches_any(&[&TokenType::Slash, &TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));

        }

        return expr
    }

    fn unary(&mut self) -> Expr{
        if self.matches_any(&[&TokenType::Bang, &TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(operator, Box::new(right))
        }

        return self.primary()
    }

    fn primary(&mut self) -> Expr{
        if self.matches_any(&[&TokenType::False]) {
            return Expr::Literal(LiteralValue::False)
        } else if self.matches_any(&[&TokenType::True]) {
            return Expr::Literal(LiteralValue::True)
        } else if self.matches_any(&[&TokenType::Nil]) {
            return Expr::Literal(LiteralValue::Nil)
        }

        if self.matches_any(&[&TokenType::Literal]){
            let literal = self.previous();
            match literal{
                Literal
            }
        }
    }

    fn matches_any(&mut self, types: &[&TokenType]) -> bool{
        for t in types{
            if self.check(t){
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn advance(&mut self) -> Token{
        if !self.is_at_end(){
            self.current += 1;
        }

        return self.previous()
    }

    fn previous(&mut self) -> Token{
        return self.tokens[self.current - 1].clone()
    }

    fn check(&self, t: &TokenType) -> bool{
        if self.is_at_end(){
            return false;
        }

        return self.peek().token_type == *t;
    }

    fn is_at_end(&self) -> bool{
        return self.peek().token_type == TokenType::Eof;
    }

    fn peek(&self) -> Token{
        return self.tokens[self.current].clone();
    }
}
