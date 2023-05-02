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

use crate::tokenizer::Token;

enum NodeKind {
    StringLiteral(String),
    NumberLiteral(f64),
    TrueKeyword,
    FalseKeyword,
    NullKeyword,
    PropertyAssignment,
    Identifier(String),
    ObjectLiteralExpression,
    ArrayLiteralExpression,
    End,
}

struct Node {
    kind: NodeKind,
    children: Vec<Node>,
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    position: usize,
}

enum ParseError {
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

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let mut root = self.parse_element()?;

        Ok()
    }

    pub fn parse_element(&mut self) -> Result<Node, ParseError> {
        let node = match self.tokens.peek() {
            Some(token) => match token {
                Token::String(val) => Node {
                    kind: NodeKind::StringLiteral(val.to_string()),
                    children: vec![],
                },
                Token::Number(val) => Node {
                    kind: NodeKind::NumberLiteral(*val),
                    children: vec![],
                },
                Token::Bool(val) => Node {
                    kind: NodeKind::TrueKeyword,
                    children: vec![],
                },
                Token::Null => Node {
                    kind: NodeKind::NullKeyword,
                    children: vec![],
                },
                Token::LeftBrace => {
                    todo!()
                }
                Token::LeftBracket => {
                    todo!()
                }
                _ => return Err(ParseError::UnexpectedToken(token.clone())),
            },
            _ => return Err(ParseError::ParseError),
        };

        Ok(node)
    }
}
