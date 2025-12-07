use crate::{
    Lexer, ast,
    error::Error,
    lex::{Token, TokenKind},
};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

type ParseResult<'a> = Result<ast::ExpressionTree, Error>;

fn infix_binding_power(kind: TokenKind) -> Option<(u8, u8)> {
    match kind {
        TokenKind::Equal => Some((2, 1)),
        TokenKind::Or | TokenKind::And => Some((3, 4)),
        TokenKind::EqualEqual | TokenKind::BangEqual => Some((5, 6)),
        TokenKind::Greater | TokenKind::Less | TokenKind::LessEqual | TokenKind::GreaterEqual => {
            Some((7, 8))
        }
        TokenKind::Plus | TokenKind::Minus => Some((9, 10)),
        TokenKind::Star | TokenKind::Slash => Some((11, 12)),
        _ => None,
    }
}

fn prefix_binding_power(kind: TokenKind) -> u8 {
    match kind {
        TokenKind::Plus | TokenKind::Minus => 51,
        _ => panic!("Unexpected token as prefix operator: {:?}", kind),
    }
}

// TODO: Consider iterating through the colleciton of tokens
impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);
        let tokens: Result<Vec<_>, crate::error::Error> = lexer.collect();
        // TODO: Propagate errors.
        let mut tokens = tokens.unwrap();
        // TODO: Append Eof token in Lexer, with sane line and column.
        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: "",
            line: 0,
            column: 0,
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

    pub fn advance(&mut self) {
        // Note: Doesn't advance the current token past Eof. Since Eof is mandatory
        // we can safely unwrap the get call.
        if self.tokens.get(self.current).unwrap().kind != TokenKind::Eof {
            self.current += 1;
        }
    }

    pub fn next(&mut self) -> &Token<'_> {
        self.advance();
        self.tokens
            .get(self.current - 1)
            .expect("Iteration past the end")
    }

    pub fn check(&self, kind: TokenKind) -> bool {
        return self.peek().kind == kind;
    }

    pub fn expect(&mut self, kind: TokenKind) -> Option<&Token<'a>> {
        if self.check(kind) {
            let tok = self.tokens.get(self.current);
            // Note: Doesn't advance the current token past Eof. Since Eof is mandatory
            // we can safely unwrap the get call.
            if tok.unwrap().kind != TokenKind::Eof {
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

    pub fn expression(&mut self) -> Result<ast::ExpressionTree, Error> {
        self.expr_bp(0)
    }

    fn expr_bp(&mut self, min_bp: u8) -> Result<ast::ExpressionTree, Error> {
        let mut lhs = match self.peek().kind {
            TokenKind::Number => {
                let val: f64 = self.next().lexeme.parse().map_err(|_| {
                    Error::ParserError("floating point number failed to parse".into())
                })?;
                ast::ExpressionTree::Number(val)
            }
            TokenKind::Ident => ast::ExpressionTree::Identifier(self.next().lexeme.into()),
            TokenKind::Plus | TokenKind::Minus => {
                let tok = self.next();
                let bp = prefix_binding_power(tok.kind);
                ast::ExpressionTree::Unary(tok.lexeme.into(), Box::new(self.expr_bp(bp)?))
            }
            TokenKind::LeftParen => {
                self.advance();
                let lhs = self.expr_bp(0)?;
                if let None = self.expect(TokenKind::RightParen) {
                    return Err(Error::ParserError(
                        "expected RightParen in expression".into(),
                    ));
                }
                lhs
            }
            t => {
                return Err(Error::ParserError(format!("unexpected token - {:?}", t)));
            }
        };
        loop {
            let op = match self.peek().kind {
                TokenKind::Eof => break,
                TokenKind::Plus => TokenKind::Plus,
                TokenKind::Minus => TokenKind::Minus,
                TokenKind::Star => TokenKind::Star,
                TokenKind::Slash => TokenKind::Slash,
                // Can follow expression, not an error but also not an operator.
                TokenKind::RightParen => TokenKind::RightParen,
                // All other tokes - error.
                t => {
                    return Err(Error::ParserError(format!("unexpected token - {:?}", t)));
                }
            };
            if let Some((l_bp, r_bp)) = infix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }
                // TODO: refactor operator to string properly.
                let s = self.peek().lexeme.to_owned();
                self.advance();
                let rhs = self.expr_bp(r_bp)?;
                lhs = ast::ExpressionTree::Binary(s, Box::new((lhs, rhs)));
                continue;
            }
            break;
        }
        Ok(lhs)
    }
}

#[test]
fn tests() {
    fn parse_expr(s: &str) -> ast::ExpressionTree {
        let mut parser = Parser::new(s);
        parser.expression().expect("failed to parse input")
    }

    let s = parse_expr("1");
    assert_eq!(s.to_string(), "1");

    let s = parse_expr("1 + 2 * 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))");

    let s = parse_expr("a + Bar * cloc * de12 + ebin");
    assert_eq!(s.to_string(), "(+ (+ a (* (* Bar cloc) de12)) ebin)");

    let s = parse_expr("--1 * 2");
    assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

    let s = parse_expr("+--foo");
    assert_eq!(s.to_string(), "(+ (- (- foo)))");

    let s = parse_expr("(((0)))");
    assert_eq!(s.to_string(), "0");
}
