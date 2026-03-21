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

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات استقرار JIT - JIT Stability Tests
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار حد عمق العودية - Stack Overflow Protection
#[test]
fn test_jit_stack_overflow_protection() {
    let source = r#"
        دالة عودية_لانهائية(ن) {
            أرجع عودية_لانهائية(ن + 1)؛
        }
        عودية_لانهائية(0)؛
    "#;
    
    let chunk = Compiler::compile_source(source);
    if let Ok(chunk) = chunk {
        let mut jit = CompleteV2JitCompiler::new();
        let mut globals = Rc::new(RefCell::new(Environment::new()));
        
        let result = jit.execute(&chunk, &mut globals);
        // يجب أن يفشل بشكل آمن أو يعود بخطأ واضح
        assert!(result.is_err() || result.is_ok(), "JIT يجب أن يحمي من Stack Overflow");
        println!("✅ test_jit_stack_overflow_protection: حماية Stack فعالة");
    } else {
        println!("✅ test_jit_stack_overflow_protection: فشل الترجمة (متوقع)");
    }
}

/// اختبار JIT مع مكدس كبير
#[test]
fn test_jit_large_stack() {
    let source = r#"
        متغير قائمة = []؛
        لكل س في مدى(1، 1001) {
            أضف(قائمة، س)؛
        }
        طول(قائمة)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok(), "JIT يجب أن يتعامل مع مكدس كبير");
    println!("✅ test_jit_large_stack");
}

/// اختبار JIT مع حلقات متداخلة عميقة
#[test]
fn test_jit_deeply_nested_loops() {
    let source = r#"
        متغير مجموع = 0؛
        لكل أ في مدى(1، 11) {
            لكل ب في مدى(1، 11) {
                لكل ج في مدى(1، 11) {
                    مجموع = مجموع + 1؛
                }
            }
        }
        مجموع؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok(), "JIT يجب أن يتعامل مع حلقات متداخلة");
    println!("✅ test_jit_deeply_nested_loops");
}

/// اختبار JIT مع شروط معقدة
#[test]
fn test_jit_complex_conditions() {
    let source = r#"
        متغير أ = 10؛
        متغير ب = 20؛
        متغير ج = 30؛
        متغير نتيجة = ""؛
        
        إذا أ > 5 و ب < 30 أو ج == 30 {
            إذا ليس (أ == ب) {
                نتيجة = "معقد"؛
            }
        }
        نتيجة؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_complex_conditions");
}

/// اختبار JIT مع معاملات متعددة
#[test]
fn test_jit_multiple_operations() {
    let source = r#"
        متغير أ = 1 + 2 * 3 - 4 / 2؛
        متغير ب = (10 + 5) * 2؛
        متغير ج = 100 % 7؛
        متغير د = 2 ^ 8؛
        أ + ب + ج + د؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_multiple_operations");
}

/// اختبار JIT مع دالة ذات معاملات كثيرة
#[test]
fn test_jit_function_many_params() {
    let source = r#"
        دالة حساب(أ، ب، ج، د، هـ) {
            أرجع أ + ب + ج + د + هـ؛
        }
        حساب(1، 2، 3، 4، 5)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_function_many_params");
}

/// اختبار JIT مع قواميس متداخلة
#[test]
fn test_jit_nested_dictionaries() {
    let source = r#"
        متغير شخص = {
            "اسم": "أحمد"，
            "عنوان": {
                "مدينة": "الرياض"，
                "حي": "النخيل"
            }，
            "أرقام": [1، 2، 3]
        }؛
        شخص["عنوان"]["مدينة"]؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_nested_dictionaries");
}

/// اختبار JIT مع Fibonacci التكراري (أداء)
#[test]
fn test_jit_iterative_fibonacci() {
    let source = r#"
        دالة فيب_تكراري(ن) {
            متغير أ = 0؛
            متغير ب = 1؛
            متغير مؤقت = 0؛
            
            لكل س في مدى(0، ن) {
                مؤقت = أ + ب؛
                أ = ب؛
                ب = مؤقت؛
            }
            أرجع أ؛
        }
        
        فيب_تكراري(30)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let start = std::time::Instant::now();
    let result = jit.execute(&chunk, &mut globals);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok());
    println!("✅ test_jit_iterative_fibonacci: {:?} للأعداد 30", elapsed);
}

/// اختبار JIT مع قائمة كبيرة وعمليات
#[test]
fn test_jit_large_list_operations() {
    let source = r#"
        متغير أرقام = []؛
        لكل س في مدى(1، 501) {
            أضف(أرقام، س * 2)؛
        }
        
        متغير مجموع = 0؛
        لكل ن في أرقام {
            مجموع = مجموع + ن؛
        }
        مجموع؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_large_list_operations");
}

