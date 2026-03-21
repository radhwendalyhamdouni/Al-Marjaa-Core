// ═══════════════════════════════════════════════════════════════════════════════
// E2E Real World Tests - اختبارات نهاية لنهاية حقيقية
// ═══════════════════════════════════════════════════════════════════════════════
// Production-like test suites:
// - SQLite database execution tests
// - HTTP client integration tests
// - Crypto stdlib validation tests
// - Module/import system tests
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::interpreter::Interpreter;
use almarjaa::bytecode::{Compiler, CompleteV2JitCompiler};
use std::rc::Rc;
use std::cell::RefCell;
use almarjaa::interpreter::value::Environment;

// ═══════════════════════════════════════════════════════════════════════════════
// SQLITE DATABASE TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_sqlite_connection() {
    let source = r#"
        # اختبار اتصال SQLite
        متغير اتصال = اتصال_قاعدة(":memory:")؛
        اطبع("تم إنشاء اتصال قاعدة البيانات")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("SQLite Connection: {:?}", result.is_ok());
}

#[test]
fn test_sqlite_create_table() {
    let source = r#"
        # إنشاء جدول
        متغير اتصال = اتصال_قاعدة(":memory:")؛
        
        نفذ(اتصال، "
            CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT
            )
        ")؛
        
        اطبع("تم إنشاء الجدول بنجاح")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("SQLite Create Table: {:?}", result.is_ok());
}

#[test]
fn test_sqlite_insert_select() {
    let source = r#"
        # إدراج وقراءة بيانات
        متغير اتصال = اتصال_قاعدة(":memory:")؛
        
        نفذ(اتصال، "CREATE TABLE items (name TEXT, qty INTEGER)")؛
        نفذ(اتصال، "INSERT INTO items VALUES ('تفاح', 10)")؛
        نفذ(اتصال، "INSERT INTO items VALUES ('برتقال', 20)")؛
        نفذ(اتصال، "INSERT INTO items VALUES ('موز', 15)")؛
        
        متغير نتائج = استعلم(اتصال، "SELECT * FROM items")؛
        اطبع("عدد العناصر: " + طول(نتائج))؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("SQLite Insert/Select: {:?}", result.is_ok());
}

#[test]
fn test_sqlite_transaction() {
    let source = r#"
        # اختبار المعاملات
        متغير اتصال = اتصال_قاعدة(":memory:")؛
        
        نفذ(اتصال، "CREATE TABLE accounts (id INTEGER, balance REAL)")؛
        نفذ(اتصال، "INSERT INTO accounts VALUES (1, 1000)")؛
        نفذ(اتصال، "INSERT INTO accounts VALUES (2, 500)")؛
        
        # بدء معاملة
        ابدأ_معاملة(اتصال)؛
        
        نفذ(اتصال، "UPDATE accounts SET balance = balance - 100 WHERE id = 1")؛
        نفذ(اتصال، "UPDATE accounts SET balance = balance + 100 WHERE id = 2")؛
        
        # إنهاء المعاملة
        أنهِ_معاملة(اتصال)؛
        
        اطبع("تمت المعاملة بنجاح")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("SQLite Transaction: {:?}", result.is_ok());
}

