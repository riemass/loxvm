use crate::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    // TODO: Add nested types to enum variants.
    // Function,
    // Closure,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Number(x) => write!(f, "{x}"),
            Value::String(s) => write!(f, "{s}"),
        }
    }
}

impl Value {
    pub fn nil() -> Self {
        Self::Nil
    }

    pub fn checked_add(self, rhs: Self) -> Result<Self, Error> {
        match (self, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => Ok(Value::Number(lhs + rhs)),
            (Value::String(lhs), Value::String(rhs)) => {
                Ok(Value::String(format!("{}{}", lhs, rhs)))
            }
            (_, _) => Err(Error::type_error("addition", "mix of number ans string")),
        }
    }

    pub fn checked_sub(self, rhs: Self) -> Result<Self, Error> {
        if let (Value::Number(lhs), Value::Number(rhs)) = (self, rhs) {
            return Ok(Value::Number(lhs - rhs));
        }
        Err(Error::type_error("subtraction", "non-number"))
    }

    pub fn checked_mul(self, rhs: Self) -> Result<Self, Error> {
        if let (Value::Number(lhs), Value::Number(rhs)) = (self, rhs) {
            return Ok(Value::Number(lhs * rhs));
        }
        Err(Error::type_error("multiplication", "non-number"))
    }

    pub fn checked_div(self, rhs: Self) -> Result<Self, Error> {
        if let (Value::Number(lhs), Value::Number(rhs)) = (self, rhs) {
            return Ok(Value::Number(lhs / rhs));
        }
        Err(Error::type_error("division", "non-number"))
    }

    pub fn checked_neg(self) -> Result<Self, Error> {
        if let Value::Number(lhs) = self {
            return Ok(Value::Number(-lhs));
        }
        Err(Error::type_error("negation", "non-number"))
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Self::Number(f)
    }
}
