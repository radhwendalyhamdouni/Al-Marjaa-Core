// src/stdlib/testing/assertions.rs
// دوال التحقق للاختبارات
// Assertion Functions for Testing

use std::fmt::Debug;

/// نتيجة التحقق
#[derive(Debug, Clone)]
pub struct نتيجة_التحقق {
    /// هل نجح التحقق
    pub ناجح: bool,
    /// الرسالة
    pub الرسالة: String,
    /// اسم الاختبار
    pub اسم_الاختبار: String,
    /// السطر
    pub السطر: Option<u32>,
    /// الملف
    pub الملف: Option<String>,
}

impl نتيجة_التحقق {
    /// إنشاء نتيجة ناجحة
    pub fn نجاح(الرسالة: &str) -> Self {
        نتيجة_التحقق {
            ناجح: true,
            الرسالة: الرسالة.to_string(),
            اسم_الاختبار: String::new(),
            السطر: None,
            الملف: None,
        }
    }

    /// إنشاء نتيجة فاشلة
    pub fn فشل(الرسالة: &str) -> Self {
        نتيجة_التحقق {
            ناجح: false,
            الرسالة: الرسالة.to_string(),
            اسم_الاختبار: String::new(),
            السطر: None,
            الملف: None,
        }
    }
}

/// تأكد من صحة شرط
/// 
/// # مثال
/// ```marjaa
/// تأكد(5 > 3، "خمسة أكبر من ثلاثة")
/// ```
#[macro_export]
macro_rules! تأكد {
    ($الشرط:expr) => {
        {
            if !($الشرط) {
                panic!("تأكد فشل: الشرط غير صحيح في {}:{}:{}", file!(), line!(), column!());
            }
        }
    };
    ($الشرط:expr, $الرسالة:expr) => {
        {
            if !($الشرط) {
                panic!("تأكد فشل: {} في {}:{}:{}", $الرسالة, file!(), line!(), column!());
            }
        }
    };
}

