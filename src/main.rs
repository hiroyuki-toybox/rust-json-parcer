use crate::{
    parser::Parser,
    tokenizer::{Tokenizer, TokenizerTrait},
};

mod parser;
mod tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new(
        r#"
        {
          "key": "value",
        }
    "#,
    );

    let result = tokenizer.tokenize();

    println!("{:?}", result.clone().unwrap());

    let mut parser = Parser::new(result.unwrap());

    let result = parser.parse();

    println!("{:?}", result.unwrap());
}
