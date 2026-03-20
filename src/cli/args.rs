use std::process;

use colored::Colorize;

#[derive(Debug, Clone, Default)]
pub struct RunOptions {
    pub show_tokens: bool,
    pub show_ast: bool,
    pub compile_only: bool,
    pub format_only: bool,
    pub debug: bool,
    pub lint_only: bool,
    pub lint_disabled_rules: Vec<String>,
    pub lint_max: Option<usize>,
    pub pm_init: Option<String>,
    pub pm_check: bool,
    pub pm_tree: bool,
    pub lsp_diag: bool,
    pub lsp_complete: Option<String>,
    pub lsp_hover: Option<(usize, usize)>,
    pub lsp_definition: Option<(usize, usize)>,
    pub lsp_references: Option<(usize, usize)>,
    // تصدير الهواتف المحمولة
    pub mobile_export: bool,
    pub mobile_platform: Option<String>,
    pub mobile_framework: Option<String>,
    pub mobile_project_name: Option<String>,
    // LLVM Backend
    pub llvm_compile: bool,
    pub llvm_handled: bool, // للأوامر مثل targets و opt
    pub llvm_opt_level: Option<u8>,
    pub llvm_target: Option<String>,
    pub llvm_output: Option<String>,
    pub llvm_emit_ir: bool,
    pub llvm_emit_bc: bool,
    pub llvm_object_only: bool,
    pub llvm_run: bool,
    // JIT Compilation
    pub jit_run: bool,
    pub jit_repl: bool,
    pub jit_tiered: bool,
    pub jit_benchmark: bool,
    pub jit_info: bool,
    pub jit_bench: bool,
    pub jit_handled: bool, // للأوامر مثل info و benchmark
}

pub struct ParsedCli {
    pub options: RunOptions,
    pub filename: Option<String>,
    pub run_repl_flag: bool,
}

pub enum CliAction {
    Help,
    Version,
    Run(Box<ParsedCli>),
}

pub fn parse_args(args: &[String]) -> CliAction {
    let mut options = RunOptions::default();
    let mut filename: Option<String> = None;
    let mut run_repl_flag = false;

    if args.len() >= 2 {
        match args[1].as_str() {
            "pm" => parse_pm_subcommand(args, &mut options),
            "lsp" => parse_lsp_subcommand(args, &mut options, &mut filename),
            "mobile" => parse_mobile_subcommand(args, &mut options, &mut filename),
            "llvm" => parse_llvm_subcommand(args, &mut options, &mut filename),
            "jit" => parse_jit_subcommand(args, &mut options, &mut filename),
            _ => {}
        }
    }

    if options.pm_init.is_none()
        && !options.pm_check
        && !options.pm_tree
        && !options.lsp_diag
        && options.lsp_complete.is_none()
        && options.lsp_hover.is_none()
        && options.lsp_definition.is_none()
        && options.lsp_references.is_none()
        && !options.mobile_export
        && !options.llvm_compile
        && !options.llvm_handled
        && !options.jit_run
        && !options.jit_repl
        && !options.jit_info
        && !options.jit_bench
        && !options.jit_handled
    {
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-h" | "--help" => return CliAction::Help,
                "-v" | "--version" => return CliAction::Version,
                "-r" | "--repl" => run_repl_flag = true,
                "-c" | "--compile" => options.compile_only = true,
                "-f" | "--format" => options.format_only = true,
                "-t" | "--tokens" => options.show_tokens = true,
                "-l" | "--lint" => options.lint_only = true,
                "--lint-disable" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!(
                            "{}",
                            crate::rtl("الخيار --lint-disable يحتاج كود قاعدة مثل L001")
                                .bright_red()
                        );
                        process::exit(1);
                    }
                    options.lint_disabled_rules.push(args[i].to_uppercase());
                }
                "--lint-max" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!(
                            "{}",
                            crate::rtl("الخيار --lint-max يحتاج رقماً صحيحاً").bright_red()
                        );
                        process::exit(1);
                    }
                    options.lint_max = Some(parse_positive_usize(&args[i], "--lint-max"));
                }
                "-a" | "--ast" => options.show_ast = true,
                "-d" | "--debug" => options.debug = true,
                "--pm-init" => {
                    i += 1;
                    if i >= args.len() {
                        eprintln!(
                            "{}",
                            crate::rtl("الخيار --pm-init يحتاج اسم مشروع").bright_red()
                        );
                        process::exit(1);
                    }
                    options.pm_init = Some(args[i].clone());
                }
                "--pm-check" => options.pm_check = true,
                "--pm-tree" => options.pm_tree = true,
                "--lsp-diag" => options.lsp_diag = true,
                arg if arg.starts_with('-') => {
                    eprintln!(
                        "{}",
                        crate::rtl(&format!("خيار غير معروف: {}", arg)).bright_red()
                    );
                    process::exit(1);
                }
                _ => {
                    if filename.is_none() {
                        filename = Some(args[i].clone());
                    } else {
                        eprintln!("{}", crate::rtl("يمكن تحديد ملف واحد فقط").bright_red());
                        process::exit(1);
                    }
                }
            }
            i += 1;
        }
    }

    CliAction::Run(Box::new(ParsedCli {
        options,
        filename,
        run_repl_flag,
    }))
}

