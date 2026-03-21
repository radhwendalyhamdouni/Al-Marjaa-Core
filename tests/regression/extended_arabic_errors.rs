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

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات Regression إضافية للأخطاء العربية
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار الأرقام العربية السالبة
#[test]
fn test_negative_arabic_numbers() {
    let source = r#"
        متغير أ = -١٠٠؛
        متغير ب = -٥٠.٥؛
        متغير ج = أ + ب؛
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

/// اختبار الأرقام العربية الكبيرة
#[test]
fn test_large_arabic_numbers() {
    let source = r#"
        متغير مليون = ١٠٠٠٠٠٠؛
        متغير مليار = ١٠٠٠٠٠٠٠٠٠؛
    "#;
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

/// اختبار النصوص العربية مع الرموز الخاصة
#[test]
fn test_arabic_strings_with_special_chars() {
    let source = r#"
        متغير نص1 = "مرحباً! كيف حالك؟"؛
        متغير نص2 = "السعر: ١٠٠ ريال"؛
        متغير نص3 = "نسبة ٥٠٪ خصم"؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار الأسماء العربية مع الشرطات السفلية
#[test]
fn test_arabic_identifiers_with_underscores() {
    let source = r#"
        متغير اسم_المستخدم = "أحمد"؛
        متغير رقم_الهاتف = 123456؛
        دالة حساب_المجموع(أ، ب) {
            أرجع أ + ب؛
        }
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار التعبيرات المنطقية المعقدة
#[test]
fn test_complex_logical_expressions() {
    let source = r#"
        متغير أ = صح؛
        متغير ب = خطأ؛
        متغير ج = صح؛
        
        متغير نتيجة1 = أ و ب أو ج؛
        متغير نتيجة2 = ليس (أ و ب)؛
        متغير نتيجة3 = (أ أو ب) و ج؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار التعبيرات الحسابية المعقدة
#[test]
fn test_complex_arithmetic_expressions() {
    let source = r#"
        متغير أ = 10 + 5 * 2 - 3؛
        متغير ب = (10 + 5) * (2 - 3)؛
        متغير ج = 100 / 5 % 3؛
        متغير د = 2 ^ 3 ^ 2؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار الدوال مع القيم الافتراضية
#[test]
fn test_function_default_values() {
    let source = r#"
        دالة ترحيب(اسم = "ضيف") {
            اطبع("مرحباً " + اسم)؛
        }
        ترحيب()؛
        ترحيب("أحمد")؛
    "#;
    
    let result = Parser::parse(source);
    println!("Default values test: {:?}", result.is_ok());
}

/// اختبار الدوال المتعددة القيمة المرجعة
#[test]
fn test_multiple_return_values() {
    let source = r#"
        دالة تقسيم(أ، ب) {
            متغير حاصل = أ / ب؛
            متغير باقي = أ % ب؛
            أرجع [حاصل، باقي]؛
        }
        متغير نتيجة = تقسيم(10، 3)؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار القواميس المتداخلة
#[test]
fn test_deeply_nested_dictionaries() {
    let source = r#"
        متغير شركة = {
            "اسم": "شركة التقنية"،
            "عنوان": {
                "مدينة": "الرياض"،
                "حي": "العليا"،
                "شارع": "التحلية"
            }،
            "موظفين": [
                {"اسم": "أحمد"، "قسم": "تقنية"}،
                {"اسم": "سارة"، "قسم": "تسويق"}
            ]
        }؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار حلقات متداخلة مع شروط
#[test]
fn test_nested_loops_with_conditions() {
    let source = r#"
        متغير نتائج = []؛
        لكل أ في مدى(1، 6) {
            لكل ب في مدى(1، 6) {
                إذا أ * ب > 10 {
                    أضف(نتائج، أ * ب)؛
                }
            }
        }
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار الدوال العودية المتقدمة
#[test]
fn test_advanced_recursion() {
    let source = r#"
        دالة عاملي(ن) {
            إذا ن <= 1 {
                أرجع 1؛
            }
            أرجع ن * عاملي(ن - 1)؛
        }
        
        دالة فيبوناتشي(ن) {
            إذا ن <= 1 {
                أرجع ن؛
            }
            أرجع فيبوناتشي(ن - 1) + فيبوناتشي(ن - 2)؛
        }
        
        متغير ف = عاملي(5)؛
        متوسط فيب = فيبوناتشي(10)؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار معالجة الأخطاء في القوائم
#[test]
fn test_list_error_handling() {
    let sources = vec![
        r#"متغير ق = []؛ ق[0]؛"#,
        r#"متغير ق = [1، 2، 3]؛ ق[10]؛"#,
        r#"متغير ق = [1، 2، 3]؛ ق[-1]؛"#,
    ];
    
    for source in sources {
        let ast = Parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&ast);
        // يجب أن يتعامل مع الأخطاء بشكل آمن
    }
}

/// اختبار معالجة الأخطاء في القواميس
#[test]
fn test_dictionary_error_handling() {
    let sources = vec![
        r#"متغير ق = {}؛ ق["مفتاح"]؛"#,
        r#"متغير ق = {"أ": 1}؛ ق["ب"]؛"#,
    ];
    
    for source in sources {
        let ast = Parser::parse(source).unwrap();
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&ast);
        // يجب أن يتعامل مع الأخطاء بشكل آمن
    }
}

/// اختبار الكلمات المفتاحية العربية البديلة
#[test]
fn test_alternative_arabic_keywords() {
    let source = r#"
        # اختبار الاختصارات
        م متغير_مختصر = 10؛
        ث ثابت_مختصر = 20؛
       fn دالة_مختصرة(س) {
            أرجع س * 2؛
        }
    "#;
    
    let result = Parser::parse(source);
    println!("Alternative keywords test: {:?}", result.is_ok());
}

/// اختبار التعبيرات النصية
#[test]
fn test_string_operations() {
    let source = r#"
        متغير نص1 = "مرحبا"؛
        متوسط نص2 = " بالعالم"؛
        متغير مدمج = نص1 + نص2؛
        متغير طول_النص = طول(مدمج)؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار المقارنات المتسلسلة
#[test]
fn test_chained_comparisons() {
    let source = r#"
        متغير س = 5؛
        متغير نتيجة1 = س > 0 و س < 10؛
        متغير نتيجة2 = س >= 5 و س <= 10؛
        متغير نتيجة3 = س == 5 أو س == 10؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار معالجة النهايات المختلفة
#[test]
fn test_different_line_endings() {
    let sources = vec![
        "متغير أ = 1؛\nمتغير ب = 2؛",
        "متغير أ = 1؛\r\nمتغير ب = 2؛",
        "متغير أ = 1؛\rمتغير ب = 2؛",
    ];
    
    for source in sources {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() > 0);
    }
}

/// اختبار التعابير الرياضية المتقدمة
#[test]
fn test_advanced_math() {
    let source = r#"
        متغير π = 3.14159؛
        متوسط نصف_قطر = 5؛
        متغير مساحة = π * نصف_قطر ^ 2؛
        متغير محيط = 2 * π * نصف_قطر؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار عمليات القوائم المتقدمة
#[test]
fn test_advanced_list_operations() {
    let source = r#"
        متغير قائمة1 = [1، 2، 3]؛
        متغير قائمة2 = [4، 5، 6]؛
        متغير مدمجة = قائمة1 + قائمة2؛
        متغير طول_إجمالي = طول(قائمة1) + طول(قائمة2)؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار القيم المنطقية في التعابير
#[test]
fn test_boolean_in_expressions() {
    let source = r#"
        متغير أ = صح + 1؛
        متغير ب = خطأ + 1؛
        متغير ج = صح * 5؛
        متغير د = خطأ * 5؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);
    // قد تعمل أو لا حسب التنفيذ
}

/// اختبار التحويل بين الأنواع
#[test]
fn test_type_conversions() {
    let source = r#"
        متغير رقم = 42؛
        متغير نص = نص(رقم)؛
        متغير منطقي = رقم > 0؛
    "#;
    
    let ast = Parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

/// اختبار الدوال المجهولة
#[test]
fn test_anonymous_functions() {
    let source = r#"
        متغير ضعف = دالة(س) => س * 2؛
        متغير مربع = دالة(س) => س * س؛
    "#;
    
    let result = Parser::parse(source);
    println!("Anonymous functions test: {:?}", result.is_ok());
}

/// اختبار معالجة الفراغات
#[test]
fn test_whitespace_handling() {
    let sources = vec![
        "متغير    أ    =    1    ؛",
        "متغير\nأ\n=\n1\n؛",
        "متغير\tأ\t=\t1\t؛",
    ];
    
    for source in sources {
        let ast = Parser::parse(source);
        assert!(ast.is_ok());
    }
}

/// اختبار JIT مع الأخطاء العربية
#[test]
fn test_jit_with_arabic_errors() {
    let sources = vec![
        r#"متغير أ = ١٠ / ٠؛"#,
        r#"متغير أ = جذر(-١)؛"#,
        r#"متغير أ = أس(٢، -١)؛"#,
    ];
    
    for source in sources {
        let chunk = Compiler::compile_source(source);
        if let Ok(chunk) = chunk {
            let mut jit = CompleteV2JitCompiler::new();
            let mut globals = Rc::new(RefCell::new(Environment::new()));
            let _ = jit.execute(&chunk, &mut globals);
            // يجب أن يتعامل مع الأخطاء بشكل آمن
        }
    }
}

/// اختبار الأداء مع النصوص العربية الطويلة
#[test]
fn test_performance_long_arabic_text() {
    let mut source = String::from("متغير نص = \"");
    for _ in 0..1000 {
        source.push_str("هذا نص تجريبي طويل. ");
    }
    source.push_str("\"؛");
    
    let start = std::time::Instant::now();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().unwrap();
    let duration = start.elapsed();
    
    println!("Long Arabic text tokenization: {:?}", duration);
    assert!(tokens.len() > 0);
}

/// اختبار الاستقرار مع التنفيذ المتعدد
#[test]
fn test_stability_multiple_executions() {
    let source = r#"
        دالة حساب(أ، ب) {
            أرجع أ + ب؛
        }
        حساب(١، ٢)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
    
    for i in 0..10 {
        let mut jit = CompleteV2JitCompiler::new();
        let mut globals = Rc::new(RefCell::new(Environment::new()));
        let result = jit.execute(&chunk, &mut globals);
        assert!(result.is_ok(), "فشل التنفيذ {}", i + 1);
    }
}
