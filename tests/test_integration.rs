use tinywrite::InterpretationError;
use tinywrite::lexer;
use tinywrite::parser;
use tinywrite::parser::types;

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
        let input = format!("({} 1 0)", op);
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
