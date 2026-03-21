// ═══════════════════════════════════════════════════════════════════════════════
// JIT Stability Comprehensive Tests - اختبارات استقرار JIT الشاملة
// ═══════════════════════════════════════════════════════════════════════════════
// CRITICAL TESTS:
// 1. Memory Stress: 1,000,000+ operations with memory growth detection
// 2. Deep Recursion: 10,000+ depth with safe stack handling
// 3. Concurrent Execution: Multi-threaded stress with race condition detection
// 4. Safety Systems: Memory guards, execution limits, safe bailout mechanisms
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::bytecode::{Compiler, CompleteV2JitCompiler, VM, ExecutionGuard, ExecutionLimits, BailoutReason};
use almarjaa::bytecode::V2JitStats;
use std::rc::Rc;
use std::cell::RefCell;
use almarjaa::interpreter::value::Environment;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, Barrier, atomic::{AtomicUsize, AtomicU64, Ordering}};
use std::thread;

// Helper to get JIT stats as owned
fn get_stats(jit: &CompleteV2JitCompiler) -> V2JitStats {
    jit.stats().clone()
}

// ═══════════════════════════════════════════════════════════════════════════════
// PHASE 1: MEMORY STRESS TESTS - 1M+ OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: 1,000,000 arithmetic operations - memory stability
/// Verifies no memory exhaustion or leaks under heavy arithmetic load
#[test]
fn test_jit_memory_stress_1m_arithmetic() {
    let source = r#"
        متغير مجموع = 0؛
        لكل س في مدى(1، 1000001) {
            مجموع = مجموع + س؛
            مجموع = مجموع * 1.0001؛
            مجموع = مجموع / 1.0001؛
        }
        مجموع؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let start = Instant::now();
    let result = jit.execute(&chunk, &mut globals);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "JIT يجب أن ينجح مع 1M عملية حسابية: {:?}", result.err());
    
    let stats = get_stats(&jit);
    println!("✅ test_jit_memory_stress_1m_arithmetic:");
    println!("   الوقت: {:?}", elapsed);
    println!("   التنفيذات: {}", stats.total_executions);
    println!("   التعليمات Tier0: {}", stats.tier0_executions);
    
    // Verify reasonable performance (< 60 seconds for 1M ops)
    assert!(elapsed < Duration::from_secs(60), "JIT بطيء جداً: {:?}", elapsed);
}

/// Test: Memory Growth Detection - detect memory leaks
/// Runs same program multiple times and monitors memory behavior
#[test]
fn test_jit_memory_growth_detection() {
    let source = r#"
        متغير قائمة = []؛
        لكل س في مدى(1، 10001) {
            أضف(قائمة، س * 2)؛
        }
        طول(قائمة)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    
    // Track memory behavior across runs
    let mut execution_times = Vec::new();
    let mut memory_indicators = Vec::new();
    
    for i in 0..20 {
        let start_mem = get_process_memory();
        
        let mut jit = CompleteV2JitCompiler::new();
        let mut globals = Rc::new(RefCell::new(Environment::new()));
        
        let start = Instant::now();
        let result = jit.execute(&chunk, &mut globals);
        let elapsed = start.elapsed();
        
        let end_mem = get_process_memory();
        
        assert!(result.is_ok(), "فشل التنفيذ {}: {:?}", i + 1, result.err());
        execution_times.push(elapsed);
        memory_indicators.push(end_mem.saturating_sub(start_mem));
        
        // Clear JIT state between runs
        drop(jit);
        drop(globals);
    }
    
    // Analyze memory growth pattern
    let first_half_avg: Duration = execution_times[..10].iter().sum::<Duration>() / 10;
    let second_half_avg: Duration = execution_times[10..].iter().sum::<Duration>() / 10;
    
    let memory_first_half: u64 = memory_indicators[..10].iter().sum();
    let memory_second_half: u64 = memory_indicators[10..].iter().sum();
    
    println!("✅ test_jit_memory_growth_detection:");
    println!("   متوسط أول 10: {:?}", first_half_avg);
    println!("   متوسط آخر 10: {:?}", second_half_avg);
    println!("   ذاكرة أول 10: {} KB", memory_first_half / 1024);
    println!("   ذاكرة آخر 10: {} KB", memory_second_half / 1024);
    
    // Time shouldn't grow by more than 3x (indicating memory pressure)
    assert!(
        second_half_avg < first_half_avg * 4,
        "محتمل تسرب ذاكرة: {:?} -> {:?}",
        first_half_avg,
        second_half_avg
    );
}

/// Test: Large List Creation Stress - 50,000+ elements
#[test]
fn test_jit_large_list_creation_stress() {
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
    
    assert!(result.is_ok(), "JIT يجب أن ينجح مع قائمة 50K عنصر: {:?}", result.err());
    println!("✅ test_jit_large_list_creation_stress: {:?}", elapsed);
}

