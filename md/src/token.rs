use std::convert::From;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenKind {

    // Beginning of line
    HeaderPrefix,
    CodeBlockFence,
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

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub line: usize,
    pub column: usize,
    pub literal: String,
    pub kind : TokenKind,
}
impl Token {
    pub fn len(&self) -> usize {
        self.literal.len()
    }
    pub fn is_header_prefix(&self) -> bool {
        self.kind == TokenKind::HeaderPrefix
    }
    pub fn is_code_block_fence(&self) -> bool {
        self.kind == TokenKind::CodeBlockFence
    }
    pub fn is_indent(&self) -> bool {
        self.kind == TokenKind::Indent
    }
    pub fn is_bullet(&self) -> bool {
        self.kind == TokenKind::Bullet
    }
    //pub fn is_quotation(&self) -> bool {
    //    self.kind == TokenKind::Quotation
    //}

    pub fn is_text(&self) -> bool {
        self.kind == TokenKind::Text
    }
    pub fn is_hashtag(&self) -> bool {
        self.kind == TokenKind::HashTag
    }
    //pub fn is_lparen(&self) -> bool {
    //    self.kind == TokenKind::LParen
    //}
    //pub fn is_rparen(&self) -> bool {
    //    self.kind == TokenKind::RParen
    //}
    pub fn is_left_bracket(&self) -> bool {
        self.kind == TokenKind::LBracket
    }
    pub fn is_right_brachet(&self) -> bool {
        self.kind == TokenKind::RBracket
    }
    pub fn is_space(&self) -> bool {
        self.kind == TokenKind::Space
    }
    pub fn is_line_break(&self) -> bool {
        self.kind == TokenKind::LineBreak
    }
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