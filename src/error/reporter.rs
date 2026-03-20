// ═══════════════════════════════════════════════════════════════════════════════
// نظام تقرير الأخطاء المحسن - Enhanced Error Reporter
// ═══════════════════════════════════════════════════════════════════════════════
// يتضمن:
// - Stack Trace عربي كامل
// - رسائل خطأ واضحة ومفصلة
// - اقتراحات ذكية للإصلاح
// - تمييز الأخطاء في الكود مع الألوان
// - تقارير متعددة الأخطاء
// ═══════════════════════════════════════════════════════════════════════════════

use colored::Colorize;
use std::fmt;
use std::collections::VecDeque;

// ═══════════════════════════════════════════════════════════════════════════════
// موقع الخطأ في الكود المصدري
// ═══════════════════════════════════════════════════════════════════════════════

/// موقع في الكود المصدري
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    /// اسم الملف
    pub file: String,
    /// رقم السطر (يبدأ من 1)
    pub line: usize,
    /// رقم العمود (يبدأ من 1)
    pub column: usize,
    /// محتوى السطر
    pub line_content: String,
    /// موضع البداية في البايت
    pub start_offset: usize,
    /// موضع النهاية في البايت
    pub end_offset: usize,
}

impl SourceLocation {
    /// إنشاء موقع جديد
    pub fn new(file: impl Into<String>, line: usize, column: usize) -> Self {
        SourceLocation {
            file: file.into(),
            line,
            column,
            line_content: String::new(),
            start_offset: 0,
            end_offset: 0,
        }
    }

    /// إضافة محتوى السطر
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.line_content = content.into();
        self
    }

    /// إضافة المدى
    pub fn with_span(mut self, start: usize, end: usize) -> Self {
        self.start_offset = start;
        self.end_offset = end;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// إطار الـ Stack Trace
// ═══════════════════════════════════════════════════════════════════════════════

/// إطار في الـ Stack Trace
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// اسم الدالة
    pub function_name: String,
    /// موقع الإطار
    pub location: SourceLocation,
    /// هل هو دالة أصلية (native)
    pub is_native: bool,
    /// معاملات الدالة (للعرض)
    pub arguments: Vec<String>,
}

impl StackFrame {
    /// إنشاء إطار جديد
    pub fn new(function_name: impl Into<String>, location: SourceLocation) -> Self {
        StackFrame {
            function_name: function_name.into(),
            location,
            is_native: false,
            arguments: Vec::new(),
        }
    }

    /// تعيين كدالة أصلية
    pub fn native(mut self) -> Self {
        self.is_native = true;
        self
    }

    /// إضافة معاملات
    pub fn with_arguments(mut self, args: Vec<String>) -> Self {
        self.arguments = args;
        self
    }
}

/// الـ Stack Trace كاملاً
#[derive(Debug, Clone)]
pub struct StackTrace {
    /// الإطارات
    pub frames: VecDeque<StackFrame>,
    /// العمق الأقصى
    pub max_depth: usize,
}

impl StackTrace {
    /// إنشاء Stack Trace جديد
    pub fn new() -> Self {
        StackTrace {
            frames: VecDeque::new(),
            max_depth: 100,
        }
    }

    /// إنشاء بعمق محدد
    pub fn with_max_depth(max_depth: usize) -> Self {
        StackTrace {
            frames: VecDeque::new(),
            max_depth,
        }
    }

    /// إضافة إطار
    pub fn push(&mut self, frame: StackFrame) {
        if self.frames.len() >= self.max_depth {
            self.frames.pop_front();
        }
        self.frames.push_back(frame);
    }

