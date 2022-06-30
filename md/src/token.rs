mod position;

pub use position::TokenPosition;

use macros::TokenLike;
use macros_derive::{TokenLike, EnumIs, EnumGet, EnumFrom,};




use std::convert::From;
use std::fmt;



#[derive(Clone, Debug, PartialEq, TokenLike, EnumIs, EnumGet, EnumFrom,)]
pub enum RawToken {

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

impl RawToken {
    
    /*pub fn len(&self) -> usize {
        match self {
            RawToken::HeadingPrefix(x) => x.len(),
            RawToken::CodeBlockFence(x) => x.len(),
            RawToken::Indent(x) => x.len(),
            RawToken::Bullet(x) => x.len(),
            RawToken::Hashtag(x) => x.len(),
            RawToken::LeftBracket(x) => x.len(),
            RawToken::RightBracket(x) => x.len(),
            RawToken::Space(x) => x.len(),
            RawToken::Word(x) => x.len(),
            RawToken::LineBreak(x) => x.len(),
        }
    }
    */
    

    //pub fn is_heading_prefix(&self) -> bool {
        //todo!()
    //}
    //pub fn heading_prefix(&self) -> Option<HeadingPrefixToken> {
        //todo!()
    //}
    //pub fn is_code_block_fence(&self) -> bool {
        //todo!()
    //}
    //pub fn is_indent(&self) -> bool {
        //todo!()
    //}
    //pub fn is_bullet(&self) -> bool {
        //todo!()
    //}
    //pub fn is_text(&self) -> bool {
        //todo!()
    //}
    //pub fn is_hashtag(&self) -> bool {
        //todo!()
    //}
    //pub fn is_left_bracket(&self) -> bool {
        //todo!()
    //}
    //pub fn is_right_bracket(&self) -> bool {
        //todo!()
    //}
    //pub fn is_space(&self) -> bool {
        //todo!()
    //}
    //pub fn is_line_break(&self) -> bool {
        //todo!()
    //}
}


/*
impl fmt::Display for RawToken{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RawToken::HeadingPrefix(x) => write!(f, "{}", x),
            RawToken::CodeBlockFence(x) => write!(f, "{}", x),
            RawToken::Indent(x) => write!(f, "{}", x),
            RawToken::Bullet(x) => write!(f, "{}", x),
            RawToken::Hashtag(x) => write!(f, "{}", x),
            RawToken::LeftBracket(x) => write!(f, "{}", x),
            RawToken::RightBracket(x) => write!(f, "{}", x),
            RawToken::Space(x) => write!(f, "{}", x),
            RawToken::Word(x) => write!(f, "{}", x),
            RawToken::LineBreak(x) => write!(f, "{}", x),
        }
    }
}
*/

#[derive(Clone, Debug, PartialEq, TokenLike)]
pub struct BulletToken{
    position: TokenPosition,
    literal: String,   
}

#[derive(Clone, Debug, PartialEq, TokenLike)]
pub struct LeftBracketToken{
    position: TokenPosition,
    literal: String,   
}

#[derive(Clone, Debug, PartialEq, TokenLike)]
pub struct RightBracketToken{
    position: TokenPosition,
    literal: String,   
}


#[derive(Clone, Debug, PartialEq, TokenLike)]
pub struct CodeBlockFenceToken{
    position: TokenPosition,
    literal: String,   
}

#[derive(Clone, Debug, PartialEq, TokenLike)]
pub struct HashtagToken{
    position: TokenPosition,
    literal: String,   
}

#[derive(Clone, Debug, PartialEq, TokenLike)]
pub struct HeadingPrefixToken{
    position: TokenPosition,
    literal: String,   
}
#[derive(Clone, Debug, PartialEq, TokenLike)]
pub struct IndentToken{
    position: TokenPosition,
    literal: String,   
}

#[derive(Clone, Debug, PartialEq, TokenLike)]
pub struct SpaceToken{
    position: TokenPosition,
    literal: String,   
}

#[derive(Clone, Debug, PartialEq, TokenLike)]
pub struct LineBreakToken{
    position: TokenPosition,
    literal: String,   
}

#[derive(Clone, Debug, PartialEq, TokenLike)]
pub struct WordToken{
    position: TokenPosition,
    literal: String,   
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn enum_derive() {
        let token = LineBreakToken{
            position: (0, 0).into(),
            literal: "\n".into(),
        };
        let raw_token = RawToken::LineBreak(token.clone());

        assert!(raw_token.is_line_break());
        assert_eq!(raw_token.line_break(), Some(&token));
        assert_eq!(RawToken::from(token), raw_token);
    }
    /*
    #[test]
    fn Rawtoken() {
        fn assert_token(s: &str, Rawtoken: Token) {
            
            assert_eq!(Token::from(s), token);
        }
        assert_token("#", Token::Heading(1));
        assert_token("-", Token::Bullet('-'));
        assert_token("*", Token::Bullet('*'));
        assert_token(" ", Token::Space(" "));

    }
    */
}