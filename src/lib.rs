pub mod ir;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod vm;

use std::collections::HashMap;

pub use parser::ParseError;
pub use vm::TypeError;

impl From<ParseError> for InterpretationError {
    fn from(value: ParseError) -> Self {
        Self::Parsing(value)
    }
}

impl From<TypeError> for InterpretationError {
    fn from(value: TypeError) -> Self {
        Self::Execution(value)
    }
}

#[derive(Debug)]
pub enum InterpretationError {
    Parsing(ParseError),
    Execution(TypeError),
}
/// Execute a single statement (used by REPL and main)
pub fn execute_statement(
    vm: &mut vm::VM,
    debug: bool,
    variables: &mut HashMap<String, usize>,
    stmt: &str,
) -> Result<(), InterpretationError> {
    // Remove trailing semicolon
    let code = stmt.trim().trim_end_matches(';').to_string() + ";";

    // Tokenize
    let tokens = lexer::tokenize(&code);

    // Parse
    let ast = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut 0)
        .map_err(InterpretationError::Parsing)?;

    // Generate IR
    let ir: Vec<ir::Command> = ir::ir(ast, variables, variables.len());
    // Execute
    vm.code = ir;
    vm.ip = 0;
    vm.execute_program(debug)
        .map_err(InterpretationError::Execution)?;

    Ok(())
}