    /// إزالة إطار
    pub fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop_back()
    }

    /// عدد الإطارات
    pub fn depth(&self) -> usize {
        self.frames.len()
    }

    /// هل فارغ
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// تنسيق للعرض بالعربية
    pub fn format(&self) -> String {
        self.format_arabic()
    }

    /// تنسيق بالعربية
    pub fn format_arabic(&self) -> String {
        let mut output = String::new();

        output.push_str(&"\n".bright_black().to_string());
        output.push_str(&"════════════════════════════════════════════════════════════".bright_black().to_string());
        output.push_str(&"\n");
        output.push_str(&"                    تتبع المكالمة                            ".bright_cyan().bold().to_string());
        output.push_str(&"\n");
        output.push_str(&"════════════════════════════════════════════════════════════".bright_black().to_string());
        output.push_str(&"\n\n");

        if self.frames.is_empty() {
            output.push_str(&"  (لا توجد معلومات تتبع)".dimmed().to_string());
            output.push_str(&"\n");
            return output;
        }

        for (i, frame) in self.frames.iter().rev().enumerate() {
            let frame_num = self.frames.len() - i;
            let is_top = i == 0;

            // السهم للموقع الحالي
            let arrow = if is_top {
                "→".bright_red().bold().to_string()
            } else {
                " ".to_string()
            };

            if frame.is_native {
                output.push_str(&format!(
                    "  {} [{}] {} {}\n",
                    arrow,
                    frame_num.to_string().bright_yellow(),
                    frame.function_name.bright_white().bold(),
                    "(دالة أصلية)".dimmed()
                ));
            } else {
                output.push_str(&format!(
                    "  {} [{}] {} في السطر {}، العمود {}\n",
                    arrow,
                    frame_num.to_string().bright_yellow(),
                    frame.function_name.bright_white().bold(),
                    frame.location.line.to_string().bright_cyan(),
                    frame.location.column.to_string().bright_cyan()
                ));

                // عرض محتوى السطر
                if !frame.location.line_content.is_empty() {
                    output.push_str(&format!(
                        "       │ {}\n",
                        frame.location.line_content.trim().bright_white()
                    ));

                    // سهم تحت الموقع
                    let spaces = frame.location.column.saturating_sub(1);
                    output.push_str(&format!(
                        "       │ {}⌃\n",
                        " ".repeat(spaces)
                    ));
                }
            }
        }

        output
    }
}

impl Default for StackTrace {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع الأخطاء
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع الخطأ مع الرمز والرسالة العربية
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    // أخطاء لغوية (E1xx)
    LexerError,      // E100
    UnclosedString,  // E101
    UnclosedComment, // E102
    InvalidNumber,   // E103
    InvalidChar,     // E104

    // أخطاء نحوية (E2xx)
    SyntaxError,      // E200
    UnexpectedToken,  // E201
    ExpectedToken,    // E202
    InvalidExpression, // E203
    MissingSemicolon, // E204
    InvalidStatement, // E205

    // أخطاء وقت التشغيل (E3xx)
    RuntimeError,       // E300
    UndefinedVariable,  // E301
    TypeError,          // E302
    DivisionByZero,     // E303
    IndexError,         // E304
    KeyError,           // E305
    AttributeError,     // E306
    ArgumentError,      // E307
    RecursionError,     // E308
    OverflowError,      // E309

    // أخطاء النظام (E4xx)
    IOError,            // E400
    FileNotFoundError,  // E401
    PermissionError,    // E402
    NetworkError,       // E403
    MemoryError,        // E404

    // أخطاء المنطق (E5xx)
    AssertionError,     // E500
    NotImplementedError, // E501

    // خطأ عام
    Generic,            // E999
}

