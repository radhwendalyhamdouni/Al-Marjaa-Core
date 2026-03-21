// ═══════════════════════════════════════════════════════════════════════════════
// JIT Safety Systems - أنظمة أمان JIT
// ═══════════════════════════════════════════════════════════════════════════════
// Provides safety guarantees for JIT execution:
// - Memory usage guard
// - Execution limits
// - Safe bailout mechanism
// - Resource tracking
// ═══════════════════════════════════════════════════════════════════════════════

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// حد الذاكرة الافتراضي (512 MB)
pub const DEFAULT_MEMORY_LIMIT: usize = 512 * 1024 * 1024;

/// حد التعليمات الافتراضي (100 مليون)
pub const DEFAULT_INSTRUCTION_LIMIT: u64 = 100_000_000;

/// حد وقت التنفيذ الافتراضي (60 ثانية)
pub const DEFAULT_TIME_LIMIT: Duration = Duration::from_secs(60);

/// أقصى عمق للعودية
pub const MAX_RECURSION_DEPTH: usize = 10_000;

/// ═══════════════════════════════════════════════════════════════════════════════
/// حارس استخدام الذاكرة
/// ═══════════════════════════════════════════════════════════════════════════════
#[derive(Debug)]
pub struct MemoryGuard {
    /// حد الذاكرة بالبايت
    limit: AtomicUsize,
    /// الاستخدام الحالي
    current: AtomicUsize,
    /// ذروة الاستخدام
    peak: AtomicUsize,
    /// عدد التحذيرات
    warnings: AtomicU64,
    /// هل تم تجاوز الحد
    exceeded: AtomicBool,
}

impl MemoryGuard {
    pub fn new(limit: usize) -> Self {
        Self {
            limit: AtomicUsize::new(limit),
            current: AtomicUsize::new(0),
            peak: AtomicUsize::new(0),
            warnings: AtomicU64::new(0),
            exceeded: AtomicBool::new(false),
        }
    }

    /// تخصيص ذاكرة
    pub fn allocate(&self, size: usize) -> Result<(), MemoryError> {
        let current = self.current.fetch_add(size, Ordering::SeqCst) + size;
        
        // تحديث الذروة
        let mut peak = self.peak.load(Ordering::SeqCst);
        while current > peak {
            match self.peak.compare_exchange_weak(
                peak,
                current,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(p) => peak = p,
            }
        }

        let limit = self.limit.load(Ordering::SeqCst);
        if current > limit {
            self.warnings.fetch_add(1, Ordering::SeqCst);
            self.exceeded.store(true, Ordering::SeqCst);
            // التراجع عن التخصيص
            self.current.fetch_sub(size, Ordering::SeqCst);
            Err(MemoryError::LimitExceeded {
                current,
                limit,
            })
        } else if current > limit * 90 / 100 {
            // تحذير عند 90%
            self.warnings.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Ok(())
        }
    }

    /// تحرير ذاكرة
    pub fn deallocate(&self, size: usize) {
        let current = self.current.fetch_sub(size, Ordering::SeqCst);
        if current < size {
            // تصحيح إذا كان السالب
            self.current.store(0, Ordering::SeqCst);
        }
    }

    /// الحصول على الإحصائيات
    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            current: self.current.load(Ordering::SeqCst),
            peak: self.peak.load(Ordering::SeqCst),
            limit: self.limit.load(Ordering::SeqCst),
            warnings: self.warnings.load(Ordering::SeqCst),
            exceeded: self.exceeded.load(Ordering::SeqCst),
        }
    }

    /// إعادة تعيين الحارس
    pub fn reset(&self) {
        self.current.store(0, Ordering::SeqCst);
        self.peak.store(0, Ordering::SeqCst);
        self.warnings.store(0, Ordering::SeqCst);
        self.exceeded.store(false, Ordering::SeqCst);
    }
}

impl Default for MemoryGuard {
    fn default() -> Self {
        Self::new(DEFAULT_MEMORY_LIMIT)
    }
}

/// إحصائيات الذاكرة
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub current: usize,
    pub peak: usize,
    pub limit: usize,
    pub warnings: u64,
    pub exceeded: bool,
}

