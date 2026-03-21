// ═══════════════════════════════════════════════════════════════════════════════
// Modules - نظام الوحدات والحزم
// ═══════════════════════════════════════════════════════════════════════════════
// يوفر نظام وحدات متكامل للغة المرجع:
// - Module System: تنظيم الكود في وحدات
// - Module Loader: تحميل وتنفيذ ملفات .mrj
// - Package Manager: إدارة الحزم والتبعيات
// - FFI: التكامل مع مكتبات خارجية
// ═══════════════════════════════════════════════════════════════════════════════

pub mod module_loader;
pub mod module_system;
pub mod package_manager;

// إعادة تصدير الأنواع الرئيسية من module_loader
pub use module_loader::{
    LoadError, LoadedModule, LoaderStats, ModuleLoader,
};

// إعادة تصدير الأنواع الرئيسية من module_system
pub use module_system::{
    ExportKind, ExportStatement, ImportKind, ImportStatement, Module, ModuleError, ModuleId,
    ModuleManager, ModuleManagerStats, ModuleScope, ModuleStats, SourceLocation, TypeDefinition,
};

pub use package_manager::{
    InstalledPackage, PackageError, PackageInfo, PackageManager, PackageManagerStats,
    PackageManifest, PackageSource,
};
