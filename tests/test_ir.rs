use std::collections::HashMap;
use tinywrite::{
    InterpretationError, ir, lexer,
    parser::{self, types},
};

/// Helper function to parse and generate IR from input code
fn parse_and_ir(input: &str) -> Result<Vec<ir::Command>, InterpretationError> {
    let tokens = lexer::tokenize(input);
    let mut index = 0;
    let ast = parser::astify(&tokens, types::ParsingMode::Code, &mut index)?;
    let mut variables = HashMap::new();
    let code = ir::translate(ast, &mut variables)?;
    Ok(code)
}

#[test]
fn test_ir_simple_number() {
    let commands = parse_and_ir("5;").unwrap();
    // A simple number should generate a Put command
    assert!(!commands.is_empty());
    assert_eq!(
        commands[0],
        ir::Command::Put(tinywrite::vm::StackValue::Int(5))
    );
}

#[test]
fn test_ir_simple_addition() {
    let commands = parse_and_ir("(+ 5 3);").unwrap();
    // Should contain Put commands for operands and an Add command
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Add)));
}

#[test]
fn test_ir_simple_subtraction() {
    let commands = parse_and_ir("(- 10 3);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Sub)));
}

#[test]
fn test_ir_simple_multiplication() {
    let commands = parse_and_ir("(* 4 5);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Mul)));
}

#[test]
fn test_ir_simple_division() {
    let commands = parse_and_ir("(/ 20 4);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Div)));
}

#[test]
fn test_ir_simple_modulo() {
    let commands = parse_and_ir("(% 17 5);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Mod)));
}

#[test]
fn test_ir_comparison_greater() {
    let commands = parse_and_ir("(> 10 5);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Gt)));
}

#[test]
fn test_ir_comparison_less() {
    let commands = parse_and_ir("(< 5 10);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Ls)));
}

#[test]
fn test_ir_comparison_equal() {
    let commands = parse_and_ir("(== 5 5);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Eq)));
}

#[test]
fn test_ir_comparison_not_equal() {
    let commands = parse_and_ir("(!= 5 3);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Neq)));
}

#[test]
fn test_ir_comparison_greater_or_equal() {
    let commands = parse_and_ir("(>= 5 5);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Geq)));
}

#[test]
fn test_ir_comparison_less_or_equal() {
    let commands = parse_and_ir("(<= 5 5);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Leq)));
}

#[test]
fn test_ir_logic_and() {
    let commands = parse_and_ir("(& 1 1);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::And)));
}

#[test]
fn test_ir_logic_or() {
    let commands = parse_and_ir("(| 1 0);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Or)));
}

#[test]
fn test_ir_logic_xor() {
    let commands = parse_and_ir("(^ 1 0);").unwrap();
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Xor)));
}

#[test]
fn test_ir_nested_expressions() {
    let commands = parse_and_ir("(+ (* 2 3) 4);").unwrap();
    // Should have multiple operations
    let add_count = commands
        .iter()
        .filter(|c| matches!(c, ir::Command::Add))
        .count();
    let mul_count = commands
        .iter()
        .filter(|c| matches!(c, ir::Command::Mul))
        .count();
    assert_eq!(mul_count, 1);
    assert_eq!(add_count, 1);
}

#[test]
fn test_ir_deeply_nested_expressions() {
    let commands = parse_and_ir("(+ (- (* (/ 100 2) 3) 5) 10);").unwrap();
    // Should have all four operation types
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Div)));
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Mul)));
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Sub)));
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Add)));
}

#[test]
fn test_ir_multiple_statements() -> Result<(), InterpretationError> {
    parse_and_ir("5; 10; (+ 2 3);")?;
    Ok(())
    // Each statement should be followed by Cls (clear stack)
}

#[test]
fn test_ir_code_block() {
    let commands = parse_and_ir("{ 5; 10; }").unwrap();
    // Code block should generate IR for statements
    assert!(!commands.is_empty());
}

#[test]
fn test_ir_print_command() {
    let commands = parse_and_ir("(: print 42 32);").unwrap();
    dbg!(&commands);
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Call(1))));
}

#[test]
fn test_ir_negative_numbers() {
    let commands = parse_and_ir("(-5);").unwrap();
    // Should have a Put command with negative number
    assert!(!commands.is_empty());
}

#[test]
fn test_ir_zero() {
    let commands = parse_and_ir("0;").unwrap();
    assert!(!commands.is_empty());
}

#[test]
fn test_ir_large_numbers() {
    let commands = parse_and_ir("999999;").unwrap();
    assert!(
        commands
            .iter()
            .any(|c| { matches!(c, ir::Command::Put(tinywrite::vm::StackValue::Int(999999))) })
    );
}

#[test]
fn test_ir_all_arithmetic_operators() {
    let operators = vec!["+", "-", "*", "/", "%"];
    for op in operators {
        let input = format!("({} 10 5);", op);
        let commands = parse_and_ir(&input);
        assert!(
            commands.is_ok(),
            "Failed to generate IR for operator: {}",
            op
        );
        assert!(!commands.unwrap().is_empty());
    }
}

#[test]
fn test_ir_all_comparison_operators() {
    let operators = vec![">", "<", "==", "!=", ">=", "<="];
    for op in operators {
        let input = format!("({} 10 5);", op);
        let commands = parse_and_ir(&input);
        assert!(
            commands.is_ok(),
            "Failed to generate IR for operator: {}",
            op
        );
        assert!(!commands.unwrap().is_empty());
    }
}

#[test]
fn test_ir_all_logic_operators() {
    let operators = vec!["&", "|", "^"];
    for op in operators {
        let input = format!("({} 1 0);", op);
        let commands = parse_and_ir(&input);
        assert!(
            commands.is_ok(),
            "Failed to generate IR for operator: {}",
            op
        );
        assert!(!commands.unwrap().is_empty());
    }
}

#[test]
fn test_ir_complex_nested_with_comparisons() {
    let commands = parse_and_ir("(> (+ 5 3) 7);").unwrap();
    // Should have Add and Gt commands
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Add)));
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Gt)));
}

#[test]
fn test_ir_complex_mixed_operators() {
    let commands = parse_and_ir("(& (> 10 5) (< 3 7));").unwrap();
    // Should have comparison and logic operations
    assert!(commands.iter().any(|c| matches!(c, ir::Command::And)));
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Gt)));
    assert!(commands.iter().any(|c| matches!(c, ir::Command::Ls)));
}

#[test]
fn test_ir_generated_commands_are_valid() {
    // Test that all generated commands are valid IR::Command variants
    let test_cases = vec![
        "(+ 1 2);",
        "(- 5 3);",
        "(* 2 3);",
        "(/ 10 2);",
        "(% 10 3);",
        "(> 5 3);",
        "(< 3 5);",
        "(== 5 5);",
        "(!= 5 3);",
        "(>= 5 5);",
        "(<= 3 5);",
        "(& 1 1);",
        "(| 1 0);",
        "(^ 1 0);",
    ];

    for test_case in test_cases {
        let commands = parse_and_ir(test_case).expect(&format!("Failed for: {}", test_case));
        assert!(
            !commands.is_empty(),
            "No commands generated for: {}",
            test_case
        );
    }
}
