// ═══════════════════════════════════════════════════════════════════════════════
// نظام الأخطاء المتقدم - Advanced Error System
// ═══════════════════════════════════════════════════════════════════════════════
// يتضمن:
// - Stack Trace كامل
// - رسائل خطأ عربية واضحة
// - اقتراحات الإصلاح
// - تمييز الأخطاء في الكود
// ═══════════════════════════════════════════════════════════════════════════════

use std::fmt;
use std::collections::VecDeque;

/// موقع الخطأ في الكود
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub line_content: String,
}

/// إطار في Stack Trace
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub location: SourceLocation,
    pub is_native: bool,
}

/// Stack Trace كامل
#[derive(Debug, Clone)]
pub struct StackTrace {
    pub frames: VecDeque<StackFrame>,
    pub max_depth: usize,
}

impl StackTrace {
    pub fn new() -> Self {
        StackTrace {
            frames: VecDeque::new(),
            max_depth: 1000,
        }
    }
    
    pub fn push_frame(&mut self, frame: StackFrame) {
        if self.frames.len() >= self.max_depth {
            self.frames.pop_front();
        }
        self.frames.push_back(frame);
    }
    
    pub fn pop_frame(&mut self) -> Option<StackFrame> {
        self.frames.pop_back()
    }
    
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    
    /// تنسيق Stack Trace للعرض
    pub fn format(&self) -> String {
        let mut output = String::new();
        output.push_str("════════════════════════════════════════════════════════════\n");
        output.push_str("                    Stack Trace                            \n");
        output.push_str("════════════════════════════════════════════════════════════\n\n");
        
        if self.frames.is_empty() {
            output.push_str("  (لا توجد معلومات Stack Trace)\n");
            return output;
        }
        
        for (i, frame) in self.frames.iter().rev().enumerate() {
            let arrow = if i == 0 { "→" } else { " " };
            
            if frame.is_native {
                output.push_str(&format!(
                    "  {} [{}] {} (دالة أصلية)\n",
                    arrow,
                    self.frames.len() - i,
                    frame.function_name
                ));
            } else {
                output.push_str(&format!(
                    "  {} [{}] {} في السطر {}، العمود {}\n",
                    arrow,
                    self.frames.len() - i,
                    frame.function_name,
                    frame.location.line,
                    frame.location.column
                ));
                
                if !frame.location.line_content.is_empty() {
                    output.push_str(&format!(
                        "       │ {}\n",
                        frame.location.line_content.trim()
                    ));
                    output.push_str(&format!(
                        "       │ {}⌃\n",
                        " ".repeat(frame.location.column.saturating_sub(1))
                    ));
                }
            }
        }
        
        output
    }
}

/// نوع الخطأ
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    // أخطاء لغوية
    LexerError,
    SyntaxError,
    
    // أخطاء وقت التشغيل
    RuntimeError,
    TypeError,
    NameError,
    IndexError,
    KeyError,
    AttributeError,
    ValueError,
    ZeroDivisionError,
    OverflowError,
    RecursionError,
    MemoryError,
    
    // أخطاء النظام
    IOError,
    FileNotFoundError,
    PermissionError,
    NetworkError,
    
    // أخطاء المنطق
    AssertionError,
    NotImplementedError,
    
    // خطأ عام
    GenericError,
}

impl ErrorType {
    /// الحصول على اسم الخطأ بالعربية
    pub fn arabic_name(&self) -> &'static str {
        match self {
            ErrorType::LexerError => "خطأ لغوي",
            ErrorType::SyntaxError => "خطأ نحوي",
            ErrorType::RuntimeError => "خطأ وقت التشغيل",
            ErrorType::TypeError => "خطأ في النوع",
            ErrorType::NameError => "خطأ في الاسم",
            ErrorType::IndexError => "خطأ في الفهرس",
            ErrorType::KeyError => "خطأ في المفتاح",
            ErrorType::AttributeError => "خطأ في الخاصية",
            ErrorType::ValueError => "خطأ في القيمة",
            ErrorType::ZeroDivisionError => "خطأ قسمة على صفر",
            ErrorType::OverflowError => "خطأ تجاوز السعة",
            ErrorType::RecursionError => "خطأ عودية عميقة",
            ErrorType::MemoryError => "خطأ في الذاكرة",
            ErrorType::IOError => "خطأ في الإدخال/الإخراج",
            ErrorType::FileNotFoundError => "ملف غير موجود",
            ErrorType::PermissionError => "خطأ في الصلاحيات",
            ErrorType::NetworkError => "خطأ في الشبكة",
            ErrorType::AssertionError => "خطأ تأكيد",
            ErrorType::NotImplementedError => "غير مطبق",
            ErrorType::GenericError => "خطأ",
        }
    }
    
    /// الحصول على رمز الخطأ
    pub fn code(&self) -> &'static str {
        match self {
            ErrorType::LexerError => "E001",
            ErrorType::SyntaxError => "E002",
            ErrorType::RuntimeError => "E003",
            ErrorType::TypeError => "E004",
            ErrorType::NameError => "E005",
            ErrorType::IndexError => "E006",
            ErrorType::KeyError => "E007",
            ErrorType::AttributeError => "E008",
            ErrorType::ValueError => "E009",
            ErrorType::ZeroDivisionError => "E010",
            ErrorType::OverflowError => "E011",
            ErrorType::RecursionError => "E012",
            ErrorType::MemoryError => "E013",
            ErrorType::IOError => "E014",
            ErrorType::FileNotFoundError => "E015",
            ErrorType::PermissionError => "E016",
            ErrorType::NetworkError => "E017",
            ErrorType::AssertionError => "E018",
            ErrorType::NotImplementedError => "E019",
            ErrorType::GenericError => "E999",
        }
    }
}

