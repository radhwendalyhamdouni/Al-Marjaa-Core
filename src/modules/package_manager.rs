// ═══════════════════════════════════════════════════════════════════════════════
// Package Manager - مدير الحزم
// ═══════════════════════════════════════════════════════════════════════════════
// إدارة حزم لغة المرجع
// تثبيت، تحديث، إزالة الحزم
// إدارة التبعيات
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
// تعريف الحزمة
// ═══════════════════════════════════════════════════════════════════════════════

/// ملف تعريف الحزمة (marjaa.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    /// اسم الحزمة
    pub name: String,
    
    /// الإصدار (Semantic Versioning)
    pub version: String,
    
    /// الوصف
    pub description: Option<String>,
    
    /// المؤلفون
    pub authors: Vec<String>,
    
    /// الترخيص
    pub license: Option<String>,
    
    /// نقطة الدخول الرئيسية
    pub main: String,
    
    /// التبعيات
    pub dependencies: HashMap<String, String>,
    
    /// تبعيات التطوير
    #[serde(default)]
    pub dev_dependencies: HashMap<String, String>,
    
    /// الكلمات المفتاحية
    #[serde(default)]
    pub keywords: Vec<String>,
    
    /// الرخصة
    pub repository: Option<String>,
    
    /// الملفات المضمنة
    #[serde(default)]
    pub include: Vec<String>,
    
    /// الملفات المستبعدة
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl PackageManifest {
    /// إنشاء ملف تعريف جديد
    pub fn new(name: &str, version: &str) -> Self {
        PackageManifest {
            name: name.to_string(),
            version: version.to_string(),
            description: None,
            authors: Vec::new(),
            license: None,
            main: "main.mrj".to_string(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            keywords: Vec::new(),
            repository: None,
            include: Vec::new(),
            exclude: Vec::new(),
        }
    }
    
    /// تحميل من ملف
    pub fn from_file(path: &Path) -> Result<Self, PackageError> {
        let content = fs::read_to_string(path)
            .map_err(|e| PackageError::IoError(e.to_string()))?;
        
        toml::from_str(&content)
            .map_err(|e| PackageError::ParseError(e.to_string()))
    }
    
    /// حفظ إلى ملف
    pub fn save(&self, path: &Path) -> Result<(), PackageError> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| PackageError::ParseError(e.to_string()))?;
        
        fs::write(path, content)
            .map_err(|e| PackageError::IoError(e.to_string()))?;
        
        Ok(())
    }
    
    /// إضافة تبعية
    pub fn add_dependency(&mut self, name: &str, version: &str) {
        self.dependencies.insert(name.to_string(), version.to_string());
    }
    
    /// إزالة تبعية
    pub fn remove_dependency(&mut self, name: &str) {
        self.dependencies.remove(name);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الحزمة المثبتة
// ═══════════════════════════════════════════════════════════════════════════════

/// حزمة مثبتة
#[derive(Debug, Clone)]
pub struct InstalledPackage {
    /// ملف التعريف
    pub manifest: PackageManifest,
    
    /// مسار التثبيت
    pub install_path: PathBuf,
    
    /// وقت التثبيت
    pub installed_at: u64,
    
    /// المصدر
    pub source: PackageSource,
}

/// مصدر الحزمة
#[derive(Debug, Clone)]
pub enum PackageSource {
    /// من السجل الرسمي
    Registry,
    /// من Git
    Git {
        url: String,
        commit: String,
    },
    /// من مسار محلي
    Local {
        path: PathBuf,
    },
}

// ═══════════════════════════════════════════════════════════拓扑═══════════════════════════════

/// خطأ في الحزمة
#[derive(Debug, Clone)]
pub enum PackageError {
    /// خطأ I/O
    IoError(String),
    
    /// خطأ في التحليل
    ParseError(String),
    
    /// الحزمة غير موجودة
    PackageNotFound(String),
    
    /// تعارض الإصدارات
    VersionConflict {
        package: String,
        required: String,
        found: String,
    },
    
    /// تبعية دائرية
    CircularDependency {
        package: String,
        chain: Vec<String>,
    },
    
    /// خطأ في الشبكة
    NetworkError(String),
    
    /// فشل التثبيت
    InstallFailed {
        package: String,
        reason: String,
    },
}

impl std::fmt::Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageError::IoError(msg) => write!(f, "خطأ I/O: {}", msg),
            PackageError::ParseError(msg) => write!(f, "خطأ في التحليل: {}", msg),
            PackageError::PackageNotFound(name) => write!(f, "الحزمة '{}' غير موجودة", name),
            PackageError::VersionConflict { package, required, found } => {
                write!(f, "تعارض إصدارات في '{}': مطلوب {}، موجود {}", package, required, found)
            }
            PackageError::CircularDependency { package, chain } => {
                write!(f, "تبعية دائرية: {} -> {}", chain.join(" -> "), package)
            }
            PackageError::NetworkError(msg) => write!(f, "خطأ في الشبكة: {}", msg),
            PackageError::InstallFailed { package, reason } => {
                write!(f, "فشل تثبيت '{}': {}", package, reason)
            }
        }
    }
}

