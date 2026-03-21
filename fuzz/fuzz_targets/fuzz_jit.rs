// ═══════════════════════════════════════════════════════════════════════════════
// Fuzz Target: JIT Compiler
// ═══════════════════════════════════════════════════════════════════════════════
// Target: JIT compiler correctness
// Detect: Panics, crashes, infinite loops during JIT compilation/execution
// ═══════════════════════════════════════════════════════════════════════════════

#![no_main]

use libfuzzer_sys::fuzz_target;
use almarjaa::bytecode::{Compiler, CompleteV2JitCompiler};
use std::rc::Rc;
use std::cell::RefCell;
use almarjaa::interpreter::value::Environment;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        // تجربة الترجمة
        if let Ok(chunk) = Compiler::compile_source(input) {
            let mut jit = CompleteV2JitCompiler::new();
            let mut globals = Rc::new(RefCell::new(Environment::new()));
            let _ = jit.execute(&chunk, &mut globals);
        }
    }
});
