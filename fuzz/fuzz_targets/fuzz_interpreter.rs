// ═══════════════════════════════════════════════════════════════════════════════
// Fuzz Target: Interpreter
// ═══════════════════════════════════════════════════════════════════════════════
// Target: Runtime stability
// Detect: Panics, crashes, infinite loops during interpretation
// ═══════════════════════════════════════════════════════════════════════════════

#![no_main]

use libfuzzer_sys::fuzz_target;
use almarjaa::interpreter::Interpreter;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        // تجربة التنفيذ
        let mut interp = Interpreter::new();
        let _ = interp.run(input);
    }
});
