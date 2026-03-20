// ═══════════════════════════════════════════════════════════════════════════════
// نظام الأخطاء المحسن للغة المرجع - Enhanced Error System for Al-Marjaa
// ═══════════════════════════════════════════════════════════════════════════════
// يتضمن:
// - Stack Trace عربي كامل
// - رسائل خطأ واضحة ومفصلة
// - اقتراحات ذكية للإصلاح
// - تمييز الأخطاء في الكود مع الألوان
// - تحديد الموقع الدقيق (ملف:سطر:عمود)
// ═══════════════════════════════════════════════════════════════════════════════

pub mod advanced_error;
pub mod reporter;

use colored::Colorize;
use std::collections::VecDeque;
use std::fmt;

// ═══════════════════════════════════════════════════════════════════════════════
// مستوى خطورة الخطأ
// ═══════════════════════════════════════════════════════════════════════════════

/// مستوى خطورة الخطأ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// تحذير - لا يمنع التنفيذ
    Warning,
    /// خطأ - يمنع التنفيذ
    Error,
    /// خطأ حرج - يتطلب إيقاف فوري
    Critical,
}

impl Severity {
    /// الاسم بالعربية
    pub fn arabic_name(&self) -> &'static str {
        match self {
            Severity::Warning => "تحذير",
            Severity::Error => "خطأ",
            Severity::Critical => "خطأ حرج",
        }
    }

    /// الأيقونة المناسبة
    pub fn icon(&self) -> &'static str {
        match self {
            Severity::Warning => "⚠️",
            Severity::Error => "❌",
            Severity::Critical => "🔥",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع الأخطاء مع الرموز
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع الخطأ مع رمز فريد ورسالة عربية
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // أخطاء لغوية (E1xx)
    E100, // رمز غير معروف
    E101, // نص غير مغلق
    E102, // تعليق غير مغلق
    E103, // رقم غير صالح
    E104, // حرف غير صالح

    // أخطاء نحوية (E2xx)
    E200, // رمز غير متوقع
    E201, // توقع رمز معين
    E202, // تعبير غير صالح
    E203, // نقطة فاصلة مفقودة
    E204, // تعليمة غير صالحة
    E205, // خطأ في الإزاحة

    // أخطاء وقت التشغيل (E3xx)
    E300, // متغير غير معرف
    E301, // خطأ في النوع
    E302, // القسمة على صفر
    E303, // الفهرس خارج النطاق
    E304, // النوع ليس دالة
    E305, // معاملات خاطئة
    E306, // مفتاح غير موجود
    E307, // خاصية غير موجودة
    E308, // عودية عميقة
    E309, // تجاوز السعة

    // أخطاء النظام (E4xx)
    E400, // خطأ داخلي
    E401, // الملف غير موجود
    E402, // لا توجد صلاحيات
    E403, // خطأ في الشبكة
    E404, // خطأ في الذاكرة

    // أخطاء المنطق (E5xx)
    E500, // فشل التأكيد
    E501, // غير مطبق

    // خطأ عام
    E999, // خطأ عام
}

