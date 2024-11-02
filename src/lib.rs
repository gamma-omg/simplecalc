use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected token at {0}")]
    TokenizerError(usize),
    #[error("Unexpected lexem at {0}")]
    LexerError(usize),
    #[error("Failed to evaluate expression")]
    EvalError,
}

pub fn eval(expr: &str) -> Result<f64, Error> {
    todo!("Implement evaluation");
}