use std::fmt;

pub struct Statement {}

pub enum ExpressionTree {
    Number(f64),
    Identifier(String),
    Unary(String, Box<ExpressionTree>),
    Binary(String, Box<(ExpressionTree, ExpressionTree)>),
}

impl fmt::Display for ExpressionTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionTree::Number(x) => write!(f, "{}", x),
            ExpressionTree::Identifier(id) => write!(f, "{}", id),
            ExpressionTree::Unary(token, operand) => {
                write!(f, "({} {})", token, operand)
            }
            ExpressionTree::Binary(token, operands) => {
                write!(f, "({} {} {})", token, operands.0, operands.1)
            }
        }
    }
}
