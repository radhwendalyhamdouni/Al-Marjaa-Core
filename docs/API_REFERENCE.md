# API Reference - المرجع البرمجي

## الإصدار: 3.4.1

## نظرة عامة

لغة المرجع توفر API مستقرة وموثقة للمطورين. هذا المستند يشرح حدود الواجهة وكيفية استخدامها.

---

## 🏗️ هيكل المشروع

```
src/
├── core/           # المكونات الأساسية (مستقر)
│   ├── lexer/      # محلل معجمي
│   ├── parser/     # محلل نحوي
│   ├── interpreter/# المفسر
│   ├── bytecode/   # نظام البايت كود و JIT
│   └── error/      # معالجة الأخطاء
│
├── libs/           # المكتبات والتوسعات
│   ├── stdlib/     # المكتبة القياسية
│   ├── modules/    # نظام الوحدات
│   └── cranelift/  # مترجم JIT (اختياري)
│
└── cli/            # واجهة سطر الأوامر
```

---

## 📦 Feature Flags

| Flag | الوصف | التبعيات |
|------|-------|---------|
| `default` | المكونات الأساسية فقط | لا شيء |
| `cranelift-backend` | مترجم Cranelift JIT | ~60s compile time |
| `network` | دعم الشبكات و HTTP | reqwest, tokio |
| `database` | دعم SQLite | rusqlite |
| `crypto` | دوال التشفير المتقدمة | aes-gcm, rsa, ... |
| `full-stdlib` | المكتبة القياسية الكاملة | network + database + crypto |
| `full` | كل شيء | جميع التبعيات |
| `bench-heavy` | اختبارات الأداء الثقيلة | criterion |

### مثال الاستخدام

```toml
# Cargo.toml

# أساسي فقط (سريع)
[dependencies]
almarjaa = "3.4"

# مع JIT
[dependencies]
almarjaa = { version = "3.4", features = ["cranelift-backend"] }

# كل شيء
[dependencies]
almarjaa = { version = "3.4", features = ["full"] }
```

---

## 🔒 Core API (مستقر)

> **ملاحظة**: Core API مضمون للتوافقية. لا يتغير إلا في إصدارات رئيسية.

### Lexer - المحلل المعجمي

```rust
use almarjaa::Lexer;

let source = r#"متغير س = 10؛"#;
let mut lexer = Lexer::new(source);
let tokens = lexer.tokenize()?;
```

### Parser - المحلل النحوي

```rust
use almarjaa::Parser;

let source = r#"متغير س = 10؛"#;
let ast = Parser::parse(source)?;
```

### Interpreter - المفسر

```rust
use almarjaa::Interpreter;

let source = r#"
    متغير ترحيب = "مرحباً بالعالم"؛
    اطبع(ترحيب)؛
"#;

let mut interp = Interpreter::new();
interp.run(source)?;
```

### Bytecode Compiler

```rust
use almarjaa::{Compiler, VM, Chunk};

let source = r#"
    متغير س = 0؛
    طالما س < 10 {
        س = س + 1؛
    }
"#;

let chunk = Compiler::compile_source(source)?;
let mut vm = VM::new();
vm.load(chunk);
let result = vm.run();
```

---

## 🔓 Extended API (قد يتغير)

> **تحذير**: Extended API قد يتغير بين الإصدارات الصغرى.

### المكتبة القياسية

```rust
use almarjaa::libs::stdlib;

// دوال التشفير الأساسية
use almarjaa::libs::stdlib::crypto::{md5, sha256};

let hash = sha256("مرحبا");
```

### نظام الوحدات

```rust
use almarjaa::libs::modules::{ModuleManager, PackageManager};

let mut manager = ModuleManager::new();
manager.load_module("رياضيات")?;
```

---

## 🎯 أفضل الممارسات

### 1. استخدم Core API للمشاريع الإنتاجية

```rust
// ✅ جيد - Core API مستقر
use almarjaa::{Lexer, Parser, Interpreter};

// ⚠️ حذر - Extended API قد يتغير
use almarjaa::libs::stdlib::crypto::experimental::*;
```

### 2. اختر Feature Flags بحكمة

```toml
# للخدمات الصغيرة - أساسي فقط
almarjaa = "3.4"

# للتطبيقات المكتبية - مع JIT
almarjaa = { version = "3.4", features = ["cranelift-backend"] }

# للخوادم - الشبكة + قواعد البيانات
almarjaa = { version = "3.4", features = ["network", "database"] }
```

### 3. تجنب التبعيات الثقيلة في CI

```bash
# بدلاً من
cargo build --all-features

# استخدم
cargo build  # الافتراضي خفيف
cargo build --features "network"  # فقط ما تحتاجه
```

---

## 📚 المكتبات الخارجية

المكتبات الخارجية متاحة في:
https://github.com/radhwendalyhamdouni/Al-Marjaa-Libraries

| المكتبة | الوصف | الحالة |
|---------|-------|--------|
| `almarjaa-web` | إطار ويب | قيد التطوير |
| `almarjaa-gui` | واجهات رسومية | قيد التطوير |
| `almarjaa-data` | معالجة البيانات | قيد التطوير |
| `almarjaa-ml` | تعلم آلي | مخطط |

---

## 🔧 التهيئة

### متغيرات البيئة

| المتغير | الوصف | الافتراضي |
|---------|-------|-----------|
| `ALMARJAA_MAX_LOOP_ITERATIONS` | حد الحلقات | 10,000,000 |
| `ALMARJAA_STACK_SIZE` | حجم المكدس | 65,536 |
| `ALMARJAA_CACHE_SIZE` | حجم كاش JIT | 256 |

### مثال

```bash
export ALMARJAA_MAX_LOOP_ITERATIONS=1000000
almarjaa run program.mrj
```

---

## 📝 التوافقية

| الإصدار | Core API | Extended API |
|---------|----------|--------------|
| 3.4.x | ✅ مستقر | ⚠️ قد يتغير |
| 4.0.x | ⚠️ قد يتغير | ⚠️ قد يتغير |

---

## 📞 الدعم

- **GitHub**: https://github.com/radhwendalyhamdouni/Al-Marjaa-Core
- **Issues**: https://github.com/radhwendalyhamdouni/Al-Marjaa-Core/issues
- **Email**: almarjaa.project@hotmail.com