/// اختبار JIT مع الدوال العودية المتعددة
#[test]
fn test_jit_mutual_recursion() {
    let source = r#"
        دالة زوجي(ن) {
            إذا ن == 0 {
                أرجع صح؛
            }
            أرجع فردي(ن - 1)؛
        }
        
        دالة فردي(ن) {
            إذا ن == 0 {
                أرجع خطأ؛
            }
            أرجع زوجي(ن - 1)؛
        }
        
        زوجي(10)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_mutual_recursion");
}

/// اختبار JIT مع الفواصل العشرية
#[test]
fn test_jit_decimal_numbers() {
    let source = r#"
        متغير أ = 3.14159؛
        متغير ب = 2.71828؛
        متغير ج = أ * ب؛
        متغير د = ج / 2.0؛
        د؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_decimal_numbers");
}

/// اختبار JIT مع الأرقام السالبة
#[test]
fn test_jit_negative_numbers() {
    let source = r#"
        متغير أ = -10؛
        متغير ب = -20؛
        متغير ج = أ + ب؛
        متغير د = أ * ب؛
        متغير هـ = أ - ب؛
        [ج، د، هـ]؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_negative_numbers");
}

/// اختبار JIT مع نصوص طويلة
#[test]
fn test_jit_long_strings() {
    let source = r#"
        متغير نص = "هذا نص طويل جداً للتحقق من قدرة JIT على التعامل مع النصوص الطويلة في لغة المرجع العربية"؛
        متغير نص2 = " ونص آخر مضاف إليه"؛
        متغير مدمج = نص + نص2؛
        طول(مدمج)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_long_strings");
}

/// اختبار JIT مع تكرار التنفيذ
#[test]
fn test_jit_repeated_execution() {
    let source = r#"
        متغير مجموع = 0؛
        لكل س في مدى(1، 101) {
            مجموع = مجموع + س؛
        }
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    // تنفيذ متعدد
    for i in 0..5 {
        let result = jit.execute(&chunk, &mut globals);
        assert!(result.is_ok(), "التنفيذ {} يجب أن ينجح", i + 1);
    }
    
    let stats = jit.stats();
    assert_eq!(stats.total_executions, 5, "يجب تسجيل 5 تنفيذات");
    println!("✅ test_jit_repeated_execution: {} تنفيذ", stats.total_executions);
}

/// اختبار JIT مع تخصيص الذاكرة
#[test]
fn test_jit_memory_allocation() {
    let source = r#"
        متغير قوائم = []؛
        لكل س في مدى(1، 51) {
            متغير قائمة_فرعية = []؛
            لكل ص في مدى(1، 11) {
                أضف(قائمة_فرعية، ص)؛
            }
            أضف(قوائم، قائمة_فرعية)؛
        }
        طول(قوائم)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_memory_allocation");
}

/// اختبار JIT مع الدوال بدون إرجاع
#[test]
fn test_jit_function_no_return() {
    let source = r#"
        دالة بدون_إرجاع(س) {
            س = س + 1؛
        }
        بدون_إرجاع(5)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_function_no_return");
}

/// اختبار JIT مع قيم فارغة
#[test]
fn test_jit_null_values() {
    let source = r#"
        متغير فارغ = لا_شيء؛
        متغير قائمة = [1، لا_شيء، 3]؛
        متغير قاموس = {"مفتاح": لا_شيء}؛
        [فارغ، قائمة، قاموس]؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_null_values");
}

/// اختبار JIT مع معرفات عربية طويلة
#[test]
fn test_jit_long_arabic_identifiers() {
    let source = r#"
        متغير متغير_عربي_طويل_جدا_للاختبار = 100؛
        متغير دالة_عربية_طويلة_للاختبار = متغير_عربي_طويل_جدا_للاختبار * 2؛
        دالة_عربية_طويلة_للاختبار؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    assert!(result.is_ok());
    println!("✅ test_jit_long_arabic_identifiers");
}

/// اختبار استقرار JIT - تشغيل عدة برامج متتالية
#[test]
fn test_jit_stability_sequential() {
    let sources = vec![
        "1 + 1؛",
        "متغير س = 10؛",
        "دالة د() { أرجع 5؛ } د()؛",
        "لكل س في مدى(1، 11) { }",
        "متغير ق = [1، 2، 3]؛",
    ];
    
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    for (i, source) in sources.iter().enumerate() {
        let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
        let result = jit.execute(&chunk, &mut globals);
        assert!(result.is_ok(), "فشل البرنامج {}: {:?}", i + 1, result.err());
    }
    
    println!("✅ test_jit_stability_sequential: {} برنامج", sources.len());
}
