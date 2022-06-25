use crate::parser::Token;
#[derive(Debug, PartialEq)]
pub struct Text {
    pub content: Vec<Token>,
}

impl Text {
    pub fn new() -> Text {
        Text{
            content: Vec::new()
        }
    }
}