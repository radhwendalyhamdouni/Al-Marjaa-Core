// ═══════════════════════════════════════════════════════════════════════════════
// Fuzz Target: Parser
// ═══════════════════════════════════════════════════════════════════════════════
// Target: Parsing correctness
// Detect: Panics, crashes, infinite loops during parsing
// ═══════════════════════════════════════════════════════════════════════════════

#![no_main]

use libfuzzer_sys::fuzz_target;
use almarjaa::parser::Parser;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        // تجربة التحليل
        let _ = Parser::parse(input);
    }
});