impl std::error::Error for PackageError {}

// ═══════════════════════════════════════════════════════════════════════════════
// مدير الحزم
// ═══════════════════════════════════════════════════════════════════════════════

/// إحصائيات مدير الحزم
#[derive(Debug, Default, Clone)]
pub struct PackageManagerStats {
    pub packages_installed: u64,
    pub packages_updated: u64,
    pub packages_removed: u64,
    pub total_download_bytes: u64,
    pub cache_hits: u64,
}

/// مدير الحزم
pub struct PackageManager {
    /// مسار الجذر للمشروع
    project_root: PathBuf,
    
    /// مسار الحزم المثبتة
    packages_dir: PathBuf,
    
    /// مسار الكاش
    #[allow(dead_code)] // سيُستخدم للتخزين المؤقت للحزم
    cache_dir: PathBuf,
    
    /// الحزم المثبتة
    installed: HashMap<String, InstalledPackage>,
    
    /// ملف التعريف الحالي
    manifest: Option<PackageManifest>,
    
    /// الإحصائيات
    stats: PackageManagerStats,
}

impl PackageManager {
    /// إنشاء مدير حزم جديد
    pub fn new(project_root: &Path) -> Self {
        PackageManager {
            project_root: project_root.to_path_buf(),
            packages_dir: project_root.join("marjaa_modules"),
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("marjaa")
                .join("cache"),
            installed: HashMap::new(),
            manifest: None,
            stats: PackageManagerStats::default(),
        }
    }
    
    /// تهيئة مشروع جديد
    pub fn init(&mut self, name: &str) -> Result<(), PackageError> {
        // إنشاء هيكل المجلدات
        fs::create_dir_all(&self.packages_dir)
            .map_err(|e| PackageError::IoError(e.to_string()))?;
        
        fs::create_dir_all(self.project_root.join("src"))
            .map_err(|e| PackageError::IoError(e.to_string()))?;
        
        // إنشاء ملف التعريف
        let manifest = PackageManifest::new(name, "0.1.0");
        manifest.save(&self.project_root.join("marjaa.toml"))?;
        
        // إنشاء ملف القفل
        self.create_lock_file()?;
        
        // إنشاء ملف .gitignore
        self.create_gitignore()?;
        
        self.manifest = Some(manifest);
        
        Ok(())
    }
    
    /// تحميل ملف التعريف
    pub fn load_manifest(&mut self) -> Result<&PackageManifest, PackageError> {
        let manifest_path = self.project_root.join("marjaa.toml");
        self.manifest = Some(PackageManifest::from_file(&manifest_path)?);
        Ok(self.manifest.as_ref().unwrap())
    }
    
