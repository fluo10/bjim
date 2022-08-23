mod content;
mod enums;
mod position;
mod token_like;

pub use content::TokenContent;
pub use enums::*;
pub use position::TokenPosition;
pub use token_like::TokenLike;
use crate::errors::ParseError;
use crate::impl_token;


use std::convert::From;
use std::collections::VecDeque;
use std::fmt;
use std::ops::{Add, AddAssign};

type Result<T> = std::result::Result<T, ParseError>;


pub enum BulletChar{
    Hyphen,
    Asterisk,
    Plus,
}

const HYPHEN_CHAR: &char = &'-';
const ASTERISK_CHAR: &char = &'*';
const PLUS_CHAR: &char = &'+';

#[derive(Clone, Debug, PartialEq,)]
pub struct BackQuoteToken {
    content: TokenContent,
}

impl_token!(BackQuoteToken);

impl TryFrom<&mut VecDeque<char>> for BackQuoteToken {
    type Error = ParseError;
    fn try_from(q: &mut VecDeque<char>) -> Result<Self> {
        match q.front() {
            Some('`') => q.pop_front().unwrap().try_into(),
            _ => Err(ParseError::ParseTokenError)
        }
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct HashToken{
    content: TokenContent,
}

impl_token!(HashToken);

impl TryFrom<&mut VecDeque<char>> for HashToken {
    type Error = ParseError;
    fn try_from(q: &mut VecDeque<char>) -> Result<Self> {
        match q.front() {
            Some(&'#') => q.pop_front().unwrap().try_into(),
            _ => Err(ParseError::ParseTokenError) 
        }
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct HyphenToken{
    content: TokenContent,
}

impl_token!(HyphenToken);

impl TryFrom<&mut VecDeque<char>> for HyphenToken {
    type Error = ParseError;
    fn try_from(q: &mut VecDeque<char>) -> Result<Self> {
        match q.get(0) {
            Some(&'-') => q.pop_front().unwrap().try_into(),
            _ => Err(ParseError::ParseTokenError)
        } 
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct TildeToken{
    content: TokenContent,
}

impl_token!(TildeToken);

impl TryFrom<&mut VecDeque<char>> for TildeToken {
    type Error = ParseError;
    fn try_from(q: &mut VecDeque<char>) -> Result<Self> {
        match q.front() {
            Some(&'~') => q.pop_front().unwrap().try_into(),
            _ => Err(ParseError::ParseTokenError)
        } 
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct LeftBracketToken{
    content: TokenContent,
}

impl_token!(LeftBracketToken);

impl TryFrom<&mut VecDeque<char>> for LeftBracketToken {
    type Error = ParseError;
    fn try_from(q: &mut VecDeque<char>) -> Result<Self> {
        match q.front() {
            Some(&'[') => q.pop_front().unwrap().try_into(),
            _ => Err(ParseError::ParseTokenError)
        }
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct RightBracketToken{
    content: TokenContent,
}

impl_token!(RightBracketToken);

impl TryFrom<&mut VecDeque<char>> for RightBracketToken {
    type Error = ParseError;
    fn try_from(q: &mut VecDeque<char>) -> Result<Self> {
        match q.front() {
            Some(']') => q.pop_front().unwrap().try_into(),
            _ => Err(ParseError::ParseTokenError)
        }
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct SpaceToken{
    content: TokenContent,
}

impl_token!(SpaceToken);

impl TryFrom<&mut VecDeque<char>> for SpaceToken {
    type Error = ParseError;
    fn try_from(q: &mut VecDeque<char>) -> Result<Self> {
        let mut buf = String::new();
        while let Some(x) = q.front() {
            match x {
                &' ' => buf.push(q.pop_front().unwrap()),
                _ => break
            }
        }
        if buf.len() > 0 {
            buf.try_into()
        } else {
            Err(ParseError::ParseTokenError)
        }
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct WordToken{
    content: TokenContent,
}

impl_token!(WordToken);

impl TryFrom<&mut VecDeque<char>> for WordToken {
    type Error = ParseError;
    fn try_from(q: &mut VecDeque<char>) -> Result<Self> {
        let mut buf = String::new();
        while let Some(x) = q.front() {
            match x {
                &'`' | &'#' | &'-' | &'-' | &'[' | &']' | &' ' | &'\n' | &'\r' => {
                    break;
                },
                _ => buf.push(q.pop_front().unwrap())
            }
        }
        if buf.len() > 0 {
            buf.try_into()
        } else {
            Err(ParseError::ParseTokenError)
        }
    }
}



#[derive(Clone, Debug, PartialEq,)]
pub struct LineBreakToken{
    content: TokenContent,
}

impl_token!(LineBreakToken);

impl TryFrom<&mut VecDeque<char>> for LineBreakToken {
    type Error = ParseError;
    fn try_from(q: &mut VecDeque<char>) -> Result<Self> {
        match (q.get(0), q.get(1)) {
            (Some(&'\n'), _) => q.pop_front().unwrap().try_into(),
            (Some(&'\r'), Some(&'\n')) => {
                let mut s = String::from(q.pop_front().unwrap());
                s.push(q.pop_front().unwrap());
                s.try_into()
            },
            _ => Err(ParseError::ParseTokenError)
        }
    }
}


#[derive(Clone, Debug, PartialEq,)]
pub enum BulletToken {
    //Asterisk(AsteriskToken)
    Hyphen(HyphenToken),
    //Plus(PlusToken)
}

impl_token!(BulletToken {
    Hyphen(HyphenToken),
 } );

#[derive(Clone, Debug, PartialEq,)]
pub struct CodeBlockFenceToken {
    content: TokenContent,
}

impl From<(BackQuoteToken, BackQuoteToken, BackQuoteToken)> for CodeBlockFenceToken {
    fn from(t: (BackQuoteToken, BackQuoteToken, BackQuoteToken)) -> Self {
        (TokenContent::from(t.0) + t.1 + t.2).try_into().unwrap()
    }
}

impl_token!(CodeBlockFenceToken);


#[derive(Clone, Debug, PartialEq,)]
pub struct HeadingPrefixToken {
    content: TokenContent,
}

impl_token!(HeadingPrefixToken);

impl AddAssign<HashToken> for HeadingPrefixToken {
    fn add_assign(&mut self, rhs: HashToken) {
        self.as_mut().add_assign(rhs);
    }
}

impl From<HashToken> for HeadingPrefixToken {
    fn from(t: HashToken) -> Self {
        TokenContent::from(t).try_into().unwrap()
    }
}

impl From<(HashToken, HashToken)> for HeadingPrefixToken {
    fn from(t: (HashToken, HashToken)) -> Self {
        (TokenContent::from(t.0) + t.1).try_into().unwrap()
    }
}

impl From<(HashToken, HashToken, HashToken)> for HeadingPrefixToken {
    fn from(t: (HashToken, HashToken, HashToken)) -> Self {
        (TokenContent::from(t.0) + t.1 + t.2).try_into().unwrap()
    }
}

impl From<(HashToken, HashToken, HashToken, HashToken)> for HeadingPrefixToken {
    fn from(t: (HashToken, HashToken, HashToken, HashToken)) -> Self {
        (TokenContent::from(t.0) + t.1 + t.2 + t.3).try_into().unwrap()
    }
}

impl From<(HashToken, HashToken, HashToken, HashToken, HashToken)> for HeadingPrefixToken {
    fn from(t: (HashToken, HashToken, HashToken, HashToken, HashToken)) -> Self {
        (TokenContent::from(t.0) + t.1 + t.2 + t.3 + t.4).try_into().unwrap()
    }
}

impl From<(HashToken, HashToken, HashToken, HashToken, HashToken, HashToken)> for HeadingPrefixToken {
    fn from(t: (HashToken, HashToken, HashToken, HashToken, HashToken, HashToken)) -> Self {
        (TokenContent::from(t.0) + t.1 + t.2 + t.3 + t.4 + t.5).try_into().unwrap()
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct IndentToken {
    content: TokenContent,
}

impl_token!(IndentToken);

impl From<SpaceToken> for IndentToken {
    fn from(t: SpaceToken) -> Self {
        TokenContent::from(t).try_into().unwrap()
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct StatusToken {
    content: TokenContent,
}

impl_token!(StatusToken);

impl TryFrom<LexedToken> for StatusToken {
    type Error = ParseError;
    fn try_from(t: LexedToken) -> Result<Self> {
        if t.len() == 1 {
            Self::try_from(TokenContent::from(t))
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct TextToken {
    content: TokenContent,
}

impl_token!(TextToken);

impl From<Vec<LexedToken>> for TextToken {
    fn from(v: Vec<LexedToken>) -> Self {
        let mut i = v.into_iter().map(|x| TokenContent::from(x));
        let mut initial = i.next().unwrap(); 
        for token in i {
            initial += token;
        } 
        Self{
            content: initial,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_str_token<T>(s: &'static str, t: T )
    where T: for<'a> TryFrom<&'a mut VecDeque<char>, Error = ParseError> + std::fmt::Debug + PartialEq,
    {
        assert_eq!(T::try_from(&mut s.chars().collect::<VecDeque<char>>()).unwrap(), t);
    }

    #[test]
    fn back_quote() {
        assert_str_token("`", BackQuoteToken::try_from("`").unwrap());
    }

    #[test]
    fn hash() {
        assert_str_token("# ", HashToken::try_from("#").unwrap());
    }
    
    #[test]
    fn tilde() {
        assert_str_token("~ ", TildeToken::try_from("~").unwrap());
    }

    #[test]
    fn left_bracket() {
        assert_str_token("[ ", LeftBracketToken::try_from("[").unwrap());
    }

    #[test]
    fn right_bracket() {
        assert_str_token("] ", RightBracketToken::try_from("]").unwrap());
    }
    
    #[test]
    fn space() {
        assert_str_token("  x", SpaceToken::try_from("  ").unwrap());
    }

    #[test]
    fn word() {
        assert_str_token("word ", WordToken::try_from("word").unwrap());
    }
    
    #[test]
    fn line_break() {
        assert_str_token("\nnext line", LineBreakToken::try_from("\n").unwrap());
    }
    /*
    #[test] 
    fn enum_derive() {
        let token = LineBreakToken{
            position: (0, 0).into(),
            literal: "\n".into(),
        };
        let Lexed_token = LexedToken::LineBreak(token.clone());

        assert!(Lexed_token.is_line_break());
        assert_eq!(raw_token.line_break(), Some(&token));
        assert_eq!(RawToken::from(token), raw_token);
    }
    */
    /*
    #[test]
    fn Rawtoken() {
        fn assert_token(s: &str, Rawtoken: Token) {
            
            assert_eq!(Token::from(s), token);
        }
        assert_token("#", Token::Heading(1));
        assert_token("-", Token::Hyphen('-'));
        assert_token("*", Token::Bullet('*'));
        assert_token(" ", Token::Space(" "));

    }
    */
}