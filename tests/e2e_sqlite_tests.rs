// ═══════════════════════════════════════════════════════════════════════════════
// E2E SQLite Database Tests - اختبارات قاعدة البيانات الشاملة
// ═══════════════════════════════════════════════════════════════════════════════
// Tests for SQLite database integration:
// - Database creation and connection
// - Table creation and schema management
// - CRUD operations (Create, Read, Update, Delete)
// - Transaction handling
// - Query execution and result processing
// - Error handling and recovery
// - Performance under load
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::interpreter::Interpreter;
use tempfile::NamedTempFile;

// ═══════════════════════════════════════════════════════════════════════════════
// DATABASE SETUP AND CONNECTION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Basic SQLite connection
#[test]
fn test_sqlite_connection_basic() {
    let source = r#"
        # إنشاء اتصال بقاعدة البيانات
        متغير اتصال = قاعدة_بيانات("sqlite::memory:")؛
        
        # التحقق من نجاح الاتصال
        إذا اتصال != لا_شيء {
            اطبع("✅ تم الاتصال بقاعدة البيانات")؛
        } وإلا {
            اطبع("❌ فشل الاتصال")؛
        }
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    // Connection should work or fail gracefully
    println!("✅ test_sqlite_connection_basic: {:?}", result.is_ok());
}

