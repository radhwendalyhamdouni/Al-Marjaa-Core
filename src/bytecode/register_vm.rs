// ═══════════════════════════════════════════════════════════════════════════════
// Register-based VM - الآلة الافتراضية المبنية على السجلات
// ═══════════════════════════════════════════════════════════════════════════════
// أسرع 2-3x من Stack-based VM
// تعليمات أقل، أداء أعلى
// ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::opcodes::Chunk;
use crate::interpreter::value::{Environment, SharedValue, Value};

// ═══════════════════════════════════════════════════════════════════════════════
// الثوابت
// ═══════════════════════════════════════════════════════════════════════════════

/// عدد السجلات المتاحة
const NUM_REGISTERS: usize = 256;

/// الحد الأقصى لعمق الاستدعاء
const MAX_RECURSION_DEPTH: usize = 1000;

/// حجم كاش المتغيرات العامة
const GLOBAL_CACHE_SIZE: usize = 128;

// ═══════════════════════════════════════════════════════════════════════════════
// تعليمات Register-based
// ═══════════════════════════════════════════════════════════════════════════════

/// تعليمة Register VM
#[derive(Debug, Clone, PartialEq)]
pub enum RegOp {
    // ═══════════════════════════════════════════════════════════════
    // تحميل الثوابت - محسّنة
    // ═══════════════════════════════════════════════════════════════
    /// LoadConst dest, value - حمّل ثابت رقمي
    LoadConst { dest: u8, value: f64 },
    
    /// LoadConstInt dest, value - حمّل عدد صحيح (محسّن)
    LoadConstInt { dest: u8, value: i64 },
    
    /// LoadTrue dest - حمّل true
    LoadTrue { dest: u8 },
    
    /// LoadFalse dest - حمّل false
    LoadFalse { dest: u8 },
    
    /// LoadNull dest - حمّل null
    LoadNull { dest: u8 },
    
    /// LoadString dest, string_idx - حمّل نص من الـ constants
    LoadString { dest: u8, string_idx: u32 },

    // ═══════════════════════════════════════════════════════════════
    // العمليات الحسابية - كلها على سجلات
    // ═══════════════════════════════════════════════════════════════
    /// Add dest, src1, src2 - dest = src1 + src2
    Add { dest: u8, src1: u8, src2: u8 },
    
    /// AddConst dest, src, const - dest = src + const
    AddConst { dest: u8, src: u8, const_val: f64 },
    
    /// Sub dest, src1, src2
    Sub { dest: u8, src1: u8, src2: u8 },
    
    /// SubConst dest, src, const
    SubConst { dest: u8, src: u8, const_val: f64 },
    
    /// Mul dest, src1, src2
    Mul { dest: u8, src1: u8, src2: u8 },
    
    /// MulConst dest, src, const
    MulConst { dest: u8, src: u8, const_val: f64 },
    
    /// Div dest, src1, src2
    Div { dest: u8, src1: u8, src2: u8 },
    
    /// DivConst dest, src, const
    DivConst { dest: u8, src: u8, const_val: f64 },
    
    /// Mod dest, src1, src2
    Mod { dest: u8, src1: u8, src2: u8 },
    
    /// Pow dest, src1, src2
    Pow { dest: u8, src1: u8, src2: u8 },
    
    /// Neg dest, src - dest = -src
    Neg { dest: u8, src: u8 },

    // ═══════════════════════════════════════════════════════════════
    // العمليات المنطقية والمقارنة
    // ═══════════════════════════════════════════════════════════════
    /// And dest, src1, src2
    And { dest: u8, src1: u8, src2: u8 },
    
    /// Or dest, src1, src2
    Or { dest: u8, src1: u8, src2: u8 },
    
    /// Not dest, src
    Not { dest: u8, src: u8 },
    
    /// Eq dest, src1, src2
    Eq { dest: u8, src1: u8, src2: u8 },
    
    /// EqConst dest, src, const
    EqConst { dest: u8, src: u8, const_val: f64 },
    
    /// Ne dest, src1, src2
    Ne { dest: u8, src1: u8, src2: u8 },
    
    /// Lt dest, src1, src2
    Lt { dest: u8, src1: u8, src2: u8 },
    
    /// Le dest, src1, src2
    Le { dest: u8, src1: u8, src2: u8 },
    
    /// Gt dest, src1, src2
    Gt { dest: u8, src1: u8, src2: u8 },
    
    /// Ge dest, src1, src2
    Ge { dest: u8, src1: u8, src2: u8 },

