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
    type Output = Result<Self, String>;

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
                return Err("Not implemented".into());
            }
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Result<Self, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        if let (Value::Number(lhs), Value::Number(rhs)) = (self, rhs) {
            return Ok(Value::Number(lhs - rhs));
        }
        return Err("Not implemented".into());
    }
}

impl std::ops::Mul for Value {
    type Output = Result<Self, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        if let (Value::Number(lhs), Value::Number(rhs)) = (self, rhs) {
            return Ok(Value::Number(lhs * rhs));
        }
        return Err("Not implemented".into());
    }
}

impl std::ops::Div for Value {
    type Output = Result<Self, String>;

    fn div(self, rhs: Self) -> Self::Output {
        if let (Value::Number(lhs), Value::Number(rhs)) = (self, rhs) {
            return Ok(Value::Number(lhs / rhs));
        }
        return Err("Not implemented".into());
    }
}

impl std::ops::Neg for Value {
    type Output = Result<Self, String>;

    fn neg(self) -> Self::Output {
        if let Value::Number(lhs) = self {
            return Ok(Value::Number(-lhs));
        }
        return Err("Not implemented".into());
    }
}
