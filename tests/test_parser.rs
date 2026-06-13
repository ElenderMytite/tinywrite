use tinywrite::parser;
#[test]
fn test_parser_simple_number() {
    let tokens = vec!["42".to_string()];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_simple_variable() {
    let tokens = vec!["x".to_string()];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_addition_expression() {
    let tokens = vec![
        "(".to_string(),
        "+".to_string(),
        "5".to_string(),
        "3".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_subtraction_expression() {
    let tokens = vec![
        "(".to_string(),
        "-".to_string(),
        "10".to_string(),
        "3".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_multiplication_expression() {
    let tokens = vec![
        "(".to_string(),
        "*".to_string(),
        "4".to_string(),
        "5".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_division_expression() {
    let tokens = vec![
        "(".to_string(),
        "/".to_string(),
        "20".to_string(),
        "4".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_modulo_expression() {
    let tokens = vec![
        "(".to_string(),
        "%".to_string(),
        "17".to_string(),
        "5".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_greater_than() {
    let tokens = vec![
        "(".to_string(),
        ">".to_string(),
        "10".to_string(),
        "5".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_less_than() {
    let tokens = vec![
        "(".to_string(),
        "<".to_string(),
        "5".to_string(),
        "10".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_equal() {
    let tokens = vec![
        "(".to_string(),
        "==".to_string(),
        "5".to_string(),
        "5".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_not_equal() {
    let tokens = vec![
        "(".to_string(),
        "!=".to_string(),
        "5".to_string(),
        "6".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_logical_and() {
    let tokens = vec![
        "(".to_string(),
        "&".to_string(),
        "1".to_string(),
        "1".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_logical_or() {
    let tokens = vec![
        "(".to_string(),
        "|".to_string(),
        "1".to_string(),
        "0".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_logical_xor() {
    let tokens = vec![
        "(".to_string(),
        "^".to_string(),
        "1".to_string(),
        "0".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_variable_assignment() {
    let tokens = vec![
        "x".to_string(),
        "=".to_string(),
        "42".to_string(),
        ";".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_assignment_with_expression() {
    let tokens = vec![
        "x".to_string(),
        "=".to_string(),
        "(".to_string(),
        "+".to_string(),
        "5".to_string(),
        "3".to_string(),
        ")".to_string(),
        ";".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_nested_expressions() {
    let tokens = vec![
        "(".to_string(),
        "+".to_string(),
        "(".to_string(),
        "*".to_string(),
        "2".to_string(),
        "3".to_string(),
        ")".to_string(),
        "4".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_multiple_statements() {
    let tokens = vec![
        "x".to_string(),
        "=".to_string(),
        "5".to_string(),
        ";".to_string(),
        "y".to_string(),
        "=".to_string(),
        "10".to_string(),
        ";".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_empty_input() {
    let tokens: Vec<String> = vec![];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_greater_or_equal() {
    let tokens = vec![
        "(".to_string(),
        ">=".to_string(),
        "10".to_string(),
        "5".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_less_or_equal() {
    let tokens = vec![
        "(".to_string(),
        "<=".to_string(),
        "5".to_string(),
        "10".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_negative_number() {
    let tokens = vec!["-42".to_string()];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_zero() {
    let tokens = vec!["0".to_string()];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_large_number() {
    let tokens = vec!["999999".to_string()];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_variable_name_with_underscores() {
    let tokens = vec!["my_var_1".to_string()];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_three_operand_addition() {
    let tokens = vec![
        "(".to_string(),
        "+".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_code_block() {
    let tokens = vec![
        "{".to_string(),
        "x".to_string(),
        "=".to_string(),
        "5".to_string(),
        ";".to_string(),
        "}".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_nand_operator() {
    let tokens = vec![
        "(".to_string(),
        "!&".to_string(),
        "1".to_string(),
        "1".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_nor_operator() {
    let tokens = vec![
        "(".to_string(),
        "!|".to_string(),
        "1".to_string(),
        "0".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_complex_nested() {
    let tokens = vec![
        "(".to_string(),
        "+".to_string(),
        "(".to_string(),
        "*".to_string(),
        "(".to_string(),
        "-".to_string(),
        "10".to_string(),
        "5".to_string(),
        ")".to_string(),
        "2".to_string(),
        ")".to_string(),
        "3".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}

#[test]
fn test_parser_vector_pack() {
    let tokens = vec![
        "(".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        ",,".to_string(),
        ")".to_string(),
    ];
    let mut index = 0;
    let result = parser::astify(&tokens, parser::types::ParsingMode::Code, &mut index);
    assert!(result.is_ok());
}
