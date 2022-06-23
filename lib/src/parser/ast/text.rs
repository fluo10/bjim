use crate::parser::Token;
#[derive(Debug, PartialEq)]
pub struct Text {
    pub content: Vec<Token>,
}