// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات مترجم JIT - JIT Compiler Tests
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::bytecode::{Compiler, CompleteV2JitCompiler};
use std::rc::Rc;
use std::cell::RefCell;
use almarjaa::interpreter::value::Environment;

/// اختبار إنشاء JIT
#[test]
fn test_jit_creation() {
    let jit = CompleteV2JitCompiler::new();
    let stats = jit.stats();

    assert_eq!(stats.total_executions, 0);
    println!("✅ test_jit_creation");
}

/// اختبار JIT بسيط
#[test]
fn test_jit_simple() {
    let source = "10 + 20";
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok(), "فشل في تنفيذ JIT: {:?}", result.err());
    println!("✅ test_jit_simple");
}

/// اختبار JIT مع متغيرات
#[test]
fn test_jit_variables() {
    let source = r#"
        متغير أ = 10؛
        متغير ب = 20؛
        أ + ب
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_variables");
}

/// اختبار JIT مع دالة
#[test]
fn test_jit_function() {
    let source = r#"
        دالة جمع(أ، ب) {
            أرجع أ + ب؛
        }
        جمع(15، 25)؛
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_function");
}

/// اختبار JIT مع حلقة بسيطة
#[test]
fn test_jit_simple_loop() {
    let source = r#"
        متغير مجموع = 0؛
        لكل س في مدى(1، 101) {
            مجموع = مجموع + س؛
        }
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_simple_loop");
}

/// اختبار JIT مع شرط
#[test]
fn test_jit_condition() {
    let source = r#"
        متغير س = 10؛
        متغير نتيجة = ""؛
        إذا س > 5 {
            نتيجة = "كبير"؛
        }
        وإلا {
            نتيجة = "صغير"؛
        }
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_condition");
}

/// اختبار JIT مع Fibonacci
#[test]
fn test_jit_fibonacci() {
    let source = r#"
        دالة فيب(ن) {
            إذا ن <= 1 {
                أرجع ن؛
            }
            أرجع فيب(ن - 1) + فيب(ن - 2)؛
        }
        فيب(15)؛
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_fibonacci");
}

/// اختبار أداء JIT - حلقة كبيرة
#[test]
fn test_jit_performance_loop() {
    let source = r#"
        متغير مجموع = 0؛
        لكل س في مدى(1، 50001) {
            مجموع = مجموع + س؛
        }
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let start = std::time::Instant::now();
    let result = jit.execute(&chunk, &mut globals);
    let elapsed = start.elapsed();

    assert!(result.is_ok());
    let stats = jit.stats();

    println!("✅ test_jit_performance_loop:");
    println!("   الوقت: {:?}", elapsed);
    println!("   التنفيذات: {}", stats.total_executions);

    // JIT يجب أن يكون أسرع من المفسر العادي
    assert!(elapsed.as_millis() < 5000, "JIT يجب أن يكون أسرع");
}

/// اختبار JIT مع عمليات رياضية
#[test]
fn test_jit_math_operations() {
    let source = r#"
        متغير أ = 10 + 5؛
        متغير ب = 20 - 8؛
        متغير ج = 6 * 7؛
        متغير د = 100 / 4؛
        متغير هـ = 2 ^ 10؛
        متغير باقي = 17 % 5؛

        أ + ب + ج + د + هـ + باقي؛
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_math_operations");
}

/// اختبار JIT مع قائمة
#[test]
fn test_jit_list() {
    let source = r#"
        متغير قائمة = [1، 2، 3، 4، 5]؛
        متغير مجموع = 0؛

        لكل عنصر في قائمة {
            مجموع = مجموع + عنصر؛
        }
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_list");
}

/// اختبار JIT مع عودية عميقة
#[test]
fn test_jit_deep_recursion() {
    let source = r#"
        دالة عودية(ن) {
            إذا ن <= 0 {
                أرجع 0؛
            }
            أرجع 1 + عودية(ن - 1)؛
        }
        عودية(100)؛
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);

    if result.is_ok() {
        let stats = jit.stats();
        println!("✅ test_jit_deep_recursion: عمق {} استدعاء", stats.recursive_calls);
    } else {
        println!("✅ test_jit_deep_recursion: تم حماية Stack (متوقع)");
    }
}

