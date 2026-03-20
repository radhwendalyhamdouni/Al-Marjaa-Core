// ═══════════════════════════════════════════════════════════════════════════════
// نظام الأنواع في Cranelift Backend
// ═══════════════════════════════════════════════════════════════════════════════
// تعريف أنواع البيانات وتحويلها إلى أنواع Cranelift
// ═══════════════════════════════════════════════════════════════════════════════

use cranelift::prelude::*;
use std::collections::HashMap;

/// أنواع البيانات في لغة المرجع
#[derive(Debug, Clone, PartialEq)]
pub enum MarjaaType {
    /// رقم (float 64-bit)
    Number,
    /// نص (UTF-8 string)
    String,
    /// منطقي (boolean)
    Boolean,
    /// قائمة (list)
    List(Box<MarjaaType>),
    /// قاموس (dictionary)
    Dict(Box<MarjaaType>, Box<MarjaaType>),
    /// كائن (object)
    Object(HashMap<String, MarjaaType>),
    /// دالة (function)
    Function(Vec<MarjaaType>, Box<MarjaaType>),
    /// لا شيء (null/void)
    Null,
    /// أي نوع (any)
    Any,
    /// نوع مخصص
    Custom(String),
}

impl MarjaaType {
    /// الحصول على حجم النوع بالبايت
    pub fn size(&self) -> u32 {
        match self {
            MarjaaType::Number => 8,      // f64
            MarjaaType::Boolean => 1,     // i1 -> stored as i8
            MarjaaType::String => 8,      // pointer
            MarjaaType::List(_) => 24,    // ptr + length + capacity
            MarjaaType::Dict(_, _) => 24, // ptr + size + capacity
            MarjaaType::Null => 8,
            MarjaaType::Any => 8,
            MarjaaType::Custom(_) => 8,
            MarjaaType::Function(_, _) => 8, // function pointer
            MarjaaType::Object(_) => 8,      // object pointer
        }
    }
}

/// نظام الأنواع
pub struct TypeSystem {
    /// أنواع Cranelift الأساسية
    types: HashMap<String, Type>,
}

impl TypeSystem {
    /// إنشاء نظام أنواع جديد
    pub fn new() -> Self {
        let mut types = HashMap::new();
        types.insert("رقم".to_string(), types::F64);
        types.insert("منطقي".to_string(), types::I8);
        types.insert("نص".to_string(), types::I64); // pointer

        TypeSystem { types }
    }

    /// الحصول على نوع Cranelift من نوع المرجع
    pub fn get_cranelift_type(&self, marjaa_type: &MarjaaType) -> Type {
        match marjaa_type {
            MarjaaType::Number => types::F64,
            MarjaaType::Boolean => types::I8,
            MarjaaType::String => types::I64, // pointer as i64
            MarjaaType::Null => types::F64,   // null as 0.0
            MarjaaType::Any => types::F64,
            MarjaaType::List(_) => types::I64, // pointer
            MarjaaType::Dict(_, _) => types::I64, // pointer
            MarjaaType::Custom(name) => {
                self.types.get(name).copied().unwrap_or(types::F64)
            }
            MarjaaType::Function(_, _) => types::I64, // function pointer
            MarjaaType::Object(_) => types::I64,      // object pointer
        }
    }

    /// تحويل نوع المرجع من سلسلة نصية
    pub fn parse_type(&self, type_str: &str) -> MarjaaType {
        match type_str.trim() {
            "رقم" | "عدد" | "number" | "float" | "f64" => MarjaaType::Number,
            "نص" | "string" | "str" => MarjaaType::String,
            "منطقي" | "bool" | "boolean" => MarjaaType::Boolean,
            "قائمة" | "list" | "array" => MarjaaType::List(Box::new(MarjaaType::Any)),
            "قاموس" | "dict" | "map" => {
                MarjaaType::Dict(Box::new(MarjaaType::String), Box::new(MarjaaType::Any))
            }
            "لا_شيء" | "null" | "void" => MarjaaType::Null,
            "أي" | "any" => MarjaaType::Any,
            name => MarjaaType::Custom(name.to_string()),
        }
    }

    /// التحقق من توافق الأنواع
    pub fn are_types_compatible(&self, expected: &MarjaaType, actual: &MarjaaType) -> bool {
        match (expected, actual) {
            (MarjaaType::Any, _) | (_, MarjaaType::Any) => true,
            (MarjaaType::Number, MarjaaType::Number) => true,
            (MarjaaType::String, MarjaaType::String) => true,
            (MarjaaType::Boolean, MarjaaType::Boolean) => true,
            (MarjaaType::Null, MarjaaType::Null) => true,
            (MarjaaType::List(e1), MarjaaType::List(a1)) => self.are_types_compatible(e1, a1),
            (MarjaaType::Dict(ek, ev), MarjaaType::Dict(ak, av)) => {
                self.are_types_compatible(ek, ak) && self.are_types_compatible(ev, av)
            }
            (MarjaaType::Custom(e), MarjaaType::Custom(a)) => e == a,
            _ => false,
        }
    }

    /// تحويل نوع المرجع إلى سلسلة نصية
    pub fn type_to_string(&self, marjaa_type: &MarjaaType) -> String {
        match marjaa_type {
            MarjaaType::Number => "رقم".to_string(),
            MarjaaType::String => "نص".to_string(),
            MarjaaType::Boolean => "منطقي".to_string(),
            MarjaaType::List(inner) => format!("قائمة[{}]", self.type_to_string(inner)),
            MarjaaType::Dict(key, val) => format!(
                "قاموس[{}: {}]",
                self.type_to_string(key),
                self.type_to_string(val)
            ),
            MarjaaType::Null => "لا_شيء".to_string(),
            MarjaaType::Any => "أي".to_string(),
            MarjaaType::Custom(name) => name.clone(),
            MarjaaType::Function(params, ret) => {
                let params_str: Vec<String> =
                    params.iter().map(|p| self.type_to_string(p)).collect();
                format!(
                    "دالة({}) -> {}",
                    params_str.join("، "),
                    self.type_to_string(ret)
                )
            }
            MarjaaType::Object(fields) => {
                let fields_str: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, self.type_to_string(v)))
                    .collect();
                format!("كائن{{{}}}", fields_str.join("، "))
            }
        }
    }
}

impl Default for TypeSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_system_creation() {
        let type_system = TypeSystem::new();
        assert_eq!(type_system.type_to_string(&MarjaaType::Number), "رقم");
    }

    #[test]
    fn test_type_parsing() {
        let type_system = TypeSystem::new();

        assert_eq!(type_system.parse_type("رقم"), MarjaaType::Number);
        assert_eq!(type_system.parse_type("نص"), MarjaaType::String);
        assert_eq!(type_system.parse_type("منطقي"), MarjaaType::Boolean);
    }

    #[test]
    fn test_type_compatibility() {
        let type_system = TypeSystem::new();

        assert!(type_system.are_types_compatible(&MarjaaType::Number, &MarjaaType::Number));
        assert!(type_system.are_types_compatible(&MarjaaType::Any, &MarjaaType::Number));
        assert!(!type_system.are_types_compatible(&MarjaaType::Number, &MarjaaType::String));
    }
}
