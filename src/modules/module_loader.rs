// ═══════════════════════════════════════════════════════════════════════════════
// Module Loader - محمل الوحدات
// ═══════════════════════════════════════════════════════════════════════════════
// يحمل وينفذ ملفات .mrj كوحدات قابلة للاستيراد
// يدعم المكتبات القياسية والمكتبات الخارجية
// ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::interpreter::value::{Environment, SharedValue, Value};
use crate::interpreter::Interpreter;
use crate::parser::Parser;

// ═══════════════════════════════════════════════════════════════════════════════
// وحدة محمّلة
// ═══════════════════════════════════════════════════════════════════════════════

/// وحدة محمّلة مع صادراتها
#[derive(Debug, Clone)]
pub struct LoadedModule {
    /// اسم الوحدة
    pub name: String,
    
    /// مسار الملف
    pub path: PathBuf,
    
    /// الصادرات (الاسم -> القيمة)
    pub exports: HashMap<String, Rc<RefCell<Value>>>,
    
    /// التصدير الافتراضي
    pub default_export: Option<String>,
    
    /// هل تم تحميلها
    pub loaded: bool,
}

impl LoadedModule {
    pub fn new(name: &str, path: PathBuf) -> Self {
        LoadedModule {
            name: name.to_string(),
            path,
            exports: HashMap::new(),
            default_export: None,
            loaded: false,
        }
    }
    
    /// إضافة تصدير
    pub fn add_export(&mut self, name: &str, value: Rc<RefCell<Value>>, is_default: bool) {
        if is_default {
            self.default_export = Some(name.to_string());
        }
        self.exports.insert(name.to_string(), value);
    }
    
    /// الحصول على تصدير
    pub fn get_export(&self, name: &str) -> Option<Rc<RefCell<Value>>> {
        self.exports.get(name).cloned()
    }
    
    /// الحصول على جميع الصادرات كقاموس
    pub fn get_all_exports(&self) -> HashMap<String, Rc<RefCell<Value>>> {
        self.exports.clone()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// خطأ التحميل
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum LoadError {
    /// الملف غير موجود
    NotFound(String),
    
    /// خطأ في القراءة
    ReadError(String),
    
    /// خطأ في التحليل
    ParseError { file: String, message: String, line: usize },
    
    /// خطأ في التنفيذ
    RuntimeError(String),
    
    /// تبعية دائرية
    CircularDependency { module: String, chain: Vec<String> },
    
    /// تصدير غير موجود
    ExportNotFound { module: String, export: String },
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::NotFound(name) => write!(f, "الوحدة '{}' غير موجودة", name),
            LoadError::ReadError(msg) => write!(f, "خطأ في قراءة الملف: {}", msg),
            LoadError::ParseError { file, message, line } => {
                write!(f, "خطأ في تحليل '{}' السطر {}: {}", file, line, message)
            }
            LoadError::RuntimeError(msg) => write!(f, "خطأ في التنفيذ: {}", msg),
            LoadError::CircularDependency { module, chain } => {
                write!(f, "تبعية دائرية: {} -> {}", chain.join(" -> "), module)
            }
            LoadError::ExportNotFound { module, export } => {
                write!(f, "التصدير '{}' غير موجود في '{}'", export, module)
            }
        }
    }
}

impl std::error::Error for LoadError {}

// ═══════════════════════════════════════════════════════════════════════════════
// محمل الوحدات
// ═══════════════════════════════════════════════════════════════════════════════

pub struct ModuleLoader {
    /// الوحدات المحمّلة
    loaded_modules: HashMap<String, LoadedModule>,
    
    /// مسارات البحث
    search_paths: Vec<PathBuf>,
    
    /// المكتبات المدمجة (Built-in)
    builtin_modules: HashMap<String, HashMap<String, Rc<RefCell<Value>>>>,
    
    /// سلسلة التحميل (لكشف التبعيات الدائرية)
    loading_stack: Vec<String>,
    
    /// الإحصائيات
    stats: LoaderStats,
}

#[derive(Debug, Default, Clone)]
pub struct LoaderStats {
    pub modules_loaded: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub load_errors: usize,
}

