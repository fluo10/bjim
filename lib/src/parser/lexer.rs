use super::token::{Token, TokenKind};

use std::convert::From;
use std::iter::Peekable;
use std::str::Chars;

use anyhow::Result;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
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
            chars: s.chars().peekable(),
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
        let token: Token;
        todo!();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        const s: &str = r#######"# Heading

Paragraph.

## List

- Note1
- Note2
    - Child note

## Check list

- [ ] Task1
- [ ] Task2
    - [ ] Child task
    - Child note
"#######;
        let v: Vec<Token> = vec![
            Token{line: 1, col: 1, kind: TokenKind::HeaderPrefix, literal: "#"},
            Token{line: 1, col: 2, kind: TokenKind::Space, literal: " "},
            Token{line: 1, col: 3, kind: TokenKind::Text, literal: "Heading\n" },
            Token{line: 2, col: 1, kind: TokenKind::BlankLine, literal: "\n" },
            Token{line: 3, col: 1, kind: TokenKind::Text, literal: "Paragraph.\n"},
            Token{line: 4, col: 1, kind: TokenKind::BlankLine, literal: "\n" },
            Token{line: 5, col: 1, kind: TokenKind::HeaderPrefix, literal: "##"},
            Token{line: 5, col: 3, kind: TokenKind::Space, literal: " "},
            Token{line: 5, col: 4, kind: TokenKind::Text, literal: "List\n" },
            Token{line: 6, col: 1, kind: TokenKind::BlankLine, literal: "\n" },
            Token{line: 7, col: 1, kind: TokenKind::Bullet, literal: "-"},
            Token{line: 7, col: 2, kind: TokenKind::Space, literal: " "},
            Token{line: 7, col: 3, kind: TokenKind::Text, literal: "item1\n"},
            Token{line: 8, col: 1, kind: TokenKind::Bullet, literal: "-"},
            Token{line: 8, col: 2, kind: TokenKind::Space, literal: " "},
            Token{line: 8, col: 3, kind: TokenKind::Text, literal: "item2\n"},
            Token{line: 9, col: 1, kind: TokenKind::Indent, literal: "    "},
            Token{line: 9, col: 5, kind: TokenKind::Bullet, literal: "-"},
            Token{line: 9, col: 6, kind: TokenKind::Space, literal: " "},
            Token{line: 9, col: 7, kind: TokenKind::Text, literal: "Child item\n"},
            Token{line: 10, col: 1, kind: TokenKind::BlankLine, literal: "\n" },
            Token{line: 11, col: 1, kind: TokenKind::HeaderPrefix, literal: "##"},
            Token{line: 11, col: 3, kind: TokenKind::Space, literal: " "},
            Token{line: 11, col: 4, kind: TokenKind::Text, literal: "Check list\n" },
            Token{line: 12, col: 1, kind: TokenKind::BlankLine, literal: "\n" },
            Token{line: 13, col: 1, kind: TokenKind::Bullet, literal: "-"},
            Token{line: 13, col: 2, kind: TokenKind::Space, literal: " "},
            Token{line: 13, col: 3, kind: TokenKind::CheckBox, literal: "[ ]"},
            Token{line: 13, col: 6, kind: TokenKind::Space, literal: " "},
            Token{line: 13, col: 7, kind: TokenKind::Text, literal: "Task1\n"},
            Token{line: 14, col: 1, kind: TokenKind::Bullet, literal: "-"},
            Token{line: 14, col: 2, kind: TokenKind::Space, literal: " "},
            Token{line: 14, col: 3, kind: TokenKind::CheckBox, literal: "[ ]"},
            Token{line: 14, col: 6, kind: TokenKind::Space, literal: " "},
            Token{line: 14, col: 7, kind: TokenKind::Text, literal: "Task2\n"},
            Token{line: 15, col: 1, kind: TokenKind::Indent, literal: "    "},
            Token{line: 15, col: 5, kind: TokenKind::Bullet, literal: "-"},
            Token{line: 15, col: 6, kind: TokenKind::Space, literal: " "},
            Token{line: 15, col: 7, kind: TokenKind::CheckBox, literal: "[ ]"},
            Token{line: 15, col: 10, kind: TokenKind::Space, literal: " "},
            Token{line: 15, col: 11, kind: TokenKind::Text, literal: "Child task\n"},
            Token{line: 16, col: 1, kind: TokenKind::Indent, literal: "    "},
            Token{line: 16, col: 5, kind: TokenKind::Bullet, literal: "-"},
            Token{line: 16, col: 6, kind: TokenKind::Space, literal: " "},
            Token{line: 16, col: 7, kind: TokenKind::Text, literal: "Child note\n"},
        ];
        let t: Vec<Token> = Lexer::from(s).collect();
        assert_eq!(t, v);
        
    }
}