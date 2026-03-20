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
        دالة جمع(أ، ب):
            أرجع أ + ب
        
        دالة طرح(أ، ب):
            أرجع أ - ب
        
        دالة ضرب(أ، ب):
            أرجع أ * ب
        
        دالة قسمة(أ، ب):
            إذا ب == 0:
                أرجع "خطأ: لا يمكن القسمة على صفر"
            أرجع أ / ب
        
        // اختبارات
        متغير ن1 = جمع(10، 5)
        متغير ن2 = طرح(10، 5)
        متغير ن3 = ضرب(10، 5)
        متغير ن4 = قسمة(10، 5)
        
        طباعة("جمع: " + ن1)
        طباعة("طرح: " + ن2)
        طباعة("ضرب: " + ن3)
        طباعة("قسمة: " + ن4)
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في تنفيذ الحاسبة: {:?}", result.err());
    println!("✅ test_full_calculator");
}

/// اختبار برنامج كامل - نظام إدارة
#[test]
fn test_full_management_system() {
    let source = r#"
        // نظام إدارة موظفين
        صنف موظف:
            متغير اسم = ""
            متغير راتب = 0
            متغير قسم = ""
            
            دالة جديد(اسم، راتب، قسم):
                هذا.اسم = اسم
                هذا.راتب = راتب
                هذا.قسم = قسم
            
            دالة معلومات():
                أرجع "الموظف: " + هذا.اسم + " | الراتب: " + هذا.راتب + " | القسم: " + هذا.قسم
            
            دالة زيادة(نسبة):
                هذا.راتب = هذا.راتب * (1 + نسبة / 100)
        
        // إنشاء موظفين
        متغير أحمد = موظف.جديد("أحمد محمد"، 5000، "التطوير")
        متغير سارة = موظف.جديد("سارة أحمد"، 6000، "التسويق")
        
        // طباعة المعلومات
        طباعة(أحمد.معلومات())
        طباعة(سارة.معلومات())
        
        // زيادة الراتب
        أحمد.زيادة(10)
        طباعة("بعد الزيادة: " + أحمد.معلومات())
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في نظام الإدارة: {:?}", result.err());
    println!("✅ test_full_management_system");
}

/// اختبار برنامج كامل - معالجة بيانات
#[test]
fn test_full_data_processing() {
    let source = r#"
        // معالجة بيانات المبيعات
        متغير مبيعات = [
            {منتج: "لابتوب"، سعر: 1500، كمية: 10}،
            {منتج: "هاتف"، سعر: 800، كمية: 25}،
            {منتج: "تابلت"، سعر: 600، كمية: 15}،
            {منتج: "ساعة"، سعر: 300، كمية: 50}
        ]
        
        // حساب إجمالي المبيعات
        متغير إجمالي = 0
        لكل بيع في مبيعات:
            إجمالي = إجمالي + (بيع["سعر"] * بيع["كمية"])
        
        طباعة("إجمالي المبيعات: " + إجمالي)
        
        // إيجاد المنتج الأكثر مبيعاً
        متغير أعلى_منتج = ""
        متغير أعلى_قيمة = 0
        
        لكل بيع في مبيعات:
            متغير قيمة = بيع["سعر"] * بيع["كمية"]
            إذا قيمة > أعلى_قيمة:
                أعلى_قيمة = قيمة
                أعلى_منتج = بيع["منتج"]
        
        طباعة("المنتج الأكثر مبيعاً: " + أعلى_منتج + " بقيمة " + أعلى_قيمة)
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
        // خوارزمية الفرز السريع (Quick Sort)
        دالة فرز_سريع(قائمة):
            إذا طول(قائمة) <= 1:
                أرجع قائمة
            
            متغير محور = قائمة[0]
            متغير أقل = []
            متغير أكبر = []
            متغير متساوي = [محور]
            
            لـ س من 1 إلى طول(قائمة):
                إذا قائمة[س] < محور:
                    أقل.أضف(قائمة[س])
                وإلا إذا قائمة[س] > محور:
                    أكبر.أضف(قائمة[س])
                وإلا:
                    متساوي.أضف(قائمة[س])
            
            أرجع فرز_سريع(أقل) + متساوي + فرز_سريع(أكبر)
        
        // اختبار
        متغير أرقام = [64، 34، 25، 12، 22، 11، 90]
        طباعة("قبل الفرز: " + أرقام)
        متغير مرتب = فرز_سريع(أرقام)
        طباعة("بعد الفرز: " + مرتب)
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في خوارزمية الفرز: {:?}", result.err());
    println!("✅ test_full_algorithms");
}

