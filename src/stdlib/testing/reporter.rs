// src/stdlib/testing/reporter.rs
// مُخبر نتائج الاختبارات
// Test Reporter for Al-Marjaa Language

use super::runner::نتائج_المجموعة;
use std::fmt::Write;

/// مُخبر النتائج
pub struct مُخبر_النتائج;

impl مُخبر_النتائج {
    /// طباعة تقرير نصي
    pub fn تقرير_نصي(النتائج: &نتائج_المجموعة) -> String {
        let mut output = String::new();
        
        let _ = writeln!(output, "════════════════════════════════════════════════════════");
        let _ = writeln!(output, "📊 تقرير الاختبارات");
        let _ = writeln!(output, "════════════════════════════════════════════════════════");
        
        // ملخص
        let _ = writeln!(output);
        let _ = writeln!(output, "📈 ملخص:");
        let _ = writeln!(output, "─────────────");
        let _ = writeln!(output, "  ✅ نجح: {}", النتائج.عدد_النجح);
        let _ = writeln!(output, "  ❌ فشل: {}", النتائج.عدد_الفشل);
        let _ = writeln!(output, "  📊 المجموع: {}", النتائج.الاختبارات.len());
        let _ = writeln!(output, "  📈 نسبة النجاح: {:.1}%", النتائج.نسبة_النجح() * 100.0);
        
        // الوقت
        if let Some(time) = النتائج.الوقت_الإجمالي() {
            let _ = writeln!(output, "  ⏱ الوقت: {:?}", time);
        }
        
        // تفاصيل الاختبارات
        if !النتائج.الاختبارات.is_empty() {
            let _ = writeln!(output);
            let _ = writeln!(output, "📋 تفاصيل الاختبارات:");
            let _ = writeln!(output, "───────────────────────");
            
            for (i, test) in النتائج.الاختبارات.iter().enumerate() {
                let status = if test.ناجح { "✅" } else { "❌" };
                let _ = writeln!(
                    output,
                    "{}. {} [{}] - {}",
                    i + 1, status, test.الاسم, test.الرسالة
                );
                
                if let Some(ref details) = test.تفاصيل {
                    for line in details.lines().take(3) {
                        let _ = writeln!(output, "    {}", line);
                    }
                    if details.lines().count() > 3 {
                        let _ = writeln!(output, "    ...");
                    }
                }
            }
        }
        
        let _ = writeln!(output);
        let _ = writeln!(output, "════════════════════════════════════════════════════════");
        
        output
    }

    /// طباعة تقرير ملون
    pub fn طباعة_ملون(النتائج: &نتائج_المجموعة) {
        // رموز ANSI للألوان
        let green = "\x1b[32m";
        let red = "\x1b[31m";
        let cyan = "\x1b[36m";
        let yellow = "\x1b[33m";
        let bold = "\x1b[1m";
        let reset = "\x1b[0m";
        
        println!("\n{}════════════════════════════════════════════════════════{}", cyan, reset);
        println!("{}📊 تقرير الاختبارات{}", bold, reset);
        println!("{}════════════════════════════════════════════════════════{}\n", cyan, reset);
        
        // ملخص
        println!("{}📈 ملخص:{}", yellow, reset);
        println!("─────────────");
        println!("  {}✅ نجح:{} {}", green, reset, النتائج.عدد_النجح);
        println!("  {}❌ فشل:{} {}", red, reset, النتائج.عدد_الفشل);
        println!("  📊 المجموع: {}", النتائج.الاختبارات.len());
        
        let success_rate = النتائج.نسبة_النجح() * 100.0;
        let color = if success_rate >= 80.0 { green } else if success_rate >= 50.0 { yellow } else { red };
        println!("  {}📈 نسبة النجاح: {:.1}%{}", color, success_rate, reset);
        
        // الوقت
        if let Some(time) = النتائج.الوقت_الإجمالي() {
            println!("  ⏱ الوقت: {:?}", time);
        }
        
        // تفاصيل الاختبارات
        if !النتائج.الاختبارات.is_empty() {
            println!("\n{}📋 تفاصيل الاختبارات:{}", yellow, reset);
            println!("───────────────────────");
            
            for (i, test) in النتائج.الاختبارات.iter().enumerate() {
                let (status, color) = if test.ناجح { ("✅", green) } else { ("❌", red) };
                println!(
                    "{}. {} {}[{}]{} - {}",
                    i + 1, color, status, test.الاسم, reset, test.الرسالة
                );
                
                if let Some(ref details) = test.تفاصيل {
                    for line in details.lines().take(3) {
                        println!("    {}", line);
                    }
                    if details.lines().count() > 3 {
                        println!("    ...");
                    }
                }
            }
        }
        
        println!("\n{}════════════════════════════════════════════════════════{}\n", cyan, reset);
    }

    /// تقرير JSON
    pub fn تقرير_json(النتائج: &نتائج_المجموعة) -> String {
        let mut tests_json = Vec::new();
        
        for test in &النتائج.الاختبارات {
            let status = if test.ناجح { "passed" } else { "failed" };
            let details_json = test.تفاصيل.as_ref()
                .map(|d| format!("\"{}\"", d.replace("\"", "\\\"")))
                .unwrap_or_else(|| "null".to_string());
            
            tests_json.push(format!(
                r#"    {{
      "name": "{}",
      "status": "{}",
      "message": "{}",
      "duration_ms": {},
      "details": {}
    }}"#,
                test.الاسم,
                status,
                test.الرسالة,
                test.الوقت.as_millis(),
                details_json
            ));
        }
        
        let duration_ms = النتائج.الوقت_الإجمالي()
            .map(|d| d.as_millis())
            .unwrap_or(0);
        
        format!(
            r#"{{
  "summary": {{
    "passed": {},
    "failed": {},
    "total": {},
    "success_rate": {},
    "duration_ms": {}
  }},
  "tests": [
{}
  ]
}}"#,
            النتائج.عدد_النجح,
            النتائج.عدد_الفشل,
            النتائج.الاختبارات.len(),
            النتائج.نسبة_النجح(),
            duration_ms,
            tests_json.join(",\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report() {
        let mut runner = super::super::runner::مشغل_الاختبارات::جديد();
        
        runner.تشغيل("اختبار_1", || assert!(true));
        runner.تشغيل("اختبار_2", || assert_eq!(1, 1));
        runner.إنهاء();
        
        let report = مُخبر_النتائج::تقرير_نصي(runner.النتائج());
        assert!(report.contains("نجح: 2"));
    }
}
