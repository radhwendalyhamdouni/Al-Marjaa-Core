// ═══════════════════════════════════════════════════════════════════════════════
// Cranelift Backend - مترجم لغة المرجع إلى كود آلة أصلي
// ═══════════════════════════════════════════════════════════════════════════════
// نظام متقدم لترجمة كود لغة المرجع إلى كود آلة أصلي باستخدام Cranelift
// مكتمل بالكامل بـ Rust - لا يتطلب تثبيت أي مكتبات خارجية
// ═══════════════════════════════════════════════════════════════════════════════

pub mod compiler;
pub mod jit;
pub mod types;

// إعادة تصدير الأنواع الرئيسية
pub use compiler::{Compiler, CompilerOptions, CompilationResult, OptimizationLevel};
pub use jit::{JitCompiler, JitResult};
pub use types::{MarjaaType, TypeSystem};

use crate::parser::ast::Program;

/// نتيجة الترجمة
#[derive(Debug)]
pub struct CraneliftResult {
    /// هل نجحت الترجمة
    pub success: bool,
    /// الأخطاء إن وجدت
    pub errors: Vec<String>,
    /// التحذيرات
    pub warnings: Vec<String>,
    /// وقت الترجمة (مللي ثانية)
    pub compile_time_ms: u64,
    /// حجم الملف المُنتج
    pub output_size: Option<u64>,
}

/// ترجمة برنامج وإنتاج ملف تنفيذي
pub fn compile_to_executable(program: &Program, options: CompilerOptions) -> CraneliftResult {
    let start = std::time::Instant::now();
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // إنشاء المترجم
    let mut compiler = match Compiler::new(options) {
        Ok(c) => c,
        Err(e) => {
            return CraneliftResult {
                success: false,
                errors: vec![e],
                warnings,
                compile_time_ms: start.elapsed().as_millis() as u64,
                output_size: None,
            };
        }
    };

    // ترجمة البرنامج
    match compiler.compile(program) {
        Ok(result) => {
            if !result.errors.is_empty() {
                errors.extend(result.errors);
            }
            if !result.warnings.is_empty() {
                warnings.extend(result.warnings);
            }
        }
        Err(e) => {
            errors.push(e);
        }
    }

    let success = errors.is_empty();
    let compile_time_ms = start.elapsed().as_millis() as u64;

    CraneliftResult {
        success,
        errors,
        warnings,
        compile_time_ms,
        output_size: None,
    }
}

/// JIT Compilation - ترجمة في الذاكرة وتنفيذ
pub fn jit_compile_and_run(program: &Program) -> Result<f64, String> {
    let mut jit = JitCompiler::new()?;
    jit.compile_and_run(program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_default() {
        let options = CompilerOptions::default();
        assert_eq!(options.output_file, "a.out");
    }
}
