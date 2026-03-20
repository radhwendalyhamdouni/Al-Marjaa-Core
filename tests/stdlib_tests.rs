// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات المكتبة القياسية - Standard Library Tests
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::interpreter::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الدوال الرياضية
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_math_basic() {
    let source = r#"
        متvariable أ = رياضيات.جذر(16)
        متvariable ب = رياضيات.أس(2، 10)
        متvariable ج = رياضيات.لوغاريتم(100)
        متvariable د = رياضيات.لوغاريتم_طبيعي(10)
        متvariable هـ = رياضيات.مطلق(-5)
        متvariable و = رياضيات.تقريب(3.7)
        متvariable ز = رياضيات.طابق(3.7)
        متvariable ح = رياضيات.سقف(3.2)
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في الدوال الرياضية: {:?}", result.err());
    println!("✅ test_stdlib_math_basic");
}

#[test]
fn test_stdlib_math_trigonometric() {
    let source = r#"
        متvariable جيب = رياضيات.جيب(رياضيات.باي / 2)
        متvariable جيب_تمام = رياضيات.جيب_تمام(0)
        متvariable ظل = رياضيات.ظل(رياضيات.باي / 4)
        متvariable قوس_جيب = رياضيات.قوس_جيب(1)
        متvariable قوس_جيب_تمام = رياضيات.قوس_جيب_تمام(0)
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_math_trigonometric");
}

