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

impl std::ops::Add for Value {
    type Output = Result<Self, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                return Ok(Value::Number(lhs + rhs));
            }
            (Value::String(mut lhs), Value::String(rhs)) => {
                lhs.push_str(rhs.as_str());
                return Ok(Value::String(lhs));
            }
            (_, _) => {
                return Err(Error::RuntimeError("Unsupported types for addition".into()));
            }
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Result<Self, Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        if let (Value::Number(lhs), Value::Number(rhs)) = (self, rhs) {
            return Ok(Value::Number(lhs - rhs));
        }
        return Err(Error::RuntimeError(
            "Unsupported types for subtraction".into(),
        ));
    }
}

impl std::ops::Mul for Value {
    type Output = Result<Self, Error>;

    fn mul(self, rhs: Self) -> Self::Output {
        if let (Value::Number(lhs), Value::Number(rhs)) = (self, rhs) {
            return Ok(Value::Number(lhs * rhs));
        }
        return Err(Error::RuntimeError(
            "Unsupported types for multiplication".into(),
        ));
    }
}

impl std::ops::Div for Value {
    type Output = Result<Self, Error>;

    fn div(self, rhs: Self) -> Self::Output {
        if let (Value::Number(lhs), Value::Number(rhs)) = (self, rhs) {
            return Ok(Value::Number(lhs / rhs));
        }
        return Err(Error::RuntimeError("Unsupported types for division".into()));
    }
}

impl std::ops::Neg for Value {
    type Output = Result<Self, Error>;

    fn neg(self) -> Self::Output {
        if let Value::Number(lhs) = self {
            return Ok(Value::Number(-lhs));
        }
        return Err(Error::RuntimeError("Unsupported types for negation".into()));
    }
}
