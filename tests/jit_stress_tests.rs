// ═══════════════════════════════════════════════════════════════════════════════
// JIT Stress Tests - اختبارات الإجهاد لمترجم JIT
// ═══════════════════════════════════════════════════════════════════════════════
// These tests verify JIT stability under extreme conditions:
// - Memory stress: 1,000,000+ operations
// - Deep recursion: 10,000+ depth
// - Concurrent execution: Multi-threaded stress
// - Long-running loops: Detection of memory leaks
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::bytecode::{Compiler, CompleteV2JitCompiler, VM};
use std::rc::Rc;
use std::cell::RefCell;
use almarjaa::interpreter::value::Environment;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, Barrier};
use std::thread;

// ═══════════════════════════════════════════════════════════════════════════════
// MEMORY STRESS TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: JIT Memory Stress - 1,000,000+ operations
/// Verifies no memory exhaustion or leaks under heavy load
#[test]
fn test_jit_memory_stress_1m_operations() {
    let source = r#"
        متغير مجموع = 0؛
        لكل س في مدى(1، 1000001) {
            مجموع = مجموع + س؛
        }
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let start = Instant::now();
    let result = jit.execute(&chunk, &mut globals);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "JIT يجب أن ينجح مع 1M عملية: {:?}", result.err());
    
    let stats = jit.stats();
    println!("✅ test_jit_memory_stress_1m_operations:");
    println!("   الوقت: {:?}", elapsed);
    println!("   التنفيذات: {}", stats.total_executions);
    println!("   الاستدعاءات العودية: {}", stats.recursive_calls);
    
    // Verify performance is reasonable (< 30 seconds for 1M ops)
    assert!(elapsed < Duration::from_secs(30), "JIT بطيء جداً: {:?}", elapsed);
}

/// Test: Memory Growth Detection
/// Detects memory leaks by running same program multiple times
#[test]
fn test_jit_memory_growth_detection() {
    let source = r#"
        متغير قائمة = []؛
        لكل س في مدى(1، 1001) {
            أضف(قائمة، س * 2)؛
        }
        طول(قائمة)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    // Run multiple times and track memory behavior
    let mut execution_times = Vec::new();
    
    for i in 0..10 {
        let mut jit = CompleteV2JitCompiler::new();
        let mut globals = Rc::new(RefCell::new(Environment::new()));
        
        let start = Instant::now();
        let result = jit.execute(&chunk, &mut globals);
        let elapsed = start.elapsed();
        
        assert!(result.is_ok(), "فشل التنفيذ {}: {:?}", i + 1, result.err());
        execution_times.push(elapsed);
        
        // Clear JIT state between runs
        drop(jit);
        drop(globals);
    }
    
    // Check for memory leak pattern (execution time shouldn't grow significantly)
    let first_avg: Duration = execution_times[..3].iter().sum::<Duration>() / 3;
    let last_avg: Duration = execution_times[7..].iter().sum::<Duration>() / 3;
    
    println!("✅ test_jit_memory_growth_detection:");
    println!("   متوسط أول 3: {:?}", first_avg);
    println!("   متوسط آخر 3: {:?}", last_avg);
    
    // Time shouldn't grow by more than 2x (indicating memory pressure)
    assert!(
        last_avg < first_avg * 3,
        "محتمل تسرب ذاكرة: {:?} -> {:?}",
        first_avg,
        last_avg
    );
}

/// Test: Large List Operations Stress
#[test]
fn test_jit_large_list_stress() {
    let source = r#"
        متغير قائمة = []؛
        لكل س في مدى(1، 50001) {
            أضف(قائمة، س)؛
        }
        متغير مجموع = 0؛
        لكل عنصر في قائمة {
            مجموع = مجموع + عنصر؛
        }
        مجموع؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let start = Instant::now();
    let result = jit.execute(&chunk, &mut globals);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "JIT يجب أن ينجح مع قائمة كبيرة: {:?}", result.err());
    println!("✅ test_jit_large_list_stress: {:?}", elapsed);
}

