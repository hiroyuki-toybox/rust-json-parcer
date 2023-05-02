// json = element

// element = object | array | string | number | boolean | null

// object = '{' , (members)? , '}'
// members = member , (',' , member)*
// member = string , ':' , element

// array = '[' , (elements)? , ']'
// elements = element , (',' , element)*

// string = '"' , (characters)? , '"'
// characters = characters
// number = number

// boolean = 'true' | 'false'

// null = 'null'

use std::iter::Peekable;
use std::vec::IntoIter;

use crate::tokenizer::{Token, TokenizerError};

#[derive(Debug, PartialEq, Clone)]
struct Member {
    key: String,
    value: Element,
}

#[derive(Debug, PartialEq, Clone)]
struct Object {
    members: Vec<Member>,
}

struct Array {
    elements: Vec<Element>,
}

enum NodeChildren {
    ObjectLiteral,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Object(Object),
    Array,
    End,
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    position: usize,
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

        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Element, ParseError> {
        let mut node = self.parse_element()?;

        Ok(node)
    }
    fn parse_element(&mut self) -> Result<Element, ParseError> {
        let node = match self.tokens.next() {
            Some(token) => match token {
                Token::String(val) => Element::String(val),
                Token::Number(val) => Element::Number(val),
                Token::Bool(val) => Element::Boolean(val),
                Token::Null => Element::Null,
                Token::LeftBrace => self.parse_object()?,
                Token::LeftBracket => {
                    // arrayのパース
                    todo!()
                }
                _ => return Err(ParseError::UnexpectedToken(token.clone())),
            },
            _ => return Err(ParseError::ParseError),
        };

        Ok(node)
    }

    fn parse_object(&mut self) -> Result<Element, ParseError> {
        let members = self.parse_members()?;

        let node = Element::Object(Object { members });

        let tok = self.tokens.next().unwrap();

        if let Token::End | Token::Comma = tok {
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
                self.tokens.next();
                break;
            }
            let member = self.parse_member()?;
            members.push(member);
        }

        Ok(members)
    }

    fn parse_member(&mut self) -> Result<Member, ParseError> {
        match self.tokens.next() {
            Some(Token::String(key)) => match self.tokens.next() {
                Some(Token::Colon) => match self.tokens.next() {
                    Some(Token::String(val)) => Ok(Member {
                        key,
                        value: Element::String(val),
                    }),
                    _ => Err(ParseError::ParseError),
                },
                _ => Err(ParseError::ParseError),
            },
            _ => Err(ParseError::ParseError),
        }
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

        // assert_eq!(
        //     result.kind,
        //     NodeKind::StringLiteral("これは文字列".to_string())
        // );
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
    }
}
