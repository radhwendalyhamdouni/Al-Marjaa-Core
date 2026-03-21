// ═══════════════════════════════════════════════════════════════════════════════
// Core Module - الوحدة الأساسية
// ═══════════════════════════════════════════════════════════════════════════════
// هذه الوحدة تحتوي على المكونات الأساسية للغة:
// - Lexer: محلل النصوص المعجمي
// - Parser: محلل البنية النحوية
// - Interpreter: المفسر
// - Error: نظام معالجة الأخطاء
// - Bytecode: نظام البايت كود و JIT
// ═══════════════════════════════════════════════════════════════════════════════

//! # Core Module - الوحدة الأساسية
//! 
//! هذه الوحدة توفر API مستقر للمكونات الأساسية للغة المرجع.
//! 
//! ## المكونات
//! 
//! - [`lexer`] - محلل النصوص المعجمي
//! - [`parser`] - محلل البنية النحوية  
//! - [`interpreter`] - المفسر
//! - [`error`] - نظام معالجة الأخطاء
//! - [`bytecode`] - نظام البايت كود و JIT
//! 
//! ## مثال
//! 
//! ```rust,ignore
//! use almarjaa::core::{Lexer, Parser, Interpreter};
//! 
//! let source = r#"متغير س = 10؛"#;
//! let mut lexer = Lexer::new(source);
//! let tokens = lexer.tokenize()?;
//! let ast = Parser::parse(source)?;
//! let mut interp = Interpreter::new();
//! interp.run(source)?;
//! ```

// إعادة تصدير المكونات الأساسية
pub use crate::lexer::{self, Lexer};
pub use crate::parser::{self, Parser};
pub use crate::interpreter::{self, Interpreter};
pub use crate::error::{self, AlMarjaaError, ErrorCode, Position, Severity, Span};
pub use crate::bytecode::{self, Compiler, VM, Chunk, OpCode};

/// Core types and traits
pub mod types {
    //! أنواع البيانات الأساسية
    
    pub use crate::interpreter::value::{Value, Environment};
    pub use crate::bytecode::{ExecutionResult, VMStats};
}

/// Core traits for extensibility
pub mod traits {
    //! السمات الأساسية للتوسع
    
    use crate::interpreter::value::Value;

    /// سمة للمفسرين
    pub trait InterpreterTrait {
        /// تنفيذ كود مصدري
        fn run(&mut self, source: &str) -> Result<Value, String>;
        
        /// تنفيذ ملف
        fn run_file(&mut self, path: &str) -> Result<Value, String>;
    }

    /// سمة للمحللات
    pub trait ParserTrait {
        /// نوع شجرة البنية
        type Output;
        
        /// تحليل كود مصدري
        fn parse(source: &str) -> Result<Self::Output, String>;
    }

    /// سمة للمحللات المعجمية
    pub trait LexerTrait {
        /// نوع الرمز
        type Token;
        
        /// تحليل كود مصدري
        fn tokenize(&mut self) -> Result<Vec<Self::Token>, String>;
    }
}
