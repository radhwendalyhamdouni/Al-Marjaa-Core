// src/stdlib/testing/mod.rs
// إطار الاختبارات الداخلي للغة المرجع
// Internal Testing Framework for Al-Marjaa Language

pub mod assertions;
pub mod runner;
pub mod reporter;

pub use assertions::*;
pub use runner::*;
pub use reporter::*;

/// معلومات إطار الاختبارات
pub fn testing_info() -> &'static str {
    "إطار الاختبارات للغة المرجع v3.4.0 - تأكد، تأكد_يساوي، تقارير"
}
