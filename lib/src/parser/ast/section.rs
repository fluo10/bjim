use crate::parser::Token;
use super::Block;
use super::Header;

#[derive(Debug, PartialEq)]
pub struct Section{
    pub header: Header,
    pub content: Vec<Block>, 
}