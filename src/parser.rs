use crate::{ast, lexer::Lexer, token::Token};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,

    current_token: Option<Token>,
    peek_token: Option<Token>,
}
impl<'a> Parser<'a> {
    pub fn new(lexer: &mut Lexer) -> Parser {
        Parser {
            lexer,
            current_token: None,
            peek_token: None,
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        return None;
    }
}
