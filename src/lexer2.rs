// fn advance(lexer: Lexer) -> Lexer {
//     Lexer {
//         input: input,
//     }
// }

use crate::token::Token;

struct Lexer<'a> {
    input: &'a str,
    position: usize,
    ch: Option<char>,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        match input.chars().next() {
            Some(ch) => Lexer{input, position: 0, ch: Some(ch),},
            None => Lexer{input, position:0, ch: None},
        }
    }
    
    pub fn next_token(&self) -> (Lexer, Option<Token>) {
        (Lexer{input: self.input, position: 0, ch: None}, None)
    }
}
#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::*;

    #[test]
    fn it_works() {
        let input = "=+(){},;";
        let tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];
        let lexer = Lexer::new(input.into());
        tokens.iter().for_each(|t| {
            let (_lexer, token) = lexer.next_token();
            assert_eq!(Some(t), token.as_ref());
        });
    }
}
