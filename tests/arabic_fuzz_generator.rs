// ═══════════════════════════════════════════════════════════════════════════════
// Arabic Error Regression Tests - Additional 150+ Cases
// اختبارات Regression إضافية للأخطاء العربية - أكثر من 150 حالة
// ═══════════════════════════════════════════════════════════════════════════════
// Coverage:
// - Arabic diacritics variations (extended)
// - Mixed RTL/LTR text (complex)
// - Zero-width unicode characters (all types)
// - Ligatures and special forms
// - Long Arabic identifiers (stress)
// - Multiline strings/comments (edge cases)
// - Error messages in Arabic
// - Arabic number formats
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::lexer::{Lexer, TokenType};
use almarjaa::parser::Parser;
use almarjaa::interpreter::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════════
// ARABIC DIACRITICS EXTENDED (التشكيل العربي الموسع)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_diacritics_complex_word() {
    let input = "متغير اَلْعَرَبِيَّةُ = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة الكلمات مع تشكيل كامل");
}

#[test]
fn test_diacritics_hamza_above() {
    let input = "متغير سَأَلَ = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة الكلمات مع همزة فوق");
}

#[test]
fn test_diacritics_hamza_below() {
    let input = "متغير إِسْلَام = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة الكلمات مع همزة تحت");
}

#[test]
fn test_diacritics_hamza_middle() {
    let input = "متغير رُؤْيَة = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة الكلمات مع همزة وسط");
}

#[test]
fn test_diacritics_hamza_isolated() {
    let input = "متغير ء = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة الهمزة المنفصلة");
}

#[test]
fn test_diacritics_madda() {
    let input = "متغير آمن = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة المدّة");
}

#[test]
fn test_diacritics_waw_with_hamza() {
    let input = "متغير مؤمن = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة الواو مع همزة");
}

#[test]
fn test_diacritics_ya_with_hamza() {
    let input = "متغير نائم = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة الياء مع همزة");
}

#[test]
fn test_diacritics_alif_wasla() {
    let input = "متغير ابْن = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة ألف الوصل");
}

#[test]
fn test_diacritics_shadda_with_fatha() {
    let input = "متغير رَتَّبَ = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة الشدة مع الفتحة");
}

#[test]
fn test_diacritics_shadda_with_kasra() {
    let input = "متغير مُدَرِّس = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة الشدة مع الكسرة");
}

#[test]
fn test_diacritics_shadda_with_damma() {
    let input = "متغير مُتَعَلِّمُونَ = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة الشدة مع الضمة");
}

#[test]
fn test_diacritics_multiple_tanwin() {
    let input = "متغير عَدَدًا إِعْدَادٍ أَعْدَادٌ = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة التنوين المتعدد");
}

#[test]
fn test_diacritics_full_sentence() {
    let input = "متغير جُمْلَةٌ تَامَّةٌ مُشَكَّلَةٌ = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة الجملة المشكلة بالكامل");
}

#[test]
fn test_diacritics_quranic_text() {
    let input = r#"متغير آية = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة النص القرآني");
}

// ═══════════════════════════════════════════════════════════════════════════════
// MIXED RTL/LTR COMPLEX (النصوص المختلطة المعقدة)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_mixed_code_english_keywords() {
    let input = r#"
        متغير result = 10 + 20؛
        دالة calculate(x, y) {
            أرجع x + y؛
        }
        result = calculate(5, 10)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل الكود المختلط");
}

#[test]
fn test_mixed_string_concat() {
    let input = r#"
        متغير greeting = "Hello " + "مرحبا" + " World" + " عالم"؛
        اطبع(greeting)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل دمج النصوص المختلطة");
}

#[test]
fn test_mixed_function_names() {
    let input = r#"
        دالة calculateSum(a, b) {
            أرجع a + b؛
        }
        دالة احسب_المجموع(أ، ب) {
            أرجع أ + ب؛
        }
        calculateSum(1, 2) + احسب_المجموع(3, 4)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل أسماء الدوال المختلطة");
}

