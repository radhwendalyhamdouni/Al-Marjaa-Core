// src/stdlib/database/errors.rs
// أخطاء قواعد البيانات للغة المرجع
// Database Errors for Al-Marjaa Language

use std::fmt;

/// أنواع أخطاء قواعد البيانات
#[derive(Debug)]
pub enum DatabaseError {
    /// خطأ في الاتصال
    ConnectionError(String),
    /// خطأ في الاستعلام
    QueryError(String),
    /// خطأ في التنفيذ
    ExecutionError(String),
    /// خطأ في المعاملة
    TransactionError(String),
    /// خطأ في التحضير
    PreparationError(String),
    /// خطأ في جلب البيانات
    FetchError(String),
    /// خطأ في الربط
    BindError(String),
    /// قاعدة بيانات غير موجودة
    DatabaseNotFound(String),
    /// جدول غير موجود
    TableNotFound(String),
    /// عمود غير موجود
    ColumnNotFound(String),
    /// نوع بيانات غير صالح
    InvalidDataType(String),
    /// خطأ عام
    GeneralError(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::ConnectionError(msg) => write!(f, "خطأ في الاتصال: {}", msg),
            DatabaseError::QueryError(msg) => write!(f, "خطأ في الاستعلام: {}", msg),
            DatabaseError::ExecutionError(msg) => write!(f, "خطأ في التنفيذ: {}", msg),
            DatabaseError::TransactionError(msg) => write!(f, "خطأ في المعاملة: {}", msg),
            DatabaseError::PreparationError(msg) => write!(f, "خطأ في التحضير: {}", msg),
            DatabaseError::FetchError(msg) => write!(f, "خطأ في جلب البيانات: {}", msg),
            DatabaseError::BindError(msg) => write!(f, "خطأ في الربط: {}", msg),
            DatabaseError::DatabaseNotFound(name) => write!(f, "قاعدة البيانات '{}' غير موجودة", name),
            DatabaseError::TableNotFound(name) => write!(f, "الجدول '{}' غير موجود", name),
            DatabaseError::ColumnNotFound(name) => write!(f, "العمود '{}' غير موجود", name),
            DatabaseError::InvalidDataType(msg) => write!(f, "نوع بيانات غير صالح: {}", msg),
            DatabaseError::GeneralError(msg) => write!(f, "خطأ: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}

impl From<rusqlite::Error> for DatabaseError {
    fn from(error: rusqlite::Error) -> Self {
        match error {
            rusqlite::Error::SqliteFailure(err, msg) => {
                let message = msg.unwrap_or_else(|| format!("SQLite error code: {:?}", err.code));
                DatabaseError::QueryError(message)
            }
            rusqlite::Error::InvalidQuery => DatabaseError::QueryError("استعلام غير صالح".to_string()),
            rusqlite::Error::InvalidParameterName(name) => {
                DatabaseError::BindError(format!("اسم معامل غير صالح: {}", name))
            }
            rusqlite::Error::InvalidColumnType(idx, name, _) => {
                DatabaseError::InvalidDataType(format!("نوع العمود {} ({}) غير صالح", idx, name))
            }
            rusqlite::Error::QueryReturnedNoRows => DatabaseError::FetchError("لا توجد نتائج".to_string()),
            rusqlite::Error::InvalidPath(path) => {
                DatabaseError::DatabaseNotFound(format!("{:?}", path))
            }
            rusqlite::Error::NulError(_) => DatabaseError::GeneralError("خطأ في معالجة النص".to_string()),
            rusqlite::Error::Utf8Error(_) => DatabaseError::GeneralError("خطأ في ترميز النص".to_string()),
            _ => DatabaseError::GeneralError(error.to_string()),
        }
    }
}

/// نتيجة عملية قاعدة البيانات
pub type DatabaseResult<T> = Result<T, DatabaseError>;
