use crate::parser::{
    errors::ParseError, 
    token::{Token, TokenKind},
};

use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub struct BlankLine{
    pub indent: Option<Token>,
    pub line_break: Token,
}

/// #Examples
/// 
/// ```
/// use blank_line::*;
/// let token = Token::from(TokenKind::LineBreak, "\n");
/// assert_eq!(BlankLine::try_from(token.clone()), BlankLine{indent: None, line_break: token.clone(),});
/// ```
impl TryFrom<Token> for BlankLine {
    type Error = ParseError;
    fn try_from(t: Token) -> Result<Self> {
        if t.is_line_break() {
            Ok(BlankLine{
                indent: None,
                line_break: t,
            })
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}
/// #Examples
/// 
/// ```
/// let x = BlankLine::try_from((Token::from(TokenKind::Indent, "    "), Token::from(TokenKind::LineBreak, "\n"));
/// assert_eq!(x, BlankLine{indent: Token::from(TokenKind::Indent, "    "), line_break: Token::from(TokenKind::LineBreak, "\n")})
/// ```
impl TryFrom<(Token, Token)> for BlankLine {
    type Error = ParseError;
    fn try_from(t: (Token, Token)) -> Result<Self> {
        let (indent, line_break) = t;
        if indent.is_indent() && line_break.is_line_break() {
            Ok(BlankLine{
                indent: Some(indent),
                line_break: line_break,
            })
        } else {
            Err(ParseError::InvalidToken)
        }
    }
}

impl TryFrom<&mut VecDeque<Token>> for BlankLine {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<Token>) -> Result<Self> {
        match (t.get(0), t.get(1)) {
            (Some(x), _) if x.is_line_break() => {
                t.pop_front().unwrap().try_into()
            },
            (Some(x), Some(y)) if x.is_indent() && y.is_line_break() => {
                (t.pop_front().unwrap(), t.pop_front().unwrap()).try_into()
            }
            _ => {
                Err(ParseError::InvalidToken)
            }
        }
    }
}