impl ErrorKind {
    /// الحصول على رمز الخطأ
    pub fn code(&self) -> &'static str {
        match self {
            ErrorKind::LexerError => "E100",
            ErrorKind::UnclosedString => "E101",
            ErrorKind::UnclosedComment => "E102",
            ErrorKind::InvalidNumber => "E103",
            ErrorKind::InvalidChar => "E104",
            ErrorKind::SyntaxError => "E200",
            ErrorKind::UnexpectedToken => "E201",
            ErrorKind::ExpectedToken => "E202",
            ErrorKind::InvalidExpression => "E203",
            ErrorKind::MissingSemicolon => "E204",
            ErrorKind::InvalidStatement => "E205",
            ErrorKind::RuntimeError => "E300",
            ErrorKind::UndefinedVariable => "E301",
            ErrorKind::TypeError => "E302",
            ErrorKind::DivisionByZero => "E303",
            ErrorKind::IndexError => "E304",
            ErrorKind::KeyError => "E305",
            ErrorKind::AttributeError => "E306",
            ErrorKind::ArgumentError => "E307",
            ErrorKind::RecursionError => "E308",
            ErrorKind::OverflowError => "E309",
            ErrorKind::IOError => "E400",
            ErrorKind::FileNotFoundError => "E401",
            ErrorKind::PermissionError => "E402",
            ErrorKind::NetworkError => "E403",
            ErrorKind::MemoryError => "E404",
            ErrorKind::AssertionError => "E500",
            ErrorKind::NotImplementedError => "E501",
            ErrorKind::Generic => "E999",
        }
    }

    /// الحصول على الاسم العربي
    pub fn arabic_name(&self) -> &'static str {
        match self {
            ErrorKind::LexerError => "خطأ لغوي",
            ErrorKind::UnclosedString => "نص غير مغلق",
            ErrorKind::UnclosedComment => "تعليق غير مغلق",
            ErrorKind::InvalidNumber => "رقم غير صالح",
            ErrorKind::InvalidChar => "رمز غير صالح",
            ErrorKind::SyntaxError => "خطأ نحوي",
            ErrorKind::UnexpectedToken => "رمز غير متوقع",
            ErrorKind::ExpectedToken => "رمز متوقع",
            ErrorKind::InvalidExpression => "تعبير غير صالح",
            ErrorKind::MissingSemicolon => "فاصلة منقوطة مفقودة",
            ErrorKind::InvalidStatement => "تعليمة غير صالحة",
            ErrorKind::RuntimeError => "خطأ وقت التشغيل",
            ErrorKind::UndefinedVariable => "متغير غير معرف",
            ErrorKind::TypeError => "خطأ في النوع",
            ErrorKind::DivisionByZero => "قسمة على صفر",
            ErrorKind::IndexError => "فهرس خارج النطاق",
            ErrorKind::KeyError => "مفتاح غير موجود",
            ErrorKind::AttributeError => "خاصية غير موجودة",
            ErrorKind::ArgumentError => "خطأ في المعاملات",
            ErrorKind::RecursionError => "عودية عميقة جداً",
            ErrorKind::OverflowError => "تجاوز السعة",
            ErrorKind::IOError => "خطأ إدخال/إخراج",
            ErrorKind::FileNotFoundError => "ملف غير موجود",
            ErrorKind::PermissionError => "لا توجد صلاحيات",
            ErrorKind::NetworkError => "خطأ في الشبكة",
            ErrorKind::MemoryError => "خطأ في الذاكرة",
            ErrorKind::AssertionError => "فشل التأكيد",
            ErrorKind::NotImplementedError => "غير مطبق",
            ErrorKind::Generic => "خطأ",
        }
    }

    /// الحصول على الأيقونة
    pub fn icon(&self) -> &'static str {
        match self {
            ErrorKind::LexerError | ErrorKind::SyntaxError => "⚠️",
            ErrorKind::RuntimeError | ErrorKind::TypeError => "❌",
            ErrorKind::DivisionByZero | ErrorKind::OverflowError => "💥",
            ErrorKind::FileNotFoundError | ErrorKind::PermissionError => "📁",
            ErrorKind::RecursionError | ErrorKind::MemoryError => "🔥",
            _ => "❌",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اقتراحات الإصلاح
// ═══════════════════════════════════════════════════════════════════════════════

/// اقتراح لإصلاح الخطأ
#[derive(Debug, Clone)]
pub struct Suggestion {
    /// وصف الاقتراح
    pub message: String,
    /// مثال على الكود المقترح
    pub code: Option<String>,
    /// أولوية الاقتراح
    pub priority: u8,
}

impl Suggestion {
    /// إنشاء اقتراح جديد
    pub fn new(message: impl Into<String>) -> Self {
        Suggestion {
            message: message.into(),
            code: None,
            priority: 0,
        }
    }

    /// إضافة مثال
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// تعيين الأولوية
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الخطأ الكامل
// ═══════════════════════════════════════════════════════════════════════════════

/// خطأ كامل مع كل المعلومات
#[derive(Debug, Clone)]
pub struct MarjaaError {
    /// نوع الخطأ
    pub kind: ErrorKind,
    /// الرسالة الرئيسية
    pub message: String,
    /// موقع الخطأ
    pub location: Option<SourceLocation>,
    /// Stack Trace
    pub stack_trace: StackTrace,
    /// اقتراحات الإصلاح
    pub suggestions: Vec<Suggestion>,
    /// نص المساعدة
    pub help: Option<String>,
    /// ملاحظات إضافية
    pub notes: Vec<String>,
}

impl MarjaaError {
    /// إنشاء خطأ جديد
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        MarjaaError {
            kind,
            message: message.into(),
            location: None,
            stack_trace: StackTrace::new(),
            suggestions: Vec::new(),
            help: None,
            notes: Vec::new(),
        }
    }

    /// إضافة موقع
    pub fn at(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// إضافة Stack Trace
    pub fn with_trace(mut self, trace: StackTrace) -> Self {
        self.stack_trace = trace;
        self
    }

    /// إضافة اقتراح
    pub fn suggest(mut self, suggestion: Suggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// إضافة اقتراح بسيط
    pub fn suggest_simple(mut self, message: &str) -> Self {
        self.suggestions.push(Suggestion::new(message));
        self
    }

    /// إضافة نص مساعدة
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// إضافة ملاحظة
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// تنسيق الخطأ للعرض
    pub fn format(&self) -> String {
        self.format_colored()
    }

    /// تنسيق مع الألوان
    pub fn format_colored(&self) -> String {
        let mut output = String::new();

        // العنوان الرئيسي
        output.push_str("\n");
        output.push_str(&"╔══════════════════════════════════════════════════════════════╗".bright_red().to_string());
        output.push_str("\n");
        output.push_str(&format!(
            "║  {} {} [{}]\n",
            self.kind.icon(),
            self.kind.arabic_name().bright_red().bold(),
            self.kind.code().bright_yellow()
        ));
        output.push_str(&"╚══════════════════════════════════════════════════════════════╝".bright_red().to_string());
        output.push_str("\n\n");

        // الرسالة الرئيسية
        output.push_str(&format!("{} {}\n", "الرسالة:".bright_cyan().bold(), self.message.bright_white()));
        output.push_str("\n");

        // الموقع
        if let Some(ref loc) = self.location {
            output.push_str(&format!(
                "{} {}:{}:{}\n",
                "الموقع:".bright_cyan().bold(),
                loc.file.bright_white().underline(),
                loc.line.to_string().bright_yellow(),
                loc.column.to_string().bright_yellow()
            ));

            if !loc.line_content.is_empty() {
                output.push_str("\n");
                output.push_str(&format!("  {} │ {}\n", loc.line.to_string().bright_blue(), loc.line_content.bright_white()));

                // تمييز الموقع
                let spaces = loc.column.saturating_sub(1);
                output.push_str(&format!(
                    "    │ {}{}\n",
                    " ".repeat(spaces),
                    "^".bright_red().bold().repeat(if loc.end_offset > loc.start_offset {
                        (loc.end_offset - loc.start_offset).max(1)
                    } else {
                        1
                    })
                ));
            }
        }

        // Stack Trace
        if !self.stack_trace.is_empty() {
            output.push_str(&self.stack_trace.format_arabic());
        }

        // الاقتراحات
        if !self.suggestions.is_empty() {
            output.push_str("\n");
            output.push_str(&format!("{} اقتراحات الإصلاح:\n", "💡".bright_yellow()));
            for (i, suggestion) in self.suggestions.iter().enumerate() {
                output.push_str(&format!(
                    "   {}. {}\n",
                    (i + 1).to_string().bright_yellow(),
                    suggestion.message.bright_green()
                ));
                if let Some(ref code) = suggestion.code {
                    output.push_str(&format!("      {} {}\n", "مثال:".dimmed(), code.bright_white().italic()));
                }
            }
        }

        // المساعدة
        if let Some(ref help) = self.help {
            output.push_str("\n");
            output.push_str(&format!("{} {}\n", "ℹ️ مساعدة:".bright_cyan(), help.bright_white()));
        }

        // الملاحظات
        for note in &self.notes {
            output.push_str(&format!("{} {}\n", "📝 ملاحظة:".bright_magenta(), note.bright_white()));
        }

        output
    }

    /// تنسيق بدون ألوان
    pub fn format_plain(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "{}: {} [{}]\n",
            self.kind.icon(),
            self.kind.arabic_name(),
            self.kind.code()
        ));
        output.push_str(&format!("الرسالة: {}\n", self.message));

        if let Some(ref loc) = self.location {
            output.push_str(&format!("الموقع: {}:{}:{}\n", loc.file, loc.line, loc.column));

            if !loc.line_content.is_empty() {
                output.push_str(&format!("  {} │ {}\n", loc.line, loc.line_content));
                let spaces = loc.column.saturating_sub(1);
                output.push_str(&format!("    │ {}^\n", " ".repeat(spaces)));
            }
        }

        for (i, suggestion) in self.suggestions.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, suggestion.message));
            if let Some(ref code) = suggestion.code {
                output.push_str(&format!("   مثال: {}\n", code));
            }
        }

        if let Some(ref help) = self.help {
            output.push_str(&format!("مساعدة: {}\n", help));
        }

        output
    }
}

