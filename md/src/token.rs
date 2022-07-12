mod position;

pub use position::TokenPosition;
use crate::errors::ParseError;


use std::convert::From;
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

pub enum CodeBlockFenceChar{}

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
    fn get_literal(&self) -> &str {
        todo!()
    }
    fn get_mut_literal(&mut self) -> &mut str {
        todo!()
    }
    fn get_position(&self) -> Option<&TokenPosition> {
        todo!()
    }
    fn get_mut_position(&mut self) -> Option<&mut TokenPosition> {
        todo!()
    }
    fn len(&self) -> usize {
        todo!()
    }
    fn has_position(&self) -> bool {
        todo!()
    }
    fn take_position(&mut self) -> Option<TokenPosition> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq)]
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

impl From<HeadingPrefixToken> for RawToken {
    fn from(t: HeadingPrefixToken) -> Self {
        Self::HeadingPrefix(t)
    }
}

impl From<CodeBlockFenceToken> for RawToken {
    fn from(t: CodeBlockFenceToken) -> Self {
        Self::CodeBlockFence(t)
    }
}

impl From<IndentToken> for RawToken {
    fn from(t: IndentToken) -> Self {
        Self::Indent(t)
    }
}

impl From<BulletToken> for RawToken {
    fn from(t: BulletToken) -> Self {
        Self::Bullet(t)
    }
}

impl From<HashtagToken> for RawToken {
    fn from(t: HashtagToken) -> Self {
        Self::Hashtag(t)
    }
}

impl From<LeftBracketToken> for RawToken {
    fn from(t: LeftBracketToken) -> Self {
        Self::LeftBracket(t)
    }
}

impl From<RightBracketToken> for RawToken {
    fn from(t: RightBracketToken) -> Self {
        Self::RightBracket(t)
    }
}

impl From<SpaceToken> for RawToken {
    fn from(t: SpaceToken) -> Self {
        Self::Space(t)
    }
}

impl From<WordToken> for RawToken {
    fn from(t: WordToken) -> Self {
        Self::Word(t)
    }
}

