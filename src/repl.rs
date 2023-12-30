use crate::lexer::Lexer;
use crate::token::Token;
use std::io::Stdin;

pub struct Repl {
    input: Stdin,
}
impl Repl {
    pub fn new(input: Stdin) -> Self {
        Self { input }
    }
    pub fn run(&self) {
        loop {
            let mut line = String::new();
            self.input.read_line(&mut line).unwrap();
            let mut lexer = Lexer::new(line);
            loop {
                let token = lexer.next_token();
                if token == Token::Eof {
                    break;
                }
                println!("{:?}", token);
            }
        }
    }
}
