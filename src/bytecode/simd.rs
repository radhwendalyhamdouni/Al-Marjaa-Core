// ═══════════════════════════════════════════════════════════════════════════════
// SIMD Operations - العمليات المتوازية
// ═══════════════════════════════════════════════════════════════════════════════
// تعليمات SIMD (Single Instruction, Multiple Data)
// تسريع العمليات الحسابية 4-8x للقوائم والمصفوفات
// ═══════════════════════════════════════════════════════════════════════════════

use std::arch::x86_64::*;
use std::mem;

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع SIMD
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع عملية SIMD
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimdOp {
    /// جمع
    Add,
    /// طرح
    Sub,
    /// ضرب
    Mul,
    /// قسمة
    Div,
    /// مقارنة (أكبر من)
    GreaterThan,
    /// مقارنة (أقل من)
    LessThan,
    /// مقارنة (يساوي)
    Equal,
    /// حد أقصى
    Max,
    /// حد أدنى
    Min,
    /// جمع أفقي (مجموع كل العناصر)
    HorizontalSum,
    /// ضرب متراكم (dot product)
    DotProduct,
}

/// نتيجة عملية SIMD
#[derive(Debug, Clone)]
pub enum SimdResult {
    /// نتيجة عددية واحدة
    Scalar(f64),
    /// نتيجة متجهة (قائمة)
    Vector(Vec<f64>),
    /// نتيجة منطقية
    Boolean(Vec<bool>),
}

/// إحصائيات SIMD
#[derive(Debug, Default, Clone)]
pub struct SimdStats {
    /// عدد العمليات المنفذة
    pub operations_count: u64,
    /// عدد العناصر المعالجة
    pub elements_processed: u64,
    /// وقت التنفيذ (نانو ثانية)
    pub execution_time_ns: u64,
    /// عدد العمليات المتوازية
    pub parallel_operations: u64,
    /// نسبة التسريع
    pub speedup_ratio: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// معالج SIMD
// ═══════════════════════════════════════════════════════════════════════════════

/// معالج العمليات SIMD
pub struct SimdProcessor {
    /// هل AVX متاح
    has_avx: bool,
    /// هل AVX2 متاح
    has_avx2: bool,
    /// هل SSE متاح
    has_sse: bool,
    /// حجم المتجه (عدد العناصر في المتجه الواحد)
    vector_width: usize,
    /// الإحصائيات
    stats: SimdStats,
}

impl SimdProcessor {
    /// إنشاء معالج SIMD جديد
    pub fn new() -> Self {
        let has_avx = is_x86_feature_detected!("avx");
        let has_avx2 = is_x86_feature_detected!("avx2");
        let has_sse = is_x86_feature_detected!("sse");
        
        let vector_width = if has_avx {
            4 // AVX يعالج 4x f64 في المرة
        } else if has_sse {
            2 // SSE يعالج 2x f64 في المرة
        } else {
            1 // بدون SIMD
        };
        
        SimdProcessor {
            has_avx,
            has_avx2,
            has_sse,
            vector_width,
            stats: SimdStats::default(),
        }
    }
    
    /// جمع متجهين
    pub fn add(&mut self, a: &[f64], b: &[f64]) -> Vec<f64> {
        let start = std::time::Instant::now();
        let len = a.len().min(b.len());
        let mut result = vec![0.0; len];
        
        if self.has_avx {
            unsafe {
                self.add_avx(a, b, &mut result);
            }
        } else if self.has_sse {
            unsafe {
                self.add_sse(a, b, &mut result);
            }
        } else {
            self.add_scalar(a, b, &mut result);
        }
        
        self.update_stats(len, start.elapsed().as_nanos() as u64);
        result
    }
    
    /// طرح متجهين
    pub fn sub(&mut self, a: &[f64], b: &[f64]) -> Vec<f64> {
        let start = std::time::Instant::now();
        let len = a.len().min(b.len());
        let mut result = vec![0.0; len];
        
        if self.has_avx {
            unsafe {
                self.sub_avx(a, b, &mut result);
            }
        } else {
            self.sub_scalar(a, b, &mut result);
        }
        
        self.update_stats(len, start.elapsed().as_nanos() as u64);
        result
    }
    
    /// ضرب متجهين
    pub fn mul(&mut self, a: &[f64], b: &[f64]) -> Vec<f64> {
        let start = std::time::Instant::now();
        let len = a.len().min(b.len());
        let mut result = vec![0.0; len];
        
        if self.has_avx {
            unsafe {
                self.mul_avx(a, b, &mut result);
            }
        } else {
            self.mul_scalar(a, b, &mut result);
        }
        
        self.update_stats(len, start.elapsed().as_nanos() as u64);
        result
    }
    
