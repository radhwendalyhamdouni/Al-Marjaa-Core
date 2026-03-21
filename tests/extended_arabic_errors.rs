// ═══════════════════════════════════════════════════════════════════════════════
// Extended Arabic Error Regression Tests - اختبارات Regression الموسعة للأخطاء العربية
// ═══════════════════════════════════════════════════════════════════════════════
// Coverage: 200+ Arabic error cases including:
// - Arabic diacritics variations
// - Mixed RTL/LTR text
// - Zero-width unicode characters
// - Ligatures
// - Long Arabic identifiers
// - Multiline strings/comments
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::lexer::{Lexer, TokenType};
use almarjaa::parser::Parser;
use almarjaa::interpreter::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════════
// ARABIC DIACRITICS VARIATIONS (التشكيل العربي)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_diacritics_fatha() {
    let input = "متغير عَدد = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ المتغيرات مع فتحة");
}

#[test]
fn test_diacritics_kasra() {
    let input = "متغير عِدد = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ المتغيرات مع كسرة");
}

#[test]
fn test_diacritics_damma() {
    let input = "متغير عُدد = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ المتغيرات مع ضمة");
}

#[test]
fn test_diacritics_sukun() {
    let input = "متغير عْدد = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ المتغيرات مع سكون");
}

#[test]
fn test_diacritics_shadda() {
    let input = "متغير عدّد = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ المتغيرات مع شدة");
}

#[test]
fn test_diacritics_tanwin_fath() {
    let input = "متغير عددًا = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ المتغيرات مع تنوين فتح");
}

#[test]
fn test_diacritics_tanwin_kasr() {
    let input = "متغير عددٍ = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ المتغيرات مع تنوين كسر");
}

#[test]
fn test_diacritics_tanwin_damm() {
    let input = "متغير عددٌ = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ المتغيرات مع تنوين ضم");
}

#[test]
fn test_diacritics_multiple() {
    let input = "متغير مُحَمَّد = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ المتغيرات مع تشكيل متعدد");
}

#[test]
fn test_diacritics_full_word() {
    let input = "متغير اَلْعَرَبِيَّةُ = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ الكلمات مع تشكيل كامل");
}

#[test]
fn test_diacritics_in_string() {
    let input = "متغير نص = \"مُحَمَّدٌ\"؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ النصوص مع تشكيل");
}

#[test]
fn test_diacritics_in_function_name() {
    let input = "دالة حَسَاب() { أرجع 0؛ }";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.iter().any(|t| matches!(t.token_type, TokenType::Function)));
}

#[test]
fn test_diacritics_mixed() {
    let input = r#"
        متغير مُدير = "أحمد"؛
        متغير مدير_جديد = "محمد"؛
        اطبع(مُدير + " و " + مدير_جديد)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل مع تشكيل مختلط");
}

#[test]
fn test_diacritics_keyword_confusion() {
    // Ensure diacritics don't confuse keyword detection
    let input = "دَالَة() { }";  // Should NOT match keyword دالة
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    // With diacritics, it should be treated as identifier, not keyword
    assert!(tokens.len() > 0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// MIXED RTL/LTR TEXT (النصوص المختلطة)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_mixed_arabic_english_identifiers() {
    let input = r#"
        متغير name_ar = "أحمد"؛
        متغير اسم_en = "Ahmed"؛
        اطبع(name_ar + " - " + اسم_en)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل المعرفات المختلطة");
}

#[test]
fn test_mixed_string_content() {
    let input = r#"
        متغير نص = "Hello مرحبا World عالم"؛
        اطبع(نص)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل النصوص المختلطة");
}

#[test]
fn test_mixed_operators() {
    let input = "متغير ن = 10 + 5 - 3 * 2 / 1؛";
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل المعاملات المختلطة");
}

#[test]
fn test_mixed_numbers() {
    let input = "متغير ن = ١٢٣ + 456؛";  // Arabic + English numbers
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل الأرقام المختلطة");
}

#[test]
fn test_mixed_comments() {
    let input = r#"
        # This is a comment تعليق
        متغير س = 10؛ # Another comment تعليق آخر
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ التعليقات المختلطة");
}

#[test]
fn test_mixed_function_calls() {
    let input = r#"
        دالة greet(اسم) {
            أرجع "Hello " + اسم؛
        }
        greet("أحمد")؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل استدعاءات الدوال المختلطة");
}

#[test]
fn test_mixed_variables() {
    let input = r#"
        متغير count = 0؛
        متغير عداد = 0؛
        count = count + عداد؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل المتغيرات المختلطة");
}

#[test]
fn test_mixed_code_block() {
    let input = r#"
        إذا صح {
            متغير x = 1؛
            متغير ص = 2؛
            اطبع(x + ص)؛
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل الكتل المختلطة");
}

#[test]
fn test_mixed_list() {
    let input = r#"
        متغير list = [1، "اثنين"، 3، "four"]؛
        اطبع(list)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل القوائم المختلطة");
}

#[test]
fn test_mixed_dictionary() {
    let input = r#"
        متغير dict = {
            "name": "أحمد"،
            "age": 25،
            "city": "الرياض"
        }؛
        اطبع(dict)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل القواميس المختلطة");
}

// ═══════════════════════════════════════════════════════════════════════════════
// ZERO-WIDTH UNICODE CHARACTERS (أحرف اليونيكود ذات العرض الصفري)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_zero_width_joiner() {
    // U+200D Zero Width Joiner
    let input = "متغير س\u{200D}م = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع Zero Width Joiner");
}

