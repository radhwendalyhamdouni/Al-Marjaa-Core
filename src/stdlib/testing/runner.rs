// src/stdlib/testing/runner.rs
// مشغل الاختبارات للغة المرجع
// Test Runner for Al-Marjaa Language

use std::time::{Instant, Duration};
use std::any::Any;

/// نتيجة اختبار
#[derive(Debug, Clone)]
pub struct نتيجة_الاختبار {
    /// اسم الاختبار
    pub الاسم: String,
    /// هل نجح
    pub ناجح: bool,
    /// الرسالة
    pub الرسالة: String,
    /// الوقت المستغرق
    pub الوقت: Duration,
    /// تفاصيل الخطأ
    pub تفاصيل: Option<String>,
}

/// نتائج مجموعة اختبارات
#[derive(Debug, Clone)]
pub struct نتائج_المجموعة {
    /// الاختبارات
    pub الاختبارات: Vec<نتيجة_الاختبار>,
    /// عدد النجاح
    pub عدد_النجح: usize,
    /// عدد الفشل
    pub عدد_الفشل: usize,
    /// وقت البدء
    pub وقت_البداية: Instant,
    /// وقت الانتهاء
    pub وقت_الانتهاء: Option<Instant>,
}

impl نتائج_المجموعة {
    /// إنشاء مجموعة جديدة
    pub fn جديد() -> Self {
        نتائج_المجموعة {
            الاختبارات: Vec::new(),
            عدد_النجح: 0,
            عدد_الفشل: 0,
            وقت_البداية: Instant::now(),
            وقت_الانتهاء: None,
        }
    }

    /// إضافة نتيجة اختبار
    pub fn إضافة(&mut self, نتيجة: نتيجة_الاختبار) {
        if نتيجة.ناجح {
            self.عدد_النجح += 1;
        } else {
            self.عدد_الفشل += 1;
        }
        self.الاختبارات.push(نتيجة);
    }

    /// إنهاء المجموعة
    pub fn إنهاء(&mut self) {
        self.وقت_الانتهاء = Some(Instant::now());
    }

    /// نسبة النجاح
    pub fn نسبة_النجح(&self) -> f64 {
        if self.الاختبارات.is_empty() {
            return 0.0;
        }
        (self.عدد_النجح as f64) / (self.الاختبارات.len() as f64)
    }

    /// الوقت الإجمالي
    pub fn الوقت_الإجمالي(&self) -> Option<Duration> {
        self.وقت_الانتهاء.map(|end| end.duration_since(self.وقت_البداية))
    }
}

impl Default for نتائج_المجموعة {
    fn default() -> Self {
        Self::جديد()
    }
}

/// استخراج رسالة من خطأ
fn extract_message(err: &Box<dyn Any + Send>) -> String {
    if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else {
        "خطأ غير معروف".to_string()
    }
}

/// مشغل الاختبارات
pub struct مشغل_الاختبارات {
    /// النتائج
    نتائج: نتائج_المجموعة,
}

impl مشغل_الاختبارات {
    /// إنشاء مشغل جديد
    pub fn جديد() -> Self {
        مشغل_الاختبارات {
            نتائج: نتائج_المجموعة::جديد(),
        }
    }

    /// تشغيل اختبار
    pub fn تشغيل<F>(&mut self, الاسم: &str, الاختبار: F)
    where
        F: FnOnce() + std::panic::UnwindSafe,
    {
        let start = Instant::now();
        let نتيجة = std::panic::catch_unwind(الاختبار);

        let elapsed = start.elapsed();
        
        match نتيجة {
            Ok(()) => {
                self.نتائج.إضافة(نتيجة_الاختبار {
                    الاسم: الاسم.to_string(),
                    ناجح: true,
                    الرسالة: "نجح".to_string(),
                    الوقت: elapsed,
                    تفاصيل: None,
                });
            }
            Err(err) => {
                let msg = extract_message(&err);
                self.نتائج.إضافة(نتيجة_الاختبار {
                    الاسم: الاسم.to_string(),
                    ناجح: false,
                    الرسالة: "فشل".to_string(),
                    الوقت: elapsed,
                    تفاصيل: Some(msg),
                });
            }
        }
    }

    /// إنهاء التشغيل
    pub fn إنهاء(&mut self) {
        self.نتائج.إنهاء();
    }

    /// الحصول على النتائج
    pub fn النتائج(&self) -> &نتائج_المجموعة {
        &self.نتائج
    }
}

impl Default for مشغل_الاختبارات {
    fn default() -> Self {
        Self::جديد()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut runner = مشغل_الاختبارات::جديد();
        
        runner.تشغيل("اختبار_بسيط", || {
            assert!(true);
        });
        
        assert_eq!(runner.النتائج().عدد_النجح, 1);
    }
}
