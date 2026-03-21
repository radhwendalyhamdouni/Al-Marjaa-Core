// ═══════════════════════════════════════════════════════════════════════════════
// Fuzz Target: Arabic Text Handling
// ═══════════════════════════════════════════════════════════════════════════════
// Target: Arabic text handling
// Detect: Issues with Arabic characters, diacritics, RTL, ligatures
// Focus: Real-world Arabic text patterns
// ═══════════════════════════════════════════════════════════════════════════════

#![no_main]

use libfuzzer_sys::fuzz_target;
use almarjaa::lexer::Lexer;
use almarjaa::parser::Parser;
use almarjaa::interpreter::Interpreter;
use arbitrary::{Arbitrary, Unstructured};

/// مدخلات عربية عشوائية
#[derive(Debug, Clone)]
struct ArabicFuzzInput {
    /// المعرفات
    identifiers: Vec<String>,
    /// النصوص
    strings: Vec<String>,
    /// الأرقام
    numbers: Vec<f64>,
}

impl<'a> Arbitrary<'a> for ArabicFuzzInput {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        // الأحرف العربية الأساسية
        let arabic_chars = "ابتثجحخدذرزسشصضطظعغفقكلمنهويءآأؤإئ";
        
        // أحرف التشكيل
        let diacritics = ['َ', 'ِ', 'ُ', 'ً', 'ٍ', 'ٌ', 'ّ', 'ْ'];
        
        // توليد معرفات عشوائية
        let mut identifiers = Vec::new();
        for _ in 0..u.int_in_range(1..=10)? {
            let len = u.int_in_range(1..=20)?;
            let mut id = String::new();
            for i in 0..len {
                let idx = u.int_in_range(0..=(arabic_chars.len() - 1))?;
                id.push(arabic_chars.chars().nth(idx).unwrap());
                // إضافة تشكيل عشوائي
                if i % 3 == 0 && u.int_in_range(0..=1)? == 0 {
                    let d_idx = u.int_in_range(0..=(diacritics.len() - 1))?;
                    id.push(diacritics[d_idx]);
                }
            }
            identifiers.push(id);
        }
        
        // توليد نصوص عشوائية
        let mut strings = Vec::new();
        for _ in 0..u.int_in_range(1..=5)? {
            let len = u.int_in_range(1..=50)?;
            let mut s = String::new();
            for _ in 0..len {
                let idx = u.int_in_range(0..=(arabic_chars.len() - 1))?;
                s.push(arabic_chars.chars().nth(idx).unwrap());
                // مسافة عشوائية
                if u.int_in_range(0..=5)? == 0 {
                    s.push(' ');
                }
            }
            strings.push(s);
        }
        
        // توليد أرقام عشوائية
        let mut numbers = Vec::new();
        for _ in 0..u.int_in_range(1..=10)? {
            numbers.push(u.arbitrary::<f64>()?);
        }
        
        Ok(ArabicFuzzInput { identifiers, strings, numbers })
    }
}

fuzz_target!(|data: &[u8]| {
    // تجربة مع أي مدخلات
    if let Ok(input) = std::str::from_utf8(data) {
        // 1. Lexer
        let mut lexer = Lexer::new(input);
        let _ = lexer.tokenize();
        
        // 2. Parser
        let _ = Parser::parse(input);
        
        // 3. Interpreter (with time limit)
        let mut interp = Interpreter::new();
        let _ = interp.run(input);
    }
    
    // تجربة مع مدخلات عربية منظمة
    if let Ok(mut unstructured) = Unstructured::new(data) {
        if let Ok(arabic_input) = ArabicFuzzInput::arbitrary(&mut unstructured) {
            // بناء برنامج عربي
            let mut program = String::new();
            
            // تعريف متغيرات
            for (i, id) in arabic_input.identifiers.iter().enumerate() {
                if let Some(&num) = arabic_input.numbers.get(i % arabic_input.numbers.len()) {
                    program.push_str(&format!("متغير {} = {}؛\n", id, num));
                }
            }
            
            // تعريف نصوص
            for (i, s) in arabic_input.strings.iter().enumerate() {
                program.push_str(&format!("متغير نص{} = \"{}\"؛\n", i, s));
            }
            
            // تجربة البرنامج
            let mut lexer = Lexer::new(&program);
            let _ = lexer.tokenize();
            let _ = Parser::parse(&program);
            let mut interp = Interpreter::new();
            let _ = interp.run(&program);
        }
    }
});
