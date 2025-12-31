use crate::{
    Lexer, ast,
    error::Error,
    lex::{Token, TokenKind},
};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

type ParseResult<T> = Result<T, Error>;

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
        _ => unreachable!(),
    }
}

// TODO: Consider iterating through the colleciton of tokens
impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);
        let tokens: Result<Vec<_>, crate::error::Error> = lexer.collect();
        // TODO: Propagate errors.
        let mut tokens = tokens.unwrap();
        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: "",
            // TODO: Fix Eof tokens line and column.
            line: tokens.last().map(|t| t.line).unwrap_or(0),
            column: tokens.last().map(|t| t.column).unwrap_or(0),
        });
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> &Token<'_> {
        self.tokens
            .get(self.current)
            .expect("Iteration past the end")
    }

    fn advance(&mut self) {
        // Note: Doesn't advance the current token past Eof. Since Eof is mandatory
        // we can safely unwrap the get call.
        if self.tokens.get(self.current).unwrap().kind != TokenKind::Eof {
            self.current += 1;
        }
    }

    fn next(&mut self) -> &Token<'_> {
        self.advance();
        self.tokens
            .get(self.current - 1)
            .expect("Iteration past the end")
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), Error> {
        let current_token = self.peek();
        if current_token.kind == kind {
            self.advance();
            return Ok(());
        }
        Err(Error::unexpected_token(Some(kind), current_token))
    }

    pub fn statements(&mut self) -> Vec<ast::Statement> {
        todo!("Parse statement");
    }

    pub fn statement(&mut self) -> Result<ast::Statement, Error> {
        match self.peek().kind {
            TokenKind::Var => self.var_declaration(),
            TokenKind::If => self.if_statement(),
            TokenKind::For => self.for_statement(),
            TokenKind::Print => self.print_statement(),
            TokenKind::Return => todo!(),
            TokenKind::While => self.while_statement(),
            TokenKind::LeftBrace => self.group_statement(),
            _ => self.expression_statement(),
        }
    }

    pub fn expression(&mut self) -> ParseResult<ast::ExpressionStmt> {
        self.expr_with_binding_power(0)
    }

    fn expr_with_binding_power(&mut self, min_bp: u8) -> ParseResult<ast::ExpressionStmt> {
        let mut lhs = match self.peek().kind {
            TokenKind::Number => {
                let current_token = self.next();
                let val: f64 = current_token
                    .lexeme
                    .parse()
                    .map_err(|_| Error::invalid_number(current_token))?;
                ast::ExpressionStmt::Number(val)
            }
            TokenKind::Ident => ast::ExpressionStmt::Identifier(self.next().lexeme.into()),
            TokenKind::Plus | TokenKind::Minus => {
                let tok = self.next();
                let bp = prefix_binding_power(tok.kind);
                ast::ExpressionStmt::Unary(
                    tok.lexeme.into(),
                    Box::new(self.expr_with_binding_power(bp)?),
                )
            }
            TokenKind::LeftParen => {
                self.advance();
                let lhs = self.expr_with_binding_power(0)?;
                self.expect(TokenKind::RightParen)?;
                lhs
            }
            _ => {
                return Err(Error::unexpected_token(None, self.peek()));
            }
        };
        loop {
            if let Some((l_bp, r_bp)) = infix_binding_power(self.peek().kind) {
                if l_bp < min_bp {
                    break;
                }
                // TODO: refactor operator to string properly.
                let s = self.peek().lexeme.to_owned();
                self.advance();
                let rhs = self.expr_with_binding_power(r_bp)?;
                lhs = ast::ExpressionStmt::Binary(s, Box::new((lhs, rhs)));
                continue;
            }
            break;
        }
        Ok(lhs)
    }

    fn expression_statement(&mut self) -> ParseResult<ast::Statement> {
        let expr = self.expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(ast::Statement::Expression(expr))
    }

    fn print_statement(&mut self) -> ParseResult<ast::Statement> {
        // Advance print kw matched in caller.
        self.advance();
        let expr = self.expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(ast::Statement::Print(ast::PrintStmt { expr }))
    }

    fn if_statement(&self) -> ParseResult<ast::Statement> {
        todo!()
    }

    fn for_statement(&self) -> ParseResult<ast::Statement> {
        todo!()
    }

    fn while_statement(&self) -> ParseResult<ast::Statement> {
        todo!()
    }

    fn group_statement(&self) -> ParseResult<ast::Statement> {
        todo!()
    }

    fn var_declaration(&mut self) -> ParseResult<ast::Statement> {
        self.advance();
        // TODO: Refactor borrowing to unify the two lines below.
        let name = self.peek().lexeme.to_owned();
        self.expect(TokenKind::Ident)?;
        let expr = if self.check(TokenKind::Equal) {
            Some(self.expression()?)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(ast::Statement::VarDeclaration(name, expr))
    }
}

#[test]
fn tests() {
    fn parse_expr(s: &str) -> ast::ExpressionStmt {
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