impl ErrorCode {
    /// الحصول على رمز الخطأ كنص
    pub fn code(&self) -> &'static str {
        match self {
            ErrorCode::E100 => "E100",
            ErrorCode::E101 => "E101",
            ErrorCode::E102 => "E102",
            ErrorCode::E103 => "E103",
            ErrorCode::E104 => "E104",
            ErrorCode::E200 => "E200",
            ErrorCode::E201 => "E201",
            ErrorCode::E202 => "E202",
            ErrorCode::E203 => "E203",
            ErrorCode::E204 => "E204",
            ErrorCode::E205 => "E205",
            ErrorCode::E300 => "E300",
            ErrorCode::E301 => "E301",
            ErrorCode::E302 => "E302",
            ErrorCode::E303 => "E303",
            ErrorCode::E304 => "E304",
            ErrorCode::E305 => "E305",
            ErrorCode::E306 => "E306",
            ErrorCode::E307 => "E307",
            ErrorCode::E308 => "E308",
            ErrorCode::E309 => "E309",
            ErrorCode::E400 => "E400",
            ErrorCode::E401 => "E401",
            ErrorCode::E402 => "E402",
            ErrorCode::E403 => "E403",
            ErrorCode::E404 => "E404",
            ErrorCode::E500 => "E500",
            ErrorCode::E501 => "E501",
            ErrorCode::E999 => "E999",
        }
    }

    /// الحصول على الرسالة العربية الافتراضية
    pub fn arabic_message(&self) -> &'static str {
        match self {
            ErrorCode::E100 => "رمز غير معروف",
            ErrorCode::E101 => "نص غير مغلق",
            ErrorCode::E102 => "تعليق غير مغلق",
            ErrorCode::E103 => "رقم غير صالح",
            ErrorCode::E104 => "حرف غير صالح",
            ErrorCode::E200 => "رمز غير متوقع",
            ErrorCode::E201 => "توقع رمزاً معيناً",
            ErrorCode::E202 => "تعبير غير صالح",
            ErrorCode::E203 => "نقطة فاصلة مفقودة",
            ErrorCode::E204 => "تعليمة غير صالحة",
            ErrorCode::E205 => "خطأ في الإزاحة",
            ErrorCode::E300 => "متغير غير معرف",
            ErrorCode::E301 => "خطأ في النوع",
            ErrorCode::E302 => "القسمة على صفر",
            ErrorCode::E303 => "الفهرس خارج النطاق",
            ErrorCode::E304 => "النوع ليس دالة",
            ErrorCode::E305 => "معاملات خاطئة",
            ErrorCode::E306 => "مفتاح غير موجود",
            ErrorCode::E307 => "خاصية غير موجودة",
            ErrorCode::E308 => "عودية عميقة جداً",
            ErrorCode::E309 => "تجاوز السعة",
            ErrorCode::E400 => "خطأ داخلي",
            ErrorCode::E401 => "الملف غير موجود",
            ErrorCode::E402 => "لا توجد صلاحيات",
            ErrorCode::E403 => "خطأ في الشبكة",
            ErrorCode::E404 => "خطأ في الذاكرة",
            ErrorCode::E500 => "فشل التأكيد",
            ErrorCode::E501 => "غير مطبق",
            ErrorCode::E999 => "خطأ",
        }
    }

    /// الحصول على الأيقونة المناسبة
    pub fn icon(&self) -> &'static str {
        match self {
            ErrorCode::E100 | ErrorCode::E101 | ErrorCode::E102 | ErrorCode::E103 | ErrorCode::E104 => "⚠️",
            ErrorCode::E200 | ErrorCode::E201 | ErrorCode::E202 | ErrorCode::E203 | ErrorCode::E204 | ErrorCode::E205 => "⚠️",
            ErrorCode::E300 | ErrorCode::E301 | ErrorCode::E302 | ErrorCode::E303 | ErrorCode::E304 |
            ErrorCode::E305 | ErrorCode::E306 | ErrorCode::E307 | ErrorCode::E308 | ErrorCode::E309 => "❌",
            ErrorCode::E400 | ErrorCode::E401 | ErrorCode::E402 | ErrorCode::E403 | ErrorCode::E404 => "🔥",
            ErrorCode::E500 | ErrorCode::E501 => "💡",
            ErrorCode::E999 => "❌",
        }
    }
}

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

    /// إنشاء موقع من معلومات أساسية
    pub fn simple(line: usize, column: usize) -> Self {
        SourceLocation {
            file: "البرنامج".to_string(),
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

    /// إضافة اسم الملف
    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = file.into();
        self
    }

    /// تنسيق الموقع للعرض
    pub fn format_location(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.column)
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        SourceLocation::simple(1, 1)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// المدى في الكود (Span)
// ═══════════════════════════════════════════════════════════════════════════════

/// موقع في الكود (بداية ونهاية)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// موضع البداية
    pub start: Position,
    /// موضع النهاية
    pub end: Position,
}

impl Span {
    /// إنشاء مدى جديد
    pub fn new(start: Position, end: Position) -> Self {
        Span { start, end }
    }

    /// دمج مدى مع آخر
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start,
            end: other.end,
        }
    }

    /// مدى فارغ
    pub fn zero() -> Self {
        Span {
            start: Position::zero(),
            end: Position::zero(),
        }
    }
}

