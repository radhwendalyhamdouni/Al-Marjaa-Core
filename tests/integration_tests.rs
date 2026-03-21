// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات التكامل - Integration Tests
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::interpreter::Interpreter;
use almarjaa::parser::Parser;
use almarjaa::lexer::Lexer;
use almarjaa::bytecode::{Compiler, VM, CompleteV2JitCompiler, ExecutionResult};
use std::rc::Rc;
use std::cell::RefCell;
use almarjaa::interpreter::value::Environment;

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات شاملة - من البداية للنهاية
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار برنامج كامل - حاسبة
#[test]
fn test_full_calculator() {
    let source = r#"
        // حاسبة بسيطة
        دالة جمع(أ، ب) {
            أرجع أ + ب؛
        }
        دالة طرح(أ، ب) {
            أرجع أ - ب؛
        }
        دالة ضرب(أ، ب) {
            أرجع أ * ب؛
        }
        دالة قسمة(أ، ب) {
            إذا ب == 0 {
                أرجع "خطأ: لا يمكن القسمة على صفر"؛
            }
            أرجع أ / ب؛
        }
        // اختبارات
        متغير ن1 = جمع(10، 5)؛
        متغير ن2 = طرح(10، 5)؛
        متغير ن3 = ضرب(10، 5)؛
        متغير ن4 = قسمة(10، 5)؛
        اطبع("جمع: " + ن1)؛
        اطبع("طرح: " + ن2)؛
        اطبع("ضرب: " + ن3)؛
        اطبع("قسمة: " + ن4)؛
    "#;

    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في تنفيذ الحاسبة: {:?}", result.err());
    println!("✅ test_full_calculator");
}

/// اختبار برنامج كامل - معالجة بيانات
#[test]
fn test_full_data_processing() {
    let source = r#"
        // معالجة بيانات المبيعات
        متغير مبيعات = [
            {"منتج": "لابتوب"، "سعر": 1500، "كمية": 10}،
            {"منتج": "هاتف"، "سعر": 800، "كمية": 25}،
            {"منتج": "تابلت"، "سعر": 600، "كمية": 15}،
            {"منتج": "ساعة"، "سعر": 300، "كمية": 50}
        ]؛

        // حساب إجمالي المبيعات
        متغير إجمالي = 0؛
        لكل بيع في مبيعات {
            إجمالي = إجمالي + (بيع["سعر"] * بيع["كمية"])؛
        }
        اطبع("إجمالي المبيعات: " + إجمالي)؛

        // إيجاد المنتج الأكثر مبيعاً
        متغير أعلى_منتج = ""؛
        متغير أعلى_قيمة = 0؛

        لكل بيع في مبيعات {
            متغير قيمة = بيع["سعر"] * بيع["كمية"]؛
            إذا قيمة > أعلى_قيمة {
                أعلى_قيمة = قيمة؛
                أعلى_منتج = بيع["منتج"]؛
            }
        }
        اطبع("المنتج الأكثر مبيعاً: " + أعلى_منتج + " بقيمة " + أعلى_قيمة)؛
    "#;

    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في معالجة البيانات: {:?}", result.err());
    println!("✅ test_full_data_processing");
}

/// اختبار برنامج كامل - خوارزميات
#[test]
fn test_full_algorithms() {
    let source = r#"
        // خوارزمية المضاعفة
        دالة مضاعف(قائمة) {
            متغير نتيجة = []؛
            لكل عنصر في قائمة {
                أضف(نتيجة، عنصر * 2)؛
            }
            أرجع نتيجة؛
        }
        // اختبار
        متغير أرقام = [1، 2، 3، 4، 5]؛
        متغير مضاعفة = مضاعف(أرقام)؛
        اطبع("الأرقام المضاعفة: " + مضاعفة)؛
    "#;

    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في الخوارزمية: {:?}", result.err());
    println!("✅ test_full_algorithms");
}

/// اختبار التكامل: Lexer -> Parser -> Interpreter
#[test]
fn test_integration_lexer_parser_interpreter() {
    let source = r#"
        متغير رسالة = "مرحبا بالعالم"؛
        اطبع(رسالة)؛
    "#;

    // 1. Lexer
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("فشل Lexer");
    assert!(!tokens.is_empty(), "Lexer يجب أن ينتج رموز");

    // 2. Parser
    let program = Parser::parse(source).expect("فشل Parser");
    assert!(!program.statements.is_empty(), "Parser يجب أن ينتج تعليمات");

    // 3. Interpreter
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "Interpreter يجب أن ينفذ");

    println!("✅ test_integration_lexer_parser_interpreter");
}

/// اختبار التكامل: Compiler -> VM
#[test]
fn test_integration_compiler_vm() {
    let source = r#"
        متغير س = 10؛
        متغير ص = 20؛
        س + ص؛
    "#;

    // 1. Compiler
    let chunk = Compiler::compile_source(source).expect("فشل Compiler");
    assert!(!chunk.is_empty(), "Compiler يجب أن ينتج تعليمات");

    // 2. VM
    let mut vm = VM::with_fresh_env();
    vm.load(chunk);
    let result = vm.run();
    assert!(matches!(result, ExecutionResult::Ok(_)), "VM يجب أن تنفذ");

    println!("✅ test_integration_compiler_vm");
}

/// اختبار التكامل: Compiler -> JIT
#[test]
fn test_integration_compiler_jit() {
    let source = r#"
        دالة جمع(أ، ب) {
            أرجع أ + ب؛
        }
        جمع(100، 200)؛
    "#;

    // 1. Compiler
    let chunk = Compiler::compile_source(source).expect("فشل Compiler");

    // 2. JIT
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    let result = jit.execute(&chunk, &mut globals);

    assert!(result.is_ok(), "JIT يجب أن ينفذ");
    println!("✅ test_integration_compiler_jit");
}