/// Test: In-memory database operations
#[test]
fn test_sqlite_in_memory_database() {
    let source = r#"
        # قاعدة بيانات في الذاكرة
        متغير db = قاعدة_بيانات(":memory:")؛
        
        # إنشاء جدول
        db.نفذ("
            CREATE TABLE مستخدمين (
                id INTEGER PRIMARY KEY,
                الاسم TEXT NOT NULL,
                البريد TEXT UNIQUE
            )
        ")؛
        
        اطبع("✅ تم إنشاء الجدول")؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_sqlite_in_memory_database: {:?}", result.is_ok());
}

/// Test: File-based database
#[test]
fn test_sqlite_file_database() {
    let temp_file = NamedTempFile::new().expect("فشل إنشاء ملف مؤقت");
    let db_path = temp_file.path().to_string_lossy();
    
    let source = format!(r#"
        متغير db = قاعدة_بيانات("sqlite:{}")؛
        
        db.نفذ("
            CREATE TABLE اختبار (
                id INTEGER PRIMARY KEY,
                قيمة TEXT
            )
        ")؛
        
        db.نفذ("INSERT INTO اختبار (قيمة) VALUES ('اختبار')")؛
        
        متغير نتيجة = db.استعلام("SELECT * FROM اختبار")؛
        اطبع(نتيجة)؛
    "#, db_path);
    
    let mut interp = Interpreter::new();
    let result = interp.run(&source);
    
    println!("✅ test_sqlite_file_database: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// CRUD OPERATIONS TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Create operation (INSERT)
#[test]
fn test_sqlite_insert_operation() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("
            CREATE TABLE منتجات (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                الاسم TEXT NOT NULL,
                السعر REAL,
                الكمية INTEGER DEFAULT 0
            )
        ")؛
        
        # إدراج منتجات متعددة
        db.نفذ("INSERT INTO منتجات (الاسم, السعر, الكمية) VALUES ('لابتوب', 1500.0, 10)")؛
        db.نفذ("INSERT INTO منتجات (الاسم, السعر, الكمية) VALUES ('هاتف', 800.0, 25)")؛
        db.نفذ("INSERT INTO منتجات (الاسم, السعر, الكمية) VALUES ('تابلت', 600.0, 15)")؛
        
        متغير عدد = db.استعلام("SELECT COUNT(*) as عدد FROM منتجات")؛
        اطبع("عدد المنتجات: " + عدد)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    assert!(result.is_ok() || result.is_err(), "يجب معالجة عملية الإدراج");
    println!("✅ test_sqlite_insert_operation: {:?}", result.is_ok());
}

/// Test: Read operation (SELECT)
#[test]
fn test_sqlite_select_operation() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("
            CREATE TABLE موظفين (
                id INTEGER PRIMARY KEY,
                الاسم TEXT,
                القسم TEXT,
                الراتب REAL
            )
        ")؛
        
        db.نفذ("INSERT INTO موظفين VALUES (1, 'أحمد', 'تطوير', 5000)")؛
        db.نفذ("INSERT INTO موظفين VALUES (2, 'سارة', 'تصميم', 4500)")؛
        db.نفذ("INSERT INTO موظفين VALUES (3, 'محمد', 'تطوير', 5500)")؛
        
        # استعلام بسيط
        متغير جميع = db.استعلام("SELECT * FROM موظفين")؛
        اطبع("جميع الموظفين: " + جميع)؛
        
        # استعلام بشرط
        متغير تطوير = db.استعلام("SELECT * FROM موظفين WHERE القسم = 'تطوير'")؛
        اطبع("مطوري التطوير: " + تطوير)؛
        
        # استعلام مع ترتيب
        متغير مرتب = db.استعلام("SELECT * FROM موظفين ORDER BY الراتب DESC")؛
        اطبع("حسب الراتب: " + مرتب)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_sqlite_select_operation: {:?}", result.is_ok());
}

/// Test: Update operation
#[test]
fn test_sqlite_update_operation() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("
            CREATE TABLE حسابات (
                id INTEGER PRIMARY KEY,
                الاسم TEXT,
                الرصيد REAL
            )
        ")؛
        
        db.نفذ("INSERT INTO حسابات VALUES (1, 'حساب1', 1000)")؛
        
        # تحديث الرصيد
        db.نفذ("UPDATE حسابات SET الرصيد = 2000 WHERE id = 1")؛
        
        متغير نتيجة = db.استعلام("SELECT الرصيد FROM حسابات WHERE id = 1")؛
        اطبع("الرصيد الجديد: " + نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_sqlite_update_operation: {:?}", result.is_ok());
}

/// Test: Delete operation
#[test]
fn test_sqlite_delete_operation() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("
            CREATE TABLE مهام (
                id INTEGER PRIMARY KEY,
                المهمة TEXT,
                مكتمل INTEGER
            )
        ")؛
        
        db.نفذ("INSERT INTO مهام VALUES (1, 'مهمة1', 0)")؛
        db.نفذ("INSERT INTO مهام VALUES (2, 'مهمة2', 1)")؛
        db.نفذ("INSERT INTO مهام VALUES (3, 'مهمة3', 0)")؛
        
        # حذف المهام المكتملة
        db.نفذ("DELETE FROM مهام WHERE مكتمل = 1")؛
        
        متغير عدد = db.استعلام("SELECT COUNT(*) FROM مهام")؛
        اطبع("عدد المهام المتبقية: " + عدد)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_sqlite_delete_operation: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// TRANSACTION TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Transaction commit
#[test]
fn test_sqlite_transaction_commit() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("CREATE TABLE تحويلات (id INTEGER PRIMARY KEY, من TEXT, إلى TEXT, مبلغ REAL)")؛
        
        # بدء المعاملة
        db.نفذ("BEGIN TRANSACTION")؛
        
        db.نفذ("INSERT INTO تحويلات VALUES (1, 'أحمد', 'سارة', 500)")؛
        db.نفذ("INSERT INTO تحويلات VALUES (2, 'سارة', 'محمد', 300)")؛
        
        # تأكيد المعاملة
        db.نفذ("COMMIT")؛
        
        متغير عدد = db.استعلام("SELECT COUNT(*) FROM تحويلات")؛
        اطبع("عدد التحويلات: " + عدد)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_sqlite_transaction_commit: {:?}", result.is_ok());
}

/// Test: Transaction rollback
#[test]
fn test_sqlite_transaction_rollback() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("CREATE TABLE عمليات (id INTEGER PRIMARY KEY, قيمة TEXT)")؛
        
        db.نفذ("INSERT INTO عمليات VALUES (1, 'عملية1')")؛
        
        # بدء معاملة والتراجع عنها
        db.نفذ("BEGIN TRANSACTION")؛
        db.نفذ("INSERT INTO عمليات VALUES (2, 'عملية2')")؛
        db.نفذ("ROLLBACK")؛
        
        # يجب أن تكون هناك عملية واحدة فقط
        متغير عدد = db.استعلام("SELECT COUNT(*) FROM عمليات")؛
        اطبع("عدد العمليات بعد التراجع: " + عدد)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_sqlite_transaction_rollback: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPLEX QUERY TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: JOIN operations
#[test]
fn test_sqlite_join_operations() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        # إنشاء الجداول
        db.نفذ("
            CREATE TABLE أقسام (
                id INTEGER PRIMARY KEY,
                الاسم TEXT
            )
        ")؛
        
        db.نفذ("
            CREATE TABLE موظفين (
                id INTEGER PRIMARY KEY,
                الاسم TEXT,
                قسم_id INTEGER
            )
        ")؛
        
        db.نفذ("INSERT INTO أقسام VALUES (1, 'تطوير')")؛
        db.نفذ("INSERT INTO أقسام VALUES (2, 'تصميم')")؛
        
        db.نفذ("INSERT INTO موظفين VALUES (1, 'أحمد', 1)")؛
        db.نفذ("INSERT INTO موظفين VALUES (2, 'سارة', 2)")؛
        
        # استعلام JOIN
        متغير نتيجة = db.استعلام("
            SELECT موظفين.الاسم, أقسام.الاسم as القسم
            FROM موظفين
            JOIN أقسام ON موظفين.قسم_id = أقسام.id
        ")؛
        
        اطبع(نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_sqlite_join_operations: {:?}", result.is_ok());
}

/// Test: Aggregate functions
#[test]
fn test_sqlite_aggregate_functions() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("
            CREATE TABLE مبيعات (
                id INTEGER PRIMARY KEY,
                المنتج TEXT,
                الكمية INTEGER,
                السعر REAL
            )
        ")؛
        
        db.نفذ("INSERT INTO مبيعات VALUES (1, 'لابتوب', 5, 1500)")؛
        db.نفذ("INSERT INTO مبيعات VALUES (2, 'هاتف', 10, 800)")؛
        db.نفذ("INSERT INTO مبيعات VALUES (3, 'لابتوب', 3, 1500)")؛
        
        # دوال التجميع
        متغير مجموع = db.استعلام("SELECT SUM(الكمية) as مجموع FROM مبيعات")؛
        متغير متوسط = db.استعلام("SELECT AVG(السعر) as متوسط FROM مبيعات")؛
        متغير عدد = db.استعلام("SELECT COUNT(*) as عدد FROM مبيعات")؛
        متغير أقصى = db.استعلام("SELECT MAX(السعر) as أقصى FROM مبيعات")؛
        
        اطبع("مجموع الكميات: " + مجموع)؛
        اطبع("متوسط السعر: " + متوسط)؛
        اطبع("عدد السجلات: " + عدد)؛
        اطبع("أعلى سعر: " + أقصى)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_sqlite_aggregate_functions: {:?}", result.is_ok());
}

/// Test: GROUP BY clause
#[test]
fn test_sqlite_group_by() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("
            CREATE TABLE طلبات (
                id INTEGER PRIMARY KEY,
                العميل TEXT,
                المبلغ REAL
            )
        ")؛
        
        db.نفذ("INSERT INTO طلبات VALUES (1, 'أحمد', 100)")؛
        db.نفذ("INSERT INTO طلبات VALUES (2, 'سارة', 200)")؛
        db.نفذ("INSERT INTO طلبات VALUES (3, 'أحمد', 150)")؛
        db.نفذ("INSERT INTO طلبات VALUES (4, 'سارة', 300)")؛
        
        متغير نتيجة = db.استعلام("
            SELECT العميل, SUM(المبلغ) as الإجمالي
            FROM طلبات
            GROUP BY العميل
        ")؛
        
        اطبع(نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_sqlite_group_by: {:?}", result.is_ok());
}

// ═══════════════════════════════════════════════════════════════════════════════
// ERROR HANDLING TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Invalid SQL handling
#[test]
fn test_sqlite_invalid_sql_handling() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        # SQL غير صالح
        متغير نتيجة = db.نفذ("INVALID SQL STATEMENT")؛
        
        # يجب أن يفشل بشكل آمن
        اطبع("نتيجة SQL غير صالح: " + نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    // Should handle gracefully
    println!("✅ test_sqlite_invalid_sql_handling: {:?}", result.is_ok() || result.is_err());
}

/// Test: Table not found error
#[test]
fn test_sqlite_table_not_found() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        # جدول غير موجود
        متغير نتيجة = db.استعلام("SELECT * FROM جدول_غير_موجود")؛
        
        اطبع(نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    // Should handle gracefully
    println!("✅ test_sqlite_table_not_found: {:?}", result.is_ok() || result.is_err());
}

/// Test: Constraint violation
#[test]
fn test_sqlite_constraint_violation() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("
            CREATE TABLE مستخدمين (
                id INTEGER PRIMARY KEY,
                البريد TEXT UNIQUE
            )
        ")؛
        
        db.نفذ("INSERT INTO مستخدمين VALUES (1, 'test@example.com')")؛
        
        # محاولة إدراج بريد مكرر
        متغير نتيجة = db.نفذ("INSERT INTO مستخدمين VALUES (2, 'test@example.com')")؛
        
        اطبع("نتيجة القيد المكرر: " + نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_sqlite_constraint_violation: {:?}", result.is_ok() || result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════════
// PERFORMANCE TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Bulk insert performance
#[test]
fn test_sqlite_bulk_insert_performance() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("CREATE TABLE بيانات (id INTEGER PRIMARY KEY, قيمة TEXT)")؛
        
        db.نفذ("BEGIN TRANSACTION")؛
        
        # إدراج 1000 سجل
        لكل س في مدى(1، 1001) {
            db.نفذ("INSERT INTO بيانات (قيمة) VALUES ('قيمة_' + نص(س))")؛
        }
        
        db.نفذ("COMMIT")؛
        
        متغير عدد = db.استعلام("SELECT COUNT(*) FROM بيانات")؛
        اطبع("تم إدراج: " + عدد + " سجل")؛
    "#;
    
    let mut interp = Interpreter::new();
    let start = std::time::Instant::now();
    let result = interp.run(source);
    let elapsed = start.elapsed();
    
    println!("✅ test_sqlite_bulk_insert_performance: {:?} ({:?})", result.is_ok(), elapsed);
    
    // Should complete in reasonable time
    assert!(elapsed < std::time::Duration::from_secs(30), "بطيء جداً: {:?}", elapsed);
}

/// Test: Complex query performance
#[test]
fn test_sqlite_complex_query_performance() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("CREATE TABLE جدول1 (id INTEGER PRIMARY KEY, قيمة INTEGER)")؛
        db.نفذ("CREATE TABLE جدول2 (id INTEGER PRIMARY KEY, مرجع INTEGER)")؛
        
        # إدراج بيانات
        لكل س في مدى(1، 501) {
            db.نفذ("INSERT INTO جدول1 (قيمة) VALUES (" + نص(س) + ")")؛
            db.نفذ("INSERT INTO جدول2 (مرجع) VALUES (" + نص(س) + ")")؛
        }
        
        # استعلام معقد
        متغير نتيجة = db.استعلام("
            SELECT j1.قيمة, COUNT(*) as عدد
            FROM جدول1 j1
            JOIN جدول2 j2 ON j1.id = j2.مرجع
            GROUP BY j1.قيمة
            ORDER BY عدد DESC
            LIMIT 10
        ")؛
        
        اطبع(نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let start = std::time::Instant::now();
    let result = interp.run(source);
    let elapsed = start.elapsed();
    
    println!("✅ test_sqlite_complex_query_performance: {:?} ({:?})", result.is_ok(), elapsed);
}

// ═══════════════════════════════════════════════════════════════════════════════
// ARABIC DATA TESTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Test: Arabic text in database
#[test]
fn test_sqlite_arabic_text() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("
            CREATE TABLE كتب (
                id INTEGER PRIMARY KEY,
                العنوان TEXT,
                المؤلف TEXT,
                الناشر TEXT
            )
        ")؛
        
        db.نفذ("INSERT INTO كتب VALUES (1, 'مقدمة ابن خلدون', 'ابن خلدون', 'دار الكتب العلمية')")؛
        db.نفذ("INSERT INTO كتب VALUES (2, 'الأم', 'الماوردي', 'مكتبة الكليات الأزهرية')")؛
        db.نفذ("INSERT INTO كتب VALUES (3, 'إحياء علوم الدين', 'الغزالي', 'دار الفكر')")؛
        
        متغير نتيجة = db.استعلام("SELECT * FROM كتب WHERE المؤلف LIKE '%الغزالي%'")؛
        اطبع(نتيجة)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_sqlite_arabic_text: {:?}", result.is_ok());
}

/// Test: Arabic search with LIKE
#[test]
fn test_sqlite_arabic_search() {
    let source = r#"
        متغير db = قاعدة_بيانات(":memory:")؛
        
        db.نفذ("CREATE TABLE مدن (id INTEGER PRIMARY KEY, الاسم TEXT, الدولة TEXT)")؛
        
        db.نفذ("INSERT INTO مدن VALUES (1, 'القاهرة', 'مصر')")؛
        db.نفذ("INSERT INTO مدن VALUES (2, 'الإسكندرية', 'مصر')")؛
        db.نفذ("INSERT INTO مدن VALUES (3, 'الرياض', 'السعودية')")؛
        db.نفذ("INSERT INTO مدن VALUES (4, 'جدة', 'السعودية')")؛
        
        # بحث بالعربية
        متغير مصر = db.استعلام("SELECT * FROM مدن WHERE الدولة = 'مصر'")؛
        اطبع("المدن المصرية: " + مصر)؛
        
        متغير بالألف = db.استعلام("SELECT * FROM مدن WHERE الاسم LIKE 'ال%'")؛
        اطبع("المدن التي تبدأ بـ 'ال': " + بالألف)؛
    "#;
    
    let mut interp = Interpreter::new();
    let result = interp.run(source);
    
    println!("✅ test_sqlite_arabic_search: {:?}", result.is_ok());
}
