use super::Token;
use super::TokenKind;
use super::Lexer;
use super::ast::*;

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
    pub fn parse(&mut self) -> Body {
        let mut body= Body::new();
        while let Some(block) = self.parse_block() {
            body.content.push(block);
        };
        body
    }
    pub fn parse_blank_line(&mut self) -> Option<BlankLine>{
        let indent = if self.get_token_kind(0) == Some(&TokenKind::Indent) {
            self.pop_token()
        } else {
            None
        };
        assert_eq!(self.get_token_kind(0), Some(&TokenKind::LineBreak));
        
        Some(BlankLine{
                    indent: indent,
                    line_break: self.pop_token().unwrap(),
        })
    }
    pub fn parse_block(&mut self)  -> Option<Block>{
        let first_token = self.get_token(0)?;

        let block: Block = match (first_token.kind, self.get_token_kind(1)) {
            (TokenKind::LineBreak, _) | (TokenKind::Indent, Some(TokenKind::LineBreak)) => {
                self.parse_blank_line().map_or_else(|| self.parse_paragraph().unwrap().into(), |x| x.into())
            },
            (TokenKind::Bullet, Some(TokenKind::Space)) => {
                self.parse_list().map_or_else(|| self.parse_paragraph().unwrap().into(), |x| x.into())
            },
            (TokenKind::HeaderPrefix, Some(TokenKind::Space)) => {
                self.parse_section().map_or_else(|| self.parse_paragraph().unwrap().into(), |x| x.into())
            },
            (_, _) => {
                self.parse_paragraph().unwrap().into()
            }
        };
        Some(block)

    }
    pub fn parse_header(&mut self) -> Option<Header> {
        todo!();
    }
    pub fn parse_list(&mut self) -> Option<List>{
        assert_eq!((self.get_token_kind(0), self.get_token_kind(1)), (Some(&TokenKind::Bullet), Some(&TokenKind::Space)));

        let mut list = List::new();
        while let Some(list_item) = self.parse_list_item() {
            list.content.push(list_item);
        }

        Some(list)
        
    }
    pub fn parse_list_item(&mut self) -> Option<ListItem>{
        
        if PeekedListItemPrefix::try_from((self.get_token(0), self.get_token(1), self.get_token(2))).is_err() {
            return None;
        }
        let mut list_item: ListItem = ListItemPrefix::from(&mut self.token_queue).into();
        while let Some(inline) = self.parse_inline() {
            list_item.content.push(inline);
        }
        while let Ok(prefix) = PeekedListItemPrefix::try_from((self.get_token(0), self.get_token(1), self.get_token(2))) {
            if  prefix.indent.map_or(0, |x| x.literal.len()) > list_item.prefix.indent.as_ref().map_or(0, |x| x.literal.len()) {
                if let Some(x) = self.parse_list_item() {
                    list_item.children.push(x);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Some(list_item)

    }
    pub fn parse_paragraph(&mut self) -> Option<Paragraph> {
        todo!();
    }
    pub fn parse_section(&mut self) -> Option<Section> {
        todo!();
    }
    /*pub fn parse_line(&mut self) -> Option<Line> {

    }*/
    pub fn parse_inline(&mut self) -> Option<Inline> {
        todo!();
    }
    pub fn parse_text(&mut self) -> Option<Text> {
        let first_token = self.get_token(0)?;

        todo!();
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
        let body: Body= Parser::from(Lexer::from(s)).try_parse_body().unwrap();
        let expected = Body{
            content: vec![
                Block::Section(Section{
                    header: Header{
                        prefix: HeaderPrefix{
                            prefix: Token{line: 1, column: 1, kind: TokenKind::HeaderPrefix, literal: "#".to_string()},
                            space: Token{line: 1, column: 2, kind: TokenKind::Space, literal: " ".to_string()},
                        },
                        content: vec![
                            Inline::Text(Text{
                                content: vec![
                                    Token{line: 1, column: 3, kind: TokenKind::Text, literal: "Heading".to_string()},
                                    Token{line: 1, column: 10, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                                ],
                            }),
                        ],
                    },
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
                                        Token{line: 3, column: 11, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                                    ],
                                })
                            ]
                        }),
                        Block::BlankLine(BlankLine{
                            indent: None,
                            line_break: Token{line: 4, column: 1, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                        }),
                        Block::Section(Section{
                            header: Header{
                                prefix: HeaderPrefix {
                                    prefix: Token{line: 5, column: 1, kind: TokenKind::HeaderPrefix, literal: "##".to_string()},
                                    space: Token{line: 5, column: 3, kind: TokenKind::Space, literal: " ".to_string()},
                                },
                                content: vec![
                                    Inline::Text(Text{
                                        content: vec![
                                            Token{line: 5, column: 4, kind: TokenKind::Text, literal: "List".to_string()},
                                            Token{line: 5, column: 8, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                                        ]
                                    })
                                ]
                            },
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
                                                        Token{line: 7, column: 8, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                                                    ]
                                                })
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
                                                        Token{line: 8, column: 8, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                                                    ],
                                                }),
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
                                                                Token{line: 9, column: 17, kind: TokenKind::LineBreak, literal: "\n".to_string()},
                                                            ],
                                                        }),
                                                    ],
                                                    children: Vec::new(),
                                                },
                                            ],
                                        },
                                    ],
                                }),                 
                            ],
                        }),
                    ],
                }),
            ],
        };
        assert_eq!(body,expected);
    }

}