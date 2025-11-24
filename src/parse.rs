use crate::{
    Lexer,
    lex::{LexerError, Token, TokenKind},
};

pub enum Op {
    Minus,
    Plus,
    Star,
    Slash,
    Equal,
}

pub enum TokenTree<'a> {
    Atom(&'a str),
    Cons(Op, Vec<TokenTree<'a>>),
}

#[derive(Debug, Clone)]
pub struct ParseError {}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

type ParseResult<'a> = Result<TokenTree<'a>, ParseError>;

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);
        let tokens: Result<Vec<_>, LexerError> = lexer.collect();
        Self {
            tokens: tokens.unwrap(),
            current: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token<'_>> {
        self.tokens.get(self.current)
    }

    pub fn expect(&mut self, kind: TokenKind) -> Option<&Token<'_>> {
        if let Some(token) = self.tokens.get(self.current) {
            if token.kind == kind {
                self.current += 1;
                return Some(token);
            }
        }
        None
    }

    fn parse_statements(&self) {
        while let Some(statement) = self.parse_statement(self) {}
    }

    fn parse_statement(&self) {}
}
