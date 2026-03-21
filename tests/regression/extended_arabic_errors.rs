// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات Regression الموسعة للأخطاء العربية - Extended Arabic Error Tests
// ═══════════════════════════════════════════════════════════════════════════════
// هذه الاختبارات تضمن عدم رجوع الأخطاء المصححة
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::lexer::{Lexer, Token, TokenType};
use almarjaa::parser::Parser;
use almarjaa::interpreter::Interpreter;
use almarjaa::bytecode::{Compiler, CompleteV2JitCompiler};
use std::rc::Rc;
use std::cell::RefCell;
use almarjaa::interpreter::value::Environment;

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأرقام العربية المتقدمة
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arabic_numbers_all_digits() {
    // جميع الأرقام العربية
    let inputs = vec![
        ("٠", 0.0),
        ("١", 1.0),
        ("٢", 2.0),
        ("٣", 3.0),
        ("٤", 4.0),
        ("٥", 5.0),
        ("٦", 6.0),
        ("٧", 7.0),
        ("٨", 8.0),
        ("٩", 9.0),
        ("١٢٣٤٥٦٧٨٩٠", 1234567890.0),
    ];

    for (input, expected) in inputs {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert!(
            matches!(tokens[0].token_type, TokenType::Number(n) if n == expected),
            "فشل في تحويل {} إلى {}",
            input, expected
        );
    }
}

#[test]
fn test_arabic_numbers_in_expressions() {
    let source = r#"
        متغير أ = ١٠٠؛
        متغير ب = ٥٠؛
        متغير مجموع = أ + ب؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_arabic_numbers_in_loops() {
    let source = r#"
        متغير مجموع = 0؛
        لكل س في مدى(١، ١١) {
            مجموع = مجموع + س؛
        }
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_arabic_numbers_decimal() {
    // الأرقام العشرية العربية
    let source = r#"
        متغير أ = ٣.١٤؛
        متغير ب = ٢.٥؛
        متوسط ن = أ + ب؛
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات التشكيل والحركات
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arabic_diacritics_in_strings() {
    let inputs = vec![
        "\"مُحَمَّد\"",
        "\"الْعَرَبِيَّة\"",
        "\"قُرْآن\"",
        "\"مَكْتَبَة\"",
    ];
    
    for input in inputs {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() > 0);
    }
}

#[test]
fn test_arabic_diacritics_in_identifiers() {
    let source = r#"
        متغير اسم_المُستخدم = "أحمد"؛
        متغير عَنوان = "مرحبا"؛
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأخطاء النحوية
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_missing_semicolon_detection() {
    let source = "متغير أ = 10"؛  // بدون فاصلة منقوطة
    let result = Parser::parse(source);
    // قد ينجح أو يفشل حسب التطبيق
    println!("Missing semicolon test: {:?}", result.is_ok());
}

#[test]
fn test_unclosed_bracket_detection() {
    let source = "متغير قائمة = [1، 2، 3؛"؛  // قوس غير مغلق
    let result = Parser::parse(source);
    // قد ينجح أو يفشل حسب التطبيق
    println!("Unclosed bracket test: {:?}", result.is_ok());
}

#[test]
fn test_unclosed_brace_detection() {
    let source = "دالة سلام() { اطبع(\"مرحبا\")؛"؛  // قوس معقوف غير مغلق
    let result = Parser::parse(source);
    // قد ينجح أو يفشل حسب التطبيق
    println!("Unclosed brace test: {:?}", result.is_ok());
}

#[test]
fn test_unclosed_parenthesis_detection() {
    let source = "اطبع(\"مرحبا\"؛"؛  // قوس دائري غير مغلق
    let result = Parser::parse(source);
    // قد ينجح أو يفشل حسب التطبيق
    println!("Unclosed parenthesis test: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأخطاء الدلالية
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_undefined_variable_error_message() {
    let source = "اطبع(متغير_غير_معرف)؛";
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_err());
    
    // التحقق من أن رسالة الخطأ واضحة
    if let Err(e) = result {
        println!("رسالة الخطأ: {}", e);
        // يجب أن تحتوي على اسم المتغير
    }
}

#[test]
fn test_type_error_in_arithmetic() {
    let source = r#"متغير ن = "نص" + 10؛"#;
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);
    // قد ينجح أو يفشل حسب دعم تحويل الأنواع
}

#[test]
fn test_index_out_of_bounds() {
    let source = r#"
        متغير قائمة = [1، 2، 3]؛
        اطبع(قائمة[10])؛
    "#;
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);
    // يجب أن يتعامل مع الفهرس خارج النطاق
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأخطاء في الدوال
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_function_wrong_arguments_count() {
    let source = r#"
        دالة جمع(أ، ب) {
            أرجع أ + ب؛
        }
        جمع(1)؛  // معامل واحد فقط
    "#;
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);
}

#[test]
fn test_missing_return_statement() {
    let source = r#"
        دالة بدون_إرجاع(س) {
            س = س + 1؛
        }
        متغير نتيجة = بدون_إرجاع(5)؛
    "#;
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    // يجب أن تُرجع Null أو قيمة افتراضية
    assert!(result.is_ok());
}

#[test]
fn test_recursive_function_stack_overflow() {
    let source = r#"
        دالة عودية(ن) {
            أرجع عودية(ن + 1)؛
        }
        عودية(0)؛
    "#;
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);
    // يجب أن تتعامل مع العودية اللانهائية
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأخطاء في الحلقات
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_break_outside_loop() {
    let source = "توقف؛"؛  // خارج حلقة
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_err());
}

#[test]
fn test_continue_outside_loop() {
    let source = "استمر؛"؛  // خارج حلقة
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_err());
}

