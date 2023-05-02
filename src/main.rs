use crate::tokenizer::{Tokenizer, TokenizerTrait};

mod parser;
mod tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new("\"これは文字列\"");

    let result = tokenizer.tokenize();

    println!("{:?}", result.unwrap());
}
