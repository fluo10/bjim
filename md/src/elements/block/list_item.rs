use crate::{
    errors::ParseError,
    token::{Token, TokenKind},
    elements::Inline,
};


use std::collections::VecDeque;
use std::convert::{From, TryFrom};

use anyhow::anyhow;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub struct ListItemPrefix {
    pub indent: Option<Token>,
    pub bullet: Token,
    pub space: Token,
}

impl<'a> TryFrom<(Token, Token)> for ListItemPrefix {
    type Error = ParseError;
    fn try_from(tokens: (Token, Token)) -> Result<ListItemPrefix> {
        match (tokens.0, tokens.1) {
            (x, y) if x.kind == TokenKind::Bullet && y.kind == TokenKind::Space => {
                Ok(ListItemPrefix{
                    indent: None,
                    bullet: x,
                    space: y,
                })
            },
            (_, _) => {
                Err(ParseError::InvalidToken)
            }
        }
    }
}

impl TryFrom<(Token, Token, Token)> for ListItemPrefix {
    type Error = ParseError;
    fn try_from(tokens: (Token, Token, Token)) -> Result<ListItemPrefix> {
        if tokens.0.is_indent() && tokens.1.is_bullet() && tokens.2.is_space() {
            Ok(ListItemPrefix{
                indent: Some(tokens.0),
                bullet: tokens.1,
                space: tokens.2,
            })

        } else {
            Err(ParseError::InvalidToken)
        }
    }
}

impl TryFrom<&mut VecDeque<Token>> for ListItemPrefix {
    type Error = ParseError;
    fn try_from(tokens: &mut VecDeque<Token>) -> Result<ListItemPrefix> {
        let indent;
        let bullet;
        let space;

        match (tokens.get(0), tokens.get(1), tokens.get(2)) {
            (Some(x), Some(y), Some(z)) if x.is_indent() && y.is_bullet() && z.is_space() => {
                indent = tokens.pop_front().unwrap();
                bullet = tokens.pop_front().unwrap();
                space = tokens.pop_front().unwrap();
                ListItemPrefix::try_from((indent, bullet, space))
            },
            (Some(x), Some(y), _) if x.is_bullet() && y.is_space() => {
                bullet = tokens.pop_front().unwrap();
                space = tokens.pop_front().unwrap();
                ListItemPrefix::try_from((bullet, space))
            }
            (_, _, _) => {
                Err(ParseError::InvalidToken)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Checkbox {
    pub open: Token,
    pub status: Token,
    pub close: Token,
    pub space: Token,
}

#[derive(Debug, PartialEq)]
pub struct ListItem {
    pub prefix: ListItemPrefix,
    pub checkbox: Option<Checkbox>,
    pub content: Vec<Inline>,
    pub children: Vec<ListItem>,
}

impl TryFrom<&mut VecDeque<Token>> for ListItem {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<Token>) -> Result<Self>{

        let mut list_item: ListItem = ListItemPrefix::try_from(&mut *t)?.into();
        while let Ok(inline) = Inline::try_from(&mut *t) {
            if inline.is_line_break() {
                list_item.content.push(inline);
                break;
            } else {
                list_item.content.push(inline);
            }
        }
        while let Ok(prefix) = PeekedListItemPrefix::try_from(&* t) {
            if  prefix.indent.map_or(0, |x| x.literal.len()) > list_item.prefix.indent.as_ref().map_or(0, |x| x.literal.len()) {
                if let Ok(x) = ListItem::try_from(&mut *t) {
                    list_item.children.push(x);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(list_item)
    }
}

impl From<ListItemPrefix> for ListItem {
    fn from(p: ListItemPrefix) -> Self {
        ListItem{
            prefix: p,
            checkbox: None,
            content: Vec::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PeekedListItemPrefix<'a>{
    pub indent: Option<&'a Token>,
    pub bullet: &'a Token,
    pub space: &'a Token,
}

impl<'a> TryFrom<(&'a Token, &'a Token)> for PeekedListItemPrefix<'a> {
    type Error = ParseError;
    fn try_from(tokens: (&'a Token, &'a Token)) -> Result<PeekedListItemPrefix<'a>> {
        match (&tokens.0.kind, &tokens.1.kind) {
            (TokenKind::Bullet, TokenKind::Space) => {
                Ok(PeekedListItemPrefix{
                    indent: None,
                    bullet: tokens.0,
                    space: tokens.1,
                })
            },
            (_, _) => {
                Err(ParseError::InvalidToken)
            }
        }
    }
}
impl<'a> TryFrom<(&'a Token, &'a Token, &'a Token)> for PeekedListItemPrefix<'a> {
    type Error = ParseError;
    fn try_from(tokens: (&'a Token, &'a Token, &'a Token)) -> Result<PeekedListItemPrefix<'a>> {
        
        match (&tokens.0.kind, &tokens.1.kind, &tokens.2.kind) {
            (TokenKind::Indent, TokenKind::Bullet, TokenKind::Space) => {
                Ok(PeekedListItemPrefix{
                    indent: Some(tokens.0),
                    bullet: tokens.1,
                    space: tokens.2,
                })
            },
            (_, _, _) => {
                Err(ParseError::InvalidToken)
            }
        }
    }
}

impl<'a> TryFrom<&'a VecDeque<Token>> for PeekedListItemPrefix<'a> {
    type Error = ParseError;
    fn try_from(t: &'a VecDeque<Token>) -> Result<PeekedListItemPrefix<'a>> {
        match (t.get(0), t.get(1), t.get(2)) {
            (Some(x), Some(y), Some(z)) if x.kind == TokenKind::Indent && y.kind == TokenKind::Bullet && z.kind == TokenKind::Space => {
                PeekedListItemPrefix::try_from((x, y, z))
            },
            (Some(x), Some(y), _) if x.kind == TokenKind::Bullet && y.kind == TokenKind::Space => {
                PeekedListItemPrefix::try_from((x, y))
            },
            (_, _, _) => {
                Err(ParseError::InvalidToken)
            }
        }
    }
}