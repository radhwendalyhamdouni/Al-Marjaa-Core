// ═══════════════════════════════════════════════════════════════════════════════
// لغة المرجع - Al-Marjaa Language (النسخة الأساسية)
// ═══════════════════════════════════════════════════════════════════════════════
// © 2026 رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI
// جميع الحقوق محفوظة | All Rights Reserved
// ═══════════════════════════════════════════════════════════════════════════════
// لغة برمجة عربية أساسية مع JIT Compiler
// الإصدار 3.4.1
// المؤلف: رضوان دالي حمدوني
// البريد: almarjaa.project@hotmail.com
// ═══════════════════════════════════════════════════════════════════════════════
// تحذير: هذا المشروع محمي بموجب حقوق الملكية الفكرية.
// الاستخدام التجاري يتطلب إذناً كتابياً صريحاً من المؤلف.
// WARNING: This project is protected by intellectual property rights.
// Commercial use requires explicit written permission from the author.
// ═══════════════════════════════════════════════════════════════════════════════

// السماح بالأحرف العربية الخاصة (مثل التطويل ـ) في المعرفات
// Allow Arabic special characters (like tatweel ـ) in identifiers
#![allow(uncommon_codepoints)]

//! # لغة المرجع - Al-Marjaa Language
//! 
//! لغة برمجة عربية أساسية مع JIT Compiler متكامل.
//! 
//! ## البنية
//! 
//! ```text
//! src/
//! ├── core/           # المكونات الأساسية (مستقر)
//! │   ├── lexer/      # محلل معجمي
//! │   ├── parser/     # محلل نحوي
//! │   ├── interpreter/# المفسر
//! │   ├── bytecode/   # نظام البايت كود
//! │   └── error/      # معالجة الأخطاء
//! │
//! ├── libs/           # المكتبات والتوسعات
//! │   ├── stdlib/     # المكتبة القياسية
//! │   ├── modules/    # نظام الوحدات
//! │   └── cranelift/  # مترجم JIT (اختياري)
//! │
//! └── cli/            # واجهة سطر الأوامر
//! ```
//! 
//! ## API Boundaries
//! 
//! ### Core API (مستقر - لا يتغير)
//! 
//! - [`core::Lexer`] - محلل النصوص
//! - [`core::Parser`] - محلل البنية
//! - [`core::Interpreter`] - المفسر
//! - [`core::Compiler`] - مترجم البايت كود
//! - [`core::VM`] - الآلة الافتراضية
//! 
//! ### Extended API (قد يتغير)
//! 
//! - [`libs::stdlib`] - المكتبة القياسية
//! - [`libs::modules`] - نظام الوحدات
//! - [`libs::cranelift`] - مترجم Cranelift
//! 
//! ## Feature Flags
//! 
//! | Flag | الوصف |
//! |------|-------|
//! | `default` | المكونات الأساسية فقط |
//! | `cranelift-backend` | مترجم Cranelift JIT |
//! | `full-stdlib` | المكتبة القياسية الكاملة |
//! | `database` | دعم قواعد البيانات |
//! | `network` | دعم الشبكات |
//! | `crypto` | دوال التشفير |
//! 
//! ## مثال
//! 
//! ```rust,ignore
//! use almarjaa::{Lexer, Parser, Interpreter};
//! 
//! let source = r#"
//!     متغير ترحيب = "مرحباً بالعالم"؛
//!     اطبع(ترحيب)؛
//! "#;
//! 
//! let mut interp = Interpreter::new();
//! interp.run(source).expect("خطأ في التنفيذ");
//! ```

// ═══════════════════════════════════════════════════════════════════════════════
// API Boundaries - حدود الواجهة
// ═══════════════════════════════════════════════════════════════════════════════

/// Core API - واجهة برمجة التطبيقات الأساسية
/// 
/// هذه الواجهة مستقرة ومضمونة للتوافقية.
/// لا تتغير إلا في إصدارات رئيسية.
pub mod core;

/// Libraries API - واجهة برمجة المكتبات
/// 
/// هذه الواجهة قد تتغير بين الإصدارات.
/// بعض المكونات تتطلب feature flags.
pub mod libs;

// ═══════════════════════════════════════════════════════════════════════════════
// المكونات الداخلية - للحفاظ على التوافقية
// ═══════════════════════════════════════════════════════════════════════════════

// المكونات الأساسية - متاحة دائماً
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

// JIT Compiler (اختياري - يتطلب feature flag)
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
// تصدير المكونات الأساسية - Core Exports
// ═══════════════════════════════════════════════════════════════════════════════

pub use error::{AlMarjaaError, ErrorCode, Position, Severity, Span};
pub use formatter::format_source;
pub use interpreter::Interpreter;
pub use interpreter::value::Environment;
pub use lexer::Lexer;
pub use linter::{
    lint_program, lint_program_with_config, lint_source, lint_source_with_config, LintConfig,
    LintDiagnostic, LintLevel,
};
pub use parser::Parser;

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير Module System & Package Manager
// ═══════════════════════════════════════════════════════════════════════════════

pub use modules::{
    ExportKind, ExportStatement, ImportKind, ImportStatement, Module, ModuleError, ModuleId,
    ModuleManager, ModuleManagerStats, ModuleStats, SourceLocation, TypeDefinition,
    InstalledPackage, PackageError, PackageInfo, PackageManager, PackageManagerStats,
    PackageManifest, PackageSource,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير Cranelift Backend (اختياري)
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
pub const VERSION: &str = "3.4.1";

/// معلومات عن اللغة
pub fn info() -> &'static str {
    r#"
    ╔═══════════════════════════════════════════════════════════════╗
    ║         لغة المرجع - Al-Marjaa Language (Core)               ║
    ║         لغة برمجة عربية أساسية                                ║
    ╠═══════════════════════════════════════════════════════════════╣
    ║  الإصدار: 3.4.1                                              ║
    ║  API: Core (مستقر) + Libraries (توسعات)                       ║
    ║  Bytecode VM: ✅ مفعّل                                        ║
    ║  JIT Compiler: ✅ مفعّل (5 مستويات)                           ║
    ║  Parallel GC: ✅ مفعّل                                        ║
    ║  Standard Library: ✅ Regex, Crypto                          ║
    ║  المؤلف: رضوان دالي حمدوني                                   ║
    ║  GitHub: github.com/radhwendalyhamdouni/Al-Marjaa-Core       ║
    ║  Libraries: github.com/radhwendalyhamdouni/Al-Marjaa-Libraries║
    ╚═══════════════════════════════════════════════════════════════╝
"#
}