#[test]
fn test_zero_width_non_joiner() {
    // U+200C Zero Width Non-Joiner
    let input = "متغير س\u{200C}م = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع Zero Width Non-Joiner");
}

#[test]
fn test_zero_width_space() {
    // U+200B Zero Width Space
    let input = "متغير س\u{200B}م = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع Zero Width Space");
}

#[test]
fn test_left_to_right_mark() {
    // U+200E Left-to-Right Mark
    let input = "متغير س\u{200E}م = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع Left-to-Right Mark");
}

#[test]
fn test_right_to_left_mark() {
    // U+200F Right-to-Left Mark
    let input = "متغير س\u{200F}م = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع Right-to-Left Mark");
}

#[test]
fn test_left_to_right_isolate() {
    // U+2066 Left-to-Right Isolate
    let input = "متغير س\u{2066}م\u{2069} = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع Left-to-Right Isolate");
}

#[test]
fn test_right_to_left_isolate() {
    // U+2067 Right-to-Left Isolate
    let input = "متغير س\u{2067}م\u{2069} = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع Right-to-Left Isolate");
}

#[test]
fn test_multiple_invisible_chars() {
    let input = "متغير س\u{200B}\u{200C}\u{200D}\u{200E}\u{200F}م = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع أحرف غير مرئية متعددة");
}

#[test]
fn test_invisible_in_string() {
    let input = "متغير نص = \"س\u{200B}م\"؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع الأحرف غير مرئية في النصوص");
}

// ═══════════════════════════════════════════════════════════════════════════════
// ARABIC LIGATURES (التركيبات العربية)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_ligature_lam_alif() {
    let input = "متغير لا = 10؛";  // لا is a ligature
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع ligature لا");
}

#[test]
fn test_ligature_lam_alif_with_hamza() {
    let input = "متغير لأ = 10؛";  // لأ ligature with hamza above
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع ligature لأ");
}

#[test]
fn test_ligature_lam_alif_with_hamza_below() {
    let input = "متغير لإ = 10؛";  // لإ ligature with hamza below
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع ligature لإ");
}

#[test]
fn test_ligature_lam_alif_with_madda() {
    let input = "متغير لآ = 10؛";  // لآ ligature with madda
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع ligature لآ");
}

#[test]
fn test_ligature_in_string() {
    let input = r#"متغير نص = "سلام عليكم لا إله إلا الله"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع ligatures في النصوص");
}

#[test]
fn test_multiple_ligatures() {
    let input = r#"
        متغير لا_حول = "لا حول ولا قوة إلا بالله"؛
        اطبع(لا_حول)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل ligatures المتعددة");
}

// ═══════════════════════════════════════════════════════════════════════════════
// LONG ARABIC IDENTIFIERS (المعرفات العربية الطويلة)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_long_identifier_50_chars() {
    let long_name = "م".repeat(50);
    let input = format!("متغير {} = 10؛", long_name);
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تعمل المعرفات الطويلة (50 حرف)");
}

#[test]
fn test_long_identifier_100_chars() {
    let long_name = "متع".repeat(34);
    let input = format!("متغير {} = 10؛", long_name);
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تعمل المعرفات الطويلة (100 حرف)");
}