/// أخطاء الذاكرة
#[derive(Debug, Clone)]
pub enum MemoryError {
    LimitExceeded { current: usize, limit: usize },
    AllocationFailed { size: usize },
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LimitExceeded { current, limit } => {
                write!(f, "تجاوز حد الذاكرة: {} > {} بايت", current, limit)
            }
            Self::AllocationFailed { size } => {
                write!(f, "فشل تخصيص {} بايت", size)
            }
        }
    }
}

impl std::error::Error for MemoryError {}

/// ═══════════════════════════════════════════════════════════════════════════════
/// حدود التنفيذ
/// ═══════════════════════════════════════════════════════════════════════════════
#[derive(Debug, Clone)]
pub struct ExecutionLimits {
    /// حد التعليمات
    pub max_instructions: u64,
    /// حد الوقت
    pub time_limit: Duration,
    /// حد عمق العودية
    pub max_recursion_depth: usize,
    /// حد حجم المكدس
    pub max_stack_size: usize,
    /// حد عدد الدوال
    pub max_functions: usize,
    /// حد حجم الكود
    pub max_code_size: usize,
}

impl Default for ExecutionLimits {
    fn default() -> Self {
        Self {
            max_instructions: DEFAULT_INSTRUCTION_LIMIT,
            time_limit: DEFAULT_TIME_LIMIT,
            max_recursion_depth: MAX_RECURSION_DEPTH,
            max_stack_size: 65_536,
            max_functions: 10_000,
            max_code_size: 10_000_000,
        }
    }
}

impl ExecutionLimits {
    pub fn strict() -> Self {
        Self {
            max_instructions: 1_000_000,
            time_limit: Duration::from_secs(5),
            max_recursion_depth: 1_000,
            max_stack_size: 4_096,
            max_functions: 100,
            max_code_size: 100_000,
        }
    }

    pub fn relaxed() -> Self {
        Self {
            max_instructions: 1_000_000_000,
            time_limit: Duration::from_secs(300),
            max_recursion_depth: 50_000,
            max_stack_size: 262_144,
            max_functions: 100_000,
            max_code_size: 100_000_000,
        }
    }
}

/// ═══════════════════════════════════════════════════════════════════════════════
/// مراقب التنفيذ
/// ═══════════════════════════════════════════════════════════════════════════════
#[derive(Debug)]
pub struct ExecutionMonitor {
    /// حدود التنفيذ
    limits: ExecutionLimits,
    /// عدد التعليمات المنفذة
    instructions_executed: AtomicU64,
    /// وقت البدء
    start_time: Option<Instant>,
    /// عمق العودية الحالي
    recursion_depth: AtomicUsize,
    /// أقصى عمق وصل إليه
    max_recursion_reached: AtomicUsize,
    /// هل تم الإلغاء
    cancelled: AtomicBool,
    /// سبب الإلغاء
    cancel_reason: std::sync::Mutex<Option<String>>,
}

impl ExecutionMonitor {
    pub fn new(limits: ExecutionLimits) -> Self {
        Self {
            limits,
            instructions_executed: AtomicU64::new(0),
            start_time: None,
            recursion_depth: AtomicUsize::new(0),
            max_recursion_reached: AtomicUsize::new(0),
            cancelled: AtomicBool::new(false),
            cancel_reason: std::sync::Mutex::new(None),
        }
    }