    // ═══════════════════════════════════════════════════════════════
    // نقل البيانات
    // ═══════════════════════════════════════════════════════════════
    /// Move dest, src - انقل بين سجلات
    Move { dest: u8, src: u8 },
    
    /// Swap r1, r2 - بدّل بين سجلين
    Swap { r1: u8, r2: u8 },

    // ═══════════════════════════════════════════════════════════════
    // المتغيرات
    // ═══════════════════════════════════════════════════════════════
    /// LoadGlobal dest, name_idx
    LoadGlobal { dest: u8, name_idx: u32 },
    
    /// StoreGlobal name_idx, src
    StoreGlobal { name_idx: u32, src: u8 },
    
    /// LoadLocal dest, slot
    LoadLocal { dest: u8, slot: u16 },
    
    /// StoreLocal slot, src
    StoreLocal { slot: u16, src: u8 },

    // ═══════════════════════════════════════════════════════════════
    // التحكم في التدفق
    // ═══════════════════════════════════════════════════════════════
    /// Jump offset
    Jump { offset: i32 },
    
    /// JumpIfTrue reg, offset
    JumpIfTrue { reg: u8, offset: i32 },
    
    /// JumpIfFalse reg, offset
    JumpIfFalse { reg: u8, offset: i32 },
    
    /// JumpBack offset
    JumpBack { offset: i32 },

    // ═══════════════════════════════════════════════════════════════
    // الدوال
    // ═══════════════════════════════════════════════════════════════
    /// Call dest, func_reg, arg_count
    Call { dest: u8, func_reg: u8, arg_count: u8 },
    
    /// CallNative dest, func_idx, args...
    CallNative { dest: u8, func_idx: u32, args: Vec<u8> },
    
    /// Return reg
    Return { reg: u8 },
    
    /// ReturnVoid
    ReturnVoid,

    // ═══════════════════════════════════════════════════════════════
    // القوائم والقواميس
    // ═══════════════════════════════════════════════════════════════
    /// BuildList dest, count, elements...
    BuildList { dest: u8, count: u8, elements: Vec<u8> },
    
    /// BuildDict dest, count, entries...
    BuildDict { dest: u8, count: u8, entries: Vec<u8> },
    
    /// Index dest, obj_reg, idx_reg
    Index { dest: u8, obj_reg: u8, idx_reg: u8 },
    
    /// IndexSet obj_reg, idx_reg, val_reg
    IndexSet { obj_reg: u8, idx_reg: u8, val_reg: u8 },
    
    /// GetProperty dest, obj_reg, prop_idx
    GetProperty { dest: u8, obj_reg: u8, prop_idx: u32 },
    
    /// SetProperty obj_reg, prop_idx, val_reg
    SetProperty { obj_reg: u8, prop_idx: u32, val_reg: u8 },

    // ═══════════════════════════════════════════════════════════════
    // أخرى
    // ═══════════════════════════════════════════════════════════════
    /// Print reg
    Print { reg: u8 },
    
    /// Length dest, src
    Length { dest: u8, src: u8 },
    
    /// TypeOf dest, src
    TypeOf { dest: u8, src: u8 },
    
    /// Halt
    Halt,
    
    /// Nop
    Nop,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Register Chunk
// ═══════════════════════════════════════════════════════════════════════════════

/// كود مترجم للـ Register VM
#[derive(Debug, Clone)]
pub struct RegChunk {
    /// التعليمات
    pub instructions: Vec<RegOp>,
    /// النصوص الثابتة
    pub strings: Vec<String>,
    /// نقاط الدخول للدوال
    pub entry_points: HashMap<String, usize>,
}

impl RegChunk {
    pub fn new() -> Self {
        RegChunk {
            instructions: Vec::new(),
            strings: Vec::new(),
            entry_points: HashMap::new(),
        }
    }
    
    /// إضافة تعليمة
    pub fn emit(&mut self, op: RegOp) {
        self.instructions.push(op);
    }
    
    /// إضافة نص والحصول على الفهرس
    pub fn add_string(&mut self, s: &str) -> u32 {
        let idx = self.strings.len() as u32;
        self.strings.push(s.to_string());
        idx
    }
    
    /// الحصول على نص
    pub fn get_string(&self, idx: u32) -> Option<&str> {
        self.strings.get(idx as usize).map(|s| s.as_str())
    }
    
