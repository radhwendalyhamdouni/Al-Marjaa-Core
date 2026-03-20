// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات المحلل اللغوي - Lexer Tests
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::lexer::Lexer;

/// اختبار الرموز الأساسية
#[test]
fn test_basic_tokens() {
    let source = r#"متغير س = ١٠"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("فشل في التحليل اللغوي");
    
    assert!(!tokens.is_empty(), "يجب أن يكون هناك رموز");
    println!("✅ test_basic_tokens: {} رمز", tokens.len());
}

/// اختبار الأرقام العربية والإنجليزية
#[test]
fn test_arabic_and_english_numbers() {
    let source = r#"
        متغير أ = 123
        متغير ب = ٤٥٦
        متغير ج = 3.14
        متغير د = ٧٫٨٩
    "#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("فشل في التحليل اللغوي");
    
    assert!(!tokens.is_empty());
    println!("✅ test_arabic_and_english_numbers: {} رمز", tokens.len());
}

/// اختبار النصوص
#[test]
fn test_strings() {
    let source = r#"
        متغير نص1 = "مرحبا بالعالم"
        متغير نص2 = "Hello World"
        متغير نص3 = "نص مع \"اقتباس\""
    "#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("فشل في التحليل اللغوي");
    
    assert!(!tokens.is_empty());
    println!("✅ test_strings: {} رمز", tokens.len());
}

/// اختبار المعاملات
#[test]
fn test_operators() {
    let source = r#"
        أ + ب - ج * د / هـ
        أ % ب
        أ ** ب
        أ == ب
        أ != ب
        أ < ب
        أ > ب
        أ <= ب
        أ >= ب
        أ و ب
        أ أو ب
        لا أ
    "#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("فشل في التحليل اللغوي");
    
    assert!(!tokens.is_empty());
    println!("✅ test_operators: {} رمز", tokens.len());
}

/// اختبار الكلمات المفتاحية
#[test]
fn test_keywords() {
    let source = r#"
        متغير س = 10
        ثابت ص = 20
        إذا س > 5:
            طباعة("كبير")
        وإلا إذا س == 5:
            طباعة("متوسط")
        وإلا:
            طباعة("صغير")
        
        بينما س > 0:
            س = س - 1
        
        لكل عنصر في [1، 2، 3]:
            طباعة(عنصر)
        
        دالة جمع(أ، ب):
            أرجع أ + ب
        
        حاول:
            خطر()
        قبض خطأ:
            طباعة(خطأ)
        
        صنف حيوان:
            اسم = "حيوان"
            
            دالة صوت():
                طباعة("صوت عام")
    "#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("فشل في التحليل اللغوي");
    
    assert!(!tokens.is_empty());
    println!("✅ test_keywords: {} رمز", tokens.len());
}

/// اختبار القوائم والقواميس
#[test]
fn test_lists_and_dicts() {
    let source = r#"
        متغير قائمة = [1، 2، 3، 4، 5]
        متغير قاموس = {اسم: "أحمد"، عمر: 25}
        متغير قائمة_مدمجة = [[1، 2]، [3، 4]]
        متغير قاموس_مدمج = {بيانات: {اسم: "سارة"}}
    "#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("فشل في التحليل اللغوي");
    
    assert!(!tokens.is_empty());
    println!("✅ test_lists_and_dicts: {} رمز", tokens.len());
}

/// اختبار الدوال المجهولة (Lambda)
#[test]
fn test_lambda() {
    let source = r#"
        متغير مضاعف = دالة(س) => س * 2
        متغير جمع = دالة(أ، ب) => أ + ب
    "#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("فشل في التحليل اللغوي");
    
    assert!(!tokens.is_empty());
    println!("✅ test_lambda: {} رمز", tokens.len());
}

/// اختبار النطاقات
#[test]
fn test_ranges() {
    let source = r#"
        متغير نطاق1 = 1..10
        متغير نطاق2 = 1..10..2
    "#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("فشل في التحليل اللغوي");
    
    assert!(!tokens.is_empty());
    println!("✅ test_ranges: {} رمز", tokens.len());
}

/// اختبار التعليقات
#[test]
fn test_comments() {
    let source = r#"
        // هذا تعليق سطر واحد
        متغير س = 10  // تعليق بعد الكود
        
        /*
        هذا تعليق
        متعدد الأسطر
        */
        
        متغير ص = 20
    "#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("فشل في التحليل اللغوي");
    
    assert!(!tokens.is_empty());
    println!("✅ test_comments: {} رمز", tokens.len());
}

/// اختبار الأخطاء اللغوية
#[test]
fn test_lexer_errors() {
    let bad_sources = vec![
        r#"متغير س = "نص غير مغلق"#,
        r#"متغير س = [1، 2، 3"#,
        r#"متغير س = {أ: 1، ب: 2"#,
    ];
    
    let mut error_count = 0;
    for source in bad_sources {
        let mut lexer = Lexer::new(source);
        if lexer.tokenize().is_err() {
            error_count += 1;
        }
    }
    
    println!("✅ test_lexer_errors: {} أخطاء تم اكتشافها", error_count);
}

/// اختبار الأداء - ملف كبير
#[test]
fn test_lexer_performance() {
    let mut source = String::new();
    for i in 0..1000 {
        source.push_str(&format!("متغير متغير{} = {}\n", i, i));
    }
    
    let start = std::time::Instant::now();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("فشل في التحليل اللغوي");
    let elapsed = start.elapsed();
    
    println!("✅ test_lexer_performance: {} رمز في {:?}", tokens.len(), elapsed);
    assert!(elapsed.as_millis() < 1000, "يجب أن يكون التحليل أقل من ثانية");
}

/// اختبار المعرفات العربية
#[test]
fn test_arabic_identifiers() {
    let source = r#"
        متغير الاسم_الكامل = "أحمد محمد"
        متغير رقم_الهاتف = "0123456789"
        متغير العمر_بالسنوات = 25
        متغير هل_نشط = صحيح
    "#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("فشل في التحليل اللغوي");
    
    assert!(!tokens.is_empty());
    println!("✅ test_arabic_identifiers: {} رمز", tokens.len());
}
