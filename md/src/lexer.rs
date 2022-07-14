use crate::token::{RawToken, TokenPosition};

use std::convert::From;
use std::iter::Peekable;
use std::str::Chars;

use anyhow::{anyhow,bail,Error,Result};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    buf: String,
    kind: Option<TokenKind>,
    prev_kind: Option<TokenKind>,
    position: TokenPosition,
    read_column: usize,   
}

impl<'a> Lexer<'a> {

    fn tokenize(&mut self) -> Option<Token>{
        let kind = self.kind?;

        let token = Token{ 
            line: self.line,
            column: self.column,
            literal: self.buf.drain(..).collect(),
            kind: kind,
        };

        if kind == TokenKind::LineBreak {
            self.line += 1;
            self.column = 1;
            self.read_column = 1;
        } else {
            self.column = self.read_column;
        };
        self.prev_kind.insert(kind);

        Some(token)
    }

    fn read_char(&mut self) {
        self.buf.push(self.chars.next().unwrap());
        self.read_column += 1;
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn peek_char_eq(&mut self, c: &char) -> bool {
        if let Some(x) = self.peek_char() {
            x == c
        } else {
            false
        }
    }

    fn try_read_header_prefix(&mut self) -> Option<TokenKind> {
        while let Some(x) =  self.peek_char() {
            match x {
                &' ' => return Some(TokenKind::HeaderPrefix),
                &'#' => self.read_char(),
                _ => {
                    if self.read_column == 1 {
                        return self.try_read_hashtag();
                    } else {
                        break;
                    }
                }
            }
        }
        None
    }

    /*
    fn try_read_code_block_fence(&mut self) -> Option<TokenKind> {
        todo!();
    }
    */

    fn try_read_indent(&mut self) -> Option<TokenKind> {
        while let Some(x) = self.peek_char() {
            if x == &' ' {
                self.read_char();
            } else {
                return Some(TokenKind::Indent);
            }
        }
        None
    }

    fn try_read_bullet(&mut self) -> Option<TokenKind> {
        if let Some(x) = self.peek_char() {
            if x == &' ' {
                return Some(TokenKind::Bullet);
            }
        }
        None
    }

    fn try_read_hashtag(&mut self) -> Option<TokenKind> {
        while let Some(x) = self.peek_char() {
            if x == &' ' {
                return Some(TokenKind::HashTag);
            } {
                self.read_char();
            }
        }
        None
    }

    fn try_read_space(&mut self) -> Option<TokenKind> {
        while self.peek_char_eq(&' ') {
            self.read_char();
        };
        Some(TokenKind::Space)
    }
    
    fn read_text(&mut self) {
        while let Some(x) =  self.peek_char() {
            match x {
                &' ' | &'\n' => break,
                _ => self.read_char()
            }
        }
        self.kind.insert(TokenKind::Text);
    }


}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(s: &'a str) -> Lexer<'a> {
        Lexer {
            chars: s.chars().peekable(),
            buf: String::new(),
            line: 1,
            column: 1,
            read_column: 1,
            kind: None,
            prev_kind: None,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next (&mut self) ->  Option<Token> {
        let ch = *self.peek_char()?;
        self.read_char();
        let line = self.line;
        let column=self.column;
        let is_after_indent = match self.prev_kind {
            Some(x) => x == TokenKind::Indent,
            None => false
        };
        self.kind = match (column, is_after_indent, ch) {
            //(1, true, _) => panic!(),
            (_, _, '\n') => Some(TokenKind::LineBreak),
            (1, false, '#') => self.try_read_header_prefix(),
            (1, false, ' ') => self.try_read_indent(),
            //(1, false, '`') => self.try_read_code_block_fence(),
            (1, false, x) | (_, true, x) => {
                match x {
                    '-' | '*' | '+' => self.try_read_bullet(),
                    _ => None
                }
            },

            (_, _, '[') => Some(TokenKind::LBracket),
            (_, _, ']') => Some(TokenKind::RBracket),
            (_, _, ' ') => self.try_read_space(),
            (_, _, '#') => self.try_read_hashtag(),
            (_, _, _) => None,
        };
        if self.kind.is_none() {
            self.read_text();
        }

        self.tokenize()

    }

}

#[derive(Clone, Debug, PartialEq)]
pub enum LexedToken {

    // Single char token
    //Asterisk(AsteriskToken),
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

}

impl LexedToken {
    pub fn is_back_quote(&self) -> bool {   
        match self {
            Self::BackQuote(_) => true,
            _ => false
        }
    }
    pub fn is_hash(&self) -> bool {
        match self {
            Self::Hash(_) => true,
            _ => false
        }
    }
    pub fn is_hyphen(&self) -> bool {
        match self {
            Self::Hyphen(_) => true,
            _ => false
        }
    }
    pub fn is_tilde(&self) -> bool {
        match self {
            Self::Tilde(_) => true,
            _ => false
        }
    }
    pub fn is_left_bracket(&self) -> bool {
        match self {
            Self::LeftBracket(_) => true,
            _ => false
        }
    }
    pub fn is_right_bracket(&self) -> bool {
        match self {
            Self::RightBracket(_) => true,
            _ => false
        }
    }
    pub fn is_space(&self) -> bool {
        match self {
            Self::Space(_) => true,
            _ => false
        }
    }
    pub fn is_word(&self) -> bool {
        match self {
            Self::Word(_) => true,
            _ => false
        }
    }
    pub fn is_line_break(&self) -> bool {
        match self {
            Self::LineBreak(_) => true,
            _ => false
        }
    }
}

impl TokenLike for LexedToken {
    fn len(&self) -> usize {
        match self {
            Self::BackQuote(x) => x.len(),
            Self::Hash(x) => x.len(),
            Self::Hyphen(x) => x.len(),
            Self::Tilde(x) => x.len(),
            Self::LeftBracket(x) => x.len(),
            Self::RightBracket(x) => x.len(),
            Self::Space(x) => x.len(),
            Self::Word(x) => x.len(),
            Self::LineBreak(x) => x.len(),
        }
    }
}

impl AsRef<TokenContent> for LexedToken {
    fn as_ref(&self) -> &TokenContent {
        match self {
            Self::BackQuote(x) => x.as_ref(),
            Self::Hash(x) => x.as_ref(),
            Self::Hyphen(x) => x.as_ref(),
            Self::Tilde(x) => x.as_ref(),
            Self::LeftBracket(x) => x.as_ref(),
            Self::RightBracket(x) => x.as_ref(),
            Self::Space(x) => x.as_ref(),
            Self::Word(x) => x.as_ref(),
            Self::LineBreak(x) => x.as_ref(),
        }
    }
}

impl AsMut<TokenContent> for LexedToken {
    fn as_mut(&mut self) -> &mut TokenContent {
        use LexedToken::*;
        match self {
            BackQuote(x) => x.as_mut(),
            Hash(x) => x.as_mut(),
            Hyphen(x) => x.as_mut(),
            Tilde(x) => x.as_mut(),
            LeftBracket(x) => x.as_mut(),
            RightBracket(x) => x.as_mut(),
            Space(x) => x.as_mut(),
            Word(x) => x.as_mut(),
            LineBreak(x) => x.as_mut(),
        }
    }
}

impl fmt::Display for LexedToken{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LexedToken::*;
        match self {
            BackQuote(x) => x.fmt(f),
            Hash(x) => x.fmt(f),
            Hyphen(x) => x.fmt(f),
            Tilde(x) => x.fmt(f),
            LeftBracket(x) => x.fmt(f),
            RightBracket(x) => x.fmt(f),
            Space(x) => x.fmt(f),
            Word(x) => x.fmt(f),
            LineBreak(x) => x.fmt(f),
        }
    }
}

impl From<BackQuoteToken> for LexedToken {
    fn from(t: BackQuoteToken) -> Self {
        Self::BackQuote(t)
    }
}

impl From<HashToken> for LexedToken {
    fn from(t: HashToken) -> Self {
        Self::Hash(t)
    }
}

impl From<HyphenToken> for LexedToken {
    fn from(t: HyphenToken) -> Self {
        Self::Hyphen(t)
    }
}

impl From<TildeToken> for LexedToken {
    fn from(t: TildeToken) -> Self {
        Self::Tilde(t)
    }
}

impl From<LeftBracketToken> for LexedToken {
    fn from(t: LeftBracketToken) -> Self {
        Self::LeftBracket(t)
    }
}

impl From<RightBracketToken> for LexedToken {
    fn from(t: RightBracketToken) -> Self {
        Self::RightBracket(t)
    }
}

impl From<SpaceToken> for LexedToken {
    fn from(t: SpaceToken) -> Self {
        Self::Space(t)
    }
}

impl From<WordToken> for LexedToken {
    fn from(t: WordToken) -> Self {
        Self::Word(t)
    }
}

impl From<LineBreakToken> for LexedToken {
    fn from(t: LineBreakToken) -> Self {
        Self::LineBreak(t)
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
            Token{line: 1, column: 1, kind: TokenKind::HeaderPrefix, literal: "#".to_string()},
            Token{line: 1, column: 2, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 1, column: 3, kind: TokenKind::Text, literal: "Heading".to_string()},
            Token{line: 1, column: 10, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 2, column: 1, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 3, column: 1, kind: TokenKind::Text, literal: "Paragraph.".to_string()},
            Token{line: 3, column: 11, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 4, column: 1, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 5, column: 1, kind: TokenKind::HeaderPrefix, literal: "##".to_string()},
            Token{line: 5, column: 3, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 5, column: 4, kind: TokenKind::Text, literal: "List".to_string()},
            Token{line: 5, column: 8, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 6, column: 1, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 7, column: 1, kind: TokenKind::Bullet, literal: "-".to_string()},
            Token{line: 7, column: 2, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 7, column: 3, kind: TokenKind::Text, literal: "Note1".to_string()},
            Token{line: 7, column: 8, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 8, column: 1, kind: TokenKind::Bullet, literal: "-".to_string()},
            Token{line: 8, column: 2, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 8, column: 3, kind: TokenKind::Text, literal: "Note2".to_string()},
            Token{line: 8, column: 8, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 9, column: 1, kind: TokenKind::Indent, literal: "    ".to_string()},
            Token{line: 9, column: 5, kind: TokenKind::Bullet, literal: "-".to_string()},
            Token{line: 9, column: 6, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 9, column: 7, kind: TokenKind::Text, literal: "Child".to_string()},
            Token{line: 9, column: 12, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 9, column: 13, kind: TokenKind::Text, literal: "note".to_string()},
            Token{line: 9, column: 17, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 10, column: 1, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 11, column: 1, kind: TokenKind::HeaderPrefix, literal: "##".to_string()},
            Token{line: 11, column: 3, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 11, column: 4, kind: TokenKind::Text, literal: "Check".to_string()},
            Token{line: 11, column: 9, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 11, column: 10, kind: TokenKind::Text, literal: "list".to_string()},
            Token{line: 11, column: 14, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 12, column: 1, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 13, column: 1, kind: TokenKind::Bullet, literal: "-".to_string()},
            Token{line: 13, column: 2, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 13, column: 3, kind: TokenKind::LBracket, literal: "[".to_string()},
            Token{line: 13, column: 4, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 13, column: 5, kind: TokenKind::RBracket, literal: "]".to_string()},
            Token{line: 13, column: 6, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 13, column: 7, kind: TokenKind::Text, literal: "Task1".to_string()},
            Token{line: 13, column: 12, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 14, column: 1, kind: TokenKind::Bullet, literal: "-".to_string()},
            Token{line: 14, column: 2, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 14, column: 3, kind: TokenKind::LBracket, literal: "[".to_string()},
            Token{line: 14, column: 4, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 14, column: 5, kind: TokenKind::RBracket, literal: "]".to_string()},
            Token{line: 14, column: 6, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 14, column: 7, kind: TokenKind::Text, literal: "Task2".to_string()},
            Token{line: 14, column: 12, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 15, column: 1, kind: TokenKind::Indent, literal: "    ".to_string()},
            Token{line: 15, column: 5, kind: TokenKind::Bullet, literal: "-".to_string()},
            Token{line: 15, column: 6, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 15, column: 7, kind: TokenKind::LBracket, literal: "[".to_string()},
            Token{line: 15, column: 8, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 15, column: 9, kind: TokenKind::RBracket, literal: "]".to_string()},
            Token{line: 15, column: 10, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 15, column: 11, kind: TokenKind::Text, literal: "Child".to_string()},
            Token{line: 15, column: 16, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 15, column: 17, kind: TokenKind::Text, literal: "task".to_string()},
            Token{line: 15, column: 21, kind: TokenKind::LineBreak, literal: "\n".to_string()},
            Token{line: 16, column: 1, kind: TokenKind::Indent, literal: "    ".to_string()},
            Token{line: 16, column: 5, kind: TokenKind::Bullet, literal: "-".to_string()},
            Token{line: 16, column: 6, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 16, column: 7, kind: TokenKind::Text, literal: "Child".to_string()},
            Token{line: 16, column: 12, kind: TokenKind::Space, literal: " ".to_string()},
            Token{line: 16, column: 13, kind: TokenKind::Text, literal: "note".to_string()},
            Token{line: 16, column: 17, kind: TokenKind::LineBreak, literal: "\n".to_string()},
        ];
        let t: Vec<Token> = Lexer::from(s).collect();
        assert_eq!(t, v);
        
    }
}