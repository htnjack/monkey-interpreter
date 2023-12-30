use crate::token::Token;

struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}
impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_int(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            0 => Token::Eof,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEquals
                } else {
                    Token::Bang
                }
            },
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Equals
                } else {
                    Token::Assign
                }
            },
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return match ident.as_str() {
                    "fn" => Token::Fn,
                    "let" => Token::Let,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    "true" => Token::True,
                    "false" => Token::False,
                    _ => Token::Ident(ident),
                };
            }
            b'0'..=b'9' => {
                return Token::Number(self.read_int());
            }
            _ => Token::Illegal,
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
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
        let mut lexer = Lexer::new(input.into());
        tokens.iter().for_each(|t| {
            let token = lexer.next_token();
            assert_eq!(t, &token);
        });
    }

    #[test]
    fn it_works_2() {
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
        "#;
        let tokens = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Number("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Number("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Fn,
            Token::Lparen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Lparen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Number("5".into()),
            Token::Semicolon,
            Token::Number("5".into()),
            Token::LessThan,
            Token::Number("10".into()),
            Token::GreaterThan,
            Token::Number("5".into()),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Number("5".into()),
            Token::LessThan,
            Token::Number("10".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Eof,
        ];
        let mut lexer = Lexer::new(input.into());
        tokens.iter().for_each(|t| {
            let token = lexer.next_token();
            assert_eq!(t, &token);
        });
    }
}
