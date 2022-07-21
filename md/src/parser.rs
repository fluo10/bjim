use crate::errors::ParseError;
use crate::{
    token::*,
    lexer::Lexer,
    elements::*,
    impl_token,
};

use std::iter::{FromIterator, Peekable};
use std::collections::{VecDeque};
use std::convert::{From, TryFrom};
use std::fmt;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub struct Parser{
    tokens: VecDeque<LexedToken>,
}

impl Parser {
    pub fn build(mut self) -> Result<Body> {
        Body::try_from(&mut self.tokens)
    }
}

impl From<Lexer> for Parser {
    fn from(l: Lexer) -> Self {
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
        let lexed = Lexer::from(s);
        let found = Parser::from(lexed).build().unwrap();
        let expected = Body::from(
            Section::from((
                HeadingElement::from((
                    HeadingPrefixToken::try_from((1, 1, "#")).unwrap(), 
                    SpaceToken::try_from((1,  2, " ")).unwrap().into(),
                    vec![
                        TextElement::from(vec![
                            WordToken::try_from(( 1,  3, "Heading")).unwrap().into(),
                        ]).into(),
                        LineBreakElement::from(LineBreakToken::try_from(( 1, 10, "\n")).unwrap()).into(),
                    ]
                )),
                vec![
                    BlankLineElement::from(LineBreakToken::try_from(( 2,  1, "\n")).unwrap()).into(),
                    ParagraphElement::from(
                        vec![
                            TextElement::from(vec![
                                WordToken::try_from(( 3,  1, "Paragraph.")).unwrap().into(),
                            ]).into(),
                            LineBreakElement::from(
                                LineBreakToken::try_from(( 3, 11, "\n")).unwrap()
                            ).into(),
                        ]
                    ).into(),
                    BlankLineElement::from(LineBreakToken::try_from(( 4,  1, "\n")).unwrap()).into(),
                ],
                vec![
                    Section::from((
                        HeadingElement::from((
                            HeadingPrefixToken::try_from(( 5,  1, "##")).unwrap(),
                            SpaceToken::try_from(( 5,  3, " ")).unwrap(),
                            vec![
                                TextElement::from(vec![
                                    WordToken::try_from(( 5,  4, "List")).unwrap().into(),
                                ]).into(),
                                LineBreakElement::from(LineBreakToken::try_from(( 5,  8, "\n")).unwrap()).into(),
                            ]
                        )),
                        vec![
                            BlankLineElement::from(
                                LineBreakToken::try_from(( 6,  1, "\n")).unwrap()
                            ).into(),
                            ListElement::from(vec![
                                ListNoteElement::from((
                                    (
                                        HyphenToken::try_from(( 7,  1, "-")).unwrap(),
                                        SpaceToken::try_from(( 7,  2, " ")).unwrap(),
                                    ),
                                    vec![
                                        TextElement::from(vec![
                                            WordToken::try_from(( 7,  3, "Note1")).unwrap().into(),
                                        ]).into(),
                                        LineBreakElement::from(
                                            LineBreakToken::try_from(( 7,  8, "\n")).unwrap()
                                        ).into(),
                                    ],
                                )).into(),
                                ListNoteElement::from((
                                    (
                                        HyphenToken::try_from(( 8,  1, "-")).unwrap(),
                                        SpaceToken::try_from(( 8,  2, " ")).unwrap(),
                                    ),
                                    vec![
                                        TextElement::from(vec![
                                            WordToken::try_from(( 8,  3, "Note2")).unwrap().into(),
                                        ]).into(),
                                        LineBreakElement::from(
                                            LineBreakToken::try_from(( 8,  8, "\n")).unwrap(),
                                        ).into(),
                                    ],
                                )).into(),
                                ListNoteElement::from((
                                    (
                                        HyphenToken::try_from(( 8,  1, "-")).unwrap(),
                                        SpaceToken::try_from(( 8,  2, " ")).unwrap(),
                                    ),
                                    vec![
                                        TextElement::from(vec![
                                            WordToken::try_from(( 8,  3, "Note2")).unwrap().into(),
                                        ]).into(),
                                        LineBreakElement::from(
                                            LineBreakToken::try_from(( 8,  8, "\n")).unwrap(),
                                        ).into(),
                                    ],
                                )).into(),
                                ListNoteElement::from((
                                    (
                                        SpaceToken::try_from(( 9,  1, "    ")).unwrap(),
                                        HyphenToken::try_from(( 9,  5, "-")).unwrap(),
                                        SpaceToken::try_from(( 9,  6, " ")).unwrap(),
                                    ),
                                    vec![
                                        TextElement::from(vec![
                                            WordToken::try_from(( 9,  7, "Child")).unwrap().into(),
                                            SpaceToken::try_from(( 9, 12, " ")).unwrap().into(),
                                            WordToken::try_from(( 9, 13, "note")).unwrap().into(),
                                        ]).into(),
                                        LineBreakElement::from(
                                            LineBreakToken::try_from(( 9, 17, "\n")).unwrap(),
                                        ).into(),
                                    ],
                                )).into(),

                            ]).into(),
                            BlankLineElement::from(
                                LineBreakToken::try_from((10,  1, "\n")).unwrap(),
                            ).into(),
                        ]
                    )),
                    Section::from((
                        HeadingElement::from((
                            HeadingPrefixToken::try_from((11,  1, "##")).unwrap().into(),
                            SpaceToken::try_from((11,  3, " ")).unwrap().into(),
                            vec![
                                TextElement::from(vec![
                                    WordToken::try_from((11,  4, "Check")).unwrap().into(),
                                    SpaceToken::try_from((11,  9, " ")).unwrap().into(),
                                    WordToken::try_from((11, 10, "list")).unwrap().into(),
                                ]).into(),
                                LineBreakElement::from(
                                    LineBreakToken::try_from((11, 14, "\n")).unwrap()
                                ).into(),
                            ],
                        )),
                        vec![
                            BlankLineElement::from(
                                LineBreakToken::try_from((12,  1, "\n")).unwrap()
                            ).into(),
                            ListElement::from(vec![
                                ListTaskElement::from((
                                    (
                                        HyphenToken::try_from((13,  1, "-")).unwrap(),
                                        SpaceToken::try_from((13,  2, " ")).unwrap(),
                                    ),
                                    (
                                        LeftBracketToken::try_from((13,  3, "[")).unwrap(),
                                        StatusToken::try_from((13,  4, " ")).unwrap(),
                                        RightBracketToken::try_from((13,  5, "]")).unwrap(),
                                        SpaceToken::try_from((13,  6, " ")).unwrap(),
                                    ),
                                    vec![
                                        TextElement::from(vec![
                                            WordToken::try_from((13,  7, "Task1")).unwrap().into(),
                                        ]).into(),
                                        LineBreakElement::from(
                                            LineBreakToken::try_from((13, 12, "\n")).unwrap()
                                        ).into(),
                                    ]
                                )).into(),
                                ListTaskElement::from((
                                    (
                                        HyphenToken::try_from((14,  1, "-")).unwrap(),
                                        SpaceToken::try_from((14,  2, " ")).unwrap(),
                                    ),
                                    (
                                        LeftBracketToken::try_from((14,  3, "[")).unwrap(),
                                        StatusToken::try_from((14,  4, " ")).unwrap(),
                                        RightBracketToken::try_from((14,  5, "]")).unwrap(),
                                        SpaceToken::try_from((14,  6, " ")).unwrap(),
                                    ),
                                    vec![
                                        TextElement::from(vec![
                                            WordToken::try_from((14,  7, "Task2")).unwrap().into(),
                                        ]).into(),
                                        LineBreakElement::from(
                                            LineBreakToken::try_from((14, 12, "\n")).unwrap(),
                                        ).into(),
                                    ],
                                    vec![
                                        ListTaskElement::from((
                                            (
                                                SpaceToken::try_from((15,  1, "    ")).unwrap(),
                                                HyphenToken::try_from((15,  5, "-")).unwrap(),
                                                SpaceToken::try_from((15,  6, " ")).unwrap(),
                                            ),
                                            (
                                                LeftBracketToken::try_from((15,  7, "[")).unwrap(),
                                                StatusToken::try_from((15,  8, " ")).unwrap(),
                                                RightBracketToken::try_from((15,  9, "]")).unwrap(),
                                                SpaceToken::try_from((15, 10, " ")).unwrap(),
                                            ),
                                            vec![
                                                TextElement::from(vec![
                                                    WordToken::try_from((15, 11, "Child")).unwrap().into(),                                                    
                                                    SpaceToken::try_from((15, 16, " ")).unwrap().into(),
                                                    WordToken::try_from((15, 17, "task")).unwrap().into(),
                                                ]).into(),
                                                LineBreakElement::from(
                                                    LineBreakToken::try_from((15, 21, "\n")).unwrap(),
                                                ).into(),
                                            ]

                                        )).into(),
                                        ListNoteElement::from((
                                            (
                                                SpaceToken::try_from((16,  1, "    ")).unwrap(),
                                                HyphenToken::try_from((16,  5, "-")).unwrap(),
                                                SpaceToken::try_from((16,  6, " ")).unwrap(),
                                            ),
                                            vec![
                                                TextElement::from(vec![
                                                    WordToken::try_from((16,  7, "Child")).unwrap().into(),
                                                    SpaceToken::try_from((16, 12, " ")).unwrap().into(),
                                                    WordToken::try_from((16, 13, "note")).unwrap().into(),
                                                ]).into(),
                                                LineBreakElement::from(
                                                    LineBreakToken::try_from((16, 17, "\n")).unwrap(),
                                                ).into(),
                                            ]
                                        )).into()
                                    ]
                                )).into(),
                            ]).into(),
                        ]
                    ))
                ]
            ))
        );
        let t: Vec<LexedToken> = Lexer::from(s).collect();
        assert_eq!(found,expected);
    }

}