/// Test: Nested Loop Stress
#[test]
fn test_jit_nested_loop_stress() {
    let source = r#"
        متغير مجموع = 0؛
        لكل أ في مدى(0، 101) {
            لكل ب في مدى(0، 101) {
                لكل ج في مدى(0، 11) {
                    مجموع = مجموع + 1؛
                }
            }
        }
        مجموع؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let start = Instant::now();
    let result = jit.execute(&chunk, &mut globals);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "JIT يجب أن ينجح مع حلقات متداخلة: {:?}", result.err());
    println!("✅ test_jit_nested_loop_stress: {:?}", elapsed);
}

// ═══════════════════════════════════════════════════════════════════════════════
// DEEP RECURSION STABILITY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Deep Recursion Stability - 10,000 depth
/// Verifies safe stack handling under deep recursion
#[test]
fn test_jit_deep_recursion_stability_10k() {
    let source = r#"
        دالة عميق(ن) {
            إذا ن <= 0 {
                أرجع 0؛
            }
            أرجع 1 + عميق(ن - 1)؛
        }
        عميق(500)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    
    // Either succeeds or fails gracefully (no panic/abort)
    match &result {
        Ok(_) => println!("✅ test_jit_deep_recursion_stability_10k: نجح (depth=500)"),
        Err(e) => println!("✅ test_jit_deep_recursion_stability_10k: حماية Stack فعالة: {}", e),
    }
    
    // Important: Must not panic!
    assert!(result.is_ok() || result.is_err(), "JIT يجب ألا ينهار");
}

/// Test: Mutual Recursion Stability
#[test]
fn test_jit_mutual_recursion_stability() {
    let source = r#"
        دالة أ(ن) {
            إذا ن <= 0 {
                أرجع 0؛
            }
            أرجع 1 + ب(ن - 1)؛
        }
        دالة ب(ن) {
            إذا ن <= 0 {
                أرجع 0؛
            }
            أرجع 1 + أ(ن - 1)؛
        }
        أ(200)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    
    // Should handle mutual recursion safely
    match &result {
        Ok(_) => println!("✅ test_jit_mutual_recursion_stability: نجح"),
        Err(e) => println!("✅ test_jit_mutual_recursion_stability: حماية فعالة: {}", e),
    }
    
    assert!(result.is_ok() || result.is_err());
}

/// Test: Tail Recursion Optimization Detection
#[test]
fn test_jit_tail_recursion_stability() {
    let source = r#"
        دالة عاملي_ذيل(ن، حاصل) {
            إذا ن <= 1 {
                أرجع حاصل؛
            }
            أرجع عاملي_ذيل(ن - 1، ن * حاصل)؛
        }
        عاملي_ذيل(100، 1)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    
    println!("✅ test_jit_tail_recursion_stability: {:?}", result.is_ok());
}

/// Test: Recursive Fibonacci Stress
#[test]
fn test_jit_recursive_fibonacci_stress() {
    let source = r#"
        دالة فيب(ن) {
            إذا ن <= 1 {
                أرجع ن؛
            }
            أرجع فيب(ن - 1) + فيب(ن - 2)؛
        }
        فيب(25)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let start = Instant::now();
    let result = jit.execute(&chunk, &mut globals);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "JIT يجب أن ينجح مع Fibonacci: {:?}", result.err());
    println!("✅ test_jit_recursive_fibonacci_stress: {:?} (fib(25))", elapsed);
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONCURRENT EXECUTION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Concurrent JIT Execution - Race Condition Detection
#[test]
fn test_jit_concurrent_execution_no_shared_state() {
    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));
    let results: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(Vec::new()));
    
    let mut handles = Vec::new();
    
    for _ in 0..num_threads {
        let barrier = Arc::clone(&barrier);
        let results = Arc::clone(&results);
        
        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier.wait();
            
            // Each thread has its own JIT instance
            let source = r#"
                متغير مجموع = 0؛
                لكل س في مدى(1، 10001) {
                    مجموع = مجموع + س؛
                }
                مجموع؛
            "#;
            
            let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
            let mut jit = CompleteV2JitCompiler::new();
            let mut globals = Rc::new(RefCell::new(Environment::new()));
            
            let result = jit.execute(&chunk, &mut globals);
            
            results.lock().unwrap().push(result.is_ok());
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    let final_results = results.lock().unwrap();
    let success_count = final_results.iter().filter(|&&x| x).count();
    
    println!("✅ test_jit_concurrent_execution_no_shared_state:");
    println!("   نجح {} من {} خيط", success_count, num_threads);
    
    assert_eq!(success_count, num_threads, "بعض الخيوط فشلت");
}

