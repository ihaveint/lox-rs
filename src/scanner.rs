use std::collections::HashMap;
use crate::Lox;
use crate::token::{Literal, Token, TokenType};
use lazy_static::lazy_static;

lazy_static!{
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
    };
}

pub struct Scanner<'a> {
    lox: &'a mut Lox,
    pub source: String,
    pub tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
}


impl<'a> Scanner<'a> {
    pub fn new(source: String, lox: &'a mut Lox) -> Scanner<'a> {
        Scanner{
            lox,
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end(){
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        return self.tokens.clone()
    }


    fn scan_token(&mut self){
        let c: char = self.advance();

        match c{
            '(' => self.add_token_without_literal(TokenType::LeftParen),
            ')' => self.add_token_without_literal(TokenType::RightParen),
            '{' => self.add_token_without_literal(TokenType::LeftBrace),
            '}' => self.add_token_without_literal(TokenType::RightBrace),
            ',' => self.add_token_without_literal(TokenType::Comma),
            '.' => self.add_token_without_literal(TokenType::Dot),
            '-' => self.add_token_without_literal(TokenType::Minus),
            '+' => self.add_token_without_literal(TokenType::Plus),
            ';' => self.add_token_without_literal(TokenType::Semicolon),
            '*' => self.add_token_without_literal(TokenType::Star),
            '!' => {
                if self.matches('=') {
                    self.add_token_without_literal(TokenType::BangEqual)
                } else {
                    self.add_token_without_literal(TokenType::Bang)
                }
            }
            '=' => {
                if self.matches('=') {
                    self.add_token_without_literal(TokenType::EqualEqual)
                } else {
                    self.add_token_without_literal(TokenType::Equal)
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token_without_literal(TokenType::LessEqual)
                } else {
                    self.add_token_without_literal(TokenType::Less)
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token_without_literal(TokenType::GreaterEqual)
                } else {
                    self.add_token_without_literal(TokenType::Greater)
                }
            }
            '/' => {
                if self.matches('/'){
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_without_literal(TokenType::Slash);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string();
                // self.add_token(TokenType::Literal,  Some(Literal::String("hallo".to_string())))
            }

            c => {
                if self.is_digit(c){
                    self.number();
                } else if self.is_alphabet(c) {
                    self.identifier();
                }
                else {
                    self.lox.error_lexer(self.line, "Unexpected character.");
                }
            },
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()){
            self.advance();
        }

        let text: &str = &self.source[self.start.. self.current];
        let token_type = KEYWORDS.get(text);
        match token_type {
            Some(reserved_keyword) => {
                self.add_token_without_literal(reserved_keyword.clone())
            }
            None => {
                self.add_token(TokenType::Literal, Some(Literal::Identifier(text.into())))
            }
        }
    }
    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alphabet(c) || self.is_digit(c)
    }
    fn is_alphabet(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
            (c >= 'A' && c <= 'Z') ||
            c == '_'
    }

    fn number(&mut self){
        while self.is_digit(self.peek()){
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()){
            self.advance();

            while self.is_digit(self.peek()){
                self.advance();
            }
        }

        self.add_token(TokenType::Literal, Some(Literal::Number(self.source[self.start .. self.current].parse().unwrap())))
    }

    fn peek_next(&self) -> char{
        if self.current + 1 >= self.source.len(){
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn is_digit(&self, c: char)->bool{
        c.is_digit(10)
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.lox.error_lexer(self.line, "Unterminated string");
            return
        }

        let value: String = self.source[self.start..self.current].into();
        self.add_token(TokenType::Literal, Some(Literal::String(value)));

        self.advance(); // should return '"'

    }

    fn peek(&self) -> char{
        if self.is_at_end(){
            return '\0'
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end(){
            return false
        }

        if self.source.chars().nth(self.current) != Some(expected){
            return false
        }

        self.current += 1;
        return true
    }

    fn advance(&mut self) -> char {
        let response = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        response
    }

    fn add_token_without_literal(&mut self, token_type: TokenType){
        self.add_token(token_type, None)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>){
        let text: String = self.source[self.start..self.current].to_string();

        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}