mod brackets;
mod bullet;
mod fences;
mod hashtag;
mod heading_prefix;
mod white_spaces;
mod word;
//mod single_char;
mod position;

pub use brackets::{LeftBracketToken, RightBracketToken};
pub use bullet::BulletToken;
pub use fences::CodeBlockFenceToken;
pub use hashtag::HashtagToken;
pub use heading_prefix::HeadingPrefixToken;
pub use white_spaces::{IndentToken, SpaceToken, LineBreakToken};
pub use word::WordToken;
pub use position::TokenPosition;

use std::convert::From;
use std::fmt;


#[derive(Clone, Debug, PartialEq)]
pub enum Token {

    // Beginning of line
    HeadingPrefix(HeadingPrefixToken),
    CodeBlockFence(CodeBlockFenceToken),
    Indent(IndentToken),
    Bullet(BulletToken),
    //Quotation,

    Hashtag(HashtagToken),
    //LParen,
    //RParen,
    LeftBracket(LeftBracketToken),
    RightBracket(RightBracketToken),
    Space(SpaceToken),
    Word(WordToken),

    // End of line
    LineBreak(LineBreakToken),
}

impl Token {
    pub fn len(&self) -> usize {
        match self {
            Token::HeadingPrefix(x) => x.len(),
            Token::CodeBlockFence(x) => x.len(),
            Token::Indent(x) => x.len(),
            Token::Bullet(x) => x.len(),
            Token::Hashtag(x) => x.len(),
            Token::LeftBracket(x) => x.len(),
            Token::RightBracket(x) => x.len(),
            Token::Space(x) => x.len(),
            Token::Word(x) => x.len(),
            Token::LineBreak(x) => x.len(),
        }
    }

    pub fn is_heading_prefix(&self) -> bool {
        todo!()
    }
    pub fn heading_prefix(&self) -> Option<HeadingPrefixToken> {
        todo!()
    }
    pub fn is_code_block_fence(&self) -> bool {
        todo!()
    }
    pub fn is_indent(&self) -> bool {
        todo!()
    }
    pub fn is_bullet(&self) -> bool {
        todo!()
    }
    pub fn is_text(&self) -> bool {
        todo!()
    }
    pub fn is_hashtag(&self) -> bool {
        todo!()
    }
    pub fn is_left_bracket(&self) -> bool {
        todo!()
    }
    pub fn is_right_bracket(&self) -> bool {
        todo!()
    }
    pub fn is_space(&self) -> bool {
        todo!()
    }
    pub fn is_line_break(&self) -> bool {
        todo!()
    }
}


impl fmt::Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::HeadingPrefix(x) => write!(f, "{}", x),
            Token::CodeBlockFence(x) => write!(f, "{}", x),
            Token::Indent(x) => write!(f, "{}", x),
            Token::Bullet(x) => write!(f, "{}", x),
            Token::Hashtag(x) => write!(f, "{}", x),
            Token::LeftBracket(x) => write!(f, "{}", x),
            Token::RightBracket(x) => write!(f, "{}", x),
            Token::Space(x) => write!(f, "{}", x),
            Token::Word(x) => write!(f, "{}", x),
            Token::LineBreak(x) => write!(f, "{}", x),
        }
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