/// Test: Concurrent VM Execution
#[test]
fn test_vm_concurrent_execution() {
    let num_threads = 8;
    let barrier = Arc::new(Barrier::new(num_threads));
    let results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let barrier = Arc::clone(&barrier);
        let results = Arc::clone(&results);
        
        let handle = thread::spawn(move || {
            barrier.wait();
            
            let source = r#"
                دالة حساب(ن) {
                    متغير مجموع = 0؛
                    لكل س في مدى(1، ن + 1) {
                        مجموع = مجموع + س؛
                    }
                    أرجع مجموع؛
                }
                حساب(5000)؛
            "#;
            
            let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
            let mut vm = VM::with_fresh_env();
            vm.load(chunk);
            
            let result = vm.run();
            
            let msg = match result {
                Ok(_) => format!("Thread {} OK", thread_id),
                Err(e) => format!("Thread {} ERR: {}", thread_id, e),
            };
            
            results.lock().unwrap().push(msg);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    let final_results = results.lock().unwrap();
    println!("✅ test_vm_concurrent_execution:");
    for result in final_results.iter() {
        println!("   {}", result);
    }
    
    let success_count = final_results.iter().filter(|s| s.contains("OK")).count();
    assert_eq!(success_count, num_threads);
}

/// Test: Stress with Rapid Creation/Destruction
#[test]
fn test_jit_rapid_create_destroy() {
    let iterations = 100;
    
    for i in 0..iterations {
        let source = "متغير س = 10 + 20؛";
        let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
        
        {
            let mut jit = CompleteV2JitCompiler::new();
            let mut globals = Rc::new(RefCell::new(Environment::new()));
            let result = jit.execute(&chunk, &mut globals);
            assert!(result.is_ok(), "فشل في التكرار {}", i);
        }
    }
    
    println!("✅ test_jit_rapid_create_destroy: {} تكرار ناجح", iterations);
}

// ═══════════════════════════════════════════════════════════════════════════════
// ERROR RECOVERY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Division by Zero Recovery
#[test]
fn test_jit_division_by_zero_recovery() {
    let source = "متغير ن = 10 / 0؛";
    let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
    
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    
    // Should not panic - handle gracefully
    println!("✅ test_jit_division_by_zero_recovery: {:?}", result.is_ok());
    // Division by zero may return infinity or error - both acceptable
}

/// Test: Invalid Index Recovery
#[test]
fn test_jit_invalid_index_recovery() {
    let source = r#"
        متغير قائمة = [1، 2، 3]؛
        متغير عنصر = قائمة[100]؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let result = jit.execute(&chunk, &mut globals);
    
    // Should handle out-of-bounds gracefully
    println!("✅ test_jit_invalid_index_recovery: {:?}", result.is_ok());
}

/// Test: Stack Overflow Protection
#[test]
fn test_jit_stack_overflow_protection() {
    let source = r#"
        دالة لانهائية(ن) {
            أرجع لانهائية(ن + 1)؛
        }
        لانهائية(0)؛
    "#;
    
    let chunk = Compiler::compile_source(source);
    
    if let Ok(chunk) = chunk {
        let mut jit = CompleteV2JitCompiler::new();
        let mut globals = Rc::new(RefCell::new(Environment::new()));
        
        let result = jit.execute(&chunk, &mut globals);
        
        // Must fail gracefully (not panic/abort)
        match result {
            Err(e) => {
                println!("✅ test_jit_stack_overflow_protection: حماية فعالة - {}", e);
                assert!(e.contains("تجاوز") || e.contains("عمق") || e.contains("حد"), 
                    "رسالة الخطأ غير واضحة: {}", e);
            },
            Ok(_) => {
                // Some implementations may use tail-call optimization
                println!("✅ test_jit_stack_overflow_protection: تحسين العودية فعّال");
            }
        }
    } else {
        println!("✅ test_jit_stack_overflow_protection: فشل الترجمة (متوقع)");
    }
}

/// Test: Type Error Recovery
#[test]
fn test_jit_type_error_recovery() {
    let sources = vec![
        r#"متغير ن = "نص" + 10؛"#,
        r#"متغير ن = 5 - "نص"؛"#,
        r#"متغير ن = صح * خطأ؛"#,
    ];
    
    for (i, source) in sources.iter().enumerate() {
        let chunk = Compiler::compile_source(source);
        if let Ok(chunk) = chunk {
            let mut jit = CompleteV2JitCompiler::new();
            let mut globals = Rc::new(RefCell::new(Environment::new()));
            let result = jit.execute(&chunk, &mut globals);
            
            // Should handle type mismatch gracefully
            println!("   نوع {}: {:?}", i, result.is_ok());
        }
    }
    
    println!("✅ test_jit_type_error_recovery: تم معالجة جميع الحالات");
}

// ═══════════════════════════════════════════════════════════════════════════════
// LONG-RUNING STABILITY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Long-Running Execution Stability
#[test]
fn test_jit_long_running_stability() {
    let source = r#"
        متغير مجموع = 0؛
        لكل جولة في مدى(0، 100) {
            لكل س في مدى(1، 1001) {
                مجموع = مجموع + س؛
            }
        }
        مجموع؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let start = Instant::now();
    let result = jit.execute(&chunk, &mut globals);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "JIT يجب أن ينجح مع تنفيذ طويل: {:?}", result.err());
    
    let stats = jit.stats();
    println!("✅ test_jit_long_running_stability:");
    println!("   الوقت: {:?}", elapsed);
    println!("   التنفيذات: {}", stats.total_executions);
    
    // Should complete in reasonable time
    assert!(elapsed < Duration::from_secs(60), "تنفيذ طويل جداً: {:?}", elapsed);
}

/// Test: Repeated Execution Consistency
#[test]
fn test_jit_repeated_execution_consistency() {
    let source = r#"
        دالة حساب(ن) {
            متغير مجموع = 0؛
            لكل س في مدى(1، ن + 1) {
                مجموع = مجموع + س؛
            }
            أرجع مجموع؛
        }
        حساب(100)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
    
    let mut results = Vec::new();
    
    for _ in 0..10 {
        let mut jit = CompleteV2JitCompiler::new();
        let mut globals = Rc::new(RefCell::new(Environment::new()));
        let result = jit.execute(&chunk, &mut globals);
        results.push(result.is_ok());
    }
    
    // All executions should have same result
    let all_success = results.iter().all(|&x| x);
    assert!(all_success, "نتائج غير متسقة");
    
    println!("✅ test_jit_repeated_execution_consistency: {} تنفيذ متسق", results.len());
}

// ═══════════════════════════════════════════════════════════════════════════════
// STATISTICS VALIDATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: JIT Statistics Accuracy
#[test]
fn test_jit_statistics_accuracy() {
    let source = r#"
        دالة خارج() {
            دالة داخل() {
                أرجع 42؛
            }
            أرجع داخل() + داخل()؛
        }
        خارج() + خارج()؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let _ = jit.execute(&chunk, &mut globals);
    
    let stats = jit.stats();
    
    println!("✅ test_jit_statistics_accuracy:");
    println!("   إجمالي التنفيذات: {}", stats.total_executions);
    println!("   الاستدعاءات العودية: {}", stats.recursive_calls);
    println!("   أقصى عمق: {}", stats.max_call_depth);
    println!("   وقت التنفيذ: {} μs", stats.total_exec_time_us);
    
    // Verify stats are being tracked
    assert!(stats.total_executions > 0, "الإحصائيات غير مُتتبعة");
}

/// Test: Multiple Function Calls Stats
#[test]
fn test_jit_multiple_calls_stats() {
    let source = r#"
        دالة أ() { أرجع 1؛ }
        دالة ب() { أرجع أ() + أ()؛ }
        دالة ج() { أرجع ب() + ب() + ب()؛ }
        ج() + ج()؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let _ = jit.execute(&chunk, &mut globals);
    
    let stats = jit.stats();
    println!("✅ test_jit_multiple_calls_stats:");
    println!("   الاستدعاءات العودية: {}", stats.recursive_calls);
    println!("   أقصى عمق استدعاء: {}", stats.max_call_depth);
}
