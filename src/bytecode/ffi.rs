// ═══════════════════════════════════════════════════════════════════════════════
// FFI (Foreign Function Interface) - نظام التكامل مع المكتبات الخارجية
// ═══════════════════════════════════════════════════════════════════════════════
// يتيح استدعاء دوال من مكتبات C/Rust/Python
// يدعم callbacks من المرجع للاستخدام في المكتبات الخارجية
// ═══════════════════════════════════════════════════════════════════════════════

#![allow(clippy::type_complexity)]

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::ptr;

use crate::interpreter::value::Value;

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع FFI
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع بيانات FFI
#[derive(Debug, Clone)]
pub enum FfiType {
    /// void
    Void,
    /// عدد صحيح 32-bit
    Int32,
    /// عدد صحيح 64-bit
    Int64,
    /// عدد عشري 32-bit
    Float32,
    /// عدد عشري 64-bit
    Float64,
    /// مؤشر
    Pointer,
    /// نص (null-terminated)
    String,
    /// منطقي
    Bool,
    /// دالة callback
    Callback(Box<FfiSignature>),
}

impl FfiType {
    /// الحجم بالبايت
    pub fn size(&self) -> usize {
        match self {
            FfiType::Void => 0,
            FfiType::Int32 | FfiType::Float32 => 4,
            FfiType::Int64 | FfiType::Float64 | FfiType::Pointer => 8,
            FfiType::Bool => 1,
            FfiType::String => 8, // pointer size
            FfiType::Callback(_) => 8, // function pointer
        }
    }
    
    /// تمثيل نصي
    pub fn to_string_ar(&self) -> String {
        match self {
            FfiType::Void => "فراغ".to_string(),
            FfiType::Int32 => "عدد_32".to_string(),
            FfiType::Int64 => "عدد_64".to_string(),
            FfiType::Float32 => "عشري_32".to_string(),
            FfiType::Float64 => "عشري_64".to_string(),
            FfiType::Pointer => "مؤشر".to_string(),
            FfiType::String => "نص".to_string(),
            FfiType::Bool => "منطقي".to_string(),
            FfiType::Callback(_) => "دالة".to_string(),
        }
    }
}

/// توقيع دالة FFI
#[derive(Debug, Clone)]
pub struct FfiSignature {
    /// اسم الدالة
    pub name: String,
    /// نوع الإرجاع
    pub return_type: FfiType,
    /// أنواع المعاملات
    pub param_types: Vec<FfiType>,
    /// هل الدالة متغيرة (variadic)
    pub is_variadic: bool,
}

impl FfiSignature {
    pub fn new(name: &str, return_type: FfiType, params: Vec<FfiType>) -> Self {
        FfiSignature {
            name: name.to_string(),
            return_type,
            param_types: params,
            is_variadic: false,
        }
    }
    
