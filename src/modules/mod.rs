// ═══════════════════════════════════════════════════════════════════════════════
// Modules - نظام الوحدات والحزم
// ═══════════════════════════════════════════════════════════════════════════════
// يوفر نظام وحدات متكامل للغة المرجع:
// - Module System: تنظيم الكود في وحدات
// - Package Manager: إدارة الحزم والتبعيات
// - FFI: التكامل مع مكتبات خارجية
// ═══════════════════════════════════════════════════════════════════════════════

pub mod module_system;
pub mod package_manager;

// إعادة تصدير الأنواع الرئيسية
pub use module_system::{
    ExportKind, ExportStatement, ImportKind, ImportStatement, Module, ModuleError, ModuleId,
    ModuleManager, ModuleManagerStats, ModuleStats, SourceLocation, TypeDefinition,
};

pub use package_manager::{
    InstalledPackage, PackageError, PackageInfo, PackageManager, PackageManagerStats,
    PackageManifest, PackageSource,
};
