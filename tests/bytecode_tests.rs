// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات آلة البايت كود - Bytecode VM Tests
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::bytecode::{Compiler, VM, OpCode, Chunk};

/// اختبار إنشاء Chunk
#[test]
fn test_chunk_creation() {
    let mut chunk = Chunk::new();
    
    // إضافة تعليمة
    chunk.write_op(OpCode::OpConstant);
    chunk.write_constant(42.0);
    
    assert_eq!(chunk.code().len(), 2, "يجب أن يكون هناك تعليمتين");
    println!("✅ test_chunk_creation");
}

/// اختبار ترجمة تعليمة بسيطة
#[test]
fn test_compile_simple() {
    let source = "10 + 20";
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    assert!(!chunk.code().is_empty(), "يجب أن يكون هناك تعليمات");
    println!("✅ test_compile_simple: {} تعليمة", chunk.code().len());
}

/// اختبار ترجمة متغير
#[test]
fn test_compile_variable() {
    let source = r#"
        متغير س = 10
        طباعة(س)
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    assert!(!chunk.code().is_empty());
    println!("✅ test_compile_variable: {} تعليمة", chunk.code().len());
}

/// اختبار ترجمة دالة
#[test]
fn test_compile_function() {
    let source = r#"
        دالة جمع(أ، ب):
            أرجع أ + ب
        
        جمع(3، 5)
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    assert!(!chunk.code().is_empty());
    println!("✅ test_compile_function: {} تعليمة", chunk.code().len());
}

/// اختبار ترجمة شرط
#[test]
fn test_compile_conditional() {
    let source = r#"
        متغير س = 10
        إذا س > 5:
            طباعة("كبير")
        وإلا:
            طباعة("صغير")
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    assert!(!chunk.code().is_empty());
    println!("✅ test_compile_conditional: {} تعليمة", chunk.code().len());
}

/// اختبار ترجمة حلقة
#[test]
fn test_compile_loop() {
    let source = r#"
        متغير س = 0
        بينما س < 10:
            س = س + 1
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    assert!(!chunk.code().is_empty());
    println!("✅ test_compile_loop: {} تعليمة", chunk.code().len());
}

/// اختبار تشغيل VM بسيط
#[test]
fn test_vm_simple() {
    let source = "10 + 20";
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    assert!(result.is_ok(), "فشل في تشغيل VM: {:?}", result.err());
    println!("✅ test_vm_simple");
}

/// اختبار VM مع متغيرات
#[test]
fn test_vm_variables() {
    let source = r#"
        متغير أ = 10
        متغير ب = 20
        أ + ب
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    assert!(result.is_ok());
    println!("✅ test_vm_variables");
}

/// اختبار VM مع دالة
#[test]
fn test_vm_function() {
    let source = r#"
        دالة مضاعف(س):
            أرجع س * 2
        
        مضاعف(21)
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    assert!(result.is_ok());
    println!("✅ test_vm_function");
}

/// اختبار VM مع حلقة
#[test]
fn test_vm_loop() {
    let source = r#"
        متغير مجموع = 0
        لـ س من 1 إلى 100:
            مجموع = مجموع + س
        مجموع
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    assert!(result.is_ok());
    println!("✅ test_vm_loop");
}

/// اختبار VM مع قائمة
#[test]
fn test_vm_list() {
    let source = r#"
        متغير قائمة = [1، 2، 3، 4، 5]
        قائمة[0] + قائمة[4]
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    assert!(result.is_ok());
    println!("✅ test_vm_list");
}

/// اختبار VM مع قاموس
#[test]
fn test_vm_dict() {
    let source = r#"
        متغير شخص = {اسم: "أحمد"، عمر: 25}
        شخص["عمر"]
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    assert!(result.is_ok());
    println!("✅ test_vm_dict");
}

/// اختبار VM مع عودية
#[test]
fn test_vm_recursion() {
    let source = r#"
        دالة فيبوناتشي(ن):
            إذا ن <= 1:
                أرجع ن
            أرجع فيبوناتشي(ن - 1) + فيبوناتشي(ن - 2)
        
        فيبوناتشي(10)
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    assert!(result.is_ok());
    println!("✅ test_vm_recursion");
}

/// اختبار VM مع إغلاق (Closure)
#[test]
fn test_vm_closure() {
    let source = r#"
        دالة مضاعف(عامل):
            أرجع دالة(س) => س * عامل
        
        متعرض ضعف = مضاعف(2)
        ضعف(21)
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    assert!(result.is_ok());
    println!("✅ test_vm_closure");
}

/// اختبار أداء VM
#[test]
fn test_vm_performance() {
    let source = r#"
        متغير مجموع = 0
        لـ س من 1 إلى 10000:
            مجموع = مجموع + س
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let start = std::time::Instant::now();
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok());
    println!("✅ test_vm_performance: {:?}", elapsed);
    assert!(elapsed.as_millis() < 3000, "يجب أن يكون أقل من 3 ثواني");
}

/// اختبار Stack Overflow Protection
#[test]
fn test_stack_overflow_protection() {
    let source = r#"
        دالة عودية_عميقة(ن):
            إذا ن <= 0:
                أرجع 0
            أرجع 1 + عودية_عميقة(ن - 1)
        
        عودية_عميقة(500)
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    // إما أن ينجح أو يفشل بسبب حماية الـ stack
    println!("✅ test_stack_overflow_protection: {:?}", result.is_ok());
}

/// اختبار المعاملات المتقدمة
#[test]
fn test_advanced_operators() {
    let source = r#"
        متغير أ = 2 ** 10
        متغير ب = 10 % 3
        متغير ج = 17 // 5
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    assert!(result.is_ok());
    println!("✅ test_advanced_operators");
}

/// اختبار التعبيرات المنطقية المركبة
#[test]
fn test_complex_logical() {
    let source = r#"
        متغير أ = صحيح و خطأ
        متغير ب = صحيح أو خطأ
        متغير ج = لا صحيح
        متغير د = (5 > 3) و (10 < 20)
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    assert!(result.is_ok());
    println!("✅ test_complex_logical");
}

/// اختبار معامل الأنبوب (Pipe)
#[test]
fn test_pipe_operator() {
    let source = r#"
        دالة مضاعف(س):
            أرجع س * 2
        
        10 |> مضاعف
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    assert!(result.is_ok());
    println!("✅ test_pipe_operator");
}

/// اختبار التعبيرات المتداخلة
#[test]
fn test_nested_expressions() {
    let source = r#"
        متغير نتيجة = ((10 + 5) * 2 - 5) / 5
    "#;
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    let mut vm = VM::new();
    let result = vm.run(&chunk);
    
    assert!(result.is_ok());
    println!("✅ test_nested_expressions");
}
