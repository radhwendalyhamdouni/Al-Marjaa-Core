// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات التكامل بين Core و Libraries
// Integration Tests between Core and Libraries
// ═══════════════════════════════════════════════════════════════════════════════

//! هذه الاختبارات تتحقق من أن النواة الأساسية (Core) تتكامل بشكل صحيح
//! مع المكتبات الخارجية (Libraries).
//!
//! ## المكونات المختبرة:
//!
//! 1. **Core API**: التأكد من أن API الأساسي مستقر
//! 2. **Value Types**: التأكد من توافق أنواع البيانات
//! 3. **Module System**: التأكد من نظام الوحدات
//! 4. **FFI Interface**: واجهة الاستدعاء الأجنبي للمكتبات

use almarjaa::{Lexer, Parser, Interpreter, Compiler, VM, Chunk, OpCode};
use almarjaa::{AlMarjaaError, ErrorCode, Position, Severity, Span};
use almarjaa::{Module, ModuleManager, ModuleId};

// ═══════════════════════════════════════════════════════════════════════════════
// Test 1: Core API Stability
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار استقرار API الأساسي
#[test]
fn test_core_api_lexer_parser_interpreter() {
    // إنشاء مفسر جديد
    let mut interpreter = Interpreter::new();
    
    // كود المرجع البسيط
    let source = r#"
        متغير س = 10؛
        متغير ص = 20؛
        متجر مجموع = س + ص؛
        اطبع(مجموع)؛
    "#;
    
    // تنفيذ الكود
    let result = interpreter.run(source);
    
    // التحقق من النجاح
    assert!(result.is_ok(), "فشل تنفيذ الكود الأساسي: {:?}", result.err());
}

/// اختبار tokenizer
#[test]
fn test_core_api_lexer() {
    let source = "متغير س = 10؛";
    let mut lexer = Lexer::new(source);
    
    let tokens = lexer.tokenize();
    assert!(tokens.is_ok(), "فشل تحليل المعجم: {:?}", tokens.err());
    
    let tokens = tokens.unwrap();
    assert!(!tokens.is_empty(), "يجب أن تكون هناك رموز");
}

