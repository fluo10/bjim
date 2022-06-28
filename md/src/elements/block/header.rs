use crate::{
    errors::ParseError, 
    token::{Token, TokenKind},
    elements::Inline,
};

use std::collections::VecDeque;
use std::convert::{From, TryFrom, TryInto};

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub struct HeaderPrefix {
    pub prefix: Token,
    pub space: Token,
}

impl HeaderPrefix {
    pub fn heading_level(&self) -> u8 {
        self.prefix.len().try_into().unwrap()
    }
}

impl TryFrom<&mut VecDeque<Token>> for HeaderPrefix {
    type Error = ParseError;
    fn try_from(t:&mut VecDeque<Token>) -> Result<Self> {
        match (t.front().map(|x| x.kind), t.get(1).map(|x| x.kind)) {
            (None, _) | (_, None) => {
                Err(ParseError::TokenNotFound)
            },
            (Some(TokenKind::HeaderPrefix), Some(TokenKind::Space)) => {
                let prefix= t.pop_front().unwrap();
                let space= t.pop_front().unwrap();
                Ok(HeaderPrefix{
                    prefix: prefix,
                    space: space,
                })
            },
            (_, _) => {
                Err(ParseError::InvalidToken)
            }
        }

    }
    
}
#[derive(Debug, PartialEq)]
pub struct PeekedHeaderPrefix<'a> {
    pub prefix: &'a Token,
    pub space: &'a Token,
}

impl<'a> PeekedHeaderPrefix<'a> {
    pub fn heading_level(&'a self) -> u8 {
        self.prefix.len().try_into().unwrap()
    }
}

impl<'a> TryFrom<(&'a Token, &'a Token)> for PeekedHeaderPrefix<'a> {
    type Error = ParseError;
    fn try_from(t: (&'a Token, &'a Token)) -> Result<Self> {
        if t.0.is_header_prefix() && t.1.is_space() {
            Ok(PeekedHeaderPrefix{
                prefix: t.0,
                space: t.1,
            })
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
impl<'a> TryFrom<&'a VecDeque<Token>> for PeekedHeaderPrefix<'a> {
    type Error = ParseError;
    fn try_from(t:&'a VecDeque<Token>) -> Result<Self> {
        match (t.get(0), t.get(1)) {
            (None, _) | (_, None) => {
                Err(ParseError::TokenNotFound)
            },
            (Some(x), Some(y)) => {
                PeekedHeaderPrefix::try_from((x, y))
            },
            (_, _) => {
                Err(ParseError::InvalidToken)
            }
        }

    }
    
}
#[derive(Debug, PartialEq)]
pub struct Header {
    pub prefix: HeaderPrefix,
    pub content: Vec<Inline>
}

impl Header {
    pub fn heading_level(&self) -> u8 {
        self.prefix.heading_level()
    }
}
impl TryFrom<&mut VecDeque<Token>> for Header {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<Token>) -> Result<Self> {
        let prefix = HeaderPrefix::try_from(&mut *t)?;
        let mut inline: Vec<Inline> = Vec::new();
        while let Ok(x) = Inline::try_from(&mut *t) {
            if x.is_line_break() {
                inline.push(x);
                break;
            } else {
                inline.push(x);
            }
        }
        Ok(Header{
            prefix: prefix,
            content: inline,
        })
    }
}
