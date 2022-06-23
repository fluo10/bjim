use crate::parser::Token;

#[derive(Debug, PartialEq)]
pub struct BlankLine{
    pub indent: Option<Token>,
    pub line_break: Token,
}