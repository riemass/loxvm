use std::fmt;

pub enum Statement {
    Expression(ExpressionStmt),
    VarDeclaration(String, Option<ExpressionStmt>),
    For,
    If,
    Print(PrintStmt),
    Return,
    While,
    Block,
}

pub enum ExpressionStmt {
    Number(f64),
    Identifier(String),
    Unary(String, Box<ExpressionStmt>),
    Binary(String, Box<(ExpressionStmt, ExpressionStmt)>),
}

impl fmt::Display for ExpressionStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionStmt::Number(x) => write!(f, "{}", x),
            ExpressionStmt::Identifier(id) => write!(f, "{}", id),
            ExpressionStmt::Unary(token, operand) => {
                write!(f, "({} {})", token, operand)
            }
            ExpressionStmt::Binary(token, operands) => {
                write!(f, "({} {} {})", token, operands.0, operands.1)
            }
        }
    }
}

pub struct PrintStmt {
    pub expr: ExpressionStmt,
}

impl fmt::Display for PrintStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(print {})", self.expr)
    }
}