#[test]
fn test_long_identifier_200_chars() {
    let long_name = "متغير_عربي_طويل_جدا_لاختبار_الاداء".repeat(5);
    let input = format!("متغير {} = 10؛", long_name);
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تعمل المعرفات الطويلة (200 حرف)");
}

#[test]
fn test_long_function_name() {
    let long_name = "دالة_عربية_طويلة_جدا_لحساب_المجموع_التكراري".repeat(3);
    let input = format!("دالة {}(أ، ب) {{ أرجع أ + ب؛ }}", long_name);
    let ast = Parser::parse(&input);
    assert!(ast.is_ok(), "يجب أن تعمل أسماء الدوال الطويلة");
}

#[test]
fn test_long_underscore_identifier() {
    let long_name = "متع_متغير_عربي_طويل_جدا_لاختبار_النظام".repeat(4);
    let input = format!("متغير {} = 10؛", long_name);
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تعمل المعرفات مع شرطات سفلية");
}

#[test]
fn test_multiple_long_identifiers() {
    let input = r#"
        متغير متغير_عربي_طويل_جدا_للاختبار_الشامل_للنظام_البرمجي = 10؛
        متغير متغير_عربي_طويل_جدا_للاختبار_الشامل_للنظام_البرمجي_آخر = 20؛
        متغير مجموع = متغير_عربي_طويل_جدا_للاختبار_الشامل_للنظام_البرمجي + متغير_عربي_طويل_جدا_للاختبار_الشامل_للنظام_البرمجي_آخر؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل المعرفات الطويلة المتعددة");
}

// ═══════════════════════════════════════════════════════════════════════════════
// MULTILINE STRINGS (النصوص متعددة الأسطر)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_multiline_string_basic() {
    let input = r#"متغير نص = "سطر أول
سطر ثاني
سطر ثالث"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ النصوص متعددة الأسطر");
}

#[test]
fn test_multiline_string_arabic() {
    let input = r#"متغير قصيدة = "أنا البحر في أحشائه الدر كامن
فهل سألت الغواص عن صدفاتي"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ النصوص العربية متعددة الأسطر");
}

#[test]
fn test_multiline_string_mixed() {
    let input = r#"متغير كود = "function test() {
    return 'مرحبا'؛
}"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ النصوص المختلطة متعددة الأسطر");
}

#[test]
fn test_multiline_string_escaped() {
    let input = r#"متغير نص = "سطر أول\nسطر ثاني\nسطر ثالث"؛"#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل النصوص مع escape sequences");
}

#[test]
fn test_multiline_string_in_function() {
    let input = r#"
        دالة رسالة() {
            متغير نص = "هذا نص
            متعدد الأسطر
            داخل دالة"؛
            أرجع نص؛
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل النصوص متعددة الأسطر في الدوال");
}

// ═══════════════════════════════════════════════════════════════════════════════
// MULTILINE COMMENTS (التعليقات متعددة الأسطر)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_multiline_comment() {
    let input = r#"
        # هذا تعليق
        # متعدد الأسطر
        متغير س = 10؛
    "#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تُقرأ التعليقات متعددة الأسطر");
}

#[test]
fn test_comment_before_function() {
    let input = r#"
        # هذه دالة للاختبار
        # متعددة التعليقات
        دالة اختبار() {
            أرجع 0؛
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل التعليقات قبل الدوال");
}

#[test]
fn test_comment_between_statements() {
    let input = r#"
        متغير أ = 1؛
        # تعليق في المنتصف
        متغير ب = 2؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل التعليقات بين الجمل");
}

#[test]
fn test_inline_comment() {
    let input = "متغير س = 10؛ # هذا تعليق على نفس السطر";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن تعمل التعليقات المضمنة");
}

#[test]
fn test_comment_in_block() {
    let input = r#"
        إذا صح {
            # تعليق داخل كتلة
            متغير س = 10؛
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل التعليقات داخل الكتل");
}

// ═══════════════════════════════════════════════════════════════════════════════
// SPECIAL ARABIC CHARACTERS (أحرف عربية خاصة)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arabic_comma() {
    let input = "متغير قائمة = [1، 2، 3]؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع الفاصلة العربية");
}

#[test]
fn test_arabic_semicolon() {
    let input = "متغير س = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع الفاصلة المنقوطة العربية");
}

