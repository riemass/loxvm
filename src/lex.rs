use std::fmt;

#[derive(Debug, Clone)]
pub enum LexerError {
    UnterminatedQuote,
    UnexpectedChar(char),
    EmptyInput,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO Better reporting
        write!(f, "Lexer error")
    }
}

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
    // Special
    Error,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'de> {
    pub kind: TokenKind,
    pub lexeme: &'de str,
    pub lineno: usize,
    pub columnno: usize,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lexeme = self.lexeme;
        let line = self.lineno;
        let column = self.columnno;
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
            TokenKind::Error => write!(f, "ERROR <{line}, {column}> {lexeme}"),
            TokenKind::Eof => write!(f, "EOF <{line}, {column}> {lexeme}"),
        }
    }
}

pub struct Lexer<'a> {
    rest: &'a str,
    peeked: Option<Result<Token<'a>, LexerError>>,
    lineno: usize,
    columnno: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            rest: input,
            peeked: None,
            lineno: 1,
            columnno: 1,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.peeked.take() {
            return Some(next);
        }
        loop {
            let token_start = self.rest;
            let mut chars = self.rest.chars();
            let c = chars.next()?;
            let c_str = &self.rest[..c.len_utf8()];

            // Update rest.
            self.rest = chars.as_str();

            // Detect the start of a multi-character Token.
            enum Started {
                Slash,
                String,
                Number,
                Ident,
                IfEqualElse(TokenKind, TokenKind),
            }

            let mut just = |kind: TokenKind| {
                let columnno = self.columnno;
                self.columnno += c.len_utf8();
                Some(Ok(Token {
                    kind,
                    lexeme: c_str,
                    lineno: self.lineno,
                    columnno,
                }))
            };

            let started = match c {
                '(' => return just(TokenKind::LeftParen),
                ')' => return just(TokenKind::RightParen),
                '{' => return just(TokenKind::LeftBrace),
                '}' => return just(TokenKind::RightBrace),
                ',' => return just(TokenKind::Comma),
                '.' => return just(TokenKind::Dot),
                '-' => return just(TokenKind::Minus),
                '+' => return just(TokenKind::Plus),
                ';' => return just(TokenKind::Semicolon),
                '*' => return just(TokenKind::Star),
                '/' => Started::Slash,
                '<' => Started::IfEqualElse(TokenKind::LessEqual, TokenKind::Less),
                '>' => Started::IfEqualElse(TokenKind::GreaterEqual, TokenKind::Greater),
                '!' => Started::IfEqualElse(TokenKind::BangEqual, TokenKind::Bang),
                '=' => Started::IfEqualElse(TokenKind::EqualEqual, TokenKind::Equal),
                '"' => Started::String,
                '0'..='9' => Started::Number,
                'a'..='z' | 'A'..='Z' | '_' => Started::Ident,
                c if c == '\n' => {
                    self.lineno += 1;
                    self.columnno = 1;
                    continue;
                }
                c if c.is_whitespace() => {
                    self.columnno += c.len_utf8();
                    continue;
                }
                c => return Some(Err(LexerError::UnexpectedChar(c))),
            };

            break match started {
                Started::String => {
                    if let Some(end) = self.rest.find('"') {
                        let lexeme = &token_start[..end + 1 + 1];
                        self.rest = &self.rest[end + 1..];
                        let columnno = self.columnno;
                        self.columnno += lexeme.len();
                        Some(Ok(Token {
                            lexeme,
                            kind: TokenKind::String,
                            lineno: self.lineno,
                            columnno,
                        }))
                    } else {
                        // swallow the remainder of input as being a string
                        self.rest = &self.rest[self.rest.len()..];
                        return Some(Err(LexerError::UnterminatedQuote));
                    }
                }
                Started::Slash => {
                    if self.rest.starts_with('/') {
                        // This is a comment!
                        let line_end = self.rest.find('\n').unwrap_or_else(|| self.rest.len());
                        self.rest = &self.rest[line_end..];
                        self.lineno += 1;
                        self.columnno = 1;
                        continue;
                    } else {
                        let columnno = self.columnno;
                        self.columnno += 1;
                        Some(Ok(Token {
                            lexeme: c_str,
                            kind: TokenKind::Slash,
                            lineno: self.lineno,
                            columnno,
                        }))
                    }
                }
                Started::Ident => {
                    let first_non_ident = token_start
                        .find(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
                        .unwrap_or_else(|| token_start.len());

                    let lexeme = &token_start[..first_non_ident];
                    let extra_bytes = lexeme.len() - c.len_utf8();
                    self.rest = &self.rest[extra_bytes..];

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
                    let columnno = self.columnno;
                    self.columnno += lexeme.len();
                    return Some(Ok(Token {
                        lexeme,
                        kind,
                        lineno: self.lineno,
                        columnno,
                    }));
                }
                Started::Number => {
                    let first_non_digit = token_start
                        .find(|c| !matches!(c, '.' | '0'..='9'))
                        .unwrap_or_else(|| token_start.len());

                    let mut lexeme = &token_start[..first_non_digit];
                    let mut dotted = lexeme.splitn(3, '.');
                    match (dotted.next(), dotted.next(), dotted.next()) {
                        (Some(one), Some(two), Some(_)) => {
                            lexeme = &lexeme[..one.len() + 1 + two.len()];
                        }
                        (Some(one), Some(two), None) if two.is_empty() => {
                            lexeme = &lexeme[..one.len()];
                        }
                        _ => {
                            // leave literal as-is
                        }
                    }
                    let extra_bytes = lexeme.len() - c.len_utf8();
                    self.rest = &self.rest[extra_bytes..];

                    let columnno = self.columnno;
                    self.columnno += lexeme.len();
                    return Some(Ok(Token {
                        lexeme,
                        kind: TokenKind::Number,
                        lineno: self.lineno,
                        columnno,
                    }));
                }
                Started::IfEqualElse(yes, no) => {
                    self.rest = self.rest.trim_start();
                    let trimmed = token_start.len() - self.rest.len() - 1;
                    if self.rest.starts_with('=') {
                        let span = &token_start[..c.len_utf8() + trimmed + 1];
                        self.rest = &self.rest[1..];
                        let columnno = self.columnno;
                        self.columnno += 2;
                        Some(Ok(Token {
                            lexeme: span,
                            kind: yes,
                            lineno: self.lineno,
                            columnno,
                        }))
                    } else {
                        let columnno = self.columnno;
                        self.columnno += 1;
                        Some(Ok(Token {
                            lexeme: c_str,
                            kind: no,
                            lineno: self.lineno,
                            columnno,
                        }))
                    }
                }
            };
        }
    }
}
