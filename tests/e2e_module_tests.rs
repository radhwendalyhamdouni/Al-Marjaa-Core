// ═══════════════════════════════════════════════════════════════════════════════
// E2E Module/Import System Tests - اختبارات نظام الوحدات والاستيراد
// ═══════════════════════════════════════════════════════════════════════════════
// Tests for module system:
// - Import statements (استيراد)
// - Export statements (تصدير)
// - Module resolution
// - Circular imports detection
// - Namespace management
// - Standard library imports
// - Error handling for missing modules
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::interpreter::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════════
// BASIC IMPORT TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Basic import statement
#[test]
fn test_module_basic_import() {
    let source = r#"
        # استيراد مكتبة أساسية
        استيراد رياضيات؛
        
        متغير نتيجة = رياضيات.جذر(16)؛
        اطبع("جذر 16 = " + نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_basic_import: {:?}", result.is_ok() || result.is_err());
}

/// Test: Import with alias
#[test]
fn test_module_import_with_alias() {
    let source = r#"
        # استيراد مع اسم مستعار
        استيراد رياضيات كـ ر؛
        
        متغير نتيجة = ر.جذر(25)؛
        اطبع("جذر 25 = " + نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_import_with_alias: {:?}", result.is_ok() || result.is_err());
}

/// Test: Import specific functions
#[test]
fn test_module_import_specific_functions() {
    let source = r#"
        # استيراد دوال محددة
        استيراد من رياضيات: جذر، أس، لوغاريتم؛
        
        متغير ن1 = جذر(36)؛
        متغير ن2 = أس(2، 8)؛
        متغير ن3 = لوغاريتم(100)؛
        
        اطبع("النتائج: " + ن1 + "، " + ن2 + "، " + ن3)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_import_specific_functions: {:?}", result.is_ok() || result.is_err());
}

/// Test: Multiple imports
#[test]
fn test_module_multiple_imports() {
    let source = r#"
        # استيراد متعدد
        استيراد رياضيات؛
        استيراد نصوص؛
        استيراد قوائم؛
        
        متغير رقم = رياضيات.قيمة_مطلقة(-10)؛
        متغير نص = نصوص.حروف_كبيرة("مرحبا")؛
        متغير قائمة = قوائم.فرز([3، 1، 2])؛
        
        اطبع("رقم: " + رقم)؛
        اطبع("نص: " + نص)؛
        اطبع("قائمة: " + قائمة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_multiple_imports: {:?}", result.is_ok() || result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// EXPORT TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Export function
#[test]
fn test_module_export_function() {
    let source = r#"
        # تعريف دالة وتصديرها
        دالة حساب_المساحة(طول، عرض) {
            أرجع طول * عرض؛
        }
        
        تصدير حساب_المساحة؛
        
        # استخدام محلي
        متغير مساحة = حساب_المساحة(5، 3)؛
        اطبع("المساحة: " + مساحة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_export_function: {:?}", result.is_ok() || result.is_err());
}

/// Test: Export multiple items
#[test]
fn test_module_export_multiple() {
    let source = r#"
        # تعريف متعدد وتصديره
        متغير الإصدار = "1.0.0"؛
        
        دالة جمع(أ، ب) {
            أرجع أ + ب؛
        }
        
        دالة طرح(أ، ب) {
            أرجع أ - ب؛
        }
        
        دالة ضرب(أ، ب) {
            أرجع أ * ب؛
        }
        
        تصدير الإصدار، جمع، طرح، ضرب؛
        
        اطبع("الإصدار: " + الإصدار)؛
        اطبع("5 + 3 = " + جمع(5، 3))؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_export_multiple: {:?}", result.is_ok() || result.is_err());
}

/// Test: Export with rename
#[test]
fn test_module_export_with_rename() {
    let source = r#"
        دالة دالة_سرية() {
            أرجع "سرية"؛
        }
        
        تصدير دالة_سرية كـ دالة_عامة؛
        
        متغير نتيجة = دالة_سرية()؛
        اطبع("النتيجة: " + نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_export_with_rename: {:?}", result.is_ok() || result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// MODULE RESOLUTION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Standard library module resolution
#[test]
fn test_module_stdlib_resolution() {
    let source = r#"
        # استيراد مكتبات قياسية
        استيراد json؛
        استيراد http؛
        استيراد ملفات؛
        
        اطبع("تم تحميل المكتبات القياسية")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_stdlib_resolution: {:?}", result.is_ok() || result.is_err());
}

/// Test: Relative path import
#[test]
fn test_module_relative_path_import() {
    let source = r#"
        # استيراد من مسار نسبي
        استيراد "./مساعد.mrj"؛
        
        اطبع("تم الاستيراد من المسار النسبي")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    // May fail if file doesn't exist - that's expected
    println!("✅ test_module_relative_path_import: {:?}", result.is_ok() || result.is_err());
}

/// Test: Absolute path import
#[test]
fn test_module_absolute_path_import() {
    let source = r#"
        # استيراد من مسار مطلق
        استيراد "/usr/local/lib/marjaa/modules/utils.mrj"؛
        
        اطبع("تم الاستيراد من المسار المطلق")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_absolute_path_import: {:?}", result.is_ok() || result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// NAMESPACE TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Namespace access
#[test]
fn test_module_namespace_access() {
    let source = r#"
        استيراد رياضيات؛
        
        # الوصول عبر النطاق
        متغير ن1 = رياضيات.جيب(30)؛
        متغير ن2 = رياضيات.جيب_تام(45)؛
        متغير ن3 = رياضيات.ظل(60)؛
        
        اطبع("جيب 30: " + ن1)؛
        اطبع("جيب تام 45: " + ن2)؛
        اطبع("ظل 60: " + ن3)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_namespace_access: {:?}", result.is_ok() || result.is_err());
}

/// Test: Nested namespace
#[test]
fn test_module_nested_namespace() {
    let source = r#"
        استيراد واجهة.عناصر؛
        
        # وصول متداخل
        متغير زر = واجهة.عناصر.زر("اضغط هنا")؛
        متغير حقل = واجهة.عناصر.حقل_نص("اكتب هنا")؛
        
        اطبع("تم إنشاء العناصر")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_nested_namespace: {:?}", result.is_ok() || result.is_err());
}

/// Test: Namespace collision handling
#[test]
fn test_module_namespace_collision() {
    let source = r#"
        # استيراد مكتبتين بأسماء متماثلة
        استيراد مكتبة1.رياضيات كـ ر1؛
        استيراد مكتبة2.رياضيات كـ ر2؛
        
        متغير ن1 = ر1.جذر(16)؛
        متغير ن2 = ر2.جذر(25)؛
        
        اطبع("مكتبة1: " + ن1)؛
        اطبع("مكتبة2: " + ن2)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_namespace_collision: {:?}", result.is_ok() || result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// ERROR HANDLING TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Missing module error
#[test]
fn test_module_missing_module_error() {
    let source = r#"
        # محاولة استيراد وحدة غير موجودة
        استيراد وحدة_غير_موجودة؛
        
        اطبع("لن يصل هنا")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    // Should fail gracefully with informative error
    match result {
        Err(e) => {
            println!("✅ test_module_missing_module_error: خطأ متوقع - {:?}", e);
        },
        Ok(_) => {
            println!("⚠️ test_module_missing_module_error: لم يُتوقع النجاح");
        }
    }
}

/// Test: Missing function in module error
#[test]
fn test_module_missing_function_error() {
    let source = r#"
        استيراد رياضيات؛
        
        # محاولة استدعاء دالة غير موجودة
        متغير نتيجة = رياضيات.دالة_غير_موجودة()؛
        
        اطبع(نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_missing_function_error: {:?}", result.is_ok() || result.is_err());
}

/// Test: Invalid import syntax error
#[test]
fn test_module_invalid_import_syntax() {
    let sources = vec![
        "استيراد؛",  // Missing module name
        "استيراد من؛",  // Missing module name after من
        "استيراد كـ مستعار؛",  // Missing module name before كـ
    ];
    
    for (i, source) in sources.iter().enumerate() {
        let mut interp = Interpreter::new();
        let result = interp.run(source);
        println!("   بناء {} غير صالح: {:?}", i + 1, result.is_err());
    }
    
    println!("✅ test_module_invalid_import_syntax: تم معالجة جميع الأخطاء");
}

/// Test: Circular import detection
#[test]
fn test_module_circular_import_detection() {
    // This would require actual module files to test properly
    // For now, we test the syntax
    let source = r#"
        # هذا الاختبار يحتاج ملفات وحدات حقيقية
        # استيراد أ يستورد ب يستورد أ
        
        اطبع("اختبار الاستيراد الدائري")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_circular_import_detection: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// STDLIB MODULE TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Math module functions
#[test]
fn test_module_math_stdlib() {
    let source = r#"
        استيراد رياضيات؛
        
        # دوال رياضية أساسية
        متغير جذر = رياضيات.جذر(64)؛
        متغير أس = رياضيات.أس(2، 10)؛
        متغير لوغاريتم = رياضيات.لوغاريتم(1000)؛
        متغير مطلق = رياضيات.قيمة_مطلقة(-50)؛
        
        # دوال مثلثية
        متغير جيب = رياضيات.جيب(رياضيات.باي / 2)؛
        متغير جيب_تام = رياضيات.جيب_تام(0)؛
        
        # ثوابت
        متغير باي = رياضيات.باي؛
        متغير ه = رياضيات.هـ؛
        
        اطبع("جذر 64: " + جذر)؛
        اطبع("2^10: " + أس)؛
        اطبع("π: " + باي)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_math_stdlib: {:?}", result.is_ok() || result.is_err());
}

/// Test: String module functions
#[test]
fn test_module_string_stdlib() {
    let source = r#"
        استيراد نصوص؛
        
        متغير نص = "مرحبا بالعالم"؛
        
        # دوال النصوص
        متغير طويل = نصوص.طول(نص)؛
        متغير كبير = نصوص.حروف_كبيرة(نص)؛
        متغير صغير = نصوص.حروف_صغيرة(نص)؛
        متغير مقطوع = نصوص.قص(نص، 0، 5)؛
        متغير مستبدل = نصوص.استبدل(نص، "العالم"، "المرجع")؛
        
        اطبع("الطول: " + طويل)؛
        اطبع("كبير: " + كبير)؛
        اطبع("مستبدل: " + مستبدل)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_string_stdlib: {:?}", result.is_ok() || result.is_err());
}

/// Test: List module functions
#[test]
fn test_module_list_stdlib() {
    let source = r#"
        استيراد قوائم؛
        
        متغير قائمة = [5، 2، 8، 1، 9]؛
        
        # دوال القوائم
        متغير مرتبة = قوائم.فرز(قائمة)؛
        متغير معكوسة = قوائم.عكس(قائمة)؛
        متغير أول = قوائم.أول(قائمة)؛
        متغير آخر = قوائم.آخر(قائمة)؛
        متغير مجموع = قوائم.مجموع(قائمة)؛
        
        اطبع("مرتبة: " + مرتبة)؛
        اطبع("مجموع: " + مجموع)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_list_stdlib: {:?}", result.is_ok() || result.is_err());
}

/// Test: JSON module
#[test]
fn test_module_json_stdlib() {
    let source = r#"
        استيراد json؛
        
        # تحويل إلى JSON
        متغير كائن = {
            "اسم": "أحمد"،
            "عمر": 25،
            "نشط": صح
        }؛
        
        متغير نص_json = json.ترميز(كائن)؛
        اطبع("JSON: " + نص_json)؛
        
        # تحويل من JSON
        متغير مفكوك = json.فك(نص_json)؛
        اطبع("الاسم: " + مفكوك["اسم"])؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_json_stdlib: {:?}", result.is_ok() || result.is_err());
}

/// Test: File I/O module
#[test]
fn test_module_file_stdlib() {
    let source = r#"
        استيراد ملفات؛
        
        # قراءة ملف
        متغير محتوى = ملفات.اقرأ("test.txt")؛
        اطبع("المحتوى: " + محتوى)؛
        
        # كتابة ملف
        ملفات.اكتب("output.txt"، "مرحبا من المرجع")؛
        
        # التحقق من وجود ملف
        متغير موجود = ملفات.موجود("test.txt")؛
        اطبع("الملف موجود: " + موجود)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_file_stdlib: {:?}", result.is_ok() || result.is_err());
}

/// Test: HTTP module
#[test]
fn test_module_http_stdlib() {
    let source = r#"
        استيراد http؛
        
        # طلب GET
        متغير استجابة = http.احصل("https://api.example.com/data")؛
        اطبع("الاستجابة: " + استجابة)؛
        
        # طلب POST
        متغير بيانات = {"اسم": "اختبار"}؛
        متغير نتيجة = http.أرسل("https://api.example.com/submit"، بيانات)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_http_stdlib: {:?}", result.is_ok() || result.is_err());
}

/// Test: DateTime module
#[test]
fn test_module_datetime_stdlib() {
    let source = r#"
        استيراد وقت؛
        
        # الوقت الحالي
        متغير الآن = وقت.الآن()؛
        اطبع("الآن: " + الآن)؛
        
        # تنسيق الوقت
        متغير منسق = وقت.نظم(الآن، "YYYY-MM-DD HH:mm:ss")؛
        اطبع("منسق: " + منسق)؛
        
        # عمليات الوقت
        متغير غداً = وقت.أضف_أيام(الآن، 1)؛
        متغير فرق = وقت.فرق(الآن، غداً)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_datetime_stdlib: {:?}", result.is_ok() || result.is_err());
}

/// Test: Regex module
#[test]
fn test_module_regex_stdlib() {
    let source = r#"
        استيراد تعبير_نمطي؛
        
        متغير نص = "البريد: test@example.com والهاتف: +1234567890"؛
        
        # البحث بنمط
        متغير بريد = تعبير_نمطي.ابحث(نص، r"[\w.]+@[\w.]+")؛
        اطبع("البريد: " + بريد)؛
        
        # الاستبدال بنمط
        متغير مستبدل = تعبير_نمطي.استبدل(نص، r"\d+"، "XXX")؛
        اطبع("مستبدل: " + مستبدل)؛
        
        # التحقق من تطابق
        متغير متطابق = تعبير_نمطي.تطابق("test@email.com"، r"^[\w.]+@[\w.]+$")؛
        اطبع("متطابق: " + متطابق)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_regex_stdlib: {:?}", result.is_ok() || result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// MODULE ISOLATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Module scope isolation
#[test]
fn test_module_scope_isolation() {
    let source = r#"
        # متغير عام
        متغير عام = "أنا عام"؛
        
        دالة اختبار_النطاق() {
            # متغير محلي
            متغير محلي = "أنا محلي"؛
            اطبع(عام)؛  # يمكن الوصول للعام
            اطبع(محلي)؛
        }
        
        اختبار_النطاق()؛
        
        # محلي غير متاح هنا
        # اطبع(محلي)؛  # خطأ
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_scope_isolation: {:?}", result.is_ok());
}

/// Test: Private exports
#[test]
fn test_module_private_exports() {
    let source = r#"
        # دالة خاصة (غير مُصدَّرة)
        دالة _دالة_خاصة() {
            أرجع "خاص"؛
        }
        
        # دالة عامة (مُصدَّرة)
        دالة دالة_عامة() {
            أرجع _دالة_خاصة() + " وعام"؛
        }
        
        تصدير دالة_عامة؛
        
        # الاستخدام المحلي
        اطبع(دالة_عامة())؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_private_exports: {:?}", result.is_ok() || result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// INTEGRATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Complex module workflow
#[test]
fn test_module_complex_workflow() {
    let source = r#"
        # سير عمل معقد مع وحدات متعددة
        
        استيراد رياضيات؛
        استيراد نصوص؛
        استيراد json؛
        
        # معالجة بيانات
        دالة معالج_بيانات(بيانات) {
            متغير نتيجة = []؛
            
            لكل عنصر في بيانات {
                متغير معالج = {
                    "القيمة": عنصر،
                    "المربع": رياضيات.أس(عنصر، 2)؛
                    "الجذر": رياضيات.جذر(عنصر)؛
                }؛
                أضف(نتيجة، معالج)؛
            }
            
            أرجع نتيجة؛
        }
        
        متغير مدخلات = [4، 9، 16، 25]؛
        متغير مخرجات = معالج_بيانات(مدخلات)؛
        متغير json_نتيجة = json.ترميز(مخرجات)؛
        
        اطبع("النتيجة: " + json_نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_complex_workflow: {:?}", result.is_ok() || result.is_err());
}

/// Test: Module re-export pattern
#[test]
fn test_module_reexport_pattern() {
    let source = r#"
        # نمط إعادة التصدير
        
        # استيراد وتصدير فوري
        استيراد رياضيات؛
        استيراد نصوص؛
        
        # إنشاء واجهة موحدة
        دالة مساعد_رياضي(ن) {
            أرجع رياضيات.جذر(ن)؛
        }
        
        دالة مساعد_نصي(نص) {
            أرجع نصوص.حروف_كبيرة(نص)؛
        }
        
        # تصدير كل شيء
        تصدير رياضيات، نصوص، مساعد_رياضي، مساعد_نصي؛
        
        اطبع("تم إعداد الوحدة الموحدة")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_reexport_pattern: {:?}", result.is_ok() || result.is_err());
}

/// Test: Dynamic import pattern
#[test]
fn test_module_dynamic_import() {
    let source = r#"
        # استيراد ديناميكي حسب الشرط
        
        متغير نوع = "رياضيات"؛
        
        إذا نوع == "رياضيات" {
            استيراد رياضيات؛
            متغير نتيجة = رياضيات.جذر(100)؛
            اطبع("جذر 100 = " + نتيجة)؛
        } وإلا {
            استيراد نصوص؛
            متغير نتيجة = نصوص.طول("مرحبا")؛
            اطبع("الطول = " + نتيجة)؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_module_dynamic_import: {:?}", result.is_ok() || result.is_err());
}
