use std::fmt;
use std::ops::{Neg, Not, Add, Sub, Div, Mul};
use std::cmp::Ordering;
use crate::{expression, Lox};
use crate::expression::{Visitor, Expr, LiteralValue};
use crate::token::TokenType;

#[derive(PartialEq)]
enum LoxValue{
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl PartialOrd for LoxValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => l.partial_cmp(r),
            (Self::String(l), Self::String(r)) => l.partial_cmp(r),
            (Self::Boolean(l), Self::Boolean(r)) => l.partial_cmp(r),
            (Self::Nil, Self::Nil) => Some(Ordering::Equal),
            _ => None,
        }
    }
}

impl LoxValue{
    fn is_numerical(&self) -> bool{
        match self{
            LoxValue::Number(_) => true,
            _ => false
        }
    }

    fn is_string(&self) -> bool{
        match self{
            LoxValue::String(_) => true,
            _ => false
        }
    }
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            LoxValue::Number(n) => write!(f, "{}", n),
            LoxValue::String(s) => write!(f, "{}", s),
            LoxValue::Boolean(b) => write!(f, "{}", b),
            LoxValue::Nil => write!(f, "nil"),
        }
    }
}

pub struct Interpreter;

impl Interpreter{
    fn evaluate(&mut self, expr: &Expr) -> LoxValue{
        expr.accept(self)
    }
}

impl Not for LoxValue{
    type Output = Self;
    fn not(self) -> Self::Output{
        match self {
            LoxValue::Boolean(b) => LoxValue::Boolean(!b),
            LoxValue::Nil => LoxValue::Boolean(true),
            _ => LoxValue::Boolean(false),
        }
    }
}

impl Neg for LoxValue {
    type Output = Self;
    fn neg(self) -> Self::Output{
        match self {
            LoxValue::Number(n) => LoxValue::Number(-n),
            LoxValue::String(s) => todo!(),
            LoxValue::Boolean(true) => LoxValue::Boolean(false),
            LoxValue::Boolean(false) => LoxValue::Boolean(false),
            LoxValue::Nil => todo!(),
        }
    }
}

impl Sub for LoxValue{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output{
        match (self, other) {
            (LoxValue::Number(n), LoxValue::Number(n2)) => LoxValue::Number(n-n2),
            _ => LoxValue::Nil,
        }
    }
}

impl Add for LoxValue{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output{
        match (self, other) {
            (LoxValue::Number(n), LoxValue::Number(n2)) => LoxValue::Number(n+n2),
            _ => LoxValue::Nil,
        }
    }
}

impl Div for LoxValue{
    type Output = Self;
    fn div(self, other: Self) -> Self::Output{
        match (self, other) {
            (LoxValue::Number(n), LoxValue::Number(n2)) => {
                if n2 == 0.0 {
                    LoxValue::Nil
                } else {
                    LoxValue::Number(n / n2)
                }
            },
            _ => LoxValue::Nil,
        }
    }
}

impl Mul for LoxValue{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output{
        match (self, other) {
            (LoxValue::Number(n), LoxValue::Number(n2)) => LoxValue::Number(n+n2),
            _ => LoxValue::Nil,
        }
    }
}

impl Visitor<LoxValue> for Interpreter{
    fn visit_literal_expr(&mut self, expr: &Expr) -> LoxValue {
        let Expr::Literal(value) = expr else { unreachable!() };

        match value {
            LiteralValue::Number(num) => LoxValue::Number(*num),
            LiteralValue::String(s) => LoxValue::String(s.clone()),
            LiteralValue::True => LoxValue::Boolean(true),
            LiteralValue::False => LoxValue::Boolean(false),
            LiteralValue::Nil => LoxValue::Nil,
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> LoxValue {
        let Expr::Grouping(expression) = expr else { unreachable!() };
        self.evaluate(expression)
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> LoxValue {
        let Expr::Unary(operator, expression) = expr else { unreachable!() };

        let right = self.evaluate(expression);
        match operator.token_type {
            TokenType::Minus => {
                return -right;
            }

            TokenType::Bang => {
                return !right;
            }
            _ => {}
        }

        unreachable!()
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> LoxValue {
        let Expr::Binary(left, operator, right) = expr else { unreachable!() };

        let left = self.evaluate(left);
        let right = self.evaluate(right);
        match operator.token_type {
            TokenType::Minus => {
                return left - right;
            }
            TokenType::Slash => {
                return left / right;
            }
            TokenType::Star => {
                return left * right;
            }
            TokenType::Plus => {
                if left.is_numerical() && right.is_numerical(){
                    return left + right;
                }

                if left.is_string() && right.is_string() {
                    return LoxValue::String(left.to_string() + right.to_string().as_str())
                }
            }
            TokenType::Greater => {
                return LoxValue::Boolean(left > right)
            }

            TokenType::GreaterEqual => {
                return LoxValue::Boolean(left >= right)
            }

            TokenType::Less => {
                return LoxValue::Boolean(left < right)
            }

            TokenType::LessEqual => {
                return LoxValue::Boolean(left <= right)
            }

            TokenType::BangEqual => {
                return LoxValue::Boolean(left != right)
            }

            TokenType::EqualEqual => {
                return LoxValue::Boolean(left == right)
            }
            _ => {}
        }

        todo!()
    }
}