    /// بدء التنفيذ
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.instructions_executed.store(0, Ordering::SeqCst);
        self.recursion_depth.store(0, Ordering::SeqCst);
        self.max_recursion_reached.store(0, Ordering::SeqCst);
        self.cancelled.store(false, Ordering::SeqCst);
        *self.cancel_reason.lock().unwrap() = None;
    }

    /// تسجيل تعليمة
    pub fn tick(&self) -> Result<(), ExecutionError> {
        // التحقق من الإلغاء
        if self.cancelled.load(Ordering::SeqCst) {
            return Err(ExecutionError::Cancelled(
                self.cancel_reason.lock().unwrap().clone().unwrap_or_else(|| "غير محدد".to_string())
            ));
        }

        // التحقق من عدد التعليمات
        let instructions = self.instructions_executed.fetch_add(1, Ordering::SeqCst) + 1;
        if instructions > self.limits.max_instructions {
            self.cancelled.store(true, Ordering::SeqCst);
            *self.cancel_reason.lock().unwrap() = Some("تجاوز حد التعليمات".to_string());
            return Err(ExecutionError::InstructionLimitExceeded {
                executed: instructions,
                limit: self.limits.max_instructions,
            });
        }

        // التحقق من الوقت
        if let Some(start) = self.start_time {
            if start.elapsed() > self.limits.time_limit {
                self.cancelled.store(true, Ordering::SeqCst);
                *self.cancel_reason.lock().unwrap() = Some("تجاوز حد الوقت".to_string());
                return Err(ExecutionError::TimeLimitExceeded {
                    elapsed: start.elapsed(),
                    limit: self.limits.time_limit,
                });
            }
        }

        Ok(())
    }

    /// دخول دالة (عودية)
    pub fn enter_function(&self) -> Result<(), ExecutionError> {
        let depth = self.recursion_depth.fetch_add(1, Ordering::SeqCst) + 1;
        
        // تحديث أقصى عمق
        let mut max = self.max_recursion_reached.load(Ordering::SeqCst);
        while depth > max {
            match self.max_recursion_reached.compare_exchange_weak(
                max,
                depth,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(m) => max = m,
            }
        }

        if depth > self.limits.max_recursion_depth {
            self.cancelled.store(true, Ordering::SeqCst);
            *self.cancel_reason.lock().unwrap() = Some("تجاوز عمق العودية".to_string());
            return Err(ExecutionError::RecursionDepthExceeded {
                depth,
                limit: self.limits.max_recursion_depth,
            });
        }

        Ok(())
    }

    /// خروج من دالة
    pub fn exit_function(&self) {
        let depth = self.recursion_depth.fetch_sub(1, Ordering::SeqCst);
        if depth == 0 {
            // تصحيح إذا كان صفر
            self.recursion_depth.store(0, Ordering::SeqCst);
        }
    }

    /// إلغاء التنفيذ
    pub fn cancel(&self, reason: &str) {
        self.cancelled.store(true, Ordering::SeqCst);
        *self.cancel_reason.lock().unwrap() = Some(reason.to_string());
    }

    /// التحقق من الإلغاء
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    /// الحصول على الإحصائيات
    pub fn stats(&self) -> ExecutionStats {
        ExecutionStats {
            instructions_executed: self.instructions_executed.load(Ordering::SeqCst),
            elapsed: self.start_time.map(|s| s.elapsed()).unwrap_or_default(),
            current_recursion_depth: self.recursion_depth.load(Ordering::SeqCst),
            max_recursion_reached: self.max_recursion_reached.load(Ordering::SeqCst),
            cancelled: self.cancelled.load(Ordering::SeqCst),
        }
    }
}

impl Default for ExecutionMonitor {
    fn default() -> Self {
        Self::new(ExecutionLimits::default())
    }
}

/// إحصائيات التنفيذ
#[derive(Debug, Clone)]
pub struct ExecutionStats {
    pub instructions_executed: u64,
    pub elapsed: Duration,
    pub current_recursion_depth: usize,
    pub max_recursion_reached: usize,
    pub cancelled: bool,
}

/// أخطاء التنفيذ
#[derive(Debug, Clone)]
pub enum ExecutionError {
    InstructionLimitExceeded { executed: u64, limit: u64 },
    TimeLimitExceeded { elapsed: Duration, limit: Duration },
    RecursionDepthExceeded { depth: usize, limit: usize },
    StackOverflow { size: usize, limit: usize },
    Cancelled(String),
    InternalError(String),
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InstructionLimitExceeded { executed, limit } => {
                write!(f, "تجاوز حد التعليمات: {} > {}", executed, limit)
            }
            Self::TimeLimitExceeded { elapsed, limit } => {
                write!(f, "تجاوز حد الوقت: {:?} > {:?}", elapsed, limit)
            }
            Self::RecursionDepthExceeded { depth, limit } => {
                write!(f, "تجاوز عمق العودية: {} > {}", depth, limit)
            }
            Self::StackOverflow { size, limit } => {
                write!(f, "تجاوز حجم المكدس: {} > {}", size, limit)
            }
            Self::Cancelled(reason) => {
                write!(f, "تم إلغاء التنفيذ: {}", reason)
            }
            Self::InternalError(msg) => {
                write!(f, "خطأ داخلي: {}", msg)
            }
        }
    }
}

impl std::error::Error for ExecutionError {}