/// موقع مفرد في الكود
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// السطر
    pub line: usize,
    /// العمود
    pub column: usize,
    /// الإزاحة بالبايت
    pub offset: usize,
}

impl Position {
    /// إنشاء موقع جديد
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Position {
            line,
            column,
            offset,
        }
    }

    /// موقع البداية
    pub fn zero() -> Self {
        Position {
            line: 1,
            column: 1,
            offset: 0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// إطار الـ Stack Trace
// ═══════════════════════════════════════════════════════════════════════════════

/// إطار في تتبع المكالمات
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
    /// عمق الإطار
    pub depth: usize,
}

impl StackFrame {
    /// إنشاء إطار جديد
    pub fn new(function_name: impl Into<String>, location: SourceLocation) -> Self {
        StackFrame {
            function_name: function_name.into(),
            location,
            is_native: false,
            arguments: Vec::new(),
            depth: 0,
        }
    }

    /// إنشاء إطار من معلومات بسيطة
    pub fn simple(function_name: &str, line: usize, column: usize) -> Self {
        StackFrame {
            function_name: function_name.to_string(),
            location: SourceLocation::simple(line, column),
            is_native: false,
            arguments: Vec::new(),
            depth: 0,
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

    /// تعيين العمق
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Stack Trace كامل
// ═══════════════════════════════════════════════════════════════════════════════

/// تتبع المكالمات كاملاً
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
        let depth = self.frames.len();
        let mut frame = frame;
        frame.depth = depth + 1;
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

    /// تنسيق للعرض بالعربية مع الألوان
    pub fn format_arabic(&self) -> String {
        let mut output = String::new();

        output.push_str(&"\n".bright_black().to_string());
        output.push_str(&"════════════════════════════════════════════════════════════════".bright_black().to_string());
        output.push('\n');
        output.push_str(&"                    📚 تتبع المكالمة                            ".bright_cyan().bold().to_string());
        output.push('\n');
        output.push_str(&"════════════════════════════════════════════════════════════════".bright_black().to_string());
        output.push_str("\n\n");

        if self.frames.is_empty() {
            output.push_str(&"  (لا توجد معلومات تتبع)".dimmed().to_string());
            output.push('\n');
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
                    "  {} [{}] {} \n",
                    arrow,
                    frame_num.to_string().bright_yellow(),
                    frame.function_name.bright_white().bold()
                ));
                
                output.push_str(&format!(
                    "       📍 {}:{}:{}\n",
                    frame.location.file.bright_cyan(),
                    frame.location.line.to_string().bright_yellow(),
                    frame.location.column.to_string().bright_yellow()
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

                // عرض المعاملات
                if !frame.arguments.is_empty() {
                    output.push_str(&format!(
                        "       📦 المعاملات: ({})\n",
                        frame.arguments.join(", ").dimmed()
                    ));
                }
            }

            if i < self.frames.len() - 1 {
                output.push_str(&"       │\n".bright_black().to_string());
            }
        }

        output
    }

    /// تنسيق بدون ألوان
    pub fn format_plain(&self) -> String {
        let mut output = String::new();

        output.push_str("\n════════════════════════════════════════════════════════════════\n");
        output.push_str("                    تتبع المكالمة\n");
        output.push_str("════════════════════════════════════════════════════════════════\n\n");

        if self.frames.is_empty() {
            output.push_str("  (لا توجد معلومات تتبع)\n");
            return output;
        }

        for (i, frame) in self.frames.iter().rev().enumerate() {
            let frame_num = self.frames.len() - i;
            let arrow = if i == 0 { "→" } else { " " };

            if frame.is_native {
                output.push_str(&format!(
                    "  {} [{}] {} (دالة أصلية)\n",
                    arrow, frame_num, frame.function_name
                ));
            } else {
                output.push_str(&format!(
                    "  {} [{}] {} في {}:{}:{}\n",
                    arrow,
                    frame_num,
                    frame.function_name,
                    frame.location.file,
                    frame.location.line,
                    frame.location.column
                ));

                if !frame.location.line_content.is_empty() {
                    output.push_str(&format!(
                        "       │ {}\n",
                        frame.location.line_content.trim()
                    ));
                    let spaces = frame.location.column.saturating_sub(1);
                    output.push_str(&format!("       │ {}⌃\n", " ".repeat(spaces)));
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
pub struct AlMarjaaError {
    /// رمز الخطأ
    pub code: ErrorCode,
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
    /// مستوى الخطورة
    pub severity: Severity,
    /// سياق الكود (عدة أسطر)
    pub code_context: Vec<(usize, String)>,
}

impl AlMarjaaError {
    /// إنشاء خطأ جديد
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        AlMarjaaError {
            code,
            message: message.into(),
            location: None,
            stack_trace: StackTrace::new(),
            suggestions: Vec::new(),
            help: None,
            notes: Vec::new(),
            severity: Severity::Error,
            code_context: Vec::new(),
        }
    }

    /// إضافة موقع
    pub fn at(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// إضافة موقع بسيط
    pub fn at_simple(mut self, line: usize, column: usize) -> Self {
        self.location = Some(SourceLocation::simple(line, column));
        self
    }

    /// إضافة موقع مع ملف
    pub fn at_file(mut self, file: &str, line: usize, column: usize) -> Self {
        self.location = Some(SourceLocation::new(file, line, column));
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

    /// إضافة اقتراح (alias للتوافق)
    pub fn with_suggestion(mut self, message: impl Into<String>) -> Self {
        self.suggestions.push(Suggestion::new(message));
        self
    }

    /// إضافة اقتراح مع مثال
    pub fn suggest_with_code(mut self, message: &str, code: &str) -> Self {
        self.suggestions.push(Suggestion::new(message).with_code(code));
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

    /// تعيين مستوى الخطورة
    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// إضافة سياق كود
    pub fn with_code_context(mut self, lines: Vec<(usize, String)>) -> Self {
        self.code_context = lines;
        self
    }

    /// إضافة سياق كود (للتوافق مع الكود القديم)
    pub fn with_source_context(mut self, context: impl Into<String>) -> Self {
        if let Some(ref mut loc) = self.location {
            loc.line_content = context.into();
        } else {
            self.location = Some(SourceLocation::simple(1, 1).with_content(context));
        }
        self
    }

    /// إضافة سياق الكود من السطر الحالي
    pub fn with_span(mut self, span: Span) -> Self {
        if let Some(ref mut loc) = self.location {
            loc.start_offset = span.start.offset;
            loc.end_offset = span.end.offset;
            loc.line = span.start.line;
            loc.column = span.start.column;
        }
        self
    }

    /// تنسيق الخطأ للعرض (يقبل معامل اختياري لاسم الملف للتوافق)
    pub fn format(&self, _filename: &str) -> String {
        self.format_colored()
    }

    /// تنسيق بدون معاملات
    pub fn format_simple(&self) -> String {
        self.format_colored()
    }

    /// تنسيق مع اسم ملف (للتوافق)
    pub fn format_with_filename(&self, _filename: &str) -> String {
        self.format_colored()
    }

    /// تنسيق مع اسم ملف (للتوافق مع Parser)
    pub fn format_with_file(&self, _filename: &str) -> String {
        self.format_colored()
    }

    /// تنسيق مع الألوان
    pub fn format_colored(&self) -> String {
        let mut output = String::new();

        // العنوان الرئيسي
        output.push('\n');
        output.push_str(&"╔════════════════════════════════════════════════════════════════╗".bright_red().to_string());
        output.push('\n');
        output.push_str(&format!(
            "║  {} {} {} [{}]\n",
            self.severity.icon(),
            self.severity.arabic_name().bright_red().bold(),
            self.code.icon(),
            self.code.code().bright_yellow()
        ));
        output.push_str(&"╚════════════════════════════════════════════════════════════════╝".bright_red().to_string());
        output.push_str("\n\n");

        // الرسالة الرئيسية
        output.push_str(&format!("{} {}\n", "📝 الرسالة:".bright_cyan().bold(), self.message.bright_white()));
        output.push('\n');

        // الموقع الدقيق
        if let Some(ref loc) = self.location {
            output.push_str(&format!(
                "{} {}:{}:{}\n",
                "📍 الموقع:".bright_cyan().bold(),
                loc.file.bright_white().underline(),
                loc.line.to_string().bright_yellow(),
                loc.column.to_string().bright_yellow()
            ));

            // عرض السياق مع الكود
            if !loc.line_content.is_empty() || !self.code_context.is_empty() {
                output.push('\n');

                // إذا كان هناك سياق متعدد الأسطر
                if !self.code_context.is_empty() {
                    for (line_num, line_content) in &self.code_context {
                        let is_error_line = *line_num == loc.line;
                        if is_error_line {
                            output.push_str(&format!(
                                "  {} │ {} {}\n",
                                line_num.to_string().bright_red().bold(),
                                "|".bright_red(),
                                line_content.bright_white().on_red()
                            ));
                            // سهم تحت الخطأ
                            let spaces = loc.column.saturating_sub(1);
                            let underline_len = if loc.end_offset > loc.start_offset {
                                (loc.end_offset - loc.start_offset).max(1)
                            } else {
                                1
                            };
                            output.push_str(&format!(
                                "    │ {}{}\n",
                                " ".repeat(spaces),
                                "^".bright_red().bold().repeat(underline_len)
                            ));
                        } else {
                            output.push_str(&format!(
                                "  {} │ {} {}\n",
                                line_num.to_string().bright_blue(),
                                "|".bright_blue(),
                                line_content.bright_white()
                            ));
                        }
                    }
                } else if !loc.line_content.is_empty() {
                    // سطر واحد فقط
                    output.push_str(&format!("  {} │ {}\n", loc.line.to_string().bright_blue(), loc.line_content.bright_white()));
                    
                    // تمييز الموقع
                    let spaces = loc.column.saturating_sub(1);
                    let underline_len = if loc.end_offset > loc.start_offset {
                        (loc.end_offset - loc.start_offset).max(1)
                    } else {
                        1
                    };
                    output.push_str(&format!(
                        "    │ {}{}\n",
                        " ".repeat(spaces),
                        "^".bright_red().bold().repeat(underline_len)
                    ));
                }
            }
            output.push('\n');
        }

        // Stack Trace
        if !self.stack_trace.is_empty() {
            output.push_str(&self.stack_trace.format_arabic());
        }

        // الاقتراحات
        if !self.suggestions.is_empty() {
            output.push('\n');
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
            output.push('\n');
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
            self.severity.icon(),
            self.severity.arabic_name(),
            self.code.code()
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

        if !self.stack_trace.is_empty() {
            output.push_str(&self.stack_trace.format_plain());
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

impl fmt::Display for AlMarjaaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_plain())
    }
}

impl std::error::Error for AlMarjaaError {}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال مساعدة لإنشاء الأخطاء الشائعة
// ═══════════════════════════════════════════════════════════════════════════════

/// وحدة مساعدة لإنشاء الأخطاء
pub mod helpers {
    use super::*;

    /// خطأ متغير غير معرف
    pub fn undefined_variable(name: &str, location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E300,
            format!("المتغير '{}' غير معرف", name),
        )
        .at(location)
        .suggest_with_code(
            "هل تقصد تعريف المتغير؟",
            &format!("متغير {} = قيمة", name),
        )
        .suggest_simple("تحقق من كتابة الاسم بشكل صحيح")
        .with_help("يجب تعريف المتغير قبل استخدامه")
    }

    /// خطأ نوع
    pub fn type_error(expected: &str, got: &str, location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E301,
            format!("متوقع نوع '{}'، لكن وجد '{}'", expected, got),
        )
        .at(location)
        .with_help("تأكد من أن القيمة من النوع الصحيح قبل استخدامها")
    }

    /// خطأ قسمة على صفر
    pub fn division_by_zero(location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E302,
            "لا يمكن القسمة على صفر",
        )
        .at(location)
        .with_help("تحقق من أن المقسوم عليه ليس صفراً قبل إجراء القسمة")
        .suggest_with_code(
            "أضف شرطاً للتحقق",
            "إذا مقسوم_عليه != 0: نتيجة = مقسوم / مقسوم_عليه",
        )
    }

    /// خطأ فهرس خارج النطاق
    pub fn index_out_of_bounds(index: i64, length: usize, location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E303,
            format!("الفهرس {} خارج النطاق [0..{}]", index, length.saturating_sub(1)),
        )
        .at(location)
        .suggest_with_code(
            "استخدم فهرساً صالحاً",
            &format!("استخدم فهرساً بين 0 و {}", length.saturating_sub(1)),
        )
        .with_help("الفهارس تبدأ من 0 وتنتهي عند الطول - 1")
    }

    /// خطأ مفتاح غير موجود
    pub fn key_not_found(key: &str, location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E306,
            format!("المفتاح '{}' غير موجود في القاموس", key),
        )
        .at(location)
        .suggest_with_code(
            "تحقق من وجود المفتاح أولاً",
            &format!("إذا \"{}\" في قاموس: قيمة = قاموس[\"{}\"]", key, key),
        )
        .with_help("يمكنك استخدام دالة 'يحتوي' للتحقق من وجود المفتاح")
    }

    /// خطأ عودية عميقة
    pub fn recursion_limit_exceeded(limit: usize, location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E308,
            format!("تجاوز الحد الأقصى لعمق العودية ({})", limit),
        )
        .at(location)
        .suggest_simple("حول العودية إلى حلقة تكرارية")
        .with_help("العودية العميقة قد تسبب استنفاد الذاكرة")
    }

    /// خطأ نحوي
    pub fn syntax_error(message: &str, location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(ErrorCode::E200, message.to_string())
            .at(location)
    }

    /// خطأ رمز غير متوقع
    pub fn unexpected_token(found: &str, expected: &str, span: Span) -> AlMarjaaError {
        let location = SourceLocation::simple(span.start.line, span.start.column);
        AlMarjaaError::new(
            ErrorCode::E200,
            format!("وجد '{}' بينما المتوقع '{}'", found, expected),
        )
        .at(location)
        .suggest_with_code(
            &format!("أضف '{}' في الموضع المناسب", expected),
            expected,
        )
    }

    /// خطأ توقع رمز (alias للتوافق)
    pub fn expected_token(expected: &str, found: &str, span: Span) -> AlMarjaaError {
        unexpected_token(found, expected, span)
    }

    /// خطأ رمز غير متوقع مع موقع
    pub fn unexpected_token_at_location(found: &str, expected: &str, location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E200,
            format!("وجد '{}' بينما المتوقع '{}'", found, expected),
        )
        .at(location)
        .suggest_with_code(
            &format!("أضف '{}' في الموضع المناسب", expected),
            expected,
        )
    }

    /// خطأ توقع رمز (alias للتوافق)
    pub fn expected_token_at_location(expected: &str, found: &str, location: SourceLocation) -> AlMarjaaError {
        unexpected_token_at_location(found, expected, location)
    }

    /// خطأ ملف غير موجود
    pub fn file_not_found(path: &str, location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E401,
            format!("الملف '{}' غير موجود", path),
        )
        .at(location)
        .suggest_simple("تحقق من مسار الملف")
        .suggest_simple("تأكد من أن الملف موجود في المجلد الصحيح")
    }

    /// خطأ تأكيد
    pub fn assertion_failed(message: &str, location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E500,
            format!("فشل التأكيد: {}", message),
        )
        .at(location)
        .with_help("التأكيد يفشل عندما يكون الشرط خطأ")
    }

    /// خطأ نص غير مغلق
    pub fn unclosed_string(line: usize, location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E101,
            format!("نص غير مغلق في السطر {}", line),
        )
        .at(location)
        .suggest_simple("أضف علامة الاقتباس المغلقة '\"' أو '\\''")
        .with_help("النصوص يجب أن تبدأ وتنتهي بنفس نوع علامة الاقتباس")
    }

    /// خطأ تعليق غير مغلق
    pub fn unclosed_comment(location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E102,
            "تعليق متعدد الأسطر غير مغلق",
        )
        .at(location)
        .suggest_simple("أضف '*/' لإغلاق التعليق")
    }

    /// خطأ رمز غير معروف
    pub fn unknown_char(ch: char, location: SourceLocation) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E100,
            format!("رمز غير معروف: '{}'", ch),
        )
        .at(location)
        .with_help("تأكد من أن الرمز مدعوم في اللغة")
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// مُبلغ الأخطاء (Error Reporter)
// ═══════════════════════════════════════════════════════════════════════════════