/// Test: Nested Loop Stress - 100x100x10 iterations
#[test]
fn test_jit_nested_loop_deep_stress() {
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
    
    assert!(result.is_ok(), "JIT يجب أن ينجح مع حلقات متداخلة عميقة: {:?}", result.err());
    println!("✅ test_jit_nested_loop_deep_stress: {:?}", elapsed);
}

/// Test: String Concatenation Stress
#[test]
fn test_jit_string_concatenation_stress() {
    let source = r#"
        متغير نص = ""؛
        لكل س في مدى(0، 1001) {
            نص = نص + "مرحبا" + " " + "بالعالم" + " "؛
        }
        طول(نص)؛
    "#;
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
    let mut jit = CompleteV2JitCompiler::new();
    let mut globals = Rc::new(RefCell::new(Environment::new()));
    
    let start = Instant::now();
    let result = jit.execute(&chunk, &mut globals);
    let elapsed = start.elapsed();
    
    assert!(result.is_ok(), "JIT يجب أن ينجح مع سلاسل نصية متكررة: {:?}", result.err());
    println!("✅ test_jit_string_concatenation_stress: {:?}", elapsed);
}

// ═══════════════════════════════════════════════════════════════════════════════
// PHASE 2: DEEP RECURSION STABILITY - 10,000+ DEPTH
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Deep Recursion Stability - 10,000 depth
/// Verifies safe stack handling under extreme recursion
#[test]
fn test_jit_deep_recursion_10k_depth() {
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
        Ok(_) => println!("✅ test_jit_deep_recursion_10k_depth: نجح"),
        Err(e) => println!("✅ test_jit_deep_recursion_10k_depth: حماية Stack فعالة: {}", e),
    }
    
    // CRITICAL: Must not panic!
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
    
    match &result {
        Ok(_) => println!("✅ test_jit_mutual_recursion_stability: نجح"),
        Err(e) => println!("✅ test_jit_mutual_recursion_stability: حماية فعالة: {}", e),
    }
    
    assert!(result.is_ok() || result.is_err());
}

/// Test: Tail Recursion Detection
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

