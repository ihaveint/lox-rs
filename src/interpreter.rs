use crate::expression::{Expr, LiteralValue, Visitor};
use crate::runtime_error;
use crate::runtime_error::RuntimeError;
use crate::token::{Token, TokenType};
use crate::{Lox, expression};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(PartialEq, Clone)]
enum LoxValue {
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

impl LoxValue {
    fn is_numerical(&self) -> bool {
        match self {
            LoxValue::Number(_) => true,
            _ => false,
        }
    }

    fn is_string(&self) -> bool {
        match self {
            LoxValue::String(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxValue::Number(n) => write!(f, "{}", n),
            LoxValue::String(s) => write!(f, "{}", s),
            LoxValue::Boolean(b) => write!(f, "{}", b),
            LoxValue::Nil => write!(f, "nil"),
        }
    }
}

pub struct Interpreter<'a> {
    lox: &'a mut Lox,
}

impl<'a> Interpreter<'a> {
    pub fn new(lox: &'a mut Lox) -> Interpreter<'a> {
        Interpreter { lox }
    }
}

impl<'a> Interpreter<'a> {
    fn stringify(&mut self, value: &LoxValue) -> String {
        match value {
            LoxValue::Number(n) => format!("{}", n),
            LoxValue::String(s) => format!("{}", s),
            LoxValue::Boolean(b) => format!("{}", b),
            LoxValue::Nil => String::from("nil"),
        }
    }

    pub fn interpret(&mut self, expr: &Expr) {
        match self.evaluate(expr) {
            Ok(lox_value) => {
                println!("{}", self.stringify(&lox_value));
            }
            Err(err) => self.lox.error_runtime(err),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        expr.accept(self)
    }

    fn check_number_operand(
        &mut self,
        operator: &Token,
        operand: &LoxValue,
    ) -> Result<(), RuntimeError> {
        if let LoxValue::Number(n) = operand {
            return Ok(());
        }

        Err(RuntimeError::new(
            operator.clone(),
            "Operand must be number.",
        ))
    }

    fn check_number_operands(
        &mut self,
        operator: &Token,
        left: &LoxValue,
        right: &LoxValue,
    ) -> Result<(), RuntimeError> {
        if let LoxValue::Number(n) = left {
            if let LoxValue::Number(n) = right {
                return Ok(());
            }
        }

        Err(RuntimeError::new(
            operator.clone(),
            "Operands must be numbers.",
        ))
    }
}

impl Not for LoxValue {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            LoxValue::Boolean(b) => LoxValue::Boolean(!b),
            LoxValue::Nil => LoxValue::Boolean(true),
            _ => LoxValue::Boolean(false),
        }
    }
}

impl Neg for LoxValue {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            LoxValue::Number(n) => LoxValue::Number(-n),
            LoxValue::String(s) => todo!(),
            LoxValue::Boolean(true) => LoxValue::Boolean(false),
            LoxValue::Boolean(false) => LoxValue::Boolean(false),
            LoxValue::Nil => todo!(),
        }
    }
}

impl Sub for LoxValue {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (LoxValue::Number(n), LoxValue::Number(n2)) => LoxValue::Number(n - n2),
            _ => LoxValue::Nil,
        }
    }
}

impl Add for LoxValue {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (LoxValue::Number(n), LoxValue::Number(n2)) => LoxValue::Number(n + n2),
            _ => LoxValue::Nil,
        }
    }
}

impl Div for LoxValue {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (LoxValue::Number(n), LoxValue::Number(n2)) => {
                if n2 == 0.0 {
                    LoxValue::Nil
                } else {
                    LoxValue::Number(n / n2)
                }
            }
            _ => LoxValue::Nil,
        }
    }
}

impl Mul for LoxValue {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (LoxValue::Number(n), LoxValue::Number(n2)) => LoxValue::Number(n + n2),
            _ => LoxValue::Nil,
        }
    }
}

impl<'a> Visitor<Result<LoxValue, RuntimeError>> for Interpreter<'a> {
    fn visit_literal_expr(&mut self, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        let Expr::Literal(value) = expr else {
            unreachable!()
        };

        match value {
            LiteralValue::Number(num) => Ok(LoxValue::Number(*num)),
            LiteralValue::String(s) => Ok(LoxValue::String(s.clone())),
            LiteralValue::True => Ok(LoxValue::Boolean(true)),
            LiteralValue::False => Ok(LoxValue::Boolean(false)),
            LiteralValue::Nil => Ok(LoxValue::Nil),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        let Expr::Grouping(expression) = expr else {
            unreachable!()
        };
        self.evaluate(expression)
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        let Expr::Unary(operator, expression) = expr else {
            unreachable!()
        };

        let right = self.evaluate(expression)?;
        match operator.token_type {
            TokenType::Minus => {
                return Ok(-right);
            }

            TokenType::Bang => {
                return Ok(!right);
            }
            _ => {}
        }

        unreachable!()
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        let Expr::Binary(left, operator, right) = expr else {
            unreachable!()
        };

        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;
        match operator.token_type {
            TokenType::Minus => {
                self.check_number_operand(operator, &right)?;

                return Ok(left - right);
            }
            TokenType::Slash => {
                self.check_number_operands(operator, &left, &right)?;

                return Ok(left / right);
            }
            TokenType::Star => {
                self.check_number_operands(operator, &left, &right)?;

                return Ok(left * right);
            }
            TokenType::Plus => {
                if left.is_numerical() && right.is_numerical() {
                    return Ok(left + right);
                }

                if left.is_string() && right.is_string() {
                    return Ok(LoxValue::String(
                        left.to_string() + right.to_string().as_str(),
                    ));
                }

                return Err(RuntimeError::new(
                    operator.clone(),
                    "Operands must be two numbers or two strings.",
                ));
            }
            TokenType::Greater => {
                self.check_number_operands(operator, &left, &right)?;
                return Ok(LoxValue::Boolean(left > right));
            }
            TokenType::GreaterEqual => {
                self.check_number_operands(operator, &left, &right)?;

                return Ok(LoxValue::Boolean(left >= right));
            }

            TokenType::Less => {
                self.check_number_operands(operator, &left, &right)?;

                return Ok(LoxValue::Boolean(left < right));
            }

            TokenType::LessEqual => {
                self.check_number_operands(operator, &left, &right)?;

                return Ok(LoxValue::Boolean(left <= right));
            }

            TokenType::BangEqual => return Ok(LoxValue::Boolean(left != right)),

            TokenType::EqualEqual => return Ok(LoxValue::Boolean(left == right)),
            _ => {}
        }

        todo!()
    }
}