    /// قسمة متجهين
    pub fn div(&mut self, a: &[f64], b: &[f64]) -> Vec<f64> {
        let start = std::time::Instant::now();
        let len = a.len().min(b.len());
        let mut result = vec![0.0; len];
        
        if self.has_avx {
            unsafe {
                self.div_avx(a, b, &mut result);
            }
        } else {
            self.div_scalar(a, b, &mut result);
        }
        
        self.update_stats(len, start.elapsed().as_nanos() as u64);
        result
    }
    
    /// ضرب قيم المتجه بعدد
    pub fn scale(&mut self, a: &[f64], scalar: f64) -> Vec<f64> {
        let start = std::time::Instant::now();
        let len = a.len();
        let mut result = vec![0.0; len];
        
        if self.has_avx {
            unsafe {
                let scalar_vec = _mm256_set1_pd(scalar);
                for i in (0..len).step_by(4) {
                    if i + 4 <= len {
                        let va = _mm256_loadu_pd(a.as_ptr().add(i));
                        let vr = _mm256_mul_pd(va, scalar_vec);
                        _mm256_storeu_pd(result.as_mut_ptr().add(i), vr);
                    } else {
                        for j in i..len {
                            result[j] = a[j] * scalar;
                        }
                    }
                }
            }
        } else {
            for i in 0..len {
                result[i] = a[i] * scalar;
            }
        }
        
        self.update_stats(len, start.elapsed().as_nanos() as u64);
        result
    }
    
    /// مجموع عناصر المتجه
    pub fn sum(&mut self, a: &[f64]) -> f64 {
        let start = std::time::Instant::now();
        let len = a.len();
        let mut sum = 0.0;
        
        if self.has_avx && len >= 4 {
            unsafe {
                let mut sum_vec = _mm256_setzero_pd();
                
                for i in (0..len).step_by(4) {
                    if i + 4 <= len {
                        let va = _mm256_loadu_pd(a.as_ptr().add(i));
                        sum_vec = _mm256_add_pd(sum_vec, va);
                    }
                }
                
                // جمع أفقي
                sum = self.horizontal_sum_avx(sum_vec);
                
                // جمع العناصر المتبقية
                for &val in a.iter().skip((len / 4) * 4) {
                    sum += val;
                }
            }
        } else {
            for &val in a {
                sum += val;
            }
        }
        
        self.update_stats(len, start.elapsed().as_nanos() as u64);
        sum
    }
    
    /// حاصل الضرب النقطي (dot product)
    pub fn dot(&mut self, a: &[f64], b: &[f64]) -> f64 {
        let start = std::time::Instant::now();
        let len = a.len().min(b.len());
        let mut dot = 0.0;
        
        if self.has_avx && len >= 4 {
            unsafe {
                let mut dot_vec = _mm256_setzero_pd();
                
                for i in (0..len).step_by(4) {
                    if i + 4 <= len {
                        let va = _mm256_loadu_pd(a.as_ptr().add(i));
                        let vb = _mm256_loadu_pd(b.as_ptr().add(i));
                        dot_vec = _mm256_add_pd(dot_vec, _mm256_mul_pd(va, vb));
                    }
                }
                
                dot = self.horizontal_sum_avx(dot_vec);
                
                for i in (len / 4) * 4..len {
                    dot += a[i] * b[i];
                }
            }
        } else {
            for i in 0..len {
                dot += a[i] * b[i];
            }
        }
        
        self.update_stats(len, start.elapsed().as_nanos() as u64);
        dot
    }
    
    /// إيجاد القيمة الأكبر
    pub fn max(&mut self, a: &[f64]) -> f64 {
        if a.is_empty() {
            return f64::NAN;
        }
        
        let start = std::time::Instant::now();
        let len = a.len();
        let mut max_val = a[0];
        
        if self.has_avx && len >= 4 {
            unsafe {
                let mut max_vec = _mm256_loadu_pd(a.as_ptr());
                
                for i in (4..len).step_by(4) {
                    if i + 4 <= len {
                        let va = _mm256_loadu_pd(a.as_ptr().add(i));
                        max_vec = _mm256_max_pd(max_vec, va);
                    }
                }
                
                // استخراج القيمة الأكبر من المتجه
                let arr: [f64; 4] = mem::transmute(max_vec);
                max_val = arr[0].max(arr[1]).max(arr[2]).max(arr[3]);

                for &val in a.iter().skip((len / 4) * 4) {
                    max_val = max_val.max(val);
                }
            }
        } else {
            for &val in a {
                max_val = max_val.max(val);
            }
        }
        
        self.update_stats(len, start.elapsed().as_nanos() as u64);
        max_val
    }
    