impl ModuleLoader {
    /// إنشاء محمل وحدات جديد
    pub fn new() -> Self {
        let mut loader = ModuleLoader {
            loaded_modules: HashMap::new(),
            search_paths: Vec::new(),
            builtin_modules: HashMap::new(),
            loading_stack: Vec::new(),
            stats: LoaderStats::default(),
        };
        
        // إضافة مسارات البحث الافتراضية
        loader.add_default_search_paths();
        
        // تسجيل المكتبات المدمجة
        loader.register_builtin_modules();
        
        loader
    }
    
    /// إضافة مسارات البحث الافتراضية
    fn add_default_search_paths(&mut self) {
        // المسار الحالي
        self.search_paths.push(PathBuf::from("."));
        
        // مجلد modules
        self.search_paths.push(PathBuf::from("modules"));
        
        // مجلد lib
        self.search_paths.push(PathBuf::from("lib"));
        
        // مجلد stdlib داخل المشروع
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // stdlib بجانب الملف التنفيذي
                self.search_paths.push(exe_dir.join("stdlib"));
                self.search_paths.push(exe_dir.join("modules").join("stdlib"));
            }
        }
        
        // من متغير البيئة
        if let Ok(stdlib_path) = std::env::var("ALMARJAA_STDLIB") {
            self.search_paths.push(PathBuf::from(stdlib_path));
        }
        
        // المسار النسبي من الكود المصدري
        self.search_paths.push(PathBuf::from("src/modules/stdlib"));
    }
    
    /// إضافة مسار بحث
    pub fn add_search_path(&mut self, path: &Path) {
        if !self.search_paths.contains(&path.to_path_buf()) {
            self.search_paths.push(path.to_path_buf());
        }
    }
    
    /// تسجيل المكتبات المدمجة
    fn register_builtin_modules(&mut self) {
        // json - مكتبة JSON
        let json_exports = self.create_json_module();
        self.builtin_modules.insert("json".to_string(), json_exports);
        
        // math - مكتبة الرياضيات
        let math_exports = self.create_math_module();
        self.builtin_modules.insert("math".to_string(), math_exports);
        
        // datetime - مكتبة التاريخ والوقت
        let datetime_exports = self.create_datetime_module();
        self.builtin_modules.insert("datetime".to_string(), datetime_exports);
        
        // regex - مكتبة التعابير النمطية
        let regex_exports = self.create_regex_module();
        self.builtin_modules.insert("regex".to_string(), regex_exports);
        
        // file - مكتبة الملفات
        let file_exports = self.create_file_module();
        self.builtin_modules.insert("file".to_string(), file_exports);
    }
    
    /// تحميل وحدة (يعيد الوحدة بالقيمة لتجنب مشاكل الاستعارة)
    pub fn load(&mut self, name: &str) -> Result<LoadedModule, LoadError> {
        // التحقق من الكاش
        if let Some(module) = self.loaded_modules.get(name) {
            self.stats.cache_hits += 1;
            return Ok(module.clone());
        }
        
        // التحقق من التبعية الدائرية
        if self.loading_stack.contains(&name.to_string()) {
            return Err(LoadError::CircularDependency {
                module: name.to_string(),
                chain: self.loading_stack.clone(),
            });
        }
        
        self.stats.cache_misses += 1;
        
        // التحقق من المكتبات المدمجة أولاً
        if let Some(exports) = self.builtin_modules.get(name) {
            let mut module = LoadedModule::new(name, PathBuf::from(format!("builtin:{}", name)));
            for (export_name, value) in exports {
                module.add_export(export_name, Rc::clone(value), false);
            }
            module.loaded = true;
            let result = module.clone();
            self.loaded_modules.insert(name.to_string(), module);
            self.stats.modules_loaded += 1;
            return Ok(result);
        }
        
        // البحث عن ملف الوحدة
        let path = self.resolve_module_path(name)?;
        
        // قراءة الملف
        let source = std::fs::read_to_string(&path)
            .map_err(|e| LoadError::ReadError(format!("{}: {}", path.display(), e)))?;
        
        // تحليل الكود
        let program = Parser::parse(&source)
            .map_err(|e| LoadError::ParseError {
                file: path.display().to_string(),
                message: e.message,
                line: e.line,
            })?;
        
        // إنشاء بيئة للوحدة
        let module_env = Rc::new(RefCell::new(Environment::new()));
        
        // تنفيذ الوحدة
        self.loading_stack.push(name.to_string());
        
        let result = self.execute_module(&program, &module_env);
        
        self.loading_stack.pop();
        
        if let Err(e) = result {
            self.stats.load_errors += 1;
            return Err(e);
        }
        
        // استخراج الصادرات
        let exports = self.extract_exports(&program, &module_env);
        
        let mut module = LoadedModule::new(name, path);
        for (export_name, value) in exports {
            module.add_export(&export_name, value, false);
        }
        module.loaded = true;
        
        let result_module = module.clone();
        self.loaded_modules.insert(name.to_string(), module);
        self.stats.modules_loaded += 1;
        
        Ok(result_module)
    }
    
    /// البحث عن مسار الوحدة
    fn resolve_module_path(&self, name: &str) -> Result<PathBuf, LoadError> {
        // أسماء الملفات المحتملة
        let candidates = vec![
            format!("{}.mrj", name),
            format!("{}/mod.mrj", name),
            format!("{}/index.mrj", name),
            format!("lib{}.mrj", name),
        ];
        
        for search_path in &self.search_paths {
            for candidate in &candidates {
                let full_path = search_path.join(&candidate);
                if full_path.exists() {
                    return Ok(full_path);
                }
            }
        }
        
        Err(LoadError::NotFound(name.to_string()))
    }
    
    /// تنفيذ وحدة
    fn execute_module(
        &self,
        program: &crate::parser::ast::Program,
        env: &Rc<RefCell<Environment>>,
    ) -> Result<(), LoadError> {
        // إنشاء مترجم مؤقت
        let mut interpreter = Interpreter::new();
        
        // تنفيذ الكود
        interpreter.interpret(program)
            .map_err(|e| LoadError::RuntimeError(e.message))?;
        
        // نسخ المتغيرات إلى بيئة الوحدة
        let vars: Vec<(String, SharedValue)> = interpreter.environment.borrow().variables
            .iter()
            .map(|(k, v)| (k.clone(), Rc::clone(v)))
            .collect();
        
        for (name, value) in vars.into_iter() {
            env.borrow_mut().define(name.as_str(), (*value.borrow()).clone(), false);
        }
        
        Ok(())
    }
    
    /// استخراج الصادرات من الوحدة
    fn extract_exports(
        &self,
        program: &crate::parser::ast::Program,
        env: &Rc<RefCell<Environment>>,
    ) -> HashMap<String, Rc<RefCell<Value>>> {
        let mut exports = HashMap::new();
        
        for stmt in &program.statements {
            match stmt {
                crate::parser::ast::Stmt::Export { name, value, .. } => {
                    if let Some(val) = env.borrow().get(name) {
                        exports.insert(name.clone(), val);
                    } else if let Some(_expr) = value {
                        // تقييم التعبير
                        if let Some(val) = env.borrow().get(name) {
                            exports.insert(name.clone(), val);
                        }
                    }
                }
                
                crate::parser::ast::Stmt::ExportList { items } => {
                    for item in items {
                        if let Some(val) = env.borrow().get(item) {
                            exports.insert(item.clone(), val);
                        }
                    }
                }
                
                // الدوال والمتغيرات في المستوى الأعلى تُصدّر تلقائياً
                crate::parser::ast::Stmt::FunctionDecl { name, .. } => {
                    if let Some(val) = env.borrow().get(name) {
                        exports.insert(name.clone(), val);
                    }
                }
                
                crate::parser::ast::Stmt::VariableDecl { name, .. } => {
                    if let Some(val) = env.borrow().get(name) {
                        exports.insert(name.clone(), val);
                    }
                }
                
                crate::parser::ast::Stmt::ClassDecl { name, .. } => {
                    if let Some(val) = env.borrow().get(name) {
                        exports.insert(name.clone(), val);
                    }
                }
                
                _ => {}
            }
        }
        
        exports
    }
    
    /// الحصول على وحدة محمّلة
    pub fn get_module(&self, name: &str) -> Option<&LoadedModule> {
        self.loaded_modules.get(name)
    }
    
    /// الحصول على تصدير من وحدة
    pub fn get_export(&self, module_name: &str, export_name: &str) -> Option<Rc<RefCell<Value>>> {
        self.loaded_modules.get(module_name)
            .and_then(|m| m.get_export(export_name))
    }
    
    /// قائمة الوحدات المحمّلة
    pub fn loaded_modules(&self) -> Vec<&str> {
        self.loaded_modules.keys().map(|s| s.as_str()).collect()
    }
    
    /// الإحصائيات
    pub fn stats(&self) -> &LoaderStats {
        &self.stats
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // المكتبات المدمجة
    // ═══════════════════════════════════════════════════════════════════════════════
    
    /// إنشاء وحدة JSON المدمجة
    fn create_json_module(&self) -> HashMap<String, Rc<RefCell<Value>>> {
        let mut exports = HashMap::new();
        
        // json.تشفير - تشفير قيمة إلى JSON
        exports.insert("تشفير".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "json_تشفير".to_string(),
            func: |args| {
                let val = &*args[0].borrow();
                let json = value_to_json(val);
                Ok(Rc::new(RefCell::new(Value::String(json))))
            },
        })));
        
        exports.insert("encode".to_string(), Rc::clone(&exports["تشفير"]));
        
        // json.فك_التشفير - فك تشفير JSON
        exports.insert("فك_التشفير".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "json_فك_التشفير".to_string(),
            func: |args| {
                match &*args[0].borrow() {
                    Value::String(s) => {
                        match json_to_value(s) {
                            Ok(v) => Ok(Rc::new(RefCell::new(v))),
                            Err(e) => Err(e),
                        }
                    }
                    _ => Err("فك_التشفير يتطلب نصاً".into()),
                }
            },
        })));
        
        exports.insert("decode".to_string(), Rc::clone(&exports["فك_التشفير"]));
        
        // json.صالح - التحقق من صحة JSON
        exports.insert("صالح".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "json_صالح".to_string(),
            func: |args| {
                match &*args[0].borrow() {
                    Value::String(s) => {
                        let valid = serde_json::from_str::<serde_json::Value>(s).is_ok();
                        Ok(Rc::new(RefCell::new(Value::Boolean(valid))))
                    }
                    _ => Ok(Rc::new(RefCell::new(Value::Boolean(false)))),
                }
            },
        })));
        
        exports.insert("valid".to_string(), Rc::clone(&exports["صالح"]));
        
        exports
    }
    
    /// إنشاء وحدة الرياضيات المدمجة
    fn create_math_module(&self) -> HashMap<String, Rc<RefCell<Value>>> {
        let mut exports = HashMap::new();
        
        // الثوابت
        exports.insert("PI".to_string(), Rc::new(RefCell::new(Value::Number(std::f64::consts::PI))));
        exports.insert("E".to_string(), Rc::new(RefCell::new(Value::Number(std::f64::consts::E))));
        exports.insert("TAU".to_string(), Rc::new(RefCell::new(Value::Number(std::f64::consts::TAU))));
        
        // دوال إضافية
        exports.insert("عشوائي".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "math_عشوائي".to_string(),
            func: |args| {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                if args.len() >= 2 {
                    let min = args[0].borrow().to_number().unwrap_or(0.0);
                    let max = args[1].borrow().to_number().unwrap_or(1.0);
                    Ok(Rc::new(RefCell::new(Value::Number(rng.gen_range(min..max)))))
                } else {
                    Ok(Rc::new(RefCell::new(Value::Number(rng.gen::<f64>()))))
                }
            },
        })));
        
        exports.insert("random".to_string(), Rc::clone(&exports["عشوائي"]));
        
        exports
    }
    
    /// إنشاء وحدة التاريخ والوقت المدمجة
    fn create_datetime_module(&self) -> HashMap<String, Rc<RefCell<Value>>> {
        let mut exports = HashMap::new();
        
        exports.insert("الآن".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "datetime_الآن".to_string(),
            func: |_| {
                let now = chrono::Local::now();
                Ok(Rc::new(RefCell::new(Value::String(now.to_rfc3339()))))
            },
        })));
        
        exports.insert("now".to_string(), Rc::clone(&exports["الآن"]));
        
        exports.insert("تاريخ".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "datetime_تاريخ".to_string(),
            func: |_| {
                let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                Ok(Rc::new(RefCell::new(Value::String(today))))
            },
        })));
        
        exports.insert("date".to_string(), Rc::clone(&exports["تاريخ"]));
        
        exports.insert("وقت".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "datetime_وقت".to_string(),
            func: |_| {
                let time = chrono::Local::now().format("%H:%M:%S").to_string();
                Ok(Rc::new(RefCell::new(Value::String(time))))
            },
        })));
        
        exports.insert("time".to_string(), Rc::clone(&exports["وقت"]));
        
        exports
    }
    
    /// إنشاء وحدة التعابير النمطية المدمجة
    fn create_regex_module(&self) -> HashMap<String, Rc<RefCell<Value>>> {
        let mut exports = HashMap::new();
        
        exports.insert("طابق".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "regex_طابق".to_string(),
            func: |args| {
                if args.len() < 2 {
                    return Err("طابق يتطلب نصين: النص والنمط".into());
                }
                let text = args[0].borrow().to_string_value();
                let pattern = args[1].borrow().to_string_value();
                
                match regex::Regex::new(&pattern) {
                    Ok(re) => {
                        let is_match = re.is_match(&text);
                        Ok(Rc::new(RefCell::new(Value::Boolean(is_match))))
                    }
                    Err(e) => Err(format!("نمط غير صالح: {}", e).into()),
                }
            },
        })));
        
        exports.insert("match".to_string(), Rc::clone(&exports["طابق"]));
        
        exports.insert("ابحث".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "regex_ابحث".to_string(),
            func: |args| {
                if args.len() < 2 {
                    return Err("ابحث يتطلب نصين: النص والنمط".into());
                }
                let text = args[0].borrow().to_string_value();
                let pattern = args[1].borrow().to_string_value();
                
                match regex::Regex::new(&pattern) {
                    Ok(re) => {
                        if let Some(m) = re.find(&text) {
                            Ok(Rc::new(RefCell::new(Value::String(m.as_str().to_string()))))
                        } else {
                            Ok(Rc::new(RefCell::new(Value::Null)))
                        }
                    }
                    Err(e) => Err(format!("نمط غير صالح: {}", e).into()),
                }
            },
        })));
        
        exports.insert("find".to_string(), Rc::clone(&exports["ابحث"]));
        
        exports.insert("استبدل".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "regex_استبدل".to_string(),
            func: |args| {
                if args.len() < 3 {
                    return Err("استبدل يتطلب ثلاثة معاملات: النص والنمط والبديل".into());
                }
                let text = args[0].borrow().to_string_value();
                let pattern = args[1].borrow().to_string_value();
                let replacement = args[2].borrow().to_string_value();
                
                match regex::Regex::new(&pattern) {
                    Ok(re) => {
                        let result = re.replace_all(&text, replacement.as_str()).to_string();
                        Ok(Rc::new(RefCell::new(Value::String(result))))
                    }
                    Err(e) => Err(format!("نمط غير صالح: {}", e).into()),
                }
            },
        })));
        
        exports.insert("replace".to_string(), Rc::clone(&exports["استبدل"]));
        
        exports
    }
    
    /// إنشاء وحدة الملفات المدمجة
    fn create_file_module(&self) -> HashMap<String, Rc<RefCell<Value>>> {
        let mut exports = HashMap::new();
        
        exports.insert("اقرأ".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "file_اقرأ".to_string(),
            func: |args| {
                let path = args[0].borrow().to_string_value();
                match std::fs::read_to_string(&path) {
                    Ok(content) => Ok(Rc::new(RefCell::new(Value::String(content)))),
                    Err(e) => Err(format!("خطأ في قراءة الملف: {}", e).into()),
                }
            },
        })));
        
        exports.insert("read".to_string(), Rc::clone(&exports["اقرأ"]));
        
        exports.insert("اكتب".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "file_اكتب".to_string(),
            func: |args| {
                if args.len() < 2 {
                    return Err("اكتب يتطلب مساراً ومحتوى".into());
                }
                let path = args[0].borrow().to_string_value();
                let content = args[1].borrow().to_string_value();
                
                match std::fs::write(&path, content) {
                    Ok(_) => Ok(Rc::new(RefCell::new(Value::Boolean(true)))),
                    Err(e) => Err(format!("خطأ في كتابة الملف: {}", e).into()),
                }
            },
        })));
        
        exports.insert("write".to_string(), Rc::clone(&exports["اكتب"]));
        
        exports.insert("موجود".to_string(), Rc::new(RefCell::new(Value::NativeFunction {
            name: "file_موجود".to_string(),
            func: |args| {
                let path = args[0].borrow().to_string_value();
                Ok(Rc::new(RefCell::new(Value::Boolean(std::path::Path::new(&path).exists()))))
            },
        })));
        
        exports.insert("exists".to_string(), Rc::clone(&exports["موجود"]));
        
        exports
    }
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال مساعدة لتحويل JSON
// ═══════════════════════════════════════════════════════════════════════════════

