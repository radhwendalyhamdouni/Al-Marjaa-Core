# دليل تكامل Al-Marjaa-Core مع المكتبات

## 📊 نظرة عامة

Al-Marjaa مقسم إلى مستودعين:
- **Al-Marjaa-Core**: النواة الأساسية (مفسر، JIT، مكتبة قياسية أساسية)
- **Al-Marjaa-Libraries**: مكتبات متخصصة (قواعد بيانات، شبكات، ذكاء اصطناعي، صناعة)

## 🔧 طرق التكامل

### الطريقة 1: استخدام Cargo Workspace

```toml
# Cargo.toml في مشروعك
[workspace]
members = [
    "almarjaa-core",
    "almarjaa-libraries/libs/database",
    "almarjaa-libraries/libs/network",
    "almarjaa-libraries/libs/ai",
]

[workspace.dependencies]
almarjaa = { path = "almarjaa-core" }
almarjaa-database = { path = "almarjaa-libraries/libs/database", features = ["full"] }
almarjaa-network = { path = "almarjaa-libraries/libs/network", features = ["full"] }
```

### الطريقة 2: استخدام crates.io (قريباً)

```toml
[dependencies]
almarjaa = "3.4"
almarjaa-database = { version = "3.4", features = ["sqlite"] }
almarjaa-network = { version = "3.4", features = ["client"] }
```

### الطريقة 3: استخدام git مباشرة

```toml
[dependencies]
almarjaa = { git = "https://github.com/radhwendalyhamdouni/Al-Marjaa-Core" }
almarjaa-database = { git = "https://github.com/radhwendalyhamdouni/Al-Marjaa-Libraries" }
```

## 📦 المكتبات المتاحة

### 1. مكتبة قواعد البيانات (almarjaa-database)

```rust
use almarjaa_database::{Database, Connection};

// SQLite
let db = Database::sqlite("app.db")?;
db.execute("CREATE TABLE users (id INTEGER, name TEXT)")?;
db.execute("INSERT INTO users VALUES (1, 'أحمد')")?;

// Query
let results = db.query("SELECT * FROM users")?;
```

**الميزات المتاحة:**
- `sqlite` - دعم SQLite
- `async-db` - دعم قواعد بيانات غير متزامنة
- `full` - جميع الميزات

### 2. مكتبة الشبكات (almarjaa-network)

```rust
use almarjaa_network::{HttpClient, Server};

// HTTP Client
let client = HttpClient::new();
let response = client.get("https://api.example.com/data").send()?;

// HTTP Server
let server = Server::new()
    .route("/api", |req| Response::json({"status": "ok"}))
    .listen("0.0.0.0:8080")?;
```

**الميزات المتاحة:**
- `client` - عميل HTTP
- `server` - خادم HTTP
- `full` - جميع الميزات

### 3. مكتبة الذكاء الاصطناعي (almarjaa-ai)

```rust
use almarjaa_ai::{Model, Inference};

// تحميل نموذج ONNX
let model = Model::load("model.onnx")?;
let output = model.inference(&input)?;

// الاستدلال مع LLM
let llm = Inference::local("model.gguf")?;
let response = llm.generate("مرحباً، كيف حالك؟")?;
```

### 4. مكتبة التطبيقات الصناعية (almarjaa-industrial)

```rust
use almarjaa_industrial::{Modbus, Scada};

// Modbus TCP
let modbus = Modbus::tcp("192.168.1.100:502")?;
let value = modbus.read_holding_register(0, 10)?;

// SCADA
let scada = Scada::new()
    .add_device("plc1", DeviceType::Modbus)
    .monitor()
    .alarms()
    .hmi();
```

## 🌟 مثال متكامل

```rust
// main.rs
use almarjaa::{Interpreter, Value};
use almarjaa_database::Database;
use almarjaa_network::HttpClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // إنشاء مفسر
    let mut interpreter = Interpreter::new();
    
    // تسجيل دوال المكتبات
    register_database_functions(&mut interpreter);
    register_network_functions(&mut interpreter);
    
    // تنفيذ كود المرجع
    interpreter.run(r#"
        # استخدام قاعدة البيانات
        متغير db = قاعدة_بيانات("app.db")؛
        db.نفذ("CREATE TABLE users (id INTEGER, name TEXT)")؛
        
        # استخدام الشبكة
        متغير عميل = عميل_شبكة()؛
        متغير رد = عميل.احصل("https://api.example.com/data")؛
        
        اطبع(رد)؛
    "#)?;
    
    Ok(())
}
```

## 📁 بنية الملفات المقترحة

```
project/
├── Cargo.toml
├── src/
│   └── main.rs
├── almarjaa-core/          # النواة (اختياري)
└── almarjaa-libraries/     # المكتبات (اختياري)
    └── libs/
        ├── database/
        ├── network/
        ├── ai/
        └── industrial/
```

## ⚙️ البناء مع ميزات محددة

```bash
# بناء النواة فقط
cargo build --release

# بناء مع دعم قواعد البيانات
cargo build --release --features database

# بناء كامل مع جميع المكتبات
cargo build --release --features full
```

## 🔄 التكامل مع اللغة

### في كود المرجع:

```mrj
# استيراد مكتبة
استيراد "قواعد_البيانات" كـ ديبي؛

# استخدام الدوال
متغير db = ديبي.اتصل("app.db")؛
متغير نتائج = db.استعلام("SELECT * FROM users")؛

لكل صف في نتائج {
    اطبع(صف["name"])؛
}
```

## 📝 ملاحظات مهمة

1. **الإصدار**: تأكد من تطابق الإصدارات (كلاهما 3.4.0)
2. **الميزات**: اختر فقط الميزات التي تحتاجها لتقليل حجم البناء
3. **الترابط**: المكتبات مستقلة، يمكنك استخدام أي منها منفردة
4. **الأداء**: JIT Compiler يعمل فقط مع النواة، المكتبات تعمل كمكملات

## 🚀 الخطوة التالية

لدمج المكتبات في النواة، يمكن:
1. إضافتها كـ optional dependencies في Cargo.toml
2. إنشاء نظام plugins للتحميل الديناميكي
3. بناء نسخة موسعة مع جميع المكتبات مدمجة