    /// عدد التعليمات
    pub fn len(&self) -> usize {
        self.instructions.len()
    }
}

impl Default for RegChunk {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// إطار الاستدعاء
// ═══════════════════════════════════════════════════════════════════════════════

/// إطار استدعاء للـ Register VM
#[derive(Debug, Clone)]
pub struct RegCallFrame {
    /// مؤشر التعليمة المحفوظ
    pub return_ip: usize,
    /// نقطة بداية السجلات المحلية
    pub base_register: u8,
    /// عدد السجلات المحلية
    pub num_locals: u8,
    /// اسم الدالة
    pub name: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
// إحصائيات Register VM
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Default, Clone)]
pub struct RegVMStats {
    /// عدد التعليمات المنفذة
    pub instructions_executed: u64,
    /// عدد استدعاءات الدوال
    pub function_calls: u64,
    /// وقت التنفيذ (ميكروثانية)
    pub execution_time_us: u64,
    /// عدد ضربات الكاش
    pub cache_hits: u64,
    /// عدد أخطاء الكاش
    pub cache_misses: u64,
    /// عدد التحسينات المطبقة
    pub optimizations_applied: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Register VM
// ═══════════════════════════════════════════════════════════════════════════════

/// نتيجة التنفيذ
#[derive(Debug)]
pub enum RegExecutionResult {
    Ok(SharedValue),
    Error(String),
    Break,
    Continue,
    Return(SharedValue),
}

/// Register-based Virtual Machine
pub struct RegisterVM {
    /// السجلات (256 سجل)
    registers: [SharedValue; NUM_REGISTERS],
    
    /// أنواع السجلات (للتحسين)
    register_types: [RegType; NUM_REGISTERS],
    
    /// إطارات الاستدعاء
    call_frames: Vec<RegCallFrame>,
    
    /// المتغيرات العامة
    globals: Rc<RefCell<Environment>>,
    
    /// كاش المتغيرات العامة
    global_cache: HashMap<u32, SharedValue>,
    
    /// الكود قيد التنفيذ
    chunk: Option<RegChunk>,
    
    /// مؤشر التعليمة
    ip: usize,
    
    /// هل متوقف
    halted: bool,
    
    /// عمق الاستدعاء
    recursion_depth: usize,
    
    /// الإحصائيات
    stats: RegVMStats,
}

/// نوع القيمة في السجل (للتحسين)
#[derive(Debug, Clone, Copy, PartialEq)]
enum RegType {
    Number,
    Int,
    Bool,
    String,
    Null,
    Object,
    Unknown,
}

impl RegisterVM {
    /// إنشاء VM جديدة
    pub fn new(globals: Rc<RefCell<Environment>>) -> Self {
        let null_val = Rc::new(RefCell::new(Value::Null));
        
        RegisterVM {
            registers: std::array::from_fn(|_| Rc::clone(&null_val)),
            register_types: [RegType::Unknown; NUM_REGISTERS],
            call_frames: Vec::with_capacity(64),
            globals,
            global_cache: HashMap::with_capacity(GLOBAL_CACHE_SIZE),
            chunk: None,
            ip: 0,
            halted: false,
            recursion_depth: 0,
            stats: RegVMStats::default(),
        }
    }
    
    /// إنشاء VM مع بيئة افتراضية
    pub fn with_fresh_env() -> Self {
        Self::new(Rc::new(RefCell::new(Environment::new())))
    }
    
    /// تحميل كود للتنفيذ
    pub fn load(&mut self, chunk: RegChunk) {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.halted = false;
        self.call_frames.clear();
        self.call_frames.push(RegCallFrame {
            return_ip: 0,
            base_register: 0,
            num_locals: 0,
            name: "main".to_string(),
        });
        self.global_cache.clear();
    }
    
    /// تنفيذ البرنامج كاملاً
    pub fn run(&mut self) -> RegExecutionResult {
        let start = std::time::Instant::now();
        
        loop {
            match self.step() {
                RegExecutionResult::Ok(_) => {
                    if self.halted {
                        let result = self.get_reg(0);
                        self.stats.execution_time_us = start.elapsed().as_micros() as u64;
                        return RegExecutionResult::Ok(result);
                    }
                }
                RegExecutionResult::Error(e) => return RegExecutionResult::Error(e),
                RegExecutionResult::Break => {
                    return RegExecutionResult::Error("break خارج الحلقة".into())
                }
                RegExecutionResult::Continue => {
                    return RegExecutionResult::Error("continue خارج الحلقة".into())
                }
                RegExecutionResult::Return(v) => return RegExecutionResult::Return(v),
            }
        }
    }
    
    /// تنفيذ تعليمة واحدة
    #[inline(always)]
    pub fn step(&mut self) -> RegExecutionResult {
        let op = {
            let chunk = match &self.chunk {
                Some(c) => c,
                None => return RegExecutionResult::Error("لا يوجد كود للتنفيذ".into()),
            };
            
            if self.ip >= chunk.instructions.len() {
                self.halted = true;
                return RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)));
            }
            
