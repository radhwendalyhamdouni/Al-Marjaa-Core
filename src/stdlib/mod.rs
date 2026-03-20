// src/stdlib/mod.rs
// المكتبة القياسية الأساسية للغة المرجع
// Core Standard Library for Al-Marjaa Language

pub mod crypto;
pub mod regex;

pub use crypto::*;
pub use regex::*;

/// معلومات المكتبة القياسية
pub fn stdlib_info() -> &'static str {
    "المكتبة القياسية الأساسية للغة المرجع v3.4.0 - Regex, Crypto"
}
