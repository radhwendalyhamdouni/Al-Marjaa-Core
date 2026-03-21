// src/stdlib/mod.rs
// المكتبة القياسية الأساسية للغة المرجع
// Core Standard Library for Al-Marjaa Language

//! # المكتبة القياسية - Standard Library
//! 
//! توفر هذه الوحدة المكتبة القياسية للغة المرجع.
//! بعض المكونات تتطلب feature flags.
//! 
//! ## المكونات
//! 
//! | المكون | Feature Flag | الوصف |
//! |--------|--------------|-------|
//! | `regex` | دائماً متاح | التعابير النمطية |
//! | `testing` | دائماً متاح | أدوات الاختبار |
//! | `crypto` | `crypto` | دوال التشفير |
//! | `database` | `database` | قواعد البيانات |

// المكونات الأساسية - متاحة دائماً
pub mod regex;
pub mod testing;

// المكونات الاختيارية - تتطلب feature flags
#[cfg(feature = "crypto")]
pub mod crypto;

#[cfg(feature = "database")]
pub mod database;

// إعادة التصدير
pub use regex::*;
pub use testing::*;

#[cfg(feature = "crypto")]
pub use crypto::*;

#[cfg(feature = "database")]
pub use database::*;

/// معلومات المكتبة القياسية
pub fn stdlib_info() -> &'static str {
    #[cfg(all(feature = "crypto", feature = "database"))]
    {
        "المكتبة القياسية الكاملة - Regex, Crypto, Database, Testing"
    }
    
    #[cfg(all(feature = "crypto", not(feature = "database")))]
    {
        "المكتبة القياسية - Regex, Crypto, Testing"
    }
    
    #[cfg(all(not(feature = "crypto"), feature = "database"))]
    {
        "المكتبة القياسية - Regex, Database, Testing"
    }
    
    #[cfg(all(not(feature = "crypto"), not(feature = "database")))]
    {
        "المكتبة القياسية الأساسية - Regex, Testing"
    }
}