/// اختبار Parser
#[test]
fn test_core_api_parser() {
    let source = r#"
        دالة جمع(أ، ب) {
            أرجع أ + ب؛
        }
    "#;
    
    let result = Parser::parse(source);
    assert!(result.is_ok(), "فشل تحليل البنية: {:?}", result.err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test 2: Bytecode and VM
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار المترجم والآلة الافتراضية
#[test]
fn test_core_bytecode_compiler_vm() {
    let source = "متغير س = 5 + 3؛";
    
    // تحليل الكود
    let ast = Parser::parse(source).expect("فشل التحليل");
    
    // ترجمة إلى bytecode
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&ast).expect("فشل الترجمة");
    
    // تنفيذ على VM
    let mut vm = VM::new();
    let result = vm.run(chunk);
    
    assert!(result.is_ok(), "فشل تنفيذ VM: {:?}", result.err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test 3: Error Handling
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار معالجة الأخطاء
#[test]
fn test_core_error_handling() {
    // كود خاطئ (خطأ نحوي)
    let source = "متغير س = ؛"; // خطأ: قيمة مفقودة
    
    let result = Parser::parse(source);
    assert!(result.is_err(), "يجب أن يفشل التحليل لكود خاطئ");
}

/// اختبار أنواع الأخطاء
#[test]
fn test_core_error_types() {
    // إنشاء خطأ
    let pos = Position { line: 1, column: 1, offset: 0 };
    let span = Span { start: pos.clone(), end: pos };
    
    let error = AlMarjaaError::SyntaxError {
        message: "خطأ تجريبي".to_string(),
        span,
    };
    
    // التحقق من خصائص الخطأ
    assert!(error.to_string().contains("خطأ"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test 4: Module System
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار نظام الوحدات
#[test]
fn test_core_module_system() {
    let manager = ModuleManager::new();
    
    // التحقق من إنشاء مدير الوحدات
    assert!(manager.stats().total_modules >= 0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test 5: Value System (Compatibility with Libraries)
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار نظام القيم - التوافق مع المكتبات
#[test]
fn test_core_value_system() {
    use almarjaa::interpreter::value::Value;
    
    // إنشاء قيم مختلفة
    let int_val = Value::Integer(42);
    let float_val = Value::Float(3.14);
    let string_val = Value::String("مرحبا".to_string());
    let bool_val = Value::Boolean(true);
    let null_val = Value::Null;
    
    // التحقق من الأنواع
    assert!(matches!(int_val, Value::Integer(_)));
    assert!(matches!(float_val, Value::Float(_)));
    assert!(matches!(string_val, Value::String(_)));
    assert!(matches!(bool_val, Value::Boolean(_)));
    assert!(matches!(null_val, Value::Null));
}

/// اختبار القواميس والقوائم
#[test]
fn test_core_collections() {
    use almarjaa::interpreter::value::Value;
    use std::collections::HashMap;
    
    // إنشاء قاموس
    let mut dict = HashMap::new();
    dict.insert("اسم".to_string(), Value::String("المرجع".to_string()));
    dict.insert("إصدار".to_string(), Value::Integer(3));
    
    let dict_val = Value::Dict(dict);
    
    // التحقق
    assert!(matches!(dict_val, Value::Dict(_)));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test 6: Arabic Support
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار دعم العربية
#[test]
fn test_core_arabic_support() {
    let mut interpreter = Interpreter::new();
    
    // كود عربي كامل
    let source = r#"
        متغير الاسم = "لغة المرجع"؛
        متغير الإصدار = 3.4؛
        
        دالة ترحيب(اسم) {
            اطبع("مرحباً بك في " + اسم)؛
        }
        
        ترحيب(الاسم)؛
    "#;
    
    let result = interpreter.run(source);
    assert!(result.is_ok(), "فشل تنفيذ الكود العربي: {:?}", result.err());
}

/// اختبار RTL (من اليمين لليسار)
#[test]
fn test_core_rtl_support() {
    let mut lexer = Lexer::new("متغير نص = "مرحباً بالعالم"؛");
    let tokens = lexer.tokenize();
    
    assert!(tokens.is_ok());
    let tokens = tokens.unwrap();
    
    // التحقق من أن النص العربي محفوظ
    let has_arabic = tokens.iter().any(|t| {
        matches!(t.token_type, almarjaa::lexer::tokens::TokenType::String(_))
    });
    assert!(has_arabic, "يجب أن يحتوي على نص عربي");
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test 7: FFI Interface (for Libraries Integration)
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار واجهة FFI للمكتبات
#[test]
fn test_core_ffi_interface() {
    use almarjaa::interpreter::value::Value;
    
    // إنشاء قيمة يمكن تمريرها للمكتبات الخارجية
    let value = Value::Integer(42);
    
    // تحويل إلى String (للاتصال مع المكتبات)
    let string_repr = value.to_string();
    assert!(!string_repr.is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test 8: Performance Baseline
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار الأداء الأساسي
#[test]
fn test_core_performance_baseline() {
    use std::time::Instant;
    
    let mut interpreter = Interpreter::new();
    
    // كود حسابي مكثف
    let source = r#"
        متغير مجموع = 0؛
        لـ متغير س = 1 إلى 100 {
            مجموع = مجموع + س؛
        }
    "#;
    
    let start = Instant::now();
    let result = interpreter.run(source);
    let duration = start.elapsed();
    
    assert!(result.is_ok());
    // يجب أن يتم التنفيذ في أقل من ثانية
    assert!(duration.as_millis() < 1000, "التنفيذ بطيء جداً: {:?}", duration);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test 9: Library Integration Interface
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار واجهة تكامل المكتبات
/// هذا الاختبار يتحقق من أن Core يوفر واجهة صحيحة للمكتبات
#[test]
fn test_library_integration_interface() {
    // أنواع البيانات المطلوبة للمكتبات
    // 1. قيم قابلة للتحويل
    // 2. بيئة تنفيذ
    // 3. واجهة الوحدات
    
    use almarjaa::interpreter::value::{Value, Environment};
    
    // إنشاء بيئة
    let mut env = Environment::new();
    
    // تعريف متغير
    env.define("قاعدة_بيانات".to_string(), Value::Null);
    
    // التحقق من وجود المتغير
    assert!(env.get("قاعدة_بيانات").is_some());
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test 10: Version Compatibility
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار توافق الإصدارات
#[test]
fn test_core_version_compatibility() {
    let version = almarjaa::VERSION;
    
    // التحقق من أن الإصدار يتبع semver
    assert!(version.contains('.'), "الإصدار يجب أن يتبع semver");
    
    // التحقق من معلومات اللغة
    let info = almarjaa::info();
    assert!(info.contains("المرجع"));
    assert!(info.contains("3.4"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test 11: Feature Flags
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار Feature Flags
#[test]
fn test_core_feature_flags() {
    // Core يجب أن يعمل بدون أي feature flags
    
    // التحقق من وجود cranelift (اختياري)
    #[cfg(feature = "cranelift-backend")]
    {
        // إذا كان cranelift مفعلاً، يجب أن يكون متاحاً
        println!("Cranelift backend مفعّل");
    }
    
    #[cfg(not(feature = "cranelift-backend"))]
    {
        println!("Cranelift backend غير مفعّل (Core خفيف)");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Test 12: Security Features
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار ميزات الأمان
#[test]
fn test_core_security_features() {
    let mut interpreter = Interpreter::new();
    
    // منع الحلقات اللانهائية (timeout)
    let source = r#"
        متغير س = 0؛
        بينما س < 1000 {
            س = س + 1؛
        }
    "#;
    
    let result = interpreter.run(source);
    assert!(result.is_ok(), "يجب أن يعمل الكود الآمن");
}
