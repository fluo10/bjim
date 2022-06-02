use std::convert::From;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Heading(i8),
    Space(&'a str),
    Bullet(char),
    CheckBox(char),
    CodeBrock(&'a str),
    Text(&'a str),
    HashTag(&'a str),
}

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
impl<'a> fmt::Display for Token<'a>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}