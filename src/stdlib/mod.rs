// src/stdlib/mod.rs
// المكتبة القياسية الأساسية للغة المرجع
// Core Standard Library for Al-Marjaa Language

pub mod crypto;
pub mod database;
pub mod regex;
pub mod testing;

pub use crypto::*;
pub use database::*;
pub use regex::*;
pub use testing::*;

/// معلومات المكتبة القياسية
pub fn stdlib_info() -> &'static str {
    "المكتبة القياسية الأساسية للغة المرجع v3.4.0 - Regex, Crypto, Database (SQLite), Testing"
}
