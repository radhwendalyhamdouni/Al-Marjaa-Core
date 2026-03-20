// ═══════════════════════════════════════════════════════════════════════════════
// نظام تقرير الأخطاء - Error Reporter System
// ═══════════════════════════════════════════════════════════════════════════════
// يوفر واجهة موحدة للإبلاغ عن الأخطاء والتحذيرات
// ═══════════════════════════════════════════════════════════════════════════════

pub use super::ErrorReporter;
pub use super::ErrorManager;

// ═══════════════════════════════════════════════════════════════════════════════
// وظائف مساعدة للطباعة
// ═══════════════════════════════════════════════════════════════════════════════

use super::{AlMarjaaError, ErrorCode, Severity, SourceLocation};

/// طباعة خطأ بشكل جميل
pub fn print_error(error: &AlMarjaaError) {
    eprintln!("{}", error.format_colored());
}

/// طباعة خطأ بسيط
pub fn print_simple_error(code: ErrorCode, message: &str, file: &str, line: usize, column: usize) {
    let error = AlMarjaaError::new(code, message)
        .at(SourceLocation::new(file, line, column));
    print_error(&error);
}

/// طباعة ملخص الأخطاء
pub fn print_error_summary(errors: &[AlMarjaaError], warnings: &[AlMarjaaError]) {
    let mut reporter = ErrorReporter::new();
    for error in errors {
        reporter.error(error.clone());
    }
    for warning in warnings {
        reporter.warning(warning.clone());
    }
    eprintln!("{}", reporter.report());
}

// ═══════════════════════════════════════════════════════════════════════════════
// منسق الأخطاء
// ═══════════════════════════════════════════════════════════════════════════════

/// منسق الأخطاء - يوفر طرق تنسيق مختلفة
pub struct ErrorFormatter;

impl ErrorFormatter {
    /// تنسيق خطأ كنص بسيط
    pub fn to_plain(error: &AlMarjaaError) -> String {
        error.format_plain()
    }

    /// تنسيق خطأ بألوان
    pub fn to_colored(error: &AlMarjaaError) -> String {
        error.format_colored()
    }

    /// تنسيق كـ JSON (للـ IDE)
    pub fn to_json(error: &AlMarjaaError) -> String {
        let location = error.location.as_ref();
        format!(
            r#"{{
  "code": "{}",
  "severity": "{}",
  "message": "{}",
  "file": "{}",
  "line": {},
  "column": {}
}}"#,
            error.code.code(),
            match error.severity {
                Severity::Warning => "warning",
                Severity::Error => "error",
                Severity::Critical => "critical",
            },
            error.message,
            location.map(|l| l.file.as_str()).unwrap_or(""),
            location.map(|l| l.line).unwrap_or(0),
            location.map(|l| l.column).unwrap_or(0)
        )
    }

    /// تنسيق متعدد الأخطاء
    pub fn format_multiple(errors: &[AlMarjaaError]) -> String {
        let mut output = String::new();
        for (i, error) in errors.iter().enumerate() {
            if i > 0 {
                output.push('\n');
            }
            output.push_str(&error.format_colored());
        }
        output
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_error() {
        let error = AlMarjaaError::new(ErrorCode::E300, "متغير غير معرف")
            .at_simple(10, 5)
            .with_help("تحقق من اسم المتغير");
        print_error(&error);
    }

    #[test]
    fn test_error_formatter() {
        let error = AlMarjaaError::new(ErrorCode::E302, "قسمة على صفر")
            .at(SourceLocation::new("test.mrj", 15, 10));

        let plain = ErrorFormatter::to_plain(&error);
        assert!(plain.contains("قسمة على صفر"));

        let json = ErrorFormatter::to_json(&error);
        assert!(json.contains("E302"));
    }

    #[test]
    fn test_format_multiple() {
        let errors = vec![
            AlMarjaaError::new(ErrorCode::E300, "خطأ 1").at_simple(1, 1),
            AlMarjaaError::new(ErrorCode::E301, "خطأ 2").at_simple(2, 1),
        ];

        let formatted = ErrorFormatter::format_multiple(&errors);
        assert!(formatted.contains("خطأ 1"));
        assert!(formatted.contains("خطأ 2"));
    }
}
