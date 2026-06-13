use std::collections::HashMap;
use tinywrite::{
    InterpretationError, ir, lexer, parser,
    parser::types,
    vm::{StackValue, VM},
};

/// Helper to execute complete program and get stack result
fn execute_program(input: &str) -> Result<Option<StackValue>, InterpretationError> {
    let tokens = lexer::tokenize(input);
    let mut index = 0;
    let ast = parser::astify(&tokens, types::ParsingMode::Code, &mut index)?;

    let mut variables = HashMap::new();
    let commands = ir::ir(ast, &mut variables, 0);

    let mut vm = VM::new(commands);
    vm.execute_program(false)?;

    Ok(vm.stack.pop())
}

#[test]
fn test_integration_lexer_to_parser_simple_addition() {
    let input = "(+ 5 3)";
    let tokens = lexer::tokenize(input);

    assert_eq!(tokens, vec!["(", "+", "5", "3", ")"]);

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_lexer_to_parser_variable_assignment() {
    let input = "x = 42;";
    let tokens = lexer::tokenize(input);

    assert_eq!(tokens, vec!["x", "=", "42", ";"]);

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_lexer_to_parser_complex_expression() {
    let input = "result = (+ (* 10 20) (- 30 5));";
    let tokens = lexer::tokenize(input);

    // Verify tokenization
    assert!(tokens.len() > 0);
    assert_eq!(tokens[0], "result");
    assert_eq!(tokens[1], "=");

    // Verify parsing
    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_lexer_to_parser_all_arithmetic_operators() {
    let operators = vec!["+", "-", "*", "/", "%"];

    for op in operators {
        let input = format!("({} 10 5)", op);
        let tokens = lexer::tokenize(&input);

        // Verify tokens are correct
        assert!(tokens.contains(&op.to_string()));

        // Verify parsing succeeds
        let mut index = 0;
        let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
        assert!(result.is_ok(), "Failed for operator: {}", op);
    }
}

#[test]
fn test_integration_lexer_to_parser_all_comparison_operators() {
    let operators = vec![">", "<", "==", "!=", ">=", "<="];

    for op in operators {
        let input = format!("({} 10 5)", op);
        let tokens = lexer::tokenize(&input);

        // Verify tokens are correct
        assert!(tokens.contains(&op.to_string()));

        // Verify parsing succeeds
        let mut index = 0;
        let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
        assert!(result.is_ok(), "Failed for operator: {}", op);
    }
}

#[test]
fn test_integration_lexer_to_parser_logical_operators() {
    let operators = vec!["&", "|", "^", "!", "!&", "!|"];

    for op in operators {
        let input = format!("({} $true $false)", op);
        let tokens = lexer::tokenize(&input);

        // Verify tokens are correct
        assert!(tokens.contains(&op.to_string()));

        // Verify parsing succeeds
        let mut index = 0;
        let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
        assert!(result.is_ok(), "Failed for operator: {}", op);
    }
}

#[test]
fn test_integration_comments_stripped_before_parsing() -> Result<(), InterpretationError> {
    let input = "x = 5; this is a comment\n";
    let tokens = lexer::tokenize(input);

    // Comments should not appear in tokens
    assert!(!tokens.contains(&"this".to_string()));

    let mut index = 0;
    parser::astify(&tokens, types::ParsingMode::Code, &mut index)?;
    Ok(())
}

#[test]
fn test_integration_negative_numbers_in_expressions() {
    let input = "(+ -5 -10)";
    let tokens = lexer::tokenize(input);

    assert_eq!(tokens, vec!["(", "+", "-5", "-10", ")"]);

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_nested_expressions_parsing() {
    let input = "(+ (* 2 3) (- 10 4))";
    let tokens = lexer::tokenize(input);

    // Verify structure
    assert_eq!(tokens[0], "(");
    assert_eq!(tokens[1], "+");

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_multiple_statements() {
    let input = "x = 10;\n y = 20;\n (+ x y);\n";
    let tokens = lexer::tokenize(input);

    // Verify tokenization captured all parts
    assert!(tokens.contains(&"x".to_string()));
    assert!(tokens.contains(&"y".to_string()));
    assert!(tokens.contains(&"+".to_string()));

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_underscores_in_variables() {
    let input = "my_var_1 = (+ value_2 result_3);";
    let tokens = lexer::tokenize(input);

    // Verify underscores preserved in identifiers
    assert!(tokens.contains(&"my_var_1".to_string()));
    assert!(tokens.contains(&"value_2".to_string()));

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_zero_handling() {
    let input = "(+ 0 0)";
    let tokens = lexer::tokenize(input);

    assert_eq!(tokens, vec!["(", "+", "0", "0", ")"]);

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_large_numbers() {
    let input = "(* 999999 123456789)";
    let tokens = lexer::tokenize(input);

    assert_eq!(tokens, vec!["(", "*", "999999", "123456789", ")"]);

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_vector_creation() {
    let input = "v = (1 2 3,,);";
    let tokens = lexer::tokenize(input);

    // Verify tokens
    assert_eq!(tokens[0], "v");
    assert_eq!(tokens[1], "=");
    assert!(tokens.contains(&",,".to_string()));

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_code_block() {
    let input = "{\n\t x = 5;\n\t y = 10;\n}"; // two statements on the same line intentionally not allowed
    let tokens = lexer::tokenize(input);
    dbg!(tokens.clone());

    assert_eq!(tokens[0], "{");
    assert_eq!(tokens[tokens.len() - 1], "}");

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_mixed_operators_and_variables() {
    let input = "a = 5;\n b = 10;\n c = (+ a b);\n d = (> c 10);\n";
    let tokens = lexer::tokenize(input);

    // Verify all variables present
    assert!(tokens.contains(&"a".to_string()));
    assert!(tokens.contains(&"b".to_string()));
    assert!(tokens.contains(&"c".to_string()));
    assert!(tokens.contains(&"d".to_string()));

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_deeply_nested_expressions() {
    let input = "(+ (- (* (/ 100 2) 3) 5) 10)";
    let tokens = lexer::tokenize(input);

    // Verify structure is maintained
    let open_count = tokens.iter().filter(|t| t.as_str() == "(").count();
    let close_count = tokens.iter().filter(|t| t.as_str() == ")").count();
    assert_eq!(open_count, close_count);

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_all_operators_in_sequence() {
    let input = "(+ 1 2) (- 3 4) (* 5 6) (/ 7 8) (% 9 10) (> 11 12) (< 13 14) (& 1 1) (| 0 1)";
    let tokens = lexer::tokenize(input);

    // Verify all operators are present
    assert!(tokens.contains(&"+".to_string()));
    assert!(tokens.contains(&"-".to_string()));
    assert!(tokens.contains(&"*".to_string()));
    assert!(tokens.contains(&"/".to_string()));
    assert!(tokens.contains(&"%".to_string()));

    let mut index = 0;
    let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_integration_whitespace_normalization() {
    let input1 = "(+ 5 3)";
    let input2 = "(  +   5   3  )";
    let input3 = "(\n+\n5\n3\n)";

    let tokens1 = lexer::tokenize(input1);
    let tokens2 = lexer::tokenize(input2);
    let tokens3 = lexer::tokenize(input3);

    // All should produce the same tokens
    assert_eq!(tokens1, tokens2);
    assert_eq!(tokens1, tokens3);
}

#[test]
fn test_integration_single_vs_multi_char_operators() {
    let inputs = vec![
        ("(== 5 5)", vec!["(", "==", "5", "5", ")"]),
        ("(> 5 3)", vec!["(", ">", "5", "3", ")"]),
        ("(!= 5 3)", vec!["(", "!=", "5", "3", ")"]),
        ("(>= 5 5)", vec!["(", ">=", "5", "5", ")"]),
        ("(<= 5 5)", vec!["(", "<=", "5", "5", ")"]),
    ];

    for (input, expected) in inputs {
        let tokens = lexer::tokenize(input);
        assert_eq!(tokens, expected, "Failed for input: {}", input);

        let mut index = 0;
        let result = parser::astify(&tokens, types::ParsingMode::Code, &mut index);
        assert!(result.is_ok(), "Parse failed for: {}", input);
    }
}

// ====== FULL PROGRAM EXECUTION TESTS ======

#[test]
fn test_integration_execute_simple_number() -> Result<(), InterpretationError> {
    let result = execute_program("5;")?;
    assert_eq!(result, Some(StackValue::Int(5)));
    Ok(())
}

#[test]
fn test_integration_execute_simple_addition() -> Result<(), InterpretationError> {
    let result = execute_program("(+ 5 3);")?;
    assert_eq!(result, Some(StackValue::Int(8)));
    Ok(())
}

#[test]
fn test_integration_execute_simple_subtraction() -> Result<(), InterpretationError> {
    let result = execute_program("(- 10 3);")?;
    assert_eq!(result, Some(StackValue::Int(7)));
    Ok(())
}

#[test]
fn test_integration_execute_simple_multiplication() -> Result<(), InterpretationError> {
    let result = execute_program("(* 4 5);")?;
    assert_eq!(result, Some(StackValue::Int(20)));
    Ok(())
}

#[test]
fn test_integration_execute_simple_division() -> Result<(), InterpretationError> {
    let result = execute_program("(/ 20 4);")?;
    assert_eq!(result, Some(StackValue::Int(5)));
    Ok(())
}

#[test]
fn test_integration_execute_simple_modulo() -> Result<(), InterpretationError> {
    let result = execute_program("(% 17 5);")?;
    assert_eq!(result, Some(StackValue::Int(2)));
    Ok(())
}

#[test]
fn test_integration_execute_nested_expressions() -> Result<(), InterpretationError> {
    let result = execute_program("(+ (* 2 3) 4);")?;
    assert_eq!(result, Some(StackValue::Int(10)));
    Ok(())
}

#[test]
fn test_integration_execute_deeply_nested_arithmetic() -> Result<(), InterpretationError> {
    let result = execute_program("(+ (- (* (/ 100 2) 3) 5) 10);")?;
    assert_eq!(result, Some(StackValue::Int(155)));
    Ok(())
}

#[test]
fn test_integration_execute_comparison_greater_true() -> Result<(), InterpretationError> {
    let result = execute_program("(> 10 5);")?;
    assert_eq!(result, Some(StackValue::Bool(true)));
    Ok(())
}

#[test]
fn test_integration_execute_comparison_greater_false() -> Result<(), InterpretationError> {
    let result = execute_program("(> 5 10);")?;
    assert_eq!(result, Some(StackValue::Bool(false)));
    Ok(())
}

#[test]
fn test_integration_execute_comparison_less_true() -> Result<(), InterpretationError> {
    let result = execute_program("(< 5 10);")?;
    assert_eq!(result, Some(StackValue::Bool(true)));
    Ok(())
}

#[test]
fn test_integration_execute_comparison_equal_true() -> Result<(), InterpretationError> {
    let result = execute_program("(== 5 5);")?;
    assert_eq!(result, Some(StackValue::Bool(true)));
    Ok(())
}

#[test]
fn test_integration_execute_comparison_not_equal_true() -> Result<(), InterpretationError> {
    let result = execute_program("(!= 5 3);")?;
    assert_eq!(result, Some(StackValue::Bool(true)));
    Ok(())
}

#[test]
fn test_integration_execute_comparison_greater_or_equal() -> Result<(), InterpretationError> {
    let result = execute_program("(>= 5 5);")?;
    assert_eq!(result, Some(StackValue::Bool(true)));
    Ok(())
}

#[test]
fn test_integration_execute_comparison_less_or_equal() -> Result<(), InterpretationError> {
    let result = execute_program("(<= 5 5);")?;
    assert_eq!(result, Some(StackValue::Bool(true)));
    Ok(())
}

#[test]
fn test_integration_execute_logic_and_both_true() -> Result<(), InterpretationError> {
    let result = execute_program("(& $true $true);")?;
    assert_eq!(result, Some(StackValue::Bool(true)));
    Ok(())
}

#[test]
fn test_integration_execute_logic_or_both_true() -> Result<(), InterpretationError> {
    let result = execute_program("(| $true $true);")?;
    assert_eq!(result, Some(StackValue::Bool(true)));
    Ok(())
}

#[test]
fn test_integration_execute_logic_or_mixed() -> Result<(), InterpretationError> {
    let result = execute_program("(| $false $true);")?;
    assert_eq!(result, Some(StackValue::Bool(true)));
    Ok(())
}

#[test]
fn test_integration_execute_logic_xor_different() -> Result<(), InterpretationError> {
    let result = execute_program("(^ $true $false);")?;
    assert_eq!(result, Some(StackValue::Bool(true)));
    Ok(())
}

#[test]
fn test_integration_execute_logic_xor_same() -> Result<(), InterpretationError> {
    let result = execute_program("(^ $true $true);")?;
    assert_eq!(result, Some(StackValue::Bool(false)));
    Ok(())
}

#[test]
fn test_integration_execute_comparison_with_arithmetic() -> Result<(), InterpretationError> {
    let result = execute_program("(> (+ 5 3) 7);")?;
    assert_eq!(result, Some(StackValue::Bool(true)));
    Ok(())
}

#[test]
fn test_integration_execute_multiple_comparisons() -> Result<(), InterpretationError> {
    let result = execute_program("(& (> 10 5) (< 3 7));")?;
    assert_eq!(result, Some(StackValue::Bool(true)));
    Ok(())
}

#[test]
fn test_integration_execute_negative_numbers() -> Result<(), InterpretationError> {
    let result = execute_program("(+ -5 3);")?;
    assert_eq!(result, Some(StackValue::Int(-2)));
    Ok(())
}

#[test]
fn test_integration_execute_subtraction_negative_result() -> Result<(), InterpretationError> {
    let result = execute_program("(- 3 5);")?;
    assert_eq!(result, Some(StackValue::Int(-2)));
    Ok(())
}

#[test]
fn test_integration_execute_division_integer() -> Result<(), InterpretationError> {
    let result = execute_program("(/ 7 2);")?;
    assert_eq!(result, Some(StackValue::Int(3)));
    Ok(())
}

#[test]
fn test_integration_execute_modulo_with_remainder() -> Result<(), InterpretationError> {
    let result = execute_program("(% 10 3);")?;
    assert_eq!(result, Some(StackValue::Int(1)));
    Ok(())
}

#[test]
fn test_integration_execute_modulo_no_remainder() -> Result<(), InterpretationError> {
    let result = execute_program("(% 10 5);")?;
    assert_eq!(result, Some(StackValue::Int(0)));
    Ok(())
}

#[test]
fn test_integration_execute_very_deeply_nested() -> Result<(), InterpretationError> {
    let result = execute_program("(+ (+ (+ (+ 1 1) 1) 1) 1);")?;
    assert_eq!(result, Some(StackValue::Int(5)));
    Ok(())
}

#[test]
fn test_integration_execute_chain_of_multiplications() -> Result<(), InterpretationError> {
    let result = execute_program("(* (* 2 3) 4);")?;
    assert_eq!(result, Some(StackValue::Int(24)));
    Ok(())
}

#[test]
fn test_integration_execute_mixed_arithmetic() -> Result<(), InterpretationError> {
    let result = execute_program("(+ (- 10 3) (* 2 4));")?;
    assert_eq!(result, Some(StackValue::Int(15)));
    Ok(())
}

#[test]
fn test_integration_execute_long_calculation_chain() -> Result<(), InterpretationError> {
    let result = execute_program("(* (- 20 10) (+ 2 3));")?;
    assert_eq!(result, Some(StackValue::Int(50)));
    Ok(())
}

#[test]
fn test_integration_execute_zero() -> Result<(), InterpretationError> {
    let result = execute_program("0;")?;
    assert_eq!(result, Some(StackValue::Int(0)));
    Ok(())
}

#[test]
fn test_integration_execute_large_number() -> Result<(), InterpretationError> {
    let result = execute_program("999999;")?;
    assert_eq!(result, Some(StackValue::Int(999999)));
    Ok(())
}

#[test]
fn test_integration_execute_all_arithmetic_operators() -> Result<(), InterpretationError> {
    let test_cases = vec![
        ("(+ 5 3);", StackValue::Int(8)),
        ("(- 10 3);", StackValue::Int(7)),
        ("(* 4 5);", StackValue::Int(20)),
        ("(/ 20 4);", StackValue::Int(5)),
        ("(% 17 5);", StackValue::Int(2)),
    ];

    for (code, expected) in test_cases {
        let result = execute_program(code)?;
        assert_eq!(result, Some(expected), "Failed for: {}", code);
    }
    Ok(())
}

#[test]
fn test_integration_execute_all_comparison_operators() -> Result<(), InterpretationError> {
    let test_cases = vec![
        ("(> 10 5);", StackValue::Bool(true)),
        ("(> 4 5);", StackValue::Bool(false)),
        ("(< 5 10);", StackValue::Bool(true)),
        ("(< 5 4);", StackValue::Bool(false)),
        ("(== 5 5);", StackValue::Bool(true)),
        ("(== 5 7);", StackValue::Bool(false)),
        ("(!= 5 3);", StackValue::Bool(true)),
        ("(!= 5 5);", StackValue::Bool(false)),
        ("(>= 5 5);", StackValue::Bool(true)),
        ("(>= 1 5);", StackValue::Bool(false)),
        ("(<= 3 5);", StackValue::Bool(true)),
        ("(<= 6 5);", StackValue::Bool(false)),
    ];

    for (code, expected) in test_cases {
        let result = execute_program(code)?;
        assert_eq!(result, Some(expected), "Failed for: {}", code);
    }
    Ok(())
}

#[test]
fn test_integration_execute_all_logic_operators() -> Result<(), InterpretationError> {
    let test_cases = vec![
        ("(& $true $true);", StackValue::Bool(true)),
        ("(| $true $false);", StackValue::Bool(true)),
        ("(^ $true $false);", StackValue::Bool(true)),
    ];

    for (code, expected) in test_cases {
        let result = execute_program(code)?;
        assert_eq!(result, Some(expected), "Failed for: {}", code);
    }
    Ok(())
}