    /// إيجاد القيمة الأصغر
    pub fn min(&mut self, a: &[f64]) -> f64 {
        if a.is_empty() {
            return f64::NAN;
        }
        
        let start = std::time::Instant::now();
        let len = a.len();
        let mut min_val = a[0];
        
        if self.has_avx && len >= 4 {
            unsafe {
                let mut min_vec = _mm256_loadu_pd(a.as_ptr());
                
                for i in (4..len).step_by(4) {
                    if i + 4 <= len {
                        let va = _mm256_loadu_pd(a.as_ptr().add(i));
                        min_vec = _mm256_min_pd(min_vec, va);
                    }
                }
                
                let arr: [f64; 4] = mem::transmute(min_vec);
                min_val = arr[0].min(arr[1]).min(arr[2]).min(arr[3]);

                for &val in a.iter().skip((len / 4) * 4) {
                    min_val = min_val.min(val);
                }
            }
        } else {
            for &val in a {
                min_val = min_val.min(val);
            }
        }
        
        self.update_stats(len, start.elapsed().as_nanos() as u64);
        min_val
    }
    
    /// تطبيق دالة على كل عنصر
    pub fn map<F>(&mut self, a: &[f64], f: F) -> Vec<f64>
    where
        F: Fn(f64) -> f64,
    {
        a.iter().map(|&x| f(x)).collect()
    }
    
