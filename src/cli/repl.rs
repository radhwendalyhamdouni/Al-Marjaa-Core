use std::thread;
use std::time::Duration;

use colored::Colorize;

use almarjaa::interpreter::value::Value;
use almarjaa::interpreter::Interpreter;

pub fn run_repl() {
    print_banner();
    print_majestic_terminal_art();
    print_legendary_intro();
    println!();
    println!(
        "{}",
        crate::rtl("  ▸ اكتب 'مساعدة' للمساعدة | 'ذكاء' للذكاء الاصطناعي | 'خروج' للإنهاء")
            .bright_green()
    );
    println!();

    let mut interpreter = Interpreter::new();

    loop {
        print!("{} ", crate::rtl("المرجع>>").bright_blue());

        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                match input {
                    "خروج" | "exit" | "quit" => {
                        print_legendary_outro();
                        println!("{}", crate::rtl("مع السلامة! 👋").bright_green());
                        break;
                    }
                    "مساعدة" | "help" => {
                        print_repl_help();
                        continue;
                    }
                    "ذكاء" | "ai" => {
                        print_ai_quick_help();
                        continue;
                    }
                    "خطة_تدريب_خفيفة" => {
                        print_low_resource_training_plan();
                        continue;
                    }
                    "مسح" | "clear" => {
                        print!("\x1B[2J\x1B[1;1H");
                        continue;
                    }
                    _ => {}
                }

                if let Some(name) = input.strip_prefix("أنشئ_وكيل ") {
                    let agent_name = name.trim();
                    if agent_name.is_empty() {
                        eprintln!(
                            "{}",
                            crate::rtl("الرجاء كتابة اسم للوكيل بعد الأمر").bright_red()
                        );
                    } else {
                        print_agent_scaffold(agent_name);
                    }
                    continue;
                }

                match interpreter.run(input) {
                    Ok(result) => {
                        let value: std::cell::Ref<Value> = result.borrow();
                        if !matches!(*value, Value::Null) {
                            println!("{}", crate::rtl(&format!("=> {}", value)).bright_yellow());
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "{}",
                            crate::rtl(&format!("خطأ: {}", e.message)).bright_red()
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!("خطأ في القراءة: {}", e)).bright_red()
                );
                break;
            }
        }
    }
}

fn print_banner() {
    // Clear screen and show professional welcome
    print!("\x1B[2J\x1B[1;1H");

    println!(
        "{}",
        "
    ╔═══════════════════════════════════════════════════════════════════════════════╗
    ║                                                                               ║
    ║     ░█████╗░░█████╗░░█████╗░░█████╗░░█████╗░░█████╗░░█████╗░░█████╗░        ║
    ║     ╚════╝░╚════╝░╚════╝░╚════╝░╚════╝░╚════╝░╚════╝░╚════╝░╚════╝░        ║
    ║                                                                               ║
    ║              🌙  لُغــةُ الـمَــرْجَــع  🌙                                    ║
    ║            ══════════════════════════                                        ║
    ║              A L - M A R J A A   L A N G U A G E                             ║
    ║                                                                               ║
    ║     ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ ║
    ║                                                                               ║
    ║               🏆 أول لغة برمجة عربية متكاملة                                  ║
    ║              The First Complete Arabic Programming Language                   ║
    ║                                                                               ║
    ║     ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ ║
    ║                                                                               ║
    ║                      ⚡ الإصدار 3.3.0 | Version 3.3.0 ⚡                      ║
    ║                                                                               ║
    ║     ──────────────────────────────────────────────────────────────────────── ║
    ║                                                                               ║
    ║            🎓 المخترع والمؤسس والحائز الأصلي:                                 ║
    ║                                                                               ║
    ║                    رضوان دالي حمدوني                                          ║
    ║                  RADHWEN DALY HAMDOUNI                                        ║
    ║                                                                               ║
    ║     ──────────────────────────────────────────────────────────────────────── ║
    ║                                                                               ║
    ║         \"نَكْتُبُ المُسْتَقْبَلَ بِثِقَةٍ... وَنُنَفِّذُ بِهَيْبَةٍ\"                      ║
    ║                                                                               ║
    ║          \"We Write the Future with Confidence... Execute with Pride\"         ║
    ║                                                                               ║
    ╚═══════════════════════════════════════════════════════════════════════════════╝
    "
        .bright_cyan()
        .bold()
    );
}

fn print_majestic_terminal_art() {
    let crest = [
        "         ╭──────────────────────────────────────────────────╮         ",
        "         │  ✦  عَرْشُ لُغَةِ المَرْجَع  ✦  │         ",
        "         │       THRONE OF AL-MARJAA LANGUAGE               │         ",
        "         ╰──────────────────────────────────────────────────╯         ",
        "                        ⬡     ⬡     ⬡                           ",
        "                     ╭───────────────────╮                        ",
        "                     │  🏛️  المرجع  🏛️  │                        ",
        "                     │    Al-Marjaa      │                        ",
        "                     ╰───────────────────╯                        ",
        "           ╭────────────────────────────────────────╮              ",
        "           │  👑 المخترع: رضوان دالي حمدوني 👑      │              ",
        "           │  Inventor: Radhwen Daly Hamdouni       │              ",
        "           ╰────────────────────────────────────────╯              ",
    ];

    for line in crest {
        typewriter(&crate::rtl(line), |s| s.bright_yellow().bold());
    }

    println!();
    for pulse in [
        "⚡ جاهز للتنفيذ ⚡",
        "⚡ Ready to Execute ⚡",
        "⚡ هَيْبَةُ الكَوْدِ العَرَبِي ⚡",
    ] {
        println!(
            "{}",
            crate::rtl(&format!("    ▸ {} ◂", pulse))
                .bright_magenta()
                .bold()
        );
        thread::sleep(Duration::from_millis(200));
    }
    println!();
}