fn value_to_json(val: &Value) -> String {
    match val {
        Value::Null => "null".to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => {
            let escaped = s
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t");
            format!("\"{}\"", escaped)
        }
        Value::List(items) => {
            let items_str: Vec<String> = items
                .iter()
                .map(|v| value_to_json(&*v.borrow()))
                .collect();
            format!("[{}]", items_str.join(","))
        }
        Value::Dictionary(map) => {
            let items_str: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("\"{}\":{}", k, value_to_json(&*v.borrow())))
                .collect();
            format!("{{{}}}", items_str.join(","))
        }
        _ => "null".to_string(),
    }
}

fn json_to_value(s: &str) -> Result<Value, String> {
    let json: serde_json::Value = serde_json::from_str(s)
        .map_err(|e| format!("خطأ في تحليل JSON: {}", e))?;
    Ok(json_value_to_value(json))
}

fn json_value_to_value(json: serde_json::Value) -> Value {
    match json {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Boolean(b),
        serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            let list: Vec<Rc<RefCell<Value>>> = arr
                .into_iter()
                .map(|v| Rc::new(RefCell::new(json_value_to_value(v))))
                .collect();
            Value::List(list)
        }
        serde_json::Value::Object(map) => {
            let dict: HashMap<String, Rc<RefCell<Value>>> = map
                .into_iter()
                .map(|(k, v)| (k, Rc::new(RefCell::new(json_value_to_value(v)))))
                .collect();
            Value::Dictionary(dict)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_loader_creation() {
        let loader = ModuleLoader::new();
        assert!(loader.builtin_modules.contains_key("json"));
        assert!(loader.builtin_modules.contains_key("math"));
    }

    #[test]
    fn test_load_builtin_json() {
        let mut loader = ModuleLoader::new();
        let result = loader.load("json");
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert!(module.exports.contains_key("تشفير"));
        assert!(module.exports.contains_key("فك_التشفير"));
    }

    #[test]
    fn test_json_encode() {
        let loader = ModuleLoader::new();
        let json_module = loader.builtin_modules.get("json").unwrap();
        let encode = json_module.get("تشفير").unwrap();
        
        // اختبار التشفير
        let input = Rc::new(RefCell::new(Value::String("مرحبا".to_string())));
        let args = vec![input];
        
        if let Value::NativeFunction { func, .. } = &*encode.borrow() {
            let result = func(&args).unwrap();
            let borrowed = result.borrow();
            assert_eq!(&*borrowed, &Value::String("\"مرحبا\"".to_string()));
        };
    }

    #[test]
    fn test_json_decode() {
        let loader = ModuleLoader::new();
        let json_module = loader.builtin_modules.get("json").unwrap();
        let decode = json_module.get("فك_التشفير").unwrap();
        
        let input = Rc::new(RefCell::new(Value::String("{\"name\":\"test\"}".to_string())));
        let args = vec![input];
        
        if let Value::NativeFunction { func, .. } = &*decode.borrow() {
            let result = func(&args).unwrap();
            let borrowed = result.borrow();
            if let Value::Dictionary(dict) = &*borrowed {
                assert!(dict.contains_key("name"));
            } else {
                panic!("Expected dictionary");
            }
        };
    }

    #[test]
    fn test_not_found_module() {
        let mut loader = ModuleLoader::new();
        let result = loader.load("nonexistent_module");
        assert!(matches!(result, Err(LoadError::NotFound(_))));
    }
}
