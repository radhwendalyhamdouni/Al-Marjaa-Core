// ═══════════════════════════════════════════════════════════════════════════════
// نظام الـ Bytecode - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// هذا المodule يوفر:
// - تعليمات Bytecode (opcodes)
// - مترجم من AST إلى Bytecode
// - آلة افتراضية سريعة مبنية على المكدس
// - JIT Compiler للكود الساخن
// - Tiered Compilation (مستويات متعددة من التحسين)
// - Tracing JIT (تتبع مسارات التنفيذ)
// - SIMD Operations (تعليمات المتجهات)
// - Threaded Code (تنفيذ متوازي)
// - Parallel Garbage Collector (جامع قمامة متوازي)
// - Type Inference (استنباط الأنواع)
// - PGO (Profile-Guided Optimization)
// - Async/Await JIT Support
// - WebAssembly Compilation Target
// - GC-JIT Integration
// - AOT (Ahead-of-Time) Compilation
// - Safety Systems (Memory guards, Execution limits)
// ═══════════════════════════════════════════════════════════════════════════════

pub mod advanced_jit;
pub mod aot_compiler;
pub mod async_jit;
pub mod benchmarks;
pub mod compiler;
pub mod complete_jit;
pub mod complete_v2_jit;
pub mod gc;
pub mod gc_jit_integration;
pub mod jit;
pub mod jit_benchmarks;
pub mod jit_optimizer;
pub mod opcodes;
pub mod optimizer;
pub mod pgo;
pub mod safety;
pub mod type_inference;
pub mod vm;
pub mod wasm_target;

// التحسينات الجديدة للإصدار 4.0
pub mod register_vm;  // Register-based VM (أسرع 2-3x)
pub mod ffi;         // Foreign Function Interface
pub mod simd;        // SIMD Operations (تسريع 4-8x)
pub mod jit_simd;    // JIT-SIMD Integration (تسريع تلقائي)

// إعادة تصدير الأنواع الرئيسية
pub use benchmarks::{print_benchmark_results, run_all_benchmarks, BenchmarkResult};
pub use compiler::{CompileResult, Compiler};
pub use jit::{CompiledCode, HotSpotInfo, JitCompiler, JitStats, OptimizedExecutor};
pub use opcodes::{Chunk, OpCode};
pub use vm::{ExecutionResult, VMStats, VM};

// تصدير JIT المتقدم
pub use advanced_jit::{
    AdvancedJitCompiler, AdvancedJitStats, CompiledTrace, SimdOperation, SimdProcessor, SimdStats,
    ThreadPool, ThreadedCodeExecutor, ThreadedStats, TierInfo, TierLevel, TierThresholds, Trace,
    TraceEntry, TraceState, TracingRecorder,
};

// تصدير جامع القمامة المتوازي
pub use gc::{
    GcObjectId, GcObjectInfo, GcStats, Generation, MemoryManager, ParallelGc, WriteBarrier,
};

// تصدير المُحسِّن
pub use optimizer::{
    OptimizationDetail, OptimizationKind, OptimizationLevel, OptimizationResult, Optimizer,
};

// تصدير JIT الكامل
pub use complete_jit::{
    CompiledCode as CompleteCompiledCode, CompleteJitCompiler,
    ExecutionResult as CompleteExecutionResult, HotSpotInfo as CompleteHotSpotInfo,
    JitStats as CompleteJitStats, TierLevel as CompleteTierLevel,
};

// تصدير اختبارات JIT
pub use jit_benchmarks::{
    compare_tiers, quick_jit_test, run_all_jit_benchmarks, BenchmarkResult as JitBenchmarkResult,
    BenchmarkSuite,
};

// تصدير Type Inference
pub use type_inference::{
    Type, TypeAnalysisResult, TypeError, TypeGuard, TypeGuardKind, TypeInferenceEngine,
    TypeWarning, VariableTypeInfo,
};

// تصدير PGO
pub use pgo::{
    BranchProfile, FunctionProfile, InstructionProfile, LoopProfile, OptimizationDecision,
    PgoOptimizationStats, PgoOptimizer, ProfilingManager,
};

// تصدير Async JIT
pub use async_jit::{
    AsyncJitCompiler, AsyncJitStats, AsyncResult, AsyncRuntime, AsyncState, AsyncStateMachine,
    AwaitPoint, AwaitType,
};

// تصدير WASM Target
pub use wasm_target::{
    WasiSupport, WasmCompiler, WasmCompilerStats, WasmFunction, WasmInstruction, WasmModule,
    WasmType,
};

// تصدير GC-JIT Integration
pub use gc_jit_integration::{
    AllocationManager, AllocationStats, GcJitCoordinator, Safepoint, SafepointManager,
    WriteBarrierGenerator, WriteBarrierType,
};

// تصدير AOT Compiler
pub use aot_compiler::{
    AotCompiler, AotCompilerStats, AotSettings, CompilationUnit, CompiledFunction, CompiledModule,
    OptimizationLevel as AotOptimizationLevel,
};

