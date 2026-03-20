// ═══════════════════════════════════════════════════════════════════════════════
// لغة المرجع - Al-Marjaa Language (النسخة الأساسية)
// ═══════════════════════════════════════════════════════════════════════════════
// © 2026 رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI
// جميع الحقوق محفوظة | All Rights Reserved
// ═══════════════════════════════════════════════════════════════════════════════
// لغة برمجة عربية أساسية مع JIT Compiler
// الإصدار 3.4.0
// المؤلف: رضوان دالي حمدوني
// البريد: almarjaa.project@hotmail.com
// ═══════════════════════════════════════════════════════════════════════════════
// تحذير: هذا المشروع محمي بموجب حقوق الملكية الفكرية.
// الاستخدام التجاري يتطلب إذناً كتابياً صريحاً من المؤلف.
// WARNING: This project is protected by intellectual property rights.
// Commercial use requires explicit written permission from the author.
// ═══════════════════════════════════════════════════════════════════════════════

// المكونات الأساسية
pub mod bytecode;
pub mod error;
pub mod formatter;
pub mod interpreter;
pub mod lexer;
pub mod linter;
pub mod modules;
pub mod parser;
pub mod runtime;
pub mod stdlib;

// JIT Compiler (اختياري)
#[cfg(feature = "cranelift-backend")]
pub mod cranelift;

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير Bytecode VM مع JIT المتقدم
// ═══════════════════════════════════════════════════════════════════════════════

pub use bytecode::{
    print_benchmark_results,
    run_all_benchmarks,
    run_bytecode,
    run_bytecode_benchmark,
    // Advanced JIT exports
    AdvancedJitCompiler,
    AdvancedJitStats,
    BenchmarkResult,
    Chunk,
    CompiledCode,
    CompiledTrace,
    Compiler,
    HotSpotInfo,
    // JIT exports
    JitCompiler,
    JitStats,
    OpCode,
    OptimizedExecutor,
    SimdOperation,
    SimdProcessor,
    SimdStats,
    ThreadPool,
    ThreadedCodeExecutor,
    ThreadedStats,
    TierInfo,
    TierLevel,
    TierThresholds,
    Trace,
    TraceEntry,
    TraceState,
    TracingRecorder,
    VM,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير المكونات الأساسية
// ═══════════════════════════════════════════════════════════════════════════════

pub use error::{AlMarjaaError, ErrorCode, Position, Severity, Span};
pub use formatter::format_source;
pub use interpreter::Interpreter;
pub use lexer::Lexer;
pub use linter::{
    lint_program, lint_program_with_config, lint_source, lint_source_with_config, LintConfig,
    LintDiagnostic, LintLevel,
};
pub use parser::Parser;

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير Cranelift Backend (افتراضي - Rust-native بدون متطلبات خارجية)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "cranelift-backend")]
pub use cranelift::{
    compile_to_executable as cranelift_compile, jit_compile_and_run,
    Compiler as CraneliftCompiler, CompilerOptions as CraneliftOptions,
    CompilationResult as CraneliftCompilationResult, OptimizationLevel as CraneliftOptimizationLevel,
    MarjaaType as CraneliftMarjaaType, TypeSystem as CraneliftTypeSystem,
    JitCompiler as CraneliftJitCompiler, JitResult as CraneliftJitResult,
};

/// الإصدار الحالي للغة
pub const VERSION: &str = "3.4.0";

/// معلومات عن اللغة
pub fn info() -> &'static str {
    r#"
    ╔═══════════════════════════════════════════════════════════════╗
    ║         لغة المرجع - Al-Marjaa Language (Core)               ║
    ║         لغة برمجة عربية أساسية                                ║
    ╠═══════════════════════════════════════════════════════════════╣
    ║  الإصدار: 3.4.0                                              ║
    ║  Bytecode VM: ✅ مفعّل                                        ║
    ║  JIT Compiler: ✅ مفعّل (5 مستويات)                           ║
    ║  Parallel GC: ✅ مفعّل                                        ║
    ║  Standard Library: ✅ Regex, Crypto                          ║
    ║  المؤلف: رضوان دالي حمدوني                                   ║
    ║  GitHub: github.com/radhwendalyhamdouni/Al-Marjaa-Core       ║
    ╚═══════════════════════════════════════════════════════════════╝
"#
}