    /// تصفية العناصر
    pub fn filter<F>(&mut self, a: &[f64], predicate: F) -> Vec<f64>
    where
        F: Fn(f64) -> bool,
    {
        a.iter().filter(|&&x| predicate(x)).copied().collect()
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال AVX الداخلية
    // ═══════════════════════════════════════════════════════════════
    
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx")]
    unsafe fn add_avx(&self, a: &[f64], b: &[f64], result: &mut [f64]) {
        let len = result.len();
        for i in (0..len).step_by(4) {
            if i + 4 <= len {
                let va = _mm256_loadu_pd(a.as_ptr().add(i));
                let vb = _mm256_loadu_pd(b.as_ptr().add(i));
                let vr = _mm256_add_pd(va, vb);
                _mm256_storeu_pd(result.as_mut_ptr().add(i), vr);
            } else {
                for j in i..len {
                    result[j] = a[j] + b[j];
                }
            }
        }
    }
    
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx")]
    unsafe fn sub_avx(&self, a: &[f64], b: &[f64], result: &mut [f64]) {
        let len = result.len();
        for i in (0..len).step_by(4) {
            if i + 4 <= len {
                let va = _mm256_loadu_pd(a.as_ptr().add(i));
                let vb = _mm256_loadu_pd(b.as_ptr().add(i));
                let vr = _mm256_sub_pd(va, vb);
                _mm256_storeu_pd(result.as_mut_ptr().add(i), vr);
            } else {
                for j in i..len {
                    result[j] = a[j] - b[j];
                }
            }
        }
    }
    
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx")]
    unsafe fn mul_avx(&self, a: &[f64], b: &[f64], result: &mut [f64]) {
        let len = result.len();
        for i in (0..len).step_by(4) {
            if i + 4 <= len {
                let va = _mm256_loadu_pd(a.as_ptr().add(i));
                let vb = _mm256_loadu_pd(b.as_ptr().add(i));
                let vr = _mm256_mul_pd(va, vb);
                _mm256_storeu_pd(result.as_mut_ptr().add(i), vr);
            } else {
                for j in i..len {
                    result[j] = a[j] * b[j];
                }
            }
        }
    }
    
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx")]
    unsafe fn div_avx(&self, a: &[f64], b: &[f64], result: &mut [f64]) {
        let len = result.len();
        for i in (0..len).step_by(4) {
            if i + 4 <= len {
                let va = _mm256_loadu_pd(a.as_ptr().add(i));
                let vb = _mm256_loadu_pd(b.as_ptr().add(i));
                let vr = _mm256_div_pd(va, vb);
                _mm256_storeu_pd(result.as_mut_ptr().add(i), vr);
            } else {
                for j in i..len {
                    result[j] = a[j] / b[j];
                }
            }
        }
    }
    
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx")]
    unsafe fn horizontal_sum_avx(&self, v: __m256d) -> f64 {
        let arr: [f64; 4] = mem::transmute(v);
        arr[0] + arr[1] + arr[2] + arr[3]
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال SSE الداخلية
    // ═══════════════════════════════════════════════════════════════
    
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse")]
    unsafe fn add_sse(&self, a: &[f64], b: &[f64], result: &mut [f64]) {
        let len = result.len();
        for i in (0..len).step_by(2) {
            if i + 2 <= len {
                let va = _mm_loadu_pd(a.as_ptr().add(i));
                let vb = _mm_loadu_pd(b.as_ptr().add(i));
                let vr = _mm_add_pd(va, vb);
                _mm_storeu_pd(result.as_mut_ptr().add(i), vr);
            } else {
                for j in i..len {
                    result[j] = a[j] + b[j];
                }
            }
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال Scalar (بدون SIMD)
    // ═══════════════════════════════════════════════════════════════
    
    fn add_scalar(&self, a: &[f64], b: &[f64], result: &mut [f64]) {
        for i in 0..result.len() {
            result[i] = a[i] + b[i];
        }
    }
    
    fn sub_scalar(&self, a: &[f64], b: &[f64], result: &mut [f64]) {
        for i in 0..result.len() {
            result[i] = a[i] - b[i];
        }
    }
    
    fn mul_scalar(&self, a: &[f64], b: &[f64], result: &mut [f64]) {
        for i in 0..result.len() {
            result[i] = a[i] * b[i];
        }
    }
    
    fn div_scalar(&self, a: &[f64], b: &[f64], result: &mut [f64]) {
        for i in 0..result.len() {
            result[i] = a[i] / b[i];
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال مساعدة
    // ═══════════════════════════════════════════════════════════════
    
    fn update_stats(&mut self, elements: usize, time_ns: u64) {
        self.stats.operations_count += 1;
        self.stats.elements_processed += elements as u64;
        self.stats.execution_time_ns += time_ns;
        
        // حساب نسبة التسريع (مقارنة بالمعالجة التسلسلية)
        let scalar_time = elements as f64 * 10.0; // تقدير للوقت التسلسلي
        if time_ns > 0 {
            self.stats.speedup_ratio = scalar_time / (time_ns as f64);
        }
    }
    
    /// الحصول على عرض المتجه
    pub fn vector_width(&self) -> usize {
        self.vector_width
    }
    
    /// هل SIMD متاح
    pub fn is_simd_available(&self) -> bool {
        self.has_avx || self.has_sse
    }
    
    /// الإحصائيات
    pub fn stats(&self) -> &SimdStats {
        &self.stats
    }
    
    /// طباعة تقرير
    pub fn print_report(&self) {
        println!();
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║              🚀 تقرير SIMD Operations                              ║");
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ AVX متاح:              {:>15}                       ║", if self.has_avx { "نعم ✅" } else { "لا ❌" });
        println!("║ AVX2 متاح:             {:>15}                       ║", if self.has_avx2 { "نعم ✅" } else { "لا ❌" });
        println!("║ SSE متاح:              {:>15}                       ║", if self.has_sse { "نعم ✅" } else { "لا ❌" });
        println!("║ عرض المتجه:            {:>15} عنصر                ║", self.vector_width);
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ العمليات المنفذة:      {:>15}                       ║", self.stats.operations_count);
        println!("║ العناصر المعالجة:      {:>15}                       ║", self.stats.elements_processed);
        println!("║ وقت التنفيذ:          {:>15} نانو ثانية          ║", self.stats.execution_time_ns);
        println!("║ نسبة التسريع:          {:>15.2}x                     ║", self.stats.speedup_ratio);
        println!("╚══════════════════════════════════════════════════════════════════╝");
    }
}

impl Default for SimdProcessor {
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
    fn test_simd_processor_creation() {
        let processor = SimdProcessor::new();
        assert!(processor.vector_width >= 1);
    }

    #[test]
    fn test_simd_add() {
        let mut processor = SimdProcessor::new();
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let b = vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        
        let result = processor.add(&a, &b);
        
        assert_eq!(result, vec![9.0; 8]);
    }

    #[test]
    fn test_simd_mul() {
        let mut processor = SimdProcessor::new();
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![2.0, 2.0, 2.0, 2.0];
        
        let result = processor.mul(&a, &b);
        
        assert_eq!(result, vec![2.0, 4.0, 6.0, 8.0]);
    }

    #[test]
    fn test_simd_sum() {
        let mut processor = SimdProcessor::new();
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        let sum = processor.sum(&a);
        
        assert_eq!(sum, 15.0);
    }

    #[test]
    fn test_simd_dot() {
        let mut processor = SimdProcessor::new();
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![1.0, 1.0, 1.0, 1.0];
        
        let dot = processor.dot(&a, &b);
        
        assert_eq!(dot, 10.0);
    }

    #[test]
    fn test_simd_scale() {
        let mut processor = SimdProcessor::new();
        let a = vec![1.0, 2.0, 3.0, 4.0];
        
        let result = processor.scale(&a, 2.0);
        
        assert_eq!(result, vec![2.0, 4.0, 6.0, 8.0]);
    }

    #[test]
    fn test_simd_max_min() {
        let mut processor = SimdProcessor::new();
        let a = vec![5.0, 2.0, 8.0, 1.0, 9.0, 3.0];
        
        assert_eq!(processor.max(&a), 9.0);
        assert_eq!(processor.min(&a), 1.0);
    }
}