    /// تثبيت تبعية
    pub fn install(&mut self, name: &str, version: Option<&str>) -> Result<(), PackageError> {
        let version = version.unwrap_or("*");
        
        // التحقق من وجود الحزمة في الكاش
        if let Some(cached) = self.check_cache(name) {
            self.stats.cache_hits += 1;
            self.copy_from_cache(&cached)?;
            return Ok(());
        }
        
        // تحميل الحزمة (محاكاة - في الواقع من السجل)
        let package_path = self.download_package(name, version)?;
        
        // استخراج وتثبيت
        self.extract_package(name, &package_path)?;
        
        // تحديث ملف التعريف
        if let Some(ref mut manifest) = self.manifest {
            manifest.add_dependency(name, version);
            manifest.save(&self.project_root.join("marjaa.toml"))?;
        }
        
        // تحديث ملف القفل
        self.update_lock_file(name, version)?;
        
        self.stats.packages_installed += 1;
        
        Ok(())
    }
    
    /// تثبيت جميع التبعيات
    pub fn install_all(&mut self) -> Result<(), PackageError> {
        let manifest = self.load_manifest()?.clone();
        
        for (name, version) in &manifest.dependencies {
            self.install(name, Some(version))?;
        }
        
        Ok(())
    }
    
    /// تحديث حزمة
    pub fn update(&mut self, name: &str) -> Result<(), PackageError> {
        // إزالة النسخة القديمة
        self.remove(name)?;
        
        // تثبيت أحدث نسخة
        self.install(name, None)?;
        
        self.stats.packages_updated += 1;
        
        Ok(())
    }
    
    /// تحديث جميع الحزم
    pub fn update_all(&mut self) -> Result<(), PackageError> {
        let manifest = self.load_manifest()?.clone();
        
        for name in manifest.dependencies.keys() {
            self.update(name)?;
        }
        
        Ok(())
    }
    
    /// إزالة حزمة
    pub fn remove(&mut self, name: &str) -> Result<(), PackageError> {
        // حذف مجلد الحزمة
        let package_path = self.packages_dir.join(name);
        if package_path.exists() {
            fs::remove_dir_all(&package_path)
                .map_err(|e| PackageError::IoError(e.to_string()))?;
        }
        
        // تحديث ملف التعريف
        if let Some(ref mut manifest) = self.manifest {
            manifest.remove_dependency(name);
            manifest.save(&self.project_root.join("marjaa.toml"))?;
        }
        
        self.installed.remove(name);
        self.stats.packages_removed += 1;
        
        Ok(())
    }
    
    /// قائمة الحزم المثبتة
    pub fn list_installed(&self) -> Vec<&str> {
        self.installed.keys().map(|s| s.as_str()).collect()
    }
    
    /// البحث عن حزمة
    pub fn search(&self, query: &str) -> Vec<PackageInfo> {
        // محاكاة البحث - في الواقع من السجل
        vec![
            PackageInfo {
                name: format!("{}-example", query),
                version: "1.0.0".to_string(),
                description: "حزمة مثال".to_string(),
            }
        ]
    }
    
    /// معلومات حزمة
    pub fn info(&self, name: &str) -> Option<&InstalledPackage> {
        self.installed.get(name)
    }
    
