use std::fs::read_to_string;
mod lexer;
mod parser;
fn main() {
    let text = read_to_string("source.txt").unwrap();
    let tokens = lexer::tokenize(text.as_str());
    let ast = parser::astify(&tokens, parser::ParsingMode::BlockCode , &mut 0).unwrap();
    println!("text: {}", text);
    println!("tokens: {:?}", tokens);
    println!("ast: {:?}", ast);
}

