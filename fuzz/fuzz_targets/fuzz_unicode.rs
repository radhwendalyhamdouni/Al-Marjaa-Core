// ═══════════════════════════════════════════════════════════════════════════════
// Fuzz Target: Unicode Handling
// ═══════════════════════════════════════════════════════════════════════════════
// Target: Unicode/Arabic handling
// Detect: Issues with Arabic characters, RTL, diacritics
// ═══════════════════════════════════════════════════════════════════════════════

#![no_main]

use libfuzzer_sys::fuzz_target;
use almarjaa::lexer::Lexer;
use almarjaa::parser::Parser;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        // تجربة على Lexer
        let mut lexer = Lexer::new(input);
        let _ = lexer.tokenize();
        
        // تجربة على Parser
        let _ = Parser::parse(input);
    }
});
