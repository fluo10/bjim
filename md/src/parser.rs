use crate::{
    token::{Token, TokenKind},
    lexer::Lexer,
    elements::*,
};
use anyhow::Result;

use std::iter::{FromIterator, Peekable};
use std::collections::{VecDeque};
use std::convert::{From, TryFrom};

#[derive(Debug, PartialEq)]
pub struct Parser{
    token_queue: VecDeque<Token>,
}



impl Parser {
    pub fn get_token_kind(&self, index: usize) -> Option<&TokenKind> {
        self.token_queue.get(index).map(|t| &t.kind)
    }
    pub fn get_token(&self, index: usize) -> Option<&Token> {
        self.token_queue.get(index)
    }
    pub fn pop_token(&mut self) -> Option<Token> {
        self.token_queue.pop_front()
    }
    pub fn parse(&mut self) -> Section {
        Section::try_from(&mut self.token_queue).unwrap()
    }

}
impl Default for Parser
{
    fn default() -> Self {
        todo!();
    }
}
impl From<Vec<Token>> for Parser {
    fn from(v: Vec<Token>) -> Self {
        Parser{
            token_queue: v.into_iter().collect(),
        }
    }
}

impl<'a> From<Lexer<'a>> for Parser {
    fn from(l: Lexer<'a>) -> Self {
        Parser{
            token_queue: l.collect(),
        }
    }
}

/*
impl<I> FromIterator<Token> for Parser<I>
where 
    I: Iterator<Item = Token>
{
    fn from_iter<T>(iter: T) -> Self
    where 
        T: IntoIterator<Item = Token>,
    {
        Parser{
            token_iter: iter.into_iter().peekable(),
            cur_token: None,
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
         const s: &str = r#######"# Heading

Paragraph.

## List

- Note1
- Note2
    - Child note
"#######;
        let found: Section = Parser::from(Lexer::from(s)).parse();
        let expected = Section{
            header: Some(Header{
                prefix: HeaderPrefix{
                    prefix: Token{line: 1, column: 1, kind: TokenKind::HeaderPrefix, literal: "#".to_string()},
                    space: Token{line: 1, column: 2, kind: TokenKind::Space, literal: " ".to_string()},
                },
                content: vec![
                    Inline::Text(Text{
                        content: vec![
                            Token{line: 1, column: 3, kind: TokenKind::Text, literal: "Heading".to_string()},
                        ],
                    }),
                    Inline::LineBreak(
                        Token{line: 1, column: 10, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                    ),   
                ],
            }),
            content: vec![
                Block::BlankLine(BlankLine{
                    indent: None,
                    line_break: Token{line: 2, column: 1, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                }),
                Block::Paragraph(Paragraph{
                    content: vec![
                        Inline::Text(Text{
                            content: vec![
                                Token{line: 3, column: 1, kind: TokenKind::Text, literal: "Paragraph.".to_string()},
                            ],
                        }),
                        Inline::LineBreak(
                            Token{line: 3, column: 11, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                        ),
                    ]
                }),
                Block::BlankLine(BlankLine{
                    indent: None,
                    line_break: Token{line: 4, column: 1, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                }),
            ],
            children: vec![
                Section{
                    header: Some(Header{
                        prefix: HeaderPrefix {
                            prefix: Token{line: 5, column: 1, kind: TokenKind::HeaderPrefix, literal: "##".to_string()},
                            space: Token{line: 5, column: 3, kind: TokenKind::Space, literal: " ".to_string()},
                        },
                        content: vec![
                            Inline::Text(Text{
                                content: vec![
                                    Token{line: 5, column: 4, kind: TokenKind::Text, literal: "List".to_string()},
                                ]
                            }),
                            Inline::LineBreak(
                                    Token{line: 5, column: 8, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                            ),
                        ]
                    }),
                    content: vec![
                        Block::BlankLine(BlankLine{
                            indent: None,
                            line_break: Token{line: 6, column: 1, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                        }),
                        Block::List(List{
                            content: vec![
                                ListItem{
                                    prefix: ListItemPrefix{
                                        indent: None,
                                        bullet: Token{line: 7, column: 1, kind: TokenKind::Bullet, literal: "-".to_string()},
                                        space: Token{line: 7, column: 2, kind: TokenKind::Space, literal: " ".to_string()},
                                    },
                                    checkbox: None,
                                    content: vec![
                                        Inline::Text(Text{
                                            content: vec![
                                                Token{line: 7, column: 3, kind: TokenKind::Text, literal: "Note1".to_string()},
                                            ]
                                        }),
                                        Inline::LineBreak(
                                                Token{line: 7, column: 8, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                                        ),
                                    ],
                                    children: Vec::new(),
                                },
                                ListItem{
                                    prefix: ListItemPrefix{
                                        indent: None,
                                        bullet: Token{line: 8, column: 1, kind: TokenKind::Bullet, literal: "-".to_string()},
                                        space: Token{line: 8, column: 2, kind: TokenKind::Space, literal: " ".to_string()},
                                    },
                                    checkbox: None,
                                    content: vec![
                                        Inline::Text(Text{
                                            content: vec![
                                                Token{line: 8, column: 3, kind: TokenKind::Text, literal: "Note2".to_string()},
                                            ]
                                        }),
                                        Inline::LineBreak(
                                                Token{line: 8, column: 8, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                                        ),
                                    ],
                                    children: vec![
                                        ListItem {
                                            prefix: ListItemPrefix{
                                                indent: Some(Token{line: 9, column: 1, kind: TokenKind::Indent, literal: "    ".to_string()}),
                                                bullet: Token{line: 9, column: 5, kind: TokenKind::Bullet, literal: "-".to_string()},
                                                space: Token{line: 9, column: 6, kind: TokenKind::Space, literal: " ".to_string()},
                                            },
                                            checkbox: None,
                                            content: vec![
                                                Inline::Text(Text{
                                                    content: vec![
                                                        Token{line: 9, column: 7, kind: TokenKind::Text, literal: "Child".to_string()},
                                                        Token{line: 9, column: 12, kind: TokenKind::Space, literal: " ".to_string()},
                                                        Token{line: 9, column: 13, kind: TokenKind::Text, literal: "note".to_string()},
                                                    ]
                                                }),
                                                Inline::LineBreak(
                                                        Token{line: 9, column: 17, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                                                ),
                                            ],
                                            children: Vec::new(),
                                        },
                                    ],
                                },
                            ],
                        }),                 
                    ],
                    children: Vec::new(),
                },
            ],
        };
        assert_eq!(found,expected);
    }

}