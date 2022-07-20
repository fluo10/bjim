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
#[derive(Debug, PartialEq)]
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

impl TryFrom<&mut VecDeque<LexedToken>> for ListItemContent {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self>{
        todo!();
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

impl From<(ListItemPrefix, ListItemContent, Vec<ListItemElement>)> for ListNoteElement {
    fn from(t: (ListItemPrefix, ListItemContent, Vec<ListItemElement>)) -> ListNoteElement {
        Self{
            prefix: t.0,
            content: t.1,
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

impl From<(ListItemPrefix, TaskPrefix, ListItemContent, Vec<ListItemElement>)> for ListTaskElement {
    fn from(t: (ListItemPrefix, TaskPrefix, ListItemContent, Vec<ListItemElement>)) -> ListTaskElement {
        Self{
            prefix: t.0,
            status: t.1,
            content: t.2,
            children: t.3,
        }
    }
}
