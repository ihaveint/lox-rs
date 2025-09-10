mod expression;
mod interpreter;
mod parser;
mod scanner;
mod token;

use crate::expression::AstPrinter;
use crate::token::{Token, TokenType};
use parser::Parser;
use scanner::Scanner;
use std::fs::exists;
use std::io::{Write, stdout};
use std::process::exit;
use std::{env, fs, io};

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Lox {
        Lox { had_error: false }
    }

    fn run(&mut self, line: String) {
        print!("running line: {}", line);
        let mut scanner = Scanner::new(line, self);
        let tokens = scanner.scan_tokens();
        println!("tokens are: {:?}", tokens);

        let mut parser = Parser::new(tokens, self);
        let expression = parser.parse();
        if self.had_error {
            return;
        }

        println!("{}", AstPrinter.print(&expression.unwrap()));
    }

    fn error_lexer(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn error_parser(&mut self, token: &Token, message: &str) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, " at end", message);
        } else {
            let where_in_cord = format!("at '{}'", token.lexeme);
            self.report(token.line, &where_in_cord.as_str(), message);
        }
    }

    fn report(&mut self, line: usize, where_in_code: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, where_in_code, message);
        self.had_error = true;
    }

    fn run_file(&mut self, file_path: String) {
        println!("Running file: {}", file_path);
        let content = fs::read_to_string(file_path).expect("Could not read file");
        self.run(content);
        if self.had_error {
            exit(65)
        }
    }

    fn run_prompt(&mut self) {
        println!("Welcome to lox!");
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let stdin = io::stdin();
            let mut buffer = String::new();
            match stdin.read_line(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    self.run(buffer);
                    self.had_error = false;
                }
                Err(error) => {
                    println!("error: {}", error);
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut args = env::args();
    let mut lox = Lox::new();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
    } else if args.len() == 2 {
        lox.run_file(args.nth(1).unwrap());
    } else {
        lox.run_prompt();
    }
}
