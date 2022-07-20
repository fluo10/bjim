use crate::{
    token::*,
    lexer::{Lexer, LexedToken},
    elements::*,
    impl_token,
};
use anyhow::Result;

use std::iter::{FromIterator, Peekable};
use std::collections::{VecDeque};
use std::convert::{From, TryFrom};


#[derive(Debug, PartialEq)]
pub struct Parser{
    tokens: VecDeque<LexedToken>,
}

impl Parser {
    pub fn build(mut self) -> Result<Body> {
        Body.try_from(&mut self.tokens)
    }
}

impl From<Lexer> for Parser {
    fn from(l: Lexer) -> Self {
        let que: VecDeque<LexedToken> = l.collect();
        Self{tokens: l.collect()}
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParsedToken {
    BackQuote(BackQuoteToken),
    Hash(HashToken),
    Hyphen(HyphenToken),
    //Plus(PlusToken),
    Tilde(TildeToken),
    //LParen,
    //RParen,
    LeftBracket(LeftBracketToken),
    RightBracket(RightBracketToken),

    // multiple char token
    Space(SpaceToken),
    Word(WordToken),

    LineBreak(LineBreakToken),

    Indent(IndentToken),
    Bullet(BulletToken),
    HeadingPrefix(HeadingPrefixToken),
    Status(StatusToken),
}

impl_token!{ ParsedToken{
    BackQuote, BackQuoteToken,
    Hash, HashToken,
    Hyphen, HyphenToken,
    Tilde, TildeToken,
    LeftBracket, LeftBracketToken,
    RightBracket, RightBracketToken,

    Space, SpaceToken,
    Word, WordToken,

    LineBreak, LineBreakToken,

    Indent, IndentToken,
    Bullet, BulletToken,
    HeadingPrefix, HeadingPrefixToken,
    Status, StatusToken,
}}
impl ParsedToken
    pub fn next_position(&self) -> Option<TokenPosition> {
        let mut position = &self.get_position()?.clone();
        Some(*position + self)
    }
}

impl TokenLike for ParsedToken {}


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