/// ═══════════════════════════════════════════════════════════════════════════════
/// آلية الخروج الآمن (Safe Bailout)
/// ═══════════════════════════════════════════════════════════════════════════════
pub struct SafeBailout {
    /// راية الإلغاء
    flag: Arc<AtomicBool>,
    /// سبب الإلغاء
    reason: std::sync::Mutex<Option<String>>,
}

impl SafeBailout {
    pub fn new() -> Self {
        Self {
            flag: Arc::new(AtomicBool::new(false)),
            reason: std::sync::Mutex::new(None),
        }
    }

    /// طلب الخروج
    pub fn request_bailout(&self, reason: &str) {
        self.flag.store(true, Ordering::SeqCst);
        *self.reason.lock().unwrap() = Some(reason.to_string());
    }

    /// التحقق من طلب الخروج
    pub fn should_bailout(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }

    /// الحصول على السبب
    pub fn reason(&self) -> Option<String> {
        self.reason.lock().unwrap().clone()
    }

    /// إعادة التعيين
    pub fn reset(&self) {
        self.flag.store(false, Ordering::SeqCst);
        *self.reason.lock().unwrap() = None;
    }

    /// إنشاء نسخة للمشاركة
    pub fn share(&self) -> BailoutHandle {
        BailoutHandle {
            flag: Arc::clone(&self.flag),
        }
    }
}

impl Default for SafeBailout {
    fn default() -> Self {
        Self::new()
    }
}

/// مقبض للتحقق من الخروج (للمشاركة بين الخيوط)
#[derive(Clone)]
pub struct BailoutHandle {
    flag: Arc<AtomicBool>,
}

impl BailoutHandle {
    pub fn should_bailout(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }
}

/// ═══════════════════════════════════════════════════════════════════════════════
/// مدير الموارد المتكامل
/// ═══════════════════════════════════════════════════════════════════════════════
pub struct ResourceManager {
    memory_guard: MemoryGuard,
    execution_monitor: ExecutionMonitor,
    bailout: SafeBailout,
    limits: ExecutionLimits,
}

impl ResourceManager {
    pub fn new(limits: ExecutionLimits) -> Self {
        let memory_limit = limits.max_stack_size * 10; // تقدير
        Self {
            memory_guard: MemoryGuard::new(memory_limit),
            execution_monitor: ExecutionMonitor::new(limits.clone()),
            bailout: SafeBailout::new(),
            limits,
        }
    }

    /// بدء تنفيذ جديد
    pub fn start_execution(&mut self) {
        self.execution_monitor.start();
        self.bailout.reset();
    }

    /// التحقق من جميع الحدود
    pub fn check_limits(&self) -> Result<(), ExecutionError> {
        // التحقق من الخروج المطلوب
        if self.bailout.should_bailout() {
            return Err(ExecutionError::Cancelled(
                self.bailout.reason().unwrap_or_else(|| "غير محدد".to_string())
            ));
        }

        // التحقق من التعليمات والوقت
        self.execution_monitor.tick()?;

        Ok(())
    }

    /// تخصيص ذاكرة
    pub fn allocate_memory(&self, size: usize) -> Result<(), MemoryError> {
        self.memory_guard.allocate(size)
    }

    /// تحرير ذاكرة
    pub fn deallocate_memory(&self, size: usize) {
        self.memory_guard.deallocate(size)
    }

    /// دخول دالة
    pub fn enter_function(&self) -> Result<(), ExecutionError> {
        self.execution_monitor.enter_function()
    }

    /// خروج من دالة
    pub fn exit_function(&self) {
        self.execution_monitor.exit_function()
    }

    /// طلب الخروج الآمن
    pub fn request_bailout(&self, reason: &str) {
        self.bailout.request_bailout(reason);
    }

    /// الحصول على مقبض الخروج
    pub fn bailout_handle(&self) -> BailoutHandle {
        self.bailout.share()
    }

    /// تقرير شامل
    pub fn report(&self) -> ResourceReport {
        ResourceReport {
            memory: self.memory_guard.stats(),
            execution: self.execution_monitor.stats(),
            limits: self.limits.clone(),
        }
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new(ExecutionLimits::default())
    }
}

/// تقرير الموارد
#[derive(Debug, Clone)]
pub struct ResourceReport {
    pub memory: MemoryStats,
    pub execution: ExecutionStats,
    pub limits: ExecutionLimits,
}

