use std::{collections::HashMap, env::args, fs::read_to_string};

use crate::vm::StackValue;
mod ir;
mod lexer;
mod parser;
mod repl;
mod vm;

fn main() {
    let args = args().skip(1).collect::<Vec<String>>();

    if args.is_empty() {
        // Interactive REPL mode
        repl::run_repl();
    } else {
        // File execution mode
        run_files(&args);
    }
}

/// Execute a single statement
fn execute_statement(
    code: &str,
    variables: &mut HashMap<String, usize>,
    env: &mut Vec<StackValue>,
) -> Result<(), String> {
    // Remove trailing semicolon
    let code = code.trim().trim_end_matches(';').to_string() + ";";

    // Tokenize
    let tokens = lexer::tokenize(&code);

    // Parse
    let ast = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut 0)
        .map_err(|e| format!("Parse error: {}", e))?;

    // Generate IR
    let ir: Vec<ir::Command> = ir::ir(ast, variables, variables.len());

    // Execute
    vm::execute(&ir, env);

    Ok(())
}

/// Run files from command line arguments
fn run_files(args: &[String]) {
    for file in args {
        match read_to_string(format!("examples/{}", file.trim())) {
            Ok(text) => {
                let tokens = lexer::tokenize(text.as_str());
                match parser::astify(&tokens, parser::types::ParsingMode::Code, &mut 0) {
                    Ok(ast) => {
                        let vars = &mut HashMap::new();
                        let ir: Vec<ir::Command> = ir::ir(ast, vars, 0);
                        vm::execute(&ir, &mut Vec::new());
                    }
                    Err(e) => {
                        eprintln!("Parse error in {}: {}", file, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", file, e);
            }
        }
    }
}
