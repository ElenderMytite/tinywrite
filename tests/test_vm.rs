use std::collections::HashMap;
use tinywrite::{
    InterpretationError, ir, lexer,
    parser::{self, types},
    vm::{StackValue, VM},
};

/// Helper function to execute code and return the top stack value
fn execute_code(input: &str) -> Result<Option<StackValue>, InterpretationError> {
    let tokens = lexer::tokenize(input);
    dbg!(tokens.clone());
    let mut index = 0;
    let ast = parser::astify(&tokens, types::ParsingMode::Code, &mut index)?;

    dbg!(ast.clone());
    let mut variables = HashMap::new();
    let commands = ir::translate(ast, &mut variables)?;

    let mut vm = VM::new(commands);
    vm.execute_program(true, false)?;
    Ok(vm.stack.pop())
}

#[test]
fn test_vm_simple_number() {
    let result = execute_code("5;").unwrap();
    assert_eq!(result, Some(StackValue::Int(5)));
}

#[test]
fn test_vm_zero() {
    let result = execute_code("0;").unwrap();
    assert_eq!(result, Some(StackValue::Int(0)));
}

#[test]
fn test_vm_negative_number() {
    let result = execute_code("-42;").unwrap();
    assert_eq!(result, Some(StackValue::Int(-42)));
}

#[test]
fn test_vm_large_number() {
    let result = execute_code("999999;").unwrap();
    assert_eq!(result, Some(StackValue::Int(999999)));
}

#[test]
fn test_vm_simple_addition() {
    let result = execute_code("(+ 5 3);").unwrap();
    assert_eq!(result, Some(StackValue::Int(8)));
}

#[test]
fn test_vm_simple_subtraction() {
    let result = execute_code("(- 10 3);").unwrap();
    assert_eq!(result, Some(StackValue::Int(7)));
}

#[test]
fn test_vm_simple_multiplication() {
    let result = execute_code("(* 4 5);").unwrap();
    assert_eq!(result, Some(StackValue::Int(20)));
}

#[test]
fn test_vm_simple_division() {
    let result = execute_code("(/ 20 4);").unwrap();
    assert_eq!(result, Some(StackValue::Int(5)));
}

#[test]
fn test_vm_simple_modulo() {
    let result = execute_code("(% 17 5);").unwrap();
    assert_eq!(result, Some(StackValue::Int(2)));
}

#[test]
fn test_vm_nested_addition_multiplication() {
    let result = execute_code("(+ (* 2 3) 4);").unwrap();
    // (2 * 3) + 4 = 6 + 4 = 10
    assert_eq!(result, Some(StackValue::Int(10)));
}

#[test]
fn test_vm_deeply_nested_arithmetic() {
    let result = execute_code("(+ (- (* (/ 100 2) 3) 5) 10);").unwrap();
    // (100/2) = 50, (50*3) = 150, (150-5) = 145, (145+10) = 155
    assert_eq!(result, Some(StackValue::Int(155)));
}