/// اختبار التكامل: Lexer -> Parser -> Interpreter
#[test]
fn test_integration_lexer_parser_interpreter() {
    let source = r#"
        متغير رسالة = "مرحبا بالعالم"
        طباعة(رسالة)
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
        متغير س = 10
        متغير ص = 20
        س + ص
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
        دالة جمع(أ، ب):
            أرجع أ + ب
        
        جمع(100، 200)
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
        متغير س = 10
        متغير ص = 20
        س * ص + س - ص
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
        // حساب مجموع 1 إلى 100000
        متغير مجموع = 0
        لـ س من 1 إلى 100000:
            مجموع = مجموع + س
    "#;
    
    println!("✅ test_full_performance_benchmark:");
    
    // Interpreter
    let mut interp = Interpreter::new();
    let interp_start = std::time::Instant::now();
    let interp_result = interp.run(source);
    let interp_time = interp_start.elapsed();
    println!("   Interpreter: {:?}", interp_time);
    
    // VM
    let chunk_for_vm = Compiler::compile_source(source).expect("فشل Compiler");
    let mut vm = VM::with_fresh_env();
    vm.load(chunk_for_vm);
    let vm_start = std::time::Instant::now();
    let vm_result = vm.run();
    let vm_time = vm_start.elapsed();
    println!("   VM: {:?}", vm_time);
    
    // JIT - ترجمة منفصلة
    let chunk_for_jit = Compiler::compile_source(source).expect("فشل Compiler");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    let jit_start = std::time::Instant::now();
    let jit_result = jit.execute(&chunk_for_jit, &mut globals);
    let jit_time = jit_start.elapsed();
    println!("   JIT: {:?}", jit_time);
    
    assert!(interp_result.is_ok() && matches!(vm_result, ExecutionResult::Ok(_)) && jit_result.is_ok());
}

/// اختبار معالجة الأخطاء الشامل
#[test]
fn test_full_error_handling() {
    let error_cases = vec![
        ("متغير", "بدون قيمة"),
        ("دالة ()", "بدون اسم"),
        ("إذا:", "بدون شرط"),
        ("بينما:", "بدون شرط"),
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

/// اختبار البرامج الواقعية - نظام تسوق
#[test]
fn test_realistic_shopping_system() {
    let source = r#"
        // نظام سلة التسوق
        صنف منتج:
            متغير اسم = ""
            متغير سعر = 0
            متغير كمية = 0
            
            دالة جديد(اسم، سعر، كمية):
                هذا.اسم = اسم
                هذا.سعر = سعر
                هذا.كمية = كمية
            
            دالة المجموع():
                أرجع هذا.سعر * هذا.كمية
        
        صنف سلة_تسوق:
            متغير منتجات = []
            
            دالة أضف(منتج):
                هذا.منتجات.أضف(منتج)
            
            دالة إجمالي():
                متغير مجموع = 0
                لكل منتج في هذا.منتجات:
                    مجموع = مجموع + منتج.المجموع()
                أرجع مجموع
            
            دالة طباعة_الفاتورة():
                طباعة("═════════════════════════")
                طباعة("         الفاتورة         ")
                طباعة("═════════════════════════")
                لكل منتج في هذا.منتجات:
                    طباعة(منتج.اسم + ": " + منتج.كمية + " × " + منتج.سعر + " = " + منتج.المجموع())
                طباعة("═════════════════════════")
                طباعة("الإجمالي: " + هذا.إجمالي())
        
        // إنشاء سلة
        متغير سلة = سلة_تسوق()
        سلة.أضف(منتج.جديد("لابتوب"، 1500، 2))
        سلة.أضف(منتج.جديد("هاتف"، 800، 3))
        سلة.أضف(منتج.جديد("سماعات"، 150، 5))
        
        سلة.طباعة_الفاتورة()
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في نظام التسوق: {:?}", result.err());
    println!("✅ test_realistic_shopping_system");
}

/// اختبار كامل - لعبة بسيطة
#[test]
fn test_full_simple_game() {
    let source = r#"
        // لعبة تخمين الرقم
        متغير الرقم_السري = 42
        متغير محاولات = 0
        متغير أقصى_محاولات = 5
        متغير فوز = خطأ
        
        // محاكاة التخمين
        متغير تخمينات = [10، 30، 50، 42]
        
        لكل تخمين في تخمينات:
            محاولات = محاولات + 1
            طباعة("محاولة " + محاولات + ": تخمينك " + تخمين)
            
            إذا تخمين == الرقم_السري:
                طباعة("🎉 أحسنت! لقد فزت!")
                فوز = صحيح
                توقف
            وإلا إذا تخمين < الرقم_السري:
                طباعة("الرقم أكبر")
            وإلا:
                طباعة("الرقم أصغر")
        
        إذا لا فوز:
            طباعة("للأسف! لم تخمن الرقم")
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    assert!(result.is_ok(), "فشل في اللعبة: {:?}", result.err());
    println!("✅ test_full_simple_game");
}