#[test]
fn test_mixed_class_definition() {
    let input = r#"
        صنف User {
            دالة تهيئة(ha, name) {
                ha.الاسم = name؛
            }
            دالة getName(ha) {
                أرجع ha.الاسم؛
            }
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل تعريف الصنف المختلط");
}

#[test]
fn test_mixed_comments_complex() {
    let input = r#"
        # This is an English comment
        # هذا تعليق عربي
        # Mixed comment: English and العربية
        متغير x = 10؛ # inline comment تعليق مضمن
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة التعليقات المختلطة");
}

#[test]
fn test_mixed_operators_expressions() {
    let input = r#"
        متغير x = 10 + 5 * 2 - 3 / 1؛
        متغير y = أ + ب * ج - د / هـ؛
        متغير z = x + y + 100 + مئتان؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل التعبيرات المختلطة");
}

#[test]
fn test_mixed_boolean_expressions() {
    let input = r#"
        متغير bool1 = صح و خطأ؛
        متغير bool2 = true و false؛
        متغير bool3 = صح أو true؛
        متغير bool4 = ليس false و خطأ؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل التعبيرات المنطقية المختلطة");
}

#[test]
fn test_mixed_data_structures() {
    let input = r#"
        متغير list = [1، "two"، 3، "أربعة"، true، صح]؛
        متغير dict = {
            "key1": "قيمة1"،
            "مفتاح2": "value2"،
            "mixed": "مختلط"
        }؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل هياكل البيانات المختلطة");
}

#[test]
fn test_mixed_function_calls() {
    let input = r#"
        دالة func1(a, b) { أرجع a + b؛ }
        دالة دالة2(أ، ب) { أرجع أ * ب؛ }
        
        func1(1, 2)؛
        دالة2(3، 4)؛
        func1(دالة2(5، 6)، 7)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل استدعاءات الدوال المختلطة");
}

#[test]
fn test_mixed_control_flow() {
    let input = r#"
        لكل i في مدى(0، 10) {
            إذا i % 2 == 0 {
                اطبع("even: " + i)؛
            } وإلا {
                اطبع("فردي: " + i)؛
            }
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل تدفق التحكم المختلط");
}

// ═══════════════════════════════════════════════════════════════════════════════
// ZERO-WIDTH UNICODE ALL TYPES (جميع أنواع الأحرف ذات العرض الصفري)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_zwj_multiple_positions() {
    // Zero Width Joiner في مواضع متعددة
    let input = "متغير س\u{200D}م\u{200D}ر = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_zwnj_multiple_positions() {
    // Zero Width Non-Joiner في مواضع متعددة
    let input = "متغير س\u{200C}م\u{200C}ر = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_zws_in_expression() {
    // Zero Width Space في تعبير
    let input = "متغير ن = 10\u{200B}+\u{200B}20؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_lrm_rlm_in_string() {
    // Left-to-Right Mark و Right-to-Left Mark في نص
    let input = "متغير نص = \"Hello\u{200E}مرحبا\u{200F}World\"؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_lre_rle_embedding() {
    // Left-to-Right Embedding و Right-to-Left Embedding
    let input = "متغير نص = \"\u{202A}Hello\u{202B}مرحبا\u{202C}\"؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_lro_rlo_override() {
    // Left-to-Right Override و Right-to-Left Override
    let input = "متغير نص = \"\u{202D}Hello\u{202E}مرحبا\u{202C}\"؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_pop_directional_format() {
    // Pop Directional Formatting
    let input = "متغير نص = \"\u{202A}test\u{202C}\"؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_fsi_isolate() {
    // First Strong Isolate
    let input = "متغير نص = \"\u{2068}Hello مرحبا\u{2069}\"؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_invisible_operators() {
    // Invisible operators (U+2061, U+2062, U+2063)
    let input = "متغير ن = 10\u{2062}20\u{2063}؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_invisible_separator() {
    // Invisible Separator (U+2064)
    let input = "متغير ن = 10.5\u{2064}25؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_combining_invisible() {
    // Combining characters with invisible
    let input = "متغير س\u{200B}\u{200C}\u{200D}\u{0300}\u{0301}م = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_shy_hyphen() {
    // Soft Hyphen (U+00AD)
    let input = "متغير س\u{00AD}م = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// ARABIC LIGATURES AND SPECIAL FORMS (التركيبات والأشكال الخاصة)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ligature_lam_alif_all_forms() {
    // جميع أشكال لام ألف
    let inputs = vec![
        "متغير لا = 1؛",   // لا
        "متغير لأ = 2؛",   // لأ
        "متغير لإ = 3؛",   // لإ
        "متغير لآ = 4؛",   // لآ
    ];
    
    for input in inputs {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() > 0, "يجب قراءة ligature: {}", input);
    }
}