// تصدير JIT Optimizer (محسن 10x)
pub use jit_optimizer::{
    AllocationStrategy, AutoVectorizer, EscapeAnalyzer, InlineCache, JitOptimizer,
    LoopOptimizer, PGOOptimization, ProfileGuidedOptimizer, VectorizablePattern, VectorizedCode,
};

// تصدير JIT v2 الكامل مع دعم العودية و async/await
pub use complete_v2_jit::{
    AsyncTask, AsyncRuntime as V2AsyncRuntime, CallFrame, CompleteV2JitCompiler, CompiledFunction as V2CompiledFunction,
    CompiledInstruction, ExecutionResult as V2ExecutionResult, FunctionInfo, JitStats as V2JitStats,
    TaskStatus, TierLevel as V2TierLevel,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير Register VM (أسرع 2-3x من Stack VM)
// ═══════════════════════════════════════════════════════════════════════════════
pub use register_vm::{
    RegChunk, RegExecutionResult, RegOp, RegVMStats, RegisterVM,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير FFI (Foreign Function Interface)
// ═══════════════════════════════════════════════════════════════════════════════
pub use ffi::{
    CallbackId, CallbackManager, FfiFunction, FfiLibrary, FfiManager, FfiSignature, FfiStats,
    FfiType, FfiValue, NativeFunction,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير SIMD Operations
// ═══════════════════════════════════════════════════════════════════════════════
pub use simd::{
    SimdOp, SimdResult,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير JIT-SIMD Integration
// ═══════════════════════════════════════════════════════════════════════════════
pub use jit_simd::{
    SimdJitOp, SimdJitOptimizer, SimdJitResult, SimdJitStats,
    SimdOpInfo, SimdPattern, SimdPatternDetector,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير Safety Systems (أنظمة الأمان)
// ═══════════════════════════════════════════════════════════════════════════════
pub use safety::{
    BailoutReason, CallScopeGuard, ExecutionGuard, ExecutionLimits, ExecutionStats,
    MemoryStats, MemoryTracker,
};

// ═══════════════════════════════════════════════════════════════════════════════
// دوال سهلة الاستخدام
// ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::value::{Environment, Value};

/// تشغيل كود المرجع باستخدام الـ VM
pub fn run_bytecode(source: &str) -> Result<Value, String> {
    // ترجمة
    let chunk = Compiler::compile_source(source)?;

    // إنشاء VM مع البيئة الافتراضية
    let globals = Rc::new(RefCell::new(Environment::new()));

    // تعريف الدوال الأصلية
    define_native_functions(&globals);

    // تشغيل
    let mut vm = VM::new(globals);
    vm.load(chunk);

    match vm.run() {
        ExecutionResult::Ok(v) => Ok((*v.borrow()).clone()),
        ExecutionResult::Error(e) => Err(e),
        ExecutionResult::Return(v) => Ok((*v.borrow()).clone()),
        _ => Ok(Value::Null),
    }
}

/// تشغيل كود مع قياس الأداء
pub fn run_bytecode_benchmark(source: &str) -> Result<(Value, VMStats), String> {
    let chunk = Compiler::compile_source(source)?;
    let globals = Rc::new(RefCell::new(Environment::new()));
    define_native_functions(&globals);

    let mut vm = VM::new(globals);
    vm.load(chunk);

    let result = vm.run();
    let stats = vm.stats().clone();

    match result {
        ExecutionResult::Ok(v) => Ok(((*v.borrow()).clone(), stats)),
        ExecutionResult::Error(e) => Err(e),
        ExecutionResult::Return(v) => Ok(((*v.borrow()).clone(), stats)),
        _ => Ok((Value::Null, stats)),
    }
}

/// تعريف الدوال الأصلية الأساسية
fn define_native_functions(env: &Rc<RefCell<Environment>>) {
    // دوال أساسية
    env.borrow_mut().define(
        "اطبع",
        Value::NativeFunction {
            name: "اطبع".to_string(),
            func: |args| {
                for arg in args {
                    print!("{} ", arg.borrow().to_string_value());
                }
                println!();
                Ok(Rc::new(RefCell::new(Value::Null)))
            },
        },
        false,
    );

    env.borrow_mut().define(
        "نص",
        Value::NativeFunction {
            name: "نص".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Ok(Rc::new(RefCell::new(Value::String(String::new()))));
                }
                Ok(Rc::new(RefCell::new(Value::String(
                    args[0].borrow().to_string_value(),
                ))))
            },
        },
        false,
    );

    env.borrow_mut().define(
        "رقم",
        Value::NativeFunction {
            name: "رقم".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Ok(Rc::new(RefCell::new(Value::Number(0.0))));
                }
                match args[0].borrow().to_number() {
                    Ok(n) => Ok(Rc::new(RefCell::new(Value::Number(n)))),
                    Err(e) => Err(e),
                }
            },
        },
        false,
    );

    env.borrow_mut().define(
        "طول",
        Value::NativeFunction {
            name: "طول".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Ok(Rc::new(RefCell::new(Value::Number(0.0))));
                }
                let len = match &*args[0].borrow() {
                    Value::List(l) => l.len(),
                    Value::String(s) => s.chars().count(),
                    Value::Dictionary(d) => d.len(),
                    _ => 0,
                };
                Ok(Rc::new(RefCell::new(Value::Number(len as f64))))
            },
        },
        false,
    );

    env.borrow_mut().define(
        "نوع",
        Value::NativeFunction {
            name: "نوع".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Ok(Rc::new(RefCell::new(Value::String("لا_شيء".into()))));
                }
                Ok(Rc::new(RefCell::new(Value::String(
                    args[0].borrow().type_name().to_string(),
                ))))
            },
        },
        false,
    );

    env.borrow_mut().define(
        "نطاق",
        Value::NativeFunction {
            name: "نطاق".to_string(),
            func: |args| {
                let (start, end, step) = match args.len() {
                    1 => (0.0, args[0].borrow().to_number().unwrap_or(0.0), 1.0),
                    2 => (
                        args[0].borrow().to_number().unwrap_or(0.0),
                        args[1].borrow().to_number().unwrap_or(0.0),
                        1.0,
                    ),
                    _ => (
                        args.first()
                            .map(|a| a.borrow().to_number().unwrap_or(0.0))
                            .unwrap_or(0.0),
                        args.get(1)
                            .map(|a| a.borrow().to_number().unwrap_or(0.0))
                            .unwrap_or(0.0),
                        args.get(2)
                            .map(|a| a.borrow().to_number().unwrap_or(1.0))
                            .unwrap_or(1.0),
                    ),
                };

                let mut list = Vec::new();
                let mut i = start;
                while if step > 0.0 { i < end } else { i > end } {
                    list.push(Rc::new(RefCell::new(Value::Number(i))));
                    i += step;
                }

                Ok(Rc::new(RefCell::new(Value::List(list))))
            },
        },
        false,
    );

    // دوال رياضية - كل واحدة على حدة
    env.borrow_mut().define(
        "جذر",
        Value::NativeFunction {
            name: "جذر".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Err("جذر يتطلب معاملاً واحداً".into());
                }
                let n = args[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.sqrt()))))
            },
        },
        false,
    );

    env.borrow_mut().define(
        "مطلق",
        Value::NativeFunction {
            name: "مطلق".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Err("مطلق يتطلب معاملاً واحداً".into());
                }
                let n = args[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.abs()))))
            },
        },
        false,
    );

    env.borrow_mut().define(
        "تقريب",
        Value::NativeFunction {
            name: "تقريب".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Err("تقريب يتطلب معاملاً واحداً".into());
                }
                let n = args[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.round()))))
            },
        },
        false,
    );

    env.borrow_mut().define(
        "طابق",
        Value::NativeFunction {
            name: "طابق".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Err("طابق يتطلب معاملاً واحداً".into());
                }
                let n = args[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.floor()))))
            },
        },
        false,
    );

    env.borrow_mut().define(
        "سقف",
        Value::NativeFunction {
            name: "سقف".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Err("سقف يتطلب معاملاً واحداً".into());
                }
                let n = args[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.ceil()))))
            },
        },
        false,
    );

    env.borrow_mut().define(
        "أس",
        Value::NativeFunction {
            name: "أس".to_string(),
            func: |args| {
                if args.len() < 2 {
                    return Err("أس يتطلب معاملين".into());
                }
                let base = args[0].borrow().to_number()?;
                let exp = args[1].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(base.powf(exp)))))
            },
        },
        false,
    );

    // ثوابت رياضية
    env.borrow_mut()
        .define("ط", Value::Number(std::f64::consts::PI), true);
    env.borrow_mut()
        .define("هـ", Value::Number(std::f64::consts::E), true);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات التكامل
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let result = run_bytecode(r#"اطبع("مرحباً بالعالم")؛"#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_arithmetic() {
        let result = run_bytecode(r#"اطبع(5 + 3 * 2)؛"#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_variables() {
        let result = run_bytecode(
            r#"
            متغير س = 10؛
            متغير ص = 20؛
            اطبع(س + ص)؛
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_performance_benchmark() {
        // تقليل التكرارات لتسريع CI
        let code = r#"
            متغير س = 0؛
            طالما س < 100 {
                س = س + 1؛
            }
        "#;

        let (result, stats) = run_bytecode_benchmark(code).unwrap();

        println!("═══════════════════════════════════");
        println!("📊 نتيجة اختبار الأداء");
        println!("═══════════════════════════════════");
        println!("📦 التعليمات المنفذة: {}", stats.instructions_executed);
        println!("⏱️ الوقت: {} ميكروثانية", stats.execution_time_us);
        println!("📊 أقصى حجم للمكدس: {}", stats.max_stack_size);

        if let Value::Number(n) = result {
            assert_eq!(n, 100.0);
        }
    }
}