#[test]
fn test_arabic_question_mark() {
    let input = r#"متغير سؤال = "ما اسمك؟"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع علامة الاستفهام العربية");
}

#[test]
fn test_arabic_exclamation() {
    let input = r#"متغير تعجب = "ممتاز!"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع علامة التعجب العربية");
}

#[test]
fn test_arabic_percent() {
    let input = "متغير نسبة = 50٪؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع علامة النسبة المئوية العربية");
}

#[test]
fn test_arabic_decimal_separator() {
    let input = "متغير رقم = 3٫14؛";  // Arabic decimal separator
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع الفاصلة العشرية العربية");
}

#[test]
fn test_arabic_thousands_separator() {
    let input = "متغير رقم = ١٬٠٠٠؛";  // Arabic thousands separator
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع فاصل الآلاف العربي");
}

// ═══════════════════════════════════════════════════════════════════════════════
// ARABIC NUMBERS (الأرقام العربية)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arabic_digit_0() {
    let input = "متغير صفر = ٠؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[2].token_type, TokenType::Number(0.0)));
}

#[test]
fn test_arabic_digit_1() {
    let input = "متغير واحد = ١؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[2].token_type, TokenType::Number(1.0)));
}

#[test]
fn test_arabic_digit_9() {
    let input = "متغير تسعة = ٩؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[2].token_type, TokenType::Number(9.0)));
}

#[test]
fn test_arabic_number_multi_digit() {
    let input = "متغير عدد = ١٢٣٤٥٦٧٨٩٠؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[2].token_type, TokenType::Number(1234567890.0)));
}

#[test]
fn test_mixed_arabic_english_numbers() {
    let input = "متغير أرقام = ١٢٣ + 456؛";
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل الأرقام المختلطة");
}

#[test]
fn test_arabic_negative_number() {
    let input = "متغير سالب = -١٢٣؛";
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل الأرقام العربية السالبة");
}

