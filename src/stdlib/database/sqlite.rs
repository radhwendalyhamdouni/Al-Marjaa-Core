// src/stdlib/database/sqlite.rs
// دعم SQLite للغة المرجع
// SQLite Support for Al-Marjaa Language

use rusqlite::{Connection, params, ToSql, types::{Value, ValueRef, ToSqlOutput}};
use std::path::Path;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::errors::{DatabaseError, DatabaseResult};

/// قيمة قاعدة البيانات
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseValue {
    /// قيمة فارغة
    Null,
    /// عدد صحيح
    Integer(i64),
    /// عدد حقيقي
    Real(f64),
    /// نص
    Text(String),
    /// بيانات ثنائية
    Blob(Vec<u8>),
}

impl DatabaseValue {
    /// إنشاء قيمة فارغة
    pub fn فارغ() -> Self {
        DatabaseValue::Null
    }

    /// إنشاء عدد صحيح
    pub fn عدد_صحيح(value: i64) -> Self {
        DatabaseValue::Integer(value)
    }

    /// إنشاء عدد حقيقي
    pub fn عدد_حقيقي(value: f64) -> Self {
        DatabaseValue::Real(value)
    }

    /// إنشاء نص
    pub fn نص(value: &str) -> Self {
        DatabaseValue::Text(value.to_string())
    }

    /// إنشاء بيانات ثنائية
    pub fn بيانات(value: Vec<u8>) -> Self {
        DatabaseValue::Blob(value)
    }

    /// هل القيمة فارغة
    pub fn هل_فارغ(&self) -> bool {
        matches!(self, DatabaseValue::Null)
    }

    /// هل القيمة عدد
    pub fn هل_عدد(&self) -> bool {
        matches!(self, DatabaseValue::Integer(_) | DatabaseValue::Real(_))
    }

    /// هل القيمة نص
    pub fn هل_نص(&self) -> bool {
        matches!(self, DatabaseValue::Text(_))
    }

    /// تحويل إلى عدد صحيح
    pub fn إلى_عدد_صحيح(&self) -> Option<i64> {
        match self {
            DatabaseValue::Integer(i) => Some(*i),
            DatabaseValue::Real(f) => Some(*f as i64),
            _ => None,
        }
    }

    /// تحويل إلى عدد حقيقي
    pub fn إلى_عدد_حقيقي(&self) -> Option<f64> {
        match self {
            DatabaseValue::Integer(i) => Some(*i as f64),
            DatabaseValue::Real(f) => Some(*f),
            _ => None,
        }
    }

    /// تحويل إلى نص
    pub fn إلى_نص(&self) -> Option<&str> {
        match self {
            DatabaseValue::Text(s) => Some(s),
            _ => None,
        }
    }

    /// تحويل إلى بيانات ثنائية
    pub fn إلى_بيانات(&self) -> Option<&[u8]> {
        match self {
            DatabaseValue::Blob(b) => Some(b),
            _ => None,
        }
    }
}

impl ToSql for DatabaseValue {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            DatabaseValue::Null => Ok(ToSqlOutput::Borrowed(ValueRef::Null)),
            DatabaseValue::Integer(i) => Ok((*i).into()),
            DatabaseValue::Real(f) => Ok((*f).into()),
            DatabaseValue::Text(s) => Ok(s.as_str().into()),
            DatabaseValue::Blob(b) => Ok(b.as_slice().into()),
        }
    }
}

impl From<&rusqlite::types::Value> for DatabaseValue {
    fn from(value: &rusqlite::types::Value) -> Self {
        match value {
            Value::Null => DatabaseValue::Null,
            Value::Integer(i) => DatabaseValue::Integer(*i),
            Value::Real(f) => DatabaseValue::Real(*f),
            Value::Text(s) => DatabaseValue::Text(s.clone()),
            Value::Blob(b) => DatabaseValue::Blob(b.clone()),
        }
    }
}

/// صف من النتائج
pub type صف = HashMap<String, DatabaseValue>;

/// نتيجة استعلام
#[derive(Debug, Clone)]
pub struct نتيجة_استعلام {
    /// الصفوف
    pub صفوف: Vec<صف>,
    /// عدد الصفوف المتأثرة
    pub عدد_المتأثرين: usize,
    /// آخر معرف مدرج
    pub آخر_معرف: Option<i64>,
}

