// src/stdlib/database/mod.rs
// دعم قواعد البيانات للغة المرجع
// Database Support for Al-Marjaa Language

pub mod sqlite;
pub mod errors;

pub use sqlite::*;
pub use errors::*;

/// معلومات وحدة قواعد البيانات
pub fn database_info() -> &'static str {
    "وحدة قواعد البيانات للغة المرجع v3.4.0 - SQLite"
}
