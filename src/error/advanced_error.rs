// ═══════════════════════════════════════════════════════════════════════════════
// أخطاء متقدمة - Advanced Error Types
// ═══════════════════════════════════════════════════════════════════════════════
// هذا الملف يوفر أنواع أخطاء إضافية متوافقة مع النظام الأساسي
// ═══════════════════════════════════════════════════════════════════════════════

use super::{AlMarjaaError, ErrorCode, SourceLocation, StackTrace};
use std::fmt;

/// خطأ وقت التشغيل
#[derive(Debug, Clone)]
pub struct RuntimeError {
    /// رسالة الخطأ
    pub message: String,
    /// موقع الخطأ
    pub location: Option<SourceLocation>,
    /// Stack Trace
    pub stack_trace: Option<StackTrace>,
}

impl RuntimeError {
    /// إنشاء خطأ وقت تشغيل جديد
    pub fn new(message: impl Into<String>) -> Self {
        RuntimeError {
            message: message.into(),
            location: None,
            stack_trace: None,
        }
    }

    /// إضافة موقع
    pub fn at(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// إضافة Stack Trace
    pub fn with_trace(mut self, trace: StackTrace) -> Self {
        self.stack_trace = Some(trace);
        self
    }

    /// تحويل إلى AlMarjaaError
    pub fn into_marjaa_error(self, code: ErrorCode) -> AlMarjaaError {
        let mut error = AlMarjaaError::new(code, self.message);
        if let Some(loc) = self.location {
            error = error.at(loc);
        }
        if let Some(trace) = self.stack_trace {
            error = error.with_trace(trace);
        }
        error
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(ref loc) = self.location {
            write!(f, " ({}:{}:{})", loc.file, loc.line, loc.column)?;
        }
        Ok(())
    }
}

impl std::error::Error for RuntimeError {}

/// خطأ في التحليل اللغوي
#[derive(Debug, Clone)]
pub struct LexerError {
    /// رسالة الخطأ
    pub message: String,
    /// موقع الخطأ
    pub location: SourceLocation,
}

impl LexerError {
    /// إنشاء خطأ تحليل لغوي
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        LexerError {
            message: message.into(),
            location: SourceLocation::simple(line, column),
        }
    }

    /// إنشاء مع ملف
    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.location.file = file.into();
        self
    }

    /// تحويل إلى AlMarjaaError
    pub fn into_marjaa_error(self, code: ErrorCode) -> AlMarjaaError {
        AlMarjaaError::new(code, self.message).at(self.location)
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (السطر {}، العمود {})",
            self.message, self.location.line, self.location.column
        )
    }
}

impl std::error::Error for LexerError {}

/// خطأ في التحليل النحوي
#[derive(Debug, Clone)]
pub struct ParseError {
    /// رسالة الخطأ
    pub message: String,
    /// موقع الخطأ
    pub location: SourceLocation,
    /// الرمز المتوقع
    pub expected: Option<String>,
    /// الرمز الموجود
    pub found: Option<String>,
}

impl ParseError {
    /// إنشاء خطأ تحليل نحوي
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        ParseError {
            message: message.into(),
            location: SourceLocation::simple(line, column),
            expected: None,
            found: None,
        }
    }

    /// إضافة المتوقع والموجود
    pub fn with_expected_found(mut self, expected: &str, found: &str) -> Self {
        self.expected = Some(expected.to_string());
        self.found = Some(found.to_string());
        self
    }

    /// تحويل إلى AlMarjaaError
    pub fn into_marjaa_error(self) -> AlMarjaaError {
        let mut error = AlMarjaaError::new(ErrorCode::E200, self.message).at(self.location);

        if let (Some(expected), Some(_found)) = (self.expected, self.found) {
            error = error.suggest_with_code(
                &format!("أضف '{}' في الموضع المناسب", expected),
                &expected,
            );
        }

        error
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (السطر {}، العمود {})",
            self.message, self.location.line, self.location.column
        )
    }
}

impl std::error::Error for ParseError {}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال إنشاء الأخطاء
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء خطأ متغير غير معرف
pub fn undefined_variable_error(name: &str, line: usize, column: usize) -> AlMarjaaError {
    AlMarjaaError::new(ErrorCode::E300, format!("المتغير '{}' غير معرف", name))
        .at_simple(line, column)
        .suggest_with_code("هل تقصد تعريف المتغير؟", &format!("متغير {} = قيمة", name))
        .suggest_simple("تحقق من كتابة الاسم بشكل صحيح")
        .with_help("يجب تعريف المتغير قبل استخدامه")
}