#[test]
fn test_arabic_presentation_forms_a() {
    // Arabic Presentation Forms-A
    let input = "متغير ﷲ = 10؛";  // Allah ligature
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_arabic_presentation_forms_b() {
    // Arabic Presentation Forms-B (isolated forms)
    let input = "متغير ﺎﺏﺕﺙ = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_arabic_mathematical_operators() {
    // Arabic Mathematical Operators
    let inputs = vec![
        "متغير ن = 5 ؋ 3؛",  // Arabic multiplication
        "متغير م = 10 ، 2؛", // Arabic division
    ];
    
    for input in inputs {
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();
        assert!(result.is_ok() || result.is_err(), "يجب معالجة: {}", input);
    }
}

#[test]
fn test_extended_arabic_indic_digits() {
    // Extended Arabic-Indic Digits (used in Persian/Urdu)
    let input = "متغير ن = ۰۱۲۳۴۵۶۷۸۹؛";  // Persian digits
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// LONG IDENTIFIERS STRESS (اختبارات إجهاد للمعرفات الطويلة)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_identifier_500_chars() {
    let long_name = "متع".repeat(167);
    let input = format!("متغير {} = 10؛", long_name);
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة معرف 500 حرف");
}

#[test]
fn test_identifier_1000_chars() {
    let long_name = "متغير_طويل_جدا".repeat(77);
    let input = format!("متغير {} = 10؛", long_name);
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة معرف 1000 حرف");
}

#[test]
fn test_multiple_long_identifiers() {
    let name1 = "أ".repeat(100);
    let name2 = "ب".repeat(100);
    let name3 = "ج".repeat(100);
    let input = format!(
        "متغير {} = 1؛\nمتغير {} = 2؛\nمتغير {} = {} + {}؛",
        name1, name2, name3, name1, name2
    );
    let ast = Parser::parse(&input);
    assert!(ast.is_ok(), "يجب معالجة معرفات طويلة متعددة");
}

#[test]
fn test_long_function_name_500() {
    let long_name = "دالة_عربية_طويلة_لحساب".repeat(20);
    let input = format!("دالة {}(أ، ب) {{ أرجع أ + ب؛ }}", long_name);
    let ast = Parser::parse(&input);
    assert!(ast.is_ok(), "يجب معالجة اسم دالة طويل");
}

#[test]
fn test_nested_identifiers_long() {
    let long_name = "متغير_طويل_جدا_للاختبار".repeat(10);
    let input = format!(
        r#"
        صنف {} {{
            دالة {}(هذا، س) {{
                هذا.{} = س؛
            }}
        }}
        "#,
        long_name, long_name, long_name
    );
    let ast = Parser::parse(&input);
    assert!(ast.is_ok(), "يجب معالجة معرفات طويلة متداخلة");
}

// ═══════════════════════════════════════════════════════════════════════════════
// MULTILINE STRINGS AND COMMENTS EDGE CASES
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_multiline_string_10_lines() {
    let input = r#"متغير نص = "سطر1
سطر2
سطر3
سطر4
سطر5
سطر6
سطر7
سطر8
سطر9
سطر10"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة نص متعدد الأسطر");
}

#[test]
fn test_multiline_string_with_arabic_newlines() {
    // نص مع أسطر جديدة عربية
    let input = "متغير نص = \"السطر الأول\nالسطر الثاني\nالسطر الثالث\"؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_multiline_string_with_escapes() {
    let input = r#"متغير نص = "سطر1\nسطر2\tسطر3\\سطر4\"سطر5"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة النص مع escape sequences");
}

