use crate::token::*;
use crate::errors::ParseError;

use std::collections::VecDeque;
use std::convert::From;
use std::fmt;
use std::iter::Peekable;
use std::ops::{Add, AddAssign};
use std::str::Chars;


use anyhow::{anyhow,bail,Error,};

type Result<T> = std::result::Result<T, ParseError>;

pub struct Lexer {
    chars: VecDeque<char>,
    position_buf: Option<TokenPosition>,
    token_buf: Option<LexedToken>,
}

impl Lexer {

    /// Insert position to token then increment this position.
    /// 
    /// # Panics
    /// 
    /// Panics if `token_buf` is `None`
    fn update_position(&mut self) -> Option<&mut TokenPosition>{
        if let Some(mut x) = self.position_buf.take() {
            self.token_buf.as_mut().unwrap().insert_position(x);
            Some(self.position_buf.insert(self.token_buf.as_ref().unwrap().next_position().unwrap()))
        } else {
            None
        }
    }
}

impl From<&str> for Lexer {
    fn from(s: &str) -> Lexer {
        Lexer {
            chars: s.chars().collect(),
            position_buf: Some(TokenPosition::new()),
            token_buf: None,
        }
    }
}

impl From<String> for Lexer {
    fn from(s: String) -> Lexer {
        Lexer {
            chars: s.chars().collect(),
            position_buf: Some(TokenPosition::new()),
            token_buf: None,
        }
    }
}

impl Iterator for Lexer {
    type Item = LexedToken;

    fn next (&mut self) ->  Option<LexedToken> {
        self.token_buf.insert(LexedToken::try_from(&mut self.chars).ok()?);
        self.update_position();
        self.token_buf.take()
    }

}

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

macro_rules! token_enum_builder {
    ($enum_name:ident {$($child_name:ident,$child_type:ty,)+}) => {
        impl AsRef<TokenContent> for $enum_name {
            fn as_ref(&self) -> &TokenContent {
                match self {
                    $(Self::$child_name(x) => x.as_ref(),)+
                }
            }
        }
        impl AsMut<TokenContent> for $enum_name {
            fn as_mut(&mut self) -> &mut TokenContent {
                match self {
                    $(Self::$child_name(x) => x.as_mut(),)+
                }
            }
        }
        impl fmt::Display for $enum_name{
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$child_name(x) => x.fmt(f),)+
                }
            }
        }
        $(
            impl From<$child_type> for $enum_name {
                fn from(t: $child_type) -> Self {
                    Self::$child_name(t)
                }
            }
        )+
    }
}

