use rhinio::{InterpretationError, ir, lexer, parser, repl, vm::VM};
use std::{collections::HashMap, env::args, fs::read};
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
        repl::run_repl(debug);
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
fn run_files(args: &[String], debug: bool) -> Result<(), InterpretationError> {
    for file in args {
        if debug {
            println!("{}", std::iter::repeat('-').take(64).collect::<String>());
            println!("executing {}", file);
        }
        match read(format!("{}", file.trim())) {
            Ok(text) => {
                let tokens = lexer::tokenize(&text)?;
                match parser::astify(&tokens, parser::types::ParsingMode::Code, &mut 0) {
                    Ok(ast) => {
                        let vars = &mut HashMap::new();
                        let (ir, strings) = ir::translate(ast, vars)?;
                        let mut vm = VM::new(ir, strings);
                        vm.execute_program(debug, true)?;
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
        if debug {
            println!("successfully executed {}", file);
            println!("{}", std::iter::repeat('-').take(64).collect::<String>());
        }
    }
    Ok(())
}
