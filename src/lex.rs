use crate::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Slash,
    Bang,
    Equal,
    String,
    Ident,
    Number,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'de> {
    pub kind: TokenKind,
    pub lexeme: &'de str,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lexeme = self.lexeme;
        let line = self.line;
        let column = self.column;
        match self.kind {
            TokenKind::LeftParen => write!(f, "LEFT_PAREN <{line}, {column}> {lexeme}"),
            TokenKind::RightParen => write!(f, "RIGHT_PAREN <{line}, {column}> {lexeme}"),
            TokenKind::LeftBrace => write!(f, "LEFT_BRACE <{line}, {column}> {lexeme}"),
            TokenKind::RightBrace => write!(f, "RIGHT_BRACE <{line}, {column}> {lexeme}"),
            TokenKind::Comma => write!(f, "COMMA <{line}, {column}> {lexeme}"),
            TokenKind::Dot => write!(f, "DOT <{line}, {column}> {lexeme}"),
            TokenKind::Minus => write!(f, "MINUS <{line}, {column}> {lexeme}"),
            TokenKind::Plus => write!(f, "PLUS <{line}, {column}> {lexeme}"),
            TokenKind::Semicolon => write!(f, "SEMICOLON <{line}, {column}> {lexeme}"),
            TokenKind::Star => write!(f, "STAR <{line}, {column}> {lexeme}"),
            TokenKind::BangEqual => write!(f, "BANG_EQUAL <{line}, {column}> {lexeme}"),
            TokenKind::EqualEqual => write!(f, "EQUAL_EQUAL <{line}, {column}> {lexeme}"),
            TokenKind::LessEqual => write!(f, "LESS_EQUAL <{line}, {column}> {lexeme}"),
            TokenKind::GreaterEqual => write!(f, "GREATER_EQUAL <{line}, {column}> {lexeme}"),
            TokenKind::Less => write!(f, "LESS <{line}, {column}> {lexeme}"),
            TokenKind::Greater => write!(f, "GREATER <{line}, {column}> {lexeme}"),
            TokenKind::Slash => write!(f, "SLASH <{line}, {column}> {lexeme}"),
            TokenKind::Bang => write!(f, "BANG <{line}, {column}> {lexeme}"),
            TokenKind::Equal => write!(f, "EQUAL <{line}, {column}> {lexeme}"),
            TokenKind::String => write!(f, "STRING <{line}, {column}> {lexeme}"),
            TokenKind::Ident => write!(f, "IDENTIFIER <{line}, {column}> {lexeme}"),
            TokenKind::Number => write!(f, "NUMBER <{line}, {column}> {lexeme}"),
            TokenKind::And => write!(f, "AND <{line}, {column}> {lexeme}"),
            TokenKind::Class => write!(f, "CLASS <{line}, {column}> {lexeme}"),
            TokenKind::Else => write!(f, "ELSE <{line}, {column}> {lexeme}"),
            TokenKind::False => write!(f, "FALSE <{line}, {column}> {lexeme}"),
            TokenKind::For => write!(f, "FOR <{line}, {column}> {lexeme}"),
            TokenKind::Fun => write!(f, "FUN <{line}, {column}> {lexeme}"),
            TokenKind::If => write!(f, "IF <{line}, {column}> {lexeme}"),
            TokenKind::Nil => write!(f, "NIL <{line}, {column}> {lexeme}"),
            TokenKind::Or => write!(f, "OR <{line}, {column}> {lexeme}"),
            TokenKind::Print => write!(f, "PRINT <{line}, {column}> {lexeme}"),
            TokenKind::Return => write!(f, "RETURN <{line}, {column}> {lexeme}"),
            TokenKind::Super => write!(f, "SUPER <{line}, {column}> {lexeme}"),
            TokenKind::This => write!(f, "THIS <{line}, {column}> {lexeme}"),
            TokenKind::True => write!(f, "TRUE <{line}, {column}> {lexeme}"),
            TokenKind::Var => write!(f, "VAR <{line}, {column}> {lexeme}"),
            TokenKind::While => write!(f, "WHILE <{line}, {column}> {lexeme}"),
            TokenKind::Eof => write!(f, "EOF <{line}, {column}> {lexeme}"),
        }
    }
}

