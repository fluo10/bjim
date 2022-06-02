use super::token::Token;

use std::convert::From;
use std::iter::Peekable;
use std::str::Chars;

use anyhow::Result;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: isize,
    read_position: isize,
    ch: Option<char>,
    line: isize,
    column: isize,
}

impl<'a> Lexer<'a> {
    fn read_char(&'a mut self) -> Result<()> {
        todo!();
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(s: &'a str) -> Lexer<'a> {
        Lexer {
            input: s.chars().peekable(),
            position: 0,
            read_position: 0,
            ch: None,
            line: 0,
            column : 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next (&mut self) -> Option<Self::Item> {
        todo!();
    }

}