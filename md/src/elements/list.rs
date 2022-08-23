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
    pub content: Vec<ListItemTree>,
}


impl ListElement {
    pub fn new() -> Self {
        ListElement {
            content: Vec::new(),
        }
    }
}

impl From<Vec<ListItemTree>> for ListElement {
    fn from(v: Vec<ListItemTree>) -> Self {
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
        while let Ok(list_item) = ListItemTree::try_from(&mut *t) {
            list.content.push(list_item);
        }
        Ok(list)
    }
}

impl From<ListElement> for Vec<ParsedToken> {
    fn from(e: ListElement) -> Vec<ParsedToken> {
        let mut v = Vec::new();
        for li in e.content.into_iter() {
            v.append(&mut li.into());
        }
        v
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListItemTree{
    value: ListItemElement,
    children: Vec<ListItemTree> 
}

impl ListItemLike for ListItemTree {
    fn depth(&self) -> u8 {
        self.value.depth()
    }
}


impl<T> From<T> for ListItemTree where
T: Into<ListItemElement>
{
    fn from(e: T) -> Self {
        Self{
            value: e.into(),
            children: Vec::new(),
        }
    }
}
impl<T, U> From<(T, Vec<U>)> for ListItemTree where
T: Into<ListItemElement>,
U: Into<ListItemTree>
{
    fn from(e: (T, Vec<U>)) -> Self {
        Self{
            value: e.0.into(),
            children: e.1.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl TryFrom<&mut VecDeque<LexedToken>> for ListItemTree {
    type Error = ParseError;
    fn try_from(t: &mut VecDeque<LexedToken>) -> Result<Self>{
        let value = ListItemElement::try_from(&mut *t)?;
        let mut children = Vec::new();
        while let Some(x) = peek_list_indent(&*t) {
            if x > value.depth() {
                if let Ok(x) = ListItemTree::try_from(&mut *t) {
                    children.push(x);
                } else {
                    unreachable!();
                }
            } else {
                break;
            }
        }
        Ok(Self{
            value: value,
            children: children,
        })
    }
}

impl From<ListItemTree> for Vec<ParsedToken> {
    fn from(e: ListItemTree) -> Vec<ParsedToken> {
        let mut v = Vec::new();
        v.append(&mut e.value.into());
        for child in e.children.into_iter() {
            v.append(&mut child.into());
        }
        v
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
        match task_prefix {
            Some(x) => Ok(ListTaskElement::from((list_prefix, x, content)).into()),
            None => Ok(ListNoteElement::from((list_prefix, content)).into())
        }
    }
}

impl From<ListItemElement> for Vec<ParsedToken> {
    fn from(e: ListItemElement) -> Vec<ParsedToken> {
        match e {
            ListItemElement::Note(x) => x.into(),
            ListItemElement::Task(x) => x.into(),
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

impl From<ListItemContent> for Vec<ParsedToken> {
    fn from(e: ListItemContent) -> Vec<ParsedToken> {
        let mut v = Vec::new();
        for i in e.content.into_iter() {
            v.append(&mut i.into());
        }
        v
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListNoteElement {
    prefix: ListItemPrefix,
    content: ListItemContent,
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
        }
    }
}

impl From<ListNoteElement> for Vec<ParsedToken> {
    fn from(e: ListNoteElement) -> Vec<ParsedToken> {
        let mut v = Vec::new();
        v.append(&mut e.prefix.into());
        v.append(&mut e.content.into());
        v
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListTaskElement {
    prefix: ListItemPrefix,
    status: TaskPrefix,
    content: ListItemContent,
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
        }
    }
}

impl From<ListTaskElement> for Vec<ParsedToken> {
    fn from(e: ListTaskElement) -> Vec<ParsedToken> {
        let mut v = Vec::new();
        v.append(&mut e.prefix.into());
        v.append(&mut e.status.into());
        v.append(&mut e.content.into());
        v
    }
}