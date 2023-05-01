use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenizerError {
    InvalidCharacter(char),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),    // 数値
    String(String), // 文字列
    LeftBracket,    // 左括弧
    RightBracket,   // 右括弧
    Comma,          // カンマ
    Bool,
    Null,
    LeftBrace,
    RightBrace,
    Colon,
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
                '0'..='9' | '+' | '-' | '.' => {
                    let mut num = c.to_string();

                    while let Some('0'..='9' | '.') = self.chars.peek() {
                        num.push(self.chars.next().unwrap());
                    }

                    tokens.push(Token::Number(num.parse::<f64>().unwrap()));
                }
                '"' => {
                    let mut str = String::new();

                    while let Some(c) = self.chars.next() {
                        match c {
                            '"' => {
                                tokens.push(Token::String(str));
                                break;
                            }
                            _ => str.push(c),
                        }
                    }
                }
                'n' => {
                    let mut str = String::new();
                    let required_chars = ['u', 'l', 'l'];

                    for &required_char in required_chars.iter() {
                        if let Some(ch) = self.chars.peek() {
                            if *ch == required_char {
                                str.push(self.chars.next().unwrap());
                            } else {
                                return Err(TokenizerError::InvalidCharacter(c));
                            }
                        } else {
                            return Err(TokenizerError::InvalidCharacter(c));
                        }
                    }

                    tokens.push(Token::Null);
                }
                '{' => tokens.push(Token::LeftBrace),
                '}' => tokens.push(Token::RightBrace),
                ':' => tokens.push(Token::Colon),
                't' => {
                    let mut str = String::new();
                    let required_chars = ['r', 'u', 'e'];

                    for &required_char in required_chars.iter() {
                        if let Some(ch) = self.chars.peek() {
                            if *ch == required_char {
                                str.push(self.chars.next().unwrap());
                            } else {
                                return Err(TokenizerError::InvalidCharacter(c));
                            }
                        } else {
                            return Err(TokenizerError::InvalidCharacter(c));
                        }
                    }

                    tokens.push(Token::Bool);
                }
                'f' => {
                    let mut str = String::new();
                    let required_chars = ['a', 'l', 's', 'e'];

                    for &required_char in required_chars.iter() {
                        if let Some(ch) = self.chars.peek() {
                            if *ch == required_char {
                                str.push(self.chars.next().unwrap());
                            } else {
                                return Err(TokenizerError::InvalidCharacter(c));
                            }
                        } else {
                            return Err(TokenizerError::InvalidCharacter(c));
                        }
                    }

                    tokens.push(Token::Bool);
                }
                '[' => tokens.push(Token::LeftBracket),
                ']' => tokens.push(Token::RightBracket),
                ',' => tokens.push(Token::Comma),
                ' ' | '\n' | '\t' => {}
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

        let mut tokenizer = Tokenizer::new("-1");
        let res = tokenizer.tokenize().unwrap();
        assert_eq!(Token::Number(-1.0), res[0]);

        let mut tokenizer = Tokenizer::new("+1");
        let res = tokenizer.tokenize().unwrap();
        assert_eq!(Token::Number(1.0), res[0]);

        let mut tokenizer = Tokenizer::new(".1");
        let res = tokenizer.tokenize().unwrap();
        assert_eq!(Token::Number(0.1), res[0]);

        let mut tokenizer = Tokenizer::new("1.6");
        let res = tokenizer.tokenize().unwrap();
        assert_eq!(Token::Number(1.6), res[0]);
    }

    #[test]
    fn string_tokenize() {
        let mut tokenizer = Tokenizer::new("\"hello world\"");
        let res = tokenizer.tokenize().unwrap();
        assert_eq!(Token::String("hello world".to_string()), res[0]);
    }

    #[test]
    fn array_tokenize() {
        let test_str = r#"["文字列1", "文字列2"]"#;
        let mut tokenizer = Tokenizer::new(test_str);
        let res = tokenizer.tokenize().unwrap();

        assert_eq!(
            vec![
                Token::LeftBracket,
                Token::String("文字列1".to_string()),
                Token::Comma,
                Token::String("文字列2".to_string()),
                Token::RightBracket,
            ],
            res
        );
    }
    #[test]
    fn boolean_tokenize() {
        let test_str = r#"true"#;
        let mut tokenizer = Tokenizer::new(test_str);
        let res = tokenizer.tokenize().unwrap();

        assert_eq!(vec![Token::Bool,], res);

        let test_str = r#"false"#;
        let mut tokenizer = Tokenizer::new(test_str);
        let res = tokenizer.tokenize().unwrap();

        assert_eq!(vec![Token::Bool,], res);
    }

    #[test]
    fn null_tokenize() {
        let test_str = r#"null"#;
        let mut tokenizer = Tokenizer::new(test_str);
        let res = tokenizer.tokenize().unwrap();

        assert_eq!(vec![Token::Null], res);
    }

    #[test]
    fn object_tokenize() {
        let test_str = r#"
          {
            "key": "value"
          }
        "#;
        let mut tokenizer = Tokenizer::new(test_str);
        let res = tokenizer.tokenize().unwrap();

        assert_eq!(
            vec![
                Token::LeftBrace,
                Token::String("key".to_string()),
                Token::Colon,
                Token::String("value".to_string()),
                Token::RightBrace
            ],
            res
        );
    }
}
