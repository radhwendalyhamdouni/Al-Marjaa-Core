// ═══════════════════════════════════════════════════════════════════════════════
// JIT Safety Systems - أنظمة أمان JIT
// ═══════════════════════════════════════════════════════════════════════════════
// Provides safety guarantees for JIT execution:
// - Memory usage guards
// - Execution time limits
// - Safe bailout mechanisms
// - Resource tracking
// ═══════════════════════════════════════════════════════════════════════════════

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

/// Maximum memory that can be allocated (in bytes)
const DEFAULT_MAX_MEMORY: usize = 512 * 1024 * 1024; // 512 MB

/// Maximum execution time (in milliseconds)
const DEFAULT_MAX_EXECUTION_TIME_MS: u64 = 60_000; // 60 seconds

/// Maximum call depth
const DEFAULT_MAX_CALL_DEPTH: usize = 10_000;

/// Maximum stack size
const DEFAULT_MAX_STACK_SIZE: usize = 1_000_000;

/// Maximum loop iterations before check
const ITERATIONS_PER_CHECK: u64 = 10_000;

// ═══════════════════════════════════════════════════════════════════════════════
// Resource Limits Configuration
// ═══════════════════════════════════════════════════════════════════════════════

/// Configuration for execution limits
#[derive(Debug, Clone)]
pub struct ExecutionLimits {
    /// Maximum memory in bytes
    pub max_memory: usize,
    /// Maximum execution time
    pub max_time: Duration,
    /// Maximum call stack depth
    pub max_call_depth: usize,
    /// Maximum stack size (number of values)
    pub max_stack_size: usize,
    /// Maximum number of operations
    pub max_operations: u64,
    /// Enable bailout on limit exceeded
    pub enable_bailout: bool,
}

impl Default for ExecutionLimits {
    fn default() -> Self {
        ExecutionLimits {
            max_memory: DEFAULT_MAX_MEMORY,
            max_time: Duration::from_millis(DEFAULT_MAX_EXECUTION_TIME_MS),
            max_call_depth: DEFAULT_MAX_CALL_DEPTH,
            max_stack_size: DEFAULT_MAX_STACK_SIZE,
            max_operations: 1_000_000_000, // 1 billion ops
            enable_bailout: true,
        }
    }
}

impl ExecutionLimits {
    /// Create limits for development (more lenient)
    pub fn development() -> Self {
        ExecutionLimits {
            max_memory: 1024 * 1024 * 1024, // 1 GB
            max_time: Duration::from_secs(300), // 5 minutes
            max_call_depth: 50_000,
            max_stack_size: 10_000_000,
            max_operations: 10_000_000_000,
            enable_bailout: true,
        }
    }

    /// Create limits for production (stricter)
    pub fn production() -> Self {
        ExecutionLimits {
            max_memory: 256 * 1024 * 1024, // 256 MB
            max_time: Duration::from_secs(30),
            max_call_depth: 5_000,
            max_stack_size: 500_000,
            max_operations: 100_000_000,
            enable_bailout: true,
        }
    }

    /// Create limits for testing (very strict)
    pub fn testing() -> Self {
        ExecutionLimits {
            max_memory: 64 * 1024 * 1024, // 64 MB
            max_time: Duration::from_secs(5),
            max_call_depth: 1_000,
            max_stack_size: 100_000,
            max_operations: 10_000_000,
            enable_bailout: true,
        }
    }

