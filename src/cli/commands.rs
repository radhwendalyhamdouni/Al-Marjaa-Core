use std::fs;
use std::process;
use std::time::Instant;

use colored::Colorize;

use crate::cli::args::RunOptions;
use almarjaa::formatter::format_source;
use almarjaa::interpreter::Interpreter;
use almarjaa::linter::{lint_source_with_config, LintConfig};

pub fn print_help() {
    println!("{}", crate::rtl("استخدام:").bright_yellow());
    println!("  almarjaa [خيارات] [ملف]");
    println!();
    println!("{}", crate::rtl("خيارات:").bright_yellow());
    println!("  -h, --help         عرض هذه الرسالة");
    println!("  -v, --version      عرض الإصدار");
    println!("  -r, --repl         تشغيل الوضع التفاعلي (افتراضي)");
    println!("  -c, --compile      تحليل الملف فقط (بدون تنفيذ)");
    println!("  -f, --format       تنسيق الملف وطباعته");
    println!("  -t, --tokens       عرض الرموز المميزة");
    println!("  -l, --lint         تحليل الشيفرة بقواعد linter (تحذيرات فقط)");
    println!("      --lint-disable  تعطيل قاعدة lint (يمكن تكرارها)");
    println!("      --lint-max      الحد الأقصى لعدد التحذيرات المعروضة");
    println!("  -a, --ast          عرض شجرة الصياغة");
    println!("  -d, --debug        عرض تفاصيل التحليل والتنفيذ");
    println!("  jit <cmd>          JIT Compiler: run/repl/info/benchmark");
    println!("  أوامر ذكاء REPL    اكتب: ذكاء");
    println!();
    println!("{}", crate::rtl("أمثلة:").bright_yellow());
    println!("  almarjaa program.mrj          تنفيذ ملف برنامج");
    println!("  almarjaa -r                   تشغيل الوضع التفاعلي");
    println!("  almarjaa -t program.mrj       عرض الرموز المميزة");
    println!("  almarjaa jit run app.mrj      تنفيذ مع JIT");
    println!("  almarjaa jit info             معلومات JIT");
}

pub fn print_version(version: &str) {
    println!(
        "{}",
        crate::rtl(&format!("لغة المرجع - الإصدار {}", version))
    );
    println!("{}", crate::rtl("لغة برمجة عربية متكاملة"));
    println!("2024 - فريق المرجع");
}

pub fn run_file(filename: &str, options: &RunOptions) {
    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "{}",
                crate::rtl(&format!("خطأ في قراءة الملف '{}': {}", filename, e)).bright_red()
            );
            process::exit(1);
        }
    };

    if options.debug {
        println!(
            "{}",
            crate::rtl(&format!("[debug] الملف: {}", filename)).bright_blue()
        );
        println!(
            "{}",
            crate::rtl(&format!("[debug] الحجم: {} بايت", content.len())).bright_blue()
        );
    }

    if options.format_only {
        let formatted = format_source(&content);
        print!("{}", formatted);
        return;
    }

    if options.lint_only {
        let lint_start = Instant::now();
        let lint_config = LintConfig {
            disabled_rules: options.lint_disabled_rules.iter().cloned().collect(),
            max_diagnostics: options.lint_max,
        };

        match lint_source_with_config(&content, &lint_config) {
            Ok(diagnostics) => {
                if diagnostics.is_empty() {
                    println!("{}", crate::rtl("✅ لا توجد تحذيرات lint").bright_green());
                } else {
                    println!("{}", crate::rtl("تحذيرات lint:").bright_yellow());
                    for diagnostic in &diagnostics {
                        println!("- [{}] {}", diagnostic.code, diagnostic.message);
                    }
                }

                if options.debug {
                    println!(
                        "{}",
                        crate::rtl(&format!(
                            "[debug] عدد تحذيرات lint: {} | زمن التحليل: {:?}",
                            diagnostics.len(),
                            lint_start.elapsed()
                        ))
                        .bright_blue()
                    );
                }
            }
            Err(err) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!("خطأ أثناء lint: {}", err)).bright_red()
                );
                process::exit(1);
            }
        }
        return;
    }

    if options.show_tokens {
        use almarjaa::lexer::Lexer;
        let lex_start = Instant::now();
        let mut lexer = Lexer::new(&content);
        match lexer.tokenize() {
            Ok(tokens) => {
                println!("{}", crate::rtl("=== الرموز المميزة ===").bright_cyan());
                for token in &tokens {
                    println!("{:4}:{:4} {:?}", token.line, token.column, token.token_type);
                }
                if options.debug {
                    println!(
                        "{}",
                        crate::rtl(&format!(
                            "[debug] عدد الرموز: {} | زمن التحليل اللغوي: {:?}",
                            tokens.len(),
                            lex_start.elapsed()
                        ))
                        .bright_blue()
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!("خطأ في التحليل اللغوي: {}", e)).bright_red()
                );
                process::exit(1);
            }
        }
        return;
    }

    use almarjaa::parser::Parser;
    if options.show_ast || options.compile_only || options.debug {
        let parse_start = Instant::now();
        match Parser::parse(&content) {
            Ok(program) => {
                if options.show_ast {
                    println!("{}", crate::rtl("=== شجرة الصياغة ===").bright_cyan());
                    for (i, stmt) in program.statements.iter().enumerate() {
                        println!("{:<3} {:?}", i + 1, stmt);
                    }
                    if options.debug {
                        println!(
                            "{}",
                            crate::rtl(&format!(
                                "[debug] عدد التعليمات: {} | زمن التحليل النحوي: {:?}",
                                program.statements.len(),
                                parse_start.elapsed()
                            ))
                            .bright_blue()
                        );
                    }
                    return;
                }

                if options.compile_only {
                    println!(
                        "{}",
                        crate::rtl(&format!(
                            "نجح التحليل: {} تعليمة | الزمن: {:?}",
                            program.statements.len(),
                            parse_start.elapsed()
                        ))
                        .bright_green()
                    );
                    return;
                }

                if options.debug {
                    println!(
                        "{}",
                        crate::rtl(&format!(
                            "[debug] عدد التعليمات: {} | زمن التحليل النحوي: {:?}",
                            program.statements.len(),
                            parse_start.elapsed()
                        ))
                        .bright_blue()
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!(
                        "خطأ في التحليل: {} (السطر {}، العمود {})",
                        e.message, e.line, e.column
                    ))
                    .bright_red()
                );
                process::exit(1);
            }
        }
    }

    let mut interpreter = Interpreter::new();
    let exec_start = Instant::now();

    match interpreter.run(&content) {
        Ok(_) => {
            if options.debug {
                println!(
                    "{}",
                    crate::rtl(&format!("[debug] زمن التنفيذ: {:?}", exec_start.elapsed()))
                        .bright_blue()
                );
            }
        }
        Err(e) => {
            eprintln!(
                "{}",
                crate::rtl(&format!("خطأ في التنفيذ: {}", e.message)).bright_red()
            );
            process::exit(1);
        }
    }
}

