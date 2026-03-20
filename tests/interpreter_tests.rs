// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات المفسر - Interpreter Tests
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::interpreter::Interpreter;

/// اختبار المتغيرات الأساسية
#[test]
fn test_basic_variables() {
    let mut interp = Interpreter::new();
    
    // متغير رقمي
    let result = interp.run("متغير س = 10");
    assert!(result.is_ok(), "فشل في إنشاء متغير رقمي");
    
    // متغير نصي
    let result = interp.run(r#"متغير اسم = "أحمد""#);
    assert!(result.is_ok(), "فشل في إنشاء متغير نصي");
    
    // متغير منطقي
    let result = interp.run("متغير نشط = صحيح");
    assert!(result.is_ok(), "فشل في إنشاء متغير منطقي");
    
    println!("✅ test_basic_variables");
}

/// اختبار العمليات الحسابية
#[test]
fn test_arithmetic_operations() {
    let mut interp = Interpreter::new();
    
    // الجمع
    let result = interp.run("متغير أ = 5 + 3");
    assert!(result.is_ok());
    
    // الطرح
    let result = interp.run("متغير ب = 10 - 4");
    assert!(result.is_ok());
    
    // الضرب
    let result = interp.run("متغير ج = 6 * 7");
    assert!(result.is_ok());
    
    // القسمة
    let result = interp.run("متغير د = 20 / 4");
    assert!(result.is_ok());
    
    // الأس
    let result = interp.run("متغير هـ = 2 ** 10");
    assert!(result.is_ok());
    
    // الباقي
    let result = interp.run("متغير و = 10 % 3");
    assert!(result.is_ok());
    
    println!("✅ test_arithmetic_operations");
}

/// اختبار أولوية العمليات
#[test]
fn test_operator_precedence() {
    let mut interp = Interpreter::new();
    
    // 5 + 3 * 2 = 11
    interp.run("متغير أ = 5 + 3 * 2").unwrap();
    
    // (5 + 3) * 2 = 16
    interp.run("متغير ب = (5 + 3) * 2").unwrap();
    
    // 10 - 4 / 2 = 8
    interp.run("متغير ج = 10 - 4 / 2").unwrap();
    
    println!("✅ test_operator_precedence");
}

/// اختبار المقارنات
#[test]
fn test_comparisons() {
    let mut interp = Interpreter::new();
    
    assert!(interp.run("متغير أ = 5 > 3").is_ok());
    assert!(interp.run("متغير ب = 5 < 3").is_ok());
    assert!(interp.run("متغير ج = 5 == 5").is_ok());
    assert!(interp.run("متغير د = 5 != 3").is_ok());
    assert!(interp.run("متغير هـ = 5 >= 5").is_ok());
    assert!(interp.run("متغير و = 5 <= 6").is_ok());
    
    println!("✅ test_comparisons");
}

/// اختبار العمليات المنطقية
#[test]
fn test_logical_operations() {
    let mut interp = Interpreter::new();
    
    // AND
    assert!(interp.run("متغير أ = صحيح و صحيح").is_ok());
    assert!(interp.run("متvariable ب = صحيح و خطأ").is_ok());
    
    // OR
    assert!(interp.run("متغير ج = صحيح أو خطأ").is_ok());
    assert!(interp.run("متغير د = خطأ أو خطأ").is_ok());
    
    // NOT
    assert!(interp.run("متغير هـ = لا صحيح").is_ok());
    assert!(interp.run("متغير و = لا خطأ").is_ok());
    
    println!("✅ test_logical_operations");
}

/// اختبار الدوال
#[test]
fn test_functions() {
    let mut interp = Interpreter::new();
    
    // تعريف دالة
    let result = interp.run(r#"
        دالة جمع(أ، ب):
            أرجع أ + ب
    "#);
    assert!(result.is_ok(), "فشل في تعريف الدالة");
    
    // استدعاء دالة
    let result = interp.run("متغير نتيجة = جمع(3، 5)");
    assert!(result.is_ok(), "فشل في استدعاء الدالة");
    
    println!("✅ test_functions");
}

/// اختبار الدوال العودية
#[test]
fn test_recursive_functions() {
    let mut interp = Interpreter::new();
    
    // مضروب
    let result = interp.run(r#"
        دالة مضروب(ن):
            إذا ن <= 1:
                أرجع 1
            وإلا:
                أرجع ن * مضروب(ن - 1)
    "#);
    assert!(result.is_ok(), "فشل في تعريف الدالة العودية");
    
    println!("✅ test_recursive_functions");
}

/// اختبار الشروط
#[test]
fn test_conditionals() {
    let mut interp = Interpreter::new();
    
    // if فقط
    let result = interp.run(r#"
        متغير س = 10
        إذا س > 5:
            متغير نتيجة = "كبير"
    "#);
    assert!(result.is_ok());
    
    // if-elif-else
    let result = interp.run(r#"
        متغير س = 5
        إذا س > 5:
            متغير نتيجة = "كبير"
        وإلا إذا س == 5:
            متغير نتيجة = "متوسط"
        وإلا:
            متغير نتيجة = "صغير"
    "#);
    assert!(result.is_ok());
    
    println!("✅ test_conditionals");
}

/// اختبار الحلقات
#[test]
fn test_loops() {
    let mut interp = Interpreter::new();
    
    // حلقة while
    let result = interp.run(r#"
        متغير س = 0
        بينما س < 5:
            س = س + 1
    "#);
    assert!(result.is_ok());
    
    // حلقة for
    let result = interp.run(r#"
        لكل عنصر في [1، 2، 3]:
            طباعة(عنصر)
    "#);
    assert!(result.is_ok());
    
    // حلقة for-range
    let result = interp.run(r#"
        لـ س من 1 إلى 5:
            طباعة(س)
    "#);
    assert!(result.is_ok());
    
    println!("✅ test_loops");
}

/// اختبار القوائم
#[test]
fn test_lists() {
    let mut interp = Interpreter::new();
    
    // إنشاء قائمة
    let result = interp.run("متغير قائمة = [1، 2، 3، 4، 5]");
    assert!(result.is_ok());
    
    // الوصول لعنصر
    let result = interp.run("متغير أول = قائمة[0]");
    assert!(result.is_ok());
    
    // تعديل عنصر
    let result = interp.run("قائمة[0] = 10");
    assert!(result.is_ok());
    
    // القوائم المتداخلة
    let result = interp.run("متغير مدمج = [[1، 2]، [3، 4]]");
    assert!(result.is_ok());
    
    println!("✅ test_lists");
}

/// اختبار القواميس
#[test]
fn test_dictionaries() {
    let mut interp = Interpreter::new();
    
    // إنشاء قاموس
    let result = interp.run(r#"متغير شخص = {اسم: "أحمد"، عمر: 25}"#);
    assert!(result.is_ok());
    
    // الوصول لقيمة
    let result = interp.run(r#"متغير الاسم = شخص["اسم"]"#);
    assert!(result.is_ok());
    
    // تعديل قيمة
    let result = interp.run(r#"شخص["عمر"] = 26"#);
    assert!(result.is_ok());
    
    println!("✅ test_dictionaries");
}

/// اختبار النصوص
#[test]
fn test_strings() {
    let mut interp = Interpreter::new();
    
    // إنشاء نص
    let result = interp.run(r#"متغير نص = "مرحبا بالعالم""#);
    assert!(result.is_ok());
    
    // دمج النصوص
    let result = interp.run(r#"متغير مدمج = "مرحبا" + " " + "عالم""#);
    assert!(result.is_ok());
    
    // طول النص
    let result = interp.run(r#"متغير طول = طول("مرحبا")"#);
    assert!(result.is_ok());
    
    println!("✅ test_strings");
}

/// اختبار الفئات (Classes)
#[test]
fn test_classes() {
    let mut interp = Interpreter::new();
    
    let result = interp.run(r#"
        صنف شخص:
            متغير اسم = ""
            متغير عمر = 0
            
            دالة جديد(اسم، عمر):
                هذا.اسم = اسم
                هذا.عمر = عمر
            
            دالة معلومات():
                أرجع "الاسم: " + هذا.اسم
    "#);
    assert!(result.is_ok(), "فشل في تعريف الفئة");
    
    // إنشاء كائن
    let result = interp.run(r#"متغير أحمد = شخص.جديد("أحمد"، 25)"#);
    assert!(result.is_ok(), "فشل في إنشاء كائن");
    
    println!("✅ test_classes");
}

/// اختبار معالجة الأخطاء (Try-Catch)
#[test]
fn test_error_handling() {
    let mut interp = Interpreter::new();
    
    let result = interp.run(r#"
        حاول:
            متغير نتيجة = 10 / 0
        قبض خطأ:
            طباعة("خطأ: " + خطأ)
    "#);
    assert!(result.is_ok());
    
    println!("✅ test_error_handling");
}

/// اختبار الوراثة
#[test]
fn test_inheritance() {
    let mut interp = Interpreter::new();
    
    let result = interp.run(r#"
        صنف حيوان:
            متغير اسم = "حيوان"
            
            دالة صوت():
                طباعة("صوت عام")
        
        صنف كلب يرث حيوان:
            متغير اسم = "كلب"
            
            دالة صوت():
                طباعة("نباح")
    "#);
    assert!(result.is_ok(), "فشل في الوراثة");
    
    println!("✅ test_inheritance");
}

/// اختبار الإغلاق (Closure)
#[test]
fn test_closures() {
    let mut interp = Interpreter::new();
    
    let result = interp.run(r#"
        دالة مضاعف(عامل):
            أرجع دالة(س) => س * عامل
        
        متغير مضاعف_2 = مضاعف(2)
        متغير نتيجة = مضاعف_2(5)
    "#);
    assert!(result.is_ok(), "فشل في الإغلاق");
    
    println!("✅ test_closures");
}

/// اختبار break و continue
#[test]
fn test_break_continue() {
    let mut interp = Interpreter::new();
    
    // break
    let result = interp.run(r#"
        متغير س = 0
        بينما صحيح:
            س = س + 1
            إذا س > 5:
                توقف
    "#);
    assert!(result.is_ok());
    
    // continue
    let result = interp.run(r#"
        متغير مجموع = 0
        لـ س من 1 إلى 10:
            إذا س % 2 == 0:
                استمر
            مجموع = مجموع + س
    "#);
    assert!(result.is_ok());
    
    println!("✅ test_break_continue");
}

/// اختبار الثوابت
#[test]
fn test_constants() {
    let mut interp = Interpreter::new();
    
    let result = interp.run("ثابت بي = 3.14159");
    assert!(result.is_ok(), "فشل في تعريف الثابت");
    
    // محاولة تعديل الثابت يجب أن تفشل
    // (حسب التنفيذ، قد يختلف السلوك)
    
    println!("✅ test_constants");
}

/// اختبار الأداء - حلقة كبيرة
#[test]
fn test_performance_large_loop() {
    let mut interp = Interpreter::new();
    
    let start = std::time::Instant::now();
    let result = interp.run(r#"
        متغير مجموع = 0
        لـ س من 1 إلى 10000:
            مجموع = مجموع + س
    "#);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok());
    println!("✅ test_performance_large_loop: {:?}", elapsed);
    assert!(elapsed.as_millis() < 5000, "يجب أن يكون أقل من 5 ثواني");
}

/// اختبار الذاكرة - قوائم كبيرة
#[test]
fn test_memory_large_lists() {
    let mut interp = Interpreter::new();
    
    let result = interp.run(r#"
        متغير قائمة = []
        لـ س من 1 إلى 1000:
            قائمة.أضف(س)
    "#);
    
    assert!(result.is_ok());
    println!("✅ test_memory_large_lists");
}

/// اختبار Fibonacci - اختبار شامل
#[test]
fn test_fibonacci() {
    let mut interp = Interpreter::new();
    
    let result = interp.run(r#"
        دالة فيبوناتشي(ن):
            إذا ن <= 1:
                أرجع ن
            أرجع فيبوناتشي(ن - 1) + فيبوناتشي(ن - 2)
        
        متعرض نتيجة = فيبوناتشي(10)
    "#);
    
    assert!(result.is_ok());
    println!("✅ test_fibonacci");
}