impl نتيجة_استعلام {
    /// إنشاء نتيجة فارغة
    pub fn فارغة() -> Self {
        نتيجة_استعلام {
            صفوف: Vec::new(),
            عدد_المتأثرين: 0,
            آخر_معرف: None,
        }
    }

    /// هل النتيجة فارغة
    pub fn هل_فارغة(&self) -> bool {
        self.صفوف.is_empty()
    }

    /// عدد الصفوف
    pub fn عدد_الصفوف(&self) -> usize {
        self.صفوف.len()
    }

    /// الحصول على صف
    pub fn صف(&self, index: usize) -> Option<&صف> {
        self.صفوف.get(index)
    }

    /// الحصول على قيمة
    pub fn قيمة(&self, row: usize, column: &str) -> Option<&DatabaseValue> {
        self.صفوف.get(row)?.get(column)
    }
}

/// اتصال قاعدة البيانات
#[derive(Debug)]
pub struct اتصال {
    /// الاتصال الداخلي
    connection: Connection,
    /// مسار الملف
    path: String,
}

impl اتصال {
    /// فتح اتصال بقاعدة بيانات
    pub fn فتح(المسار: &str) -> DatabaseResult<Self> {
        let conn = if المسار == ":memory:" || المسار == "ذاكرة" {
            Connection::open_in_memory()?
        } else {
            Connection::open(Path::new(المسار))?
        };

        // تفعيل المفاتيح الأجنبية
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        Ok(اتصال {
            connection: conn,
            path: المسار.to_string(),
        })
    }

    /// إنشاء قاعدة بيانات في الذاكرة
    pub fn في_الذاكرة() -> DatabaseResult<Self> {
        Self::فتح(":memory:")
    }

    /// تنفيذ استعلام بدون نتائج
    pub fn تنفيذ(&self, الاستعلام: &str) -> DatabaseResult<usize> {
        let affected = self.connection.execute(الاستعلام, params![])?;
        Ok(affected)
    }

    /// تنفيذ استعلام مع معاملات
    pub fn تنفيذ_مع(&self, الاستعلام: &str, المعاملات: &[DatabaseValue]) -> DatabaseResult<usize> {
        let params: Vec<&dyn ToSql> = المعاملات.iter().map(|v| v as &dyn ToSql).collect();
        let affected = self.connection.execute(الاستعلام, params.as_slice())?;
        Ok(affected)
    }

    /// تنفيذ استعلام وجلب النتائج
    pub fn استعلام(&self, الاستعلام: &str) -> DatabaseResult<نتيجة_استعلام> {
        let mut stmt = self.connection.prepare(الاستعلام)?;
        let column_names: Vec<String> = stmt.column_names().into_iter().map(|s| s.to_string()).collect();

        let rows = stmt.query_map([], |row| {
            let mut result = HashMap::new();
            for (i, name) in column_names.iter().enumerate() {
                let value = row.get::<usize, Value>(i)?;
                result.insert(name.clone(), DatabaseValue::from(&value));
            }
            Ok(result)
        })?;

        let صفوف: Vec<صف> = rows.collect::<Result<Vec<_>, _>>()?;

        Ok(نتيجة_استعلام {
            صفوف,
            عدد_المتأثرين: 0,
            آخر_معرف: None,
        })
    }

    /// تنفيذ استعلام مع معاملات
    pub fn استعلام_مع(&self, الاستعلام: &str, المعاملات: &[DatabaseValue]) -> DatabaseResult<نتيجة_استعلام> {
        let mut stmt = self.connection.prepare(الاستعلام)?;
        let column_names: Vec<String> = stmt.column_names().into_iter().map(|s| s.to_string()).collect();

        let params: Vec<&dyn ToSql> = المعاملات.iter().map(|v| v as &dyn ToSql).collect();
        let rows = stmt.query_map(params.as_slice(), |row| {
            let mut result = HashMap::new();
            for (i, name) in column_names.iter().enumerate() {
                let value = row.get::<usize, Value>(i)?;
                result.insert(name.clone(), DatabaseValue::from(&value));
            }
            Ok(result)
        })?;

        let صفوف: Vec<صف> = rows.collect::<Result<Vec<_>, _>>()?;

        Ok(نتيجة_استعلام {
            صفوف,
            عدد_المتأثرين: 0,
            آخر_معرف: None,
        })
    }

