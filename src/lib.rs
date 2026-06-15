pub mod ir;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod vm;

use std::collections::HashMap;

pub use ir::TranslationError;
pub use parser::ParseError;
pub use vm::ExecutionError;
impl From<TranslationError> for InterpretationError {
    fn from(value: TranslationError) -> Self {
        Self::Translating(value)
    }
}

impl From<ParseError> for InterpretationError {
    fn from(value: ParseError) -> Self {
        Self::Parsing(value)
    }
}

impl From<ExecutionError> for InterpretationError {
    fn from(value: ExecutionError) -> Self {
        Self::Execution(value)
    }
}

#[derive(Debug)]
pub enum InterpretationError {
    Parsing(ParseError),
    Translating(TranslationError),
    Execution(ExecutionError),
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
    let ir: Vec<ir::Command> = ir::translate(ast, variables)?;
    // Execute
    vm.code = ir;
    vm.ip = 0;
    vm.execute_program(debug, true)?;

    Ok(())
}
