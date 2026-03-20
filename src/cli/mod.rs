use std::env;

pub mod args;
pub mod commands;
pub mod repl;

pub fn run_from_env(version: &str) {
    let args: Vec<String> = env::args().collect();
    run(&args, version);
}

pub fn run(args: &[String], version: &str) {
    match args::parse_args(args) {
        args::CliAction::Help => commands::print_help(),
        args::CliAction::Version => commands::print_version(version),
        args::CliAction::Run(parsed) => {
            let parsed = *parsed;

            // معالجة أمر JIT Info
            if parsed.options.jit_info {
                commands::handle_jit_info();
                return;
            }

            // معالجة أمر JIT Benchmark
            if parsed.options.jit_bench {
                commands::handle_jit_benchmark();
                return;
            }

            // معالجة أوامر JIT Compiler
            if commands::handle_jit_command(&parsed.options, parsed.filename.as_deref()) {
                return;
            }

            // تشغيل ملف أو REPL
            if let Some(fname) = parsed.filename {
                commands::run_file(&fname, &parsed.options);
            } else if parsed.run_repl_flag || args.len() == 1 {
                repl::run_repl();
            }
        }
    }
}
