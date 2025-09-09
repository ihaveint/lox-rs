mod token;
mod scanner;

use std::{env, fs, io};
use std::fs::exists;
use std::io::{stdout, Write};
use std::process::exit;
use scanner::Scanner;

struct Lox{
    had_error: bool,
}

impl Lox{
    fn new() -> Lox{
        Lox{
            had_error: false,
        }
    }

    fn run(&mut self, line: String){
        print!("running line: {}", line);
        let mut scanner = Scanner::new(line, self);
        let tokens = scanner.scan_tokens();
        // self.error(1, "test error"); // Example of setting had_error

    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);

    }

    fn report(&mut self, line: usize, where_in_code: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, where_in_code, message);
        self.had_error = true;
    }

    fn run_file(&mut self, file_path: String) {
        println!("Running file: {}", file_path);
        let content = fs::read_to_string(file_path).expect("Could not read file");
        self.run(content);
        if self.had_error{
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