/// اقتراح إصلاح
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    pub message: String,
    pub suggested_code: Option<String>,
}

/// خطأ متقدم مع كل المعلومات
#[derive(Debug, Clone)]
pub struct MarjaaError {
    pub error_type: ErrorType,
    pub message: String,
    pub location: Option<SourceLocation>,
    pub stack_trace: StackTrace,
    pub suggestions: Vec<FixSuggestion>,
    pub help_text: Option<String>,
}

impl MarjaaError {
    pub fn new(error_type: ErrorType, message: String) -> Self {
        MarjaaError {
            error_type,
            message,
            location: None,
            stack_trace: StackTrace::new(),
            suggestions: Vec::new(),
            help_text: None,
        }
    }
    
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
    
    pub fn with_stack_trace(mut self, trace: StackTrace) -> Self {
        self.stack_trace = trace;
        self
    }
    
    pub fn with_suggestion(mut self, message: &str, code: Option<&str>) -> Self {
        self.suggestions.push(FixSuggestion {
            message: message.to_string(),
            suggested_code: code.map(|c| c.to_string()),
        });
        self
    }
    
    pub fn with_help(mut self, help: &str) -> Self {
        self.help_text = Some(help.to_string());
        self
    }
    
    /// تنسيق الخطأ للعرض
    pub fn format(&self) -> String {
        let mut output = String::new();
        
        // العنوان الرئيسي
        output.push_str("\n");
        output.push_str("╔══════════════════════════════════════════════════════════════╗\n");
        output.push_str(&format!(
            "║  ❌ {} [{}]\n",
            self.error_type.arabic_name(),
            self.error_type.code()
        ));
        output.push_str("╚══════════════════════════════════════════════════════════════╝\n\n");
        
        // الرسالة الرئيسية
        output.push_str(&format!("الرسالة: {}\n\n", self.message));
        
        // الموقع
        if let Some(ref loc) = self.location {
            output.push_str(&format!(
                "الموقع: {}:{}:{}\n",
                loc.file, loc.line, loc.column
            ));
            
            if !loc.line_content.is_empty() {
                output.push_str("\n");
                output.push_str(&format!("  {} │ {}\n", loc.line, loc.line_content));
                output.push_str(&format!(
                    "    │ {}^\n",
                    " ".repeat(loc.column.saturating_sub(1))
                ));
            }
        }
        
        // Stack Trace
        if !self.stack_trace.frames.is_empty() {
            output.push_str(&self.stack_trace.format());
        }
        
        // الاقتراحات
        if !self.suggestions.is_empty() {
            output.push_str("\n💡 اقتراحات الإصلاح:\n");
            for (i, suggestion) in self.suggestions.iter().enumerate() {
                output.push_str(&format!("   {}. {}\n", i + 1, suggestion.message));
                if let Some(ref code) = suggestion.suggested_code {
                    output.push_str(&format!("      مثال: {}\n", code));
                }
            }
        }
        
        // نص المساعدة
        if let Some(ref help) = self.help_text {
            output.push_str(&format!("\nℹ️ مساعدة: {}\n", help));
        }
        
        output
    }
}

impl fmt::Display for MarjaaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl std::error::Error for MarjaaError {}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال إنشاء الأخطاء الشائعة
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء خطأ نوع
pub fn type_error(expected: &str, got: &str) -> MarjaaError {
    MarjaaError::new(
        ErrorType::TypeError,
        format!("متوقع نوع '{}'، لكن وجد '{}'", expected, got),
    )
    .with_help("تأكد من أن القيمة من النوع الصحيح قبل استخدامها")
}