impl From<LineBreakToken> for RawToken {
    fn from(t: LineBreakToken) -> Self {
        Self::LineBreak(t)
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct BulletToken{
    content: TokenContent,
}

impl TokenLike for BulletToken {}

impl AsMut<TokenContent> for BulletToken {
    fn as_mut(&mut self) -> &mut TokenContent {
        &mut self.content
    }
}

impl AsRef<TokenContent> for BulletToken {
    fn as_ref(&self) -> &TokenContent {
        &self.content
    }
}

impl TryFrom<TokenContent> for BulletToken {
    type Error = ParseError;
    fn try_from(t: TokenContent) -> Result<Self> {
        Ok(Self{
            content: t,
        })
    }
}

impl TryFrom<&str> for BulletToken {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<String> for BulletToken {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<(usize, usize, &str)> for BulletToken {
    type Error = ParseError;
    fn try_from(t: (usize, usize, &str)) -> Result<Self> {
        TokenContent::from(t).try_into()
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct LeftBracketToken{
    content: TokenContent,
}

impl TokenLike for LeftBracketToken {}

impl AsRef<TokenContent> for LeftBracketToken {
    fn as_ref(&self) -> &TokenContent {
        &self.content
    }
}

impl AsMut<TokenContent> for LeftBracketToken {
    fn as_mut(&mut self) -> &mut TokenContent {
        &mut self.content
    }
}

impl TryFrom<TokenContent> for LeftBracketToken {
    type Error = ParseError;
    fn try_from(t: TokenContent) -> Result<Self> {
        Ok(Self{
            content: t,
        })
    }
}

impl TryFrom<&str> for LeftBracketToken {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<String> for LeftBracketToken {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<(usize, usize, &str)> for LeftBracketToken {
    type Error = ParseError;
    fn try_from(t: (usize, usize, &str)) -> Result<Self> {
        TokenContent::from(t).try_into()
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct RightBracketToken{
    content: TokenContent,
}

impl TokenLike for RightBracketToken {}

impl AsRef<TokenContent> for RightBracketToken {
    fn as_ref(&self) -> &TokenContent {
        &self.content
    }
}

impl AsMut<TokenContent> for RightBracketToken {
    fn as_mut(&mut self) -> &mut TokenContent {
        &mut self.content
    }
}

impl TryFrom<TokenContent> for RightBracketToken {
    type Error = ParseError;
    fn try_from(t: TokenContent) -> Result<Self> {
        Ok(Self{
            content: t,
        })
    }
}

impl TryFrom<&str> for RightBracketToken {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<String> for RightBracketToken {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<(usize, usize, &str)> for RightBracketToken {
    type Error = ParseError;
    fn try_from(t: (usize, usize, &str)) -> Result<Self> {
        TokenContent::from(t).try_into()
    }
}


#[derive(Clone, Debug, PartialEq,)]
pub struct CodeBlockFenceToken{
    content: TokenContent,
}

impl TokenLike for CodeBlockFenceToken {}

impl AsRef<TokenContent> for CodeBlockFenceToken {
    fn as_ref(&self) -> &TokenContent {
        &self.content
    }
}

impl AsMut<TokenContent> for CodeBlockFenceToken {
    fn as_mut(&mut self) -> &mut TokenContent {
        &mut self.content
    }
}

impl TryFrom<TokenContent> for CodeBlockFenceToken {
    type Error = ParseError;
    fn try_from(t: TokenContent) -> Result<Self> {
        Ok(Self{
            content: t,
        })
    }
}

impl TryFrom<&str> for CodeBlockFenceToken {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<String> for CodeBlockFenceToken {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<(usize, usize, &str)> for CodeBlockFenceToken {
    type Error = ParseError;
    fn try_from(t: (usize, usize, &str)) -> Result<Self> {
        TokenContent::from(t).try_into()
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct HashtagToken{
    content: TokenContent,
}

impl TokenLike for HashtagToken {}

impl AsRef<TokenContent> for HashtagToken {
    fn as_ref(&self) -> &TokenContent {
        &self.content
    }
}

impl AsMut<TokenContent> for HashtagToken {
    fn as_mut(&mut self) -> &mut TokenContent {
        &mut self.content
    }
}

impl TryFrom<TokenContent> for HashtagToken {
    type Error = ParseError;
    fn try_from(t: TokenContent) -> Result<Self> {
        Ok(Self{
            content: t,
        })
    }
}

impl TryFrom<&str> for HashtagToken {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<String> for HashtagToken {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<(usize, usize, &str)> for HashtagToken {
    type Error = ParseError;
    fn try_from(t: (usize, usize, &str)) -> Result<Self> {
        TokenContent::from(t).try_into()
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct HeadingPrefixToken{
    content: TokenContent,
}

impl TokenLike for HeadingPrefixToken {}

impl AsRef<TokenContent> for HeadingPrefixToken {
    fn as_ref(&self) -> &TokenContent {
        &self.content
    }
}

impl AsMut<TokenContent> for HeadingPrefixToken {
    fn as_mut(&mut self) -> &mut TokenContent {
        &mut self.content
    }
}

impl TryFrom<TokenContent> for HeadingPrefixToken {
    type Error = ParseError;
    fn try_from(t: TokenContent) -> Result<Self> {
        Ok(Self{
            content: t,
        })
    }
}

impl TryFrom<&str> for HeadingPrefixToken {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<String> for HeadingPrefixToken {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<(usize, usize, &str)> for HeadingPrefixToken {
    type Error = ParseError;
    fn try_from(t: (usize, usize, &str)) -> Result<Self> {
        TokenContent::from(t).try_into()
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct IndentToken{
    content: TokenContent,
}

impl TokenLike for IndentToken {}

impl AsRef<TokenContent> for IndentToken {
    fn as_ref(&self) -> &TokenContent {
        &self.content
    }
}

impl AsMut<TokenContent> for IndentToken {
    fn as_mut(&mut self) -> &mut TokenContent {
        &mut self.content
    }
}

impl TryFrom<TokenContent> for IndentToken {
    type Error = ParseError;
    fn try_from(t: TokenContent) -> Result<Self> {
        Ok(Self{
            content: t,
        })
    }
}

impl TryFrom<&str> for IndentToken {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<String> for IndentToken {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<(usize, usize, &str)> for IndentToken {
    type Error = ParseError;
    fn try_from(t: (usize, usize, &str)) -> Result<Self> {
        TokenContent::from(t).try_into()
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct SpaceToken{
    content: TokenContent,
}

impl TokenLike for SpaceToken {}

impl AsRef<TokenContent> for SpaceToken {
    fn as_ref(&self) -> &TokenContent {
        &self.content
    }
}

impl AsMut<TokenContent> for SpaceToken {
    fn as_mut(&mut self) -> &mut TokenContent {
        &mut self.content
    }
}

impl TryFrom<TokenContent> for SpaceToken {
    type Error = ParseError;
    fn try_from(t: TokenContent) -> Result<Self> {
        Ok(Self{
            content: t,
        })
    }
}

impl TryFrom<&str> for SpaceToken {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<String> for SpaceToken {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<(usize, usize, &str)> for SpaceToken {
    type Error = ParseError;
    fn try_from(t: (usize, usize, &str)) -> Result<Self> {
        TokenContent::from(t).try_into()
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct LineBreakToken{
    content: TokenContent,
}

impl TokenLike for LineBreakToken {}

impl AsRef<TokenContent> for LineBreakToken {
    fn as_ref(&self) -> &TokenContent {
        &self.content
    }
}

impl AsMut<TokenContent> for LineBreakToken {
    fn as_mut(&mut self) -> &mut TokenContent {
        &mut self.content
    }
}

impl TryFrom<TokenContent> for LineBreakToken {
    type Error = ParseError;
    fn try_from(t: TokenContent) -> Result<Self> {
        Ok(Self{
            content: t,
        })
    }
}

impl TryFrom<&str> for LineBreakToken {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<String> for LineBreakToken {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<(usize, usize, &str)> for LineBreakToken {
    type Error = ParseError;
    fn try_from(t: (usize, usize, &str)) -> Result<Self> {
        TokenContent::from(t).try_into()
    }
}

#[derive(Clone, Debug, PartialEq,)]
pub struct WordToken{
    content: TokenContent,
}

impl TokenLike for WordToken {}

impl AsRef<TokenContent> for WordToken {
    fn as_ref(&self) -> &TokenContent {
        &self.content
    }
}

impl AsMut<TokenContent> for WordToken {
    fn as_mut(&mut self) -> &mut TokenContent {
        &mut self.content
    }
}

impl TryFrom<TokenContent> for WordToken {
    type Error = ParseError;
    fn try_from(t: TokenContent) -> Result<Self> {
        Ok(Self{
            content: t,
        })
    }
}

impl TryFrom<&str> for WordToken {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<String> for WordToken {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self> {
        TokenContent::from(s).try_into()
    }
}

impl TryFrom<(usize, usize, &str)> for WordToken {
    type Error = ParseError;
    fn try_from(t: (usize, usize, &str)) -> Result<Self> {
        TokenContent::from(t).try_into()
    }
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