    /// التحقق من التحديثات
    pub fn check_updates(&self) -> Vec<(String, String, String)> {
        // (name, current, latest)
        self.installed.iter()
            .map(|(name, pkg)| {
                (name.clone(), pkg.manifest.version.clone(), "0.0.0".to_string())
            })
            .collect()
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال داخلية
    // ═══════════════════════════════════════════════════════════════
    
    fn check_cache(&self, _name: &str) -> Option<PathBuf> {
        // TODO: التحقق من الكاش
        None
    }
    
    fn copy_from_cache(&mut self, _cached: &Path) -> Result<(), PackageError> {
        Ok(())
    }
    
    fn download_package(&mut self, name: &str, _version: &str) -> Result<PathBuf, PackageError> {
        // محاكاة - في الواقع تحميل من السجل
        let package_dir = self.packages_dir.join(name);
        fs::create_dir_all(&package_dir)
            .map_err(|e| PackageError::IoError(e.to_string()))?;
        
        Ok(package_dir)
    }
    
    fn extract_package(&mut self, name: &str, _path: &Path) -> Result<(), PackageError> {
        // محاكاة فك الضغط
        let installed = InstalledPackage {
            manifest: PackageManifest::new(name, "1.0.0"),
            install_path: self.packages_dir.join(name),
            installed_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            source: PackageSource::Registry,
        };
        
        self.installed.insert(name.to_string(), installed);
        
        Ok(())
    }
    
    fn create_lock_file(&self) -> Result<(), PackageError> {
        let lock = PackageLock {
            version: 1,
            packages: HashMap::new(),
        };
        
        let content = toml::to_string_pretty(&lock)
            .map_err(|e| PackageError::ParseError(e.to_string()))?;
        
        fs::write(self.project_root.join("marjaa.lock"), content)
            .map_err(|e| PackageError::IoError(e.to_string()))?;
        
        Ok(())
    }
    
    fn update_lock_file(&mut self, name: &str, version: &str) -> Result<(), PackageError> {
        // TODO: تحديث ملف القفل
        let _ = (name, version);
        Ok(())
    }
    
    fn create_gitignore(&self) -> Result<(), PackageError> {
        let content = r#"# Marjaa
marjaa_modules/
marjaa.lock

# Build
target/

# IDE
.vscode/
.idea/
"#;
        
        fs::write(self.project_root.join(".gitignore"), content)
            .map_err(|e| PackageError::IoError(e.to_string()))?;
        
        Ok(())
    }
    
    /// الإحصائيات
    pub fn stats(&self) -> &PackageManagerStats {
        &self.stats
    }
    
    /// طباعة تقرير
    pub fn print_report(&self) {
        println!();
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║                📦 تقرير مدير الحزم                                ║");
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ الحزم المثبتة:         {:>15}                       ║", self.stats.packages_installed);
        println!("║ الحزم المحدثة:         {:>15}                       ║", self.stats.packages_updated);
        println!("║ الحزم المحذوفة:        {:>15}                       ║", self.stats.packages_removed);
        println!("║ ضربات الكاش:           {:>15}                       ║", self.stats.cache_hits);
        println!("╠══════════════════════════════════════════════════════════════════╣");
        
        if !self.installed.is_empty() {
            println!("║ 📚 الحزم المثبتة:                                                 ║");
            for (name, pkg) in &self.installed {
                println!("║   ├── {} v{}                               ║",
                    name,
                    pkg.manifest.version
                );
            }
        }
        
        println!("╚══════════════════════════════════════════════════════════════════╝");
    }
}

impl Default for PackageManager {
    fn default() -> Self {
        Self::new(Path::new("."))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ملف القفل
// ═══════════════════════════════════════════════════════════════════════════════

/// ملف قفل الحزم (marjaa.lock)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackageLock {
    /// إصدار ملف القفل
    version: u32,
    
    /// الحزم المقفلة
    packages: HashMap<String, LockedPackage>,
}

/// حزمة مقفلة
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LockedPackage {
    /// الإصدار
    version: String,
    
    /// الهاش
    checksum: String,
    
    /// المصدر
    source: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
// معلومات الحزمة
// ═══════════════════════════════════════════════════════════════════════════════

/// معلومات حزمة (للبحث)
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_package_manifest_creation() {
        let manifest = PackageManifest::new("test-package", "1.0.0");
        assert_eq!(manifest.name, "test-package");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.main, "main.mrj");
    }

    #[test]
    fn test_package_manifest_dependencies() {
        let mut manifest = PackageManifest::new("test", "1.0.0");
        manifest.add_dependency("dep1", "1.0");
        manifest.add_dependency("dep2", "2.0");
        
        assert_eq!(manifest.dependencies.len(), 2);
        assert_eq!(manifest.dependencies.get("dep1"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_package_manager_init() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = PackageManager::new(temp_dir.path());
        
        let result = manager.init("test-project");
        assert!(result.is_ok());
    }

    #[test]
    fn test_package_manager_stats() {
        let manager = PackageManager::new(Path::new("."));
        assert_eq!(manager.stats().packages_installed, 0);
    }
}