impl fmt::Display for MarjaaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_plain())
    }
}

impl std::error::Error for MarjaaError {}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال مساعدة لإنشاء الأخطاء الشائعة
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء خطأ متغير غير معرف
pub fn undefined_variable(name: &str, location: SourceLocation) -> MarjaaError {
    MarjaaError::new(
        ErrorKind::UndefinedVariable,
        format!("المتغير '{}' غير معرف", name),
    )
    .at(location)
    .suggest(Suggestion::new("هل تقصد تعريف المتغير؟").with_code(format!("متغير {} = قيمة", name)))
    .suggest(Suggestion::new("تحقق من كتابة الاسم بشكل صحيح"))
    .with_help("يجب تعريف المتغير قبل استخدامه")
}

/// إنشاء خطأ نوع
pub fn type_error(expected: &str, got: &str, location: SourceLocation) -> MarjaaError {
    MarjaaError::new(
        ErrorKind::TypeError,
        format!("متوقع نوع '{}'، لكن وجد '{}'", expected, got),
    )
    .at(location)
    .with_help("تأكد من أن القيمة من النوع الصحيح قبل استخدامها")
}

/// إنشاء خطأ قسمة على صفر
pub fn division_by_zero(location: SourceLocation) -> MarjaaError {
    MarjaaError::new(
        ErrorKind::DivisionByZero,
        "لا يمكن القسمة على صفر".to_string(),
    )
    .at(location)
    .with_help("تحقق من أن المقسوم عليه ليس صفراً قبل إجراء القسمة")
    .suggest(Suggestion::new("أضف شرطاً للتحقق").with_code("إذا مقسوم_عليه != 0: نتيجة = مقسوم / مقسوم_عليه"))
}

