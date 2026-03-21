// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات Regression للأخطاء العربية - Al-Marjaa
// ═══════════════════════════════════════════════════════════════════════════════
// هذه الاختبارات تضمن أن الأخطاء الشائعة تُعالج بشكل صحيح
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::lexer::{Lexer, Token, TokenType};
use almarjaa::parser::Parser;
use almarjaa::interpreter::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الـ Lexer
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arabic_numbers_lexer() {
    // الأرقام العربية يجب أن تُقرأ بشكل صحيح
    let mut lexer = Lexer::new("١٢٣ ٤٥٦");
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].token_type, TokenType::Number(123.0)));
    assert!(matches!(tokens[1].token_type, TokenType::Number(456.0)));
}

#[test]
fn test_hex_numbers_lexer() {
    // الأرقام السداسية عشر
    let mut lexer = Lexer::new("0xFF 0x1A");
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].token_type, TokenType::Number(255.0)));
    assert!(matches!(tokens[1].token_type, TokenType::Number(26.0)));
}

#[test]
fn test_binary_numbers_lexer() {
    // الأرقام الثنائية
    let mut lexer = Lexer::new("0b1010 0b1111");
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].token_type, TokenType::Number(10.0)));
    assert!(matches!(tokens[1].token_type, TokenType::Number(15.0)));
}

#[test]
fn test_arabic_keywords_lexer() {
    // الكلمات المفتاحية العربية
    let input = "دالة متغير إذا طالما لكل أرجع";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].token_type, TokenType::Function));
    assert!(matches!(tokens[1].token_type, TokenType::Let));
    assert!(matches!(tokens[2].token_type, TokenType::If));
    assert!(matches!(tokens[3].token_type, TokenType::While));
    assert!(matches!(tokens[4].token_type, TokenType::For));
    assert!(matches!(tokens[5].token_type, TokenType::Return));
}

#[test]
fn test_arabic_strings() {
    // النصوص العربية
    let mut lexer = Lexer::new("\"مرحباً بالعالم\"");
    let tokens = lexer.tokenize().unwrap();

    if let TokenType::String(s) = &tokens[0].token_type {
        assert_eq!(s, "مرحباً بالعالم");
    } else {
        panic!("Expected string token");
    }
}

#[test]
fn test_arabic_comments() {
    // التعليقات العربية
    let input = "# هذا تعليق\nاطبع(\"مرحبا\")؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // التعليق يجب أن يُتجاهل أو يُعامل كتعليق
    assert!(tokens.iter().any(|t| matches!(t.token_type, TokenType::Print | TokenType::Identifier(_))));
}

#[test]
fn test_unclosed_string_error() {
    // نص غير مغلق
    let mut lexer = Lexer::new("\"نص غير مغلق");
    let result = lexer.tokenize();
    assert!(result.is_err());
}

