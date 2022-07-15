mod position;

pub use position::TokenPosition;
use crate::errors::ParseError;


use std::convert::From;
use std::collections::VecDeque;
use std::fmt;

type Result<T> = std::result::Result<T, ParseError>;


pub enum BulletChar{
    Hyphen,
    Asterisk,
    Plus,
}

const HYPHEN_CHAR: &char = &'-';
const ASTERISK_CHAR: &char = &'*';
const PLUS_CHAR: &char = &'+';

impl BulletChar{

    pub fn as_char(&self) -> &'static char{
        use BulletChar::*;
        match self {
            Hyphen => HYPHEN_CHAR,
            Asterisk => ASTERISK_CHAR,
            Plus => PLUS_CHAR,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        use BulletChar::*;
        match &c {
            HYPHEN_CHAR => Some(Hyphen),
            ASTERISK_CHAR => Some(Asterisk),
            PLUS_CHAR => Some(Plus),
            _ => None
        }
    }
    pub fn contains(c: &char) -> bool {
        match c {
            HYPHEN_CHAR | ASTERISK_CHAR | PLUS_CHAR => true,
            _ => false
        }
    }
}

impl fmt::Display for BulletChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BulletChar::*;
        write!(f, "{}", self.as_char())
    }
}

impl TryFrom<char> for BulletChar {
    type Error = ParseError;
    fn try_from(c: char) -> Result<BulletChar> {
        Self::from_char(c).ok_or(ParseError::InvalidChar{expected: "-*+", found: c})
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenContent {
    position: Option<TokenPosition>,
    literal: String, 
}

impl AsRef<TokenContent> for TokenContent {
    fn as_ref(&self) -> &TokenContent {
        self
    }
}

impl AsMut<TokenContent> for TokenContent {
    fn as_mut(&mut self) -> &mut TokenContent {
        self
    }
}

impl fmt::Display for TokenContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}
impl From<char> for TokenContent {
    fn from(c: char) -> Self {
        TokenContent{
            position: None,
            literal: String::from(c),
        }
    }
}

impl From<&str> for TokenContent {
    fn from(s: &str) -> Self {
        TokenContent{
            position: None,
            literal: s.to_string(),
        }
    }
}

impl From<String> for TokenContent {
    fn from(s: String) -> TokenContent {
        TokenContent{
            position: None,
            literal: s,
        }
    }
}

impl From<(usize, usize, &str)> for TokenContent {
    fn from(t: (usize, usize, &str)) -> TokenContent {
        let (line, column, literal) = t;
        TokenContent{
            position: Some((line, column).into()),
            literal: literal.to_string(),
        }
    }
}

impl From<(usize, usize, String)> for TokenContent {
    fn from(t: (usize, usize, String)) -> TokenContent {
        let (line, column, literal) = t;
        TokenContent{
            position: Some((line, column).into()),
            literal: literal,
        }
    }
}

pub trait TokenLike: AsRef<TokenContent> + AsMut<TokenContent> {
    fn get_literal(&self) -> &str;
    fn get_mut_literal(&mut self) -> &mut str;
    fn get_position(&self) -> Option<&TokenPosition>;
    fn get_mut_position(&mut self) -> Option<&mut TokenPosition>;
    fn len(&self) -> usize;
    fn has_position(&self) -> bool;
    fn take_position(&mut self) -> Option<TokenPosition>;
    fn insert_position(&mut self, p: TokenPosition);
}

impl<T> TokenLike for T where
T: AsRef<TokenContent> + AsMut<TokenContent>,
{
    fn get_literal(&self) -> &str {
        &self.as_ref().literal
    }
    fn get_mut_literal(&mut self) -> &mut str {
        &mut self.as_mut().literal
    }
    fn get_position(&self) -> Option<&TokenPosition> {
        self.as_ref().position.as_ref()
    }
    fn get_mut_position(&mut self) -> Option<&mut TokenPosition> {
        self.as_mut().position.as_mut()
    }
    fn len(&self) -> usize {
        self.as_ref().literal.len()
    }
    fn has_position(&self) -> bool {
        todo!()
    }
    fn take_position(&mut self) -> Option<TokenPosition> {
        todo!()
    }
    fn insert_position(&mut self, p: TokenPosition) {
        self.as_mut().position.insert(p);
    }
}

macro_rules! token_struct_builder {
    ($struct_name:ident) => {
        impl AsRef<TokenContent> for $struct_name {
            fn as_ref(&self) -> &TokenContent {
                &self.content
            }
        }
        impl AsMut<TokenContent> for $struct_name {
            fn as_mut(&mut self) -> &mut TokenContent {
                &mut self.content
            }
        }
        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.content.fmt(f)
            }
        }
        impl TryFrom<char> for $struct_name {
            type Error = ParseError;
            fn try_from(c: char) -> Result<Self> {
                TokenContent::from(c).try_into()
            }
        }
        impl TryFrom<TokenContent> for $struct_name {
            type Error = ParseError;
            fn try_from(t: TokenContent) -> Result<Self> {
                Ok(Self{
                    content: t,
                })
            }
        }
        impl TryFrom<&str> for $struct_name {
            type Error = ParseError;
            fn try_from(s: &str) -> Result<Self> {
                TokenContent::from(s).try_into()
            }
        }
        impl TryFrom<String> for $struct_name {
            type Error = ParseError;
            fn try_from(s: String) -> Result<Self> {
                TokenContent::from(s).try_into()
            }
        }
        impl TryFrom<(usize, usize, &str)> for $struct_name {
            type Error = ParseError;
            fn try_from(t: (usize, usize, &str)) -> Result<Self> {
                TokenContent::from(t).try_into()
            }
        }
    } 
}

#[derive(Clone, Debug, PartialEq,)]
pub struct BackQuoteToken {
    content: TokenContent,
}

token_struct_builder!(BackQuoteToken);

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

token_struct_builder!(HashToken);

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

token_struct_builder!(HyphenToken);

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

token_struct_builder!(TildeToken);

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

token_struct_builder!(LeftBracketToken);

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

token_struct_builder!(RightBracketToken);

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

token_struct_builder!(SpaceToken);

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

token_struct_builder!(WordToken);

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

token_struct_builder!(LineBreakToken);

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