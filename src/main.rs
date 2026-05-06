use std::fs::read_to_string;
mod lexer;
fn main() {
    let text = read_to_string("source.txt").unwrap();
    let tokens = lexer::tokenize(text.as_str());
    
    println!("text: {}", text);
    println!("tokens: {:?}", tokens)
}

