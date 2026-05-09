use std::{collections::HashMap, fs::read_to_string};
mod ir;
mod lexer;
mod parser;
fn main() {
    let text = read_to_string("source.txt").unwrap();
    println!("text: {}", text);
    let tokens = lexer::tokenize(text.as_str());
    println!("tokens: {:?}", tokens);
    let ast = parser::astify(&tokens, parser::ParsingMode::BlockCode, &mut 0).unwrap();
    println!("ast: {:#?}", ast);
    let ir = ir::ir(ast, &mut HashMap::new());
    println!("ir: {:?}", ir);
}