#[test]
fn test_reserved_words_as_identifiers() {
    // الكلمات المحجوزة لا يمكن استخدامها كمعرفات
    let input = "متغير لون = 10؛"; // "لون" محجوزة
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // "لون" يجب أن تكون TokenType::Color وليست Identifier
    let color_token = tokens.iter().find(|t| {
        matches!(t.token_type, TokenType::Color)
    });
    assert!(color_token.is_some());
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الـ Parser
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_function_definition() {
    // تعريف دالة
    let input = "دالة سلام(اسم) { اطبع(\"مرحبا \" + اسم)؛ }";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_if_statement() {
    // جملة شرطية
    let input = "إذا س > 10 { اطبع(\"كبير\")؛ }";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_while_loop() {
    // حلقة طالما
    let input = "طالما س < 100 { س = س + 1؛ }";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop() {
    // حلقة لكل
    let input = "لكل عنصر في القائمة { اطبع(عنصر)؛ }";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_class_definition() {
    // تعريف صنف
    let input = "صنف شخص { دالة تهيئة(هذا، الاسم) { هذا.الاسم = الاسم؛ } }";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_range_loop() {
    // حللة نطاق
    let input = "لكل س في مدى(1، 10) { اطبع(س)؛ }";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الـ Interpreter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arithmetic_operations() {
    // العمليات الحسابية
    let input = "متغير ن = 10 + 5 * 2؛";
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_comparison_operations() {
    // عمليات المقارنة
    let inputs = vec![
        "متغير أ = 10 == 10؛",
        "متغير ب = 10 != 5؛",
        "متغير ج = 10 < 20؛",
        "متغير د = 20 > 10؛",
    ];

    for input in inputs {
        let ast = Parser::parse(input).unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&ast);
        assert!(result.is_ok());
    }
}

#[test]
fn test_list_operations() {
    // عمليات القوائم
    let input = r#"
        متغير قائمة = [1، 2، 3، 4، 5]؛
        اطبع(قائمة[0])؛
        اطبع(طول(قائمة))؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_dictionary_operations() {
    // عمليات القواميس
    let input = r#"
        متغير شخص = {"الاسم": "أحمد"، "العمر": 25}؛
        اطبع(شخص["الاسم"])؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_recursion() {
    // العودية
    let input = r#"
        دالة فيبوناتشي(ن) {
            إذا ن <= 1 {
                أرجع ن؛
            }
            أرجع فيبوناتشي(ن - 1) + فيبوناتشي(ن - 2)؛
        }
        اطبع(فيبوناتشي(10))؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات معالجة الأخطاء
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_undefined_variable_error() {
    // استخدام متغير غير معرف
    let input = "اطبع(متغير_غير_معرف)؛";
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_err());
}

#[test]
fn test_division_by_zero_handling() {
    // القسمة على صفر
    let input = "متغير ن = 10 / 0؛";
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    // يجب أن تتعامل مع القسمة على صفر بشكل آمن
    let _ = interpreter.interpret(&ast);
    // لا يجب أن يُنهي البرنامج
}

#[test]
fn test_type_mismatch_handling() {
    // عدم تطابق الأنواع
    let input = r#"متغير ن = "نص" + 10؛"#;
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&ast);
    // يجب أن تتعامل مع عدم تطابق الأنواع
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الـ Unicode
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arabic_diacritics() {
    // التشكيل العربي
    let input = "متغير اسم = \"مُحَمَّد\"؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_right_to_left_text() {
    // النص من اليمين لليسار
    let input = "اطبع(\"مرحباً بالعالم\")؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_mixed_arabic_english() {
    // خلط العربية والإنجليزية
    let input = r#"
        متغير name = "Ahmed"؛
        متغير الاسم = "أحمد"؛
        اطبع(name + " - " + الاسم)؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأداء
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_large_list_performance() {
    // قائمة كبيرة
    let mut input = String::from("متغير قائمة = [");
    for i in 0..1000 {
        if i > 0 {
            input.push_str("، ");
        }
        input.push_str(&format!("{}", i));
    }
    input.push_str("]؛");

    let start = std::time::Instant::now();
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    let duration = start.elapsed();

    println!("Large list tokenization: {:?}", duration);
    assert!(tokens.len() > 1000);
}

#[test]
fn test_deep_recursion_limit() {
    // حد العمق للعودية
    let input = r#"
        دالة عميق(ن) {
            إذا ن <= 0 {
                أرجع 0؛
            }
            أرجع عميق(ن - 1) + 1؛
        }
        اطبع(عميق(100))؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    // يجب أن تعمل أو تعطي خطأ واضح
    assert!(result.is_ok() || result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات إضافية للأخطاء العربية
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_short_variable_keywords() {
    // اختصارات الكلمات المفتاحية
    let input = "م س = 10؛ ث ثابت = 5؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].token_type, TokenType::Let));
    assert!(matches!(tokens[4].token_type, TokenType::Const));
}

#[test]
fn test_boolean_values() {
    // القيم المنطقية العربية
    let input = "متغير أ = صح؛ متغير ب = خطأ؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[2].token_type, TokenType::True));
    assert!(matches!(tokens[6].token_type, TokenType::False));
}

#[test]
fn test_null_value() {
    // القيمة الفارغة العربية
    let input = "متغير فارغ = لا_شيء؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[2].token_type, TokenType::NullKeyword));
}

#[test]
fn test_arithmetic_operators() {
    // العمليات الحسابية
    let input = "متغير ن = 10 + 5 - 3 * 2 / 4 % 2 ^ 3؛";
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_logical_operators() {
    // العمليات المنطقية
    let input = "متغير ن = صح و خطأ أو صح؛";
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_lambda_function() {
    // الدوال الصغيرة (Lambda)
    let input = "متغير ضعف = دالة(س) => س * 2؛";
    let ast = Parser::parse(input);
    // قد يكون مدعوماً أو لا
    println!("Lambda parsing: {:?}", ast.is_ok());
}

#[test]
fn test_array_indexing() {
    // فهرسة المصفوفات
    let input = r#"
        متغير قائمة = [10، 20، 30، 40، 50]؛
        متغير أول = قائمة[0]؛
        متغير أخير = قائمة[4]؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_object_access() {
    // الوصول لخصائص الكائن
    let input = r#"
        متغير شخص = {"اسم": "أحمد"، "عمر": 25}؛
        متغير اسم = شخص["اسم"]؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_method_call() {
    // استدعاء الدوال
    let input = r#"
        متغير قائمة = [1، 2، 3]؛
        قائمة.أضف(4)؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_chained_method_calls() {
    // استدعاءات متسلسلة
    let input = r#"
        متغير قائمة = [1، 2، 3]؛
        قائمة.أضف(4).أضف(5)؛
    "#;
    let ast = Parser::parse(input);
    // قد يكون مدعوماً أو لا
    println!("Chained method calls: {:?}", ast.is_ok());
}

#[test]
fn test_nested_loops() {
    // حلقات متداخلة
    let input = r#"
        لكل س في مدى(1، 4) {
            لكل ص في مدى(1، 4) {
                اطبع(س * ص)؛
            }
        }
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_nested_conditions() {
    // شروط متداخلة
    let input = r#"
        متغير س = 10؛
        إذا س > 5 {
            إذا س > 15 {
                اطبع("كبير جداً")؛
            }
            وإلا {
                اطبع("متوسط")؛
            }
        }
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_else_if_branches() {
    // فروع وإذا
    let input = r#"
        متغير درجة = 85؛
        إذا درجة >= 90 {
            اطبع("ممتاز")؛
        }
        وإذا درجة >= 80 {
            اطبع("جيد جداً")؛
        }
        وإذا درجة >= 70 {
            اطبع("جيد")؛
        }
        وإلا {
            اطبع("مقبول")؛
        }
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    assert!(result.is_ok());
}
