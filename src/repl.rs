use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::vm::{StackValue, print_value};

/// Run interactive REPL (Read-Eval-Print Loop)
pub fn run_repl() {
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
                    match crate::execute_statement(&statement_buffer, &mut variables, &mut env) {
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
fn print_variables(variables: &HashMap<String, usize>, env: &Vec<StackValue>) {
    if variables.is_empty() {
        println!("No variables defined.");
    } else {
        println!("Defined variables:");
        let mut vars: Vec<_> = variables.iter().collect();
        vars.sort_by_key(|&(_, &idx)| idx);
        for (name, idx) in vars {
            println!("{}: {})", name, print_value(&env[*idx]));
        }
    }
    println!();
}