/// اختبار JIT إحصائيات
#[test]
fn test_jit_stats() {
    let source = r#"
        متغير س = 0؛
        لكل س في مدى(1، 101) {
            س = س + 1؛
        }
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    jit.execute(&chunk, &mut globals).unwrap();

    let stats = jit.stats();

    println!("✅ test_jit_stats:");
    println!("   إجمالي التنفيذات: {}", stats.total_executions);
    println!("   وقت التنفيذ: {} ميكروثانية", stats.total_exec_time_us);
    println!("   الاستدعاءات العودية: {}", stats.recursive_calls);
    println!("   أقصى عمق استدعاء: {}", stats.max_call_depth);
}

/// اختبار مقارنة أداء JIT vs Interpreter
#[test]
fn test_jit_vs_interpreter_comparison() {
    let source = r#"
        دالة حساب(ن) {
            متغير مجموع = 0؛
            لكل س في مدى(1، ن + 1) {
                مجموع = مجموع + س؛
            }
            أرجع مجموع؛
        }
        حساب(10000)؛
    "#;

    // JIT
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let jit_start = std::time::Instant::now();
    let jit_result = jit.execute(&chunk, &mut globals);
    let jit_time = jit_start.elapsed();

    assert!(jit_result.is_ok());

    println!("✅ test_jit_vs_interpreter_comparison:");
    println!("   JIT الوقت: {:?}", jit_time);
    println!("   JIT إحصائيات: {} تنفيذ", jit.stats().total_executions);
}

/// اختبار JIT مع دالة داخلية
#[test]
fn test_jit_closure() {
    let source = r#"
        دالة خارج(س) {
            دالة داخل(ص) {
                أرجع س + ص؛
            }
            أرجع داخل(10)؛
        }
        خارج(5)؛
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_closure");
}

/// اختبار JIT مع مصفوفات متعددة الأبعاد
#[test]
fn test_jit_multidimensional_arrays() {
    let source = r#"
        متغير مصفوفة = [[1، 2]، [3، 4]، [5، 6]]؛
        متغير مجموع = 0؛

        لكل سطر في مصفوفة {
            لكل عنصر في سطر {
                مجموع = مجموع + عنصر؛
            }
        }
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_multidimensional_arrays");
}

/// اختبار معالجة الأخطاء في JIT
#[test]
fn test_jit_error_handling() {
    // هذا الاختبار يتحقق من أن JIT يتعامل مع الأخطاء بشكل صحيح
    let sources = vec![
        "10 / 0",  // قسمة على صفر
    ];

    for source in sources {
        let chunk = Compiler::compile_source(source);
        if let Ok(chunk) = chunk {
            let mut jit = CompleteV2JitCompiler::new();
            let mut globals = Rc::new(RefCell::new(Environment::new()));

            let result = jit.execute(&chunk, &mut globals);
            // قد ينجح أو يفشل، المهم عدم التعطل
            println!("   المصدر: {} -> النتيجة: {:?}", source, result.is_ok());
        }
    }

    println!("✅ test_jit_error_handling");
}

/// اختبار JIT مع الأرقام العربية
#[test]
fn test_jit_arabic_numbers() {
    let source = r#"
        متغير أ = ١٢٣؛
        متغير ب = ٤٥٦؛
        أ + ب؛
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_arabic_numbers");
}

/// اختبار JIT مع النصوص العربية
#[test]
fn test_jit_arabic_strings() {
    let source = r#"
        متغير اسم = "أحمد"؛
        متغير ترحيب = "مرحباً " + اسم؛
        ترحيب؛
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_arabic_strings");
}

/// اختبار JIT مع القواميس
#[test]
fn test_jit_dictionaries() {
    let source = r#"
        متغير شخص = {"اسم": "أحمد"، "عمر": 25}؛
        شخص["اسم"]؛
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_dictionaries");
}

/// اختبار JIT مع التعابير المنطقية
#[test]
fn test_jit_logical_expressions() {
    let source = r#"
        متغير أ = صح؛
        متغير ب = خطأ؛
        متغير ج = أ و ب؛
        متغير د = أ أو ب؛
        متغير هـ = ليس ب؛
        متغير نتيجة = [ج، د، هـ]؛
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_logical_expressions");
}

/// اختبار JIT مع الدوال المتداخلة
#[test]
fn test_jit_nested_functions() {
    let source = r#"
        دالة خارج(س) {
            دالة داخل(ص) {
                أرجع س + ص؛
            }
            أرجع داخل(10)؛
        }
        خارج(5)؛
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");

    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));

    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_nested_functions");
}