    /// Create unlimited limits (dangerous!)
    pub fn unlimited() -> Self {
        ExecutionLimits {
            max_memory: usize::MAX,
            max_time: Duration::from_secs(u64::MAX),
            max_call_depth: usize::MAX,
            max_stack_size: usize::MAX,
            max_operations: u64::MAX,
            enable_bailout: false,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Bailout Reason
// ═══════════════════════════════════════════════════════════════════════════════

/// Reason for execution bailout
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BailoutReason {
    /// Execution time exceeded
    Timeout {
        elapsed_ms: u64,
        limit_ms: u64,
    },
    /// Memory limit exceeded
    MemoryExceeded {
        used_bytes: usize,
        limit_bytes: usize,
    },
    /// Call stack depth exceeded
    StackOverflow {
        depth: usize,
        limit: usize,
    },
    /// Operation count exceeded
    OperationLimit {
        count: u64,
        limit: u64,
    },
    /// Stack size exceeded
    StackSizeExceeded {
        size: usize,
        limit: usize,
    },
    /// External interrupt
    Interrupted,
    /// Custom reason
    Custom(String),
}

impl std::fmt::Display for BailoutReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BailoutReason::Timeout { elapsed_ms, limit_ms } => {
                write!(f, "انتهت مهلة التنفيذ: {} مللي ثانية (الحد: {} مللي ثانية)", elapsed_ms, limit_ms)
            }
            BailoutReason::MemoryExceeded { used_bytes, limit_bytes } => {
                write!(f, "تجاوز حد الذاكرة: {} بايت (الحد: {} بايت)", used_bytes, limit_bytes)
            }
            BailoutReason::StackOverflow { depth, limit } => {
                write!(f, "تجاوز عمق الاستدعاء: {} (الحد: {})", depth, limit)
            }
            BailoutReason::OperationLimit { count, limit } => {
                write!(f, "تجاوز حد العمليات: {} (الحد: {})", count, limit)
            }
            BailoutReason::StackSizeExceeded { size, limit } => {
                write!(f, "تجاوز حجم المكدس: {} (الحد: {})", size, limit)
            }
            BailoutReason::Interrupted => {
                write!(f, "تم إيقاف التنفيذ خارجياً")
            }
            BailoutReason::Custom(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Execution Guard
// ═══════════════════════════════════════════════════════════════════════════════

/// Thread-safe execution guard for resource monitoring
pub struct ExecutionGuard {
    /// Configuration limits
    limits: ExecutionLimits,
    /// Start time
    start_time: Instant,
    /// Operation count
    operation_count: AtomicU64,
    /// Current call depth
    call_depth: AtomicUsize,
    /// Current stack size
    stack_size: AtomicUsize,
    /// Memory allocated (approximate)
    memory_used: AtomicUsize,
    /// Bailout flag
    bailout: AtomicBool,
    /// Bailout reason
    bailout_reason: RwLock<Option<BailoutReason>>,
    /// External interrupt flag
    interrupted: AtomicBool,
}

impl ExecutionGuard {
    /// Create a new execution guard with default limits
    pub fn new() -> Self {
        Self::with_limits(ExecutionLimits::default())
    }

    /// Create a new execution guard with custom limits
    pub fn with_limits(limits: ExecutionLimits) -> Self {
        ExecutionGuard {
            limits,
            start_time: Instant::now(),
            operation_count: AtomicU64::new(0),
            call_depth: AtomicUsize::new(0),
            stack_size: AtomicUsize::new(0),
            memory_used: AtomicUsize::new(0),
            bailout: AtomicBool::new(false),
            bailout_reason: RwLock::new(None),
            interrupted: AtomicBool::new(false),
        }
    }

    /// Check if execution should continue
    /// Returns Err(reason) if should bail out
    #[inline(always)]
    pub fn check(&self) -> Result<(), BailoutReason> {
        if !self.limits.enable_bailout {
            return Ok(());
        }

        // Check bailout flag first (fast path)
        if self.bailout.load(Ordering::Relaxed) {
            let reason = self.bailout_reason.read().unwrap().clone();
            return Err(reason.unwrap_or(BailoutReason::Interrupted));
        }

        // Check interrupt
        if self.interrupted.load(Ordering::Relaxed) {
            return Err(BailoutReason::Interrupted);
        }

        // Check time (less frequent)
        if self.operation_count.load(Ordering::Relaxed) % ITERATIONS_PER_CHECK == 0 {
            let elapsed = self.start_time.elapsed();
            if elapsed > self.limits.max_time {
                self.trigger_bailout(BailoutReason::Timeout {
                    elapsed_ms: elapsed.as_millis() as u64,
                    limit_ms: self.limits.max_time.as_millis() as u64,
                });
            }
        }

        Ok(())
    }

    /// Record an operation
    #[inline(always)]
    pub fn record_operation(&self) -> Result<(), BailoutReason> {
        let count = self.operation_count.fetch_add(1, Ordering::Relaxed) + 1;
        
        if count > self.limits.max_operations {
            self.trigger_bailout(BailoutReason::OperationLimit {
                count,
                limit: self.limits.max_operations,
            });
        }
        
        // Periodic check
        if count % ITERATIONS_PER_CHECK == 0 {
            self.check()?;
        }
        
        Ok(())
    }

    /// Enter a function call
    #[inline(always)]
    pub fn enter_call(&self) -> Result<(), BailoutReason> {
        let depth = self.call_depth.fetch_add(1, Ordering::Relaxed) + 1;
        
        if depth > self.limits.max_call_depth {
            self.trigger_bailout(BailoutReason::StackOverflow {
                depth,
                limit: self.limits.max_call_depth,
            });
        }
        
        Ok(())
    }

    /// Exit a function call
    #[inline(always)]
    pub fn exit_call(&self) {
        self.call_depth.fetch_sub(1, Ordering::Relaxed);
    }

    /// Update stack size
    #[inline(always)]
    pub fn update_stack_size(&self, size: usize) -> Result<(), BailoutReason> {
        self.stack_size.store(size, Ordering::Relaxed);
        
        if size > self.limits.max_stack_size {
            self.trigger_bailout(BailoutReason::StackSizeExceeded {
                size,
                limit: self.limits.max_stack_size,
            });
        }
        
        Ok(())
    }

    /// Record memory allocation
    #[inline(always)]
    pub fn record_memory(&self, bytes: usize) -> Result<(), BailoutReason> {
        let used = self.memory_used.fetch_add(bytes, Ordering::Relaxed) + bytes;
        
        if used > self.limits.max_memory {
            self.trigger_bailout(BailoutReason::MemoryExceeded {
                used_bytes: used,
                limit_bytes: self.limits.max_memory,
            });
        }
        
        Ok(())
    }

    /// Record memory deallocation
    #[inline(always)]
    pub fn release_memory(&self, bytes: usize) {
        self.memory_used.fetch_sub(bytes, Ordering::Relaxed);
    }

    /// Trigger a bailout
    pub fn trigger_bailout(&self, reason: BailoutReason) {
        *self.bailout_reason.write().unwrap() = Some(reason.clone());
        self.bailout.store(true, Ordering::Release);
    }

    /// Interrupt execution externally
    pub fn interrupt(&self) {
        self.interrupted.store(true, Ordering::Release);
    }

    /// Check if bailout has been triggered
    pub fn is_bailed_out(&self) -> bool {
        self.bailout.load(Ordering::Relaxed)
    }

    /// Get bailout reason
    pub fn get_bailout_reason(&self) -> Option<BailoutReason> {
        self.bailout_reason.read().unwrap().clone()
    }

    /// Reset the guard for reuse
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
        self.operation_count.store(0, Ordering::Relaxed);
        self.call_depth.store(0, Ordering::Relaxed);
        self.stack_size.store(0, Ordering::Relaxed);
        self.memory_used.store(0, Ordering::Relaxed);
        self.bailout.store(false, Ordering::Relaxed);
        *self.bailout_reason.write().unwrap() = None;
        self.interrupted.store(false, Ordering::Relaxed);
    }

    /// Get current statistics
    pub fn stats(&self) -> ExecutionStats {
        ExecutionStats {
            elapsed_time: self.start_time.elapsed(),
            operation_count: self.operation_count.load(Ordering::Relaxed),
            call_depth: self.call_depth.load(Ordering::Relaxed),
            stack_size: self.stack_size.load(Ordering::Relaxed),
            memory_used: self.memory_used.load(Ordering::Relaxed),
        }
    }
}

impl Default for ExecutionGuard {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Execution Statistics
// ═══════════════════════════════════════════════════════════════════════════════

/// Statistics about execution
#[derive(Debug, Clone)]
pub struct ExecutionStats {
    /// Elapsed time
    pub elapsed_time: Duration,
    /// Number of operations
    pub operation_count: u64,
    /// Current call depth
    pub call_depth: usize,
    /// Current stack size
    pub stack_size: usize,
    /// Memory used
    pub memory_used: usize,
}

impl std::fmt::Display for ExecutionStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "═══════════════════════════════════════════════════════════════")?;
        writeln!(f, "                    إحصائيات التنفيذ                          ")?;
        writeln!(f, "═══════════════════════════════════════════════════════════════")?;
        writeln!(f, "  الوقت المنقضي:      {:?}", self.elapsed_time)?;
        writeln!(f, "  عدد العمليات:       {}", self.operation_count)?;
        writeln!(f, "  عمق الاستدعاء:      {}", self.call_depth)?;
        writeln!(f, "  حجم المكدس:         {}", self.stack_size)?;
        writeln!(f, "  الذاكرة المستخدمة:  {} بايت ({:.2} MB)", 
            self.memory_used,
            self.memory_used as f64 / (1024.0 * 1024.0)
        )?;
        writeln!(f, "═══════════════════════════════════════════════════════════════")?;
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Memory Usage Tracker
// ═══════════════════════════════════════════════════════════════════════════════

/// Thread-safe memory tracker
pub struct MemoryTracker {
    /// Current allocated bytes
    allocated: AtomicUsize,
    /// Peak allocated bytes
    peak: AtomicUsize,
    /// Number of allocations
    allocations: AtomicU64,
    /// Number of deallocations
    deallocations: AtomicU64,
    /// Limit
    limit: usize,
}

impl MemoryTracker {
    pub fn new(limit: usize) -> Self {
        MemoryTracker {
            allocated: AtomicUsize::new(0),
            peak: AtomicUsize::new(0),
            allocations: AtomicU64::new(0),
            deallocations: AtomicU64::new(0),
            limit,
        }
    }

    /// Allocate memory, returns false if limit exceeded
    pub fn allocate(&self, bytes: usize) -> bool {
        let current = self.allocated.fetch_add(bytes, Ordering::Relaxed) + bytes;
        
        // Update peak
        let mut peak = self.peak.load(Ordering::Relaxed);
        while current > peak {
            match self.peak.compare_exchange_weak(
                peak,
                current,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(p) => peak = p,
            }
        }
        
        self.allocations.fetch_add(1, Ordering::Relaxed);
        
        current <= self.limit
    }

    /// Deallocate memory
    pub fn deallocate(&self, bytes: usize) {
        self.allocated.fetch_sub(bytes, Ordering::Relaxed);
        self.deallocations.fetch_add(1, Ordering::Relaxed);
    }

    /// Get current usage
    pub fn current(&self) -> usize {
        self.allocated.load(Ordering::Relaxed)
    }

    /// Get peak usage
    pub fn peak(&self) -> usize {
        self.peak.load(Ordering::Relaxed)
    }

    /// Get statistics
    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            current: self.allocated.load(Ordering::Relaxed),
            peak: self.peak.load(Ordering::Relaxed),
            allocations: self.allocations.load(Ordering::Relaxed),
            deallocations: self.deallocations.load(Ordering::Relaxed),
        }
    }
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub current: usize,
    pub peak: usize,
    pub allocations: u64,
    pub deallocations: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Scoped Guard
// ═══════════════════════════════════════════════════════════════════════════════

/// RAII-style scope guard for function calls
pub struct CallScopeGuard<'a> {
    guard: &'a ExecutionGuard,
}

impl<'a> CallScopeGuard<'a> {
    pub fn new(guard: &'a ExecutionGuard) -> Result<Self, BailoutReason> {
        guard.enter_call()?;
        Ok(CallScopeGuard { guard })
    }
}

impl<'a> Drop for CallScopeGuard<'a> {
    fn drop(&mut self) {
        self.guard.exit_call();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_execution_guard_creation() {
        let guard = ExecutionGuard::new();
        assert!(!guard.is_bailed_out());
        assert_eq!(guard.stats().operation_count, 0);
    }

    #[test]
    fn test_operation_counting() {
        let guard = ExecutionGuard::new();
        
        for _ in 0..100 {
            guard.record_operation().unwrap();
        }
        
        assert_eq!(guard.stats().operation_count, 100);
    }

    #[test]
    fn test_call_depth_tracking() {
        let guard = ExecutionGuard::new();
        
        {
            let _scope1 = CallScopeGuard::new(&guard).unwrap();
            assert_eq!(guard.stats().call_depth, 1);
            
            {
                let _scope2 = CallScopeGuard::new(&guard).unwrap();
                assert_eq!(guard.stats().call_depth, 2);
            }
            
            assert_eq!(guard.stats().call_depth, 1);
        }
        
        assert_eq!(guard.stats().call_depth, 0);
    }

    #[test]
    fn test_stack_overflow_detection() {
        let limits = ExecutionLimits {
            max_call_depth: 10,
            ..ExecutionLimits::default()
        };
        
        let guard = ExecutionGuard::with_limits(limits);
        
        for _ in 0..10 {
            guard.enter_call().unwrap();
        }
        
        // 11th call should fail
        let result = guard.enter_call();
        assert!(result.is_err());
        
        match result {
            Err(BailoutReason::StackOverflow { depth, limit }) => {
                assert_eq!(limit, 10);
                assert!(depth > 10);
            }
            _ => panic!("Expected StackOverflow error"),
        }
    }

    #[test]
    fn test_timeout_detection() {
        let limits = ExecutionLimits {
            max_time: Duration::from_millis(50),
            ..ExecutionLimits::default()
        };
        
        let guard = ExecutionGuard::with_limits(limits);
        
        // Simulate work
        thread::sleep(Duration::from_millis(100));
        
        // Next check should detect timeout
        let result = guard.check();
        assert!(result.is_err() || guard.is_bailed_out());
    }

    #[test]
    fn test_operation_limit() {
        let limits = ExecutionLimits {
            max_operations: 100,
            ..ExecutionLimits::default()
        };
        
        let guard = ExecutionGuard::with_limits(limits);
        
        for _ in 0..100 {
            guard.record_operation().unwrap();
        }
        
        // 101st operation should fail
        let result = guard.record_operation();
        assert!(result.is_err());
    }

    #[test]
    fn test_external_interrupt() {
        let guard = ExecutionGuard::new();
        
        guard.interrupt();
        
        let result = guard.check();
        assert!(result.is_err());
        
        match result {
            Err(BailoutReason::Interrupted) => (),
            _ => panic!("Expected Interrupted error"),
        }
    }

    #[test]
    fn test_memory_tracker() {
        let tracker = MemoryTracker::new(1000);
        
        assert!(tracker.allocate(500));
        assert_eq!(tracker.current(), 500);
        
        assert!(tracker.allocate(400));
        assert_eq!(tracker.current(), 900);
        
        // Should exceed limit
        assert!(!tracker.allocate(200));
        
        tracker.deallocate(300);
        assert_eq!(tracker.current(), 600);
    }

    #[test]
    fn test_execution_limits_presets() {
        // Development - lenient
        let dev = ExecutionLimits::development();
        assert!(dev.max_memory > ExecutionLimits::default().max_memory);
        
        // Production - stricter
        let prod = ExecutionLimits::production();
        assert!(prod.max_memory < ExecutionLimits::default().max_memory);
        
        // Testing - very strict
        let test = ExecutionLimits::testing();
        assert!(test.max_time < prod.max_time);
    }

    #[test]
    fn test_guard_reset() {
        let mut guard = ExecutionGuard::new();
        
        for _ in 0..50 {
            guard.record_operation().unwrap();
            guard.enter_call().unwrap();
        }
        
        guard.reset();
        
        assert_eq!(guard.stats().operation_count, 0);
        assert_eq!(guard.stats().call_depth, 0);
        assert!(!guard.is_bailed_out());
    }

    #[test]
    fn test_stats_display() {
        let guard = ExecutionGuard::new();
        
        for _ in 0..100 {
            guard.record_operation().unwrap();
        }
        
        let stats = guard.stats();
        let display = format!("{}", stats);
        
        assert!(display.contains("إحصائيات التنفيذ"));
        assert!(display.contains("عدد العمليات"));
    }
}
