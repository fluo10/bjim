use std::convert::From;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenKind {

    // Beginning of line
    HeaderPrefix,
    CodeBrockFence,
    Indent,
    Bullet,
    //Quotation,

    Text,
    HashTag,
    //LParen,
    //RParen,
    LBracket,
    RBracket,
    Space,

    // End of line
    LineBreak,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub line: usize,
    pub column: usize,
    pub literal: String,
    pub kind : TokenKind,
}

impl From<(usize, usize, TokenKind, String)> for Token {
    fn from( f: (usize, usize, TokenKind, String)) -> Self {
        Token{
            line: f.0 ,
            column: f.1,
            kind: f.2,
            literal: f.3,
        }
    }
}

#[cfg(test)]
impl From<(TokenKind, &str)> for Token {
    fn from( f: (TokenKind, &str)) -> Self {
        Token{
            line: 0,
            column: 0,
            kind: f.0,
            literal: f.1.to_string(),
        }
    }
}
/*
impl<'a> From<&'a str> for Token<'a> {
    fn from(s:&'a str) -> Token<'a> {
        match s {
            "#" => Token::Heading(1),
            "-" => Token::Bullet('-'),
            "*" => Token::Bullet('*'),
            " " => Token::Space(" "),
            _ => Token::Text(s),
        }

    }

}
*/

impl fmt::Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
    #[test]
    fn token() {
        fn assert_token(s: &str, token: Token) {
            
            assert_eq!(Token::from(s), token);
        }
        assert_token("#", Token::Heading(1));
        assert_token("-", Token::Bullet('-'));
        assert_token("*", Token::Bullet('*'));
        assert_token(" ", Token::Space(" "));

    }
    */
}