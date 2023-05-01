use crate::tokenizer::{Tokenizer, TokenizerTrait};

mod tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new("1");

    let result = tokenizer.tokenize();

    println!("{:?}", result.unwrap());
}
