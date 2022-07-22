use crate::impl_token;
use crate::token::*;

use crate::errors::ParseError;

use std::collections::VecDeque;
use std::convert::From;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum LexedToken {

    // Single char token
    //Asterisk(AsteriskToken),
    BackQuote(BackQuoteToken),
    Hash(HashToken),
    Hyphen(HyphenToken),
    //Plus(PlusToken),
    Tilde(TildeToken),
    //LParen,
    //RParen,
    LeftBracket(LeftBracketToken),
    RightBracket(RightBracketToken),

    // multiple char token
    Space(SpaceToken),
    Word(WordToken),

    LineBreak(LineBreakToken),

}

impl LexedToken {

    pub fn is_back_quote(&self) -> bool {   
        match self {
            Self::BackQuote(_) => true,
            _ => false
        }
    }
    pub fn is_hash(&self) -> bool {
        match self {
            Self::Hash(_) => true,
            _ => false
        }
    }
    pub fn is_hyphen(&self) -> bool {
        match self {
            Self::Hyphen(_) => true,
            _ => false
        }
    }
    pub fn is_tilde(&self) -> bool {
        match self {
            Self::Tilde(_) => true,
            _ => false
        }
    }
    pub fn is_left_bracket(&self) -> bool {
        match self {
            Self::LeftBracket(_) => true,
            _ => false
        }
    }
    pub fn is_right_bracket(&self) -> bool {
        match self {
            Self::RightBracket(_) => true,
            _ => false
        }
    }
    pub fn is_space(&self) -> bool {
        match self {
            Self::Space(_) => true,
            _ => false
        }
    }
    pub fn is_word(&self) -> bool {
        match self {
            Self::Word(_) => true,
            _ => false
        }
    }
    pub fn is_line_break(&self) -> bool {
        match self {
            Self::LineBreak(_) => true,
            _ => false
        }
    }

    pub fn next_position(&self) -> Option<TokenPosition> {
        let mut position = &self.get_position()?.clone();
        Some(*position + self)
    }
}


impl_token!{
    LexedToken {
        BackQuote(BackQuoteToken),
        Hash(HashToken),
        Hyphen(HyphenToken),
        Tilde(TildeToken),
        LeftBracket(LeftBracketToken),
        RightBracket(RightBracketToken),
        Space(SpaceToken),
        Word(WordToken),
        LineBreak(LineBreakToken),
    }
}

impl TryFrom<&mut VecDeque<char>> for LexedToken {
    type Error = ParseError;
    fn try_from(q: &mut VecDeque<char>) -> Result<LexedToken> {
        let result : LexedToken = match q.front().ok_or(ParseError::ParseTokenError)? {
            &'`' => {
                BackQuoteToken::try_from(q).unwrap().into()
            },
            &'#' => {
                HashToken::try_from(q).unwrap().into()
            },
            &'-' => {
                HyphenToken::try_from(q).unwrap().into()
            },
            &'~' => {
                TildeToken::try_from(q).unwrap().into()
            },
            &'[' => {
                LeftBracketToken::try_from(q).unwrap().into()
            },
            &']' => {
                RightBracketToken::try_from(q).unwrap().into()
            },
            &' ' => {
                SpaceToken::try_from(q).unwrap().into()
            },
            &'\n' | '\r' => {
                LineBreakToken::try_from(q).unwrap().into()
            },
            _ => {
                WordToken::try_from(q).unwrap().into()
            }
        };
        Ok(result)
    }

}

#[derive(Clone, Debug, PartialEq)]
pub enum ParsedToken {
    BackQuote(BackQuoteToken),
    Hash(HashToken),
    Hyphen(HyphenToken),
    //Plus(PlusToken),
    Tilde(TildeToken),
    //LParen,
    //RParen,
    LeftBracket(LeftBracketToken),
    RightBracket(RightBracketToken),

    // multiple char token
    Space(SpaceToken),
    Word(WordToken),

    LineBreak(LineBreakToken),

    Indent(IndentToken),
    Bullet(BulletToken),
    HeadingPrefix(HeadingPrefixToken),
    Status(StatusToken),
}

impl_token!{
    ParsedToken{
        BackQuote(BackQuoteToken),
        Hash(HashToken),
        Hyphen(HyphenToken),
        Tilde(TildeToken),
        LeftBracket(LeftBracketToken),
        RightBracket(RightBracketToken),
        Space(SpaceToken),
        Word(WordToken),
        LineBreak(LineBreakToken),
        Indent(IndentToken),
        Bullet(BulletToken),
        HeadingPrefix(HeadingPrefixToken),
        Status(StatusToken),
}}

impl From<LexedToken> for ParsedToken {
    fn from(l: LexedToken) -> Self {
        match l {
            LexedToken::BackQuote(x) => Self::BackQuote(x),
            LexedToken::Hash(x) => Self::Hash(x),
            LexedToken::Hyphen(x) => Self::Hyphen(x),
            LexedToken::Tilde(x) => Self::Tilde(x),
            LexedToken::LeftBracket(x) => Self::LeftBracket(x),
            LexedToken::RightBracket(x) => Self::RightBracket(x),
            LexedToken::Space(x) => Self::Space(x),
            LexedToken::Word(x) => Self::Word(x),
            LexedToken::LineBreak(x) => Self::LineBreak(x),
        }
    }
}