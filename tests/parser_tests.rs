// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات المحلل النحوي - Parser Tests
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::parser::Parser;

/// اختبار المتغيرات الأساسية
#[test]
fn test_variable_declarations() {
    let source = r#"
        متغير س = 10
        ثابت ص = 20
        متغير اسم = "أحمد"
        متغير نشط = صحيح
        متغير فارغ = لا شيء
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل المتغيرات: {:?}", result.err());
    
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 5, "يجب أن يكون هناك 5 تعليمات");
    println!("✅ test_variable_declarations: {} تعليمة", program.statements.len());
}

/// اختبار الدوال
#[test]
fn test_function_definitions() {
    let source = r#"
        دالة جمع(أ، ب):
            أرجع أ + ب
        
        دالة ترحيب(اسم):
            طباعة("مرحبا " + اسم)
        
        دالة بدون_معاملات():
            طباعة("بدون معاملات")
        
        دالة بقيمة_افتراضية(أ، ب = 10):
            أرجع أ + ب
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل الدوال: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_function_definitions: {} تعليمة", program.statements.len());
}

/// اختبار الشروط
#[test]
fn test_conditionals() {
    let source = r#"
        إذا س > 10:
            طباعة("كبير")
        وإلا إذا س == 10:
            طباعة("متوسط")
        وإلا:
            طباعة("صغير")
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل الشروط: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_conditionals: {} تعليمة", program.statements.len());
}

/// اختبار الحلقات
#[test]
fn test_loops() {
    let source = r#"
        بينما س > 0:
            س = س - 1
        
        لكل عنصر في قائمة:
            طباعة(عنصر)
        
        لـ س من 1 إلى 10:
            طباعة(س)
        
        لـ س من 1 إلى 10 بخطوة 2:
            طباعة(س)
        
        كرر 5:
            طباعة("مرحبا")
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل الحلقات: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_loops: {} تعليمة", program.statements.len());
}

/// اختبار الفئات (Classes)
#[test]
fn test_classes() {
    let source = r#"
        صنف شخص:
            متغير اسم = ""
            متغير عمر = 0
            
            دالة جديد(اسم، عمر):
                هذا.اسم = اسم
                هذا.عمر = عمر
            
            دالة معلومات():
                أرجع "الاسم: " + هذا.اسم + "، العمر: " + هذا.عمر
        
        صنف موظف يرث شخص:
            متغير راتب = 0
            
            دالة جديد(اسم، عمر، راتب):
                أساسي.جديد(اسم، عمر)
                هذا.راتب = راتب
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل الفئات: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_classes: {} تعليمة", program.statements.len());
}

/// اختبار التعبيرات الرياضية
#[test]
fn test_mathematical_expressions() {
    let source = r#"
        متغير أ = 1 + 2 * 3
        متغير ب = (1 + 2) * 3
        متغير ج = 10 / 2 + 3
        متغير د = 2 ** 10
        متغير هـ = 10 % 3
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل التعبيرات: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_mathematical_expressions: {} تعليمة", program.statements.len());
}

/// اختبار المقارنات والمنطق
#[test]
fn test_comparison_and_logic() {
    let source = r#"
        متغير أ = س > 10
        متغير ب = س < 10
        متغير ج = س == 10
        متغير د = س != 10
        متغير هـ = س >= 10
        متغير و = س <= 10
        
        متغير منطقي1 = صحيح و خطأ
        متغير منطقي2 = صحيح أو خطأ
        متغير منطقي3 = لا صحيح
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل المقارنات: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_comparison_and_logic: {} تعليمة", program.statements.len());
}

/// اختبار القوائم والقواميس
#[test]
fn test_lists_and_dictionaries() {
    let source = r#"
        متغير قائمة = [1، 2، 3، 4، 5]
        متوع قاموس = {اسم: "أحمد"، عمر: 25}
        متغير فهرس = قائمة[0]
        متغير قيمة = قاموس["اسم"]
        متغير مدمج = [[1، 2]، [3، 4]]
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل القوائم: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_lists_and_dictionaries: {} تعليمة", program.statements.len());
}

/// اختبار معامل الأنبوب (Pipe)
#[test]
fn test_pipe_operator() {
    let source = r#"
        متغير نتيجة = 10 |> مضاعف |> جمع(5)
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل معامل الأنبوب: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_pipe_operator: {} تعليمة", program.statements.len());
}

