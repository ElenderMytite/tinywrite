use std::{collections::HashMap, env::args, fs::read_to_string};
use tinywrite::{
    InterpretationError, ir, lexer, parser, repl,
    vm::{TypeError, VM},
};
fn main() -> Result<(), InterpretationError> {
    let args: Vec<String> = args().skip(1).collect();

    // Check for --debug flag
    let debug = args.contains(&"--debug".to_string());

    // Filter out the --debug flag from file arguments
    let file_args: Vec<String> = args.into_iter().filter(|arg| arg != "--debug").collect();

    if file_args.is_empty() {
        // Interactive REPL mode
        if debug {
            println!("[DEBUG MODE ENABLED]");
        }
        repl::run_repl();
    } else {
        // File execution mode
        if debug {
            println!("[DEBUG MODE ENABLED]");
        }
        run_files(&file_args, debug)?;
    }
    Ok(())
}

/// Run files from command line arguments
fn run_files(args: &[String], debug: bool) -> Result<(), TypeError> {
    for file in args {
        match read_to_string(format!("examples/{}", file.trim())) {
            Ok(text) => {
                let tokens = lexer::tokenize(text.as_str());
                match parser::astify(&tokens, parser::types::ParsingMode::Code, &mut 0) {
                    Ok(ast) => {
                        let vars = &mut HashMap::new();
                        let ir: Vec<ir::Command> = ir::ir(ast, vars, 0);
                        if debug {
                            println!(
                                "{}",
                                ir.iter()
                                    .enumerate()
                                    .map(|(k, v)| format!("{k}: {v:?}"))
                                    .collect::<Vec<String>>()
                                    .join("\n")
                            );
                        }
                        let mut vm = VM::new(ir);
                        vm.execute_program(debug)?;
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
    Ok(())
}