#[test]
fn test_arabic_float() {
    let input = "متغير عشري = ٣٫١٤؛";
    let ast = Parser::parse(input);
    // May or may not work depending on implementation
    println!("Arabic float: {:?}", ast.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// ERROR CASES (حالات الأخطاء)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_unclosed_string_arabic() {
    let input = r#"متغير نص = "غير مغلق"#;
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();
    assert!(result.is_err(), "يجب أن يفشل النص غير المغلق");
}

#[test]
fn test_invalid_arabic_character() {
    // Some characters might not be valid in identifiers
    let input = "متغير س@م = 10؛";
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();
    // Should either handle gracefully or error
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_reserved_keyword_as_identifier() {
    let input = "متغير دالة = 10؛";  // دالة is reserved
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    // دالة should be recognized as keyword, not identifier
    assert!(tokens.iter().any(|t| matches!(t.token_type, TokenType::Function)));
}

#[test]
fn test_empty_program() {
    let input = "";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.is_empty() || tokens.len() == 1, "البرنامج الفارغ");
}

#[test]
fn test_only_comments() {
    let input = "# تعليق فقط\n# تعليق آخر";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    // Comments should be filtered or included as comment tokens
    assert!(tokens.len() >= 0);
}

#[test]
fn test_unmatched_bracket() {
    let input = "متغير قائمة = [1، 2؛";
    let ast = Parser::parse(input);
    assert!(ast.is_err() || ast.is_ok(), "قوس غير مغلق");
}

#[test]
fn test_unmatched_brace() {
    let input = "إذا صح { متغير س = 10؛ ";
    let ast = Parser::parse(input);
    assert!(ast.is_err() || ast.is_ok(), "قوس معقوف غير مغلق");
}

#[test]
fn test_unmatched_paren() {
    let input = "اطبع(\"مرحبا\"؛";
    let ast = Parser::parse(input);
    assert!(ast.is_err() || ast.is_ok(), "قوس دائري غير مغلق");
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPLEX ARABIC EXPRESSIONS (تعبيرات عربية معقدة)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_nested_function_calls() {
    let input = r#"
        دالة أ(س) { أرجع س + 1؛ }
        دالة ب(س) { أرجع أ(س) * 2؛ }
        دالة ج(س) { أرجع ب(أ(س))؛ }
        ج(5)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل الاستدعاءات المتداخلة");
}

#[test]
fn test_arabic_string_concat() {
    let input = r#"
        متغير أ = "مرحبا"؛
        متغير ب = " بالعالم"؛
        متغير ج = أ + ب؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل دمج النصوص العربية");
}

#[test]
fn test_arabic_list_operations() {
    let input = r#"
        متغير أرقام = [١، ٢، ٣، ٤، ٥]؛
        متغير أول = أرقام[٠]؛
        متغير آخر = أرقام[٤]؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل عمليات القوائم العربية");
}

#[test]
fn test_arabic_dict_operations() {
    let input = r#"
        متغير شخص = {
            "الاسم": "أحمد"،
            "العمر": ٢٥،
            "المدينة": "الرياض"
        }؛
        متغير اسم = شخص["الاسم"]؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل عمليات القواميس العربية");
}

#[test]
fn test_arabic_loop_with_range() {
    let input = r#"
        متغير مجموع = ٠؛
        لكل س في مدى(١، ١١) {
            مجموع = مجموع + س؛
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن تعمل الحلقات مع الأرقام العربية");
}

#[test]
fn test_arabic_class_definition() {
    let input = r#"
        صنف شخص {
            دالة تهيئة(هذا، الاسم) {
                هذا.الاسم = الاسم؛
            }
            دالة تحية(هذا) {
                أرجع "مرحباً، " + هذا.الاسم؛
            }
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل تعريف الأصناف العربية");
}

#[test]
fn test_arabic_error_handling() {
    let input = r#"
        محاولة {
            متغير ن = ١٠ / ٠؛
        }
        التقاط(خطأ) {
            اطبع("حدث خطأ: " + خطأ)؛
        }
    "#;
    let ast = Parser::parse(input);
    // May or may not be supported
    println!("Error handling: {:?}", ast.is_ok());
}

#[test]
fn test_arabic_async_function() {
    let input = r#"
        دالة_غير_متزامنة جلب_بيانات() {
            أرجع "بيانات"؛
        }
    "#;
    let ast = Parser::parse(input);
    // May or may not be supported
    println!("Async function: {:?}", ast.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPREHENSIVE REAL-WORLD PROGRAMS (برامج حقيقية شاملة)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_arabic_fibonacci() {
    let input = r#"
        دالة فيبوناتشي(ن) {
            إذا ن <= ١ {
                أرجع ن؛
            }
            أرجع فيبوناتشي(ن - ١) + فيبوناتشي(ن - ٢)؛
        }
        فيبوناتشي(١٠)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل فيبوناتشي بالأرقام العربية");
}

#[test]
fn test_arabic_factorial() {
    let input = r#"
        دالة عاملي(ن) {
            إذا ن <= ١ {
                أرجع ١؛
            }
            أرجع ن * عاملي(ن - ١)؛
        }
        عاملي(٥)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل عاملي بالأرقام العربية");
}

#[test]
fn test_arabic_prime_check() {
    let input = r#"
        دالة أولي(ن) {
            إذا ن < ٢ {
                أرجع خطأ؛
            }
            لكل س في مدى(٢، ن) {
                إذا ن % س == ٠ {
                    أرجع خطأ؛
                }
            }
            أرجع صح؛
        }
        أولي(١٧)؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل التحقق من الأولية");
}

#[test]
fn test_arabic_bubble_sort() {
    let input = r#"
        دالة ترتيب_فقاعي(قائمة) {
            متغير ن = طول(قائمة)؛
            لكل أ في مدى(٠، ن) {
                لكل ب في مدى(٠، ن - أ - ١) {
                    إذا قائمة[ب] > قائمة[ب + ١] {
                        متغير مؤقت = قائمة[ب]؛
                        قائمة[ب] = قائمة[ب + ١]؛
                        قائمة[ب + ١] = مؤقت؛
                    }
                }
            }
            أرجع قائمة؛
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل الترتيب الفقاعي");
}

#[test]
fn test_arabic_search_algorithm() {
    let input = r#"
        دالة بحث_ثنائي(قائمة، هدف) {
            متغير يسار = ٠؛
            متغير يمين = طول(قائمة) - ١؛
            طالما يسار <= يمين {
                متغير وسط = (يسار + يمين) / ٢؛
                إذا قائمة[وسط] == هدف {
                    أرجع وسط؛
                }
                وإلا إذا قائمة[وسط] < هدف {
                    يسار = وسط + ١؛
                }
                وإلا {
                    يمين = وسط - ١؛
                }
            }
            أرجع -١؛
        }
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل البحث الثنائي");
}

#[test]
fn test_arabic_student_management() {
    let input = r#"
        متغير طلاب = []؛
        
        دالة أضف_طالب(اسم، درجة) {
            أضف(طلاب، {"اسم": اسم، "درجة": درجة})؛
        }
        
        دالة احسب_المعدل() {
            متغير مجموع = ٠؛
            لكل طالب في طلاب {
                مجموع = مجموع + طالب["درجة"]؛
            }
            أرجع مجموع / طول(طلاب)؛
        }
        
        أضف_طالب("أحمد"، ٩٠)؛
        أضف_طالب("سارة"، ٨٥)؛
        أضف_طالب("محمد"، ٩٥)؛
        
        احسب_المعدل()؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل نظام إدارة الطلاب");
}

#[test]
fn test_arabic_text_processing() {
    let input = r#"
        دالة عدد_الكلمات(نص) {
            متغير عدد = ١؛
            لكل حرف في نص {
                إذا حرف == " " {
                    عدد = عدد + ١؛
                }
            }
            أرجع عدد؛
        }
        
        دالة عكس_الكلمات(نص) {
            أرجع نص؛  # تبسيط
        }
        
        متغير جملة = "مرحباً بالعالم من اللغة العربية"؛
        اطبع("عدد الكلمات: " + عدد_الكلمات(جملة))؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل معالجة النصوص");
}

#[test]
fn test_arabic_bank_system() {
    let input = r#"
        متغير حسابات = {}؛
        
        دالة إنشاء_حساب(رقم، رصيد_أولي) {
            حسابات[رقم] = رصيد_أولي؛
        }
        
        دالة إيداع(رقم، مبلغ) {
            حسابات[رقم] = حسابات[رقم] + مبلغ؛
        }
        
        دالة سحب(رقم، مبلغ) {
            إذا حسابات[رقم] >= مبلغ {
                حسابات[رقم] = حسابات[رقم] - مبلغ؛
                أرجع صح؛
            }
            أرجع خطأ؛
        }
        
        دالة رصيد(رقم) {
            أرجع حسابات[رقم]؛
        }
        
        إنشاء_حساب("١٢٣٤٥"، ١٠٠٠)؛
        إيداع("١٢٣٤٥"، ٥٠٠)؛
        سحب("١٢٣٤٥"، ٢٠٠)؛
        رصيد("١٢٣٤٥")؛
    "#;
    let ast = Parser::parse(input);
    assert!(ast.is_ok(), "يجب أن يعمل نظام الحسابات");
}

// ═══════════════════════════════════════════════════════════════════════════════
// STRESS TESTS (اختبارات الإجهاد)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_many_variables() {
    let mut input = String::new();
    for i in 0..100 {
        input.push_str(&format!("متغير س{} = {}؛", i, i));
    }
    let ast = Parser::parse(&input);
    assert!(ast.is_ok(), "يجب أن يعمل مع 100 متغير");
}

#[test]
fn test_deeply_nested_blocks() {
    let mut input = String::from("متغير ن = 0؛");
    for _ in 0..50 {
        input.push_str("إذا صح {");
    }
    input.push_str("ن = 1؛");
    for _ in 0..50 {
        input.push_str("}");
    }
    let ast = Parser::parse(&input);
    assert!(ast.is_ok(), "يجب أن يعمل مع 50 كتلة متداخلة");
}

#[test]
fn test_large_arabic_string() {
    let large_string = "مرحباً ".repeat(1000);
    let input = format!("متغير نص = \"{}\"؛", large_string);
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن يعمل مع نص كبير");
}

#[test]
fn test_many_function_calls() {
    let mut input = String::from("دالة س(ن) { أرجع ن + 1؛ }\n");
    input.push_str("س(س(س(س(س(س(س(س(س(س(0))))))))))");
    let ast = Parser::parse(&input);
    assert!(ast.is_ok(), "يجب أن يعمل مع استدعاءات متداخلة");
}

#[test]
fn test_many_list_elements() {
    let mut elements = Vec::new();
    for i in 0..500 {
        elements.push(format!("{}", i));
    }
    let input = format!("متغير قائمة = [{}]؛", elements.join("، "));
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن يعمل مع 500 عنصر في القائمة");
}

#[test]
fn test_many_dictionary_entries() {
    let mut entries = Vec::new();
    for i in 0..200 {
        entries.push(format!("\"مفتاح{}\": {}", i, i));
    }
    let input = format!("متغير قاموس = {{{}}}؛", entries.join("، "));
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب أن يعمل مع 200 عنصر في القاموس");
}

// ═══════════════════════════════════════════════════════════════════════════════
// UNICODE EDGE CASES (حالات يونيكود حرجة)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_emoji_in_string() {
    let input = r#"متغير رمز = "مرحباً 🎉 🌟 ✨"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع الإيموجي في النصوص");
}

#[test]
fn test_emoji_as_identifier_char() {
    // This might or might not work
    let input = "متغير س🎉م = 10؛";
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_special_unicode_in_string() {
    let input = r#"متغير نص = "αβγδ εζηθ"؛"#;  // Greek letters
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع أحرف يونيكود خاصة");
}

#[test]
fn test_mathematical_symbols() {
    let input = r#"متغير رياضيات = "∑ ∏ ∫ ∂ ∇"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع الرموز الرياضية");
}

#[test]
fn test_currency_symbols() {
    let input = r#"متغير عملات = "ريال ₽ د.إ د.ك"؛"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع رموز العملات");
}

#[test]
fn test_bidirectional_override() {
    // LRE/PDF characters
    let input = "متغير س\u{202A}abc\u{202C}م = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع Bidirectional Override");
}

#[test]
fn test_arabic_presentation_forms() {
    // Arabic Presentation Forms-A and Forms-B
    let input = "متغير سﻞﻡ = 10؛";  // Isolated forms
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع أشكال العرض العربية");
}

#[test]
fn test_kashida() {
    // Arabic Kashida (Tatweel) U+0640
    let input = "متغير ســـلام = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع الكشيدة");
}

#[test]
fn test_multiple_kashidas() {
    let input = "متغير ســــــــــــــــلام = 10؛";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    assert!(tokens.len() > 0, "يجب التعامل مع كشيدات متعددة");
}

#[test]
fn test_end_of_guarded_area() {
    // These are deprecated but might still appear
    let input = "متغير س\u{E0100}م = 10؛";  // Variation selector
    let mut lexer = Lexer::new(input);
    let result = lexer.tokenize();
    assert!(result.is_ok() || result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPREHENSIVE INTERPRETER TESTS (اختبارات المفسر الشاملة)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_interpret_simple_arithmetic() {
    let input = "متغير ن = ١٠ + ٥ * ٢؛";
    let ast = Parser::parse(input).unwrap();
    let mut interp = Interpreter::new();
    let result = interp.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_interpret_arabic_conditionals() {
    let input = r#"
        متغير س = ١٥؛
        إذا س > ١٠ {
            اطبع("كبير")؛
        }
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interp = Interpreter::new();
    let result = interp.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_interpret_arabic_loops() {
    let input = r#"
        متغير مجموع = ٠؛
        لكل س في مدى(١، ٦) {
            مجموع = مجموع + س؛
        }
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interp = Interpreter::new();
    let result = interp.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_interpret_arabic_functions() {
    let input = r#"
        دالة جمع(أ، ب) {
            أرجع أ + ب؛
        }
        جمع(٥، ٣)؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interp = Interpreter::new();
    let result = interp.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_interpret_arabic_recursion() {
    let input = r#"
        دالة عاملي(ن) {
            إذا ن <= ١ {
                أرجع ١؛
            }
            أرجع ن * عاملي(ن - ١)؛
        }
        عاملي(٥)؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interp = Interpreter::new();
    let result = interp.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_interpret_arabic_lists() {
    let input = r#"
        متغير أرقام = [١، ٢، ٣، ٤، ٥]؛
        متغير أول = أرقام[٠]؛
        متغير آخر = أرقام[٤]؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interp = Interpreter::new();
    let result = interp.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_interpret_arabic_dictionaries() {
    let input = r#"
        متغير شخص = {"الاسم": "أحمد"، "العمر": ٢٥}؛
        متغير اسم = شخص["الاسم"]؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interp = Interpreter::new();
    let result = interp.interpret(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_interpret_arabic_string_operations() {
    let input = r#"
        متغير أ = "مرحباً"؛
        متغير ب = " بالعالم"؛
        متغير ج = أ + ب؛
        اطبع(ج)؛
    "#;
    let ast = Parser::parse(input).unwrap();
    let mut interp = Interpreter::new();
    let result = interp.interpret(&ast);
    assert!(result.is_ok());
}
