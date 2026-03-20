// ═══════════════════════════════════════════════════════════════════════════════
// نظام الوحدات - Module System
// ═══════════════════════════════════════════════════════════════════════════════
// يتضمن:
// - تحميل الوحدات من ملفات .mrj
// - استيراد من المكتبة القياسية
// - استيراد من مكتبات خارجية
// - إدارة التبعيات الدائرية
// - التخزين المؤقت للوحدات
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════════
// تعريفات أساسية
// ═══════════════════════════════════════════════════════════════════════════════

/// معرف وحدة فريد
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleId {
    /// اسم الوحدة
    pub name: String,
    /// مسار الوحدة
    pub path: Option<PathBuf>,
    /// نوع الوحدة
    pub kind: ModuleKind,
}

impl ModuleId {
    /// إنشاء معرف وحدة من اسم
    pub fn from_name(name: impl Into<String>) -> Self {
        ModuleId {
            name: name.into(),
            path: None,
            kind: ModuleKind::Named,
        }
    }

    /// إنشاء معرف وحدة من مسار
    pub fn from_path(path: PathBuf) -> Self {
        let name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        ModuleId {
            name,
            path: Some(path),
            kind: ModuleKind::File,
        }
    }

    /// إنشاء معرف للمكتبة القياسية
    pub fn stdlib(name: impl Into<String>) -> Self {
        ModuleId {
            name: name.into(),
            path: None,
            kind: ModuleKind::Stdlib,
        }
    }

    /// إنشاء معرف لمكتبة خارجية
    pub fn external(name: impl Into<String>) -> Self {
        ModuleId {
            name: name.into(),
            path: None,
            kind: ModuleKind::External,
        }
    }
}

impl std::fmt::Display for ModuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.path {
            Some(p) => write!(f, "{} ({})", self.name, p.display()),
            None => write!(f, "{}", self.name),
        }
    }
}

/// نوع الوحدة
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleKind {
    /// وحدة مسماة
    Named,
    /// وحدة من ملف
    File,
    /// وحدة من المكتبة القياسية
    Stdlib,
    /// وحدة من مكتبة خارجية
    External,
}

// ═══════════════════════════════════════════════════════════════════════════════
// نتيجة الاستيراد
// ═══════════════════════════════════════════════════════════════════════════════

/// نتيجة الاستيراد
#[derive(Debug, Clone)]
pub struct Import {
    /// معرف الوحدة المستوردة
    pub module_id: ModuleId,
    /// العناصر المستوردة
    pub items: Vec<ImportItem>,
    /// الاسم المستعار للوحدة
    pub alias: Option<String>,
    /// هل هو استيراد كامل (*)
    pub is_glob: bool,
}

impl Import {
    /// إنشاء استيراد جديد
    pub fn new(module_id: ModuleId) -> Self {
        Import {
            module_id,
            items: Vec::new(),
            alias: None,
            is_glob: false,
        }
    }

    /// إضافة عنصر مستورد
    pub fn add_item(mut self, item: ImportItem) -> Self {
        self.items.push(item);
        self
    }

    /// تعيين الاسم المستعار
    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }

    /// تعيين كاستيراد كامل
    pub fn as_glob(mut self) -> Self {
        self.is_glob = true;
        self
    }
}

/// عنصر مستورد
#[derive(Debug, Clone)]
pub struct ImportItem {
    /// الاسم الأصلي
    pub original_name: String,
    /// الاسم المحلي
    pub local_name: String,
}

