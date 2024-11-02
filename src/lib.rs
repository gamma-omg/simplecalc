mod lexer;
mod tokenizer;

use lexer::{parse, Lexem, LexemStream, Operator};
use std::num::ParseFloatError;
use thiserror::Error;
use tokenizer::tokenize;

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

pub fn eval(expr: &str) -> Result<f64, Error> {
    let tokens = tokenize(expr)?;
    let lexems = parse(&tokens)?;
    let postfix = postfix_repr(&lexems);
    let mut stack = Vec::new();

    for lexem in postfix.iter() {
        match lexem {
            Lexem::Number(num) => stack.push(*num),
            Lexem::Operator(op) => {
                let b = stack.pop().ok_or(Error::EvalError)?;
                let a = stack.pop().ok_or(Error::EvalError)?;
                match op {
                    Operator::Add => stack.push(a + b),
                    Operator::Sub => stack.push(a - b),
                    Operator::Mul => stack.push(a * b),
                    Operator::Div => stack.push(a / b),
                    Operator::Pow => stack.push(a.powf(b)),
                }
            }
            Lexem::ParOpen => return Err(Error::EvalError),
            Lexem::ParClose => return Err(Error::EvalError),
        }
    }

    let result = *stack.last().ok_or(Error::EvalError)?;
    Ok(result)
}

fn postfix_repr(infix: &LexemStream) -> LexemStream {
    let mut postfix = LexemStream::new();
    let mut stack = Vec::new();
    for lexem in infix.iter() {
        match lexem {
            Lexem::Number(_) => postfix.push(lexem.clone()),
            Lexem::Operator(op) => {
                while let Some(Lexem::Operator(cur)) = stack.last() {
                    if cur.priority() >= op.priority() {
                        postfix.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                stack.push(lexem.clone());
            }
            Lexem::ParOpen => stack.push(lexem.clone()),
            Lexem::ParClose => {
                while let Some(top) = stack.pop() {
                    if top == Lexem::ParOpen {
                        break;
                    }

                    postfix.push(top);
                }
            }
        }
    }

    while let Some(lexem) = stack.pop() {
        postfix.push(lexem);
    }

    postfix
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        eval,
        lexer::{parse, Lexem, Operator},
        postfix_repr,
        tokenizer::tokenize,
    };

    #[test]
    fn test_postfix_repr() {
        let postfix = postfix_repr(&parse(&tokenize("1+2*3+4").unwrap()).unwrap());
        assert_eq!(
            postfix,
            vec![
                Lexem::Number(1.0),
                Lexem::Number(2.0),
                Lexem::Number(3.0),
                Lexem::Operator(Operator::Mul),
                Lexem::Operator(Operator::Add),
                Lexem::Number(4.0),
                Lexem::Operator(Operator::Add),
            ]
        )
    }

    #[test]
    fn test_postfix_repr_leading_minus() {
        let postfix = postfix_repr(&parse(&tokenize("-5+10").unwrap()).unwrap());
        assert_eq!(
            postfix,
            vec![
                Lexem::Number(-5.0),
                Lexem::Number(10.0),
                Lexem::Operator(Operator::Add),
            ]
        )
    }

    #[test]
    fn test_postfix_repr_parethesis() {
        let postfix = postfix_repr(&parse(&tokenize("2*(100+50)").unwrap()).unwrap());
        assert_eq!(
            postfix,
            vec![
                Lexem::Number(2.0),
                Lexem::Number(100.0),
                Lexem::Number(50.0),
                Lexem::Operator(Operator::Add),
                Lexem::Operator(Operator::Mul),
            ]
        )
    }

    #[test]
    fn test_eval() {
        assert_eq!(8.0, eval("2**3").unwrap());
        assert_eq!(-3.0, eval("1+2**3-10/5*6").unwrap());
        assert_eq!(300.0, eval("2*(100+50)").unwrap());
        assert_eq!(-1.0, eval("-5+4").unwrap());
    }
}