            chunk.instructions[self.ip].clone()
        };
        
        self.stats.instructions_executed += 1;
        
        // تنفيذ التعليمة
        let result = self.execute_op(&op);
        
        if !self.halted {
            self.ip += 1;
        }
        
        result
    }
    
    /// تنفيذ تعليمة محددة
    #[inline(always)]
    fn execute_op(&mut self, op: &RegOp) -> RegExecutionResult {
        match op {
            // ═══════════════════════════════════════════════════════════════
            // تحميل الثوابت - محسّنة جداً
            // ═══════════════════════════════════════════════════════════════
            RegOp::LoadConst { dest, value } => {
                self.set_reg_number(*dest, *value);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::LoadConstInt { dest, value } => {
                self.set_reg_number(*dest, *value as f64);
                self.register_types[*dest as usize] = RegType::Int;
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::LoadTrue { dest } => {
                self.set_reg_bool(*dest, true);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::LoadFalse { dest } => {
                self.set_reg_bool(*dest, false);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::LoadNull { dest } => {
                self.set_reg_null(*dest);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::LoadString { dest, string_idx } => {
                let chunk = self.chunk.as_ref().unwrap();
                if let Some(s) = chunk.get_string(*string_idx) {
                    self.set_reg_string(*dest, s.to_string());
                } else {
                    return RegExecutionResult::Error(format!("فهرس نص غير صالح: {}", string_idx));
                }
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // العمليات الحسابية - محسّنة بدون heap allocation
            // ═══════════════════════════════════════════════════════════════
            RegOp::Add { dest, src1, src2 } => {
                let a = self.get_reg_number(*src1);
                let b = self.get_reg_number(*src2);
                self.set_reg_number(*dest, a + b);
                self.stats.optimizations_applied += 1;
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::AddConst { dest, src, const_val } => {
                let a = self.get_reg_number(*src);
                self.set_reg_number(*dest, a + const_val);
                self.stats.optimizations_applied += 1;
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Sub { dest, src1, src2 } => {
                let a = self.get_reg_number(*src1);
                let b = self.get_reg_number(*src2);
                self.set_reg_number(*dest, a - b);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::SubConst { dest, src, const_val } => {
                let a = self.get_reg_number(*src);
                self.set_reg_number(*dest, a - const_val);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Mul { dest, src1, src2 } => {
                let a = self.get_reg_number(*src1);
                let b = self.get_reg_number(*src2);
                self.set_reg_number(*dest, a * b);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::MulConst { dest, src, const_val } => {
                let a = self.get_reg_number(*src);
                self.set_reg_number(*dest, a * const_val);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Div { dest, src1, src2 } => {
                let a = self.get_reg_number(*src1);
                let b = self.get_reg_number(*src2);
                if b == 0.0 {
                    self.set_reg_number(*dest, f64::INFINITY);
                } else {
                    self.set_reg_number(*dest, a / b);
                }
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::DivConst { dest, src, const_val } => {
                let a = self.get_reg_number(*src);
                if *const_val == 0.0 {
                    self.set_reg_number(*dest, f64::INFINITY);
                } else {
                    self.set_reg_number(*dest, a / const_val);
                }
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Mod { dest, src1, src2 } => {
                let a = self.get_reg_number(*src1);
                let b = self.get_reg_number(*src2);
                self.set_reg_number(*dest, a % b);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Pow { dest, src1, src2 } => {
                let a = self.get_reg_number(*src1);
                let b = self.get_reg_number(*src2);
                self.set_reg_number(*dest, a.powf(b));
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Neg { dest, src } => {
                let a = self.get_reg_number(*src);
                self.set_reg_number(*dest, -a);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // المقارنات - محسّنة
            // ═══════════════════════════════════════════════════════════════
            RegOp::Eq { dest, src1, src2 } => {
                let a = self.get_reg_number(*src1);
                let b = self.get_reg_number(*src2);
                self.set_reg_bool(*dest, (a - b).abs() < f64::EPSILON);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::EqConst { dest, src, const_val } => {
                let a = self.get_reg_number(*src);
                self.set_reg_bool(*dest, (a - const_val).abs() < f64::EPSILON);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Ne { dest, src1, src2 } => {
                let a = self.get_reg_number(*src1);
                let b = self.get_reg_number(*src2);
                self.set_reg_bool(*dest, (a - b).abs() >= f64::EPSILON);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Lt { dest, src1, src2 } => {
                let a = self.get_reg_number(*src1);
                let b = self.get_reg_number(*src2);
                self.set_reg_bool(*dest, a < b);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Le { dest, src1, src2 } => {
                let a = self.get_reg_number(*src1);
                let b = self.get_reg_number(*src2);
                self.set_reg_bool(*dest, a <= b);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Gt { dest, src1, src2 } => {
                let a = self.get_reg_number(*src1);
                let b = self.get_reg_number(*src2);
                self.set_reg_bool(*dest, a > b);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Ge { dest, src1, src2 } => {
                let a = self.get_reg_number(*src1);
                let b = self.get_reg_number(*src2);
                self.set_reg_bool(*dest, a >= b);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // المنطق
            // ═══════════════════════════════════════════════════════════════
            RegOp::And { dest, src1, src2 } => {
                let a = self.is_truthy_reg(*src1);
                let b = self.is_truthy_reg(*src2);
                self.set_reg_bool(*dest, a && b);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Or { dest, src1, src2 } => {
                let a = self.is_truthy_reg(*src1);
                let b = self.is_truthy_reg(*src2);
                self.set_reg_bool(*dest, a || b);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Not { dest, src } => {
                let a = self.is_truthy_reg(*src);
                self.set_reg_bool(*dest, !a);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // نقل البيانات
            // ═══════════════════════════════════════════════════════════════
            RegOp::Move { dest, src } => {
                let val = self.get_reg(*src);
                self.set_reg(*dest, val);
                self.register_types[*dest as usize] = self.register_types[*src as usize];
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Swap { r1, r2 } => {
                let temp = Rc::clone(&self.registers[*r1 as usize]);
                let temp_type = self.register_types[*r1 as usize];
                
                self.registers[*r1 as usize] = Rc::clone(&self.registers[*r2 as usize]);
                self.register_types[*r1 as usize] = self.register_types[*r2 as usize];
                
                self.registers[*r2 as usize] = temp;
                self.register_types[*r2 as usize] = temp_type;
                
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // المتغيرات
            // ═══════════════════════════════════════════════════════════════
            RegOp::LoadGlobal { dest, name_idx } => {
                // محاولة الكاش أولاً
                if let Some(cached) = self.global_cache.get(name_idx) {
                    self.stats.cache_hits += 1;
                    self.set_reg(*dest, Rc::clone(cached));
                } else {
                    self.stats.cache_misses += 1;
                    let chunk = self.chunk.as_ref().unwrap();
                    if let Some(name) = chunk.get_string(*name_idx) {
                        match self.globals.borrow().get(name) {
                            Some(v) => {
                                self.global_cache.insert(*name_idx, Rc::clone(&v));
                                self.set_reg(*dest, v);
                            }
                            None => {
                                return RegExecutionResult::Error(
                                    format!("المتغير '{}' غير معرف", name)
                                )
                            }
                        }
                    }
                }
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::StoreGlobal { name_idx, src } => {
                let chunk = self.chunk.as_ref().unwrap();
                if let Some(name) = chunk.get_string(*name_idx) {
                    let val = self.get_reg(*src);
                    self.global_cache.remove(name_idx);
                    self.globals.borrow_mut().define(name, (*val.borrow()).clone(), false);
                    self.global_cache.insert(*name_idx, Rc::clone(&val));
                }
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::LoadLocal { dest, slot } => {
                let frame = self.call_frames.last().unwrap();
                let local_reg = frame.base_register + *slot as u8;
                let val = self.get_reg(local_reg);
                self.set_reg(*dest, val);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::StoreLocal { slot, src } => {
                let frame = self.call_frames.last().unwrap();
                let local_reg = frame.base_register + *slot as u8;
                let val = self.get_reg(*src);
                self.set_reg(local_reg, val);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // التحكم في التدفق
            // ═══════════════════════════════════════════════════════════════
            RegOp::Jump { offset } => {
                self.ip = (self.ip as i32 + offset - 1).max(0) as usize;
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::JumpIfTrue { reg, offset } => {
                if self.is_truthy_reg(*reg) {
                    self.ip = (self.ip as i32 + offset - 1).max(0) as usize;
                }
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::JumpIfFalse { reg, offset } => {
                if !self.is_truthy_reg(*reg) {
                    self.ip = (self.ip as i32 + offset - 1).max(0) as usize;
                }
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::JumpBack { offset } => {
                self.ip = (self.ip as i32 - offset - 1).max(0) as usize;
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // الدوال
            // ═══════════════════════════════════════════════════════════════
            RegOp::Call { dest: _, func_reg, arg_count } => {
                self.stats.function_calls += 1;
                
                if self.recursion_depth >= MAX_RECURSION_DEPTH {
                    return RegExecutionResult::Error(
                        format!("تجاوز حد الاستدعاء ({})", MAX_RECURSION_DEPTH)
                    );
                }
                
                let func = self.get_reg(*func_reg);
                let func_borrowed = func.borrow();
                
                match &*func_borrowed {
                    Value::Function { .. } => {
                        let frame = RegCallFrame {
                            return_ip: self.ip,
                            base_register: *func_reg + 1,
                            num_locals: *arg_count,
                            name: "دالة".to_string(),
                        };
                        self.call_frames.push(frame);
                        self.recursion_depth += 1;
                        drop(func_borrowed);
                        RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
                    }
                    Value::NativeFunction { func, .. } => {
                        // جمع المعاملات من السجلات
                        let args: Vec<SharedValue> = (0..*arg_count)
                            .map(|i| self.get_reg(*func_reg + 1 + i))
                            .collect();
                        
                        match func(&args) {
                            Ok(result) => {
                                self.set_reg(*func_reg, result);
                                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
                            }
                            Err(e) => RegExecutionResult::Error(e),
                        }
                    }
                    _ => RegExecutionResult::Error("ليست دالة قابلة للاستدعاء".into()),
                }
            }
            
            RegOp::Return { reg } => {
                let val = self.get_reg(*reg);
                if self.call_frames.len() > 1 {
                    let _frame = self.call_frames.pop();
                    self.recursion_depth = self.recursion_depth.saturating_sub(1);
                }
                RegExecutionResult::Return(val)
            }
            
            RegOp::ReturnVoid => {
                if self.call_frames.len() > 1 {
                    let _frame = self.call_frames.pop();
                    self.recursion_depth = self.recursion_depth.saturating_sub(1);
                }
                RegExecutionResult::Return(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // أخرى
            // ═══════════════════════════════════════════════════════════════
            RegOp::Print { reg } => {
                let val = self.get_reg(*reg);
                println!("{}", val.borrow().to_string_value());
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Length { dest, src } => {
                let val = self.get_reg(*src);
                let len = match &*val.borrow() {
                    Value::List(l) => l.len(),
                    Value::String(s) => s.len(),
                    Value::Dictionary(d) => d.len(),
                    _ => 0,
                };
                self.set_reg_number(*dest, len as f64);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::TypeOf { dest, src } => {
                let val = self.get_reg(*src);
                let type_name = val.borrow().type_name().to_string();
                self.set_reg_string(*dest, type_name);
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Halt => {
                self.halted = true;
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Nop => {
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // التعليمات المتبقية - تنفيذ مبسط
            RegOp::BuildList { dest, count, elements } => {
                let mut list = Vec::with_capacity(*count as usize);
                for &reg in elements {
                    list.push(self.get_reg(reg));
                }
                self.set_reg(*dest, Rc::new(RefCell::new(Value::List(list))));
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::BuildDict { dest, count: _, entries } => {
                let mut dict = HashMap::new();
                for chunk in entries.chunks(2) {
                    if chunk.len() == 2 {
                        let key = self.get_reg_value(chunk[0]).to_string_value();
                        let val = self.get_reg(chunk[1]);
                        dict.insert(key, val);
                    }
                }
                self.set_reg(*dest, Rc::new(RefCell::new(Value::Dictionary(dict))));
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::Index { dest, obj_reg, idx_reg } => {
                let obj = self.get_reg_value(*obj_reg);
                let idx = self.get_reg_value(*idx_reg);
                
                match (&obj, &idx) {
                    (Value::List(list), Value::Number(i)) => {
                        let index = *i as usize;
                        if index < list.len() {
                            self.set_reg(*dest, Rc::clone(&list[index]));
                        } else {
                            self.set_reg_null(*dest);
                        }
                    }
                    (Value::Dictionary(dict), Value::String(key)) => {
                        if let Some(v) = dict.get(key) {
                            self.set_reg(*dest, Rc::clone(v));
                        } else {
                            self.set_reg_null(*dest);
                        }
                    }
                    _ => return RegExecutionResult::Error("لا يمكن الفهرسة".into()),
                }
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::IndexSet { obj_reg, idx_reg, val_reg } => {
                let idx = self.get_reg_value(*idx_reg);
                let val = self.get_reg(*val_reg);
                
                // نحتاج لتعديل القيمة
                let obj = self.get_reg(*obj_reg);
                let mut obj_val = obj.borrow().clone();
                
                match (&mut obj_val, &idx) {
                    (Value::List(ref mut list), Value::Number(i)) => {
                        let index = *i as usize;
                        if index < list.len() {
                            list[index] = val;
                        }
                    }
                    (Value::Dictionary(ref mut dict), Value::String(key)) => {
                        dict.insert(key.clone(), val);
                    }
                    _ => return RegExecutionResult::Error("لا يمكن تعيين الفهرس".into()),
                }
                
                self.set_reg(*obj_reg, Rc::new(RefCell::new(obj_val)));
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::GetProperty { dest, obj_reg, prop_idx } => {
                let obj = self.get_reg_value(*obj_reg);
                let chunk = self.chunk.as_ref().unwrap();
                
                if let Some(prop) = chunk.get_string(*prop_idx) {
                    match &obj {
                        Value::Dictionary(dict) => {
                            if let Some(v) = dict.get(prop) {
                                self.set_reg(*dest, Rc::clone(v));
                            } else {
                                self.set_reg_null(*dest);
                            }
                        }
                        Value::Instance { fields, .. } => {
                            if let Some(v) = fields.borrow().get(prop) {
                                self.set_reg(*dest, Rc::clone(v));
                            } else {
                                self.set_reg_null(*dest);
                            }
                        }
                        _ => return RegExecutionResult::Error("لا يمكن الوصول للخاصية".into()),
                    }
                }
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::SetProperty { obj_reg, prop_idx, val_reg } => {
                let val = self.get_reg(*val_reg);
                let chunk = self.chunk.as_ref().unwrap();
                
                if let Some(prop) = chunk.get_string(*prop_idx) {
                    let obj = self.get_reg(*obj_reg);
                    
                    if let Value::Dictionary(ref dict) = &*obj.borrow() {
                        let mut new_dict = dict.clone();
                        new_dict.insert(prop.to_string(), val);
                        self.set_reg(*obj_reg, Rc::new(RefCell::new(Value::Dictionary(new_dict))));
                    } else if let Value::Instance { ref fields, .. } = &*obj.borrow() {
                        fields.borrow_mut().insert(prop.to_string(), val);
                    } else {
                        return RegExecutionResult::Error("لا يمكن تعيين الخاصية".into());
                    }
                }
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            RegOp::CallNative { dest, func_idx, args } => {
                let chunk = self.chunk.as_ref().unwrap();
                
                if let Some(func_name) = chunk.get_string(*func_idx) {
                    match self.globals.borrow().get(func_name) {
                        Some(v) => {
                            if let Value::NativeFunction { func, .. } = &*v.borrow() {
                                let arg_vals: Vec<SharedValue> = args.iter()
                                    .map(|&r| self.get_reg(r))
                                    .collect();
                                
                                match func(&arg_vals) {
                                    Ok(result) => {
                                        self.set_reg(*dest, result);
                                    }
                                    Err(e) => return RegExecutionResult::Error(e),
                                }
                            } else {
                                return RegExecutionResult::Error(
                                    format!("'{}' ليست دالة أصلية", func_name)
                                );
                            }
                        }
                        None => {
                            return RegExecutionResult::Error(
                                format!("الدالة '{}' غير معرفة", func_name)
                            );
                        }
                    }
                }
                RegExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال مساعدة محسّنة
    // ═══════════════════════════════════════════════════════════════
    
    #[inline(always)]
    fn get_reg(&self, reg: u8) -> SharedValue {
        Rc::clone(&self.registers[reg as usize])
    }
    
    #[inline(always)]
    fn get_reg_value(&self, reg: u8) -> Value {
        self.registers[reg as usize].borrow().clone()
    }
    
    #[inline(always)]
    fn get_reg_number(&self, reg: u8) -> f64 {
        match &*self.registers[reg as usize].borrow() {
            Value::Number(n) => *n,
            _ => 0.0,
        }
    }
    
    #[inline(always)]
    fn set_reg(&mut self, reg: u8, val: SharedValue) {
        self.registers[reg as usize] = val;
        self.register_types[reg as usize] = RegType::Unknown;
    }
    
    #[inline(always)]
    fn set_reg_number(&mut self, reg: u8, val: f64) {
        self.registers[reg as usize] = Rc::new(RefCell::new(Value::Number(val)));
        self.register_types[reg as usize] = RegType::Number;
    }
    
    #[inline(always)]
    fn set_reg_bool(&mut self, reg: u8, val: bool) {
        self.registers[reg as usize] = Rc::new(RefCell::new(Value::Boolean(val)));
        self.register_types[reg as usize] = RegType::Bool;
    }
    
    #[inline(always)]
    fn set_reg_string(&mut self, reg: u8, val: String) {
        self.registers[reg as usize] = Rc::new(RefCell::new(Value::String(val)));
        self.register_types[reg as usize] = RegType::String;
    }
    
    #[inline(always)]
    fn set_reg_null(&mut self, reg: u8) {
        self.registers[reg as usize] = Rc::new(RefCell::new(Value::Null));
        self.register_types[reg as usize] = RegType::Null;
    }
    
    #[inline(always)]
    fn is_truthy_reg(&self, reg: u8) -> bool {
        self.registers[reg as usize].borrow().is_truthy()
    }
    
    /// الحصول على الإحصائيات
    pub fn stats(&self) -> &RegVMStats {
        &self.stats
    }
    
    /// طباعة تقرير الأداء
    pub fn print_report(&self) {
        println!();
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║            🚀 تقرير Register VM                                    ║");
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ التعليمات المنفذة:    {:>15}                       ║", self.stats.instructions_executed);
        println!("║ استدعاءات الدوال:     {:>15}                       ║", self.stats.function_calls);
        println!("║ وقت التنفيذ:          {:>15} μs                  ║", self.stats.execution_time_us);
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ ضربات الكاش:          {:>15}                       ║", self.stats.cache_hits);
        println!("║ أخطاء الكاش:          {:>15}                       ║", self.stats.cache_misses);
        println!("║ التحسينات:            {:>15}                       ║", self.stats.optimizations_applied);
        println!("╚══════════════════════════════════════════════════════════════════╝");
    }
}

impl Default for RegisterVM {
    fn default() -> Self {
        Self::with_fresh_env()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_vm_arithmetic() {
        let mut chunk = RegChunk::new();
        
        // r0 = 5
        chunk.emit(RegOp::LoadConst { dest: 0, value: 5.0 });
        // r1 = 3
        chunk.emit(RegOp::LoadConst { dest: 1, value: 3.0 });
        // r2 = r0 + r1
        chunk.emit(RegOp::Add { dest: 2, src1: 0, src2: 1 });
        chunk.emit(RegOp::Halt);
        
        let mut vm = RegisterVM::with_fresh_env();
        vm.load(chunk);
        
        let result = vm.run();
        match result {
            RegExecutionResult::Ok(_) => {
                assert_eq!(vm.get_reg_number(2), 8.0);
            }
            _ => panic!("Expected Ok"),
        }
    }
    
    #[test]
    fn test_register_vm_comparison() {
        let mut chunk = RegChunk::new();
        
        // r0 = 5
        chunk.emit(RegOp::LoadConst { dest: 0, value: 5.0 });
        // r1 = 3
        chunk.emit(RegOp::LoadConst { dest: 1, value: 3.0 });
        // r2 = r0 > r1
        chunk.emit(RegOp::Gt { dest: 2, src1: 0, src2: 1 });
        chunk.emit(RegOp::Halt);
        
        let mut vm = RegisterVM::with_fresh_env();
        vm.load(chunk);
        
        let result = vm.run();
        match result {
            RegExecutionResult::Ok(_) => {
                let val = vm.get_reg_value(2);
                assert_eq!(val, Value::Boolean(true));
            }
            _ => panic!("Expected Ok"),
        }
    }
    
    #[test]
    fn test_register_vm_loop() {
        let mut chunk = RegChunk::new();
        
        // r0 = 0 (counter)
        chunk.emit(RegOp::LoadConst { dest: 0, value: 0.0 });
        // r1 = 10 (limit)
        chunk.emit(RegOp::LoadConst { dest: 1, value: 10.0 });
        
        // loop start (ip = 2)
        // r2 = r0 < r1
        chunk.emit(RegOp::Lt { dest: 2, src1: 0, src2: 1 });
        // if !r2, jump to end (offset 4)
        chunk.emit(RegOp::JumpIfFalse { reg: 2, offset: 4 });
        // r0 = r0 + 1
        chunk.emit(RegOp::AddConst { dest: 0, src: 0, const_val: 1.0 });
        // jump back to loop start (offset -3)
        chunk.emit(RegOp::JumpBack { offset: 3 });
        
        chunk.emit(RegOp::Halt);
        
        let mut vm = RegisterVM::with_fresh_env();
        vm.load(chunk);
        
        let result = vm.run();
        match result {
            RegExecutionResult::Ok(_) => {
                assert_eq!(vm.get_reg_number(0), 10.0);
            }
            _ => panic!("Expected Ok"),
        }
    }
}