pub struct Lexer<'a> {
    rest: &'a str,
    peeked: Option<Result<Token<'a>, Error>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            rest: input,
            peeked: None,
            line: 1,
            column: 1,
        }
    }

    // Emits the specified token and updates the lexer state.
    fn emit_token(
        &mut self,
        token_start: &'a str,
        rest: &'a str,
        kind: TokenKind,
    ) -> Option<Result<Token<'a>, Error>> {
        self.rest = rest;
        let consumed_len = token_start.len() - self.rest.len();
        let column = self.column;
        self.column += consumed_len;
        let lexeme = &token_start[..consumed_len];
        Some(Ok(Token {
            kind,
            lexeme,
            line: self.line,
            column,
        }))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.peeked.take() {
            return Some(next);
        }
        loop {
            let token_start = self.rest;
            let mut chars = self.rest.chars();

            // Signal if started reading a multi-character token.
            enum Started {
                Slash,
                String,
                Number,
                Ident,
            }

            let next_char = chars.next()?;
            let started = match next_char {
                '(' => return self.emit_token(token_start, chars.as_str(), TokenKind::LeftParen),
                ')' => return self.emit_token(token_start, chars.as_str(), TokenKind::RightParen),
                '{' => return self.emit_token(token_start, chars.as_str(), TokenKind::LeftBrace),
                '}' => return self.emit_token(token_start, chars.as_str(), TokenKind::RightBrace),
                ',' => return self.emit_token(token_start, chars.as_str(), TokenKind::Comma),
                '.' => return self.emit_token(token_start, chars.as_str(), TokenKind::Dot),
                '-' => return self.emit_token(token_start, chars.as_str(), TokenKind::Minus),
                '+' => return self.emit_token(token_start, chars.as_str(), TokenKind::Plus),
                ';' => return self.emit_token(token_start, chars.as_str(), TokenKind::Semicolon),
                '*' => return self.emit_token(token_start, chars.as_str(), TokenKind::Star),
                '/' => Started::Slash,
                '"' => Started::String,
                '<' => {
                    if chars.as_str().starts_with('=') {
                        chars.next();
                        return self.emit_token(token_start, chars.as_str(), TokenKind::LessEqual);
                    } else {
                        return self.emit_token(token_start, chars.as_str(), TokenKind::Less);
                    }
                }
                '>' => {
                    if chars.as_str().starts_with('=') {
                        chars.next();
                        return self.emit_token(
                            token_start,
                            chars.as_str(),
                            TokenKind::GreaterEqual,
                        );
                    } else {
                        return self.emit_token(token_start, chars.as_str(), TokenKind::Greater);
                    }
                }
                '!' => {
                    if chars.as_str().starts_with('=') {
                        chars.next();
                        return self.emit_token(token_start, chars.as_str(), TokenKind::BangEqual);
                    } else {
                        return self.emit_token(token_start, chars.as_str(), TokenKind::Bang);
                    }
                }
                '=' => {
                    if chars.as_str().starts_with('=') {
                        chars.next();
                        return self.emit_token(token_start, chars.as_str(), TokenKind::EqualEqual);
                    } else {
                        return self.emit_token(token_start, chars.as_str(), TokenKind::Equal);
                    }
                }
                '0'..='9' => Started::Number,
                'a'..='z' | 'A'..='Z' | '_' => Started::Ident,
                c if c == '\n' => {
                    self.line += 1;
                    self.column = 1;
                    self.rest = chars.as_str();
                    continue;
                }
                c if c.is_whitespace() => {
                    self.column += c.len_utf8();
                    self.rest = chars.as_str();
                    continue;
                }
                c => return Some(Err(Error::unexpected_char(c, self.line, self.column))),
            };

            match started {
                Started::String => {
                    if let Some(_) = chars.find(|&c| c == '"') {
                        return self.emit_token(token_start, chars.as_str(), TokenKind::String);
                    } else {
                        return Some(Err(Error::unterminated_string(self.line, self.column)));
                    }
                }
                Started::Slash => {
                    if chars.as_str().starts_with('/') {
                        // This is a comment!
                        let _line_end = chars.find(|&c| c == '\n');
                        self.rest = chars.as_str();
                        self.line += 1;
                        self.column = 1;
                        continue;
                    } else {
                        return self.emit_token(token_start, chars.as_str(), TokenKind::Slash);
                    }
                }
                Started::Ident => {
                    let first_non_ident = token_start
                        .find(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
                        .unwrap_or_else(|| token_start.len());
                    let lexeme = &token_start[..first_non_ident];
                    let kind = match lexeme {
                        "and" => TokenKind::And,
                        "class" => TokenKind::Class,
                        "else" => TokenKind::Else,
                        "false" => TokenKind::False,
                        "for" => TokenKind::For,
                        "fun" => TokenKind::Fun,
                        "if" => TokenKind::If,
                        "nil" => TokenKind::Nil,
                        "or" => TokenKind::Or,
                        "print" => TokenKind::Print,
                        "return" => TokenKind::Return,
                        "super" => TokenKind::Super,
                        "this" => TokenKind::This,
                        "true" => TokenKind::True,
                        "var" => TokenKind::Var,
                        "while" => TokenKind::While,
                        _ => TokenKind::Ident,
                    };
                    let column = self.column;
                    self.column += lexeme.len();
                    self.rest = &token_start[first_non_ident..];
                    return Some(Ok(Token {
                        lexeme,
                        kind,
                        line: self.line,
                        column,
                    }));
                }
                Started::Number => {
                    let first_non_digit = token_start
                        .find(|c| !matches!(c, '.' | '0'..='9'))
                        .unwrap_or_else(|| token_start.len());
                    return self.emit_token(
                        token_start,
                        &token_start[first_non_digit..],
                        TokenKind::Number,
                    );
                }
            };
        }
    }
}

#[test]
fn test_lexer() {
    let test_input = include_str!("../programs/binary_trees.lox");
    let test_output = include_str!("../programs/binary_trees.tokens.txt");
    let mut expected_lins = test_output.lines();

    let lexer = Lexer::new(test_input);
    let tokens: Result<Vec<_>, Error> = lexer.collect();
    assert!(tokens.is_ok());
    for token in tokens.unwrap() {
        assert_eq!(Some(token.to_string().as_str()), expected_lins.next());
    }
}
