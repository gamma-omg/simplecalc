mod lexer;
mod tokenizer;

use std::num::ParseFloatError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected token at {0}")]
    TokenizerError(usize),
    #[error("Unexpected lexem at {0}")]
    LexerError(usize),
    #[error("Failed to parse a number")]
    ParseNumberError(#[from] ParseFloatError),
    #[error("Failed to parse operator {0}")]
    ParseOperatorError(String),
    #[error("Failed to evaluate expression")]
    EvalError,
}

pub fn eval(_expr: &str) -> Result<f64, Error> {
    todo!("Implement evaluation");
}