/// تأكد من تساوي قيمتين
/// 
/// # مثال
/// ```marjaa
/// تأكد_يساوي(5، 5، "القيم متساوية")
/// ```
#[macro_export]
macro_rules! تأكد_يساوي {
    ($القيمة1:expr, $القيمة2:expr) => {
        {
            let val1 = &$القيمة1;
            let val2 = &$القيمة2;
            if val1 != val2 {
                panic!(
                    "تأكد_يساوي فشل: القيم غير متساوية\n  المتوقع: {:?}\n  الفعلي: {:?}\n  في {}:{}:{}",
                    val2, val1, file!(), line!(), column!()
                );
            }
        }
    };
    ($القيمة1:expr, $القيمة2:expr, $الرسالة:expr) => {
        {
            let val1 = &$القيمة1;
            let val2 = &$القيمة2;
            if val1 != val2 {
                panic!(
                    "تأكد_يساوي فشل: {}\n  المتوقع: {:?}\n  الفعلي: {:?}\n  في {}:{}:{}",
                    $الرسالة, val2, val1, file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من عدم تساوي قيمتين
#[macro_export]
macro_rules! تأكد_لا_يساوي {
    ($القيمة1:expr, $القيمة2:expr) => {
        {
            let val1 = &$القيمة1;
            let val2 = &$القيمة2;
            if val1 == val2 {
                panic!(
                    "تأكد_لا_يساوي فشل: القيم متساوية\n  القيمة: {:?}\n  في {}:{}:{}",
                    val1, file!(), line!(), column!()
                );
            }
        }
    };
    ($القيمة1:expr, $القيمة2:expr, $الرسالة:expr) => {
        {
            let val1 = &$القيمة1;
            let val2 = &$القيمة2;
            if val1 == val2 {
                panic!(
                    "تأكد_لا_يساوي فشل: {}\n  القيمة: {:?}\n  في {}:{}:{}",
                    $الرسالة, val1, file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من أن القيمة صحيحة (true)
#[macro_export]
macro_rules! تأكد_صحيح {
    ($القيمة:expr) => {
        {
            let val = $القيمة;
            if !val {
                panic!(
                    "تأكد_صحيح فشل: القيمة خطأ\n  في {}:{}:{}",
                    file!(), line!(), column!()
                );
            }
        }
    };
    ($القيمة:expr, $الرسالة:expr) => {
        {
            let val = $القيمة;
            if !val {
                panic!(
                    "تأكد_صحيح فشل: {}\n  في {}:{}:{}",
                    $الرسالة, file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من أن القيمة خاطئة (false)
#[macro_export]
macro_rules! تأكد_خطأ {
    ($القيمة:expr) => {
        {
            let val = $القيمة;
            if val {
                panic!(
                    "تأكد_خطأ فشل: القيمة صحيحة\n  في {}:{}:{}",
                    file!(), line!(), column!()
                );
            }
        }
    };
    ($القيمة:expr, $الرسالة:expr) => {
        {
            let val = $القيمة;
            if val {
                panic!(
                    "تأكد_خطأ فشل: {}\n  في {}:{}:{}",
                    $الرسالة, file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من أن القيمة فارغة (None أو null)
#[macro_export]
macro_rules! تأكد_فارغ {
    ($القيمة:expr) => {
        {
            match $القيمة {
                Some(_) => panic!("تأكد_فارغ فشل: القيمة ليست فارغة في {}:{}", file!(), line!()),
                None => (),
            }
        }
    };
}

/// تأكد من أن القيمة ليست فارغة
#[macro_export]
macro_rules! تأكد_ليس_فارغ {
    ($القيمة:expr) => {
        {
            match $القيمة {
                Some(val) => val,
                None => panic!("تأكد_ليس_فارغ فشل: القيمة فارغة في {}:{}", file!(), line!()),
            }
        }
    };
}

/// تأكد من أن القيمة أكبر من
#[macro_export]
macro_rules! تأكد_أكبر {
    ($القيمة:expr, $الحد:expr) => {
        {
            let val = $القيمة;
            let limit = $الحد;
            if !(val > limit) {
                panic!(
                    "تأكد_أكبر فشل: {} ليس أكبر من {}\n  في {}:{}:{}",
                    val, limit, file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من أن القيمة أصغر من
#[macro_export]
macro_rules! تأكد_أصغر {
    ($القيمة:expr, $الحد:expr) => {
        {
            let val = $القيمة;
            let limit = $الحد;
            if !(val < limit) {
                panic!(
                    "تأكد_أصغر فشل: {} ليس أصغر من {}\n  في {}:{}:{}",
                    val, limit, file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من أن القيمة في نطاق
#[macro_export]
macro_rules! تأكد_في_النطاق {
    ($القيمة:expr, $البداية:expr, $النهاية:expr) => {
        {
            let val = $القيمة;
            let start = $البداية;
            let end = $النهاية;
            if !(val >= start && val <= end) {
                panic!(
                    "تأكد_في_النطاق فشل: {} ليس في النطاق [{}, {}]\n  في {}:{}:{}",
                    val, start, end, file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من أن النص يحتوي على نص آخر
#[macro_export]
macro_rules! تأكد_يحتوي {
    ($النص:expr, $المطلوب:expr) => {
        {
            let text = $النص;
            let pattern = $المطلوب;
            if !text.contains(pattern) {
                panic!(
                    "تأكد_يحتوي فشل: النص '{}' لا يحتوي على '{}'\n  في {}:{}:{}",
                    text, pattern, file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من أن النص يبدأ بنص معين
/// ملاحظة: حرف التطويل (ـ) مستخدم لربط الكلمات في العربية
#[macro_export]
macro_rules! تأكد_يبدأ_بـ {
    ($النص:expr, $البداية:expr) => {
        {
            let text = $النص;
            let prefix = $البداية;
            if !text.starts_with(prefix) {
                panic!(
                    "تأكد_يبدأ_بـ فشل: '{}' لا يبدأ بـ '{}'\n  في {}:{}:{}",
                    text, prefix, file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من أن النص ينتهي بنص معين
/// ملاحظة: حرف التطويل (ـ) مستخدم لربط الكلمات في العربية
#[macro_export]
macro_rules! تأكد_ينتهي_بـ {
    ($النص:expr, $النهاية:expr) => {
        {
            let text = $النص;
            let suffix = $النهاية;
            if !text.ends_with(suffix) {
                panic!(
                    "تأكد_ينتهي_بـ فشل: '{}' لا ينتهي بـ '{}'\n  في {}:{}:{}",
                    text, suffix, file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من أن القائمة فارغة
#[macro_export]
macro_rules! تأكد_القائمة_فارغة {
    ($القائمة:expr) => {
        {
            let list = &$القائمة;
            if !list.is_empty() {
                panic!(
                    "تأكد_القائمة_فارغة فشل: القائمة تحتوي على {} عنصر\n  في {}:{}:{}",
                    list.len(), file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من أن القائمة ليست فارغة
#[macro_export]
macro_rules! تأكد_القائمة_ليست_فارغة {
    ($القائمة:expr) => {
        {
            let list = &$القائمة;
            if list.is_empty() {
                panic!(
                    "تأكد_القائمة_ليست_فارغة فشل: القائمة فارغة\n  في {}:{}:{}",
                    file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من أن القائمة تحتوي على عدد معين من العناصر
#[macro_export]
macro_rules! تأكد_عدد_العناصر {
    ($القائمة:expr, $العدد:expr) => {
        {
            let list = &$القائمة;
            let expected = $العدد;
            if list.len() != expected {
                panic!(
                    "تأكد_عدد_العناصر فشل: المتوقع {} عنصر، الفعلي {}\n  في {}:{}:{}",
                    expected, list.len(), file!(), line!(), column!()
                );
            }
        }
    };
}

/// تأكد من حدوث خطأ (panic)
#[macro_export]
macro_rules! تأكد_حدوث_خطأ {
    ($التعبير:expr) => {
        {
            let result = std::panic::catch_unwind(|| $التعبير);
            match result {
                Ok(_) => panic!("تأكد_حدوث_خطأ فشل: لم يحدث خطأ في {}:{}", file!(), line!()),
                Err(_) => (),
            }
        }
    };
}

/// تأكد من عدم حدوث خطأ
#[macro_export]
macro_rules! تأكد_عدم_حدوث_خطأ {
    ($التعبير:expr) => {
        {
            let result = std::panic::catch_unwind(|| $التعبير);
            match result {
                Ok(val) => val,
                Err(e) => panic!("تأكد_عدم_حدوث_خطأ فشل: حدث خطأ غير متوقع في {}:{}", file!(), line!()),
            }
        }
    };
}

/// تأكد من أن القيمة قريبة من قيمة أخرى (للأرقام العشرية)
#[macro_export]
macro_rules! تأكد_تقريبا_يساوي {
    ($القيمة1:expr, $القيمة2:expr, $التحمل:expr) => {
        {
            let val1 = $القيمة1 as f64;
            let val2 = $القيمة2 as f64;
            let tolerance = $التحمل as f64;
            if (val1 - val2).abs() > tolerance {
                panic!(
                    "تأكد_تقريبا_يساوي فشل: {} ليس تقريباً مساوياً لـ {} (التحمل: {})\n  في {}:{}:{}",
                    val1, val2, tolerance, file!(), line!(), column!()
                );
            }
        }
    };
}

/// دوال التحقق كدوال عادية (للاستخدام بدون ماكرو)

/// تأكد من صحة شرط
pub fn تأكد(الشرط: bool, الرسالة: &str) -> Result<(), String> {
    if !الشرط {
        Err(format!("تأكد فشل: {}", الرسالة))
    } else {
        Ok(())
    }
}

/// تأكد من تساوي قيمتين
pub fn تأكد_يساوي<T: PartialEq + Debug>(القيمة1: T, القيمة2: T, الرسالة: &str) -> Result<(), String> {
    if القيمة1 != القيمة2 {
        Err(format!("تأكد_يساوي فشل: {}\n  المتوقع: {:?}\n  الفعلي: {:?}", الرسالة, القيمة2, القيمة1))
    } else {
        Ok(())
    }
}

/// تأكد من عدم تساوي قيمتين
pub fn تأكد_لا_يساوي<T: PartialEq + Debug>(القيمة1: T, القيمة2: T, الرسالة: &str) -> Result<(), String> {
    if القيمة1 == القيمة2 {
        Err(format!("تأكد_لا_يساوي فشل: {}\n  القيمة: {:?}", الرسالة, القيمة1))
    } else {
        Ok(())
    }
}

/// تأكد من أن القيمة أكبر من
pub fn تأكد_أكبر<T: PartialOrd + Debug>(القيمة: T, الحد: T, الرسالة: &str) -> Result<(), String> {
    if القيمة <= الحد {
        Err(format!("تأكد_أكبر فشل: {}\n  القيمة: {:?}\n  الحد: {:?}", الرسالة, القيمة, الحد))
    } else {
        Ok(())
    }
}

/// تأكد من أن القيمة أصغر من
pub fn تأكد_أصغر<T: PartialOrd + Debug>(القيمة: T, الحد: T, الرسالة: &str) -> Result<(), String> {
    if القيمة >= الحد {
        Err(format!("تأكد_أصغر فشل: {}\n  القيمة: {:?}\n  الحد: {:?}", الرسالة, القيمة, الحد))
    } else {
        Ok(())
    }
}

/// تأكد من أن النص يحتوي على نص آخر
pub fn تأكد_يحتوي(النص: &str, المطلوب: &str, الرسالة: &str) -> Result<(), String> {
    if !النص.contains(المطلوب) {
        Err(format!("تأكد_يحتوي فشل: {}\n  النص: '{}'\n  المطلوب: '{}'", الرسالة, النص, المطلوب))
    } else {
        Ok(())
    }
}

/// تأكد من أن القائمة فارغة
pub fn تأكد_القائمة_فارغة<T>(القائمة: &[T], الرسالة: &str) -> Result<(), String> {
    if !القائمة.is_empty() {
        Err(format!("تأكد_القائمة_فارغة فشل: {}\n  عدد العناصر: {}", الرسالة, القائمة.len()))
    } else {
        Ok(())
    }
}

/// تأكد من أن القائمة ليست فارغة
pub fn تأكد_القائمة_ليست_فارغة<T>(القائمة: &[T], الرسالة: &str) -> Result<(), String> {
    if القائمة.is_empty() {
        Err(format!("تأكد_القائمة_ليست_فارغة فشل: {}", الرسالة))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_تأكد() {
        assert!(تأكد(true, "الشرط صحيح").is_ok());
        assert!(تأكد(false, "الشرط خاطئ").is_err());
    }

    #[test]
    fn test_تأكد_يساوي() {
        assert!(تأكد_يساوي(5, 5, "القيم متساوية").is_ok());
        assert!(تأكد_يساوي(5, 3, "القيم غير متساوية").is_err());
    }

    #[test]
    fn test_تأكد_يحتوي() {
        assert!(تأكد_يحتوي("مرحباً بالعالم", "مرحباً", "النص موجود").is_ok());
        assert!(تأكد_يحتوي("مرحباً", "وداعاً", "النص غير موجود").is_err());
    }

    #[test]
    fn test_تأكد_أكبر() {
        assert!(تأكد_أكبر(10, 5, "القيمة أكبر").is_ok());
        assert!(تأكد_أكبر(3, 5, "القيمة ليست أكبر").is_err());
    }
}
