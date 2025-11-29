use crate::{
    Lexer,
    lex::{LexerError, Token, TokenKind},
};

pub mod ast {
    pub struct Statement {}

    pub enum BinaryOp {
        Minus,
        Plus,
        Star,
        Slash,
        Equal,
    }

    pub enum UnaryOp {
        Minus,
        Bang,
    }

    pub enum ExpressionTree {
        Number(u32),
        Identifier(String),
        Unary(UnaryOp, Box<ExpressionTree>),
        Binary(BinaryOp, Box<(ExpressionTree, ExpressionTree)>),
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

type ParseResult<'a> = Result<ast::ExpressionTree, ParseError>;

fn precedence(kind: TokenKind) -> u8 {
    match kind {
        // None
        TokenKind::Equal => 1,

        TokenKind::Or => 3,
        TokenKind::And => 3,

        TokenKind::EqualEqual => 5,
        TokenKind::BangEqual => 5,

        TokenKind::Greater => 7,
        TokenKind::Less => 7,
        TokenKind::LessEqual => 7,
        TokenKind::GreaterEqual => 7,

        TokenKind::Plus => 9,
        TokenKind::Minus => 9,

        TokenKind::Star => 9,
        TokenKind::Slash => 9,

        // Unary minus
        TokenKind::Bang => 11,

        // Dot and call
        TokenKind::Dot => 13,
        TokenKind::LeftParen => 13,
        TokenKind::RightParen => 13,

        _ => panic!("TokenKind doesn't have precedence"),
        // TokenKind::LeftBrace => todo!(),
        // TokenKind::RightBrace => todo!(),
        // TokenKind::Comma => todo!(),
        // TokenKind::Semicolon => todo!(),
        // TokenKind::String => todo!(),
        // TokenKind::Ident => todo!(),
        // TokenKind::Number(_) => todo!(),
        // TokenKind::Class => todo!(),
        // TokenKind::Else => todo!(),
        // TokenKind::False => todo!(),
        // TokenKind::For => todo!(),
        // TokenKind::Fun => todo!(),
        // TokenKind::If => todo!(),
        // TokenKind::Nil => todo!(),
        // TokenKind::Print => todo!(),
        // TokenKind::Return => todo!(),
        // TokenKind::Super => todo!(),
        // TokenKind::This => todo!(),
        // TokenKind::True => todo!(),
        // TokenKind::Var => todo!(),
        // TokenKind::While => todo!(),
    }
}

// TODO: Consider iterating through the colleciton of tokens
impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);
        let tokens: Result<Vec<_>, LexerError> = lexer.collect();
        // TODO: Propagate errors.
        let mut tokens = tokens.unwrap();
        // TODO: Append Eof token in Lexer, with sane line and column.
        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: "",
            lineno: 0,
            columnno: 0,
        });
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn peek(&self) -> &Token<'_> {
        self.tokens
            .get(self.current)
            .expect("Iteration past the end")
    }

    pub fn next(&mut self) {
        // Note: Doesn't advance the current token past Eof. Since Eof is mandatory
        // we can safely unwrap the get call.
        if self.tokens.get(self.current).unwrap().kind != TokenKind::Eof {
            self.current += 1;
        }
    }

    pub fn check(&self, kind: TokenKind) -> bool {
        return self.peek().kind == kind;
    }

    pub fn expect(&mut self, kind: TokenKind) -> Option<&Token<'a>> {
        if self.check(kind) {
            let tok = self.tokens.get(self.current);
            // Note: Doesn't advance the current token past Eof. Since Eof is mandatory
            // we can safely unwrap the get call.
            if self.tokens.get(self.current).unwrap().kind != TokenKind::Eof {
                self.current += 1;
            }
            return tok;
        }
        None
    }

    fn statements(&mut self) -> Vec<ast::Statement> {
        todo!("Parse statements");
    }

    fn statement(&mut self) -> ast::Statement {
        todo!("Parse statement");
    }

    pub fn expression(&mut self) -> ast::ExpressionTree {
        self.expr_bp(0)
    }

    fn expr_bp(&mut self, min_bp: u8) -> ast::ExpressionTree {
        let lhs = match self.peek().kind {
            TokenKind::Number => ast::ExpressionTree::Number(
                self.peek()
                    .lexeme
                    .parse()
                    .expect("Lexer validated number failed to parse"),
            ),
            TokenKind::Ident => ast::ExpressionTree::Identifier(self.peek().lexeme.into()),
            TokenKind::LeftParen => {
                let lhs = self.expr_bp(0);
                // TODO: Handle error.
                self.expect(TokenKind::RightParen);
                lhs
            }
            // TODO: Prefix operators.
            t => panic!("bad token: {:?}", t),
        };
        loop {
            let op = match self.peek().kind {
                TokenKind::Eof => break,
                TokenKind::Plus => ast::BinaryOp::Plus,
                t => panic!("bad token: {:?}", t),
            };
            // TODO: Handle postfix operators.

            // TODO: Make op copy.
            if let Some((l_bp, r_bp)) = Self::infix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }
                self.next();
                let rhs = self.expr_bp(r_bp);
                return ast::ExpressionTree::Binary(op, Box::new((lhs, rhs)));
            }
            break;
        }
        lhs
    }

    // fn prefix_binding_power(op: char) -> ((), u8) {
    //     match op {
    //         '+' | '-' => ((), 9),
    //         _ => panic!("bad op: {:?}", op),
    //     }
    // }
    //
    // fn postfix_binding_power(op: char) -> Option<(u8, ())> {
    //     let res = match op {
    //         '!' => (11, ()),
    //         '[' => (11, ()),
    //         _ => return None,
    //     };
    //     Some(res)
    // }

    fn infix_binding_power(op: &ast::BinaryOp) -> Option<(u8, u8)> {
        let res = match op {
            ast::BinaryOp::Equal => (2, 1),
            ast::BinaryOp::Plus | ast::BinaryOp::Minus => (5, 6),
            ast::BinaryOp::Star | ast::BinaryOp::Slash => (7, 8),
            // _ => return None,
        };
        Some(res)
    }
}