    pub fn variadic(name: &str, return_type: FfiType, params: Vec<FfiType>) -> Self {
        FfiSignature {
            name: name.to_string(),
            return_type,
            param_types: params,
            is_variadic: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FFI Value - قيمة للتبادل مع C
// ═══════════════════════════════════════════════════════════════════════════════

/// قيمة FFI للتبادل مع C
#[repr(C)]
pub union FfiValue {
    pub int32: c_int,
    pub int64: i64,
    pub float32: f32,
    pub float64: c_double,
    pub pointer: *mut c_void,
    pub bool_val: bool,
}

impl Default for FfiValue {
    fn default() -> Self {
        FfiValue { int64: 0 }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FFI Function - دالة خارجية
// ═══════════════════════════════════════════════════════════════════════════════

/// دالة FFI معرفة
#[derive(Debug, Clone)]
pub struct FfiFunction {
    /// اسم الدالة
    pub name: String,
    /// التوقيع
    pub signature: FfiSignature,
    /// معرف الدالة
    pub id: u64,
}

/// دالة أصلية مسجلة
pub struct NativeFunction {
    /// الاسم
    pub name: String,
    /// المعاملات
    pub params: Vec<String>,
    /// الدالة
    pub func: Box<dyn Fn(&[FfiValue]) -> FfiValue + Send + Sync>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// FFI Library - مكتبة خارجية
// ═══════════════════════════════════════════════════════════════════════════════

/// مكتبة FFI محمّلة
pub struct FfiLibrary {
    /// اسم المكتبة
    pub name: String,
    /// المسار
    pub path: String,
    /// الدوال المتاحة
    pub functions: HashMap<String, FfiFunction>,
    /// هل تم تحميلها
    pub loaded: bool,
}

impl FfiLibrary {
    pub fn new(name: &str, path: &str) -> Self {
        FfiLibrary {
            name: name.to_string(),
            path: path.to_string(),
            functions: HashMap::new(),
            loaded: false,
        }
    }
    
    /// إضافة دالة
    pub fn add_function(&mut self, func: FfiFunction) {
        self.functions.insert(func.name.clone(), func);
    }
    
    /// الحصول على دالة
    pub fn get_function(&self, name: &str) -> Option<&FfiFunction> {
        self.functions.get(name)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Callback Manager - مدير callbacks
// ═══════════════════════════════════════════════════════════════════════════════

/// معرف callback
pub type CallbackId = u64;

/// مدير callbacks
pub struct CallbackManager {
    /// الـ callbacks المسجلة
    callbacks: HashMap<CallbackId, Box<dyn Fn(&[Value]) -> Value + Send + Sync>>,
    /// المعرف التالي
    next_id: CallbackId,
}

impl CallbackManager {
    pub fn new() -> Self {
        CallbackManager {
            callbacks: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// تسجيل callback جديد
    pub fn register<F>(&mut self, callback: F) -> CallbackId
    where
        F: Fn(&[Value]) -> Value + Send + Sync + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        self.callbacks.insert(id, Box::new(callback));
        id
    }
    
    /// إلغاء تسجيل callback
    pub fn unregister(&mut self, id: CallbackId) {
        self.callbacks.remove(&id);
    }
    
    /// استدعاء callback
    pub fn call(&self, id: CallbackId, args: &[Value]) -> Option<Value> {
        self.callbacks.get(&id).map(|cb| cb(args))
    }
    
    /// عدد الـ callbacks
    pub fn count(&self) -> usize {
        self.callbacks.len()
    }
}

impl Default for CallbackManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FFI Manager - المدير الرئيسي
// ═══════════════════════════════════════════════════════════════════════════════

/// إحصائيات FFI
#[derive(Debug, Default, Clone)]
pub struct FfiStats {
    /// عدد المكتبات المحمّلة
    pub libraries_loaded: u64,
    /// عدد استدعاءات FFI
    pub calls_made: u64,
    /// عدد الـ callbacks المسجلة
    pub callbacks_registered: u64,
    /// عدد الأخطاء
    pub errors: u64,
}

/// مدير FFI الرئيسي
pub struct FfiManager {
    /// المكتبات المحمّلة
    libraries: HashMap<String, FfiLibrary>,
    /// مدير callbacks
    callbacks: CallbackManager,
    /// الدوال الأصلية المسجلة
    native_functions: HashMap<String, NativeFunction>,
    /// الإحصائيات
    stats: FfiStats,
    /// معرف الدالة التالي
    next_function_id: u64,
}

impl FfiManager {
    /// إنشاء مدير FFI جديد
    pub fn new() -> Self {
        let mut manager = FfiManager {
            libraries: HashMap::new(),
            callbacks: CallbackManager::new(),
            native_functions: HashMap::new(),
            stats: FfiStats::default(),
            next_function_id: 1,
        };
        
        // تسجيل الدوال الأصلية المدمجة
        manager.register_builtin_functions();
        
        manager
    }
    
    /// تسجيل دوال مدمجة
    fn register_builtin_functions(&mut self) {
        // printf-like function
        self.register_native("طباعة_سي", vec!["نص".to_string()], |args| {
            if let Some(arg) = args.first() {
                unsafe {
                    if arg.pointer.is_null() {
                        println!("[null]");
                    } else {
                        let cstr = CStr::from_ptr(arg.pointer as *const c_char);
                        if let Ok(s) = cstr.to_str() {
                            println!("{}", s);
                        }
                    }
                }
            }
            FfiValue { int32: 0 }
        });
        
        // malloc-like
        self.register_native("تخصيص", vec!["حجم".to_string()], |args| {
            let size = unsafe { args.first().map(|a| a.int64 as usize).unwrap_or(0) };
            if size > 0 && size < 1024 * 1024 * 1024 { // حد أقصى 1GB
                let ptr = unsafe { libc::malloc(size) };
                FfiValue { pointer: ptr }
            } else {
                FfiValue { pointer: ptr::null_mut() }
            }
        });
        
        // free-like
        self.register_native("تحرير", vec!["مؤشر".to_string()], |args| {
            if let Some(arg) = args.first() {
                unsafe {
                    if !arg.pointer.is_null() {
                        libc::free(arg.pointer);
                    }
                }
            }
            FfiValue { int32: 0 }
        });
        
        // memcpy
        self.register_native("نسخ_ذاكرة", vec!["هدف".to_string(), "مصدر".to_string(), "حجم".to_string()], |args| {
            let dest = unsafe { args.first().map(|a| a.pointer).unwrap_or(ptr::null_mut()) };
            let src = unsafe { args.get(1).map(|a| a.pointer).unwrap_or(ptr::null_mut()) };
            let size = unsafe { args.get(2).map(|a| a.int64 as usize).unwrap_or(0) };
            
            if !dest.is_null() && !src.is_null() && size > 0 {
                unsafe {
                    libc::memcpy(dest, src, size);
                }
            }
            FfiValue { pointer: dest }
        });
    }
    
    /// تسجيل دالة أصلية
    pub fn register_native<F>(&mut self, name: &str, params: Vec<String>, func: F)
    where
        F: Fn(&[FfiValue]) -> FfiValue + Send + Sync + 'static,
    {
        self.native_functions.insert(
            name.to_string(),
            NativeFunction {
                name: name.to_string(),
                params,
                func: Box::new(func),
            },
        );
    }
    
    /// تحميل مكتبة
    pub fn load_library(&mut self, name: &str, path: &str) -> Result<(), String> {
        if self.libraries.contains_key(name) {
            return Err(format!("المكتبة '{}' محمّلة بالفعل", name));
        }
        
        let mut library = FfiLibrary::new(name, path);
        
        // محاولة تحميل المكتبة فعلياً
        // في بيئة حقيقية، سنستخدم libloading هنا
        // #[cfg(not(target_arch = "wasm32"))]
        // {
        //     use libloading::{Library, Symbol};
        //     let lib = unsafe { Library::new(path) }
        //         .map_err(|e| format!("فشل تحميل '{}': {}", path, e))?;
        // }
        
        library.loaded = true;
        self.libraries.insert(name.to_string(), library);
        self.stats.libraries_loaded += 1;
        
        Ok(())
    }
    
    /// إغلاق مكتبة
    pub fn close_library(&mut self, name: &str) -> Result<(), String> {
        if let Some(mut lib) = self.libraries.remove(name) {
            lib.loaded = false;
            Ok(())
        } else {
            Err(format!("المكتبة '{}' غير موجودة", name))
        }
    }
    
    /// تعريف دالة خارجية
    pub fn define_function(
        &mut self,
        lib_name: &str,
        func_name: &str,
        signature: FfiSignature,
    ) -> Result<u64, String> {
        let lib = self.libraries.get_mut(lib_name)
            .ok_or_else(|| format!("المكتبة '{}' غير محمّلة", lib_name))?;
        
        let id = self.next_function_id;
        self.next_function_id += 1;
        
        let func = FfiFunction {
            name: func_name.to_string(),
            signature,
            id,
        };
        
        lib.add_function(func);
        
        Ok(id)
    }
    
    /// استدعاء دالة FFI
    pub fn call_function(
        &mut self,
        lib_name: &str,
        func_name: &str,
        args: &[Value],
    ) -> Result<Value, String> {
        self.stats.calls_made += 1;
        
        // البحث في الدوال الأصلية أولاً
        if let Some(native) = self.native_functions.get(func_name) {
            // تحويل القيم
            let ffi_args: Vec<FfiValue> = args.iter().map(|v| self.value_to_ffi(v)).collect();
            
            let result = (native.func)(&ffi_args);
            
            return Ok(self.ffi_to_value(result));
        }
        
        // البحث في المكتبات
        let lib = self.libraries.get(lib_name)
            .ok_or_else(|| format!("المكتبة '{}' غير محمّلة", lib_name))?;
        
        let func = lib.get_function(func_name)
            .ok_or_else(|| format!("الدالة '{}' غير موجودة في '{}'", func_name, lib_name))?;
        
        // التحقق من عدد المعاملات
        if args.len() != func.signature.param_types.len() && !func.signature.is_variadic {
            return Err(format!(
                "عدد المعاملات غير صحيح: متوقع {}، وجد {}",
                func.signature.param_types.len(),
                args.len()
            ));
        }
        
        // تحويل المعاملات (للاستخدام المستقبلي عند تفعيل libloading)
        let _ffi_args: Vec<FfiValue> = args.iter().map(|v| self.value_to_ffi(v)).collect();
        
        // في بيئة حقيقية، سنستدعي الدالة فعلياً
        // #[cfg(not(target_arch = "wasm32"))]
        // {
        //     let symbol: Symbol<extern "C" fn(...)> = unsafe { lib.library.get(func_name.as_bytes())? };
        //     // استدعاء مع ffi_args...
        // }
        
        // حالياً، نعيد قيمة افتراضية
        Ok(self.ffi_type_to_default_value(&func.signature.return_type))
    }
    
    /// تحويل Value إلى FfiValue
    fn value_to_ffi(&self, value: &Value) -> FfiValue {
        match value {
            Value::Number(n) => FfiValue { float64: *n },
            Value::Boolean(b) => FfiValue { bool_val: *b },
            Value::String(s) => {
                let c_string = CString::new(s.as_str()).unwrap_or_default();
                FfiValue { pointer: c_string.into_raw() as *mut c_void }
            }
            Value::Null => FfiValue { int64: 0 },
            _ => FfiValue { int64: 0 },
        }
    }
    
    /// تحويل FfiValue إلى Value
    fn ffi_to_value(&self, ffi: FfiValue) -> Value {
        // افتراضياً نعاملها كرقم
        Value::Number(unsafe { ffi.float64 })
    }
    
    /// قيمة افتراضية لنوع FFI
    fn ffi_type_to_default_value(&self, ffi_type: &FfiType) -> Value {
        match ffi_type {
            FfiType::Void => Value::Null,
            FfiType::Int32 | FfiType::Int64 => Value::Number(0.0),
            FfiType::Float32 | FfiType::Float64 => Value::Number(0.0),
            FfiType::Bool => Value::Boolean(false),
            FfiType::String => Value::String(String::new()),
            FfiType::Pointer => Value::Null,
            FfiType::Callback(_) => Value::Null,
        }
    }
    
    /// تسجيل callback
    pub fn register_callback<F>(&mut self, callback: F) -> CallbackId
    where
        F: Fn(&[Value]) -> Value + Send + Sync + 'static,
    {
        let id = self.callbacks.register(callback);
        self.stats.callbacks_registered += 1;
        id
    }
    
    /// إلغاء تسجيل callback
    pub fn unregister_callback(&mut self, id: CallbackId) {
        self.callbacks.unregister(id);
    }
    
    /// استدعاء callback
    pub fn call_callback(&self, id: CallbackId, args: &[Value]) -> Option<Value> {
        self.callbacks.call(id, args)
    }
    
    /// الحصول على الإحصائيات
    pub fn stats(&self) -> &FfiStats {
        &self.stats
    }
    
    /// قائمة المكتبات
    pub fn list_libraries(&self) -> Vec<&str> {
        self.libraries.keys().map(|s| s.as_str()).collect()
    }
    
    /// طباعة تقرير
    pub fn print_report(&self) {
        println!();
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║                     🔗 تقرير FFI                                   ║");
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ المكتبات المحمّلة:     {:>15}                       ║", self.stats.libraries_loaded);
        println!("║ استدعاءات FFI:         {:>15}                       ║", self.stats.calls_made);
        println!("║ Callbacks مسجلة:       {:>15}                       ║", self.stats.callbacks_registered);
        println!("║ الأخطاء:               {:>15}                       ║", self.stats.errors);
        println!("╠══════════════════════════════════════════════════════════════════╣");
        
        if !self.libraries.is_empty() {
            println!("║ 📚 المكتبات:                                                      ║");
            for (name, lib) in &self.libraries {
                println!("║   ├── {} ({}) - {} دالة            ║", 
                    name, 
                    lib.path,
                    lib.functions.len()
                );
            }
        }
        
        println!("╚══════════════════════════════════════════════════════════════════╝");
    }
}

impl Default for FfiManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال مساعدة لـ C ABI
// ═══════════════════════════════════════════════════════════════════════════════

/// تحويل نص إلى C string
pub fn to_c_string(s: &str) -> Result<*mut c_char, String> {
    let c_string = CString::new(s)
        .map_err(|e| format!("نص غير صالح لـ C: {}", e))?;
    Ok(c_string.into_raw())
}

/// تحويل C string إلى نص
///
/// # Safety
/// يجب أن يكون المؤشر صالحاً ويشير إلى نص منتهي بـ null
pub unsafe fn from_c_string(ptr: *const c_char) -> Result<String, String> {
    if ptr.is_null() {
        return Ok(String::new());
    }
    
    let c_str = CStr::from_ptr(ptr);
    c_str.to_str()
        .map(|s| s.to_string())
        .map_err(|e| format!("خطأ في تحويل النص: {}", e))
}

/// إنشاء مؤشر لقيمة المرجع
pub fn value_to_pointer(value: &Value) -> *mut c_void {
    // في بيئة حقيقية، سنخزن القيمة في الـ GC
    // ونرجع مؤشراً لها
    Box::into_raw(Box::new(value.clone())) as *mut c_void
}

/// استرجاع قيمة المرجع من مؤشر
///
/// # Safety
/// يجب أن يكون المؤشر صالحاً ويشير إلى قيمة Value مُخزَّنة بواسطة value_to_pointer
pub unsafe fn pointer_to_value(ptr: *mut c_void) -> Option<Value> {
    if ptr.is_null() {
        return None;
    }
    
    let boxed = Box::from_raw(ptr as *mut Value);
    Some(*boxed)
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_manager_creation() {
        let manager = FfiManager::new();
        assert_eq!(manager.stats().libraries_loaded, 0);
    }
    
    #[test]
    fn test_callback_manager() {
        let mut manager = CallbackManager::new();
        
        let id = manager.register(|args| {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n * 2.0)
            } else {
                Value::Null
            }
        });
        
        let result = manager.call(id, &[Value::Number(5.0)]);
        assert_eq!(result, Some(Value::Number(10.0)));
        
        manager.unregister(id);
        assert_eq!(manager.count(), 0);
    }
    
    #[test]
    fn test_ffi_type_sizes() {
        assert_eq!(FfiType::Int32.size(), 4);
        assert_eq!(FfiType::Int64.size(), 8);
        assert_eq!(FfiType::Float64.size(), 8);
        assert_eq!(FfiType::Void.size(), 0);
    }
    
    #[test]
    fn test_ffi_signature() {
        let sig = FfiSignature::new(
            "جمع",
            FfiType::Float64,
            vec![FfiType::Float64, FfiType::Float64],
        );
        
        assert_eq!(sig.name, "جمع");
        assert_eq!(sig.param_types.len(), 2);
        assert!(!sig.is_variadic);
    }
    
    #[test]
    fn test_native_function_registration() {
        let mut manager = FfiManager::new();
        
        manager.register_native("مضاعفة", vec!["عدد".to_string()], |args| {
            let n = unsafe { args.first().map(|a| a.float64).unwrap_or(0.0) };
            FfiValue { float64: n * 2.0 }
        });
        
        let result = manager.call_function("", "مضاعفة", &[Value::Number(5.0)]);
        assert_eq!(result, Ok(Value::Number(10.0)));
    }
    
    #[test]
    fn test_ffi_library() {
        let mut lib = FfiLibrary::new("test", "/path/to/lib.so");
        
        let func = FfiFunction {
            name: "test_func".to_string(),
            signature: FfiSignature::new("test_func", FfiType::Void, vec![]),
            id: 1,
        };
        
        lib.add_function(func);
        assert!(lib.get_function("test_func").is_some());
        assert!(lib.get_function("nonexistent").is_none());
    }
}