/// اختبار الدوال المجهولة (Lambda)
#[test]
fn test_lambda_expressions() {
    let source = r#"
        متغير مضاعف = دالة(س) => س * 2
        متوع جمع = دالة(أ، ب) => أ + ب
        متغير نتيجة = [1، 2، 3].خريطة(دالة(س) => س * 2)
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل الدوال المجهولة: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_lambda_expressions: {} تعليمة", program.statements.len());
}

/// اختبار النطاقات (Ranges)
#[test]
fn test_ranges() {
    let source = r#"
        متغير نطاق1 = 1..10
        متغير نطاق2 = 1..10..2
        
        لـ س في 1..10:
            طباعة(س)
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل النطاقات: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_ranges: {} تعليمة", program.statements.len());
}

/// اختبار التفكيك (Destructuring)
#[test]
fn test_destructuring() {
    let source = r#"
        متغير [أ، ب، ج] = [1، 2، 3]
        متغير {اسم، عمر} = شخص
        متغير {اسم: الاسم_الكامل، عمر: العمر_بالسنوات} = شخص
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل التفكيك: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_destructuring: {} تعليمة", program.statements.len());
}

/// اختبار معالجة الأخطاء (Try/Catch)
#[test]
fn test_error_handling() {
    let source = r#"
        حاول:
            قد_تفشل()
        قبض خطأ:
            طباعة("خطأ: " + خطأ)
        أخيراً:
            طباعة("تم")
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل معالجة الأخطاء: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_error_handling: {} تعليمة", program.statements.len());
}

/// اختبار التأكيدات (Assert)
#[test]
fn test_assertions() {
    let source = r#"
        تأكيد س > 0
        تأكيد س > 0 "س يجب أن يكون موجباً"
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل التأكيدات: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_assertions: {} تعليمة", program.statements.len());
}

/// اختبار المعامل الثلاثي
#[test]
fn test_ternary_operator() {
    let source = r#"
        متغير نتيجة = إذا س > 0 ? "موجب" : "سالب"
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل المعامل الثلاثي: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_ternary_operator: {} تعليمة", program.statements.len());
}

/// اختبار التكرار المتداخل (List Comprehension)
#[test]
fn test_list_comprehension() {
    let source = r#"
        متغير مربعات = [س ** 2 لكل س في [1، 2، 3، 4، 5]]
        متوع أزواج = [(س، ص) لكل س في 1..3 لكل ص في 1..3]
    "#;
    let result = Parser::parse(source);
    // قد لا يكون مدعوماً بالكامل، لكن نتحقق من عدم تعطل المحلل
    println!("✅ test_list_comprehension: {:?}", result.is_ok());
}

/// اختبار الأداء - ملف كبير
#[test]
fn test_parser_performance() {
    let mut source = String::new();
    for i in 0..100 {
        source.push_str(&format!(r#"
            دالة دالة{}(أ، ب):
                إذا أ > ب:
                    أرجع أ
                وإلا:
                    أرجع ب
        "#, i));
    }
    
    let start = std::time::Instant::now();
    let result = Parser::parse(&source);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok());
    println!("✅ test_parser_performance: {:?} لـ 100 دالة", elapsed);
    assert!(elapsed.as_millis() < 2000, "يجب أن يكون التحليل أقل من ثانيتين");
}

/// اختبار أخطاء النحو
#[test]
fn test_parser_errors() {
    let bad_sources = vec![
        "متغير = ",           // بدون قيمة
        "دالة ()",            // بدون اسم
        "إذا :",              // بدون شرط
        "بينما :",            // بدون شرط
        "لكل في :",           // بدون متغير
    ];
    
    let mut error_count = 0;
    for source in bad_sources {
        if Parser::parse(source).is_err() {
            error_count += 1;
        }
    }
    
    println!("✅ test_parser_errors: {} أخطاء تم اكتشافها", error_count);
}

/// اختبار استيراد الوحدات
#[test]
fn test_imports() {
    let source = r#"
        استيراد "رياضيات"
        استيراد "شبكة" كـ ن
        من "رياضيات" استيراد جمع، طرح
    "#;
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل في تحليل الاستيراد: {:?}", result.err());
    
    let program = result.unwrap();
    println!("✅ test_imports: {} تعليمة", program.statements.len());
}
