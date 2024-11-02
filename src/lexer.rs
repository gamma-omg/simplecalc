use crate::{
    tokenizer::{Token, TokenStream},
    Error,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Lexem {
    Number(f64),
    Operator(Operator),
    ParOpen,
    ParClose,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Operator {
    pub fn priority(&self) -> u8 {
        match self {
            Operator::Add => 0,
            Operator::Sub => 0,
            Operator::Mul => 1,
            Operator::Div => 1,
            Operator::Pow => 2,
        }
    }
}

pub type LexemStream = Vec<Lexem>;

#[derive(Debug, PartialEq)]
enum LexerState<'a> {
    Initial,
    LeadingSign(f64),
    Number { val: &'a str, sign: f64 },
    Operator(&'a str),
    ParOpen,
    ParClose,
    End,
}

pub fn parse(tokens: &TokenStream) -> Result<LexemStream, Error> {
    let mut lexems = LexemStream::new();
    let mut state = LexerState::Initial;
    let mut pars = 0;

    for (pos, token) in tokens.iter().enumerate() {
        state = match state {
            LexerState::Initial => match token {
                Token::Whitespace(_) => LexerState::Initial,
                Token::Number(val) => LexerState::Number { val, sign: 1.0 },
                Token::Operator(op) if *op == "+" || *op == "-" => {
                    LexerState::LeadingSign(if *op == "-" { -1.0 } else { 1.0 })
                }
                Token::ParOpen => LexerState::ParOpen,
                _ => return Err(Error::LexerError(pos)),
            },
            LexerState::LeadingSign(sign) => match token {
                Token::Number(val) => LexerState::Number { val, sign },
                Token::ParOpen => {
                    lexems.push(Lexem::Number(sign.into()));
                    lexems.push(Lexem::Operator(Operator::Mul));
                    LexerState::ParOpen
                }
                _ => return Err(Error::LexerError(pos)),
            },
            LexerState::Number { val, sign } => {
                if let Token::Whitespace(_) = token {
                    LexerState::Number { val, sign };
                }

                let num: f64 = val.parse().map_err(|e| Error::ParseNumberError(e))?;
                lexems.push(Lexem::Number(num * sign));

                match token {
                    Token::Operator(op) => LexerState::Operator(op),
                    Token::ParClose => LexerState::ParClose,
                    Token::End => LexerState::End,
                    _ => return Err(Error::LexerError(pos)),
                }
            }
            LexerState::Operator(op) => {
                if let Token::Whitespace(_) = token {
                    LexerState::Operator(op);
                }

                match op {
                    "+" => lexems.push(Lexem::Operator(Operator::Add)),
                    "-" => lexems.push(Lexem::Operator(Operator::Sub)),
                    "*" => lexems.push(Lexem::Operator(Operator::Mul)),
                    "/" => lexems.push(Lexem::Operator(Operator::Div)),
                    "**" => lexems.push(Lexem::Operator(Operator::Pow)),
                    _ => return Err(Error::ParseOperatorError(op.to_string())),
                }

                match token {
                    Token::Number(val) => LexerState::Number { val, sign: 1.0 },
                    Token::ParOpen => LexerState::ParOpen,
                    _ => return Err(Error::LexerError(pos)),
                }
            }
            LexerState::ParOpen => {
                if let Token::Whitespace(_) = token {
                    continue;
                }

                pars += 1;
                lexems.push(Lexem::ParOpen);
                match token {
                    Token::Operator(op) if *op == "+" || *op == "-" => {
                        LexerState::LeadingSign(if *op == "-" { -1.0 } else { 1.0 })
                    }
                    Token::Number(val) => LexerState::Number { val, sign: 1.0 },
                    Token::ParOpen => LexerState::ParOpen,
                    _ => return Err(Error::LexerError(pos)),
                }
            }
            LexerState::ParClose => {
                if let Token::Whitespace(_) = token {
                    continue;
                }

                pars -= 1;
                lexems.push(Lexem::ParClose);

                match token {
                    Token::Operator(op) => LexerState::Operator(op),
                    Token::ParClose => LexerState::ParClose,
                    Token::End => LexerState::End,
                    _ => return Err(Error::LexerError(pos)),
                }
            }
            LexerState::End => return Err(Error::LexerError(pos)),
        }
    }

    if state != LexerState::End || pars != 0 {
        Err(Error::LexerError(tokens.len()))
    } else {
        Ok(lexems)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::{parse, Lexem, Operator};
    use crate::tokenizer::tokenize;

    #[test]
    fn test_lexer() {
        let lexems = parse(&tokenize("12+13**10").unwrap()).unwrap();
        assert_eq!(
            lexems,
            vec![
                Lexem::Number(12.0),
                Lexem::Operator(Operator::Add),
                Lexem::Number(13.0),
                Lexem::Operator(Operator::Pow),
                Lexem::Number(10.0),
            ]
        )
    }

    #[test]
    fn test_leading_minus() {
        let lexems = parse(&tokenize("-5*(-10)").unwrap()).unwrap();
        assert_eq!(
            lexems,
            vec![
                Lexem::Number(-5.0),
                Lexem::Operator(Operator::Mul),
                Lexem::ParOpen,
                Lexem::Number(-10.0),
                Lexem::ParClose,
            ]
        )
    }

    #[test]
    fn test_leading_plus() {
        let lexems = parse(&tokenize("+5*(+10)").unwrap()).unwrap();
        assert_eq!(
            lexems,
            vec![
                Lexem::Number(5.0),
                Lexem::Operator(Operator::Mul),
                Lexem::ParOpen,
                Lexem::Number(10.0),
                Lexem::ParClose,
            ]
        )
    }

    #[test]
    fn test_leading_minus_before_par() {
        let lexems = parse(&tokenize("-(-1)").unwrap()).unwrap();
        assert_eq!(
            lexems,
            vec![
                Lexem::Number(-1.0),
                Lexem::Operator(Operator::Mul),
                Lexem::ParOpen,
                Lexem::Number(-1.0),
                Lexem::ParClose
            ]
        )
    }

    #[test]
    fn test_parse_floats() {
        let lexems = parse(&tokenize("12.34+(-56.78)").unwrap()).unwrap();
        assert_eq!(
            lexems,
            vec![
                Lexem::Number(12.34),
                Lexem::Operator(Operator::Add),
                Lexem::ParOpen,
                Lexem::Number(-56.78),
                Lexem::ParClose
            ]
        )
    }

    #[test]
    fn test_parse_pow() {
        let lexems = parse(&tokenize("2**3").unwrap()).unwrap();
        assert_eq!(
            lexems,
            vec![
                Lexem::Number(2.0),
                Lexem::Operator(Operator::Pow),
                Lexem::Number(3.0),
            ]
        )
    }

    #[test]
    fn test_parse_errors() {
        assert!(parse(&tokenize("2+").unwrap()).is_err());
        assert!(parse(&tokenize("2+(").unwrap()).is_err());
        assert!(parse(&tokenize("11 22").unwrap()).is_err());
        assert!(parse(&tokenize("5+)").unwrap()).is_err());
        assert!(parse(&tokenize("8/()").unwrap()).is_err());
        assert!(parse(&tokenize("8/(2))").unwrap()).is_err());
        assert!(parse(&tokenize("8.8.10").unwrap()).is_err());
    }
}