/// مُبلغ الأخطاء - يجمع ويبلغ عن أخطاء متعددة
#[derive(Debug, Default)]
pub struct ErrorReporter {
    /// الأخطاء المجمعة
    errors: Vec<AlMarjaaError>,
    /// التحذيرات
    warnings: Vec<AlMarjaaError>,
    /// الحد الأقصى للأخطاء قبل التوقف
    max_errors: usize,
    /// الملف الحالي
    current_file: String,
}

impl ErrorReporter {
    /// إنشاء مُبلغ جديد
    pub fn new() -> Self {
        ErrorReporter {
            errors: Vec::new(),
            warnings: Vec::new(),
            max_errors: 100,
            current_file: "البرنامج".to_string(),
        }
    }

    /// تعيين الملف الحالي
    pub fn set_file(&mut self, file: impl Into<String>) {
        self.current_file = file.into();
    }

    /// تعيين الحد الأقصى للأخطاء
    pub fn with_max_errors(mut self, max: usize) -> Self {
        self.max_errors = max;
        self
    }

    /// إضافة خطأ
    pub fn error(&mut self, error: AlMarjaaError) {
        self.errors.push(error);
    }

    /// إضافة خطأ بسيط
    pub fn simple_error(&mut self, code: ErrorCode, message: &str, line: usize, column: usize) {
        let error = AlMarjaaError::new(code, message)
            .at_file(&self.current_file.clone(), line, column);
        self.errors.push(error);
    }

