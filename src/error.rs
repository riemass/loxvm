use crate::lex::{Token, TokenKind};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    // Lexer errors
    UnexpectedChar {
        ch: char,
        line: usize,
        column: usize,
    },
    UnterminatedString {
        line: usize,
        column: usize,
    },
    // Parser errors
    InvalidNumber {
        lexeme: String,
        line: usize,
        column: usize,
    },
    UnexpectedToken {
        expected: Option<TokenKind>,
        found: TokenKind,
        line: usize,
        column: usize,
    },
    // Runtime errors
    InvalidInstruction {
        opcode: u8,
        offset: usize,
    },
    StackUnderflow {
        operation: String,
    },
    TypeError {
        operation: String,
        type_name: String,
    },
    DivisionByZero {
        line: usize,
    },
}

impl Error {
    pub fn unexpected_char(ch: char, line: usize, column: usize) -> Self {
        Self::UnexpectedChar { ch, line, column }
    }

    pub fn unterminated_string(line: usize, column: usize) -> Self {
        Self::UnterminatedString { line, column }
    }

    pub fn invalid_number(token: &Token<'_>) -> Self {
        Self::InvalidNumber {
            lexeme: token.lexeme.into(),
            line: token.line,
            column: token.column,
        }
    }

    pub fn unexpected_token(expected: Option<TokenKind>, token: &Token<'_>) -> Self {
        Self::UnexpectedToken {
            expected,
            found: token.kind,
            line: token.line,
            column: token.column,
        }
    }

    pub fn invalid_instruction(opcode: u8, offset: usize) -> Self {
        Self::InvalidInstruction { opcode, offset }
    }

    pub fn stack_underflow(operation: impl Into<String>) -> Self {
        Self::StackUnderflow {
            operation: operation.into(),
        }
    }

    pub fn type_error(operation: impl Into<String>, type_name: impl Into<String>) -> Self {
        Self::TypeError {
            operation: operation.into(),
            type_name: type_name.into(),
        }
    }

    pub fn division_by_zero(line: usize) -> Self {
        Self::DivisionByZero { line }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedChar { ch, line, column } => {
                write!(f, "[{line}:{column}] Unexpected character '{ch}'")
            }
            Error::UnterminatedString { line, column } => {
                write!(f, "[{line}:{column}] Unterminated string literal")
            }
            Error::InvalidNumber {
                lexeme,
                line,
                column,
            } => {
                write!(f, "[{line}:{column}] Invalid number: '{lexeme}'")
            }
            Error::UnexpectedToken {
                expected,
                found,
                line,
                column,
            } => {
                if let Some(exp) = expected {
                    write!(f, "[{line}:{column}] Expected {exp:?}, found {found:?}")
                } else {
                    write!(f, "[{line}:{column}] Unexpected token: {found:?}")
                }
            }
            Error::InvalidInstruction { opcode, offset } => {
                write!(f, "Invalid instruction {opcode:#x} at offset {offset}")
            }
            Error::StackUnderflow { operation } => {
                write!(f, "Stack underflow during {operation}")
            }
            Error::TypeError {
                operation,
                type_name,
            } => {
                write!(f, "Type error: cannot {operation} on {type_name}")
            }
            Error::DivisionByZero { line } => {
                write!(f, "[{line}] Division by zero")
            }
        }
    }
}

impl std::error::Error for Error {}
