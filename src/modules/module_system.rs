// ═══════════════════════════════════════════════════════════════════════════════
// Module System - نظام الوحدات
// ═══════════════════════════════════════════════════════════════════════════════
// يتيح تنظيم الكود في وحدات قابلة لإعادة الاستخدام
// دعم الاستيراد والتصدير
// إدارة التبعيات
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::bytecode::Chunk;
use crate::interpreter::value::Value;

// ═══════════════════════════════════════════════════════════════════════════════
// معرف الوحدة
// ═══════════════════════════════════════════════════════════════════════════════

/// معرف فريد لوحدة
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ModuleId {
    /// اسم الوحدة
    pub name: String,
    /// الإصدار
    pub version: String,
    /// المسار
    pub path: PathBuf,
}

impl ModuleId {
    pub fn new(name: &str, version: &str, path: &Path) -> Self {
        ModuleId {
            name: name.to_string(),
            version: version.to_string(),
            path: path.to_path_buf(),
        }
    }
    
    /// معرف من مسار ملف
    pub fn from_path(path: &Path) -> Self {
        let name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        ModuleId {
            name,
            version: "0.0.0".to_string(),
            path: path.to_path_buf(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع الاستيراد
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع الاستيراد
#[derive(Debug, Clone)]
pub enum ImportKind {
    /// استيراد كل شيء (import *)
    All {
        /// الوحدة المستوردة منها
        module: ModuleId,
        /// الاسم المستعار (اختياري)
        alias: Option<String>,
    },
    
    /// استيراد عناصر محددة
    Named {
        /// العناصر المستوردة (اسم_أصلي -> اسم_محلي)
        items: Vec<(String, String)>,
        /// الوحدة المستوردة منها
        module: ModuleId,
    },
    
    /// استيراد افتراضي
    Default {
        /// الاسم المحلي
        local_name: String,
        /// الوحدة المستوردة منها
        module: ModuleId,
    },
    
    /// استيراد الوحدة كاملة ككائن
    Module {
        /// الاسم المحلي
        local_name: String,
        /// الوحدة المستوردة
        module: ModuleId,
    },
}

/// بيان استيراد
#[derive(Debug, Clone)]
pub struct ImportStatement {
    /// نوع الاستيراد
    pub kind: ImportKind,
    /// موقع في الكود
    pub location: SourceLocation,
}

/// موقع في الكود المصدري
#[derive(Debug, Clone, Default)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع التصدير
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع التصدير
#[derive(Debug, Clone)]
pub enum ExportKind {
    /// تصدير قيمة
    Value {
        name: String,
        value: Value,
    },
    
    /// تصدير دالة
    Function {
        name: String,
        chunk: Chunk,
        params: Vec<String>,
    },
    
    /// تصدير ثابت
    Constant {
        name: String,
        value: Value,
    },
    
    /// تصدير نوع (للاستخدام المستقبلي)
    Type {
        name: String,
        type_def: TypeDefinition,
    },
    
    /// إعادة تصدير من وحدة أخرى
    Reexport {
        name: String,
        source_module: ModuleId,
        source_name: String,
    },
}

/// تعريف نوع
#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub name: String,
    pub fields: Vec<(String, String)>, // (name, type)
}

/// بيان تصدير
#[derive(Debug, Clone)]
pub struct ExportStatement {
    /// نوع التصدير
    pub kind: ExportKind,
    /// هل هو التصدير الافتراضي
    pub is_default: bool,
    /// الموقع في الكود
    pub location: SourceLocation,
}

// ═══════════════════════════════════════════════════════════════════════════════
// الوحدة
// ═══════════════════════════════════════════════════════════════════════════════

/// وحدة محمّلة
#[derive(Debug)]
pub struct Module {
    /// معرف الوحدة
    pub id: ModuleId,
    
    /// الصادرات
    pub exports: HashMap<String, ExportKind>,
    
    /// التصدير الافتراضي
    pub default_export: Option<String>,
    
    /// الواردات
    pub imports: Vec<ImportStatement>,
    
    /// التبعيات
    pub dependencies: Vec<ModuleId>,
    
    /// مسار الملف المصدري
    pub source_path: PathBuf,
    
    /// الكود المترجم
    pub compiled_chunk: Option<Chunk>,
    
    /// هل تم تحميلها
    pub loaded: bool,
    
    /// إحصائيات الوحدة
    pub stats: ModuleStats,
}

/// إحصائيات الوحدة
#[derive(Debug, Default, Clone)]
pub struct ModuleStats {
    pub load_time_ms: u64,
    pub compile_time_ms: u64,
    pub size_bytes: usize,
    pub export_count: usize,
    pub import_count: usize,
}

impl Module {
    /// إنشاء وحدة جديدة
    pub fn new(id: ModuleId) -> Self {
        Module {
            id,
            exports: HashMap::new(),
            default_export: None,
            imports: Vec::new(),
            dependencies: Vec::new(),
            source_path: PathBuf::new(),
            compiled_chunk: None,
            loaded: false,
            stats: ModuleStats::default(),
        }
    }
    
    /// إضافة تصدير
    pub fn add_export(&mut self, name: &str, kind: ExportKind, is_default: bool) {
        if is_default {
            self.default_export = Some(name.to_string());
        }
        self.exports.insert(name.to_string(), kind);
    }
    
    /// الحصول على تصدير
    pub fn get_export(&self, name: &str) -> Option<&ExportKind> {
        self.exports.get(name)
    }
    
    /// الحصول على التصدير الافتراضي
    pub fn get_default_export(&self) -> Option<&ExportKind> {
        self.default_export.as_ref()
            .and_then(|name| self.exports.get(name))
    }
    
    /// قائمة أسماء الصادرات
    pub fn export_names(&self) -> Vec<&str> {
        self.exports.keys().map(|s| s.as_str()).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// مدير الوحدات
// ═══════════════════════════════════════════════════════════════════════════════

/// خطأ في الوحدة
#[derive(Debug, Clone)]
pub enum ModuleError {
    /// الوحدة غير موجودة
    NotFound(String),
    
    /// تبعية دائرية
    CircularDependency {
        module: String,
        chain: Vec<String>,
    },
    
    /// خطأ في التحليل
    ParseError {
        file: String,
        message: String,
        line: usize,
    },
    
    /// خطأ في الترجمة
    CompileError {
        module: String,
        message: String,
    },
    
    /// تصدير غير موجود
    ExportNotFound {
        module: String,
        export: String,
    },
    
    /// خطأ I/O
    IoError(String),
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleError::NotFound(name) => write!(f, "الوحدة '{}' غير موجودة", name),
            ModuleError::CircularDependency { module, chain } => {
                write!(f, "تبعية دائرية: {} -> {}", chain.join(" -> "), module)
            }
            ModuleError::ParseError { file, message, line } => {
                write!(f, "خطأ في تحليل '{}' السطر {}: {}", file, line, message)
            }
            ModuleError::CompileError { module, message } => {
                write!(f, "خطأ في ترجمة '{}': {}", module, message)
            }
            ModuleError::ExportNotFound { module, export } => {
                write!(f, "التصدير '{}' غير موجود في '{}'", export, module)
            }
            ModuleError::IoError(msg) => write!(f, "خطأ I/O: {}", msg),
        }
    }
}

impl std::error::Error for ModuleError {}

/// نتيجة تحميل وحدة
#[derive(Debug)]
pub struct LoadResult {
    pub module: Module,
    pub warnings: Vec<String>,
}

/// مدير الوحدات
pub struct ModuleManager {
    /// الوحدات المحمّلة
    modules: HashMap<String, Module>,
    
    /// مسارات البحث عن الوحدات
    search_paths: Vec<PathBuf>,
    
    /// كاش الوحدات
    cache: ModuleCache,
    
    /// سلسلة التحليل (لكشف التبعيات الدائرية)
    resolution_stack: Vec<String>,
    
    /// الإحصائيات
    stats: ModuleManagerStats,
}

/// كاش الوحدات
#[derive(Debug, Default)]
struct ModuleCache {
    path_to_id: HashMap<PathBuf, String>,
    compiled: HashMap<String, Chunk>,
}

/// إحصائيات مدير الوحدات
#[derive(Debug, Default, Clone)]
pub struct ModuleManagerStats {
    pub modules_loaded: u64,
    pub total_load_time_ms: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub circular_dependencies_caught: u64,
}

impl ModuleManager {
    /// إنشاء مدير وحدات جديد
    pub fn new() -> Self {
        ModuleManager {
            modules: HashMap::new(),
            search_paths: vec![
                PathBuf::from("."),
                PathBuf::from("modules"),
                PathBuf::from("lib"),
            ],
            cache: ModuleCache::default(),
            resolution_stack: Vec::new(),
            stats: ModuleManagerStats::default(),
        }
    }
    
    /// إضافة مسار بحث
    pub fn add_search_path(&mut self, path: &Path) {
        self.search_paths.push(path.to_path_buf());
    }
    
    /// تحميل وحدة
    pub fn load_module(&mut self, name: &str) -> Result<&Module, ModuleError> {
        let start = std::time::Instant::now();
        
        // التحقق من الكاش
        if let Some(module) = self.modules.get(name) {
            self.stats.cache_hits += 1;
            return Ok(module);
        }
        
        // التحقق من التبعية الدائرية
        if self.resolution_stack.contains(&name.to_string()) {
            self.stats.circular_dependencies_caught += 1;
            return Err(ModuleError::CircularDependency {
                module: name.to_string(),
                chain: self.resolution_stack.clone(),
            });
        }
        
        self.stats.cache_misses += 1;
        self.resolution_stack.push(name.to_string());
        
        // البحث عن الملف
        let path = self.resolve_module_path(name)?;
        
        // قراءة الملف
        let source = std::fs::read_to_string(&path)
            .map_err(|e| ModuleError::IoError(e.to_string()))?;
        
        // تحليل الكود
        let program = crate::parser::Parser::parse(&source)
            .map_err(|e| ModuleError::ParseError {
                file: path.display().to_string(),
                message: e.message,
                line: e.position.line,
            })?;
        
        // استخراج الواردات والصادرات
        let imports = self.extract_imports(&program)?;
        let exports = self.extract_exports(&program);
        
        // ترجمة الكود
        let chunk = crate::bytecode::Compiler::compile(&program)
            .map_err(|e| ModuleError::CompileError {
                module: name.to_string(),
                message: e.errors.join("\n"),
            })?;
        
        // إنشاء الوحدة
        let id = ModuleId::from_path(&path);
        let mut module = Module::new(id);
        module.imports = imports;
        module.exports = exports;
        module.source_path = path;
        module.compiled_chunk = Some(chunk);
        module.loaded = true;
        module.stats.load_time_ms = start.elapsed().as_millis() as u64;
        
        self.resolution_stack.pop();
        self.modules.insert(name.to_string(), module);
        self.stats.modules_loaded += 1;
        self.stats.total_load_time_ms += start.elapsed().as_millis() as u64;
        
        Ok(self.modules.get(name).unwrap())
    }
    
    /// البحث عن مسار الوحدة
    fn resolve_module_path(&self, name: &str) -> Result<PathBuf, ModuleError> {
        // أسماء الملفات المحتملة
        let candidates = vec![
            format!("{}.mrj", name),
            format!("{}.mrj", name.replace('.', "/")),
            format!("mod.mrj"),
        ];
        
        for search_path in &self.search_paths {
            for candidate in &candidates {
                let full_path = search_path.join(candidate);
                if full_path.exists() {
                    return Ok(full_path);
                }
            }
        }
        
        Err(ModuleError::NotFound(name.to_string()))
    }
    
    /// استخراج الواردات من البرنامج
    fn extract_imports(&mut self, _program: &crate::parser::ast::Program) -> Result<Vec<ImportStatement>, ModuleError> {
        // TODO: تحليل AST لاستخراج بيانات الاستيراد
        Ok(Vec::new())
    }
    
    /// استخراج الصادرات من البرنامج
    fn extract_exports(&self, _program: &crate::parser::ast::Program) -> HashMap<String, ExportKind> {
        // TODO: تحليل AST لاستخراج بيانات التصدير
        HashMap::new()
    }
    
    /// الحصول على وحدة
    pub fn get_module(&self, name: &str) -> Option<&Module> {
        self.modules.get(name)
    }
    
    /// الحصول على وحدة للتعديل
    pub fn get_module_mut(&mut self, name: &str) -> Option<&mut Module> {
        self.modules.get_mut(name)
    }
    
    /// تفريغ كاش الوحدات
    pub fn clear_cache(&mut self) {
        self.cache.path_to_id.clear();
        self.cache.compiled.clear();
    }
    
    /// قائمة الوحدات المحمّلة
    pub fn loaded_modules(&self) -> Vec<&str> {
        self.modules.keys().map(|s| s.as_str()).collect()
    }
    
    /// الإحصائيات
    pub fn stats(&self) -> &ModuleManagerStats {
        &self.stats
    }
    
    /// طباعة تقرير
    pub fn print_report(&self) {
        println!();
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║                📦 تقرير نظام الوحدات                              ║");
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ الوحدات المحمّلة:      {:>15}                       ║", self.stats.modules_loaded);
        println!("║ وقت التحميل الكلي:    {:>15} مللي ثانية           ║", self.stats.total_load_time_ms);
        println!("║ ضربات الكاش:          {:>15}                       ║", self.stats.cache_hits);
        println!("║ أخطاء الكاش:          {:>15}                       ║", self.stats.cache_misses);
        println!("║ تبعيات دائرية:        {:>15}                       ║", self.stats.circular_dependencies_caught);
        println!("╠══════════════════════════════════════════════════════════════════╣");
        
        if !self.modules.is_empty() {
            println!("║ 📚 الوحدات:                                                       ║");
            for (name, module) in &self.modules {
                println!("║   ├── {} ({}) - {} صادرات            ║",
                    name,
                    module.id.version,
                    module.exports.len()
                );
            }
        }
        
        println!("╚══════════════════════════════════════════════════════════════════╝");
    }
}

impl Default for ModuleManager {
    fn default() -> Self {
        Self::new()
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
        let id = ModuleId::new("math", "1.0.0", Path::new("modules/math.mrj"));
        assert_eq!(id.name, "math");
        assert_eq!(id.version, "1.0.0");
    }

    #[test]
    fn test_module_creation() {
        let id = ModuleId::new("test", "0.0.0", Path::new("test.mrj"));
        let module = Module::new(id);
        
        assert!(module.exports.is_empty());
        assert!(!module.loaded);
    }

    #[test]
    fn test_module_exports() {
        let id = ModuleId::new("test", "0.0.0", Path::new("test.mrj"));
        let mut module = Module::new(id);
        
        module.add_export("pi", ExportKind::Constant {
            name: "pi".to_string(),
            value: Value::Number(3.14159),
        }, false);
        
        assert!(module.get_export("pi").is_some());
        assert_eq!(module.exports.len(), 1);
    }

    #[test]
    fn test_module_manager_creation() {
        let manager = ModuleManager::new();
        assert_eq!(manager.stats.modules_loaded, 0);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut manager = ModuleManager::new();
        
        // محاكاة تبعية دائرية
        manager.resolution_stack.push("a".to_string());
        manager.resolution_stack.push("b".to_string());
        
        let result = manager.load_module("a");
        assert!(matches!(result, Err(ModuleError::CircularDependency { .. })));
    }
}
