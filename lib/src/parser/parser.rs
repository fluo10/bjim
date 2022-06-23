use super::Token;
use super::TokenKind;
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