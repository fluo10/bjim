use super::Token;
use super::Lexer;
use super::ast::*;

use anyhow::Result;

use std::iter::FromIterator;

pub struct Parser {
    tokens: Vec<Token>,
    
    cur_token: Option<Token>,
    peek_token: Option<Token>
}

impl Parser {
    pub fn try_parse_body(&mut self) -> Result<Body>{
        todo!();
    }
    pub fn try_parse_blank_line(&mut self) {
        todo!();
    }
    pub fn try_parse_header(&mut self) {
        todo!();
    }
    pub fn try_parse_list(&mut self) {
        todo!();
    }
    pub fn try_parse_paragraph(&mut self) {
        todo!();
    }
    pub fn try_parse_section(&mut self) {
        todo!();
    }
    pub fn try_parse_text(&mut self) {
        todo!();
    }
}

impl From<Vec<Token>> for Parser {
    fn from(v: Vec<Token>) -> Self {
        Parser{
            tokens: v,
            cur_token: None,
            peek_token: None,
        }
    }
}

impl From<Lexer<'_>> for Parser {
    fn from(l: Lexer) -> Self {
        Parser::from(l.collect::<Vec<Token>>())
    }
}

impl FromIterator<Token> for Parser {
    fn from_iter<T: IntoIterator<Item = Token>>(iter: T) -> Parser {
        let tokens: Vec<Token> = iter.into_iter().collect();
        Parser::from(tokens)
    }
}