impl ResourceReport {
    pub fn print(&self) {
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║               تقرير الموارد - Resource Report                    ║");
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ الذاكرة:                                                          ║");
        println!("║   الحالي: {} بايت", self.memory.current);
        println!("║   الذروة: {} بايت", self.memory.peak);
        println!("║   الحد:   {} بايت", self.memory.limit);
        println!("║   تحذيرات: {}", self.memory.warnings);
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ التنفيذ:                                                          ║");
        println!("║   التعليمات: {}", self.execution.instructions_executed);
        println!("║   الوقت: {:?}", self.execution.elapsed);
        println!("║   عمق العودية: {}", self.execution.current_recursion_depth);
        println!("║   أقصى عمق: {}", self.execution.max_recursion_reached);
        println!("║   ملغي: {}", self.execution.cancelled);
        println!("╚══════════════════════════════════════════════════════════════════╝");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_guard_basic() {
        let guard = MemoryGuard::new(1000);
        
        assert!(guard.allocate(500).is_ok());
        assert!(guard.allocate(400).is_ok());
        
        let stats = guard.stats();
        assert_eq!(stats.current, 900);
        assert_eq!(stats.peak, 900);
        
        guard.deallocate(200);
        let stats = guard.stats();
        assert_eq!(stats.current, 700);
        assert_eq!(stats.peak, 900);
    }

    #[test]
    fn test_memory_guard_limit() {
        let guard = MemoryGuard::new(100);
        
        assert!(guard.allocate(50).is_ok());
        assert!(guard.allocate(60).is_err()); // يتجاوز الحد
        
        let stats = guard.stats();
        assert!(stats.exceeded);
    }

    #[test]
    fn test_execution_monitor_instructions() {
        let mut monitor = ExecutionMonitor::new(ExecutionLimits {
            max_instructions: 10,
            ..Default::default()
        });
        
        monitor.start();
        
        for _ in 0..10 {
            assert!(monitor.tick().is_ok());
        }
        
        // التعليمة 11 يجب أن تفشل
        assert!(monitor.tick().is_err());
    }

    #[test]
    fn test_execution_monitor_recursion() {
        let monitor = ExecutionMonitor::new(ExecutionLimits {
            max_recursion_depth: 5,
            ..Default::default()
        });
        
        for _ in 0..5 {
            assert!(monitor.enter_function().is_ok());
        }
        
        // الدخول السادس يجب أن يفشل
        assert!(monitor.enter_function().is_err());
        
        let stats = monitor.stats();
        assert_eq!(stats.max_recursion_reached, 6);
    }

    #[test]
    fn test_safe_bailout() {
        let bailout = SafeBailout::new();
        
        assert!(!bailout.should_bailout());
        
        bailout.request_bailout("اختبار");
        
        assert!(bailout.should_bailout());
        assert_eq!(bailout.reason(), Some("اختبار".to_string()));
        
        bailout.reset();
        assert!(!bailout.should_bailout());
    }

    #[test]
    fn test_bailout_handle() {
        let bailout = SafeBailout::new();
        let handle = bailout.share();
        
        assert!(!handle.should_bailout());
        
        bailout.request_bailout("من الرئيسي");
        
        assert!(handle.should_bailout());
    }

    #[test]
    fn test_resource_manager() {
        let mut rm = ResourceManager::new(ExecutionLimits::strict());
        
        rm.start_execution();
        
        assert!(rm.check_limits().is_ok());
        assert!(rm.allocate_memory(100).is_ok());
        assert!(rm.enter_function().is_ok());
        
        rm.exit_function();
        rm.deallocate_memory(100);
        
        let report = rm.report();
        assert!(report.execution.instructions_executed >= 1);
    }

    #[test]
    fn test_strict_limits() {
        let limits = ExecutionLimits::strict();
        
        assert_eq!(limits.max_instructions, 1_000_000);
        assert_eq!(limits.time_limit, Duration::from_secs(5));
        assert_eq!(limits.max_recursion_depth, 1_000);
    }

    #[test]
    fn test_relaxed_limits() {
        let limits = ExecutionLimits::relaxed();
        
        assert_eq!(limits.max_instructions, 1_000_000_000);
        assert_eq!(limits.time_limit, Duration::from_secs(300));
        assert_eq!(limits.max_recursion_depth, 50_000);
    }
}
