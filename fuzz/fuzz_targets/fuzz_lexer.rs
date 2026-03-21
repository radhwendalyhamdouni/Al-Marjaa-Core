// ═══════════════════════════════════════════════════════════════════════════════
// Fuzz Target: Lexer
// ═══════════════════════════════════════════════════════════════════════════════
// Target: Tokenization robustness
// Detect: Panics, crashes, infinite loops
// ═══════════════════════════════════════════════════════════════════════════════

#![no_main]

use libfuzzer_sys::fuzz_target;
use almarjaa::lexer::Lexer;

fuzz_target!(|data: &[u8]| {
    // تحويل البايتات إلى نص
    if let Ok(input) = std::str::from_utf8(data) {
        // إنشاء lexer وتجربة التحويل
        let mut lexer = Lexer::new(input);
        
        // يجب ألا يحدث panic أو crash
        let _ = lexer.tokenize();
    }
});