/// إنشاء خطأ فهرس خارج النطاق
pub fn index_out_of_bounds(index: i64, length: usize, location: SourceLocation) -> MarjaaError {
    MarjaaError::new(
        ErrorKind::IndexError,
        format!("الفهرس {} خارج النطاق [0..{}]", index, length.saturating_sub(1)),
    )
    .at(location)
    .suggest(Suggestion::new("استخدم فهرساً صالحاً").with_code(format!("استخدم فهرساً بين 0 و {}", length.saturating_sub(1))))
    .with_help("الفهارس تبدأ من 0 وتنتهي عند الطول - 1")
}

/// إنشاء خطأ مفتاح غير موجود
pub fn key_not_found(key: &str, location: SourceLocation) -> MarjaaError {
    MarjaaError::new(
        ErrorKind::KeyError,
        format!("المفتاح '{}' غير موجود في القاموس", key),
    )
    .at(location)
    .suggest(Suggestion::new("تحقق من وجود المفتاح أولاً").with_code(format!("إذا \"{}\" في قاموس: قيمة = قاموس[\"{}\"]", key, key)))
    .with_help("يمكنك استخدام دالة 'يحتوي' للتحقق من وجود المفتاح")
}

/// إنشاء خطأ عودية عميقة
pub fn recursion_limit_exceeded(limit: usize, location: SourceLocation) -> MarjaaError {
    MarjaaError::new(
        ErrorKind::RecursionError,
        format!("تجاوز الحد الأقصى لعمق العودية ({})", limit),
    )
    .at(location)
    .suggest(Suggestion::new("حول العودية إلى حلقة تكرارية"))
    .with_help("العودية العميقة قد تسبب استنفاد الذاكرة")
}