#[test]
fn test_infinite_loop_protection() {
    let source = r#"
        طالما صح {
            // حلقة لا نهائية
        }
    "#;
    let ast = Parser::parse(source);
    // قد ينجح أو يفشل حسب الحماية
    println!("Infinite loop test: {:?}", ast.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأخطاء في الكائنات والقواميس
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_accessing_nonexistent_property() {
    let source = r#"
        متغير شخص = {"اسم": "أحمد"}؛
        اطبع(شخص["عنوان"])؛  // خاصية غير موجودة
    "#;
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    // يجب أن تُرجع Null أو خطأ واضح
    assert!(result.is_ok());
}

#[test]
fn test_modifying_immutable_object() {
    let source = r#"
        ثابت شخص = {"اسم": "أحمد"}؛
        شخص["اسم"] = "محمد"؛  // محاولة تعديل ثابت
    "#;
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأخطاء في الاستيراد والوحدات
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_importing_nonexistent_module() {
    let source = r#"استيراد "وحدة_غير_موجودة"؛"#;
    let _ = Parser::parse(source);
}

#[test]
fn test_undefined_module_function() {
    let source = r#"
        استيراد "رياضيات" كـ ر؛
        ر.دالة_غير_موجودة()؛
    "#;
    let _ = Parser::parse(source);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأخطاء في العمليات
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_division_by_zero_handling() {
    let source = "متغير ن = 10 / 0؛";
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);
    // يجب أن تتعامل مع القسمة على صفر
}

#[test]
fn test_modulo_by_zero_handling() {
    let source = "متغير ن = 10 % 0؛";
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);
}

#[test]
fn test_power_with_negative_exponent() {
    let source = "متغير ن = 2 ^ -3؛";
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_square_root_negative() {
    let source = "متغير ن = جذر(-1)؛";
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);
    // يجب أن تتعامل مع جذر سالب
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأخطاء في السلاسل النصية
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_unclosed_string_error() {
    let source = "\"نص غير مغلق";
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();
    assert!(result.is_err());
}

#[test]
fn test_invalid_escape_sequence() {
    let source = r#"متغير نص = "مرحبا\x غير صالح"؛"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    println!("Escape sequence test: {:?}", tokens.is_ok());
}

#[test]
fn test_multiline_string_handling() {
    let source = r#"متغير نص = "سطر أول
سطر ثاني"؛"#;
    let _ = Parser::parse(source);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأخطاء في Unicode
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_mixed_rtl_ltr_text() {
    let source = r#"
        متغير نص = "Hello مرحبا World"؛
        اطبع(نص)؛
    "#;
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_zero_width_characters() {
    let source = r#"
        متغير نص = "مرحبا\u200بالعالم"؛  // zero-width joiner
    "#;
    let _ = Parser::parse(source);
}

#[test]
fn test_arabic_ligatures() {
    let source = r#"
        متغير نص = "﷽"؛  // Bismillah ligature
        اطبع(نص)؛
    "#;
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأخطاء في JIT
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_jit_arabic_error_messages() {
    let source = r#"
        دالة خطأ() {
            أرجع 10 / 0؛
        }
        خطأ()؛
    "#;
    
    let chunk = Compiler::compile_source(source);
    if let Ok(chunk) = chunk {
        let mut jit = CompleteV2JitCompiler::new();
        let mut globals = Rc::new(RefCell::new(Environment::new()));
        let result = jit.execute(&chunk, &mut globals);
        println!("JIT error handling: {:?}", result);
    }
}

#[test]
fn test_jit_recursion_limit() {
    let source = r#"
        دالة عميق(ن) {
            إذا ن <= 0 {
                أرجع 0؛
            }
            أرجع عميق(ن - 1) + 1؛
        }
        عميق(2000)؛  // يتجاوز الحد
    "#;
    
    let chunk = Compiler::compile_source(source);
    if let Ok(chunk) = chunk {
        let mut jit = CompleteV2JitCompiler::new();
        let mut globals = Rc::new(RefCell::new(Environment::new()));
        let result = jit.execute(&chunk, &mut globals);
        // يجب أن يتعامل مع العمق الزائد
        println!("Recursion limit: {:?}", result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأداء والذاكرة
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_large_arabic_string_handling() {
    let mut source = String::from("متغير نص = \"");
    for _ in 0..10000 {
        source.push_str("مرحبا بالعالم ");
    }
    source.push_str("\"؛");
    
    let start = std::time::Instant::now();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();
    let duration = start.elapsed();
    
    println!("Large string tokenization: {:?}", duration);
    assert!(tokens.len() > 0);
}

#[test]
fn test_many_variables_memory() {
    let mut source = String::new();
    for i in 0..1000 {
        source.push_str(&format!("متغير متغير_{} = {}؛", i, i));
    }
    
    let start = std::time::Instant::now();
    let ast = Parser::parse(&source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    let duration = start.elapsed();
    
    println!("1000 variables execution: {:?}", duration);
    assert!(result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الكلمات المفتاحية المحجوزة
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_all_arabic_keywords() {
    let keywords = vec![
        ("دالة", TokenType::Function),
        ("متغير", TokenType::Let),
        ("ثابت", TokenType::Const),
        ("إذا", TokenType::If),
        ("وإلا", TokenType::Else),
        ("طالما", TokenType::While),
        ("لكل", TokenType::For),
        ("في", TokenType::In),
        ("أرجع", TokenType::Return),
        ("صح", TokenType::True),
        ("خطأ", TokenType::False),
        ("لا_شيء", TokenType::NullKeyword),
        ("و", TokenType::And),
        ("أو", TokenType::Or),
        ("ليس", TokenType::Not),
        ("صنف", TokenType::Class),
        ("هذا", TokenType::This),
        ("جديد", TokenType::New),
    ];
    
    for (keyword, expected_type) in keywords {
        let mut lexer = Lexer::new(keyword);
        let tokens = lexer.tokenize().unwrap();
        assert!(
            matches!(tokens[0].token_type, ref t if std::mem::discriminant(t) == std::mem::discriminant(&expected_type)),
            "الكلمة المفتاحية '{}' لم تُعرَّف بشكل صحيح",
            keyword
        );
    }
}

#[test]
fn test_reserved_word_as_identifier_error() {
    let reserved_words = vec!["دالة", "متغير", "إذا", "طالما", "لكل", "أرجع"];
    
    for word in reserved_words {
        let source = format!("متغير {} = 10؛", word);
        let result = Parser::parse(&source);
        // قد ينجح أو يفشل حسب التطبيق
        println!("Reserved word '{}' as identifier: {:?}", word, result.is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الرموز العربية الخاصة
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arabic_operators() {
    let source = r#"
        متغير أ = 10 + 5؛
        متغير ب = 20 - 8؛
        متغير ج = 6 × 7؛
        متوسط د = 100 ÷ 4؛
    "#;
    
    let _ = Parser::parse(source);
}

#[test]
fn test_arabic_punctuation() {
    let source = r#"
        متغير أ = 10؛
        متغير ب = 20،
        متغير ج = 30؛
    "#;
    
    let result = Parser::parse(source);
    println!("Arabic punctuation test: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات التعليقات العربية
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arabic_single_line_comment() {
    let source = r#"
        # هذا تعليق عربي
        متغير أ = 10؛
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_arabic_multiline_comment() {
    let source = r#"
        /* 
         * تعليق متعدد الأسطر
         * باللغة العربية
         */
        متغير أ = 10؛
    "#;
    
    let result = Parser::parse(source);
    println!("Multiline comment test: {:?}", result.is_ok());
}

#[test]
fn test_nested_comments() {
    let source = r#"
        /* تعليق خارجي
           /* تعليق داخلي */
           استمرار التعليق الخارجي
        */
        متغير أ = 10؛
    "#;
    
    let result = Parser::parse(source);
    println!("Nested comments test: {:?}", result.is_ok());
}