/// إنشاء خطأ نوع
pub fn type_error_error(expected: &str, got: &str, line: usize, column: usize) -> AlMarjaaError {
    AlMarjaaError::new(
        ErrorCode::E301,
        format!("متوقع نوع '{}'، لكن وجد '{}'", expected, got),
    )
    .at_simple(line, column)
    .with_help("تأكد من أن القيمة من النوع الصحيح قبل استخدامها")
}

/// إنشاء خطأ قسمة على صفر
pub fn division_by_zero_error(line: usize, column: usize) -> AlMarjaaError {
    AlMarjaaError::new(ErrorCode::E302, "لا يمكن القسمة على صفر")
        .at_simple(line, column)
        .with_help("تحقق من أن المقسوم عليه ليس صفراً قبل إجراء القسمة")
        .suggest_with_code(
            "أضف شرطاً للتحقق",
            "إذا مقسوم_عليه != 0: نتيجة = مقسوم / مقسوم_عليه",
        )
}

/// إنشاء خطأ فهرس خارج النطاق
pub fn index_error(index: i64, length: usize, line: usize, column: usize) -> AlMarjaaError {
    AlMarjaaError::new(
        ErrorCode::E303,
        format!("الفهرس {} خارج النطاق [0..{}]", index, length.saturating_sub(1)),
    )
    .at_simple(line, column)
    .suggest_with_code(
        "استخدم فهرساً صالحاً",
        &format!("استخدم فهرساً بين 0 و {}", length.saturating_sub(1)),
    )
    .with_help("الفهارس تبدأ من 0 وتنتهي عند الطول - 1")
}

/// إنشاء خطأ مفتاح غير موجود
pub fn key_error(key: &str, line: usize, column: usize) -> AlMarjaaError {
    AlMarjaaError::new(
        ErrorCode::E306,
        format!("المفتاح '{}' غير موجود في القاموس", key),
    )
    .at_simple(line, column)
    .suggest_with_code(
        "تحقق من وجود المفتاح أولاً",
        &format!("إذا \"{}\" في قاموس: قيمة = قاموس[\"{}\"]", key, key),
    )
    .with_help("يمكنك استخدام دالة 'يحتوي' للتحقق من وجود المفتاح")
}

/// إنشاء خطأ عودية عميقة
pub fn recursion_error(max_depth: usize, line: usize, column: usize) -> AlMarjaaError {
    AlMarjaaError::new(
        ErrorCode::E308,
        format!("تجاوز الحد الأقصى لعمق العودية ({})", max_depth),
    )
    .at_simple(line, column)
    .suggest_simple("حول العودية إلى حلقة تكرارية")
    .with_help("العودية العميقة قد تسبب استنفاد الذاكرة")
}

/// إنشاء خطأ تأكيد
pub fn assertion_error(message: &str, line: usize, column: usize) -> AlMarjaaError {
    AlMarjaaError::new(ErrorCode::E500, format!("فشل التأكيد: {}", message))
        .at_simple(line, column)
        .with_help("التأكيد يفشل عندما يكون الشرط خطأ")
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_error() {
        let error = RuntimeError::new("خطأ في وقت التشغيل")
            .at(SourceLocation::new("test.mrj", 10, 5));

        assert!(error.location.is_some());
        println!("{}", error);
    }

    #[test]
    fn test_lexer_error() {
        let error = LexerError::new("رمز غير معروف", 5, 10);
        assert_eq!(error.location.line, 5);
        assert_eq!(error.location.column, 10);
    }

    #[test]
    fn test_parse_error() {
        let error = ParseError::new("توقع '؛'", 10, 5)
            .with_expected_found("؛", "نهاية السطر");

        assert!(error.expected.is_some());
        assert!(error.found.is_some());
    }

    #[test]
    fn test_error_conversion() {
        let runtime = RuntimeError::new("خطأ").at(SourceLocation::simple(1, 1));
        let marjaa = runtime.into_marjaa_error(ErrorCode::E300);
        assert_eq!(marjaa.code, ErrorCode::E300);
    }
}