fn parse_pm_subcommand(args: &[String], options: &mut RunOptions) {
    match args.get(2).map(String::as_str) {
        Some("init") => {
            let name = args.get(3).cloned().unwrap_or_else(|| {
                eprintln!(
                    "{}",
                    crate::rtl("الأمر 'pm init' يحتاج اسم مشروع").bright_red()
                );
                process::exit(1);
            });
            options.pm_init = Some(name);
        }
        Some("check") => options.pm_check = true,
        Some("tree") => options.pm_tree = true,
        Some(other) => {
            eprintln!(
                "{}",
                crate::rtl(&format!("أمر pm غير معروف: {}", other)).bright_red()
            );
            process::exit(1);
        }
        None => {
            eprintln!(
                "{}",
                crate::rtl("استخدم: almarjaa pm [init|check|tree]").bright_red()
            );
            process::exit(1);
        }
    }
}

fn parse_lsp_subcommand(args: &[String], options: &mut RunOptions, filename: &mut Option<String>) {
    match args.get(2).map(String::as_str) {
        Some("diag") => {
            options.lsp_diag = true;
            *filename = args.get(3).cloned();
        }
        Some("complete") => {
            *filename = args.get(3).cloned();
            options.lsp_complete = Some(args.get(4).cloned().unwrap_or_default());
        }
        Some("hover") => {
            *filename = args.get(3).cloned();
            let line = args
                .get(4)
                .map(|v| parse_positive_usize(v, "line"))
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        crate::rtl("الأمر 'lsp hover' يحتاج line و column").bright_red()
                    );
                    process::exit(1);
                });
            let column = args
                .get(5)
                .map(|v| parse_positive_usize(v, "column"))
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        crate::rtl("الأمر 'lsp hover' يحتاج line و column").bright_red()
                    );
                    process::exit(1);
                });
            options.lsp_hover = Some((line, column));
        }
        Some("definition") => {
            *filename = args.get(3).cloned();
            let line = args
                .get(4)
                .map(|v| parse_positive_usize(v, "line"))
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        crate::rtl("الأمر 'lsp definition' يحتاج line و column").bright_red()
                    );
                    process::exit(1);
                });
            let column = args
                .get(5)
                .map(|v| parse_positive_usize(v, "column"))
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        crate::rtl("الأمر 'lsp definition' يحتاج line و column").bright_red()
                    );
                    process::exit(1);
                });
            options.lsp_definition = Some((line, column));
        }
        Some("references") => {
            *filename = args.get(3).cloned();
            let line = args
                .get(4)
                .map(|v| parse_positive_usize(v, "line"))
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        crate::rtl("الأمر 'lsp references' يحتاج line و column").bright_red()
                    );
                    process::exit(1);
                });
            let column = args
                .get(5)
                .map(|v| parse_positive_usize(v, "column"))
                .unwrap_or_else(|| {
                    eprintln!(
                        "{}",
                        crate::rtl("الأمر 'lsp references' يحتاج line و column").bright_red()
                    );
                    process::exit(1);
                });
            options.lsp_references = Some((line, column));
        }
        Some(other) => {
            eprintln!(
                "{}",
                crate::rtl(&format!("أمر lsp غير معروف: {}", other)).bright_red()
            );
            process::exit(1);
        }
        None => {
            eprintln!(
                "{}",
                crate::rtl("استخدم: almarjaa lsp [diag|complete|hover|definition|references]")
                    .bright_red()
            );
            process::exit(1);
        }
    }
}

