use std::{collections::HashMap, fs::read_to_string};
mod lexer;
mod parser;
mod vm;
fn main() {
    let text = read_to_string("source.txt").unwrap();
    println!("text: {}", text);
    let tokens = lexer::tokenize(text.as_str());
    println!("tokens: {:?}", tokens);
    let ast = parser::astify(&tokens, parser::ParsingMode::BlockCode, &mut 0).unwrap();
    println!("ast: {:#?}", ast);
    let ir = vm::ir(ast, &mut HashMap::new());
    println!("ir: {:?}", ir);
}
