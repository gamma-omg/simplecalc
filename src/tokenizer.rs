use crate::Error;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Number(&'a str),
    Operator(&'a str),
    Whitespace(&'a str),
    ParOpen,
    ParClose,
    End,
}

pub type TokenStream<'a> = Vec<Token<'a>>;

#[derive(Debug, PartialEq)]
enum TokenizerState {
    Initial,
    ParseNumber,
    ParseOperator,
    ParseWhitespace,
    ParOpen(usize),
    ParClose(usize),
}

pub fn tokenize(input: &str) -> Result<TokenStream, Error> {
    let mut tokens = TokenStream::new();
    let mut state = TokenizerState::Initial;
    let mut token_start = 0;

    for (pos, ch) in input.chars().enumerate() {
        let next_state = match ch {
            _ if ch.is_digit(10) || ch == '.' => TokenizerState::ParseNumber,
            _ if ch.is_whitespace() => TokenizerState::ParseWhitespace,
            _ if ch == '+' || ch == '-' || ch == '*' || ch == '/' => TokenizerState::ParseOperator,
            '(' => TokenizerState::ParOpen(pos),
            ')' => TokenizerState::ParClose(pos),
            _ => return Err(Error::TokenizerError(pos, input.chars().nth(pos).unwrap())),
        };

        if next_state != state {
            if let Some(token) = yield_token(state, &input, token_start, pos) {
                tokens.push(token);
            }

            token_start = pos;
            state = next_state;
        }
    }

    if let Some(token) = yield_token(state, &input, token_start, input.len()) {
        tokens.push(token);
    }

    tokens.push(Token::End);
    Ok(tokens)
}

fn yield_token(
    current_state: TokenizerState,
    input: &str,
    start: usize,
    end: usize,
) -> Option<Token> {
    match current_state {
        TokenizerState::Initial => None,
        TokenizerState::ParseNumber => Some(Token::Number(&input[start..end])),
        TokenizerState::ParseOperator => Some(Token::Operator(&input[start..end])),
        TokenizerState::ParseWhitespace => Some(Token::Whitespace(&input[start..end])),
        TokenizerState::ParOpen(_) => Some(Token::ParOpen),
        TokenizerState::ParClose(_) => Some(Token::ParClose),
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::tokenizer::{tokenize, Token};

    #[test]
    fn test_numbers() {
        let tokens = tokenize("21 43.5 .7 0").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number("21"),
                Token::Whitespace(" "),
                Token::Number("43.5"),
                Token::Whitespace(" "),
                Token::Number(".7"),
                Token::Whitespace(" "),
                Token::Number("0"),
                Token::End,
            ]
        );
    }

    #[test]
    fn test_operators() {
        let tokens = tokenize("2+3**4/5*6++//").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number("2"),
                Token::Operator("+"),
                Token::Number("3"),
                Token::Operator("**"),
                Token::Number("4"),
                Token::Operator("/"),
                Token::Number("5"),
                Token::Operator("*"),
                Token::Number("6"),
                Token::Operator("++//"),
                Token::End
            ]
        );
    }

    #[test]
    fn test_parenthesis() {
        let tokens = tokenize(")(() (").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::ParClose,
                Token::ParOpen,
                Token::ParOpen,
                Token::ParClose,
                Token::Whitespace(" "),
                Token::ParOpen,
                Token::End
            ]
        )
    }

    #[test]
    fn test_whitespace() {
        let tokens = tokenize("  1 + \t2\n\n").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Whitespace("  "),
                Token::Number("1"),
                Token::Whitespace(" "),
                Token::Operator("+"),
                Token::Whitespace(" \t"),
                Token::Number("2"),
                Token::Whitespace("\n\n"),
                Token::End,
            ]
        );

        let tokens = tokenize("\n\n  1 \n").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Whitespace("\n\n  "),
                Token::Number("1"),
                Token::Whitespace(" \n"),
                Token::End,
            ]
        )
    }

    #[test]
    fn test_error() {
        assert!(tokenize("1+2+@").is_err());
        assert!(tokenize("11,3").is_err());
    }
}