fn parse_mobile_subcommand(
    args: &[String],
    options: &mut RunOptions,
    filename: &mut Option<String>,
) {
    options.mobile_export = true;

    match args.get(2).map(String::as_str) {
        Some("export") => {
            // mobile export <file> --platform <platform> --framework <framework> --name <name>
            *filename = args.get(3).cloned();

            let mut i = 4;
            while i < args.len() {
                match args[i].as_str() {
                    "--platform" | "-p" => {
                        i += 1;
                        options.mobile_platform = args.get(i).cloned();
                    }
                    "--framework" | "-f" => {
                        i += 1;
                        options.mobile_framework = args.get(i).cloned();
                    }
                    "--name" | "-n" => {
                        i += 1;
                        options.mobile_project_name = args.get(i).cloned();
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        Some("list") => {
            // عرض المنصات والأطر المتاحة
            println!("{}", crate::rtl("المنصات المتاحة:").bright_cyan());
            println!("  - أندرويد (android)");
            println!("  - آيفون (ios)");
            println!("  - كلاهما (both)");
            println!();
            println!("{}", crate::rtl("الأطر المتاحة:").bright_cyan());
            println!("  - فلاتر (flutter) - موصى به");
            println!("  - React Native (react-native)");
            println!("  - أصلي (native)");
            println!("  - Capacitor (capacitor)");
        }
        Some(other) => {
            eprintln!(
                "{}",
                crate::rtl(&format!("أمر mobile غير معروف: {}", other)).bright_red()
            );
            process::exit(1);
        }
        None => {
            eprintln!(
                "{}",
                crate::rtl("استخدم: almarjaa mobile [export|list]").bright_red()
            );
            process::exit(1);
        }
    }
}

fn parse_positive_usize(value: &str, option_name: &str) -> usize {
    match value.parse::<usize>() {
        Ok(v) if v > 0 => v,
        _ => {
            eprintln!(
                "{}",
                crate::rtl(&format!(
                    "قيمة {} يجب أن تكون رقماً صحيحاً موجباً",
                    option_name
                ))
                .bright_red()
            );
            process::exit(1);
        }
    }
}

fn parse_llvm_subcommand(args: &[String], options: &mut RunOptions, filename: &mut Option<String>) {
    match args.get(2).map(String::as_str) {
        Some("compile") => {
            options.llvm_compile = true;
            // llvm_compile سيتم تعيينه فقط للأوامر التي تحتاجه
            // llvm compile <file> [options]
            *filename = args.get(3).cloned();

            let mut i = 4;
            while i < args.len() {
                match args[i].as_str() {
                    "--opt" | "-O" => {
                        i += 1;
                        if let Some(level_str) = args.get(i) {
                            options.llvm_opt_level = level_str.parse::<u8>().ok();
                        }
                    }
                    "--target" | "-t" => {
                        i += 1;
                        options.llvm_target = args.get(i).cloned();
                    }
                    "--output" | "-o" => {
                        i += 1;
                        options.llvm_output = args.get(i).cloned();
                    }
                    "--emit-ir" => {
                        options.llvm_emit_ir = true;
                    }
                    "--emit-bc" => {
                        options.llvm_emit_bc = true;
                    }
                    "--object" | "-c" => {
                        options.llvm_object_only = true;
                    }
                    "--run" | "-r" => {
                        options.llvm_run = true;
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        Some("targets") => {
            options.llvm_handled = true;
            // عرض الأهداف المتاحة - لا يحتاج llvm_compile
            println!("{}", crate::rtl("الأهداف المتاحة:").bright_cyan());
            println!("  - x86_64-unknown-linux-gnu   (Linux x64)");
            println!("  - x86_64-pc-windows-msvc    (Windows x64)");
            println!("  - aarch64-apple-darwin      (macOS ARM64)");
            println!("  - x86_64-apple-darwin       (macOS Intel)");
            println!("  - wasm32-unknown-unknown    (WebAssembly)");
            println!("  - aarch64-linux-android     (Android ARM64)");
            println!("  - aarch64-apple-ios         (iOS ARM64)");
        }
        Some("opt") | Some("optimize") => {
            options.llvm_handled = true;
            // عرض مستويات التحسين - لا يحتاج llvm_compile
            println!("{}", crate::rtl("مستويات التحسين:").bright_cyan());
            println!("  -O0  بدون تحسين (للتصحيح)");
            println!("  -O1  تحسين أساسي");
            println!("  -O2  تحسين متوسط (افتراضي)");
            println!("  -O3  تحسين أقصى");
            println!("  -Os  تحسين للحجم");
            println!("  -Oz  تحسين أقصى للحجم");
        }
        Some("run") => {
            options.llvm_compile = true;
            options.llvm_run = true;
            // تشغيل ملف مترجم
            *filename = args.get(3).cloned();
        }
        Some(other) => {
            eprintln!(
                "{}",
                crate::rtl(&format!("أمر llvm غير معروف: {}", other)).bright_red()
            );
            process::exit(1);
        }
        None => {
            eprintln!(
                "{}",
                crate::rtl("استخدم: almarjaa llvm [compile|targets|opt|run]").bright_red()
            );
            process::exit(1);
        }
    }
}

fn parse_jit_subcommand(args: &[String], options: &mut RunOptions, filename: &mut Option<String>) {
    match args.get(2).map(String::as_str) {
        Some("run") => {
            options.jit_run = true;
            *filename = args.get(3).cloned();

            let mut i = 4;
            while i < args.len() {
                match args[i].as_str() {
                    "--tiered" | "-t" => {
                        options.jit_tiered = true;
                    }
                    "--benchmark" | "-b" => {
                        options.jit_benchmark = true;
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        Some("repl") => {
            options.jit_repl = true;
        }
        Some("info") => {
            options.jit_info = true;
        }
        Some("benchmark") | Some("test") => {
            options.jit_bench = true;
        }
        Some(other) => {
            eprintln!(
                "{}",
                crate::rtl(&format!("أمر jit غير معروف: {}", other)).bright_red()
            );
            process::exit(1);
        }
        None => {
            eprintln!(
                "{}",
                crate::rtl("استخدم: almarjaa jit [run|repl|info|benchmark]").bright_red()
            );
            process::exit(1);
        }
    }
}
