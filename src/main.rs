use std::{collections::HashMap, env::args, fs::read_to_string};
mod ir;
mod lexer;
mod parser;
mod vm;
fn main() {
    let args = args().skip(1).collect::<Vec<String>>();
    for file in args {
        // println!("executing file: {}", file.trim());
        let text = read_to_string(file.trim()).unwrap();
        // println!("text: {}", text);
        let tokens = lexer::tokenize(text.as_str());
        // println!("tokens: {:?}", tokens);
        let ast = parser::astify(&tokens, parser::ParsingMode::Code, &mut 0).unwrap();
        // println!("ast: {:#?}", ast);
        let vars = &mut HashMap::new();
        let ir: Vec<ir::Command> = ir::ir(ast, vars, 0);
        // for (i, command) in ir.iter().enumerate() {
        //     println!("{}: {:?}", i, command);
        // }
        vm::execute(&ir, Some(vars.len()));
    }
}