fn typewriter(text: &str, color: fn(&str) -> colored::ColoredString) {
    for ch in text.chars() {
        print!("{}", color(&ch.to_string()));
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(12));
    }
    println!();
}

fn print_legendary_intro() {
    println!();
    typewriter(
        &crate::rtl("◆ مرحباً بك في لغة المرجع - أول لغة برمجة عربية متكاملة"),
        |s| s.bright_magenta().bold(),
    );
    typewriter(
        &crate::rtl("◆ Version 3.3.0 | JIT Compiler | ONNX Support | AI Integration"),
        |s| s.bright_cyan(),
    );
    typewriter(
        &crate::rtl("◆ المخترع والمؤسس: رضوان دالي حمدوني | Inventor & Founder"),
        |s| s.bright_green(),
    );
    println!();
}

fn print_legendary_outro() {
    println!();
    typewriter(
        &crate::rtl("◆ شكراً لاستخدام لغة المرجع"),
        |s| s.bright_magenta().bold(),
    );
    typewriter(
        &crate::rtl("◆ إلى اللقاء أيها المبرمج العظيم - نلتقي في المستقبل"),
        |s| s.bright_green(),
    );
    typewriter(
        &crate::rtl("◆ المخترع: رضوان دالي حمدوني | almarjaa.project@hotmail.com"),
        |s| s.bright_cyan(),
    );
    println!();
}

fn print_ai_quick_help() {
    println!(
        "{}",
        crate::rtl("\nأوامر الذكاء الاصطناعي السريعة:").bright_magenta()
    );
    println!("  ذكاء                      عرض لوحة الأوامر الذكية باللغة العربية");
    println!("  خطة_تدريب_خفيفة          خطة حديثة لتدريب/تكييف نموذج بأقل رام وهاردوير");
    println!("  أنشئ_وكيل <اسم>          توليد قالب وكيل ذكي عربي قابل للتعديل");
}

fn print_low_resource_training_plan() {
    println!(
        "{}",
        crate::rtl("\nخارطة تدريب نموذج ذكاء اصطناعي بكفاءة عالية:").bright_cyan()
    );
    println!("  ١) اختر نموذجاً أساسياً صغيراً (3B-8B) مع دعم quantization.");
    println!("  ٢) استخدم QLoRA (4-bit NF4) لتقليل استهلاك RAM/VRAM.");
    println!("  ٣) فعّل gradient checkpointing و mixed precision (bf16/fp16). ");
    println!("  ٤) اضبط micro-batch صغير مع gradient accumulation.");
    println!("  ٥) ابدأ بـ SFT على بيانات عربية نظيفة ثم DPO/ORPO لتحسين الجودة.");
    println!("  ٦) قيّم على benchmarks عربية + حالات استخدام واقعية قبل الإطلاق.");
    println!(
        "{}",
        crate::rtl(
            "نصيحة: استهدف fine-tuning بدلاً من التدريب من الصفر للحصول على نتيجة رائعة بأقل تكلفة."
        )
        .bright_green()
    );
}

fn print_agent_scaffold(agent_name: &str) {
    println!(
        "{}",
        crate::rtl(&format!("\nقالب وكيل ذكي جاهز: {}", agent_name)).bright_cyan()
    );
    println!("  المتطلبات: هدف واضح + أدوات + ذاكرة سياقية");
    println!("  الدور: مساعد عربي خبير");
    println!("  الهدف: تنفيذ المهام بأوامر عربية بسيطة");
    println!("  الأدوات: [بحث، قراءة_ملف، تنفيذ_أمر]");
    println!("  النمط: دقيق، مختصر، مهني");
    println!("  الحماية: منع الأوامر الخطرة + توثيق كل خطوة");
    println!("  ----------------------------------------------");
    println!(
        "{}",
        crate::rtl("يمكنك نسخ القالب وتخصيصه فوراً لسيناريوك.").bright_green()
    );
}

fn print_repl_help() {
    println!("{}", crate::rtl("\nأوامر REPL:").bright_yellow());
    println!("  خروج, exit, quit    الخروج من البرنامج");
    println!("  مساعدة, help        عرض هذه المساعدة");
    println!("  ذكاء, ai            لوحة الأوامر الذكية");
    println!("  خطة_تدريب_خفيفة    وصفة تدريب حديثة قليلة الموارد");
    println!("  أنشئ_وكيل <اسم>    إنشاء قالب وكيل ذكي عربي");
    println!("  مسح, clear          مسح الشاشة");
    println!();
    println!("{}", crate::rtl("أمثلة على الكود:").bright_yellow());
    println!("  متغير س = ١٠؛");
    println!("  اطبع(س + ٥)؛");
    println!("  دالة جمع(أ، ب) {{ أرجع أ + ب؛ }}");
    println!("  إذا س > ٥ {{ اطبع(\"كبير\")؛ }}");
    println!();
}
