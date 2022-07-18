use crate::errors::ParseError;
use crate::token::*;

use std::convert::{From, TryFrom};
use std::fmt;
use std::ops::Add;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenContent {
    pub position: Option<TokenPosition>,
    pub literal: String, 
}

impl<T> Add<T> for TokenContent where
T: Into<TokenContent>,
{
    type Output = TokenContent;
    fn add(self, other: T) -> TokenContent {
        TokenContent {
            position: self.position,
            literal: (self.literal + &other.into().literal),
        }
    } 
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

macro_rules! token_content_from {
    ($enum_name:ident {$($child_name:ident,$child_type:ty,)+}) => {
        impl From<$enum_name> for TokenContent {
            fn from($enum_name) -> TokenContent {
                match self {
                    $(Self::$child_name(x) => x.into(),)+
                }
            }
        }
    };
    ($($struct_name:ident),+) => {
        $(
            impl From<$struct_name> for TokenContent {
                fn from(t: $struct_name) -> Self {
                    t.content
                }
            }
        )+
    };
}


token_content_from!(
    BackQuoteToken,
    HashToken,
    HyphenToken,
    TildeToken,
    LeftBracketToken,
    RightBracketToken,
    SpaceToken,
    WordToken,
    LineBreakToken
);

