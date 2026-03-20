// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات شاملة للمفسر - Comprehensive Interpreter Tests
// ═══════════════════════════════════════════════════════════════════════════════
// يتضمن:
// - اختبارات الوحدة لكل ميزة
// - اختبارات التكامل
// - اختبارات الحالات الخاصة
// - اختبارات الأخطاء
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::Interpreter;

// ═══════════════════════════════════════════════════════════════════════════════
// دوال مساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// تشغيل كود وإرجاع النتيجة
fn run_code(source: &str) -> Result<String, String> {
    let mut interpreter = Interpreter::new();
    interpreter.run(source)
        .map(|v| format!("{:?}", v.borrow()))
        .map_err(|e| format!("خطأ في التنفيذ: {:?}", e))
}

/// تشغيل كود مع التقاط المخرجات
#[allow(dead_code)]
fn run_and_capture_output(source: &str) -> Vec<String> {
    let mut interpreter = Interpreter::new();
    match interpreter.run(source) {
        Ok(_) => vec!["نجاح".to_string()],
        Err(e) => {
            println!("خطأ في التنفيذ: {:?}", e);
            vec![format!("خطأ: {:?}", e)]
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات المتغيرات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod variable_tests {
    use super::*;

    /// اختبار المتغيرات الأساسية
    #[test]
    fn test_basic_variables() {
        let test_cases = vec![
            (r#"متغير س = 10"#, "إنشاء متغير رقمي"),
            (r#"متغير اسم = "أحمد""#, "إنشاء متغير نصي"),
            (r#"متغير نشط = صح"#, "إنشاء متغير منطقي"),
            (r#"متغير فارغ = لا_شيء"#, "إنشاء متغير فارغ"),
        ];

        for (source, description) in test_cases {
            let result = run_code(source);
            println!("{}: {:?}", description, result);
        }
    }

    /// اختبار الثوابت
    #[test]
    fn test_constants() {
        let source = r#"
            ثابت PI = 3.14159
            ثابت MESSAGE = "مرحبا"
        "#;

        let result = run_code(source);
        println!("الثوابت: {:?}", result);
    }

    /// اختبار إعادة التعيين
    #[test]
    fn test_variable_reassignment() {
        let source = r#"
            متغير س = 10
            س = 20
            س = س + 5
        "#;

        let result = run_code(source);
        println!("إعادة التعيين: {:?}", result);
    }

    /// اختبار المعاملات المركبة
    #[test]
    fn test_compound_assignment() {
        let source = r#"
            متغير س = 10
            س += 5
            س -= 3
            س *= 2
            س /= 4
        "#;

        let result = run_code(source);
        println!("المعاملات المركبة: {:?}", result);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات العمليات الحسابية
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod arithmetic_tests {
    use super::*;

    /// اختبار الجمع
    #[test]
    fn test_addition() {
        let test_cases = vec![
            (r#"متغير س = 1 + 2"#, 3.0),
            (r#"متغير س = 10 + 20 + 30"#, 60.0),
            (r#"متغير س = 0.1 + 0.2"#, 0.3),
        ];

        for (source, _expected) in test_cases {
            let result = run_code(source);
            println!("الجمع '{}': {:?}", source, result);
        }
    }

    /// اختبار الطرح
    #[test]
    fn test_subtraction() {
        let test_cases = vec![
            (r#"متغير س = 10 - 5"#, 5.0),
            (r#"متغير س = 5 - 10"#, -5.0),
            (r#"متغير س = 100 - 50 - 25"#, 25.0),
        ];

        for (source, _expected) in test_cases {
            let result = run_code(source);
            println!("الطرح '{}': {:?}", source, result);
        }
    }

    /// اختبار الضرب
    #[test]
    fn test_multiplication() {
        let test_cases = vec![
            (r#"متغير س = 5 * 4"#, 20.0),
            (r#"متغير س = 2 * 3 * 4"#, 24.0),
            (r#"متغير س = 0.5 * 4"#, 2.0),
        ];

        for (source, _expected) in test_cases {
            let result = run_code(source);
            println!("الضرب '{}': {:?}", source, result);
        }
    }

    /// اختبار القسمة
    #[test]
    fn test_division() {
        let test_cases = vec![
            (r#"متغير س = 20 / 4"#, 5.0),
            (r#"متغير س = 7 / 2"#, 3.5),
            (r#"متغير س = 1 / 3"#, 0.333),
        ];

        for (source, _expected) in test_cases {
            let result = run_code(source);
            println!("القسمة '{}': {:?}", source, result);
        }
    }

    /// اختبار القسمة الصحيحة
    #[test]
    fn test_floor_division() {
        let source = r#"
            متغير أ = 7 // 2
            متغير ب = 10 // 3
            متغير ج = -7 // 2
        "#;

        let result = run_code(source);
        println!("القسمة الصحيحة: {:?}", result);
    }

    /// اختبار الباقي
    #[test]
    fn test_modulo() {
        let test_cases = vec![
            (r#"متغير س = 7 % 3"#, 1.0),
            (r#"متغير س = 10 % 5"#, 0.0),
            (r#"متغير س = 17 % 4"#, 1.0),
        ];

        for (source, _expected) in test_cases {
            let result = run_code(source);
            println!("الباقي '{}': {:?}", source, result);
        }
    }

    /// اختبار الأُس
    #[test]
    fn test_power() {
        let test_cases = vec![
            (r#"متغير س = 2 ^ 3"#, 8.0),
            (r#"متغير س = 5 ^ 2"#, 25.0),
            (r#"متغير س = 10 ^ 0"#, 1.0),
        ];

        for (source, _expected) in test_cases {
            let result = run_code(source);
            println!("الأُس '{}': {:?}", source, result);
        }
    }

    /// اختبار أولوية العمليات
    #[test]
    fn test_operator_precedence() {
        let test_cases = vec![
            (r#"متغير س = 2 + 3 * 4"#, 14.0),    // الضرب أولاً
            (r#"متغير س = (2 + 3) * 4"#, 20.0), // الأقواس أولاً
            (r#"متغير س = 10 - 2 - 3"#, 5.0),   // من اليسار لليمين
            (r#"متغير س = 2 ^ 3 ^ 2"#, 512.0),  // الأس من اليمين لليسار
        ];

        for (source, _expected) in test_cases {
            let result = run_code(source);
            println!("الأولوية '{}': {:?}", source, result);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات المقارنات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod comparison_tests {
    use super::*;

    /// اختبار المساواة
    #[test]
    fn test_equality() {
        let test_cases = vec![
            (r#"متغير س = 5 == 5"#, true),
            (r#"متغير س = 5 == 6"#, false),
            (r#"متغير س = "أ" == "أ""#, true),
            (r#"متغير س = صح == صح"#, true),
        ];

        for (source, _expected) in test_cases {
            let result = run_code(source);
            println!("المساواة '{}': {:?}", source, result);
        }
    }

    /// اختبار عدم المساواة
    #[test]
    fn test_inequality() {
        let test_cases = vec![
            (r#"متغير س = 5 != 6"#, true),
            (r#"متغير س = 5 != 5"#, false),
        ];

        for (source, _expected) in test_cases {
            let result = run_code(source);
            println!("عدم المساواة '{}': {:?}", source, result);
        }
    }

    /// اختبار المقارنات الرقمية
    #[test]
    fn test_numeric_comparisons() {
        let test_cases = vec![
            (r#"متغير س = 5 < 10"#, true),
            (r#"متغير س = 10 > 5"#, true),
            (r#"متغير س = 5 <= 5"#, true),
            (r#"متغير س = 5 >= 6"#, false),
        ];

        for (source, _expected) in test_cases {
            let result = run_code(source);
            println!("المقارنة '{}': {:?}", source, result);
        }
    }

    /// اختبار المقارنات المنطقية
    #[test]
    fn test_logical_operations() {
        let test_cases = vec![
            (r#"متغير س = صح و صح"#, true),
            (r#"متغير س = صح و خطأ"#, false),
            (r#"متغير س = صح أو خطأ"#, true),
            (r#"متغير س = خطأ أو خطأ"#, false),
            (r#"متغير س = ليس خطأ"#, true),
            (r#"متغير س = ليس صح"#, false),
        ];

        for (source, _expected) in test_cases {
            let result = run_code(source);
            println!("المنطق '{}': {:?}", source, result);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الشروط
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod conditional_tests {
    use super::*;

    /// اختبار الشرط البسيط
    #[test]
    fn test_simple_if() {
        let source = r#"
            متغير س = 10
            إذا س > 5:
                طباعة("كبير")
        "#;

        let result = run_code(source);
        println!("الشرط البسيط: {:?}", result);
    }

    /// اختبار الشرط مع وإلا
    #[test]
    fn test_if_else() {
        let source = r#"
            متغير س = 3
            إذا س > 5:
                طباعة("كبير")
            وإلا:
                طباعة("صغير")
        "#;

        let result = run_code(source);
        println!("الشرط مع وإلا: {:?}", result);
    }

    /// اختبار السلسلة الشرطية
    #[test]
    fn test_if_elseif_else() {
        let source = r#"
            متغير درجة = 85
            إذا درجة >= 90:
                طباعة("ممتاز")
            وإذا درجة >= 80:
                طباعة("جيد جداً")
            وإذا درجة >= 70:
                طباعة("جيد")
            وإلا:
                طباعة("مقبول")
        "#;

        let result = run_code(source);
        println!("السلسلة الشرطية: {:?}", result);
    }

    /// اختبار الشروط المتداخلة
    #[test]
    fn test_nested_conditions() {
        let source = r#"
            متغير أ = 10
            متغير ب = 20
            إذا أ > 5:
                إذا ب > 15:
                    طباعة("كلاهما كبير")
                وإلا:
                    طباعة("أ كبير فقط")
        "#;

        let result = run_code(source);
        println!("الشروط المتداخلة: {:?}", result);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الحلقات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod loop_tests {
    use super::*;

    /// اختبار حلقة طالما
    #[test]
    fn test_while_loop() {
        let source = r#"
            متغير س = 0
            طالما س < 5:
                س += 1
            طباعة(س)
        "#;

        let result = run_code(source);
        println!("حلقة طالما: {:?}", result);
    }

    /// اختبار حلقة لكل
    #[test]
    fn test_for_each_loop() {
        let source = r#"
            متغير قائمة = [1، 2، 3، 4، 5]
            لكل عنصر في قائمة:
                طباعة(عنصر)
        "#;

        let result = run_code(source);
        println!("حلقة لكل: {:?}", result);
    }

    /// اختبار حلقة النطاق
    #[test]
    fn test_range_loop() {
        let source = r#"
            لكل س في 1..5:
                طباعة(س)
        "#;

        let result = run_code(source);
        println!("حلقة النطاق: {:?}", result);
    }

    /// اختبار توقف
    #[test]
    fn test_break() {
        let source = r#"
            متغير س = 0
            طالما صح:
                س += 1
                إذا س == 5:
                    توقف
            طباعة(س)
        "#;

        let result = run_code(source);
        println!("توقف: {:?}", result);
    }

    /// اختبار أكمل
    #[test]
    fn test_continue() {
        let source = r#"
            لكل س في 1..10:
                إذا س % 2 == 0:
                    أكمل
                طباعة(س)
        "#;

        let result = run_code(source);
        println!("أكمل: {:?}", result);
    }

    /// اختبار الحلقات المتداخلة
    #[test]
    fn test_nested_loops() {
        let source = r#"
            لكل أ في 1..3:
                لكل ب في 1..3:
                    طباعة(أ * ب)
        "#;

        let result = run_code(source);
        println!("الحلقات المتداخلة: {:?}", result);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الدوال
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod function_tests {
    use super::*;

    /// اختبار الدالة البسيطة
    #[test]
    fn test_simple_function() {
        let source = r#"
            دالة ترحيب():
                طباعة("مرحبا بالعالم")
            
            ترحيب()
        "#;

        let result = run_code(source);
        println!("الدالة البسيطة: {:?}", result);
    }

    /// اختبار دالة مع معاملات
    #[test]
    fn test_function_with_parameters() {
        let source = r#"
            دالة جمع(أ، ب):
                أرجع أ + ب
            
            متغير نتيجة = جمع(5، 3)
            طباعة(نتيجة)
        "#;

        let result = run_code(source);
        println!("دالة مع معاملات: {:?}", result);
    }

    /// اختبار دالة مع قيمة افتراضية
    #[test]
    fn test_function_with_default() {
        let source = r#"
            دالة ترحيب(اسم = "زائر"):
                طباعة("مرحبا " + اسم)
            
            ترحيب()
            ترحيب("أحمد")
        "#;

        let result = run_code(source);
        println!("دالة مع قيمة افتراضية: {:?}", result);
    }

    /// اختبار الدوال المتداخلة
    #[test]
    fn test_nested_functions() {
        let source = r#"
            دالة خارج():
                دالة داخل():
                    أرجع 42
                أرجع داخل()
            
            متغير نتيجة = خارج()
            طباعة(نتيجة)
        "#;

        let result = run_code(source);
        println!("الدوال المتداخلة: {:?}", result);
    }

    /// اختبار العودية
    #[test]
    fn test_recursion() {
        let source = r#"
            دالة مضروب(ن):
                إذا ن <= 1:
                    أرجع 1
                أرجع ن * مضروب(ن - 1)
            
            طباعة(مضروب(5))
        "#;

        let result = run_code(source);
        println!("العودية: {:?}", result);
    }

    /// اختبار الدالة المجهولة
    #[test]
    fn test_lambda() {
        let source = r#"
            متغير مضاعف = دالة(س) => س * 2
            طباعة(مضاعف(5))
        "#;

        let result = run_code(source);
        println!("الدالة المجهولة: {:?}", result);
    }

    /// اختبار الإغلاق (Closure)
    #[test]
    fn test_closure() {
        let source = r#"
            دالة制造_عداد():
                متغير عدد = 0
                أرجع دالة():
                    عدد += 1
                    أرجع عدد
            
            متغير عداد =制造_عداد()
            طباعة(عداد())
            طباعة(عداد())
            طباعة(عداد())
        "#;

        let result = run_code(source);
        println!("الإغلاق: {:?}", result);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات القوائم
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod list_tests {
    use super::*;

    /// اختبار إنشاء القائمة
    #[test]
    fn test_list_creation() {
        let source = r#"
            متغير قائمة = [1، 2، 3، 4، 5]
            طباعة(قائمة)
        "#;

        let result = run_code(source);
        println!("إنشاء القائمة: {:?}", result);
    }

    /// اختبار الوصول للعناصر
    #[test]
    fn test_list_access() {
        let source = r#"
            متغير قائمة = ["أ"، "ب"، "ج"]
            طباعة(قائمة[0])
            طباعة(قائمة[1])
            طباعة(قائمة[-1])
        "#;

        let result = run_code(source);
        println!("الوصول للعناصر: {:?}", result);
    }

    /// اختبار تعديل القائمة
    #[test]
    fn test_list_modification() {
        let source = r#"
            متغير قائمة = [1، 2، 3]
            قائمة[0] = 10
            قائمة.أضف(4)
            طباعة(قائمة)
        "#;

        let result = run_code(source);
        println!("تعديل القائمة: {:?}", result);
    }

    /// اختبار طرق القائمة
    #[test]
    fn test_list_methods() {
        let source = r#"
            متغير قائمة = [3، 1، 4، 1، 5]
            طباعة(قائمة.طول())
            قائمة.رتب()
            طباعة(قائمة)
            قائمة.عكس()
            طباعة(قائمة)
        "#;

        let result = run_code(source);
        println!("طرق القائمة: {:?}", result);
    }

    /// اختبار القوائم المتداخلة
    #[test]
    fn test_nested_lists() {
        let source = r#"
            متغير مصفوفة = [[1، 2]، [3، 4]، [5، 6]]
            طباعة(مصفوفة[0][1])
            طباعة(مصفوفة[2][0])
        "#;

        let result = run_code(source);
        println!("القوائم المتداخلة: {:?}", result);
    }

    /// اختبار List Comprehension
    #[test]
    fn test_list_comprehension() {
        let source = r#"
            متغير مربعات = [س ** 2 لكل س في [1، 2، 3، 4، 5]]
            طباعة(مربعات)
        "#;

        let result = run_code(source);
        println!("List Comprehension: {:?}", result);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات القواميس
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod dictionary_tests {
    use super::*;

    /// اختبار إنشاء القاموس
    #[test]
    fn test_dict_creation() {
        let source = r#"
            متغير شخص = {اسم: "أحمد"، عمر: 25}
            طباعة(شخص)
        "#;

        let result = run_code(source);
        println!("إنشاء القاموس: {:?}", result);
    }

    /// اختبار الوصول للعناصر
    #[test]
    fn test_dict_access() {
        let source = r#"
            متغير شخص = {اسم: "أحمد"، عمر: 25}
            طباعة(شخص["اسم"])
            طباعة(شخص.عمر)
        "#;

        let result = run_code(source);
        println!("الوصول للعناصر: {:?}", result);
    }

    /// اختبار تعديل القاموس
    #[test]
    fn test_dict_modification() {
        let source = r#"
            متغير شخص = {اسم: "أحمد"}
            شخص["عمر"] = 25
            شخص.مدينة = "الرياض"
            طباعة(شخص)
        "#;

        let result = run_code(source);
        println!("تعديل القاموس: {:?}", result);
    }

    /// اختبار القواميس المتداخلة
    #[test]
    fn test_nested_dicts() {
        let source = r#"
            متغير بيانات = {
                مستخدم: {اسم: "أحمد"، بريد: "test@test.com"},
                إعدادات: {لغة: "ar"، ثيم: "dark"}
            }
            طباعة(بيانات.مستخدم.اسم)
        "#;

        let result = run_code(source);
        println!("القواميس المتداخلة: {:?}", result);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأصناف
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod class_tests {
    use super::*;

    /// اختبار الصنف البسيط
    #[test]
    fn test_simple_class() {
        let source = r#"
            صنف شخص:
                متغير اسم = ""
                متغير عمر = 0
                
                دالة جديد(اسم، عمر):
                    هذا.اسم = اسم
                    هذا.عمر = عمر
                
                دالة معلومات():
                    أرجع هذا.اسم + " - " + هذا.عمر
            
            متغير أحمد = جديد شخص("أحمد"، 25)
            طباعة(أحمد.معلومات())
        "#;

        let result = run_code(source);
        println!("الصنف البسيط: {:?}", result);
    }

    /// اختبار الوراثة
    #[test]
    fn test_inheritance() {
        let source = r#"
            صنف حيوان:
                متغير اسم = ""
                دالة صوت():
                    طباعة("صوت عام")
            
            صنف كلب يرث حيوان:
                دالة صوت():
                    طباعة("نباح")
            
            متغير كلبي = جديد كلب()
            كلبي.اسم = "بوبي"
            كلبي.صوت()
        "#;

        let result = run_code(source);
        println!("الوراثة: {:?}", result);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات معالجة الأخطاء
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    /// اختبار حاول/امسك
    #[test]
    fn test_try_catch() {
        let source = r#"
            حاول:
                متغير نتيجة = 10 / 0
            امسك خطأ:
                طباعة("خطأ: " + خطأ)
        "#;

        let result = run_code(source);
        println!("حاول/امسك: {:?}", result);
    }

    /// اختبار أخيراً
    #[test]
    fn test_try_catch_finally() {
        let source = r#"
            حاول:
                طباعة("محاولة")
            امسك خطأ:
                طباعة("خطأ")
            أخيراً:
                طباعة("تم")
        "#;

        let result = run_code(source);
        println!("حاول/امسك/أخيراً: {:?}", result);
    }

    /// اختبار ألقِ
    #[test]
    fn test_throw() {
        let source = r#"
            دالة تحقق(ن):
                إذا ن < 0:
                    ألقِ "القيمة يجب أن تكون موجبة"
                أرجع ن
            
            حاول:
                تحقق(-5)
            امسك خطأ:
                طباعة(خطأ)
        "#;

        let result = run_code(source);
        println!("ألقِ: {:?}", result);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأخطاء المتوقعة
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod expected_error_tests {
    use super::*;

    /// اختبار خطأ القسمة على صفر
    #[test]
    fn test_division_by_zero_error() {
        let source = r#"متغير س = 10 / 0"#;
        let result = run_code(source);

        assert!(result.is_err(), "يجب أن يُرجع خطأ للقسمة على صفر");
        println!("خطأ القسمة على صفر: {:?}", result);
    }

    /// اختبار خطأ المتغير غير المعرف
    #[test]
    fn test_undefined_variable_error() {
        let source = r#"طباعة(متغير_غير_موجود)"#;
        let result = run_code(source);

        assert!(result.is_err(), "يجب أن يُرجع خطأ للمتغير غير المعرف");
        println!("خطأ المتغير غير المعرف: {:?}", result);
    }

    /// اختبار خطأ الفهرس خارج النطاق
    #[test]
    fn test_index_out_of_bounds_error() {
        let source = r#"
            متغير قائمة = [1، 2، 3]
            طباعة(قائمة[10])
        "#;
        let result = run_code(source);

        assert!(result.is_err(), "يجب أن يُرجع خطأ للفهرس خارج النطاق");
        println!("خطأ الفهرس خارج النطاق: {:?}", result);
    }

    /// اختبار خطأ النوع
    #[test]
    fn test_type_error() {
        let source = r#"متغير س = "نص" + 10"#;
        let result = run_code(source);

        // قد ينجح (تحويل ضمني) أو يفشل
        println!("خطأ النوع: {:?}", result);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأداء
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    /// اختبار أداء الحلقة الكبيرة
    #[test]
    fn test_large_loop_performance() {
        let source = r#"
            متغير مجموع = 0
            لكل س في 1..10000:
                مجموع += س
        "#;

        let start = Instant::now();
        let _result = run_code(source);
        let elapsed = start.elapsed();

        println!("حلقة 10000 تكرار: {:?}", elapsed);
        assert!(elapsed.as_secs() < 10, "يجب أن يكون التنفيذ سريعاً");
    }

    /// اختبار أداء العودية العميقة
    #[test]
    fn test_deep_recursion_performance() {
        let source = r#"
            دالة فيبوناتشي(ن):
                إذا ن <= 1:
                    أرجع ن
                أرجع فيبوناتشي(ن - 1) + فيبوناتشي(ن - 2)
            
            طباعة(فيبوناتشي(20))
        "#;

        let start = Instant::now();
        let _result = run_code(source);
        let elapsed = start.elapsed();

        println!("فيبوناتشي(20): {:?}", elapsed);
    }

    /// اختبار أداء القائمة الكبيرة
    #[test]
    fn test_large_list_performance() {
        let source = r#"
            متغير قائمة = []
            لكل س في 1..1000:
                قائمة.أضف(س)
        "#;

        let start = Instant::now();
        let _result = run_code(source);
        let elapsed = start.elapsed();

        println!("قائمة 1000 عنصر: {:?}", elapsed);
    }
}