impl ImportItem {
    /// إنشاء عنصر استيراد
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        ImportItem {
            original_name: name.clone(),
            local_name: name,
        }
    }

    /// إنشاء مع اسم محلي مختلف
    pub fn with_alias(original: impl Into<String>, local: impl Into<String>) -> Self {
        ImportItem {
            original_name: original.into(),
            local_name: local.into(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الوحدة المحملة
// ═══════════════════════════════════════════════════════════════════════════════

/// وحدة محملة
#[derive(Debug, Clone)]
pub struct Module {
    /// معرف الوحدة
    pub id: ModuleId,
    /// الصادرات
    pub exports: HashMap<String, Export>,
    /// الواردات
    pub imports: Vec<Import>,
    /// الكود المصدري
    pub source: Option<String>,
    /// حالة التحميل
    pub state: ModuleState,
}

impl Module {
    /// إنشاء وحدة جديدة
    pub fn new(id: ModuleId) -> Self {
        Module {
            id,
            exports: HashMap::new(),
            imports: Vec::new(),
            source: None,
            state: ModuleState::Unloaded,
        }
    }

    /// إضافة تصدير
    pub fn export(&mut self, name: impl Into<String>, export: Export) {
        self.exports.insert(name.into(), export);
    }

    /// التحقق من وجود تصدير
    pub fn has_export(&self, name: &str) -> bool {
        self.exports.contains_key(name)
    }

    /// الحصول على تصدير
    pub fn get_export(&self, name: &str) -> Option<&Export> {
        self.exports.get(name)
    }

    /// قائمة أسماء الصادرات
    pub fn export_names(&self) -> Vec<&String> {
        self.exports.keys().collect()
    }

    /// عدد الصادرات
    pub fn export_count(&self) -> usize {
        self.exports.len()
    }
}

/// حالة الوحدة
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleState {
    /// غير محملة
    Unloaded,
    /// قيد التحميل
    Loading,
    /// محملة جزئياً
    PartiallyLoaded,
    /// محملة بالكامل
    Loaded,
    /// خطأ في التحميل
    Error,
}

/// نوع التصدير
#[derive(Debug, Clone)]
pub struct Export {
    /// اسم التصدير
    pub name: String,
    /// نوع العنصر المصدر
    pub kind: ExportKind,
    /// هل هو التصدير الافتراضي
    pub is_default: bool,
}

impl Export {
    /// إنشاء تصدير جديد
    pub fn new(name: impl Into<String>, kind: ExportKind) -> Self {
        Export {
            name: name.into(),
            kind,
            is_default: false,
        }
    }

    /// تعيين كتصدير افتراضي
    pub fn as_default(mut self) -> Self {
        self.is_default = true;
        self
    }
}

/// نوع العنصر المصدر
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportKind {
    /// متغير
    Variable,
    /// ثابت
    Constant,
    /// دالة
    Function,
    /// صنف
    Class,
    /// وحدة
    Module,
    /// قيمة افتراضية
    Default,
}

// ═══════════════════════════════════════════════════════════════════════════════
// مدير الوحدات
// ═══════════════════════════════════════════════════════════════════════════════

/// مدير الوحدات
pub struct ModuleManager {
    /// الوحدات المحملة
    modules: HashMap<ModuleId, Module>,
    /// مسارات البحث
    search_paths: Vec<PathBuf>,
    /// الوحدات قيد التحميل (لكشف التبعيات الدائرية)
    loading_stack: Vec<ModuleId>,
    /// إعدادات
    config: ModuleConfig,
}

impl ModuleManager {
    /// إنشاء مدير وحدات جديد
    pub fn new() -> Self {
        ModuleManager {
            modules: HashMap::new(),
            search_paths: vec![
                PathBuf::from("."),
                PathBuf::from("libs"),
                PathBuf::from("stdlib"),
            ],
            loading_stack: Vec::new(),
            config: ModuleConfig::default(),
        }
    }

    /// إضافة مسار بحث
    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }

    /// تحميل وحدة
    pub fn load(&mut self, id: &ModuleId) -> Result<&Module, ModuleError> {
        // التحقق من التخزين المؤقت
        if self.modules.contains_key(id) {
            return Ok(self.modules.get(id).unwrap());
        }

        // التحقق من التبعيات الدائرية
        if self.loading_stack.contains(id) {
            return Err(ModuleError::CircularDependency(id.clone()));
        }

        // بدء التحميل
        self.loading_stack.push(id.clone());

        // تحميل الوحدة حسب نوعها
        let module = match id.kind {
            ModuleKind::Stdlib => self.load_stdlib(&id.name)?,
            ModuleKind::File => self.load_file(id.path.as_ref().unwrap())?,
            ModuleKind::Named => self.load_named(&id.name)?,
            ModuleKind::External => self.load_external(&id.name)?,
        };

        // إنهاء التحميل
        self.loading_stack.pop();

        // تخزين الوحدة
        self.modules.insert(id.clone(), module);

        Ok(self.modules.get(id).unwrap())
    }

    /// تحميل وحدة من المكتبة القياسية
    fn load_stdlib(&mut self, name: &str) -> Result<Module, ModuleError> {
        // التحقق من وجود الوحدة في المكتبة القياسية
        let source = get_stdlib_source(name)
            .ok_or_else(|| ModuleError::NotFound(ModuleId::stdlib(name)))?;

        // إنشاء الوحدة
        let mut module = Module::new(ModuleId::stdlib(name));
        module.source = Some(source);
        module.state = ModuleState::Loaded;

        // إضافة الصادرات الافتراضية
        self.populate_stdlib_exports(&mut module, name);

        Ok(module)
    }

    /// تحميل وحدة من ملف
    fn load_file(&mut self, path: &PathBuf) -> Result<Module, ModuleError> {
        // قراءة الملف
        let source = std::fs::read_to_string(path)
            .map_err(|e| ModuleError::IoError(path.clone(), e.to_string()))?;

        // إنشاء الوحدة
        let mut module = Module::new(ModuleId::from_path(path.clone()));
        module.source = Some(source);
        module.state = ModuleState::Loaded;

        // تحليل الصادرات والواردات
        self.parse_module(&mut module)?;

        Ok(module)
    }

    /// تحميل وحدة مسماة
    fn load_named(&mut self, name: &str) -> Result<Module, ModuleError> {
        // البحث في مسارات البحث
        for search_path in &self.search_paths {
            let possible_paths = vec![
                search_path.join(format!("{}.mrj", name)),
                search_path.join(name).join("mod.mrj"),
                search_path.join(name).join(format!("{}.mrj", name)),
            ];

            for path in possible_paths {
                if path.exists() {
                    return self.load_file(&path);
                }
            }
        }

        // محاولة تحميل من المكتبة القياسية
        if has_stdlib_module(name) {
            return self.load_stdlib(name);
        }

        Err(ModuleError::NotFound(ModuleId::from_name(name)))
    }

    /// تحميل وحدة خارجية
    fn load_external(&mut self, name: &str) -> Result<Module, ModuleError> {
        // البحث في مجلد المكتبات الخارجية
        let external_path = PathBuf::from(&self.config.external_path).join(name);

        if external_path.exists() {
            let mod_path = external_path.join("mod.mrj");
            if mod_path.exists() {
                return self.load_file(&mod_path);
            }

            let lib_path = external_path.join(format!("{}.mrj", name));
            if lib_path.exists() {
                return self.load_file(&lib_path);
            }
        }

        Err(ModuleError::NotFound(ModuleId::external(name)))
    }

    /// تحليل الوحدة لاستخراج الصادرات والواردات
    fn parse_module(&self, module: &mut Module) -> Result<(), ModuleError> {
        // استخراج المصدر أولاً لتجنب borrow conflict
        let source = module.source.clone();
        if let Some(ref source) = source {
            // بحث بسيط عن تصديرات
            for line in source.lines() {
                let trimmed = line.trim();

                // استيراد
                if trimmed.starts_with("استيراد ") || trimmed.starts_with("استورد ") {
                    self.parse_import_line(module, trimmed);
                }

                // تصدير دالة
                if trimmed.starts_with("دالة ") || trimmed.starts_with("تصدير دالة ") {
                    if let Some(name) = self.extract_function_name(trimmed) {
                        module.export(name.clone(), Export::new(name, ExportKind::Function));
                    }
                }

                // تصدير متغير
                if trimmed.starts_with("متغير ") || trimmed.starts_with("تصدير متغير ") {
                    if let Some(name) = self.extract_variable_name(trimmed) {
                        module.export(name.clone(), Export::new(name, ExportKind::Variable));
                    }
                }

                // تصدير ثابت
                if trimmed.starts_with("ثابت ") || trimmed.starts_with("تصدير ثابت ") {
                    if let Some(name) = self.extract_variable_name(trimmed) {
                        module.export(name.clone(), Export::new(name, ExportKind::Constant));
                    }
                }

                // تصدير صنف
                if trimmed.starts_with("صنف ") || trimmed.starts_with("تصدير صنف ") {
                    if let Some(name) = self.extract_class_name(trimmed) {
                        module.export(name.clone(), Export::new(name, ExportKind::Class));
                    }
                }
            }
        }

        Ok(())
    }

    /// تحليل سطر الاستيراد
    fn parse_import_line(&self, module: &mut Module, line: &str) {
        let line = line.trim_start_matches("استيراد ")
            .trim_start_matches("استورد ");

        // استيراد بسيط
        let import = Import::new(ModuleId::from_name(line.trim()));
        module.imports.push(import);
    }

    /// استخراج اسم الدالة من السطر
    fn extract_function_name(&self, line: &str) -> Option<String> {
        let line = line.trim_start_matches("تصدير ").trim_start_matches("دالة ");
        let parts: Vec<&str> = line.split('(').collect();
        parts.first().map(|s| s.trim().to_string())
    }

    /// استخراج اسم المتغير من السطر
    fn extract_variable_name(&self, line: &str) -> Option<String> {
        let line = line.trim_start_matches("تصدير ")
            .trim_start_matches("متغير ")
            .trim_start_matches("ثابت ");
        let parts: Vec<&str> = line.split('=').collect();
        parts.first().map(|s| s.trim().to_string())
    }

    /// استخراج اسم الصنف من السطر
    fn extract_class_name(&self, line: &str) -> Option<String> {
        let line = line.trim_start_matches("تصدير ").trim_start_matches("صنف ");
        let parts: Vec<&str> = line.split(':').collect();
        parts.first().map(|s| s.trim().to_string())
    }

    /// ملء صادرات المكتبة القياسية
    fn populate_stdlib_exports(&self, module: &mut Module, name: &str) {
        match name {
            "رياضيات" | "math" => {
                module.export("PI", Export::new("PI", ExportKind::Constant));
                module.export("E", Export::new("E", ExportKind::Constant));
                module.export("جذر", Export::new("جذر", ExportKind::Function));
                module.export("أس", Export::new("أس", ExportKind::Function));
                module.export("لوغاريتم", Export::new("لوغاريتم", ExportKind::Function));
                module.export("مطلق", Export::new("مطلق", ExportKind::Function));
                module.export("تقريب", Export::new("تقريب", ExportKind::Function));
                module.export("أعلى", Export::new("أعلى", ExportKind::Function));
                module.export("أدنى", Export::new("أدنى", ExportKind::Function));
                module.export("عشوائي", Export::new("عشوائي", ExportKind::Function));
                module.export("جيب", Export::new("جيب", ExportKind::Function));
                module.export("جيب_تمام", Export::new("جيب_تمام", ExportKind::Function));
                module.export("ظل", Export::new("ظل", ExportKind::Function));
            }
            "نصوص" | "string" => {
                module.export("طول", Export::new("طول", ExportKind::Function));
                module.export("جزء", Export::new("جزء", ExportKind::Function));
                module.export("بحث", Export::new("بحث", ExportKind::Function));
                module.export("استبدال", Export::new("استبدال", ExportKind::Function));
                module.export("تقسيم", Export::new("تقسيم", ExportKind::Function));
                module.export("دمج", Export::new("دمج", ExportKind::Function));
                module.export("أحرف_كبيرة", Export::new("أحرف_كبيرة", ExportKind::Function));
                module.export("أحرف_صغيرة", Export::new("أحرف_صغيرة", ExportKind::Function));
                module.export("تقليم", Export::new("تقليم", ExportKind::Function));
                module.export("يبدأ_بـ", Export::new("يبدأ_بـ", ExportKind::Function));
                module.export("ينتهي_بـ", Export::new("ينتهي_بـ", ExportKind::Function));
                module.export("يحتوي", Export::new("يحتوي", ExportKind::Function));
            }
            "قوائم" | "list" => {
                module.export("إنشاء", Export::new("إنشاء", ExportKind::Function));
                module.export("طول", Export::new("طول", ExportKind::Function));
                module.export("أضف", Export::new("أضف", ExportKind::Function));
                module.export("أدرج", Export::new("أدرج", ExportKind::Function));
                module.export("احذف", Export::new("احذف", ExportKind::Function));
                module.export("رتب", Export::new("رتب", ExportKind::Function));
                module.export("عكس", Export::new("عكس", ExportKind::Function));
                module.export("خريطة", Export::new("خريطة", ExportKind::Function));
                module.export("فلتر", Export::new("فلتر", ExportKind::Function));
                module.export("اختزال", Export::new("اختزال", ExportKind::Function));
                module.export("بحث", Export::new("بحث", ExportKind::Function));
                module.export("كل", Export::new("كل", ExportKind::Function));
                module.export("بعض", Export::new("بعض", ExportKind::Function));
            }
            "ملفات" | "file" => {
                module.export("اقرأ", Export::new("اقرأ", ExportKind::Function));
                module.export("اكتب", Export::new("اكتب", ExportKind::Function));
                module.export("أضف", Export::new("أضف", ExportKind::Function));
                module.export("احذف", Export::new("احذف", ExportKind::Function));
                module.export("وجود", Export::new("وجود", ExportKind::Function));
                module.export("أنشئ", Export::new("أنشئ", ExportKind::Function));
                module.export("محتوى", Export::new("محتوى", ExportKind::Function));
            }
            "شبكة" | "network" => {
                module.export("طلب", Export::new("طلب", ExportKind::Function));
                module.export("احصل", Export::new("احصل", ExportKind::Function));
                module.export("أرسل", Export::new("أرسل", ExportKind::Function));
                module.export("رأس", Export::new("رأس", ExportKind::Function));
            }
            "تشفير" | "crypto" => {
                module.export("تجزئة", Export::new("تجزئة", ExportKind::Function));
                module.export("مشفر", Export::new("مشفر", ExportKind::Function));
                module.export("فك_التشفير", Export::new("فك_التشفير", ExportKind::Function));
                module.export("توقيع", Export::new("توقيع", ExportKind::Function));
                module.export("تحقق", Export::new("تحقق", ExportKind::Function));
            }
            "تاريخ" | "datetime" => {
                module.export("الآن", Export::new("الآن", ExportKind::Function));
                module.export("تاريخ", Export::new("تاريخ", ExportKind::Function));
                module.export("وقت", Export::new("وقت", ExportKind::Function));
                module.export("صيغة", Export::new("صيغة", ExportKind::Function));
            }
            "json" => {
                module.export("تشفير", Export::new("تشفير", ExportKind::Function));
                module.export("فك_التشفير", Export::new("فك_التشفير", ExportKind::Function));
            }
            "regex" => {
                module.export("إنشاء", Export::new("إنشاء", ExportKind::Function));
                module.export("مطابقة", Export::new("مطابقة", ExportKind::Function));
                module.export("بحث", Export::new("بحث", ExportKind::Function));
                module.export("استبدال", Export::new("استبدال", ExportKind::Function));
            }
            _ => {}
        }
    }

    /// الحصول على وحدة
    pub fn get(&self, id: &ModuleId) -> Option<&Module> {
        self.modules.get(id)
    }

    /// التحقق من وجود وحدة
    pub fn has(&self, id: &ModuleId) -> bool {
        self.modules.contains_key(id)
    }

    /// قائمة الوحدات المحملة
    pub fn loaded_modules(&self) -> Vec<&ModuleId> {
        self.modules.keys().collect()
    }

    /// عدد الوحدات المحملة
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }
}

impl Default for ModuleManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// إعدادات الوحدات
// ═══════════════════════════════════════════════════════════════════════════════

/// إعدادات نظام الوحدات
#[derive(Debug, Clone)]
pub struct ModuleConfig {
    /// السماح بالاستيراد الدائري
    pub allow_circular: bool,
    /// الحد الأقصى لعمق الاستيراد
    pub max_depth: usize,
    /// مسار المكتبات الخارجية
    pub external_path: PathBuf,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        ModuleConfig {
            allow_circular: false,
            max_depth: 100,
            external_path: PathBuf::from("external"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// أخطاء الوحدات
// ═══════════════════════════════════════════════════════════════════════════════

/// خطأ في الوحدة
#[derive(Debug, Clone)]
pub enum ModuleError {
    /// الوحدة غير موجودة
    NotFound(ModuleId),
    /// خطأ في الإدخال/الإخراج
    IoError(PathBuf, String),
    /// تبعية دائرية
    CircularDependency(ModuleId),
    /// خطأ في التحليل
    ParseError(String),
    /// خطأ في التحميل
    LoadError(String),
    /// تصدير غير موجود
    ExportNotFound(String),
    /// خطأ في الاستيراد
    ImportError(String),
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleError::NotFound(id) => {
                write!(f, "الوحدة '{}' غير موجودة", id.name)
            }
            ModuleError::IoError(path, msg) => {
                write!(f, "خطأ في قراءة '{}': {}", path.display(), msg)
            }
            ModuleError::CircularDependency(id) => {
                write!(f, "تبعية دائرية في الوحدة '{}'", id.name)
            }
            ModuleError::ParseError(msg) => {
                write!(f, "خطأ في تحليل الوحدة: {}", msg)
            }
            ModuleError::LoadError(msg) => {
                write!(f, "خطأ في تحميل الوحدة: {}", msg)
            }
            ModuleError::ExportNotFound(name) => {
                write!(f, "التصدير '{}' غير موجود", name)
            }
            ModuleError::ImportError(msg) => {
                write!(f, "خطأ في الاستيراد: {}", msg)
            }
        }
    }
}

impl std::error::Error for ModuleError {}

// ═══════════════════════════════════════════════════════════════════════════════
// المكتبة القياسية المدمجة
// ═══════════════════════════════════════════════════════════════════════════════

/// التحقق من وجود وحدة في المكتبة القياسية
pub fn has_stdlib_module(name: &str) -> bool {
    matches!(
        name,
        "رياضيات" | "math" |
        "نصوص" | "string" |
        "قوائم" | "list" |
        "ملفات" | "file" |
        "شبكة" | "network" |
        "تشفير" | "crypto" |
        "تاريخ" | "datetime" |
        "json" | "regex"
    )
}

/// الحصول على مصدر وحدة المكتبة القياسية
pub fn get_stdlib_source(name: &str) -> Option<String> {
    match name {
        "رياضيات" | "math" => Some(include_str!("stdlib/math.mrj").to_string()),
        "نصوص" | "string" => Some(include_str!("stdlib/string.mrj").to_string()),
        "قوائم" | "list" => Some(include_str!("stdlib/list.mrj").to_string()),
        "ملفات" | "file" => Some(include_str!("stdlib/file.mrj").to_string()),
        "شبكة" | "network" => Some(include_str!("stdlib/network.mrj").to_string()),
        "تشفير" | "crypto" => Some(include_str!("stdlib/crypto.mrj").to_string()),
        "تاريخ" | "datetime" => Some(include_str!("stdlib/datetime.mrj").to_string()),
        "json" => Some(include_str!("stdlib/json.mrj").to_string()),
        "regex" => Some(include_str!("stdlib/regex.mrj").to_string()),
        _ => None,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_id_creation() {
        let id = ModuleId::from_name("رياضيات");
        assert_eq!(id.name, "رياضيات");
        assert_eq!(id.kind, ModuleKind::Named);

        let id = ModuleId::stdlib("math");
        assert_eq!(id.kind, ModuleKind::Stdlib);
    }

    #[test]
    fn test_module_creation() {
        let mut module = Module::new(ModuleId::stdlib("رياضيات"));
        module.export("PI", Export::new("PI", ExportKind::Constant));

        assert!(module.has_export("PI"));
        assert!(!module.has_export("E"));
        assert_eq!(module.export_count(), 1);
    }

    #[test]
    fn test_import_item() {
        let item = ImportItem::new("جذر");
        assert_eq!(item.original_name, "جذر");
        assert_eq!(item.local_name, "جذر");

        let item = ImportItem::with_alias("جذر", "sqrt");
        assert_eq!(item.original_name, "جذر");
        assert_eq!(item.local_name, "sqrt");
    }

    #[test]
    fn test_import_builder() {
        let import = Import::new(ModuleId::stdlib("رياضيات"))
            .add_item(ImportItem::new("جذر"))
            .add_item(ImportItem::with_alias("أس", "pow"))
            .with_alias("ر");

        assert_eq!(import.items.len(), 2);
        assert_eq!(import.alias, Some("ر".to_string()));
    }

    #[test]
    fn test_module_manager() {
        let mut manager = ModuleManager::new();

        // تحميل وحدة من المكتبة القياسية
        let result = manager.load(&ModuleId::stdlib("رياضيات"));
        assert!(result.is_ok());

        let module = result.unwrap();
        assert!(module.has_export("PI"));
        assert!(module.has_export("جذر"));
    }

    #[test]
    fn test_stdlib_detection() {
        assert!(has_stdlib_module("رياضيات"));
        assert!(has_stdlib_module("math"));
        assert!(has_stdlib_module("json"));
        assert!(!has_stdlib_module("unknown_module"));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut manager = ModuleManager::new();

        // محاولة تحميل وحدة غير موجودة
        let result = manager.load(&ModuleId::from_name("nonexistent_module"));
        assert!(result.is_err());

        match result {
            Err(ModuleError::NotFound(id)) => assert_eq!(id.name, "nonexistent_module"),
            _ => panic!("Expected NotFound error"),
        }
    }
}