/// معالجة أوامر JIT Compiler
pub fn handle_jit_command(options: &RunOptions, filename: Option<&str>) -> bool {
    if !options.jit_run && !options.jit_repl {
        return false;
    }

    if options.jit_repl {
        // تشغيل REPL مع JIT
        println!(
            "{}",
            crate::rtl("🚀 تشغيل REPL مع JIT Compiler...").bright_cyan()
        );
        println!("{}", crate::rtl("اضغط Ctrl+D للخروج").bright_yellow());
        println!();

        // استخدام REPL العادي مع تفعيل JIT
        crate::cli::repl::run_repl();
        return true;
    }

    if options.jit_run {
        let fname = match filename {
            Some(f) => f,
            None => {
                eprintln!("{}", crate::rtl("JIT run يحتاج ملفاً").bright_red());
                process::exit(1);
            }
        };

        let content = match fs::read_to_string(fname) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!("خطأ في قراءة الملف '{}': {}", fname, e)).bright_red()
                );
                process::exit(1);
            }
        };

        println!(
            "{}",
            crate::rtl("🔥 تشغيل مع JIT Compiler v2...").bright_cyan()
        );

        let start = Instant::now();

        // استخدام JIT v2 الجديد مع دعم العودية
        use almarjaa::bytecode::{Compiler, CompleteV2JitCompiler};
        use std::rc::Rc;
        use std::cell::RefCell;
        use almarjaa::interpreter::value::Environment;
        
        // ترجمة إلى bytecode
        let chunk = match Compiler::compile_source(&content) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", crate::rtl(&format!("❌ خطأ في الترجمة: {}", e)).bright_red());
                process::exit(1);
            }
        };
        
        // إنشاء JIT وتنفيذ
        let mut jit = CompleteV2JitCompiler::new();
        let mut globals = Rc::new(RefCell::new(Environment::new()));
        
        match jit.execute(&chunk, &mut globals) {
            Ok(value) => {
                let elapsed = start.elapsed();
                println!();
                println!(
                    "{}",
                    crate::rtl("═════════════════════════════════════").bright_green()
                );
                println!(
                    "{}",
                    crate::rtl("✅ تم التنفيذ بنجاح").bright_green().bold()
                );
                println!(
                    "{}",
                    crate::rtl("═════════════════════════════════════").bright_green()
                );

                if options.jit_benchmark {
                    println!();
                    println!("{}", crate::rtl("📊 إحصائيات الأداء:").bright_cyan());
                    let stats = jit.stats();
                    println!("  ⏱️  الوقت الكلي: {:.2} ميكروثانية", elapsed.as_micros());
                    println!("  ⏱️  وقت التنفيذ: {} ميكروثانية", stats.total_exec_time_us);
                    println!("  📦 إجمالي التنفيذات: {}", stats.total_executions);
                    println!("  🔄 الاستدعاءات العودية: {}", stats.recursive_calls);
                    println!("  📊 أقصى عمق استدعاء: {}", stats.max_call_depth);
                    println!("  ⚡ المهام Async: {}", stats.async_tasks_created);
                    println!("  📦 النتيجة: {:?}", value);
                    
                    // طباعة تقرير JIT
                    jit.print_report();
                }
            }
            Err(e) => {
                eprintln!("{}", crate::rtl(&format!("❌ خطأ: {}", e)).bright_red());
                process::exit(1);
            }
        }
    }

    true
}