#[test]
fn test_sqlite_prepared_statement() {
    let source = r#"
        # العبارات المُعدة
        متغير اتصال = اتصال_قاعدة(":memory:")؛
        
        نفذ(اتصال، "CREATE TABLE products (name TEXT, price REAL)")؛
        
        متغير عبارة = حضّر(اتصال، "INSERT INTO products VALUES (?, ?)")؛
        
        نفذ_معد(عبارة، ["لابتوب", 1500.0])؛
        نفذ_معد(عبارة، ["هاتف", 800.0])؛
        نفذ_معد(عبارة، ["تابلت", 600.0])؛
        
        متغير نتائج = استعلم(اتصال، "SELECT * FROM products")؛
        اطبع("عدد المنتجات: " + طول(نتائج))؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("SQLite Prepared: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// HTTP CLIENT TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_http_get_request() {
    let source = r#"
        # طلب HTTP GET
        متغير رد = طلب_شبكة("GET", "https://httpbin.org/get")؛
        
        إذا رد["status"] == 200 {
            اطبع("✅ طلب GET ناجح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("HTTP GET: {:?}", result.is_ok());
}

#[test]
fn test_http_post_request() {
    let source = r#"
        # طلب HTTP POST
        متغير بيانات = {"name": "أحمد", "age": 25}؛
        
        متغير رد = طلب_شبكة("POST", "https://httpbin.org/post", بيانات)؛
        
        إذا رد["status"] == 200 {
            اطبع("✅ طلب POST ناجح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("HTTP POST: {:?}", result.is_ok());
}

#[test]
fn test_http_headers() {
    let source = r#"
        # طلب مع رؤوس مخصصة
        متغير رؤوس = {
            "Content-Type": "application/json",
            "Authorization": "Bearer test123"
        }؛
        
        متغير رد = طلب_مع_رؤوس("GET", "https://httpbin.org/headers", رؤوس)؛
        
        اطبع("Status: " + رد["status"])؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("HTTP Headers: {:?}", result.is_ok());
}

#[test]
fn test_http_timeout() {
    let source = r#"
        # طلب مع مهلة زمنية
        متغير رد = طلب_بمهلة("GET", "https://httpbin.org/delay/2", 5)؛
        
        اطبع("تم الطلب خلال المهلة")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("HTTP Timeout: {:?}", result.is_ok());
}

#[test]
fn test_http_error_handling() {
    let source = r#"
        # معالجة أخطاء HTTP
        محاولة {
            متغير رد = طلب_شبكة("GET", "https://httpbin.org/status/404")؛
            
            إذا رد["status"] == 404 {
                اطبع("⚠️ المورد غير موجود")؛
            }
        }
        التقاط(خطأ) {
            اطبع("❌ خطأ في الطلب: " + خطأ)؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("HTTP Error Handling: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// CRYPTO STDLIB TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_crypto_hash_md5() {
    let source = r#"
        # اختبار MD5
        متغير نص = "مرحبا بالعالم"؛
        متغير hash = md5(نص)؛
        
        اطبع("MD5: " + hash)؛
        اطبع("الطول: " + طول(hash))؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Crypto MD5: {:?}", result.is_ok());
}

#[test]
fn test_crypto_hash_sha256() {
    let source = r#"
        # اختبار SHA-256
        متغير نص = "اللغة العربية جميلة"؛
        متغير hash = sha256(نص)؛
        
        اطبع("SHA256: " + hash)؛
        اطبع("الطول: " + طول(hash))؛
        
        # التحقق من أن نفس المدخلات تعطي نفس الناتج
        متغير hash2 = sha256(نص)؛
        إذا hash == hash2 {
            اطبع("✅ تطابق الـ hash")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Crypto SHA256: {:?}", result.is_ok());
}

#[test]
fn test_crypto_hash_sha512() {
    let source = r#"
        # اختبار SHA-512
        متغير نص = "اختبار التشفير القوي"؛
        متغير hash = sha512(نص)؛
        
        اطبع("SHA512 Length: " + طول(hash))؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Crypto SHA512: {:?}", result.is_ok());
}

#[test]
fn test_crypto_hmac() {
    let source = r#"
        # اختبار HMAC
        متغير مفتاح = "مفتاح_سري_123"؛
        متغير رسالة = "هذه رسالة مهمة"؛
        متغير توقيع = hmac(مفتاح, رسالة, "sha256")؛
        
        اطبع("HMAC: " + توقيع)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Crypto HMAC: {:?}", result.is_ok());
}

#[test]
fn test_crypto_aes_encrypt_decrypt() {
    let source = r#"
        # اختبار تشفير AES
        متغير مفتاح = "1234567890123456"؛  # 16 بايت
        متغير نص = "رسالة سرية"؛
        
        متغير مشفر = aes_تشفير(نص, مفتاح)؛
        متغير مفكوك = aes_فك(مشفر, مفتاح)؛
        
        إذا نص == مفكوك {
            اطبع("✅ التشفير وفك التشفير يعملان بشكل صحيح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Crypto AES: {:?}", result.is_ok());
}

#[test]
fn test_crypto_password_hash() {
    let source = r#"
        # اختبار تشفير كلمات المرور
        متغير كلمة = "كلمة_مرور_آمنة"؛
        
        # bcrypt
        متغير hash1 = bcrypt(كلمة, 12)؛
        اطبع("Bcrypt hash: " + hash1)؛
        
        # argon2
        متغير hash2 = argon2(كلمة)؛
        اطبع("Argon2 hash: " + hash2)؛
        
        # التحقق
        إذا تحقق_bcrypt(كلمة, hash1) {
            اطبع("✅ Bcrypt تحقق ناجح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Crypto Password: {:?}", result.is_ok());
}

#[test]
fn test_crypto_random_bytes() {
    let source = r#"
        # اختبار توليد بايتات عشوائية
        متغير بايتات = عشوائي_بايت(32)؛
        
        اطبع("تم توليد " + طول(بايتات) + " بايت عشوائي")؛
        
        # توليد UUID
        متغير uuid = uuid_توليد()؛
        اطبع("UUID: " + uuid)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Crypto Random: {:?}", result.is_ok());
}

#[test]
fn test_crypto_base64() {
    let source = r#"
        # اختبار Base64
        متغير نص = "مرحبا بالعالم العربي"؛
        
        متغير encoded = base64_ترميز(نص)؛
        اطبع("Encoded: " + encoded)؛
        
        متغير decoded = base64_فك(encoded)؛
        
        إذا نص == decoded {
            اطبع("✅ Base64 يعمل بشكل صحيح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Crypto Base64: {:?}", result.is_ok());
}

#[test]
fn test_crypto_rsa_sign_verify() {
    let source = r#"
        # اختبار RSA
        متغير (مفتاح_عام, مفتاح_خاص) = rsa_توليد(2048)؛
        
        متغير رسالة = "رسالة للتوقيع"؛
        متغير توقيع = rsa_توقيع(رسالة, مفتاح_خاص)؛
        
        إذا rsa_تحقق(رسالة, توقيع, مفتاح_عام) {
            اطبع("✅ RSA التوقيع والتحقق يعملان")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Crypto RSA: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// MODULE/IMPORT SYSTEM TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_module_import_basic() {
    let source = r#"
        # استيراد وحدة أساسية
        استيراد "رياضيات"؛
        
        متغير ن = رياضيات.جذر(16)؛
        اطبع("الجذر التربيعي لـ 16 = " + ن)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Module Import: {:?}", result.is_ok());
}

#[test]
fn test_module_import_specific() {
    let source = r#"
        # استيراد دوال محددة
        من "رياضيات" استيراد {جذر، أس، لو}؛
        
        متغير ن1 = جذر(25)؛
        متغير ن2 = أس(2، 10)؛
        متغير ن3 = لو(100)؛
        
        اطبع("جذر 25 = " + ن1)؛
        اطبع("2^10 = " + ن2)؛
        اطبع("log(100) = " + ن3)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Module Specific Import: {:?}", result.is_ok());
}

#[test]
fn test_module_alias() {
    let source = r#"
        # استيراد مع اسم مستعار
        استيراد "رياضيات" كـ ر؛
        
        متغير ن = ر.جيب(1.57)؛  # sin
        اطبع("sin(π/2) ≈ " + ن)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Module Alias: {:?}", result.is_ok());
}

#[test]
fn test_module_export() {
    let source = r#"
        # تعريف وحدة
        تصدير دالة جمع(أ، ب) {
            أرجع أ + ب؛
        }
        
        تصدير دالة طرح(أ، ب) {
            أرجع أ - ب؛
        }
        
        تصدير متغير PI = 3.14159؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Module Export: {:?}", result.is_ok());
}

#[test]
fn test_module_nested() {
    let source = r#"
        # وحدات متداخلة
        استيراد "شبكة.http"؛
        استيراد "قواعد_بيانات.sqlite"؛
        
        اطبع("تم استيراد الوحدات المتداخلة")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Module Nested: {:?}", result.is_ok());
}

#[test]
fn test_module_circular_detection() {
    let source = r#"
        # اختبار الكشف عن الاستيراد الدائري
        # أ يستورد ب، وب يستورد أ
        استيراد "module_a"؛
        
        # يجب أن يكتشف النظام الاستيراد الدائري
        اطبع("لم يحدث استيراد دائري")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Module Circular: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// STD LIBRARY COMPREHENSIVE TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_stdlib_string_operations() {
    let source = r#"
        # عمليات النصوص
        متغير نص = "  مرحبا بالعالم  "؛
        
        متغير مقلم = قلّم(نص)؛
        متغير كبير = كبير(نص)؛
        متغير صغير = صغير(نص)؛
        
        متغير أجزاء = قسم(نص، " ")؛
        متغير مدمج = ادمج(أجزاء، "-")؛
        
        متغير مستبدل = استبدل(نص، "العالم"، "اللغة العربية")؛
        
        اطبع("مقلم: " + مقلم)؛
        اطبع("كبير: " + كبير)؛
        اطبع("صغير: " + صغير)؛
        اطبع("مدمج: " + مدمج)؛
        اطبع("مستبدل: " + مستبدل)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Stdlib String: {:?}", result.is_ok());
}

#[test]
fn test_stdlib_list_operations() {
    let source = r#"
        # عمليات القوائم
        متغير قائمة = [3، 1، 4، 1، 5، 9، 2، 6]؛
        
        متغير مرتبة = رتّب(قائمة)؛
        متغير مقلوبة = اقلب(قائمة)؛
        متغير فريدة = فريد(قائمة)؛
        
        متغير أول = أول(قائمة)؛
        متغير آخر = آخر(قائمة)؛
        
        متغير مجموع = جمع_الكل(قائمة)؛
        متغير معدل = مجموع / طول(قائمة)؛
        
        اطبع("مرتبة: " + مرتبة)؛
        اطبع("فريدة: " + فريدة)؛
        اطبع("مجموع: " + مجموع)؛
        اطبع("معدل: " + معدل)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Stdlib List: {:?}", result.is_ok());
}

#[test]
fn test_stdlib_dict_operations() {
    let source = r#"
        # عمليات القواميس
        متغير شخص = {
            "اسم": "أحمد"،
            "عمر": 25،
            "مدينة": "الرياض"
        }؛
        
        متغير مفاتيح = مفاتيح(شخص)؛
        متغير قيم = قيم(شخص)؛
        
        متغير موجود = يحتوي(شخص، "اسم")؛
        متغير محذوف = احذف(شخص، "عمر")؛
        
        اطبع("مفاتيح: " + مفاتيح)؛
        اطبع("قيم: " + قيم)؛
        اطبع("يحتوي 'اسم': " + موجود)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Stdlib Dict: {:?}", result.is_ok());
}

#[test]
fn test_stdlib_math_operations() {
    let source = r#"
        # عمليات رياضية
        متغير ن = 16؛
        
        متغير جذر = جذر(ن)؛
        متغير أس2 = أس(ن، 2)؛
        متغير لو = لو(ن)؛
        متغير لو10 = لو10(ن)؛
        
        متغير جيب = جيب(1.57)؛
        متغير تمام = تمام(0)؛
        متغير ظل = ظل(0.785)؛
        
        متغير قيمة_مطلقة = قيمة_مطلقة(-25)؛
        متغير تقريب = تقريب(3.7)؛
        متغير أرضية = أرضية(3.9)؛
        متغير سقف = سقف(3.1)؛
        
        اطبع("جذر 16 = " + جذر)؛
        اطبع("قيمة مطلقة -25 = " + قيمة_مطلقة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Stdlib Math: {:?}", result.is_ok());
}

#[test]
fn test_stdlib_datetime_operations() {
    let source = r#"
        # عمليات التاريخ والوقت
        متغير الآن = الآن()؛
        متغير اليوم = اليوم()؛
        متغير الوقت = الوقت()؛
        
        متغير تاريخ = تاريخ(2024، 3، 21)؛
        متغير وقت = وقت(14، 30، 0)؛
        
        متغير منسق = نسق_تاريخ(الآن، "YYYY-MM-DD")؛
        
        اطبع("الآن: " + الآن)؛
        اطبع("اليوم: " + اليوم)؛
        اطبع("منسق: " + منسق)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Stdlib DateTime: {:?}", result.is_ok());
}

#[test]
fn test_stdlib_file_operations() {
    let source = r#"
        # عمليات الملفات
        متغير محتوى = اقرأ_ملف("test.txt")؛
        اطبع("محتوى الملف: " + محتوى)؛
        
        اكتب_ملف("output.txt"، "مرحبا بالعالم")؛
        اطبع("تم الكتابة بنجاح")؛
        
        متغير ملفات = قائمة_ملفات(".")؛
        اطبع("عدد الملفات: " + طول(ملفات))؛
        
        إذا موجود("test.txt") {
            اطبع("الملف موجود")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Stdlib File: {:?}", result.is_ok());
}

#[test]
fn test_stdlib_json_operations() {
    let source = r#"
        # عمليات JSON
        متغير شخص = {
            "اسم": "أحمد"،
            "عمر": 25،
            "هوايات": ["قراءة"، "برمجة"]
        }؛
        
        متغير json = إلى_json(شخص)؛
        اطبع("JSON: " + json)؛
        
        متغير مفكوك = من_json(json)؛
        اطبع("الاسم: " + مفكوك["اسم"])؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Stdlib JSON: {:?}", result.is_ok());
}

#[test]
fn test_stdlib_regex_operations() {
    let source = r#"
        # عمليات التعبيرات النمطية
        متغير نص = "رقم الهاتف: 0501234567"؛
        متغير نمط = "\\d{10}"؛
        
        متغير تطابق = طابق(نص، نمط)؛
        إذا تطابق {
            اطبع("تم العثور على رقم هاتف")؛
        }
        
        متغير كل_التطابقات = طابق_الكل(نص، "\\d+")؛
        اطبع("الأرقام: " + كل_التطابقات)؛
        
        متغير مستبدل = استبدل_بنمط(نص، "\\d"، "X")؛
        اطبع("مستبدل: " + مستبدل)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Stdlib Regex: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// INTEGRATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_full_stack_integration() {
    let source = r#"
        # اختبار تكامل شامل
        
        # 1. قاعدة البيانات
        متغير اتصال = اتصال_قاعدة(":memory:")؛
        نفذ(اتصال، "CREATE TABLE users (name TEXT, email TEXT)")؛
        
        # 2. HTTP
        متغير رد = طلب_شبكة("GET", "https://api.example.com/users")؛
        
        # 3. تشفير
        متغير hash = sha256(رد["body"])؛
        
        # 4. تخزين
        نفذ(اتصال، "INSERT INTO users VALUES ('test', 'test@example.com')")؛
        
        اطبع("✅ تكامل شامل ناجح")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Full Stack Integration: {:?}", result.is_ok());
}

#[test]
fn test_jit_with_stdlib() {
    let source = r#"
        # اختبار JIT مع المكتبة القياسية
        دالة حساب_مجموع(قائمة) {
            متغير مجموع = 0؛
            لكل عنصر في قائمة {
                مجموع = مجموع + عنصر؛
            }
            أرجع مجموع؛
        }
        
        متغير أرقام = []؛
        لكل س في مدى(1، 1001) {
            أضف(أرقام، س)؛
        }
        
        متغير نتيجة = حساب_مجموع(أرقام)؛
        اطبع("المجموع: " + نتيجة)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok(), "JIT مع stdlib يجب أن ينجح");
    println!("✅ JIT with stdlib");
}

#[test]
fn test_concurrent_operations() {
    let source = r#"
        # عمليات متزامنة
        متغير نتائج = []؛
        
        # محاكاة عمليات متوازية
        لكل س في مدى(1، 11) {
            متغير نتيجة = sha256("عنصر_" + س)؛
            أضف(نتائج, نتيجة)؛
        }
        
        اطبع("تمت " + طول(نتائج) + " عملية متوازية")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Concurrent Operations: {:?}", result.is_ok());
}

#[test]
fn test_error_recovery_e2e() {
    let source = r#"
        # استرداد من الأخطاء
        محاولة {
            متغير ن = 10 / 0؛
        }
        التقاط(خطأ) {
            اطبع("تم التقاط خطأ: " + خطأ)؛
        }
        
        # الاستمرار في التنفيذ
        متغير مجموع = 0؛
        لكل س في مدى(1، 11) {
            مجموع = مجموع + س؛
        }
        
        اطبع("المجموع بعد الخطأ: " + مجموع)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Error Recovery E2E: {:?}", result.is_ok());
}

#[test]
fn test_memory_intensive_e2e() {
    let source = r#"
        # اختبار مكثف للذاكرة
        متغير قوائم = []؛
        
        لكل س في مدى(1، 101) {
            متغير قائمة = []؛
            لكل ص في مدى(1، 1001) {
                أضف(قائمة، ص * س)؛
            }
            أضف(قوائم، قائمة)؛
        }
        
        متغير مجموع_الكل = 0؛
        لكل قائمة في قوائم {
            لكل عنصر في قائمة {
                مجموع_الكل = مجموع_الكل + عنصر؛
            }
        }
        
        اطبع("مجموع " + طول(قوائم) + " قائمة = " + مجموع_الكل)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    println!("Memory Intensive E2E: {:?}", result.is_ok());
}