/// إنشاء خطأ اسم
pub fn name_error(name: &str) -> MarjaaError {
    MarjaaError::new(
        ErrorType::NameError,
        format!("الاسم '{}' غير معرف", name),
    )
    .with_suggestion("هل قمت بتعريف المتغير؟", Some(&format!("متغير {} = قيمة", name)))
    .with_suggestion("هل هناك خطأ إملائي؟", None)
}

/// إنشاء خطأ فهرس
pub fn index_error(index: usize, length: usize) -> MarjaaError {
    MarjaaError::new(
        ErrorType::IndexError,
        format!("الفهرس {} خارج النطاق (الطول: {})", index, length),
    )
    .with_help("الفهرس يجب أن يكون بين 0 والطول - 1")
}

/// إنشاء خطأ مفتاح
pub fn key_error(key: &str) -> MarjaaError {
    MarjaaError::new(
        ErrorType::KeyError,
        format!("المفتاح '{}' غير موجود في القاموس", key),
    )
    .with_suggestion("تحقق من وجود المفتاح قبل الوصول إليه", None)
}

/// إنشاء خطأ قسمة على صفر
pub fn zero_division_error() -> MarjaaError {
    MarjaaError::new(
        ErrorType::ZeroDivisionError,
        "لا يمكن القسمة على صفر".to_string(),
    )
    .with_help("تحقق من أن المقسوم عليه ليس صفراً قبل القسمة")
}

/// إنشاء خطأ عودية عميقة
pub fn recursion_error(max_depth: usize) -> MarjaaError {
    MarjaaError::new(
        ErrorType::RecursionError,
        format!("تجاوز الحد الأقصى لعمق العودية ({})", max_depth),
    )
    .with_help("حاول تحويل العودية إلى حلقة تكرارية أو زيادة الحد الأقصى")
}

/// إنشاء خطأ سعة
pub fn overflow_error(operation: &str) -> MarjaaError {
    MarjaaError::new(
        ErrorType::OverflowError,
        format!("تجاوز السعة في العملية: {}", operation),
    )
    .with_help("القيمة كبيرة جداً للنوع المستخدم")
}

/// إنشاء خطأ نحوي
pub fn syntax_error(message: &str, line: usize, column: usize, source: &str) -> MarjaaError {
    let line_content = source.lines()
        .nth(line.saturating_sub(1))
        .unwrap_or("")
        .to_string();
    
    MarjaaError::new(
        ErrorType::SyntaxError,
        message.to_string(),
    )
    .with_location(SourceLocation {
        file: "البرنامج".to_string(),
        line,
        column,
        line_content,
    })
}

/// إنشاء خطأ تأكيد
pub fn assertion_error(message: &str) -> MarjaaError {
    MarjaaError::new(
        ErrorType::AssertionError,
        format!("فشل التأكيد: {}", message),
    )
}

/// إنشاء خطأ ملف غير موجود
pub fn file_not_found_error(path: &str) -> MarjaaError {
    MarjaaError::new(
        ErrorType::FileNotFoundError,
        format!("الملف '{}' غير موجود", path),
    )
    .with_suggestion("تحقق من مسار الملف", None)
    .with_suggestion("تأكد من أن الملف موجود في المجلد الصحيح", None)
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_formatting() {
        let error = MarjaaError::new(
            ErrorType::TypeError,
            "متوقع رقم، وجد نص".to_string(),
        )
        .with_help("حول النص إلى رقم باستخدام دالة رقم()");
        
        let formatted = error.format();
        assert!(formatted.contains("خطأ في النوع"));
        assert!(formatted.contains("متوقع رقم"));
        println!("{}", formatted);
    }
    
    #[test]
    fn test_stack_trace() {
        let mut trace = StackTrace::new();
        
        trace.push_frame(StackFrame {
            function_name: "دالة_رئيسية".to_string(),
            location: SourceLocation {
                file: "برنامج.mrj".to_string(),
                line: 10,
                column: 5,
                line_content: "نتيجة = حساب(5)".to_string(),
            },
            is_native: false,
        });
        
        trace.push_frame(StackFrame {
            function_name: "حساب".to_string(),
            location: SourceLocation {
                file: "برنامج.mrj".to_string(),
                line: 25,
                column: 3,
                line_content: "أرجع س / 0".to_string(),
            },
            is_native: false,
        });
        
        let formatted = trace.format();
        assert!(formatted.contains("حساب"));
        println!("{}", formatted);
    }
    
    #[test]
    fn test_helper_functions() {
        let error = name_error("متغير_غير_موجود");
        assert!(error.suggestions.len() > 0);
        
        let error = type_error("رقم", "نص");
        assert_eq!(error.error_type, ErrorType::TypeError);
        
        let error = index_error(10, 5);
        assert!(error.message.contains("10"));
    }
}
