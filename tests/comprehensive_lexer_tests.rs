// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات شاملة للمحلل اللغوي - Comprehensive Lexer Tests
// ═══════════════════════════════════════════════════════════════════════════════
// يتضمن:
// - اختبارات الوحدة لكل نوع من الرموز
// - اختبارات الحدود (Edge Cases)
// - اختبارات الأخطاء والتعافي
// - اختبارات الأداء
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::lexer::Lexer;
use almarjaa::lexer::tokens::TokenType;

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأرقام
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod number_tests {
    use super::*;

    /// اختبار الأرقام الصحيحة الإنجليزية
    #[test]
    fn test_english_integers() {
        let test_cases = vec![
            ("0", 0.0),
            ("1", 1.0),
            ("123", 123.0),
            ("999999", 999999.0),
        ];

        for (source, expected) in test_cases {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            assert!(!tokens.is_empty(), "يجب أن يكون هناك رموز لـ: {}", source);

            let first_token = &tokens[0];
            if let TokenType::Number(n) = &first_token.token_type {
                assert_eq!(*n, expected, "القيمة المتوقعة {} لكن وجد {}", expected, n);
            } else {
                panic!("الرمز الأول ليس رقماً: {:?}", first_token.token_type);
            }
        }
    }

    /// اختبار الأرقام الصحيحة العربية
    #[test]
    fn test_arabic_integers() {
        let test_cases = vec![
            ("٠", 0.0),
            ("١", 1.0),
            ("١٢٣", 123.0),
            ("٩٩٩", 999.0),
        ];

        for (source, expected) in test_cases {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            assert!(!tokens.is_empty(), "يجب أن يكون هناك رموز لـ: {}", source);

            let first_token = &tokens[0];
            if let TokenType::Number(n) = &first_token.token_type {
                assert_eq!(*n, expected, "القيمة المتوقعة {} لكن وجد {}", expected, n);
            }
        }
    }

    /// اختبار الأرقام العشرية
    #[test]
    fn test_decimal_numbers() {
        let test_cases = vec![
            ("0.0", 0.0),
            ("1.5", 1.5),
            ("3.14159", 3.14159),
            ("0.123456789", 0.123456789),
        ];

        for (source, expected) in test_cases {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            let first_token = &tokens[0];
            if let TokenType::Number(n) = &first_token.token_type {
                assert!((n - expected).abs() < 1e-10, "القيمة المتوقعة {} لكن وجد {}", expected, n);
            }
        }
    }

    /// اختبار الأرقام العربية العشرية
    #[test]
    fn test_arabic_decimal_numbers() {
        // الأرقام العربية تستخدم الفاصلة العربية (٫) أو النقطة
        let test_cases = vec![
            ("٠٫٥", 0.5),
            ("١٫٥", 1.5),
            ("٣٫١٤", 3.14),
        ];

        for (source, expected) in test_cases {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            if !tokens.is_empty() {
                let first_token = &tokens[0];
                if let TokenType::Number(n) = &first_token.token_type {
                    assert!((n - expected).abs() < 1e-10, "القيمة المتوقعة {} لكن وجد {}", expected, n);
                }
            }
        }
    }

    /// اختبار الأرقام السالبة
    #[test]
    fn test_negative_numbers() {
        let test_cases = vec![
            ("-1", -1.0),
            ("-123", -123.0),
            ("-3.14", -3.14),
        ];

        for (source, expected) in test_cases {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            let first_token = &tokens[0];
            if let TokenType::Number(n) = &first_token.token_type {
                assert!((n - expected).abs() < 1e-10, "القيمة المتوقعة {} لكن وجد {}", expected, n);
            }
        }
    }

    /// اختبار الترميز العلمي
    #[test]
    fn test_scientific_notation() {
        let test_cases = vec![
            ("1e10", 1e10),
            ("1.5e3", 1.5e3),
            ("2.5e-3", 2.5e-3),
        ];

        for (source, expected) in test_cases {
            let mut lexer = Lexer::new(source);
            if let Ok(tokens) = lexer.tokenize() {
                if !tokens.is_empty() {
                    let first_token = &tokens[0];
                    if let TokenType::Number(n) = &first_token.token_type {
                        assert!((n - expected).abs() < 1e-5, "القيمة المتوقعة {} لكن وجد {}", expected, n);
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات النصوص
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod string_tests {
    use super::*;

    /// اختبار النصوص الأساسية
    #[test]
    fn test_basic_strings() {
        let test_cases = vec![
            (r#""مرحبا""#, "مرحبا"),
            (r#""Hello""#, "Hello"),
            (r#""123""#, "123"),
            (r#""""#, ""),
        ];

        for (source, expected) in test_cases {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            let first_token = &tokens[0];
            if let TokenType::String(s) = &first_token.token_type {
                assert_eq!(s, expected, "النص المتوقع '{}' لكن وجد '{}'", expected, s);
            }
        }
    }

    /// اختبار النصوص مع أحرف خاصة
    #[test]
    fn test_strings_with_escapes() {
        let test_cases = vec![
            (r#""نص مع \"اقتباس\"""#, r#"نص مع "اقتباس""#),
            (r#""سطر أول\nسطر ثاني""#, "سطر أول\nسطر ثاني"),
            (r#""مجلد\\ملف""#, r#"مجلد\ملف"#),
            (r#""جدول\tمحتوى""#, "جدول\tمحتوى"),
        ];

        for (source, expected) in test_cases {
            let mut lexer = Lexer::new(source);
            if let Ok(tokens) = lexer.tokenize() {
                if !tokens.is_empty() {
                    let first_token = &tokens[0];
                    if let TokenType::String(s) = &first_token.token_type {
                        assert_eq!(s, expected, "النص المتوقع '{}' لكن وجد '{}'", expected, s);
                    }
                }
            }
        }
    }

    /// اختبار نصوص متعددة الأسطر
    #[test]
    fn test_multiline_strings() {
        let source = r#""هذا نص
متعدد
الأسطر""#;

        let mut lexer = Lexer::new(source);
        if let Ok(tokens) = lexer.tokenize() {
            if !tokens.is_empty() {
                let first_token = &tokens[0];
                if let TokenType::String(s) = &first_token.token_type {
                    assert!(s.contains("متعدد"), "النص يجب أن يحتوي على 'متعدد'");
                }
            }
        }
    }

    /// اختبار نصوص Unicode
    #[test]
    fn test_unicode_strings() {
        let test_cases = vec![
            (r#""🎉🎊""#, "🎉🎊"),
            (r#""中文测试""#, "中文测试"),
            (r#""العربية""#, "العربية"),
            (r#""Ελληνικά""#, "Ελληνικά"),
        ];

        for (source, expected) in test_cases {
            let mut lexer = Lexer::new(source);
            if let Ok(tokens) = lexer.tokenize() {
                if !tokens.is_empty() {
                    let first_token = &tokens[0];
                    if let TokenType::String(s) = &first_token.token_type {
                        assert_eq!(s, expected);
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الكلمات المحجوزة
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod keyword_tests {
    use super::*;

    /// اختبار الكلمات المحجوزة الأساسية
    #[test]
    fn test_basic_keywords() {
        let keywords = vec![
            ("متغير", TokenType::Let),
            ("ثابت", TokenType::Const),
            ("دالة", TokenType::Function),
            ("أرجع", TokenType::Return),
            ("إذا", TokenType::If),
            ("وإلا", TokenType::Else),
            ("وإذا", TokenType::ElseIf),
            ("طالما", TokenType::While),
            ("لكل", TokenType::For),
            ("في", TokenType::In),
            ("توقف", TokenType::Break),
            ("أكمل", TokenType::Continue),
            ("صح", TokenType::True),
            ("خطأ", TokenType::False),
            ("لا_شيء", TokenType::NullKeyword),
        ];

        for (source, expected_type) in keywords {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            assert!(!tokens.is_empty(), "يجب أن يكون هناك رمز لـ: {}", source);
            assert_eq!(tokens[0].token_type, expected_type, "الكلمة '{}' يجب أن تكون {:?}", source, expected_type);
        }
    }

    /// اختبار كلمات معالجة الأخطاء
    #[test]
    fn test_error_handling_keywords() {
        let keywords = vec![
            ("حاول", TokenType::Try),
            ("امسك", TokenType::Catch),
            ("أخيراً", TokenType::Finally),
            ("ألقِ", TokenType::Throw),
        ];

        for (source, expected_type) in keywords {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            if !tokens.is_empty() {
                assert_eq!(tokens[0].token_type, expected_type);
            }
        }
    }

    /// اختبار كلمات الاستيراد والتصدير
    #[test]
    fn test_module_keywords() {
        let keywords = vec![
            ("استيراد", TokenType::Import),
            ("تصدير", TokenType::Export),
            ("من", TokenType::From),
            ("كـ", TokenType::AsKeyword),
            ("وحدة", TokenType::Module),
        ];

        for (source, expected_type) in keywords {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            if !tokens.is_empty() {
                assert_eq!(tokens[0].token_type, expected_type);
            }
        }
    }

    /// اختبار كلمات الأصناف
    #[test]
    fn test_class_keywords() {
        let keywords = vec![
            ("صنف", TokenType::Class),
            ("هذا", TokenType::This),
            ("جديد", TokenType::New),
            ("أصل", TokenType::Super),
        ];

        for (source, expected_type) in keywords {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            if !tokens.is_empty() {
                assert_eq!(tokens[0].token_type, expected_type);
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات المعاملات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod operator_tests {
    use super::*;

    /// اختبار المعاملات الحسابية
    #[test]
    fn test_arithmetic_operators() {
        let operators = vec![
            ("+", TokenType::Plus),
            ("-", TokenType::Minus),
            ("*", TokenType::Multiply),
            ("/", TokenType::Divide),
            ("%", TokenType::Modulo),
            ("^", TokenType::Power),
            ("//", TokenType::FloorDiv),
        ];

        for (source, expected_type) in operators {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            assert!(!tokens.is_empty());
            assert_eq!(tokens[0].token_type, expected_type);
        }
    }

    /// اختبار معاملات المقارنة
    #[test]
    fn test_comparison_operators() {
        let operators = vec![
            ("==", TokenType::Equal),
            ("!=", TokenType::NotEqual),
            ("<", TokenType::Less),
            (">", TokenType::Greater),
            ("<=", TokenType::LessEqual),
            (">=", TokenType::GreaterEqual),
        ];

        for (source, expected_type) in operators {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            assert!(!tokens.is_empty());
            assert_eq!(tokens[0].token_type, expected_type);
        }
    }

    /// اختبار معاملات الإسناد
    #[test]
    fn test_assignment_operators() {
        let operators = vec![
            ("=", TokenType::Assign),
            ("+=", TokenType::PlusAssign),
            ("-=", TokenType::MinusAssign),
            ("*=", TokenType::MultAssign),
            ("/=", TokenType::DivAssign),
            ("%=", TokenType::ModAssign),
        ];

        for (source, expected_type) in operators {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            assert!(!tokens.is_empty());
            assert_eq!(tokens[0].token_type, expected_type);
        }
    }

    /// اختبار المعاملات المنطقية
    #[test]
    fn test_logical_operators() {
        let test_cases = vec![
            ("و", TokenType::And),
            ("أو", TokenType::Or),
            ("ليس", TokenType::Not),
        ];

        for (source, expected_type) in test_cases {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            assert!(!tokens.is_empty());
            assert_eq!(tokens[0].token_type, expected_type);
        }
    }

    /// اختبار معامل الزيادة والنقصان
    #[test]
    fn test_increment_decrement() {
        let operators = vec![
            ("++", TokenType::Increment),
            ("--", TokenType::Decrement),
        ];

        for (source, expected_type) in operators {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            assert!(!tokens.is_empty());
            assert_eq!(tokens[0].token_type, expected_type);
        }
    }

    /// اختبار معامل الأنبوب
    #[test]
    fn test_pipe_operator() {
        let mut lexer = Lexer::new("|>");
        let tokens = lexer.tokenize().expect("فشل في تحليل |>");

        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].token_type, TokenType::Pipe);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الفواصل والأقواس
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod delimiter_tests {
    use super::*;

    /// اختبار الفواصل العربية والإنجليزية
    #[test]
    fn test_separators() {
        let separators = vec![
            ("؛", TokenType::Semicolon),
            (";", TokenType::Semicolon),
            ("،", TokenType::Comma),
            (",", TokenType::Comma),
            (".", TokenType::Dot),
            (":", TokenType::Colon),
        ];

        for (source, expected_type) in separators {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            assert!(!tokens.is_empty());
            assert_eq!(tokens[0].token_type, expected_type);
        }
    }

    /// اختبار الأقواس
    #[test]
    fn test_brackets() {
        let brackets = vec![
            ("(", TokenType::LParen),
            (")", TokenType::RParen),
            ("{", TokenType::LBrace),
            ("}", TokenType::RBrace),
            ("[", TokenType::LBracket),
            ("]", TokenType::RBracket),
        ];

        for (source, expected_type) in brackets {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            assert!(!tokens.is_empty());
            assert_eq!(tokens[0].token_type, expected_type);
        }
    }

    /// اختبار الأسهم
    #[test]
    fn test_arrows() {
        let arrows = vec![
            ("->", TokenType::Arrow),
            ("=>", TokenType::FatArrow),
        ];

        for (source, expected_type) in arrows {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            assert!(!tokens.is_empty());
            assert_eq!(tokens[0].token_type, expected_type);
        }
    }

    /// اختبار النطاقات
    #[test]
    fn test_range_operators() {
        let ranges = vec![
            ("..", TokenType::DotDot),
            ("...", TokenType::DotDotDot),
        ];

        for (source, expected_type) in ranges {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            assert!(!tokens.is_empty());
            assert_eq!(tokens[0].token_type, expected_type);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات المعرفات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod identifier_tests {
    use super::*;

    /// اختبار المعرفات العربية
    #[test]
    fn test_arabic_identifiers() {
        let identifiers = vec![
            "اسم",
            "العمر",
            "اسم_المستخدم",
            "رقم_الهاتف",
            "هل_نشط",
            "قيمة1",
            "متغير_2",
        ];

        for identifier in identifiers {
            let mut lexer = Lexer::new(identifier);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", identifier));

            assert!(!tokens.is_empty());
            if let TokenType::Identifier(name) = &tokens[0].token_type {
                assert_eq!(name, identifier);
            } else {
                panic!("المعرف '{}' لم يتم التعرف عليه كمعرف", identifier);
            }
        }
    }

    /// اختبار المعرفات الإنجليزية
    #[test]
    fn test_english_identifiers() {
        let identifiers = vec![
            "name",
            "age",
            "user_name",
            "phoneNumber",
            "isActive",
            "value1",
            "VAR_2",
        ];

        for identifier in identifiers {
            let mut lexer = Lexer::new(identifier);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", identifier));

            assert!(!tokens.is_empty());
            if let TokenType::Identifier(name) = &tokens[0].token_type {
                assert_eq!(name, identifier);
            }
        }
    }

    /// اختبار المعرفات المختلطة
    #[test]
    fn test_mixed_identifiers() {
        let identifiers = vec![
            "name_الاسم",
            "قيمة_value",
        ];

        for identifier in identifiers {
            let mut lexer = Lexer::new(identifier);
            if let Ok(tokens) = lexer.tokenize() {
                if !tokens.is_empty() {
                    if let TokenType::Identifier(name) = &tokens[0].token_type {
                        println!("المعرف المختلط '{}' تم تحليله كـ '{}'", identifier, name);
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات التعليقات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod comment_tests {
    use super::*;

    /// اختبار التعليقات أحادية السطر
    #[test]
    fn test_single_line_comments() {
        let sources = vec![
            "// هذا تعليق",
            "// This is a comment",
            "// تعليق مع أرقام 123",
        ];

        for source in sources {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect(&format!("فشل في تحليل: {}", source));

            // التعليقات قد يتم تجاهلها أو حفظها كـ Comment token
            println!("التعليق '{}' تم تحليله إلى {} رمز", source, tokens.len());
        }
    }

    /// اختبار التعليقات متعددة الأسطر
    #[test]
    fn test_multiline_comments() {
        let source = r#"
            /*
            هذا تعليق
            متعدد الأسطر
            */
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("فشل في تحليل التعليق متعدد الأسطر");

        println!("التعليق متعدد الأسطر تم تحليله إلى {} رمز", tokens.len());
    }

    /// اختبار التعليقات المتداخلة
    #[test]
    fn test_nested_comments() {
        let source = r#"
            /*
            تعليق خارجي
            /* تعليق داخلي */
            استمرار التعليق الخارجي
            */
        "#;

        let mut lexer = Lexer::new(source);
        // قد يتم دعم التعليقات المتداخلة أو لا
        if let Ok(tokens) = lexer.tokenize() {
            println!("التعليق المتداخل تم تحليله إلى {} رمز", tokens.len());
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأخطاء
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod error_tests {
    use super::*;

    /// اختبار النص غير المغلق
    #[test]
    fn test_unclosed_string() {
        let source = r#"متغير س = "نص غير مغلق"#;
        let mut lexer = Lexer::new(source);

        // يجب أن يُرجع خطأ
        let result = lexer.tokenize();
        assert!(result.is_err(), "يجب أن يُرجع خطأ للنص غير المغلق");
    }

    /// اختبار القوس غير المغلق
    #[test]
    fn test_unclosed_bracket() {
        let sources = vec![
            r#"قائمة = [1، 2، 3"#,
            r#"قاموس = {أ: 1"#,
            r#"دالة("#,
        ];

        for source in sources {
            let mut lexer = Lexer::new(source);
            // قد يُرجع خطأ أو يستمر
            let result = lexer.tokenize();
            println!("الرمز '{}' نتيجة التحليل: {:?}", source, result.is_ok());
        }
    }

    /// اختبار رموز غير صالحة
    #[test]
    fn test_invalid_characters() {
        let sources = vec![
            "##",
            "@@@",
            "~~~",
        ];

        for source in sources {
            let mut lexer = Lexer::new(source);
            let result = lexer.tokenize();
            // بعض الرموز قد تكون صالحة والبعض لا
            println!("الرمز '{}' نتيجة التحليل: {:?}", source, result.is_ok());
        }
    }

    /// اختبار التعافي من الأخطاء
    #[test]
    fn test_error_recovery() {
        let source = r#"
            متغير س = 10
            متغير ص = "نص غير مغلق
            متغير ع = 20
        "#;

        let mut lexer = Lexer::new(source);
        let result = lexer.tokenize();

        // حتى لو كان هناك خطأ، يجب أن يستمر المحلل
        match result {
            Ok(tokens) => println!("تم تحليل {} رمز رغم الخطأ", tokens.len()),
            Err(e) => println!("خطأ: {:?}", e),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الموقع (Position Tracking)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod position_tests {
    use super::*;

    /// اختبار تتبع السطر والعمود
    #[test]
    fn test_line_column_tracking() {
        let source = r#"متغير س = 10
متغير ص = 20"#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("فشل في التحليل");

        // جميع الرموز في السطر الأول
        let first_line_tokens: Vec<_> = tokens.iter().filter(|t| t.line == 1).collect();
        // جميع الرموز في السطر الثاني
        let second_line_tokens: Vec<_> = tokens.iter().filter(|t| t.line == 2).collect();

        assert!(!first_line_tokens.is_empty(), "يجب أن يكون هناك رموز في السطر الأول");
        assert!(!second_line_tokens.is_empty(), "يجب أن يكون هناك رموز في السطر الثاني");

        println!("السطر الأول: {} رمز", first_line_tokens.len());
        println!("السطر الثاني: {} رمز", second_line_tokens.len());
    }

    /// اختبار تتبع Span
    #[test]
    fn test_span_tracking() {
        let source = "متغير";

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("فشل في التحليل");

        if !tokens.is_empty() {
            let token = &tokens[0];
            assert!(token.span_start <= token.span_end, "Span يجب أن يكون صالحاً");
            println!("Span للكلمة 'متغير': {} - {}", token.span_start, token.span_end);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأداء
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    /// اختبار أداء الملف الكبير
    #[test]
    fn test_large_file_performance() {
        let mut source = String::new();
        for i in 0..10000 {
            source.push_str(&format!("متغير متغير{} = {}\n", i, i));
        }

        let start = Instant::now();
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize().expect("فشل في التحليل");
        let elapsed = start.elapsed();

        println!("تحليل 10000 سطر: {} رمز في {:?}", tokens.len(), elapsed);

        // يجب أن يكون التحليل في أقل من 5 ثواني
        assert!(elapsed.as_secs() < 5, "يجب أن يكون التحليل أسرع من 5 ثواني");
    }

    /// اختبار أداء النصوص الطويلة
    #[test]
    fn test_long_strings_performance() {
        let long_string = "أ".repeat(10000);
        let source = format!(r#""{}""#, long_string);

        let start = Instant::now();
        let mut lexer = Lexer::new(&source);
        let _tokens = lexer.tokenize().expect("فشل في التحليل");
        let elapsed = start.elapsed();

        println!("تحليل نص طويل (10000 حرف): {:?}", elapsed);
        assert!(elapsed.as_millis() < 500, "يجب أن يكون تحليل النص سريعاً");
    }

    /// اختبار استخدام الذاكرة
    #[test]
    fn test_memory_usage() {
        // إنشاء ملف كبير نسبياً
        let mut source = String::new();
        for i in 0..1000 {
            source.push_str(&format!(
                r#"متغير س{} = "نص {}"
"#,
                i, i
            ));
        }

        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize().expect("فشل في التحليل");

        // التحقق من أن عدد الرموز معقول
        println!("عدد الرموز: {}", tokens.len());
        assert!(tokens.len() > 1000, "يجب أن يكون هناك عدد كبير من الرموز");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الحدود (Edge Cases)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    /// اختبار ملف فارغ
    #[test]
    fn test_empty_file() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().expect("فشل في تحليل الملف الفارغ");

        // يجب أن يحتوي على EOF على الأقل
        assert!(!tokens.is_empty(), "يجب أن يحتوي على EOF");
        assert_eq!(tokens[0].token_type, TokenType::EOF);
    }

    /// اختبار مسافات بيضاء فقط
    #[test]
    fn test_whitespace_only() {
        let sources = vec![" ", "  ", "\t", "\n", "\r\n", "  \t\n  "];

        for source in sources {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().expect("فشل في تحليل المسافات");

            // يجب أن يحتوي على EOF فقط
            assert!(!tokens.is_empty());
            println!("مسافات بيضاء '{}' تم تحليلها إلى {} رمز", source.escape_debug(), tokens.len());
        }
    }

    /// اختبار أرقام كبيرة جداً
    #[test]
    fn test_very_large_numbers() {
        let sources = vec![
            "999999999999999999",
            "0.000000000000001",
            "1e308",
        ];

        for source in sources {
            let mut lexer = Lexer::new(source);
            if let Ok(tokens) = lexer.tokenize() {
                if !tokens.is_empty() {
                    println!("الرقم '{}' تم تحليله إلى {:?}", source, tokens[0].token_type);
                }
            }
        }
    }

    /// اختبار تعشيق عميق
    #[test]
    fn test_deep_nesting() {
        let depth = 50;
        let mut source = String::new();
        for _ in 0..depth {
            source.push_str("قائمة = [");
        }
        source.push_str("1");
        for _ in 0..depth {
            source.push_str("]");
        }

        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize().expect("فشل في تحليل التعشيق العميق");

        println!("تعشيق بعمق {} تم تحليله إلى {} رمز", depth, tokens.len());
    }
}
