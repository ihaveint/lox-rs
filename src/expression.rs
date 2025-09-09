use crate::token::{self, Literal, Token};

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

pub enum Expr{
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(LiteralValue),
    Unary(Token, Box<Expr>),
}

trait Visitor<T> {
    fn visit_binary_expr(&mut self, expr: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
    fn visit_literal_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, expr: &Expr) -> T;
}

impl Expr{
    fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T{
        match self {
            Expr::Binary(..) => visitor.visit_binary_expr(self),
            Expr::Grouping(..) => visitor.visit_grouping_expr(self),
            Expr::Literal(..) => visitor.visit_literal_expr(self),
            Expr::Unary(..) => visitor.visit_unary_expr(self),
        }
    }

}

pub struct AstPrinter;

impl AstPrinter{
    pub fn print(&mut self, expr: &Expr) -> String{
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String{
        let mut builder = String::new();
        builder.push_str("(");
        builder.push_str(name);
        for expr in exprs{
            builder.push_str(" ");
            builder.push_str(&expr.accept(self));
        }

        builder.push_str(")");
        builder
    }
}

impl Visitor<String> for AstPrinter{
    fn visit_binary_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Binary(left, operator, right) = expr{
            self.parenthesize(&operator.lexeme, &[&left, &right])
        } else {
            todo!("not implemented")
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Grouping(expression) = expr{
            self.parenthesize("group", &[&expression])
        } else {
            todo!("not implemented")
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Literal(value) = expr {
            match value {
                LiteralValue::Number(num) => num.to_string(),
                LiteralValue::String(s) => format!("\"{}\"", s),
                LiteralValue::True => String::from("true"),
                LiteralValue::False => String::from("false"),
                LiteralValue::Nil => String::from("nil"),
            }
        } else {
            todo!("not implemented")
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Unary(operator, right) = expr {
            self.parenthesize(&operator.lexeme, &[&right])
        } else {
            todo!("not implemented")
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::token::TokenType;
    use super::*;

    #[test]
    fn test_ast_printer_literal(){
        let expression = Expr::Literal(LiteralValue::Number(123.0));
        let mut printer = AstPrinter;
        assert_eq!(printer.print(&expression), "123");
    }

    #[test]
    fn test_ast_printer_binary_expression() {
        let expression = Expr::Binary(
            Box::new(Expr::Literal(LiteralValue::Number(1.0))),
            Token::new(TokenType::Plus, "+".into(), None, 1),
            Box::new(Expr::Literal(LiteralValue::Number(2.0))),
        );

        let mut printer = AstPrinter;
        assert_eq!(printer.print(&expression), "(+ 1 2)");
    }
}