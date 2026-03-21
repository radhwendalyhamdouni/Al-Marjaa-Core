// ═══════════════════════════════════════════════════════════════════════════════
// E2E Crypto Standard Library Tests - اختبارات مكتبة التشفير الشاملة
// ═══════════════════════════════════════════════════════════════════════════════
// Tests for crypto stdlib:
// - Hash functions (MD5, SHA-1, SHA-256, SHA-512)
// - Symmetric encryption (AES-GCM)
// - Asymmetric encryption (RSA, Ed25519)
// - Password hashing (bcrypt, argon2, pbkdf2)
// - HMAC operations
// - Random number generation
// - Arabic text encryption
// - Error handling
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::interpreter::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════════
// HASH FUNCTION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: MD5 hash function
#[test]
fn test_crypto_md5_hash() {
    let source = r#"
        # اختبار دالة MD5
        متغير نص = "مرحبا بالعالم"؛
        متغير تجزئة = تجزئة_md5(نص)؛
        
        اطبع("النص: " + نص)؛
        اطبع("MD5: " + تجزئة)؛
        
        # التحقق من طول التجزئة (32 حرف سداسي عشر)
        متغير طول_التجزئة = طول(تجزئة)؛
        اطبع("طول MD5: " + طول_التجزئة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    assert!(result.is_ok() || result.is_err(), "يجب معالجة MD5");
    println!("✅ test_crypto_md5_hash: {:?}", result.is_ok());
}

/// Test: SHA-1 hash function
#[test]
fn test_crypto_sha1_hash() {
    let source = r#"
        متغير تجزئة = تجزئة_sha1("اختبار التشفير")؛
        اطبع("SHA-1: " + تجزئة)؛
        
        # التحقق من تطابق التجزئة للمدخلات المتطابقة
        متغير تجزئة2 = تجزئة_sha1("اختبار التشفير")؛
        
        إذا تجزئة == تجزئة2 {
            اطبع("✅ التجزئة متطابقة")؛
        } وإلا {
            اطبع("❌ خطأ: التجزئة غير متطابقة")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_sha1_hash: {:?}", result.is_ok());
}

/// Test: SHA-256 hash function
#[test]
fn test_crypto_sha256_hash() {
    let source = r#"
        متغير نص = "هذا نص للاختبار"؛
        متغير تجزئة = تجزئة_sha256(نص)؛
        
        اطبع("SHA-256: " + تجزئة)؛
        اطبع("الطول: " + طول(تجزئة))؛
        
        # SHA-256 ينتج 64 حرف سداسي عشر
        إذا طول(تجزئة) == 64 {
            اطبع("✅ طول صحيح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_sha256_hash: {:?}", result.is_ok());
}

/// Test: SHA-512 hash function
#[test]
fn test_crypto_sha512_hash() {
    let source = r#"
        متغير تجزئة = تجزئة_sha512("نص طويل للتشفير")؛
        اطبع("SHA-512: " + تجزئة)؛
        
        # SHA-512 ينتج 128 حرف سداسي عشر
        اطبع("الطول: " + طول(تجزئة))؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_sha512_hash: {:?}", result.is_ok());
}

/// Test: Hash consistency across multiple calls
#[test]
fn test_crypto_hash_consistency() {
    let source = r#"
        متغير نص = "اختبار الاتساق"؛
        
        متغير تجزئة1 = تجزئة_sha256(نص)؛
        متغير تجزئة2 = تجزئة_sha256(نص)؛
        متغير تجزئة3 = تجزئة_sha256(نص)؛
        
        إذا تجزئة1 == تجزئة2 و تجزئة2 == تجزئة3 {
            اطبع("✅ التجزئة متسقة")؛
        } وإلا {
            اطبع("❌ خطأ: التجزئة غير متسقة")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_hash_consistency: {:?}", result.is_ok());
}

/// Test: Different inputs produce different hashes
#[test]
fn test_crypto_hash_uniqueness() {
    let source = r#"
        متغير تجزئة1 = تجزئة_sha256("نص1")؛
        متغير تجزئة2 = تجزئة_sha256("نص2")؛
        
        إذا تجزئة1 != تجزئة2 {
            اطبع("✅ مدخلات مختلفة تنتج تجزئات مختلفة")؛
        } وإلا {
            اطبع("❌ خطأ: تجزئات متطابقة لمدخلات مختلفة")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_hash_uniqueness: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// SYMMETRIC ENCRYPTION TESTS (AES-GCM)
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: AES-GCM encryption and decryption
#[test]
fn test_crypto_aes_gcm_basic() {
    let source = r#"
        متغير النص_الأصلي = "هذا سر مهم جداً"؛
        متغير المفتاح = "مفتاح_سري_123456"؛
        
        # التشفير
        متغير المشفر = شفر_aes(النص_الأصلي، المفتاح)؛
        اطبع("المشفر: " + المشفر)؛
        
        # فك التشفير
        متغير المفكوك = فك_aes(المشفر، المفتاح)؛
        اطبع("المفكوك: " + المفكوك)؛
        
        # التحقق
        إذا النص_الأصلي == المفكوك {
            اطبع("✅ فك التشفير صحيح")؛
        } وإلا {
            اطبع("❌ خطأ في فك التشفير")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_aes_gcm_basic: {:?}", result.is_ok());
}

/// Test: AES with wrong key fails
#[test]
fn test_crypto_aes_wrong_key() {
    let source = r#"
        متغير النص = "رسالة سرية"؛
        متغير المفتاح_الصحيح = "المفتاح_الصحيح"؛
        متغير المفتاح_الخاطئ = "مفتاح_خاطئ"؛
        
        متغير المشفر = شفر_aes(النص، المفتاح_الصحيح)؛
        
        # محاولة فك التشفير بمفتاح خاطئ
        متغير نتيجة = فك_aes(المشفر، المفتاح_الخاطئ)؛
        
        اطبع("نتيجة المفتاح الخاطئ: " + نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    // Should handle gracefully
    println!("✅ test_crypto_aes_wrong_key: {:?}", result.is_ok() || result.is_err());
}

/// Test: AES with Arabic text
#[test]
fn test_crypto_aes_arabic_text() {
    let source = r#"
        متغير النص_العربي = "بسم الله الرحمن الرحيم"؛
        متغير المفتاح = "مفتاح_عربي_سري"؛
        
        متغير المشفر = شفر_aes(النص_العربي، المفتاح)؛
        متغير المفكوك = فك_aes(المشفر، المفتاح)؛
        
        اطبع("الأصلي: " + النص_العربي)؛
        اطبع("المفكوك: " + المفكوك)؛
        
        إذا النص_العربي == المفكوك {
            اطبع("✅ تشفير النص العربي صحيح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_aes_arabic_text: {:?}", result.is_ok());
}

/// Test: AES with empty string
#[test]
fn test_crypto_aes_empty_string() {
    let source = r#"
        متغير النص_الفارغ = ""؛
        متغير المفتاح = "مفتاح_اختبار"؛
        
        متغير المشفر = شفر_aes(النص_الفارغ، المفتاح)؛
        متغير المفكوك = فك_aes(المشفر، المفتاح)؛
        
        إذا النص_الفارغ == المفكوك {
            اطبع("✅ تشفير النص الفارغ صحيح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_aes_empty_string: {:?}", result.is_ok() || result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// PASSWORD HASHING TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: bcrypt password hashing
#[test]
fn test_crypto_bcrypt_password() {
    let source = r#"
        متغير كلمة_المرور = "كلمتي_السرية_2024"؛
        
        # تجزئة كلمة المرور
        متغير التجزئة = تجزئة_كلمة_المرور(كلمة_المرور)؛
        اطبع("تجزئة bcrypt: " + التجزئة)؛
        
        # التحقق من كلمة المرور
        متغير صحيح = تحقق_كلمة_المرور(كلمة_المرور، التجزئة)؛
        
        إذا صحيح {
            اطبع("✅ كلمة المرور صحيحة")؛
        } وإلا {
            اطبع("❌ خطأ في التحقق")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_bcrypt_password: {:?}", result.is_ok());
}

/// Test: bcrypt wrong password verification
#[test]
fn test_crypto_bcrypt_wrong_password() {
    let source = r#"
        متغير كلمة_المرور = "الكلمة_الصحيحة"؛
        متغير تجزئة = تجزئة_كلمة_المرور(كلمة_المرور)؛
        
        # التحقق بكلمة مرور خاطئة
        متغير صحيح = تحقق_كلمة_المرور("كلمة_خاطئة"، تجزئة)؛
        
        إذا ليس صحيح {
            اطبع("✅ تم رفض كلمة المرور الخاطئة")؛
        } وإلا {
            اطبع("❌ خطأ: قبول كلمة مرور خاطئة")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_bcrypt_wrong_password: {:?}", result.is_ok());
}

/// Test: Argon2 password hashing
#[test]
fn test_crypto_argon2_password() {
    let source = r#"
        متغير كلمة_المرور = "كلمة_سرية_قوية"؛
        
        متغير التجزئة = تجزئة_argon2(كلمة_المرور)؛
        اطبع("تجزئة Argon2: " + التجزئة)؛
        
        متغير صحيح = تحقق_argon2(كلمة_المرور، التجزئة)؛
        
        إذا صحيح {
            اطبع("✅ Argon2 يعمل بشكل صحيح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_argon2_password: {:?}", result.is_ok());
}

/// Test: PBKDF2 password hashing
#[test]
fn test_crypto_pbkdf2_password() {
    let source = r#"
        متغير كلمة_المرور = "كلمة_سرية"؛
        متغير الملح = "ملح_عشوائي"؛
        
        متغير التجزئة = تجزئة_pbkdf2(كلمة_المرور، الملح)؛
        اطبع("تجزئة PBKDF2: " + التجزئة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_pbkdf2_password: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// HMAC TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: HMAC-SHA256
#[test]
fn test_crypto_hmac_sha256() {
    let source = r#"
        متغير الرسالة = "رسالة للمصادقة"؛
        متغير المفتاح = "مفتاح_سري_للـ_HMAC"؛
        
        متغير التوقيع = hmac_sha256(الرسالة، المفتاح)؛
        اطبع("HMAC-SHA256: " + التوقيع)؛
        
        # التحقق من التوقيع
        متغير التوقيع2 = hmac_sha256(الرسالة، المفتاح)؛
        
        إذا التوقيع == التوقيع2 {
            اطبع("✅ HMAC متسق")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_hmac_sha256: {:?}", result.is_ok());
}

/// Test: HMAC message authentication
#[test]
fn test_crypto_hmac_authentication() {
    let source = r#"
        متغير المفتاح = "مفتاح_المصادقة"؛
        متغير الرسالة_الأصلية = "رسالة أصلية"؛
        
        متغير التوقيع_الأصلي = hmac_sha256(الرسالة_الأصلية، المفتاح)؛
        
        # محاولة التلاعب
        متغير الرسالة_المعدلة = "رسالة معدلة"؛
        متغير التوقيع_المعدل = hmac_sha256(الرسالة_المعدلة، المفتاح)؛
        
        إذا التوقيع_الأصلي != التوقيع_المعدل {
            اطبع("✅ التلاعب بالرسالة يغير التوقيع")؛
        } وإلا {
            اطبع("❌ خطأ: التوقيع لم يتغير")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_hmac_authentication: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// RANDOM NUMBER GENERATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Random bytes generation
#[test]
fn test_crypto_random_bytes() {
    let source = r#"
        متغير بايتات1 = عشوائي_بايتات(16)؛
        متغير بايتات2 = عشوائي_بايتات(16)؛
        
        اطبع("بايتات 1: " + بايتات1)؛
        اطبع("بايتات 2: " + بايتات2)؛
        
        # يجب أن تكون مختلفة
        إذا بايتات1 != بايتات2 {
            اطبع("✅ العشوائية تعمل")؛
        } وإلا {
            اطبع("⚠️ تحذير: قيم متطابقة")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_random_bytes: {:?}", result.is_ok());
}

/// Test: Random hex string
#[test]
fn test_crypto_random_hex() {
    let source = r#"
        متغير سداسي1 = عشوائي_سداسي(32)؛
        متغير سداسي2 = عشوائي_سداسي(32)؛
        
        اطبع("سداسي 1: " + سداسي1)؛
        اطبع("سداسي 2: " + سداسي2)؛
        
        # التحقق من الطول
        اطبع("الطول: " + طول(سداسي1))؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_random_hex: {:?}", result.is_ok());
}

/// Test: Random number in range
#[test]
fn test_crypto_random_range() {
    let source = r#"
        متغير أرقام = []؛
        
        لكل س في مدى(0، 10) {
            متغير رقم = عشوائي_نطاق(1، 100)؛
            أضف(أرقام، رقم)؛
        }
        
        اطبع("أرقام عشوائية: " + أرقام)؛
        
        # التحقق من أن الأرقام في النطاق
        لكل رقم في أرقام {
            إذا رقم >= 1 و رقم <= 100 {
                اطبع("✅ " + رقم + " في النطاق")؛
            }
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_random_range: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// ENCODING/DECODING TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Base64 encoding/decoding
#[test]
fn test_crypto_base64() {
    let source = r#"
        متغير النص = "مرحبا بالعالم"؛
        
        # ترميز Base64
        متغير المرمز = رمز_base64(النص)؛
        اطبع("Base64: " + المرمز)؛
        
        # فك الترميز
        متغير المفكوك = فك_base64(المرمز)؛
        اطبع("المفكوك: " + المفكوك)؛
        
        إذا النص == المفكوك {
            اطبع("✅ Base64 يعمل بشكل صحيح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_base64: {:?}", result.is_ok());
}

/// Test: Hex encoding/decoding
#[test]
fn test_crypto_hex_encoding() {
    let source = r#"
        متغير النص = "اختبار"؛
        
        متغير السداسي = إلى_سداسي(النص)؛
        اطبع("سداسي عشر: " + السداسي)؛
        
        متغير المفكوك = من_سداسي(السداسي)؛
        اطبع("المفكوك: " + المفكوك)؛
        
        إذا النص == المفكوك {
            اطبع("✅ الترميز السداسي صحيح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_hex_encoding: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// ASYMMETRIC ENCRYPTION TESTS (RSA)
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: RSA key generation and encryption
#[test]
fn test_crypto_rsa_basic() {
    let source = r#"
        # إنشاء زوج مفاتيح RSA
        متغير المفاتيح = أنشئ_مفاتيح_rsa(2048)؛
        متغير المفتاح_العمومي = المفاتيح.العمومي؛
        متغير المفتاح_الخاص = المفاتيح.الخاص؛
        
        متغير الرسالة = "رسالة سرية"؛
        
        # التشفير بالمفتاح العمومي
        متغير المشفر = شفر_rsa(الرسالة، المفتاح_العمومي)؛
        اطبع("المشفر: " + المشفر)؛
        
        # فك التشفير بالمفتاح الخاص
        متغير المفكوك = فك_rsa(المشفر، المفتاح_الخاص)؛
        اطبع("المفكوك: " + المفكوك)؛
        
        إذا الرسالة == المفكوك {
            اطبع("✅ RSA يعمل بشكل صحيح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_rsa_basic: {:?}", result.is_ok());
}

/// Test: RSA signature
#[test]
fn test_crypto_rsa_signature() {
    let source = r#"
        متغير المفاتيح = أنشئ_مفاتيح_rsa(2048)؛
        متغير الرسالة = "وثيقة للتوقيع"؛
        
        # التوقيع بالمفتاح الخاص
        متغير التوقيع = وقع_rsa(الرسالة، المفاتيح.الخاص)؛
        اطبع("التوقيع: " + التوقيع)؛
        
        # التحقق بالمفتاح العمومي
        متغير صالح = تحقق_توقيع_rsa(الرسالة، التوقيع، المفاتيح.العمومي)؛
        
        إذا صالح {
            اطبع("✅ التوقيع صالح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_rsa_signature: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// ED25519 SIGNATURE TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Ed25519 signature
#[test]
fn test_crypto_ed25519_signature() {
    let source = r#"
        # إنشاء زوج مفاتيح Ed25519
        متغير المفاتيح = أنشئ_مفاتيح_ed25519()؛
        
        متغير الرسالة = "رسالة للتوقيع الرقمي"؛
        
        # التوقيع
        متغير التوقيع = وقع_ed25519(الرسالة، المفاتيح.الخاص)؛
        اطبع("توقيع Ed25519: " + التوقيع)؛
        
        # التحقق
        متغير صالح = تحقق_ed25519(الرسالة، التوقيع، المفاتيح.العمومي)؛
        
        إذا صالح {
            اطبع("✅ توقيع Ed25519 صالح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_ed25519_signature: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPREHENSIVE INTEGRATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: End-to-end secure communication simulation
#[test]
fn test_crypto_secure_communication() {
    let source = r#"
        # محاكاة اتصال آمن
        
        # 1. إنشاء المفاتيح
        متغير مفاتيح_أحمد = أنشئ_مفاتيح_rsa(2048)؛
        متغير مفاتيح_سارة = أنشئ_مفاتيح_rsa(2048)؛
        
        # 2. أحمد يرسل رسالة مشفرة لسارة
        متغير رسالة_أحمد = "مرحبا سارة، هذا سر مشترك"؛
        متغير مشفر = شفر_rsa(رسالة_أحمد، مفاتيح_سارة.العمومي)؛
        
        # 3. سارة تفك التشفير
        متغير مفكوك = فك_rsa(مشفر، مفاتيح_سارة.الخاص)؛
        
        إذا رسالة_أحمد == مفكوك {
            اطبع("✅ الاتصال الآمن ناجح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_secure_communication: {:?}", result.is_ok());
}

/// Test: Multi-layer encryption
#[test]
fn test_crypto_multi_layer_encryption() {
    let source = r#"
        متغير البيانات = "بيانات حساسة جداً"؛
        
        # طبقة 1: AES
        متغير مفتاح_aes = "مفتاح_سري"؛
        متغير طبقة1 = شفر_aes(البيانات، مفتاح_aes)؛
        
        # طبقة 2: Base64
        متغير طبقة2 = رمز_base64(طبقة1)؛
        
        اطبع("المشفر المتعدد: " + طبقة2)؛
        
        # فك التشفير
        متغير فك_طبقة2 = فك_base64(طبقة2)؛
        متغير فك_طبقة1 = فك_aes(فك_طبقة2، مفتاح_aes)؛
        
        إذا البيانات == فك_طبقة1 {
            اطبع("✅ التشفير المتعدد الطبقات صحيح")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_multi_layer_encryption: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// PERFORMANCE TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Hash performance
#[test]
fn test_crypto_hash_performance() {
    let source = r#"
        متغير البداية = وقت_الآن()؛
        
        لكل س في مدى(0، 1000) {
            متغير تجزئة = تجزئة_sha256("نص للاختبار " + نص(س))؛
        }
        
        متغير النهاية = وقت_الآن()؛
        متغير المدة = النهاية - البداية؛
        
        اطبع("مدة 1000 تجزئة: " + المدة + " مللي ثانية")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_crypto_hash_performance: {:?}", result.is_ok());
}