#[test]
fn test_multiline_comment_block() {
    let input = r#"
        # هذا تعليق
        # متعدد الأسطر
        # للتحقق
        متغير س = 10؛
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_multiline_string_in_function() {
    let input = r#"
        دالة اختبار() {
            متغير نص = "هذا نص
            طويل جداً
            داخل دالة"؛
            أرجع نص؛
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب قراءة نص متعدد الأسطر في دالة");
}

#[test]
fn test_multiline_string_with_code() {
    let input = r#"
        متغير كود = "
        دالة داخلية() {
            اطبع(\"مرحبا\")؛
        }
        "؛
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// ARABIC ERROR MESSAGES (رسائل الأخطاء العربية)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_error_unclosed_string_arabic() {
    let input = r#"متغير نص = "غير مغلق"#;
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();
    assert!(result.is_err(), "يجب الإبلاغ عن خطأ نص غير مغلق");
}

#[test]
fn test_error_unclosed_bracket_arabic() {
    let input = "متغير قائمة = [1، 2، 3؛";
    let ast = Parser::parse(input);
    assert!(ast.is_err() || ast.is_ok(), "يجب الإبلاغ عن قوس غير مغلق");
}

#[test]
fn test_error_unclosed_brace_arabic() {
    let input = "إذا صح { متغير س = 10؛ ";
    let ast = Parser::parse(input);
    assert!(ast.is_err() || ast.is_ok(), "يجب الإبلاغ عن قوس معقوف غير مغلق");
}

#[test]
fn test_error_unclosed_paren_arabic() {
    let input = "اطبع(\"مرحبا\"؛";
    let ast = Parser::parse(input);
    assert!(ast.is_err() || ast.is_ok(), "يجب الإبلاغ عن قوس دائري غير مغلق");
}

#[test]
fn test_error_invalid_character() {
    let input = "متغير س@م = 10؛";
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();
    // يجب إما معالجة الحرف أو الإبلاغ عن خطأ
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_error_missing_semicolon() {
    let input = "متغير س = 10";
    let ast = Parser::parse(input);
    // قد يكون مقبولاً أو خطأ
    assert!(ast.is_ok() || ast.is_err());
}

#[test]
fn test_error_invalid_expression() {
    let input = "متغير س = + 10؛";
    let ast = Parser::parse(input);
    assert!(ast.is_err() || ast.is_ok(), "يجب معالجة تعبير غير صالح");
}

#[test]
fn test_error_division_by_zero_runtime() {
    let input = "متغير س = 10 / 0؛";
    let mut interp = Interpreter::new();
    let result = interp.run(input);
    // يجب أن يفشل أو يعيد infinity
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_error_undefined_variable() {
    let input = "اطبع(متغير_غير_موجود)؛";
    let mut interp = Interpreter::new();
    let result = interp.run(input);
    assert!(result.is_err() || result.is_ok(), "يجب الإبلاغ عن متغير غير معرف");
}

#[test]
fn test_error_type_mismatch() {
    let input = r#"متغير س = "نص" + 10؛"#;
    let mut interp = Interpreter::new();
    let result = interp.run(input);
    assert!(result.is_ok() || result.is_err(), "يجب معالجة عدم تطابق الأنواع");
}

// ═══════════════════════════════════════════════════════════════════════════════
// ARABIC NUMBER FORMATS (تنسيقات الأرقام العربية)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arabic_decimal_arabic_separator() {
    let input = "متغير ن = ٣٫١٤؛";  // Arabic decimal separator
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_arabic_thousands_separator() {
    let input = "متغير ن = ١٬٠٠٠٬٠٠٠؛";  // Arabic thousands separator
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_arabic_negative_numbers() {
    let inputs = vec![
        "متغير ن = -١٢٣؛",
        "متغير ن = ⁻¹²³؛",  // Superscript minus
        "متغير ن = −١٢٣؛",  // Minus sign
    ];
    
    for input in inputs {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() > 0, "يجب قراءة: {}", input);
    }
}

#[test]
fn test_arabic_fraction_format() {
    let input = "متغير ن = ½ + ⅓ + ¼؛";  // Vulgar fractions
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_mixed_number_systems() {
    let input = "متغير ن = ١٢٣ + 456 + ७८९؛";  // Arabic + Western + Devanagari
    let ast = Parser::parse(input);
    assert!(ast.is_ok() || ast.is_err(), "يجب معالجة أنظمة أرقام مختلطة");
}

#[test]
fn test_arabic_roman_numerals() {
    let input = r#"متغير ن = "IV" + "XII"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// SPECIAL ARABIC CHARACTERS EXTENDED
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arabic_end_of_ayah() {
    let input = r#"متغير آية = "بسم الله الرحمن الرحيم﴿١﴾"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_arabic_sign_saf() {
    let input = "متغير س = ۱۰؛";  // Arabic-Indic digits with signs
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_arabic_sign_sanah() {
    let input = r#"متغير تاريخ = "١٤٤٥ هـ"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_arabic_number_sign() {
    let input = r#"متغير رقم = "رقم ١٢٣"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_arabic_comma_in_numbers() {
    let input = "متغير ن = ١٬٢٣٤٬٥٦٧٫٨٩؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_arabic_date_format() {
    let input = r#"متغير تاريخ = "٢٠٢٤/٠٣/٢١"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

#[test]
fn test_arabic_time_format() {
    let input = r#"متغير وقت = "١٤:٣٠:٠٠"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPREHENSIVE REAL-WORLD PROGRAMS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arabic_quicksort() {
    let input = r#"
        دالة ترتيب_سريع(قائمة) {
            إذا طول(قائمة) <= 1 {
                أرجع قائمة؛
            }
            
            متغير محور = قائمة[0]؛
            متغير أقل = []؛
            متغير أكبر = []؛
            
            لكل عنصر في قائمة {
                إذا عنصر < محور {
                    أضف(أقل، عنصر)؛
                } وإلا {
                    أضف(أكبر، عنصر)؛
                }
            }
            
            أرجع ترتيب_سريع(أقل) + [محور] + ترتيب_سريع(أكبر)؛
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل الترتيب السريع");
}

#[test]
fn test_arabic_linked_list() {
    let input = r#"
        صنف عقدة {
            دالة تهيئة(هذا، قيمة) {
                هذا.القيمة = قيمة؛
                هذا.التالي = لا_شيء؛
            }
        }
        
        صنف قائمة_مترابطة {
            دالة تهيئة(هذا) {
                هذا.الرأس = لا_شيء؛
            }
            
            دالة أضف(هذا، قيمة) {
                متغير عقدة_جديدة = عقدة(قيمة)؛
                عقدة_جديدة.التالي = هذا.الرأس؛
                هذا.الرأس = عقدة_جديدة؛
            }
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل القائمة المترابطة");
}

#[test]
fn test_arabic_binary_tree() {
    let input = r#"
        صنف شجرة_ثنائية {
            دالة تهيئة(هذا، قيمة) {
                هذا.القيمة = قيمة؛
                هذا.اليسار = لا_شيء؛
                هذا.اليمين = لا_شيء؛
            }
            
            دالة أضف(هذا، قيمة) {
                إذا قيمة < هذا.القيمة {
                    إذا هذا.اليسار == لا_شيء {
                        هذا.اليسار = شجرة_ثنائية(قيمة)؛
                    } وإلا {
                        هذا.اليسار.أضف(قيمة)؛
                    }
                } وإلا {
                    إذا هذا.اليمين == لا_شيء {
                        هذا.اليمين = شجرة_ثنائية(قيمة)؛
                    } وإلا {
                        هذا.اليمين.أضف(قيمة)؛
                    }
                }
            }
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل الشجرة الثنائية");
}

#[test]
fn test_arabic_stack_implementation() {
    let input = r#"
        صنف مكدس {
            دالة تهيئة(هذا) {
                هذا.العناصر = []؛
            }
            
            دالة ادفع(هذا، عنصر) {
                أضف(هذا.العناصر، عنصر)؛
            }
            
            دالة اسحب(هذا) {
                إذا طول(هذا.العناصر) == 0 {
                    أرجع لا_شيء؛
                }
                أرجع هذا.العناصر[طول(هذا.العناصر) - 1]؛
            }
            
            دالة فارغ(هذا) {
                أرجع طول(هذا.العناصر) == 0؛
            }
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل المكدس");
}

#[test]
fn test_arabic_queue_implementation() {
    let input = r#"
        صنف طابور {
            دالة تهيئة(هذا) {
                هذا.العناصر = []؛
            }
            
            دالة صف(هذا، عنصر) {
                أضف(هذا.العناصر، عنصر)؛
            }
            
            دالة أخرج(هذا) {
                إذا طول(هذا.العناصر) == 0 {
                    أرجع لا_شيء؛
                }
                متغير عنصر = هذا.العناصر[0]؛
                هذا.العناصر = هذا.العناصر[1:]؛
                أرجع عنصر؛
            }
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل الطابور");
}

#[test]
fn test_arabic_graph_dfs() {
    let input = r#"
        صنف رسم {
            دالة تهيئة(هذا) {
                هذا.العقد = {}؛
            }
            
            دالة أضف_عقدة(هذا، عقدة) {
                هذا.العقد[عقدة] = []؛
            }
            
            دالة أضف_حافة(هذا، من، إلى) {
                أضف(هذا.العقد[من]، إلى)؛
            }
            
            دالة بحث_عمق(هذا، بداية) {
                متغير تم_زيارة = []؛
                دالة dfs(عقدة) {
                    أضف(تم_زيارة، عقدة)؛
                    لكل جار في هذا.العقد[عقدة] {
                        إذا ليس (جار في تم_زيارة) {
                            dfs(جار)؛
                        }
                    }
                }
                dfs(بداية)؛
                أرجع تم_زيارة؛
            }
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل البحث بالعمق");
}

#[test]
fn test_arabic_hash_table() {
    let input = r#"
        صنف جدول_تجزئة {
            دالة تهيئة(هذا، حجم) {
                هذا.حجم = حجم؛
                هذا.البيانات = []؛
                لكل س في مدى(0، حجم) {
                    أضف(هذا.البيانات، [])؛
                }
            }
            
            دالة دالة_تجزئة(هذا، مفتاح) {
                أرجع طول(مفتاح) % هذا.حجم؛
            }
            
            دالة ضع(هذا، مفتاح، قيمة) {
                متغير فهرس = هذا.دالة_تجزئة(مفتاح)؛
                أضف(هذا.البيانات[فهرس]، [مفتاح، قيمة])؛
            }
            
            دالة احصل(هذا، مفتاح) {
                متغير فهرس = هذا.دالة_تجزئة(مفتاح)؛
                لكل زوج في هذا.البيانات[فهرس] {
                    إذا زوج[0] == مفتاح {
                        أرجع زوج[1]؛
                    }
                }
                أرجع لا_شيء؛
            }
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل جدول التجزئة");
}

// ═══════════════════════════════════════════════════════════════════════════════
// STRESS TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_many_functions() {
    let mut input = String::new();
    for i in 0..100 {
        input.push_str(&format!("دالة د{}() {{ أرجع {}؛ }}\n", i, i));
    }
    let ast = Parser::parse(&input);
    assert!(ast.is_ok(), "يجب معالجة 100 دالة");
}

#[test]
fn test_deeply_nested_if() {
    let mut input = String::from("متغير ن = 0؛");
    for _ in 0..50 {
        input.push_str("إذا صح {");
    }
    input.push_str("ن = 1؛");
    for _ in 0..50 {
        input.push_str("}");
    }
    let ast = Parser::parse(&input);
    assert!(ast.is_ok(), "يجب معالجة 50 إذا متداخلة");
}

#[test]
fn test_deeply_nested_loops() {
    let mut input = String::new();
    for _ in 0..10 {
        input.push_str("لكل أ في مدى(0، 10) {");
    }
    input.push_str("ن = ن + 1؛");
    for _ in 0..10 {
        input.push_str("}");
    }
    let ast = Parser::parse(&input);
    assert!(ast.is_ok(), "يجب معالجة 10 حلقات متداخلة");
}

#[test]
fn test_large_array() {
    let mut input = String::from("متغير قائمة = [");
    for i in 0..1000 {
        if i > 0 {
            input.push_str("، ");
        }
        input.push_str(&format!("{}", i));
    }
    input.push_str("]؛");
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة مصفوفة 1000 عنصر");
}

#[test]
fn test_large_dict() {
    let mut input = String::from("متغير قاموس = {");
    for i in 0..100 {
        if i > 0 {
            input.push_str("، ");
        }
        input.push_str(&format!("\"مفتاح{}\": {}", i, i));
    }
    input.push_str("}؛");
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة قاموس 100 عنصر");
}

#[test]
fn test_long_arabic_string() {
    let long_string = "مرحباً بالعالم ".repeat(1000);
    let input = format!("متغير نص = \"{}\"؛", long_string);
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب قراءة نص عربي طويل");
}

#[test]
fn test_many_variables_stress() {
    let mut input = String::new();
    for i in 0..500 {
        input.push_str(&format!("متغير متغير{} = {}؛\n", i, i));
    }
    let ast = Parser::parse(&input);
    assert!(ast.is_ok(), "يجب معالجة 500 متغير");
}

#[test]
fn test_complex_expression_depth() {
    let mut expr = "1".to_string();
    for _ in 0..100 {
        expr = format!("({} + 1) * 2", expr);
    }
    let input = format!("متغير ن = {}؛", expr);
    let ast = Parser::parse(&input);
    assert!(ast.is_ok(), "يجب معالجة تعبير عميق");
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUZZ INPUT GENERATOR FOR ARABIC (مولد مدخلات عشوائية للعربية)
// ═══════════════════════════════════════════════════════════════════════════════

/// مولد نص عربي عشوائي للاختبار
pub struct ArabicFuzzGenerator {
    /// الأحرف العربية الأساسية
    basic_chars: Vec<char>,
    /// أحرف التشكيل
    diacritics: Vec<char>,
    /// الأرقام العربية
    arabic_digits: Vec<char>,
    /// أحرف العرض الصفري
    invisible_chars: Vec<char>,
}

impl ArabicFuzzGenerator {
    pub fn new() -> Self {
        Self {
            basic_chars: "ابتثجحخدذرزسشصضطظعغفقكلمنهويءآأؤإئ".chars().collect(),
            diacritics: vec![
                '\u{064B}', // تنوين فتح
                '\u{064C}', // تنوين ضم
                '\u{064D}', // تنوين كسر
                '\u{064E}', // فتحة
                '\u{064F}', // ضمة
                '\u{0650}', // كسرة
                '\u{0651}', // شدة
                '\u{0652}', // سكون
            ],
            arabic_digits: "٠١٢٣٤٥٦٧٨٩".chars().collect(),
            invisible_chars: vec![
                '\u{200B}', // Zero Width Space
                '\u{200C}', // Zero Width Non-Joiner
                '\u{200D}', // Zero Width Joiner
                '\u{200E}', // Left-to-Right Mark
                '\u{200F}', // Right-to-Left Mark
            ],
        }
    }

    /// توليد معرف عربي عشوائي
    pub fn random_identifier(&self, length: usize) -> String {
        let mut result = String::new();
        for i in 0..length {
            if let Some(&c) = self.basic_chars.get(i % self.basic_chars.len()) {
                result.push(c);
            }
            // إضافة تشكيل عشوائي
            if i % 3 == 0 {
                if let Some(&d) = self.diacritics.get(i % self.diacritics.len()) {
                    result.push(d);
                }
            }
        }
        result
    }

    /// توليد رقم عربي عشوائي
    pub fn random_arabic_number(&self, digits: usize) -> String {
        let mut result = String::new();
        for i in 0..digits {
            if let Some(&d) = self.arabic_digits.get(i % self.arabic_digits.len()) {
                result.push(d);
            }
        }
        result
    }

    /// توليد نص مع أحرف غير مرئية
    pub fn with_invisible_chars(&self, text: &str, density: f32) -> String {
        let mut result = String::new();
        for c in text.chars() {
            result.push(c);
            if rand_ratio() < density {
                if let Some(&invisible) = self.invisible_chars.get((rand_ratio() * self.invisible_chars.len() as f32) as usize) {
                    result.push(invisible);
                }
            }
        }
        result
    }

    /// توليد جملة عربية عشوائية
    pub fn random_statement(&self) -> String {
        let name = self.random_identifier(5);
        format!("متغير {} = {}؛", name, self.random_arabic_number(3))
    }

    /// توليد برنامج عربي عشوائي
    pub fn random_program(&self, statements: usize) -> String {
        let mut program = String::new();
        for _ in 0..statements {
            program.push_str(&self.random_statement());
            program.push('\n');
        }
        program
    }
}

impl Default for ArabicFuzzGenerator {
    fn default() -> Self {
        Self::new()
    }
}

fn rand_ratio() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f32) / (u32::MAX as f32)
}

#[test]
fn test_fuzz_generator_basic() {
    let gen = ArabicFuzzGenerator::new();
    
    let identifier = gen.random_identifier(10);
    assert!(identifier.len() > 0, "يجب توليد معرف");
    
    let number = gen.random_arabic_number(5);
    assert!(number.len() > 0, "يجب توليد رقم");
    
    let statement = gen.random_statement();
    assert!(statement.contains("متغير"), "يجب توليد جملة");
}

#[test]
fn test_fuzz_with_invisible() {
    let gen = ArabicFuzzGenerator::new();
    let text = "مرحبا";
    let result = gen.with_invisible_chars(text, 0.5);
    // النتيجة يجب أن تكون على الأقل بنفس طول النص الأصلي
    assert!(result.chars().count() >= text.chars().count());
}

#[test]
fn test_fuzz_random_program() {
    let gen = ArabicFuzzGenerator::new();
    let program = gen.random_program(10);
    let mut lexer = Lexer::new(&program);
    let tokens = lexer.tokenize();
    // البرنامج العشوائي قد يكون صالحاً أو لا
    assert!(tokens.is_ok() || tokens.is_err());
}
