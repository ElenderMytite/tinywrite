pub mod ir;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod vm;

use std::collections::HashMap;

pub use ir::TranslationError;
pub use lexer::SyntaxError;
pub use parser::ParseError;
pub use vm::ExecutionError;
impl From<SyntaxError> for InterpretationError {
    fn from(value: SyntaxError) -> Self {
        Self::Tokenizing(value)
    }
}
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
    Tokenizing(SyntaxError),
    Parsing(ParseError),
    Translating(TranslationError),
    Execution(ExecutionError),
}
/// Execute a single statement (used by REPL and main)
pub fn execute_statement(
    vm: &mut vm::VM,
    debug: bool,
    variables: &mut HashMap<String, usize>,
    stmt: Vec<u8>,
) -> Result<(), InterpretationError> {
    // Tokenize
    let tokens = lexer::tokenize(&stmt)?;
    if debug {
        eprintln!("tokens: {:?}", &tokens);
    }
    // Parse
    let ast = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut 0)
        .map_err(InterpretationError::Parsing)?;

    // Generate IR
    let (ir, strings) = ir::translate(ast, variables)?;
    if debug {
        dbg!(&ir);
    }
    // Execute
    vm.set_environment(ir, strings);
    vm.execute_program(debug, true)?;

    Ok(())
}
