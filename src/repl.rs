use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::{
    execute_statement,
    vm::{VM, format_value},
};

/// Run interactive REPL (Read-Eval-Print Loop)
pub fn run_repl(debug: bool) {
    println!("╔════════════════════════════════════════╗");
    println!("║     TinyWrite Interactive REPL         ║");
    println!("╚════════════════════════════════════════╝");
    println!();
    println!(
        "Type 'vars', to display variables, 'clear' to clear variables, 'exit' or 'quit' to exit."
    );
    println!();

    let mut variables: HashMap<String, usize> = HashMap::new();
    let mut line_buffer = String::new();
    let mut statement_buffer = String::new();
    let mut vm = VM::new(Vec::new());
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
                if debug {
                    dbg!(input);
                }
                // Handle special commands
                match input {
                    "exit" | "quit" => {
                        break;
                    }
                    "vars" | "v" => {
                        print_variables(&variables, &vm);
                        continue;
                    }
                    "stack" | "s" => {
                        print_stack(&vm, None);
                        continue;
                    }
                    "heap" | "h" => {
                        for (i, collection) in vm.heap.values().enumerate() {
                            println!("{i}: {:?}", collection)
                        }
                        continue;
                    }
                    "refs" => {
                        vm.debug_refs(debug);
                        continue;
                    }
                    "" => continue,
                    _ => (),
                }

                // Accumulate input until we have a complete statement
                statement_buffer.push_str(input);
                statement_buffer.push('\n');

                // Check if statement is complete (ends with semicolon)
                if statement_buffer.trim().ends_with(';') {
                    // Execute the statement
                    match execute_statement(&mut vm, debug, &mut variables, &statement_buffer) {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("Error: {:?}", err);
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
/// prints top 'num' elements from the stack. if num is none, prints whole stack
fn print_stack(vm: &VM, num: Option<usize>) {
    let num = num.unwrap_or(vm.stack.len());
    for value in vm.stack.iter().rev().take(num).rev() {
        print!("{} ", format_value(&value, vm))
    }
    println!()
}
fn print_variables(variables: &HashMap<String, usize>, vm: &VM) {
    if variables.is_empty() {
        println!("No variables defined.");
    } else {
        println!("Defined variables:");
        for (name, idx) in variables.iter() {
            eprintln!("{:?}", vm.vars);
            println!("{}: {}", name, format_value(&vm.vars[*idx], vm));
        }
    }
}
