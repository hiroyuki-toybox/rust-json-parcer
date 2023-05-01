use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenizerError {
    InvalidCharacter(char),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64), // 数値
}

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

pub trait TokenizerTrait<'a> {
    fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError>;
}

impl<'a> Tokenizer<'a> {
    pub fn new(str: &'a str) -> Self {
        let chars = str.chars().into_iter().peekable();

        Tokenizer { chars }
    }
}

impl TokenizerTrait<'_> for Tokenizer<'_> {
    fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut tokens = Vec::new();

        while let Some(c) = self.chars.next() {
            match c {
                '0'..='9' => {
                    let mut num = c.to_string();

                    while let Some('0'..='9') = self.chars.peek() {
                        num.push(self.chars.next().unwrap());
                    }

                    tokens.push(Token::Number(num.parse::<f64>().unwrap()));
                }
                _ => return Err(TokenizerError::InvalidCharacter(c)),
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number_tokenize() {
        let mut tokenizer = Tokenizer::new("1");
        let res = tokenizer.tokenize().unwrap();
        assert_eq!(Token::Number(1.0), res[0]);

        let mut tokenizer = Tokenizer::new("a");
        let res = tokenizer.tokenize();
        assert_eq!(Err(TokenizerError::InvalidCharacter('a')), res);
    }
}