    /// إدراج صف
    pub fn إدراج(&self, الجدول: &str, البيانات: &صف) -> DatabaseResult<i64> {
        let columns: Vec<&str> = البيانات.keys().map(|s| s.as_str()).collect();
        let values: Vec<&dyn ToSql> = البيانات.values().map(|v| v as &dyn ToSql).collect();
        let placeholders: Vec<&str> = columns.iter().map(|_| "?").collect();

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            الجدول,
            columns.join(", "),
            placeholders.join(", ")
        );

        self.connection.execute(&query, values.as_slice())?;
        Ok(self.connection.last_insert_rowid())
    }

    /// تحديث صفوف
    pub fn تحديث(&self, الجدول: &str, البيانات: &صف, الشرط: &str) -> DatabaseResult<usize> {
        let set_clause: Vec<String> = البيانات.keys().map(|k| format!("{} = ?", k)).collect();
        let values: Vec<&dyn ToSql> = البيانات.values().map(|v| v as &dyn ToSql).collect();

        let query = format!(
            "UPDATE {} SET {} WHERE {}",
            الجدول,
            set_clause.join(", "),
            الشرط
        );

        let affected = self.connection.execute(&query, values.as_slice())?;
        Ok(affected)
    }

    /// حذف صفوف
    pub fn حذف(&self, الجدول: &str, الشرط: &str) -> DatabaseResult<usize> {
        let query = format!("DELETE FROM {} WHERE {}", الجدول, الشرط);
        let affected = self.connection.execute(&query, params![])?;
        Ok(affected)
    }

    /// حذف كل الصفوف
    pub fn حذف_الكل(&self, الجدول: &str) -> DatabaseResult<usize> {
        let query = format!("DELETE FROM {}", الجدول);
        let affected = self.connection.execute(&query, params![])?;
        Ok(affected)
    }

    /// اختيار صفوف
    pub fn اختيار(&self, الجدول: &str, الأعمدة: &[&str], الشرط: Option<&str>) -> DatabaseResult<نتيجة_استعلام> {
        let columns = if الأعمدة.is_empty() { "*".to_string() } else { الأعمدة.join(", ") };
        let query = match الشرط {
            Some(cond) => format!("SELECT {} FROM {} WHERE {}", columns, الجدول, cond),
            None => format!("SELECT {} FROM {}", columns, الجدول),
        };
        self.استعلام(&query)
    }

    /// اختيار صف واحد
    pub fn اختيار_واحد(&self, الجدول: &str, الأعمدة: &[&str], الشرط: &str) -> DatabaseResult<Option<صف>> {
        let result = self.اختيار(الجدول, الأعمدة, Some(الشرط))?;
        Ok(result.صفوف.into_iter().next())
    }

    /// عدد الصفوف في جدول
    pub fn عدد_الصفوف(&self, الجدول: &str) -> DatabaseResult<i64> {
        let query = format!("SELECT COUNT(*) as count FROM {}", الجدول);
        let result = self.استعلام(&query)?;
        if let Some(row) = result.صفوف.first() {
            if let Some(DatabaseValue::Integer(count)) = row.get("count") {
                return Ok(*count);
            }
        }
        Ok(0)
    }

    /// إنشاء جدول
    pub fn إنشاء_جدول(&self, الجدول: &str, الأعمدة: &str) -> DatabaseResult<()> {
        let query = format!("CREATE TABLE IF NOT EXISTS {} ({})", الجدول, الأعمدة);
        self.connection.execute(&query, params![])?;
        Ok(())
    }

    /// حذف جدول
    pub fn حذف_جدول(&self, الجدول: &str) -> DatabaseResult<()> {
        let query = format!("DROP TABLE IF EXISTS {}", الجدول);
        self.connection.execute(&query, params![])?;
        Ok(())
    }

    /// هل الجدول موجود
    pub fn هل_الجدول_موجود(&self, الجدول: &str) -> DatabaseResult<bool> {
        let query = "SELECT name FROM sqlite_master WHERE type='table' AND name=?";
        let result = self.استعلام_مع(query, &[DatabaseValue::نص(الجدول)])?;
        Ok(!result.صفوف.is_empty())
    }

    /// قائمة الجداول
    pub fn قائمة_الجداول(&self) -> DatabaseResult<Vec<String>> {
        let query = "SELECT name FROM sqlite_master WHERE type='table'";
        let result = self.استعلام(query)?;
        let tables: Vec<String> = result.صفوف.iter()
            .filter_map(|row| {
                if let Some(DatabaseValue::Text(name)) = row.get("name") {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();
        Ok(tables)
    }

    /// بدء معاملة
    pub fn بدء_معاملة(&self) -> DatabaseResult<()> {
        self.connection.execute("BEGIN TRANSACTION", params![])?;
        Ok(())
    }

    /// تأكيد المعاملة
    pub fn تأكيد(&self) -> DatabaseResult<()> {
        self.connection.execute("COMMIT", params![])?;
        Ok(())
    }

    /// إلغاء المعاملة
    pub fn إلغاء(&self) -> DatabaseResult<()> {
        self.connection.execute("ROLLBACK", params![])?;
        Ok(())
    }

    /// تنفيذ معاملة
    pub fn معاملة<F>(&self, العملية: F) -> DatabaseResult<()>
    where
        F: FnOnce() -> DatabaseResult<()>,
    {
        self.بدء_معاملة()?;
        match العملية() {
            Ok(()) => self.تأكيد(),
            Err(e) => {
                let _ = self.إلغاء();
                Err(e)
            }
        }
    }

    /// تنفيذ سكربت
    pub fn تنفيذ_سكربت(&self, السكربت: &str) -> DatabaseResult<()> {
        self.connection.execute_batch(السكربت)?;
        Ok(())
    }

    /// إغلاق الاتصال
    pub fn إغلاق(self) -> DatabaseResult<()> {
        self.connection.close().map_err(|(_, e)| DatabaseError::GeneralError(e.to_string()))?;
        Ok(())
    }

    /// الحصول على المسار
    pub fn المسار(&self) -> &str {
        &self.path
    }
}

/// اتصال آمن (thread-safe)
pub type اتصال_آمن = Arc<Mutex<اتصال>>;

/// إنشاء اتصال آمن
pub fn اتصال_آمن(المسار: &str) -> DatabaseResult<اتصال_آمن> {
    let conn = اتصال::فتح(المسار)?;
    Ok(Arc::new(Mutex::new(conn)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_database() {
        let conn = اتصال::في_الذاكرة().unwrap();

        // إنشاء جدول
        conn.إنشاء_جدول(
            "مستخدمين",
            "id INTEGER PRIMARY KEY, الاسم TEXT, العمر INTEGER"
        ).unwrap();

        // إدراج
        let mut data = HashMap::new();
        data.insert("الاسم".to_string(), DatabaseValue::نص("أحمد"));
        data.insert("العمر".to_string(), DatabaseValue::عدد_صحيح(25));
        let id = conn.إدراج("مستخدمين", &data).unwrap();
        assert_eq!(id, 1);

        // اختيار
        let result = conn.اختيار("مستخدمين", &[], None).unwrap();
        assert_eq!(result.عدد_الصفوف(), 1);

        // عدد الصفوف
        let count = conn.عدد_الصفوف("مستخدمين").unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_database_value() {
        let null = DatabaseValue::فارغ();
        assert!(null.هل_فارغ());

        let int = DatabaseValue::عدد_صحيح(42);
        assert!(int.هل_عدد());
        assert_eq!(int.إلى_عدد_صحيح(), Some(42));

        let text = DatabaseValue::نص("مرحباً");
        assert!(text.هل_نص());
        assert_eq!(text.إلى_نص(), Some("مرحباً"));
    }

    #[test]
    fn test_transaction() {
        let conn = اتصال::في_الذاكرة().unwrap();
        conn.إنشاء_جدول("حسابات", "id INTEGER PRIMARY KEY, رصيد REAL").unwrap();

        let result = conn.معاملة(|| {
            let mut data = HashMap::new();
            data.insert("رصيد".to_string(), DatabaseValue::عدد_حقيقي(100.0));
            conn.إدراج("حسابات", &data)?;
            Ok(())
        });

        assert!(result.is_ok());
        let count = conn.عدد_الصفوف("حسابات").unwrap();
        assert_eq!(count, 1);
    }
}