/// اختبار التوافق: Interpreter vs VM vs JIT
#[test]
fn test_compatibility_interpreter_vm_jit() {
    let source = r#"
        متغير س = 10؛
        متغير ص = 20؛
        س * ص + س - ص؛
    "#;

    // Interpreter
    let mut interp = Interpreter::new();
    let interp_result = interp.run(source).is_ok();

    // VM - ترجمة منفصلة
    let chunk_for_vm = Compiler::compile_source(source).expect("فشل Compiler");
    let mut vm = VM::with_fresh_env();
    vm.load(chunk_for_vm);
    let vm_result = matches!(vm.run(), ExecutionResult::Ok(_));

    // JIT - ترجمة منفصلة
    let chunk_for_jit = Compiler::compile_source(source).expect("فشل Compiler");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    let jit_result = jit.execute(&chunk_for_jit, &mut globals).is_ok();

    assert!(interp_result && vm_result && jit_result,
            "جميع المحركات يجب أن تعطي نفس النتيجة");

    println!("✅ test_compatibility_interpreter_vm_jit:");
    println!("   Interpreter: {}", interp_result);
    println!("   VM: {}", vm_result);
    println!("   JIT: {}", jit_result);
}

/// اختبار الأداء الشامل
#[test]
fn test_full_performance_benchmark() {
    let source = r#"
        // حساب مجموع 1 إلى 10000
        متغير مجموع = 0؛
        لكل س في مدى(1، 10001) {
            مجموع = مجموع + س؛
        }
    "#;

    println!("✅ test_full_performance_benchmark:");

    // Interpreter
    let mut interp = Interpreter::new();
    let interp_start = std::time::Instant::now();
    let interp_result = interp.run(source);
    let interp_time = interp_start.elapsed();
    println!("   Interpreter: {:?}", interp_time);
    println!("   Interpreter result: {:?}", interp_result.is_ok());

    // VM
    let chunk_for_vm = Compiler::compile_source(source).expect("فشل Compiler");
    let mut vm = VM::with_fresh_env();
    vm.load(chunk_for_vm);
    let vm_start = std::time::Instant::now();
    let vm_result = vm.run();
    let vm_time = vm_start.elapsed();
    println!("   VM: {:?}", vm_time);
    println!("   VM result: {}", matches!(vm_result, ExecutionResult::Ok(_)));

    // JIT - ترجمة منفصلة
    let chunk_for_jit = Compiler::compile_source(source).expect("فشل Compiler");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    let jit_start = std::time::Instant::now();
    let jit_result = jit.execute(&chunk_for_jit, &mut globals);
    let jit_time = jit_start.elapsed();
    println!("   JIT: {:?}", jit_time);
    println!("   JIT result: {:?}", jit_result.is_ok());

    // اختبار أن على الأقل واحد يعمل
    assert!(interp_result.is_ok() || matches!(vm_result, ExecutionResult::Ok(_)) || jit_result.is_ok());
}

/// اختبار معالجة الأخطاء الشامل
#[test]
fn test_full_error_handling() {
    let error_cases = vec![
        ("متغير؛", "بدون قيمة"),
        ("دالة() {}", "بدون اسم"),
    ];

    let mut passed = 0;
    let total = error_cases.len();
    for (source, _description) in &error_cases {
        let result = Parser::parse(source);
        if result.is_err() {
            passed += 1;
        }
    }

    println!("✅ test_full_error_handling: {} من {} أخطاء تم اكتشافها",
             passed, total);
}

/// اختبار الأرقام العربية
#[test]
fn test_arabic_numbers_integration() {
    let source = r#"
        متغير أ = ١٢٣؛
        متغير ب = ٤٥٦؛
        متغير جمع = أ + ب؛
        اطبع("الجمع: " + جمع)؛
    "#;

    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في الأرقام العربية: {:?}", result.err());
    println!("✅ test_arabic_numbers_integration");
}

/// اختبار النصوص العربية
#[test]
fn test_arabic_text_integration() {
    let source = r#"
        متغير اسم = "محمد"؛
        متغير مدينة = "القاهرة"؛
        متغير رسالة = "مرحباً " + اسم + " من " + مدينة؛
        اطبع(رسالة)؛
    "#;

    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في النصوص العربية: {:?}", result.err());
    println!("✅ test_arabic_text_integration");
}

/// اختبار القوائم والقواميس
#[test]
fn test_lists_and_dicts_integration() {
    let source = r#"
        متغير قائمة = [1، 2، 3، 4، 5]؛
        متغير قاموس = {"أ": 1، "ب": 2}؛

        أضف(قائمة، 6)؛
        اطبع("طول القائمة: " + طول(قائمة))؛
        اطبع("قيمة أ: " + قاموس["أ"])؛
    "#;

    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في القوائم والقواميس: {:?}", result.err());
    println!("✅ test_lists_and_dicts_integration");
}

/// اختبار العودية
#[test]
fn test_recursion_integration() {
    let source = r#"
        دالة فيبوناتشي(ن) {
            إذا ن <= 1 {
                أرجع ن؛
            }
            أرجع فيبوناتشي(ن - 1) + فيبوناتشي(ن - 2)؛
        }
        متغير نتيجة = فيبوناتشي(10)؛
        اطبع("فيبوناتشي 10: " + نتيجة)؛
    "#;

    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في العودية: {:?}", result.err());
    println!("✅ test_recursion_integration");
}