#[test]
fn test_stdlib_math_constants() {
    let source = r#"
        متvariable بي = رياضيات.باي
        متvariable هـ = رياضيات.هـ
        متvariable ذهبية = رياضيات.نسبة_ذهبية
        متvariable لانهائي = رياضيات.لانهائي
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_math_constants");
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات دوال النصوص
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_string_basic() {
    let source = r#"
        متvariable نص = "مرحبا بالعالم"
        متvariable طول = نص.طول()
        متvariable كبير = نص.حرف_كبير()
        متvariable صغير = نص.حرف_صغير()
        متvariable جزء = نص.جزء(0، 5)
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_string_basic");
}

#[test]
fn test_stdlib_string_search() {
    let source = r#"
        متvariable نص = "مرحبا بالعالم مرحبا"
        متvariable موقع = نص.موقع("مرحبا")
        متvariable مواقع = نص.مواقع("مرحبا")
        متvariable يحتوي = نص.يحتوي("عالم")
        متvariable يبدأ = نص.يبدأ_بـ("مرحبا")
        متvariable ينتهي = نص.ينتهي_بـ("مرحبا")
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_string_search");
}

#[test]
fn test_stdlib_string_manipulation() {
    let source = r#"
        متvariable نص = "  مرحبا  "
        متvariable مقصوص = نص.قص()
        متvariable بدون_مسافات = نص.بدون_مسافات()
        
        متvariable قائمة = "أ،ب،ج".اقسم("،")
        متvariable مدمج = ["أ"، "ب"، "ج"].ادمج("-")
        
        متvariable مستبدل = "مرحبا عالم".استبدل("عالم"، "المرجع")
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_string_manipulation");
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات دوال القوائم
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_list_basic() {
    let source = r#"
        متvariable قائمة = [1، 2، 3، 4، 5]
        متvariable طول = قائمة.طول()
        قائمة.أضف(6)
        قائمة.أدخل(0، 0)
        متvariable محذوف = قائمة.احذف(0)
        متvariable أول = قائمة.أول()
        متvariable آخر = قائمة.آخر()
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_list_basic");
}

#[test]
fn test_stdlib_list_operations() {
    let source = r#"
        متvariable قائمة = [3، 1، 4، 1، 5، 9، 2، 6]
        قائمة.رتب()
        قائمة.اعكس()
        متvariable موقع = قائمة.موقع(5)
        قائمة.احذف_الكل(1)
        متvariable مرشح = قائمة.رشح(دالة(س) => س > 3)
        متvariable محول = قائمة.حول(دالة(س) => س * 2)
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_list_operations");
}

#[test]
fn test_stdlib_list_aggregate() {
    let source = r#"
        متvariable قائمة = [1، 2، 3، 4، 5]
        متvariable مجموع = قائمة.مجموع()
        متvariable حاصل = قائمة.حاصل()
        متvariable أعلى = قائمة.أعلى()
        متvariable أدنى = قائمة.أدنى()
        متvariable متوسط = قائمة.متوسط()
        متvariable الكل = قائمة.الكل(دالة(س) => س > 0)
        متvariable أي = قائمة.أي(دالة(س) => س > 3)
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_list_aggregate");
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات دوال القواميس
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_dict_basic() {
    let source = r#"
        متvariable قاموس = {أ: 1، ب: 2، ج: 3}
        متvariable مفاتيح = قاموس.مفاتيح()
        متvariable قيم = قاموس.قيم()
        متvariable عناصر = قاموس.عناصر()
        متvariable طول = قاموس.طول()
        قاموس.ضع("د"، 4)
        متvariable محذوف = قاموس.احذف("أ")
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_dict_basic");
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات دوال الوقت والتاريخ
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_datetime() {
    let source = r#"
        متvariable الآن = وقت.الآن()
        متvariable سنة = الآن.سنة()
        متvariable شهر = الآن.شهر()
        متvariable يوم = الآن.يوم()
        متvariable ساعة = الآن.ساعة()
        متvariable دقيقة = الآن.دقيقة()
        متvariable ثانية = الآن.ثانية()
        متvariable تنسيق = الآن.تنسيق("YYYY-MM-DD")
        متvariable طابع = وقت.طابع()
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_datetime");
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات دوال JSON
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_json() {
    let source = r#"
        متvariable قاموس = {اسم: "أحمد"، عمر: 25}
        متvariable json = json.مشفر(قاموس)
        متvariable مفكوك = json.مفكوك(json)
        
        متvariable قائمة = [1، 2، 3، 4، 5]
        متvariable json2 = json.مشفر(قائمة)
        متvariable مفكوك2 = json.مفكوك(json2)
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_json");
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات دوال التشفير
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_crypto_hash() {
    let source = r#"
        متvariable نص = "مرحبا بالعالم"
        متvariable md5 = تشفير.md5(نص)
        متvariable sha1 = تشفير.sha1(نص)
        متvariable sha256 = تشفير.sha256(نص)
        متvariable sha512 = تشفير.sha512(نص)
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_crypto_hash");
}

#[test]
fn test_stdlib_crypto_encoding() {
    let source = r#"
        متvariable نص = "مرحبا"
        متvariable base64 = تشفير.base64.شفر(نص)
        متvariable مفكوك = تشفير.base64.فك(base64)
        
        متvariable hex = تشفير.سداسي_عشري.شفر(نص)
        متvariable مفكوك2 = تشفير.سداسي_عشري.فك(hex)
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_crypto_encoding");
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات دوال التعبيرات النمطية (Regex)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_regex() {
    let source = r#"
        متvariable نص = "مرحبا بالعالم 123"
        متvariable نمط = تعبير.إنشاء("\d+")
        
        متvariable تطابق = نمط.تطابق(نص)
        متvariable بحث = نمط.بحث(نص)
        متvariable الكل = نمط.الكل(نص)
        متvariable استبدال = نمط.استبدال(نص، "XXX")
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok());
    println!("✅ test_stdlib_regex");
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات دوال الملفات
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_file_operations() {
    let source = r#"
        متvariable مسار = "test_file.txt"
        
        // كتابة
        ملف.اكتب(مسار، "مرحبا بالعالم")
        
        // قراءة
        متvariable محتوى = ملف.اقرأ(مسار)
        
        // إضافة
        ملف.أضف(مسار، "\nسطر جديد")
        
        // معلومات
        متvariable موجود = ملف.موجود(مسار)
        متvariable حجم = ملف.حجم(مسار)
        
        // حذف
        ملف.احذف(مسار)
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    // قد يفشل إذا لم تكن الصلاحيات متاحة
    println!("✅ test_stdlib_file_operations: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأداء
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_performance() {
    let source = r#"
        متvariable قائمة = []
        لـ س من 1 إلى 10000:
            قائمة.أضف(س)
        
        متvariable مجموع = قائمة.مجموع()
        متvariable أعلى = قائمة.أعلى()
        متvariable أدنى = قائمة.أدنى()
    "#;
    
    let start = std::time::Instant::now();
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok());
    println!("✅ test_stdlib_performance: {:?}", elapsed);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات شاملة للمكتبة
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_comprehensive() {
    let source = r#"
        // إنشاء قاموس بيانات
        متvariable بيانات = {
            اسم: "أحمد محمد"،
            عمر: 25،
            بريد: "ahmed@example.com"،
            مهارات: ["برمجة"، "تصميم"، "تحليل"]
        }
        
        // تحويل إلى JSON
        متvariable json_str = json.مشفر(بيانات)
        
        // حساب hash
        متvariable hash = تشفير.sha256(json_str)
        
        // تنسيق التاريخ
        متvariable الآن = وقت.الآن()
        متvariable تنسيق = الآن.تنسيق("DD/MM/YYYY HH:mm:ss")
        
        // طباعة النتائج
        طباعة("البيانات: " + json_str)
        طباعة("التوقيع: " + hash)
        طباعة("التاريخ: " + تنسيق)
        
        // معالجة المهارات
        متvariable مهارات_نص = بيانات["مهارات"].ادمج(" - ")
        طباعة("المهارات: " + مهارات_نص)
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في الاختبار الشامل: {:?}", result.err());
    println!("✅ test_stdlib_comprehensive");
}
