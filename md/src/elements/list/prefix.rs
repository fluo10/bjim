use crate::errors::ParseError;
use crate::token::*;

use std::collections::VecDeque;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub struct ListItemPrefix {
    pub indent: Option<SpaceToken>,
    pub bullet: BulletToken,
    pub space: SpaceToken,
}

impl ListItemPrefix {
    pub fn depth(&self) -> u8 {
        self.indent.as_ref().map_or(0, |x| x.len()).try_into().unwrap_or(u8::MAX)
    }
}

impl<T> From<(T, SpaceToken)> for ListItemPrefix where
T: Into<BulletToken>,
{
    fn from(t: (T, SpaceToken)) -> Self {
        ListItemPrefix {
            indent: None,
            bullet: t.0.into(),
            space: t.1,
        }
    }
}

impl<T> From<(SpaceToken, T, SpaceToken)> for ListItemPrefix where
T: Into<BulletToken>
{
    fn from(t: (SpaceToken, T, SpaceToken)) -> Self {
        ListItemPrefix {
            indent: Some(t.0),
            bullet: t.1.into(),
            space: t.2,
        }
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for ListItemPrefix {
    type Error = ParseError;
    fn try_from(tokens: &mut VecDeque<LexedToken>) -> Result<ListItemPrefix> {
        use LexedToken::*;
        match (tokens.get(0), tokens.get(1), tokens.get(2)) {
            (Some(&Space(_)),Some(&Hyphen(_)),Some(&Space(_))) => {
                if let (Space(x), Hyphen(y), Space(z)) = (tokens.pop_front().unwrap(), tokens.pop_front().unwrap(), tokens.pop_front().unwrap()) {
                    Ok(ListItemPrefix::from((x, y, z)))
                } else {
                    unreachable!()
                }
            },
            (Some(&Hyphen(_)), Some(&Space(_)), _) => {
                if let (Hyphen(x), Space(y)) = (tokens.pop_front().unwrap(), tokens.pop_front().unwrap()) {
                    Ok(ListItemPrefix::from((x, y)))
                } else {
                    unreachable!()
                }
            },
            (_, _, _) => {
                Err(ParseError::InvalidToken)
            }
        }
    }
}

pub fn peek_list_indent(t: &VecDeque<LexedToken>) -> Option<u8> {
    use LexedToken::*;
    match (t.get(0), t.get(1), t.get(2)) {
        (Some(&Space(_)),Some(&Hyphen(_)),Some(&Space(_))) => {
            Some(t.get(0).as_ref().unwrap().len().try_into().unwrap_or(u8::MAX))
        },
        (Some(&Hyphen(_)), Some(&Space(_)), _) => {
            Some(0)
        }
        (_, _, _) => {
            None
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct TaskPrefix {
    pub open: LeftBracketToken,
    pub status: StatusToken,
    pub close: RightBracketToken,
    pub space: SpaceToken,
}

impl From<(LeftBracketToken, StatusToken, RightBracketToken, SpaceToken)> for TaskPrefix {
    fn from(t: (LeftBracketToken, StatusToken, RightBracketToken, SpaceToken)) -> Self {
        TaskPrefix {
            open: t.0,
            status: t.1,
            close: t.2,
            space: t.3,
        }
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for TaskPrefix {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        use LexedToken::*;
        match (t.get(0), t.get(1), t.get(2), t.get(3)) {
            (None, _, _, _) => {
                Err(ParseError::TokenNotFound)
            },
            (
                Some(LexedToken::LeftBracket(_)),
                Some(x),
                Some(LexedToken::RightBracket(_)),
                Some(LexedToken::Space(_))
            ) if x.len() == 1 => {
                if let (LeftBracket(x), y, RightBracket(z), Space(a)) = (t.pop_front().unwrap(), t.pop_front().unwrap(), t.pop_front().unwrap(), t.pop_front().unwrap()) {
                    Ok(TaskPrefix::from((
                        x,
                        y.try_into().unwrap(),
                        z,
                        a
                    )))
                } else {unreachable!()}
            },
            _ => Err(ParseError::InvalidToken)
        }
    }
}
