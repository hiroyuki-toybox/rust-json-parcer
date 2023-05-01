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
    Bool(bool),
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

                    tokens.push(Token::Bool(true));
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

                    tokens.push(Token::Bool(false));
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

        assert_eq!(vec![Token::Bool(true)], res);

        let test_str = r#"false"#;
        let mut tokenizer = Tokenizer::new(test_str);
        let res = tokenizer.tokenize().unwrap();

        assert_eq!(vec![Token::Bool(false)], res);
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

    #[test]
    fn test_comprehensive_json_tokenization() {
        let test_str = r#"
      {
        "string": "こんにちは、世界！",
        "integer": 42,
        "float": 3.1,
        "boolean": true,
        "null": null,
        "array": [1, "two", 3.0, true, null],
        "nested_array": [
          ["a", "b", "c"],
          [1, 2, 3],
          [true, false, null]
        ],
        "object": {
          "name": "山田太郎",
          "age": 30,
          "address": {
            "country": "日本",
            "city": "東京",
            "postal_code": "123-4567"
          },
          "hobbies": ["読書", "映画", "旅行"]
        },
        "empty_array": [],
        "empty_object": {},
        "unicode": "\u3053\u3093\u306B\u3061\u306F"
      }
      "#;

        let mut tokenizer = Tokenizer::new(test_str);
        let res = tokenizer.tokenize().unwrap();

        assert_eq!(
            vec![
                Token::LeftBrace,
                Token::String("string".to_string()),
                Token::Colon,
                Token::String("こんにちは、世界！".to_string()),
                Token::Comma,
                Token::String("integer".to_string()),
                Token::Colon,
                Token::Number(42.0),
                Token::Comma,
                Token::String("float".to_string()),
                Token::Colon,
                Token::Number(3.1),
                Token::Comma,
                Token::String("boolean".to_string()),
                Token::Colon,
                Token::Bool(true),
                Token::Comma,
                Token::String("null".to_string()),
                Token::Colon,
                Token::Null,
                Token::Comma,
                Token::String("array".to_string()),
                Token::Colon,
                Token::LeftBracket,
                Token::Number(1.0),
                Token::Comma,
                Token::String("two".to_string()),
                Token::Comma,
                Token::Number(3.0),
                Token::Comma,
                Token::Bool(true),
                Token::Comma,
                Token::Null,
                Token::RightBracket,
                Token::Comma,
                Token::String("nested_array".to_string()),
                Token::Colon,
                Token::LeftBracket,
                Token::LeftBracket,
                Token::String("a".to_string()),
                Token::Comma,
                Token::String("b".to_string()),
                Token::Comma,
                Token::String("c".to_string()),
                Token::RightBracket,
                Token::Comma,
                Token::LeftBracket,
                Token::Number(1.0),
                Token::Comma,
                Token::Number(2.0),
                Token::Comma,
                Token::Number(3.0),
                Token::RightBracket,
                Token::Comma,
                Token::LeftBracket,
                Token::Bool(true),
                Token::Comma,
                Token::Bool(false),
                Token::Comma,
                Token::Null,
                Token::RightBracket,
                Token::RightBracket,
                Token::Comma,
                Token::String("object".to_string()),
                Token::Colon,
                Token::LeftBrace,
                Token::String("name".to_string()),
                Token::Colon,
                Token::String("山田太郎".to_string()),
                Token::Comma,
                Token::String("age".to_string()),
                Token::Colon,
                Token::Number(30.0),
                Token::Comma,
                Token::String("address".to_string()),
                Token::Colon,
                Token::LeftBrace,
                Token::String("country".to_string()),
                Token::Colon,
                Token::String("日本".to_string()),
                Token::Comma,
                Token::String("city".to_string()),
                Token::Colon,
                Token::String("東京".to_string()),
                Token::Comma,
                Token::String("postal_code".to_string()),
                Token::Colon,
                Token::String("123-4567".to_string()),
                Token::RightBrace,
                Token::Comma,
                Token::String("hobbies".to_string()),
                Token::Colon,
                Token::LeftBracket,
                Token::String("読書".to_string()),
                Token::Comma,
                Token::String("映画".to_string()),
                Token::Comma,
                Token::String("旅行".to_string()),
                Token::RightBracket,
                Token::RightBrace,
                Token::Comma,
                Token::String("empty_array".to_string()),
                Token::Colon,
                Token::LeftBracket,
                Token::RightBracket,
                Token::Comma,
                Token::String("empty_object".to_string()),
                Token::Colon,
                Token::LeftBrace,
                Token::RightBrace,
                Token::Comma,
                Token::String("unicode".to_string()),
                Token::Colon,
                Token::String(r#"\u3053\u3093\u306B\u3061\u306F"#.to_string()),
                Token::RightBrace,
            ],
            res
        );
    }
}