#[test]
fn test_vm_comparison_greater_true() {
    let result = execute_code("(> 10 5);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_comparison_greater_false() {
    let result = execute_code("(> 5 10);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(false)));
}

#[test]
fn test_vm_comparison_less_true() {
    let result = execute_code("(< 5 10);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_comparison_less_false() {
    let result = execute_code("(< 10 5);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(false)));
}

#[test]
fn test_vm_comparison_equal_true() {
    let result = execute_code("(== 5 5);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_comparison_equal_false() {
    let result = execute_code("(== 5 3);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(false)));
}

#[test]
fn test_vm_comparison_not_equal_true() {
    let result = execute_code("(!= 5 3);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_comparison_not_equal_false() {
    let result = execute_code("(!= 5 5);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(false)));
}

#[test]
fn test_vm_comparison_greater_or_equal_true_equal() {
    let result = execute_code("(>= 5 5);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_comparison_greater_or_equal_true_greater() {
    let result = execute_code("(>= 10 5);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_comparison_greater_or_equal_false() {
    let result = execute_code("(>= 3 5);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(false)));
}

#[test]
fn test_vm_comparison_less_or_equal_true_equal() {
    let result = execute_code("(<= 5 5);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_comparison_less_or_equal_true_less() {
    let result = execute_code("(<= 3 5);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_comparison_less_or_equal_false() {
    let result = execute_code("(<= 10 5);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(false)));
}

#[test]
fn test_vm_logic_and_both_true() {
    let result = execute_code("(& $true $true);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_logic_and_first_false() {
    let result = execute_code("(& $false $true);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(false)));
}

#[test]
fn test_vm_logic_and_both_false() {
    let result = execute_code("(& $false $false);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(false)));
}

#[test]
fn test_vm_logic_or_both_true() {
    let result = execute_code("(| $true $true);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_logic_or_first_true() {
    let result = execute_code("(| $true $false);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_logic_or_both_false() {
    let result = execute_code("(| $false $false);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(false)));
}

#[test]
fn test_vm_logic_xor_different() {
    let result = execute_code("(^ $true $false);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_logic_xor_same_true() {
    let result = execute_code("(^ $true $true);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(false)));
}

#[test]
fn test_vm_logic_xor_same_false() {
    let result = execute_code("(^ $false $false);").unwrap();
    assert_eq!(result, Some(StackValue::Bool(false)));
}

#[test]
fn test_vm_nested_comparison_with_arithmetic() {
    let result = execute_code("(> (+ 5 3) 7);").unwrap();
    // (5 + 3) = 8, (8 > 7) = true
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_multiple_nested_comparisons() {
    let result = execute_code("(& (> 10 5) (< 3 7));").unwrap();
    // (10 > 5) = true, (3 < 7) = true, (true & true) = 1
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_complex_arithmetic_comparison() {
    let result = execute_code("(>= (- 10 3) 7);").unwrap();
    // (10 - 3) = 7, (7 >= 7) = true
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_arithmetic_with_negative_numbers() {
    let result = execute_code("(+ -5 3);").unwrap();
    assert_eq!(result, Some(StackValue::Int(-2)));
}

#[test]
fn test_vm_subtraction_resulting_in_negative() {
    let result = execute_code("(- 3 5);").unwrap();
    assert_eq!(result, Some(StackValue::Int(-2)));
}

#[test]
fn test_vm_division_with_remainder() {
    let result = execute_code("(/ 7 2);").unwrap();
    // Integer division: 7 / 2 = 3
    assert_eq!(result, Some(StackValue::Int(3)));
}

#[test]
fn test_vm_modulo_zero_remainder() {
    let result = execute_code("(% 10 5);").unwrap();
    assert_eq!(result, Some(StackValue::Int(0)));
}

#[test]
fn test_vm_modulo_with_remainder() {
    let result = execute_code("(% 10 3);").unwrap();
    assert_eq!(result, Some(StackValue::Int(1)));
}

#[test]
fn test_vm_multiple_operations_sequence() {
    // Test that multiple statements work correctly
    let result = execute_code("5; (+ 3 2);").unwrap();
    // Last operation should be on the stack: (3 + 2) = 5
    assert_eq!(result, Some(StackValue::Int(5)));
}

#[test]
fn test_vm_very_deeply_nested() {
    let result = execute_code("(+ (+ (+ (+ 1 1) 1) 1) 1);").unwrap();
    // 1+1 = 2, 2+1 = 3, 3+1 = 4, 4+1 = 5
    assert_eq!(result, Some(StackValue::Int(5)));
}

#[test]
fn test_vm_chain_of_multiplications() {
    let result = execute_code("(* (* 2 3) 4);").unwrap();
    // (2 * 3) = 6, (6 * 4) = 24
    assert_eq!(result, Some(StackValue::Int(24)));
}

#[test]
fn test_vm_mixed_arithmetic_operations() {
    let result = execute_code("(+ (- 10 3) (* 2 4));").unwrap();
    // (10 - 3) = 7, (2 * 4) = 8, (7 + 8) = 15
    assert_eq!(result, Some(StackValue::Int(15)));
}

#[test]
fn test_vm_division_by_small_number() {
    let result = execute_code("(/ 100 10);").unwrap();
    assert_eq!(result, Some(StackValue::Int(10)));
}

#[test]
fn test_vm_long_calculation_chain() {
    let result = execute_code("(* (- 20 10) (+ 2 3));").unwrap();
    // (20 - 10) = 10, (2 + 3) = 5, (10 * 5) = 50
    assert_eq!(result, Some(StackValue::Int(50)));
}

#[test]
fn test_vm_comparison_chain_and() {
    let result = execute_code("(& (& (> 10 5) (< 5 20)) (== 5 5));").unwrap();
    // (10 > 5) = true, (5 < 20) = true, (true & true) = 1, (1 & true) = 1
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_comparison_chain_mixed_operators() {
    let result = execute_code("(| (> 5 10) (< 5 10));").unwrap();
    // (5 > 10) = false, (5 < 10) = true, (false | true) = 1
    assert_eq!(result, Some(StackValue::Bool(true)));
}

#[test]
fn test_vm_all_arithmetic_operators() {
    let test_cases = vec![
        ("(+ 5 3);", Some(StackValue::Int(8))),
        ("(- 10 3);", Some(StackValue::Int(7))),
        ("(* 4 5);", Some(StackValue::Int(20))),
        ("(/ 20 4);", Some(StackValue::Int(5))),
        ("(% 17 5);", Some(StackValue::Int(2))),
    ];

    for (code, expected) in test_cases {
        let result = execute_code(code).expect(&format!("Execution failed for: {}", code));
        assert_eq!(result, expected, "Failed for: {}", code);
    }
}

#[test]
fn test_vm_all_comparison_operators() {
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
        let result = execute_code(code).expect(&format!("Execution failed for: {}", code));
        assert_eq!(result, Some(expected), "Failed for: {}", code);
    }
}

#[test]
fn test_vm_all_logic_operators() {
    let test_cases = vec![
        ("(& $true $true);", Some(StackValue::Bool(true))),
        ("(| $true $false);", Some(StackValue::Bool(true))),
        ("(^ $true $false);", Some(StackValue::Bool(true))),
    ];

    for (code, expected) in test_cases {
        let result = execute_code(code).expect(&format!("Execution failed for: {}", code));
        assert_eq!(result, expected, "Failed for: {}", code);
    }
}