    /// إضافة تحذير
    pub fn warning(&mut self, warning: AlMarjaaError) {
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
                output.push('\n');
            }
        }

        // عرض التحذيرات
        if !self.warnings.is_empty() {
            for warning in &self.warnings {
                output.push_str(&warning.format_colored());
                output.push('\n');
            }
        }

        // الملخص
        output.push_str(&"\n".bright_black().to_string());
        output.push_str(&"════════════════════════════════════════════════════════════════".bright_black().to_string());
        output.push('\n');

        let summary = format!(
            "📊 الملخص: {} خطأ، {} تحذير",
            self.error_count(),
            self.warning_count()
        );

        if self.has_errors() {
            output.push_str(&summary.bright_red().bold().to_string());
        } else if self.has_warnings() {
            output.push_str(&summary.bright_yellow().to_string());
        } else {
            output.push_str(&"✅ لا توجد أخطاء أو تحذيرات".bright_green().to_string());
        }

        output.push('\n');

        output
    }

    /// مسح الأخطاء
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
    }

    /// أخذ الأخطاء
    pub fn take_errors(&mut self) -> Vec<AlMarjaaError> {
        std::mem::take(&mut self.errors)
    }

    /// أخذ التحذيرات
    pub fn take_warnings(&mut self) -> Vec<AlMarjaaError> {
        std::mem::take(&mut self.warnings)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// مدير الأخطاء العالمي
// ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::rc::Rc;

/// مدير الأخطاء العالمي
pub struct ErrorManager {
    /// المُبلغ
    reporter: Rc<RefCell<ErrorReporter>>,
    /// Stack Trace الحالي
    stack_trace: Rc<RefCell<StackTrace>>,
    /// الملف الحالي
    current_file: Rc<RefCell<String>>,
    /// الكود المصدري الحالي
    source_lines: Rc<RefCell<Vec<String>>>,
}

impl ErrorManager {
    /// إنشاء مدير جديد
    pub fn new() -> Self {
        ErrorManager {
            reporter: Rc::new(RefCell::new(ErrorReporter::new())),
            stack_trace: Rc::new(RefCell::new(StackTrace::new())),
            current_file: Rc::new(RefCell::new("البرنامج".to_string())),
            source_lines: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// تعيين الملف الحالي
    pub fn set_file(&self, file: &str) {
        *self.current_file.borrow_mut() = file.to_string();
        self.reporter.borrow_mut().set_file(file);
    }

    /// تعيين الكود المصدري
    pub fn set_source(&self, source: &str) {
        *self.source_lines.borrow_mut() = source.lines().map(|s| s.to_string()).collect();
    }

    /// الحصول على سطر من الكود
    pub fn get_line(&self, line: usize) -> Option<String> {
        self.source_lines.borrow().get(line.saturating_sub(1)).cloned()
    }

    /// إنشاء موقع مع محتوى السطر
    pub fn create_location(&self, line: usize, column: usize) -> SourceLocation {
        let line_content = self.get_line(line).unwrap_or_default();
        SourceLocation::new(self.current_file.borrow().clone(), line, column)
            .with_content(line_content)
    }

    /// إضافة إطار للـ Stack Trace
    pub fn push_frame(&self, frame: StackFrame) {
        self.stack_trace.borrow_mut().push(frame);
    }

    /// إزالة إطار من الـ Stack Trace
    pub fn pop_frame(&self) -> Option<StackFrame> {
        self.stack_trace.borrow_mut().pop()
    }

    /// الحصول على نسخة من الـ Stack Trace
    pub fn get_stack_trace(&self) -> StackTrace {
        self.stack_trace.borrow().clone()
    }

    /// الإبلاغ عن خطأ
    pub fn report_error(&self, error: AlMarjaaError) {
        self.reporter.borrow_mut().error(error);
    }

    /// الإبلاغ عن تحذير
    pub fn report_warning(&self, warning: AlMarjaaError) {
        self.reporter.borrow_mut().warning(warning);
    }

    /// هل توجد أخطاء
    pub fn has_errors(&self) -> bool {
        self.reporter.borrow().has_errors()
    }

    /// عدد الأخطاء
    pub fn error_count(&self) -> usize {
        self.reporter.borrow().error_count()
    }

    /// الحصول على التقرير
    pub fn get_report(&self) -> String {
        self.reporter.borrow().report()
    }

    /// مسح الأخطاء
    pub fn clear(&self) {
        self.reporter.borrow_mut().clear();
        self.stack_trace.borrow_mut().frames.clear();
    }

    /// استنساخ المرجع
    pub fn clone_ref(&self) -> Self {
        ErrorManager {
            reporter: Rc::clone(&self.reporter),
            stack_trace: Rc::clone(&self.stack_trace),
            current_file: Rc::clone(&self.current_file),
            source_lines: Rc::clone(&self.source_lines),
        }
    }
}

impl Default for ErrorManager {
    fn default() -> Self {
        Self::new()
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
        let error = AlMarjaaError::new(
            ErrorCode::E300,
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
            AlMarjaaError::new(ErrorCode::E300, "المتغير 'س' غير معرف")
                .at(SourceLocation::new("test.mrj", 5, 1)),
        );

        reporter.warning(
            AlMarjaaError::new(ErrorCode::E999, "متغير غير مستخدم")
                .at(SourceLocation::new("test.mrj", 3, 1)),
        );

        assert_eq!(reporter.error_count(), 1);
        assert_eq!(reporter.warning_count(), 1);

        println!("{}", reporter.report());
    }

    #[test]
    fn test_helper_functions() {
        let error = helpers::undefined_variable("متغير_غير_موجود", SourceLocation::new("test.mrj", 10, 5));
        assert_eq!(error.code, ErrorCode::E300);

        let error = helpers::division_by_zero(SourceLocation::new("test.mrj", 15, 10));
        assert_eq!(error.code, ErrorCode::E302);

        let error = helpers::index_out_of_bounds(10, 5, SourceLocation::new("test.mrj", 20, 3));
        assert!(error.message.contains("10"));
    }

    #[test]
    fn test_error_manager() {
        let manager = ErrorManager::new();
        manager.set_file("test.mrj");
        manager.set_source("متغير س = 10\nاطبع(ص)");

        let location = manager.create_location(2, 8);
        assert_eq!(location.line, 2);
        assert_eq!(location.line_content, "اطبع(ص)");

        manager.push_frame(StackFrame::simple("الدالة_الرئيسية", 2, 1));

        let error = helpers::undefined_variable("ص", location);
        manager.report_error(error);

        assert!(manager.has_errors());
        println!("{}", manager.get_report());
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(ErrorCode::E300.code(), "E300");
        assert_eq!(ErrorCode::E300.arabic_message(), "متغير غير معرف");
        assert_eq!(ErrorCode::E302.code(), "E302");
        assert_eq!(ErrorCode::E302.arabic_message(), "القسمة على صفر");
    }
}