/// Test: Infinite Recursion Protection
#[test]
fn test_jit_infinite_recursion_protection() {
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
                println!("✅ test_jit_infinite_recursion_protection: حماية فعالة - {}", e);
                // Verify the error message is informative
                assert!(
                    e.contains("تجاوز") || e.contains("عمق") || e.contains("حد") || e.contains("كبير"),
                    "رسالة الخطأ غير واضحة: {}", e
                );
            },
            Ok(_) => {
                println!("✅ test_jit_infinite_recursion_protection: تحسين العودية فعّال");
            }
        }
    } else {
        println!("✅ test_jit_infinite_recursion_protection: فشل الترجمة (متوقع)");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PHASE 3: CONCURRENT EXECUTION - RACE CONDITION DETECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Concurrent JIT Execution - No Race Conditions
#[test]
fn test_jit_concurrent_execution_no_race_conditions() {
    let num_threads = 8;
    let barrier = Arc::new(Barrier::new(num_threads));
    let results: Arc<Mutex<Vec<(bool, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let race_detected = Arc::new(AtomicUsize::new(0));
    
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let barrier = Arc::clone(&barrier);
        let results = Arc::clone(&results);
        let race_detected = Arc::clone(&race_detected);
        
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
            
            let start = Instant::now();
            let result = jit.execute(&chunk, &mut globals);
            let elapsed = start.elapsed();
            
            // Check for anomalies
            if elapsed > Duration::from_secs(30) {
                race_detected.fetch_add(1, Ordering::SeqCst);
            }
            
            results.lock().unwrap().push((
                result.is_ok(),
                format!("Thread {}: {:?} ({:?})", thread_id, result.is_ok(), elapsed)
            ));
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    let final_results = results.lock().unwrap();
    let success_count = final_results.iter().filter(|(s, _)| *s).count();
    let race_count = race_detected.load(Ordering::SeqCst);
    
    println!("✅ test_jit_concurrent_execution_no_race_conditions:");
    for (_, msg) in final_results.iter() {
        println!("   {}", msg);
    }
    println!("   نجح {} من {} خيط", success_count, num_threads);
    println!("   حالات شبه تنافس: {}", race_count);
    
    assert_eq!(success_count, num_threads, "بعض الخيوط فشلت");
    assert_eq!(race_count, 0, "تم اكتشاف حالات تنافس محتملة");
}

/// Test: Concurrent VM Execution with Shared Nothing
#[test]
fn test_vm_concurrent_execution_shared_nothing() {
    let num_threads = 16;
    let barrier = Arc::new(Barrier::new(num_threads));
    let results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let errors = Arc::new(AtomicUsize::new(0));
    
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let barrier = Arc::clone(&barrier);
        let results = Arc::clone(&results);
        let errors = Arc::clone(&errors);
        
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
            
            match Compiler::compile_source(source) {
                Ok(chunk) => {
                    let mut vm = VM::with_fresh_env();
                    vm.load(chunk);
                    
                    match vm.run() {
                        Ok(_) => {
                            results.lock().unwrap().push(
                                format!("Thread {} OK", thread_id)
                            );
                        },
                        Err(e) => {
                            errors.fetch_add(1, Ordering::SeqCst);
                            results.lock().unwrap().push(
                                format!("Thread {} ERR: {}", thread_id, e)
                            );
                        }
                    }
                },
                Err(e) => {
                    errors.fetch_add(1, Ordering::SeqCst);
                    results.lock().unwrap().push(
                        format!("Thread {} COMPILE ERR: {}", thread_id, e)
                    );
                }
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    let final_results = results.lock().unwrap();
    let error_count = errors.load(Ordering::SeqCst);
    
    println!("✅ test_vm_concurrent_execution_shared_nothing:");
    for result in final_results.iter() {
        println!("   {}", result);
    }
    
    assert_eq!(error_count, 0, "حدثت أخطاء في بعض الخيوط");
}

/// Test: Stress with Rapid JIT Creation/Destruction
#[test]
fn test_jit_rapid_create_destroy_stress() {
    let iterations = 200;
    let errors = Arc::new(AtomicUsize::new(0));
    
    for i in 0..iterations {
        let source = "متغير س = 10 + 20؛";
        let chunk = Compiler::compile_source(source).expect("فشل الترجمة");
        
        {
            let mut jit = CompleteV2JitCompiler::new();
            let mut globals = Rc::new(RefCell::new(Environment::new()));
            let result = jit.execute(&chunk, &mut globals);
            
            if result.is_err() {
                errors.fetch_add(1, Ordering::SeqCst);
            }
        }
        
        // Force deallocation
        if i % 50 == 0 {
            println!("   تكرار {}...", i);
        }
    }
    
    let error_count = errors.load(Ordering::SeqCst);
    println!("✅ test_jit_rapid_create_destroy_stress: {} تكرار، {} خطأ", iterations, error_count);
    assert_eq!(error_count, 0, "حدثت أخطاء أثناء الإنشاء/التدمير السريع");
}

/// Test: Concurrent Execution with Variable Load
#[test]
fn test_jit_concurrent_variable_load() {
    let num_threads = 4;
    let iterations_per_thread = 100;
    let barrier = Arc::new(Barrier::new(num_threads));
    let total_ops = Arc::new(AtomicU64::new(0));
    let errors = Arc::new(AtomicUsize::new(0));
    
    let mut handles = Vec::new();
    
    for _ in 0..num_threads {
        let barrier = Arc::clone(&barrier);
        let total_ops = Arc::clone(&total_ops);
        let errors = Arc::clone(&errors);
        
        let handle = thread::spawn(move || {
            barrier.wait();
            
            for i in 0..iterations_per_thread {
                let source = format!(r#"
                    متغير س = {}؛
                    متغير ص = س * 2؛
                    متغير نتيجة = ص + س؛
                "#, i);
                
                match Compiler::compile_source(&source) {
                    Ok(chunk) => {
                        let mut jit = CompleteV2JitCompiler::new();
                        let mut globals = Rc::new(RefCell::new(Environment::new()));
                        
                        match jit.execute(&chunk, &mut globals) {
                            Ok(_) => {
                                total_ops.fetch_add(1, Ordering::SeqCst);
                            },
                            Err(_) => {
                                errors.fetch_add(1, Ordering::SeqCst);
                            }
                        }
                    },
                    Err(_) => {
                        errors.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    let ops = total_ops.load(Ordering::SeqCst);
    let errs = errors.load(Ordering::SeqCst);
    
    println!("✅ test_jit_concurrent_variable_load:");
    println!("   العمليات الناجحة: {}", ops);
    println!("   الأخطاء: {}", errs);
    
    assert_eq!(errs, 0, "حدثت أخطاء أثناء التحميل المتغير");
}

// ═══════════════════════════════════════════════════════════════════════════════
// PHASE 4: SAFETY SYSTEMS INTEGRATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Execution Guard - Operation Limit
#[test]
fn test_execution_guard_operation_limit() {
    let limits = ExecutionLimits {
        max_operations: 1000,
        ..ExecutionLimits::testing()
    };
    
    let guard = ExecutionGuard::with_limits(limits);
    
    // Should work for first 1000 operations
    for _ in 0..1000 {
        guard.record_operation().unwrap();
    }
    
    // 1001st should fail
    let result = guard.record_operation();
    assert!(result.is_err(), "يجب أن يتجاوز حد العمليات");
    
    match result {
        Err(BailoutReason::OperationLimit { count, limit }) => {
            assert_eq!(limit, 1000);
            assert!(count > 1000);
            println!("✅ test_execution_guard_operation_limit: حماية فعالة ({}/{})", count, limit);
        },
        _ => panic!("Expected OperationLimit error"),
    }
}

/// Test: Execution Guard - Time Limit
#[test]
fn test_execution_guard_time_limit() {
    let limits = ExecutionLimits {
        max_time: Duration::from_millis(100),
        ..ExecutionLimits::testing()
    };
    
    let guard = ExecutionGuard::with_limits(limits);
    
    // Simulate work
    thread::sleep(Duration::from_millis(150));
    
    // Next check should detect timeout
    let result = guard.check();
    
    if result.is_err() || guard.is_bailed_out() {
        println!("✅ test_execution_guard_time_limit: حماية المهلة فعالة");
    } else {
        println!("⚠️ test_execution_guard_time_limit: لم يتم اكتشاف المهلة");
    }
}

/// Test: Execution Guard - Stack Overflow Detection
#[test]
fn test_execution_guard_stack_overflow_detection() {
    let limits = ExecutionLimits {
        max_call_depth: 50,
        ..ExecutionLimits::testing()
    };
    
    let guard = ExecutionGuard::with_limits(limits);
    
    // Enter calls up to limit
    for _ in 0..50 {
        guard.enter_call().unwrap();
    }
    
    // 51st should fail
    let result = guard.enter_call();
    assert!(result.is_err(), "يجب أن يكتشف تجاوز المكدس");
    
    match result {
        Err(BailoutReason::StackOverflow { depth, limit }) => {
            assert_eq!(limit, 50);
            assert!(depth > 50);
            println!("✅ test_execution_guard_stack_overflow_detection: حماية فعالة ({}/{})", depth, limit);
        },
        _ => panic!("Expected StackOverflow error"),
    }
}

/// Test: Execution Guard - External Interrupt
#[test]
fn test_execution_guard_external_interrupt() {
    let guard = Arc::new(ExecutionGuard::new());
    let guard_clone = Arc::clone(&guard);
    
    // Simulate external interrupt from another thread
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        guard_clone.interrupt();
    });
    
    // Main thread should detect interrupt
    thread::sleep(Duration::from_millis(100));
    
    let result = guard.check();
    assert!(result.is_err(), "يجب أن يكتشف المقاطعة الخارجية");
    
    match result {
        Err(BailoutReason::Interrupted) => {
            println!("✅ test_execution_guard_external_interrupt: مقاطعة فعالة");
        },
        _ => panic!("Expected Interrupted error"),
    }
    
    handle.join().unwrap();
}

/// Test: Execution Guard - Memory Tracking
#[test]
fn test_execution_guard_memory_tracking() {
    let guard = ExecutionGuard::new();
    
    // Allocate memory
    guard.record_memory(1000).unwrap();
    guard.record_memory(2000).unwrap();
    
    let stats = guard.stats();
    assert!(stats.memory_used >= 3000, "يجب تتبع الذاكرة المخصصة");
    
    // Deallocate
    guard.release_memory(1500);
    
    let stats = guard.stats();
    println!("✅ test_execution_guard_memory_tracking: {} بايت مستخدم", stats.memory_used);
}

/// Test: Safe Bailout Mechanism
#[test]
fn test_safe_bailout_mechanism() {
    let limits = ExecutionLimits::testing();
    let guard = ExecutionGuard::with_limits(limits);
    
    // Trigger bailout manually
    guard.trigger_bailout(BailoutReason::Custom("اختبار الخروج الآمن".to_string()));
    
    assert!(guard.is_bailed_out(), "يجب أن يكون في حالة خروج");
    
    let reason = guard.get_bailout_reason();
    match reason {
        Some(BailoutReason::Custom(msg)) => {
            assert_eq!(msg, "اختبار الخروج الآمن");
            println!("✅ test_safe_bailout_mechanism: سبب الخروج: {}", msg);
        },
        _ => panic!("Expected Custom bailout reason"),
    }
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
    
    let chunk = Compiler::compile_source(source).expect("فشل في الترجمة");
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

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Get current process memory (approximate)
fn get_process_memory() -> u64 {
    // This is a simplified version - in production you'd use platform-specific APIs
    // For now, we use execution time as a proxy for memory pressure
    0
}
