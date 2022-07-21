mod prefix;
mod traits;

pub use prefix::*;
pub use traits::*;

use crate::errors::ParseError;
use crate::token::*;
use crate::elements::*;

use std::collections::VecDeque;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ParseError>;
#[derive(Clone, Debug, PartialEq)]
pub struct ListElement {
    pub content: Vec<ListItemElement>,
}


impl ListElement {
    pub fn new() -> Self {
        ListElement {
            content: Vec::new(),
        }
    }
}

impl From<Vec<ListItemElement>> for ListElement {
    fn from(v: Vec<ListItemElement>) -> Self {
        Self{
            content: v,
        }
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for ListElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self> {
        if peek_list_indent(&*t).unwrap_or(u8::MAX) != 0 {
            return Err(ParseError::InvalidToken);
        }
        let mut list = ListElement::new();
        while let Ok(list_item) = ListItemElement::try_from(&mut *t) {
            list.content.push(list_item);
        }
        Ok(list)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ListItemElement{
    Note(ListNoteElement),
    Task(ListTaskElement),
}

impl ListItemLike for ListItemElement {
    fn depth(&self) -> u8 {
        match self {
            ListItemElement::Note(x) => x.depth(),
            ListItemElement::Task(x) => x.depth(),
        }
    }
}

impl From<ListNoteElement> for ListItemElement {
    fn from(e: ListNoteElement) -> Self {
        Self::Note(e)
    }
}

impl From<ListTaskElement> for ListItemElement {
    fn from(e: ListTaskElement) -> Self {
        Self::Task(e)
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for ListItemElement {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self>{
        let list_prefix = ListItemPrefix::try_from(&mut *t)?;
        let task_prefix =  TaskPrefix::try_from(&mut *t).ok();

        let content = ListItemContent::try_from(&mut *t).unwrap();
        let mut children = Vec::<ListItemElement>::new();
        while let Some(x) = peek_list_indent(&*t) {
            if x > list_prefix.depth() {
                if let Ok(x) = ListItemElement::try_from(&mut *t) {
                    children.push(x);
                } else {
                    unreachable!();
                }
            } else {
                break;
            }
        }
        match task_prefix {
            Some(x) => Ok(ListTaskElement::from((list_prefix, x, content, children)).into()),
            None => Ok(ListNoteElement::from((list_prefix, content, children)).into())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListItemContent {
    content: Vec<InlineElement>,
}

impl From<Vec<InlineElement>> for ListItemContent {
    fn from(v: Vec<InlineElement>) -> Self {
        Self{
            content: v,
        }
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for ListItemContent {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self>{
        if t.is_empty() {
            return Err(ParseError::TokenNotFound);
        }
        let mut v: Vec<InlineElement> = Vec::new();
        while let Ok(x) = InlineElement::try_from(&mut *t){
            match &x {
                InlineElement::LineBreak(_) => {
                    v.push(x);
                    break;
                },
                _ => {
                    v.push(x);
                }
            }
        }
        Ok(v.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListNoteElement {
    prefix: ListItemPrefix,
    content: ListItemContent,
    children: Vec<ListItemElement>
}

impl ListItemLike for ListNoteElement {
    fn depth(&self) -> u8 {
        self.prefix.depth()
    }
}

impl<T, U> From<(T, U)> for ListNoteElement where 
T: Into<ListItemPrefix>,
U: Into<ListItemContent>,
{
    fn from(t: (T, U)) -> Self {
        Self {
            prefix: t.0.into(),
            content: t.1.into(),
            children: Vec::new(),
        }
    }
}

impl<T, U> From<(T, U, Vec<ListItemElement>)> for ListNoteElement where 
T: Into<ListItemPrefix>,
U: Into<ListItemContent>,
{
    fn from(t: (T, U, Vec<ListItemElement>)) -> Self {
        Self {
            prefix: t.0.into(),
            content: t.1.into(),
            children: t.2,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListTaskElement {
    prefix: ListItemPrefix,
    status: TaskPrefix,
    content: ListItemContent,
    children: Vec<ListItemElement>
}

impl ListItemLike for ListTaskElement {
    fn depth(&self) -> u8 {
        self.prefix.depth()
    }
}

impl<T, U, V> From<(T, U, V)> for ListTaskElement where 
T: Into<ListItemPrefix>,
U: Into<TaskPrefix>,
V: Into<ListItemContent>,
{
    fn from(t: (T, U, V)) -> Self {
        Self {
            prefix: t.0.into(),
            status: t.1.into(),
            content: t.2.into(),
            children: Vec::new(),
        }
    }
}

impl<T, U, V> From<(T, U, V, Vec<ListItemElement>)> for ListTaskElement where 
T: Into<ListItemPrefix>,
U: Into<TaskPrefix>,
V: Into<ListItemContent>,
{
    fn from(t: (T, U, V, Vec<ListItemElement>)) -> Self {
        Self {
            prefix: t.0.into(),
            status: t.1.into(),
            content: t.2.into(),
            children: t.3,
        }
    }
}