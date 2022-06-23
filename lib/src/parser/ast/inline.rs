use crate::parser::Token;
use super::Text;

#[derive(Debug, PartialEq)]
pub enum Inline {
    Text(Text),
}