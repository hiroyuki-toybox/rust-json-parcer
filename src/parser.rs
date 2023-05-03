// json = element

// element = object | array | string | number | boolean | null

// object = '{' , (members)? , '}'
// members = member , (',' , member)*
// member = string , ':' , element

// array = '[' , (elements)? , ']'
// elements = element , (',' , element)*

// string = '"' , (characters)? , '"'
// characters =
// number =

// boolean = 'true' | 'false'

// null = 'null'

use std::iter::Peekable;
use std::vec::IntoIter;

use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Clone)]
struct Member {
    key: String,
    value: Element,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    members: Vec<Member>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
    elements: Vec<Element>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Object(Object),
    Array(Array),
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEof,
    ParseError,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tokens = tokens.into_iter().peekable();

        Self { tokens }
    }

    pub fn parse(&mut self) -> Result<Element, ParseError> {
        let node = self.parse_element().expect("parse element error");

        Ok(node)
    }
    fn parse_element(&mut self) -> Result<Element, ParseError> {
        let node = match self.tokens.next() {
            Some(token) => match token {
                Token::String(val) => Element::String(val),
                Token::Number(val) => Element::Number(val),
                Token::Bool(val) => Element::Boolean(val),
                Token::Null => Element::Null,
                Token::LeftBrace => self.parse_object().expect("parse object error"),
                Token::LeftBracket => self.parse_array().expect("parse array error"),
                _ => return Err(ParseError::UnexpectedToken(token.clone())),
            },
            _ => return Err(ParseError::ParseError),
        };

        Ok(node)
    }

    fn parse_object(&mut self) -> Result<Element, ParseError> {
        let members = self.parse_members().expect("parse members error in object");

        // println!("{:?}", members);

        let node = Element::Object(Object { members });

        if let Token::RightBrace = self.tokens.next().expect("next token error") {
            return Ok(node);
        };

        Err(ParseError::ParseError)
    }

    fn parse_members(&mut self) -> Result<Vec<Member>, ParseError> {
        let mut members: Vec<Member> = vec![];

        while let Some(tok) = self.tokens.peek().cloned() {
            // Commaだったら進める
            if let Token::Comma = tok {
                self.tokens.next();
            }
            if let Token::RightBrace = self.tokens.peek().unwrap() {
                break;
            }
            let member = self.parse_member().expect("parse member error in members");
            members.push(member);
        }

        Ok(members)
    }

    fn parse_member(&mut self) -> Result<Member, ParseError> {
        match self.tokens.next() {
            Some(Token::String(key)) => match self.tokens.next() {
                Some(Token::Colon) => {
                    let value = self.parse_element().expect("parse element error in member");
                    Ok(Member { key, value })
                }
                _ => Err(ParseError::ParseError),
            },
            _ => Err(ParseError::ParseError),
        }
    }

    fn parse_array(&mut self) -> Result<Element, ParseError> {
        let elements: Vec<Element> = self.parse_elements().expect("parse elements error");

        let node = Element::Array(Array { elements });

        if let Token::RightBracket = self.tokens.next().expect("next token error") {
            return Ok(node);
        };

        Err(ParseError::ParseError)
    }

    fn parse_elements(&mut self) -> Result<Vec<Element>, ParseError> {
        let mut elements: Vec<Element> = vec![];

        while let Some(tok) = self.tokens.peek().cloned() {
            // Commaだったら進める
            if let Token::Comma = tok {
                self.tokens.next();
            }
            if let Token::RightBracket = self.tokens.peek().unwrap() {
                break;
            }
            let element = self
                .parse_element()
                .expect("parse element error in parse_elements");
            elements.push(element);
        }

        Ok(elements)
    }
}

#[cfg(test)]
mod test {
    use crate::tokenizer::TokenizerTrait;

    use super::*;

    fn get_parser(str: String) -> Parser {
        let mut tokenizer = crate::tokenizer::Tokenizer::new(str.as_str());

        let tokens = tokenizer.tokenize().unwrap();

        Parser::new(tokens)
    }

    #[test]
    fn parse_string() {
        let mut parser = get_parser("\"これは文字列\"".to_string());

        let result = parser.parse().unwrap();

        println!("{:?}", result);

        assert_eq!(result, Element::String("これは文字列".to_string()),);
    }

    #[test]
    fn parse_object() {
        let mut parser = get_parser(
            r#"
        {}
        "#
            .to_string(),
        );

        let result = parser.parse().unwrap();

        assert_eq!(result, Element::Object(Object { members: vec![] }));

        let mut parser = get_parser(
            r#"
        {
          "key": "value"
        }
        "#
            .to_string(),
        );

        let result = parser.parse().unwrap();
        //
        assert_eq!(
            result,
            Element::Object(Object {
                members: vec![Member {
                    key: "key".to_string(),
                    value: Element::String("value".to_string()),
                }]
            })
        );

        // けつカンマあり
        let mut parser = get_parser(
            r#"
      {
        "key": "value",
        "key2": {
          "key3": "value3",
        },
      }
      "#
            .to_string(),
        );

        let result = parser.parse().unwrap();
        println!("{:?}", result);
        assert_eq!(
            result,
            Element::Object(Object {
                members: vec![
                    Member {
                        key: "key".to_string(),
                        value: Element::String("value".to_string()),
                    },
                    Member {
                        key: "key2".to_string(),
                        value: Element::Object(Object {
                            members: vec![Member {
                                key: "key3".to_string(),
                                value: Element::String("value3".to_string()),
                            }]
                        }),
                    }
                ]
            })
        );

        // けつカンマなし
        let mut parser = get_parser(
            r#"
      {
        "key": "value",
        "key2": {
          "key3": "value3",
        }
      }
      "#
            .to_string(),
        );

        let result = parser.parse().unwrap();
        println!("{:?}", result);
        assert_eq!(
            result,
            Element::Object(Object {
                members: vec![
                    Member {
                        key: "key".to_string(),
                        value: Element::String("value".to_string()),
                    },
                    Member {
                        key: "key2".to_string(),
                        value: Element::Object(Object {
                            members: vec![Member {
                                key: "key3".to_string(),
                                value: Element::String("value3".to_string()),
                            }]
                        }),
                    }
                ]
            })
        );
    }

    #[test]
    fn parse_array() {
        let mut parser = get_parser(
            r#"
            [1,2,3]
      "#
            .to_string(),
        );

        let result = parser.parse().unwrap();
        println!("{:?}", result);
        assert_eq!(
            result,
            Element::Array(Array {
                elements: vec![
                    Element::Number(1.0),
                    Element::Number(2.0),
                    Element::Number(3.0)
                ]
            })
        );

        let mut parser = get_parser(
            r#"
            [1, "string", null]
      "#
            .to_string(),
        );

        let result = parser.parse().unwrap();
        println!("{:?}", result);
        assert_eq!(
            result,
            Element::Array(Array {
                elements: vec![
                    Element::Number(1.0),
                    Element::String("string".to_string()),
                    Element::Null
                ]
            })
        );

        let mut parser = get_parser(
            r#"
            {
              "key": [1, "string", null]
            }
      "#
            .to_string(),
        );

        let result = parser.parse().unwrap();
        println!("{:?}", result);
        assert_eq!(
            result,
            Element::Object(Object {
                members: vec![Member {
                    key: "key".to_string(),
                    value: Element::Array(Array {
                        elements: vec![
                            Element::Number(1.0),
                            Element::String("string".to_string()),
                            Element::Null
                        ]
                    })
                }]
            })
        );
    }
}