token_enum_builder!{
    LexedToken {
        BackQuote, BackQuoteToken,
        Hash, HashToken,
        Hyphen, HyphenToken,
        Tilde, TildeToken,
        LeftBracket, LeftBracketToken,
        RightBracket, RightBracketToken,
        Space, SpaceToken,
        Word, WordToken,
        LineBreak, LineBreakToken,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        const s: &str = r#######"# Heading

Paragraph.

## List

- Note1
- Note2
    - Child note

## Check list

- [ ] Task1
- [ ] Task2
    - [ ] Child task
    - Child note
"#######;
        use LexedToken::*;
        let v: Vec<LexedToken> = vec![
            HashToken::try_from(( 1,  1, "#")).unwrap().into(),
            SpaceToken::try_from((1,  2, " ")).unwrap().into(),
            WordToken::try_from(( 1,  3, "Heading")).unwrap().into(),
            LineBreakToken::try_from(( 1, 10, "\n")).unwrap().into(),
            LineBreakToken::try_from(( 2,  1, "\n")).unwrap().into(),
            WordToken::try_from(( 3,  1, "Paragraph.")).unwrap().into(),
            LineBreakToken::try_from(( 3, 11, "\n")).unwrap().into(),
            LineBreakToken::try_from(( 4,  1, "\n")).unwrap().into(),
            HashToken::try_from(( 5,  1, "#")).unwrap().into(),
            HashToken::try_from(( 5,  2, "#")).unwrap().into(),
            SpaceToken::try_from(( 5,  3, " ")).unwrap().into(),
            WordToken::try_from(( 5,  4, "List")).unwrap().into(),
            LineBreakToken::try_from(( 5,  8, "\n")).unwrap().into(),
            LineBreakToken::try_from(( 6,  1, "\n")).unwrap().into(),
            HyphenToken::try_from(( 7,  1, "-")).unwrap().into(),
            SpaceToken::try_from(( 7,  2, " ")).unwrap().into(),
            WordToken::try_from(( 7,  3, "Note1")).unwrap().into(),
            LineBreakToken::try_from(( 7,  8, "\n")).unwrap().into(),
            HyphenToken::try_from(( 8,  1, "-")).unwrap().into(),
            SpaceToken::try_from(( 8,  2, " ")).unwrap().into(),
            WordToken::try_from(( 8,  3, "Note2")).unwrap().into(),
            LineBreakToken::try_from(( 8,  8, "\n")).unwrap().into(),
            SpaceToken::try_from(( 9,  1, "    ")).unwrap().into(),
            HyphenToken::try_from(( 9,  5, "-")).unwrap().into(),
            SpaceToken::try_from(( 9,  6, " ")).unwrap().into(),
            WordToken::try_from(( 9,  7, "Child")).unwrap().into(),
            SpaceToken::try_from(( 9, 12, " ")).unwrap().into(),
            WordToken::try_from(( 9, 13, "note")).unwrap().into(),
            LineBreakToken::try_from(( 9, 17, "\n")).unwrap().into(),
            LineBreakToken::try_from((10,  1, "\n")).unwrap().into(),
            HashToken::try_from((11,  1, "#")).unwrap().into(),
            HashToken::try_from((11,  2, "#")).unwrap().into(),
            SpaceToken::try_from((11,  3, " ")).unwrap().into(),
            WordToken::try_from((11,  4, "Check")).unwrap().into(),
            SpaceToken::try_from((11,  9, " ")).unwrap().into(),
            WordToken::try_from((11, 10, "list")).unwrap().into(),
            LineBreakToken::try_from((11, 14, "\n")).unwrap().into(),
            LineBreakToken::try_from((12,  1, "\n")).unwrap().into(),
            HyphenToken::try_from((13,  1, "-")).unwrap().into(),
            SpaceToken::try_from((13,  2, " ")).unwrap().into(),
            LeftBracketToken::try_from((13,  3, "[")).unwrap().into(),
            SpaceToken::try_from((13,  4, " ")).unwrap().into(),
            RightBracketToken::try_from((13,  5, "]")).unwrap().into(),
            SpaceToken::try_from((13,  6, " ")).unwrap().into(),
            WordToken::try_from((13,  7, "Task1")).unwrap().into(),
            LineBreakToken::try_from((13, 12, "\n")).unwrap().into(),
            HyphenToken::try_from((14,  1, "-")).unwrap().into(),
            SpaceToken::try_from((14,  2, " ")).unwrap().into(),
            LeftBracketToken::try_from((14,  3, "[")).unwrap().into(),
            SpaceToken::try_from((14,  4, " ")).unwrap().into(),
            RightBracketToken::try_from((14,  5, "]")).unwrap().into(),
            SpaceToken::try_from((14,  6, " ")).unwrap().into(),
            WordToken::try_from((14,  7, "Task2")).unwrap().into(),
            LineBreakToken::try_from((14, 12, "\n")).unwrap().into(),
            SpaceToken::try_from((15,  1, "    ")).unwrap().into(),
            HyphenToken::try_from((15,  5, "-")).unwrap().into(),
            SpaceToken::try_from((15,  6, " ")).unwrap().into(),
            LeftBracketToken::try_from((15,  7, "[")).unwrap().into(),
            SpaceToken::try_from((15,  8, " ")).unwrap().into(),
            RightBracketToken::try_from((15,  9, "]")).unwrap().into(),
            SpaceToken::try_from((15, 10, " ")).unwrap().into(),
            WordToken::try_from((15, 11, "Child")).unwrap().into(),
            SpaceToken::try_from((15, 16, " ")).unwrap().into(),
            WordToken::try_from((15, 17, "task")).unwrap().into(),
            LineBreakToken::try_from((15, 21, "\n")).unwrap().into(),
            SpaceToken::try_from((16,  1, "    ")).unwrap().into(),
            HyphenToken::try_from((16,  5, "-")).unwrap().into(),
            SpaceToken::try_from((16,  6, " ")).unwrap().into(),
            WordToken::try_from((16,  7, "Child")).unwrap().into(),
            SpaceToken::try_from((16, 12, " ")).unwrap().into(),
            WordToken::try_from((16, 13, "note")).unwrap().into(),
            LineBreakToken::try_from((16, 17, "\n")).unwrap().into(),
        ];
        let t: Vec<LexedToken> = Lexer::from(s).collect();
        assert_eq!(t, v);
        
    }
}