/// إنشاء خطأ نحوي
pub fn syntax_error(message: &str, location: SourceLocation) -> MarjaaError {
    MarjaaError::new(ErrorKind::SyntaxError, message.to_string())
        .at(location)
}

/// إنشاء خطأ رمز غير متوقع
pub fn unexpected_token(found: &str, expected: &str, location: SourceLocation) -> MarjaaError {
    MarjaaError::new(
        ErrorKind::UnexpectedToken,
        format!("وجد '{}' بينما المتوقع '{}'", found, expected),
    )
    .at(location)
    .suggest(Suggestion::new(&format!("أضف '{}' في الموضع المناسب", expected)))
}

/// إنشاء خطأ ملف غير موجود
pub fn file_not_found(path: &str, location: SourceLocation) -> MarjaaError {
    MarjaaError::new(
        ErrorKind::FileNotFoundError,
        format!("الملف '{}' غير موجود", path),
    )
    .at(location)
    .suggest(Suggestion::new("تحقق من مسار الملف"))
    .suggest(Suggestion::new("تأكد من أن الملف موجود في المجلد الصحيح"))
}

/// إنشاء خطأ تأكيد
pub fn assertion_failed(message: &str, location: SourceLocation) -> MarjaaError {
    MarjaaError::new(
        ErrorKind::AssertionError,
        format!("فشل التأكيد: {}", message),
    )
    .at(location)
    .with_help("التأكيد يفشل عندما يكون الشرط خطأ")
}

// ═══════════════════════════════════════════════════════════════════════════════
// مُبلغ الأخطاء (Error Reporter)
// ═══════════════════════════════════════════════════════════════════════════════

/// مُبلغ الأخطاء - يجمع ويبلغ عن أخطاء متعددة
#[derive(Debug, Default)]
pub struct ErrorReporter {
    /// الأخطاء المجمعة
    errors: Vec<MarjaaError>,
    /// التحذيرات
    warnings: Vec<MarjaaError>,
    /// الحد الأقصى للأخطاء قبل التوقف
    max_errors: usize,
}

impl ErrorReporter {
    /// إنشاء مُبلغ جديد
    pub fn new() -> Self {
        ErrorReporter {
            errors: Vec::new(),
            warnings: Vec::new(),
            max_errors: 100,
        }
    }

    /// تعيين الحد الأقصى للأخطاء
    pub fn with_max_errors(mut self, max: usize) -> Self {
        self.max_errors = max;
        self
    }

    /// إضافة خطأ
    pub fn error(&mut self, error: MarjaaError) {
        self.errors.push(error);
    }

