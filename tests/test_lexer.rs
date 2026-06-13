use tinywrite::lexer;

#[test]
fn test_lexer_simple_numbers() {
    let input = "42 100 -5";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["42", "100", "-5"]);
}

#[test]
fn test_lexer_variable_names() {
    let input = "x my_var result counter_1";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["x", "my_var", "result", "counter_1"]);
}

#[test]
fn test_lexer_arithmetic_operators() {
    let input = "+ - * / %";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["+", "-", "*", "/", "%"]);
}

#[test]
fn test_lexer_comparison_operators() {
    let input = "== != > < >= <=";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["==", "!=", ">", "<", ">=", "<="]);
}

#[test]
fn test_lexer_logical_operators() {
    let input = "& | ^ ! !& !|";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["&", "|", "^", "!", "!&", "!|"]);
}

#[test]
fn test_lexer_parentheses_and_braces() {
    let input = "( ) { } [ ]";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["(", ")", "{", "}", "[", "]"]);
}

#[test]
fn test_lexer_comments_full_line() {
    let input = "x = 5; # this is a comment";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["x", "=", "5", ";"]);
}

#[test]
fn test_lexer_comments_ignored() {
    let input = "; entire line is comment\nx = 5;";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["x", "=", "5", ";"]);
}

#[test]
fn test_lexer_mixed_expression() {
    let input = "(+ 5 3)";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["(", "+", "5", "3", ")"]);
}

#[test]
fn test_lexer_negative_numbers() {
    let input = "-42 -100 -999";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["-42", "-100", "-999"]);
}

#[test]
fn test_lexer_semicolon_separator() {
    let input = "x = 5;\n y = 10;\n";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["x", "=", "5", ";", "y", "=", "10", ";"]);
}

#[test]
fn test_lexer_underscores_in_identifiers() {
    let input = "_var var_1 _my_var_";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["_var", "var_1", "_my_var_"]);
}

#[test]
fn test_lexer_empty_input() {
    let input = "";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens.len(), 0);
}

#[test]
fn test_lexer_whitespace_only() {
    let input = "   \t\n  ";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens.len(), 0);
}

#[test]
fn test_lexer_complex_expression() {
    let input = "result = (+ (* 10 20) (- 30 5));";
    let tokens = lexer::tokenize(input);
    let expected = vec![
        "result", "=", "(", "+", "(", "*", "10", "20", ")", "(", "-", "30", "5", ")", ")", ";",
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_assignment_operator() {
    let input = "x = 42;";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["x", "=", "42", ";"]);
}

#[test]
fn test_lexer_multiple_operators_no_spaces() {
    let input = "(+5(3)";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["(", "+", "5", "(", "3", ")"]);
}

#[test]
fn test_lexer_vector_syntax() {
    let input = "v = (1 2 3,,);";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["v", "=", "(", "1", "2", "3", ",,", ")", ";"]);
}

#[test]
fn test_lexer_zero() {
    let input = "0";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["0"]);
}

#[test]
fn test_lexer_large_numbers() {
    let input = "999999 123456789";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["999999", "123456789"]);
}

#[test]
fn test_lexer_alphanumeric_identifiers() {
    let input = "var1 var2test abc123xyz";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["var1", "var2test", "abc123xyz"]);
}

#[test]
fn test_lexer_all_comparison_operators() {
    let input = "> < >= <= == !=";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec![">", "<", ">=", "<=", "==", "!="]);
}

#[test]
fn test_lexer_assignment_with_expression() {
    let input = "x = (+ 5 3);";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["x", "=", "(", "+", "5", "3", ")", ";"]);
}

#[test]
fn test_lexer_numbers_with_underscores() {
    let input = "1_000 5_0_0";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["1000", "500"]);
}

#[test]
fn test_lexer_mixed_case_identifiers() {
    let input = "Var VAR var VaR";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["Var", "VAR", "var", "VaR"]);
}

#[test]
fn test_lexer_colon_operator() {
    let tokens = lexer::tokenize("v :len;");
    assert_eq!(tokens, vec!["v", ":", "len", ";"]);
}

#[test]
fn test_lexer_long_variable_name() {
    let input = "this_is_a_very_long_variable_name_for_testing";
    let tokens = lexer::tokenize(input);
    assert_eq!(
        tokens,
        vec!["this_is_a_very_long_variable_name_for_testing"]
    );
}

#[test]
fn test_lexer_comment_with_special_chars() {
    let input = "x = 5; comment with #!@#$%^&*()";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["x", "=", "5", ";"]);
}

#[test]
fn test_lexer_expression_without_spaces() {
    let input = "(+5(3)(7))";
    let tokens = lexer::tokenize(input);
    assert_eq!(
        tokens,
        vec!["(", "+", "5", "(", "3", ")", "(", "7", ")", ")"]
    );
}

#[test]
fn test_lexer_vector_operations() {
    let input = "v :push 4 :pop :get 0";
    let tokens = lexer::tokenize(input);
    assert_eq!(
        tokens,
        vec!["v", ":", "push", "4", ":", "pop", ":", "get", "0"]
    );
}

#[test]
fn test_lexer_comparison_chain() {
    let input = "(> 10 5) (< 5 10)";
    let tokens = lexer::tokenize(input);
    assert_eq!(
        tokens,
        vec!["(", ">", "10", "5", ")", "(", "<", "5", "10", ")"]
    );
}

#[test]
fn test_lexer_modulo_operator() {
    let input = "(% 17 5)";
    let tokens = lexer::tokenize(input);
    assert_eq!(tokens, vec!["(", "%", "17", "5", ")"]);
}
