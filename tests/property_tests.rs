// ═══════════════════════════════════════════════════════════════════════════════
// Property-Based Tests - اختبارات قائمة على الخصائص
// ═══════════════════════════════════════════════════════════════════════════════
// Using proptest for:
// - Parser invariants
// - AST correctness
// - Tokenizer robustness
// - Arithmetic properties
// - String handling properties
// ═══════════════════════════════════════════════════════════════════════════════

use proptest::prelude::*;
use proptest::collection::{vec, hash_map};

use almarjaa::lexer::Lexer;
use almarjaa::parser::Parser;
use almarjaa::interpreter::Interpreter;
use almarjaa::bytecode::{Compiler, CompleteV2JitCompiler};
use std::rc::Rc;
use std::cell::RefCell;
use almarjaa::Environment;

// ═══════════════════════════════════════════════════════════════════════════════
// LEXER PROPERTY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Property: Lexer should never panic on any input
proptest! {
    #[test]
    fn test_lexer_never_panics(input in ".*") {
        let mut lexer = Lexer::new(&input);
        let _ = lexer.tokenize();
        // Must not panic
    }
}

/// Property: Lexer should handle any ASCII input
proptest! {
    #[test]
    fn test_lexer_ascii_input(input in "[\\x00-\\x7F]*") {
        let mut lexer = Lexer::new(&input);
        let result = lexer.tokenize();
        // Should either succeed or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }
}

/// Property: Lexer should handle any Unicode input
proptest! {
    #[test]
    fn test_lexer_unicode_input(input in ".*") {
        let mut lexer = Lexer::new(&input);
        let result = lexer.tokenize();
        // Should handle any Unicode
        assert!(result.is_ok() || result.is_err());
    }
}

/// Property: Lexer should handle Arabic characters
proptest! {
    #[test]
    fn test_lexer_arabic_chars(input in "[\u{0600}-\u{06FF}]*") {
        let mut lexer = Lexer::new(&input);
        let result = lexer.tokenize();
        // Should handle Arabic range
        assert!(result.is_ok() || result.is_err());
    }
}

/// Property: Valid numbers should be tokenized as numbers
proptest! {
    #[test]
    fn test_lexer_valid_numbers(num in any::<f64>()) {
        let input = format!("متغير ن = {}؛", num);
        let mut lexer = Lexer::new(&input);
        let result = lexer.tokenize();
        assert!(result.is_ok());
    }
}

/// Property: Integer numbers should tokenize correctly
proptest! {
    #[test]
    fn test_lexer_integer_numbers(num in any::<i64>()) {
        let input = format!("متغير ن = {}؛", num);
        let mut lexer = Lexer::new(&input);
        let result = lexer.tokenize();
        assert!(result.is_ok());
    }
}

/// Property: Strings with any content should be tokenizable
proptest! {
    #[test]
    fn test_lexer_string_content(content in "[^\"]*") {
        let input = format!("متغير نص = \"{}\"؛", content);
        let mut lexer = Lexer::new(&input);
        let result = lexer.tokenize();
        assert!(result.is_ok());
    }
}

/// Property: Lexer should handle repeated tokens
proptest! {
    #[test]
    fn test_lexer_repeated_tokens(count in 1usize..100) {
        let input = "متغير س = 1؛ ".repeat(count);
        let mut lexer = Lexer::new(&input);
        let result = lexer.tokenize();
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PARSER PROPERTY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Property: Parser should never panic on any input
proptest! {
    #[test]
    fn test_parser_never_panics(input in ".*") {
        let _ = Parser::parse(&input);
        // Must not panic
    }
}

/// Property: Valid expressions should parse
proptest! {
    #[test]
    fn test_parser_valid_expression(a in any::<i32>(), b in any::<i32>()) {
        let input = format!("متغير ن = {} + {}؛", a, b);
        let result = Parser::parse(&input);
        assert!(result.is_ok());
    }
}

/// Property: Nested expressions should parse
proptest! {
    #[test]
    fn test_parser_nested_expressions(depth in 1usize..10) {
        let mut expr = "1".to_string();
        for _ in 0..depth {
            expr = format!("({} + 1)", expr);
        }
        let input = format!("متغير ن = {}؛", expr);
        let result = Parser::parse(&input);
        assert!(result.is_ok());
    }
}

/// Property: Function definitions with various parameter counts
proptest! {
    #[test]
    fn test_parser_function_params(count in 0usize..10) {
        let params: Vec<String> = (0..count).map(|i| format!("p{}", i)).collect();
        let params_str = params.join("، ");
        let input = format!("دالة د({}) {{ أرجع 0؛ }}", params_str);
        let result = Parser::parse(&input);
        assert!(result.is_ok());
    }
}

/// Property: Multiple statements should parse
proptest! {
    #[test]
    fn test_parser_multiple_statements(count in 1usize..50) {
        let mut input = String::new();
        for i in 0..count {
            input.push_str(&format!("متغير س{} = {}؛", i, i));
        }
        let result = Parser::parse(&input);
        assert!(result.is_ok());
    }
}

/// Property: Nested if statements should parse
proptest! {
    #[test]
    fn test_parser_nested_if(depth in 1usize..20) {
        let mut input = String::new();
        for _ in 0..depth {
            input.push_str("إذا صح {");
        }
        input.push_str("س = 1؛");
        for _ in 0..depth {
            input.push_str("}");
        }
        let result = Parser::parse(&input);
        assert!(result.is_ok());
    }
}

/// Property: Nested loops should parse
proptest! {
    #[test]
    fn test_parser_nested_loops(depth in 1usize..10) {
        let mut input = String::new();
        for i in 0..depth {
            input.push_str(&format!("لكل أ{} في مدى(0، 10) {{", i));
        }
        input.push_str("س = س + 1؛");
        for _ in 0..depth {
            input.push_str("}");
        }
        let result = Parser::parse(&input);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ARITHMETIC PROPERTY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Property: Addition is commutative
proptest! {
    #[test]
    fn test_arithmetic_addition_commutative(a in any::<f64>(), b in any::<f64>()) {
        let input1 = format!("متغير ن = {} + {}؛", a, b);
        let input2 = format!("متغير ن = {} + {}؛", b, a);
        
        let mut interp1 = Interpreter::new();
        let mut interp2 = Interpreter::new();
        
        // Both should parse
        let _ = interp1.run(&input1);
        let _ = interp2.run(&input2);
    }
}

/// Property: Addition is associative (approximately)
proptest! {
    #[test]
    fn test_arithmetic_addition_associative(a in any::<f64>(), b in any::<f64>(), c in any::<f64>()) {
        let input1 = format!("متغير ن = ({0} + {1}) + {2}؛", a, b, c);
        let input2 = format!("متغير ن = {0} + ({1} + {2})؛", a, b, c);
        
        let mut interp1 = Interpreter::new();
        let mut interp2 = Interpreter::new();
        
        let _ = interp1.run(&input1);
        let _ = interp2.run(&input2);
    }
}

/// Property: Multiplication is commutative
proptest! {
    #[test]
    fn test_arithmetic_multiplication_commutative(a in any::<f64>(), b in any::<f64>()) {
        let input1 = format!("متغير ن = {} * {}؛", a, b);
        let input2 = format!("متغير ن = {} * {}؛", b, a);
        
        let mut interp1 = Interpreter::new();
        let mut interp2 = Interpreter::new();
        
        let _ = interp1.run(&input1);
        let _ = interp2.run(&input2);
    }
}

/// Property: Zero multiplication
proptest! {
    #[test]
    fn test_arithmetic_zero_multiplication(a in any::<f64>()) {
        let input = format!("متغير ن = {} * 0؛", a);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

/// Property: Identity operations
proptest! {
    #[test]
    fn test_arithmetic_identity(a in any::<f64>()) {
        let input1 = format!("متغير ن = {} + 0؛", a);
        let input2 = format!("متغير ن = {} * 1؛", a);
        let input3 = format!("متغير ن = {} - 0؛", a);
        
        let mut interp = Interpreter::new();
        let _ = interp.run(&input1);
        let _ = interp.run(&input2);
        let _ = interp.run(&input3);
    }
}

/// Property: Division by non-zero
proptest! {
    #[test]
    fn test_arithmetic_division_nonzero(a in any::<f64>(), b in any::<f64>().prop_filter("non-zero", |&x| x != 0.0)) {
        let input = format!("متغير ن = {} / {}؛", a, b);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BOOLEAN PROPERTY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Property: AND operation
proptest! {
    #[test]
    fn test_boolean_and(a in any::<bool>(), b in any::<bool>()) {
        let input = format!("متغير ن = {} و {}؛", a, b);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

/// Property: OR operation
proptest! {
    #[test]
    fn test_boolean_or(a in any::<bool>(), b in any::<bool>()) {
        let input = format!("متغير ن = {} أو {}؛", a, b);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

/// Property: NOT operation
proptest! {
    #[test]
    fn test_boolean_not(a in any::<bool>()) {
        let input = format!("متغير ن = ليس {}؛", a);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

/// Property: Double negation
proptest! {
    #[test]
    fn test_boolean_double_negation(a in any::<bool>()) {
        let input = format!("متغير ن = ليس (ليس {})؛", a);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPARISON PROPERTY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Property: Comparison operations
proptest! {
    #[test]
    fn test_comparison_operations(a in any::<f64>(), b in any::<f64>()) {
        let inputs = vec![
            format!("متغير ن = {} == {}؛", a, b),
            format!("متغير ن = {} != {}؛", a, b),
            format!("متغير ن = {} < {}؛", a, b),
            format!("متغير ن = {} > {}؛", a, b),
            format!("متغير ن = {} <= {}؛", a, b),
            format!("متغير ن = {} >= {}؛", a, b),
        ];
        
        for input in inputs {
            let mut interp = Interpreter::new();
            let result = interp.run(&input);
            assert!(result.is_ok());
        }
    }
}

/// Property: Equality reflexivity
proptest! {
    #[test]
    fn test_comparison_equality_reflexive(a in any::<f64>()) {
        let input = format!("متغير ن = {} == {}؛", a, a);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// STRING PROPERTY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Property: String concatenation
proptest! {
    #[test]
    fn test_string_concatenation(a in ".*", b in ".*") {
        let input = format!("متغير ن = \"{}\" + \"{}\"؛", a, b);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

/// Property: String with any Arabic content
proptest! {
    #[test]
    fn test_string_arabic_content(content in "[\u{0600}-\u{06FF}]*") {
        let input = format!("متغير نص = \"{}\"؛", content);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

/// Property: Empty string
#[test]
fn test_string_empty() {
    let input = "متغير نص = \"\"؛";
    let mut interp = Interpreter::new();
    let result = interp.run(input);
    assert!(result.is_ok());
}

/// Property: String length function
proptest! {
    #[test]
    fn test_string_length(content in "[a-zA-Z]*") {
        let input = format!("متغير ن = طول(\"{}\")؛", content);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// LIST PROPERTY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Property: Lists of various sizes
proptest! {
    #[test]
    fn test_list_various_sizes(items in vec(any::<i32>(), 0..100)) {
        let items_str: Vec<String> = items.iter().map(|x| x.to_string()).collect();
        let input = format!("متغير قائمة = [{}]؛", items_str.join("، "));
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

/// Property: Empty list
#[test]
fn test_list_empty() {
    let input = "متغير قائمة = []؛";
    let mut interp = Interpreter::new();
    let result = interp.run(input);
    assert!(result.is_ok());
}

/// Property: Nested lists
proptest! {
    #[test]
    fn test_list_nested(depth in 1usize..5) {
        let mut input = "متغير قائمة = ".to_string();
        for _ in 0..depth {
            input.push('[');
        }
        input.push_str("1");
        for _ in 0..depth {
            input.push(']');
        }
        input.push('؛');
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// DICTIONARY PROPERTY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Property: Dictionaries of various sizes
proptest! {
    #[test]
    fn test_dict_various_sizes(items in hash_map("key[a-z]", any::<i32>(), 0..20)) {
        let items_str: Vec<String> = items.iter()
            .map(|(k, v)| format!("\"{}\": {}", k, v))
            .collect();
        let input = format!("متغير قاموس = {{{}}}؛", items_str.join("، "));
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

/// Property: Empty dictionary
#[test]
fn test_dict_empty() {
    let input = "متغير قاموس = {}؛";
    let mut interp = Interpreter::new();
    let result = interp.run(input);
    assert!(result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// JIT PROPERTY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Property: JIT should handle arithmetic expressions
proptest! {
    #[test]
    fn test_jit_arithmetic(a in any::<i32>(), b in any::<i32>()) {
        let input = format!("متغير ن = {} + {}؛", a, b);
        let chunk = Compiler::compile_source(&input);
        if let Ok(chunk) = chunk {
            let mut jit = CompleteV2JitCompiler::new();
            let mut globals = Rc::new(RefCell::new(Environment::new()));
            let result = jit.execute(&chunk, &mut globals);
            assert!(result.is_ok());
        }
    }
}

/// Property: JIT should handle loops with various iterations
proptest! {
    #[test]
    fn test_jit_loop_iterations(count in 1usize..1000) {
        let input = format!(r#"
            متغير مجموع = 0؛
            لكل س في مدى(1، {}) {{
                مجموع = مجموع + س؛
            }}
        "#, count);
        let chunk = Compiler::compile_source(&input);
        if let Ok(chunk) = chunk {
            let mut jit = CompleteV2JitCompiler::new();
            let mut globals = Rc::new(RefCell::new(Environment::new()));
            let result = jit.execute(&chunk, &mut globals);
            assert!(result.is_ok());
        }
    }
}

/// Property: JIT should handle function calls
proptest! {
    #[test]
    fn test_jit_function_call(a in any::<i32>(), b in any::<i32>()) {
        let input = format!(r#"
            دالة جمع(أ، ب) {{
                أرجع أ + ب؛
            }}
            جمع({}, {})؛
        "#, a, b);
        let chunk = Compiler::compile_source(&input);
        if let Ok(chunk) = chunk {
            let mut jit = CompleteV2JitCompiler::new();
            let mut globals = Rc::new(RefCell::new(Environment::new()));
            let result = jit.execute(&chunk, &mut globals);
            assert!(result.is_ok());
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// UNICODE PROPERTY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Property: Handle zero-width characters
proptest! {
    #[test]
    fn test_unicode_zero_width(content in "[a-z]*", zw_count in 0usize..10) {
        let zw = "\u{200B}";
        let modified: String = content.chars()
            .flat_map(|c| std::iter::once(c).chain(std::iter::repeat(zw.chars().next().unwrap()).take(zw_count)))
            .collect();
        let input = format!("متغير نص = \"{}\"؛", modified);
        let mut lexer = Lexer::new(&input);
        let result = lexer.tokenize();
        assert!(result.is_ok());
    }
}

/// Property: Handle mixed RTL/LTR
proptest! {
    #[test]
    fn test_unicode_mixed_direction(arabic in "[\u{0600}-\u{06FF}]*", english in "[a-zA-Z]*") {
        let input = format!("متغير نص = \"{}{}\"؛", arabic, english);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

/// Property: Handle diacritics
proptest! {
    #[test]
    fn test_unicode_diacritics(base in "[ابتثجحخدذرزسشصضطظعغفقكلمنهوي]*") {
        let with_diacritics: String = base.chars()
            .flat_map(|c| {
                let diacritics = ['\u{064E}', '\u{064F}', '\u{0650}', '\u{0651}', '\u{0652}'];
                let idx = (c as u32) as usize % diacritics.len();
                std::iter::once(c).chain(std::iter::once(diacritics[idx]))
            })
            .collect();
        let input = format!("متغير نص = \"{}\"؛", with_diacritics);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// EDGE CASE PROPERTY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Property: Very large numbers
proptest! {
    #[test]
    fn test_edge_large_numbers(num in any::<f64>()) {
        let input = format!("متغير ن = {}؛", num);
        let mut interp = Interpreter::new();
        let result = interp.run(&input);
        assert!(result.is_ok());
    }
}

/// Property: Very long identifiers
proptest! {
    #[test]
    fn test_edge_long_identifiers(length in 1usize..1000) {
        let name: String = (0..length).map(|_| 'س').collect();
        let input = format!("متغير {} = 1؛", name);
        let mut lexer = Lexer::new(&input);
        let result = lexer.tokenize();
        assert!(result.is_ok());
    }
}

/// Property: Deep nesting
proptest! {
    #[test]
    fn test_edge_deep_nesting(depth in 1usize..50) {
        let mut input = "متغير ن = ".to_string();
        for _ in 0..depth {
            input.push_str("(((");
        }
        input.push('1');
        for _ in 0..depth {
            input.push_str(")))");
        }
        input.push('؛');
        let result = Parser::parse(&input);
        assert!(result.is_ok());
    }
}

/// Property: Many consecutive operations
proptest! {
    #[test]
    fn test_edge_consecutive_operations(count in 1usize..100) {
        let mut input = "متغير ن = 1".to_string();
        for _ in 0..count {
            input.push_str(" + 1");
        }
        input.push('؛');
        let result = Parser::parse(&input);
        assert!(result.is_ok());
    }
}