    /// إضافة تحذير
    pub fn warning(&mut self, warning: MarjaaError) {
        self.warnings.push(warning);
    }

    /// هل وصل للحد الأقصى
    pub fn has_exceeded_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }

    /// عدد الأخطاء
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// عدد التحذيرات
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    /// هل توجد أخطاء
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// هل توجد تحذيرات
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// تقرير كامل
    pub fn report(&self) -> String {
        let mut output = String::new();

        // عرض الأخطاء
        if !self.errors.is_empty() {
            for error in &self.errors {
                output.push_str(&error.format_colored());
                output.push_str("\n");
            }
        }

        // عرض التحذيرات
        if !self.warnings.is_empty() {
            for warning in &self.warnings {
                output.push_str(&warning.format_colored());
                output.push_str("\n");
            }
        }

        // الملخص
        output.push_str(&"\n".bright_black().to_string());
        output.push_str(&"════════════════════════════════════════════════════════════".bright_black().to_string());
        output.push_str(&"\n");

        let summary = format!(
            "الملخص: {} خطأ، {} تحذير",
            self.error_count(),
            self.warning_count()
        );

        if self.has_errors() {
            output.push_str(&summary.bright_red().bold().to_string());
        } else if self.has_warnings() {
            output.push_str(&summary.bright_yellow().to_string());
        } else {
            output.push_str(&"لا توجد أخطاء أو تحذيرات ✓".bright_green().to_string());
        }

        output.push_str(&"\n".to_string());

        output
    }

    /// مسح الأخطاء
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
    }

    /// أخذ الأخطاء
    pub fn take_errors(&mut self) -> Vec<MarjaaError> {
        std::mem::take(&mut self.errors)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الاختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = MarjaaError::new(
            ErrorKind::UndefinedVariable,
            "المتغير 'س' غير معرف",
        )
        .at(SourceLocation::new("test.mrj", 10, 5).with_content("متغير س = 10"))
        .suggest_simple("تحقق من كتابة الاسم")
        .with_help("يجب تعريف المتغير قبل استخدامه");

        assert!(error.location.is_some());
        assert_eq!(error.suggestions.len(), 1);
        assert!(error.help.is_some());

        println!("{}", error.format_colored());
    }

    #[test]
    fn test_stack_trace() {
        let mut trace = StackTrace::new();

        trace.push(StackFrame::new(
            "الدالة_الرئيسية",
            SourceLocation::new("program.mrj", 10, 5).with_content("نتيجة = حساب(5)"),
        ));

        trace.push(StackFrame::new(
            "حساب",
            SourceLocation::new("program.mrj", 25, 3).with_content("أرجع س / 0"),
        ));

        assert_eq!(trace.depth(), 2);

        println!("{}", trace.format_arabic());
    }

    #[test]
    fn test_error_reporter() {
        let mut reporter = ErrorReporter::new();

        reporter.error(
            MarjaaError::new(ErrorKind::UndefinedVariable, "المتغير 'س' غير معرف")
                .at(SourceLocation::new("test.mrj", 5, 1)),
        );

        reporter.warning(
            MarjaaError::new(ErrorKind::Generic, "متغير غير مستخدم")
                .at(SourceLocation::new("test.mrj", 3, 1)),
        );

        assert_eq!(reporter.error_count(), 1);
        assert_eq!(reporter.warning_count(), 1);

        println!("{}", reporter.report());
    }

    #[test]
    fn test_helper_functions() {
        let error = undefined_variable("متغير_غير_موجود", SourceLocation::new("test.mrj", 10, 5));
        assert_eq!(error.kind, ErrorKind::UndefinedVariable);

        let error = division_by_zero(SourceLocation::new("test.mrj", 15, 10));
        assert_eq!(error.kind, ErrorKind::DivisionByZero);

        let error = index_out_of_bounds(10, 5, SourceLocation::new("test.mrj", 20, 3));
        assert!(error.message.contains("10"));
    }
}
