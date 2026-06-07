use std::{
    collections::HashMap,
    env::args,
    fs::read_to_string,
    io::{self, Write},
};

use crate::vm::StackValue;
mod ir;
mod lexer;
mod parser;
mod vm;

fn main() {
    let args = args().skip(1).collect::<Vec<String>>();

    if args.is_empty() {
        // Interactive REPL mode
        run_repl();
    } else {
        // File execution mode
        run_files(&args);
    }
}

/// Run interactive REPL (Read-Eval-Print Loop)
fn run_repl() {
    println!("╔════════════════════════════════════════╗");
    println!("║     TinyWrite Interactive REPL         ║");
    println!("╚════════════════════════════════════════╝");
    println!();
    println!(
        "Type 'vars', to display variables, 'clear' to clear variables, 'exit' or 'quit' to exit."
    );
    println!();

    let mut variables: HashMap<String, usize> = HashMap::new();
    let mut env: Vec<StackValue> = Vec::new();
    let mut line_buffer = String::new();
    let mut statement_buffer = String::new();
    loop {
        // Show prompt
        if statement_buffer.is_empty() {
            print!(">> ");
        } else {
            print!(".. ");
        }
        io::stdout().flush().unwrap();
        // Read input
        line_buffer.clear();
        match io::stdin().read_line(&mut line_buffer) {
            Ok(0) => {
                // EOF reached (Ctrl+D)
                println!();
                break;
            }
            Ok(_) => {
                let input = line_buffer.trim();

                // Handle special commands
                match input {
                    "exit" | "quit" => {
                        println!("Goodbye!");
                        break;
                    }
                    "clear" => {
                        variables.clear();
                        env.clear();
                        println!("Variables cleared.");
                        continue;
                    }
                    "vars" | "v" => {
                        print_variables(&variables, &env);
                        continue;
                    }
                    "" => continue,
                    _ => (),
                }

                // Accumulate input until we have a complete statement
                statement_buffer.push_str(input);
                statement_buffer.push(' ');

                // Check if statement is complete (ends with semicolon)
                if statement_buffer.trim().ends_with(';') {
                    // Execute the statement
                    match execute_statement(&statement_buffer, &mut variables, &mut env) {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("Error: {}", err);
                        }
                    }
                    statement_buffer.clear();
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
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
/// Print all defined variables
fn print_variables(variables: &HashMap<String, usize>, env: &Vec<StackValue>) {
    if variables.is_empty() {
        println!("No variables defined.");
    } else {
        println!("Defined variables:");
        let mut vars: Vec<_> = variables.iter().collect();
        vars.sort_by_key(|&(_, &idx)| idx);
        for (name, idx) in vars {
            println!("{}: {})", name, vm::print_value(&env[*idx]));
        }
    }
    println!();
}
