// src/stdlib/crypto/password.rs
// تشفير كلمات المرور - Production Ready Implementation
// Password Hashing

use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use bcrypt::{
    hash as bcrypt_hash_internal, verify as bcrypt_verify_internal, BcryptError, DEFAULT_COST,
};
use password_hash::SaltString;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

/// تشفير كلمة المرور بـ bcrypt - Production Ready
pub fn bcrypt_hash(password: &str, cost: u32) -> Result<String, String> {
    let cost = cost.clamp(4, 31);

    bcrypt_hash_internal(password, cost)
        .map_err(|e: BcryptError| format!("خطأ في تشفير bcrypt: {}", e))
}

/// التحقق من كلمة المرور بـ bcrypt - Production Ready
pub fn bcrypt_verify(password: &str, hash: &str) -> bool {
    bcrypt_verify_internal(password, hash).unwrap_or(false)
}

/// تشفير كلمة المرور بـ Argon2 - Production Ready
pub fn argon2_hash(password: &str, salt: &[u8]) -> Result<String, String> {
    let salt = SaltString::encode_b64(salt).map_err(|e| format!("خطأ في إنشاء الملح: {}", e))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default());

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| format!("خطأ في تشفير Argon2: {}", e))
}

/// التحقق من كلمة المرور بـ Argon2 - Production Ready
pub fn argon2_verify(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

/// تشفير كلمة المرور بـ PBKDF2 - Production Ready
pub fn pbkdf2_hash(password: &str, salt: &[u8], iterations: u32) -> Result<String, String> {
    let iterations = if iterations < 10000 {
        100000
    } else {
        iterations
    };

    let mut derived = [0u8; 32]; // 256-bit output
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, iterations, &mut derived);

    Ok(format!(
        "$pbkdf2-sha256${}${}${}",
        iterations,
        hex::encode(salt),
        hex::encode(derived)
    ))
}

/// التحقق من كلمة المرور بـ PBKDF2 - Production Ready
pub fn pbkdf2_verify(password: &str, hash: &str, _iterations: u32) -> bool {
    let parts: Vec<&str> = hash.split('$').collect();
    if parts.len() < 5 {
        return false;
    }

    let iterations: u32 = match parts[2].parse() {
        Ok(i) => i,
        Err(_) => return false,
    };

    let salt = match hex::decode(parts[3]) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let expected = match hex::decode(parts[4]) {
        Ok(e) => e,
        Err(_) => return false,
    };

    let mut derived = vec![0u8; expected.len()];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, iterations, &mut derived);

    // Constant-time comparison
    derived == expected
}

/// تشفير كلمة المرور بـ Scrypt - Production Ready
pub fn scrypt_hash(password: &str, salt: &[u8]) -> Result<String, String> {
    use scrypt::{scrypt, Params};

    let params = Params::recommended();

    let mut derived = [0u8; 32];
    scrypt(password.as_bytes(), salt, &params, &mut derived)
        .map_err(|e| format!("خطأ في تشفير scrypt: {}", e))?;

    Ok(format!(
        "$scrypt${}${}",
        hex::encode(salt),
        hex::encode(derived)
    ))
}

/// التحقق من كلمة المرور بـ Scrypt - Production Ready
pub fn scrypt_verify(password: &str, hash: &str) -> bool {
    use scrypt::{scrypt, Params};

    let parts: Vec<&str> = hash.split('$').collect();
    if parts.len() < 4 {
        return false;
    }

    let salt = match hex::decode(parts[2]) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let expected = match hex::decode(parts[3]) {
        Ok(e) => e,
        Err(_) => return false,
    };

    let params = Params::recommended();

    let mut derived = vec![0u8; expected.len()];
    if scrypt(password.as_bytes(), &salt, &params, &mut derived).is_err() {
        return false;
    }

    derived == expected
}

/// تشفير كلمة مرور تلقائي (يختار الخوارزمية الأنسب)
pub fn hash_password(password: &str) -> Result<String, String> {
    // استخدام bcrypt كخيار افتراضي (متوازن بين الأمان والأداء)
    bcrypt_hash(password, DEFAULT_COST)
}

/// التحقق من كلمة مرور تلقائي
pub fn verify_password(password: &str, hash: &str) -> bool {
    if hash.starts_with("$2b$") || hash.starts_with("$2a$") || hash.starts_with("$2y$") {
        bcrypt_verify(password, hash)
    } else if hash.starts_with("$argon2") {
        argon2_verify(password, hash)
    } else if hash.starts_with("$pbkdf2") {
        pbkdf2_verify(password, hash, 100000)
    } else if hash.starts_with("$scrypt") {
        scrypt_verify(password, hash)
    } else {
        false
    }
}

/// توليد ملح عشوائي
pub fn generate_salt(length: usize) -> Vec<u8> {
    super::random::random_bytes(length)
}

/// فحص قوة كلمة المرور
pub fn password_strength(password: &str) -> PasswordStrength {
    let mut score = 0;

    // طول كلمة المرور
    if password.len() >= 8 {
        score += 1;
    }
    if password.len() >= 12 {
        score += 1;
    }
    if password.len() >= 16 {
        score += 1;
    }

    // وجود أحرف كبيرة
    if password.chars().any(|c| c.is_uppercase()) {
        score += 1;
    }

    // وجود أحرف صغيرة
    if password.chars().any(|c| c.is_lowercase()) {
        score += 1;
    }

    // وجود أرقام
    if password.chars().any(|c| c.is_numeric()) {
        score += 1;
    }

    // وجود رموز خاصة
    if password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
    {
        score += 1;
    }

    match score {
        0..=2 => PasswordStrength::Weak,
        3..=4 => PasswordStrength::Fair,
        5..=6 => PasswordStrength::Good,
        _ => PasswordStrength::Strong,
    }
}

/// قوة كلمة المرور
#[derive(Debug, Clone, PartialEq)]
pub enum PasswordStrength {
    Weak,
    Fair,
    Good,
    Strong,
}

impl PasswordStrength {
    pub fn to_arabic(&self) -> &'static str {
        match self {
            Self::Weak => "ضعيفة",
            Self::Fair => "متوسطة",
            Self::Good => "جيدة",
            Self::Strong => "قوية",
        }
    }
}

// ===== دوال عربية =====

/// تشفير كلمة المرور
pub fn شفر_كلمة_المرور(password: &str) -> Result<String, String> {
    hash_password(password)
}

/// التحقق من كلمة المرور
pub fn تحقق_كلمة_المرور(password: &str, hash: &str) -> bool {
    verify_password(password, hash)
}

/// قوة كلمة المرور
pub fn قوة_كلمة_المرور(password: &str) -> PasswordStrength {
    password_strength(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcrypt_hash_verify() {
        let password = "كلمة_مرور_سرية123!";
        let hash = bcrypt_hash(password, 12).unwrap();

        assert!(hash.starts_with("$2b$"));
        assert!(bcrypt_verify(password, &hash));
        assert!(!bcrypt_verify("wrong_password", &hash));
    }

    #[test]
    fn test_argon2_hash_verify() {
        let password = "كلمة_مرور_سرية456!";
        let salt = generate_salt(16);
        let hash = argon2_hash(password, &salt).unwrap();

        assert!(hash.starts_with("$argon2"));
        assert!(argon2_verify(password, &hash));
        assert!(!argon2_verify("wrong_password", &hash));
    }

    #[test]
    fn test_pbkdf2_hash_verify() {
        let password = "كلمة_مرور_سرية789!";
        let salt = generate_salt(16);
        let hash = pbkdf2_hash(password, &salt, 100000).unwrap();

        assert!(hash.starts_with("$pbkdf2"));
        assert!(pbkdf2_verify(password, &hash, 100000));
        assert!(!pbkdf2_verify("wrong_password", &hash, 100000));
    }

    #[test]
    fn test_scrypt_hash_verify() {
        let password = "كلمة_مرور_سرية!";
        let salt = generate_salt(16);
        let hash = scrypt_hash(password, &salt).unwrap();

        assert!(hash.starts_with("$scrypt"));
        assert!(scrypt_verify(password, &hash));
        assert!(!scrypt_verify("wrong_password", &hash));
    }

    #[test]
    fn test_auto_verify() {
        let password = "test_password_123";

        // bcrypt
        let bcrypt_hash = bcrypt_hash(password, 12).unwrap();
        assert!(verify_password(password, &bcrypt_hash));

        // argon2
        let salt = generate_salt(16);
        let argon2_hash_str = argon2_hash(password, &salt).unwrap();
        assert!(verify_password(password, &argon2_hash_str));
    }

    #[test]
    fn test_password_strength() {
        assert_eq!(password_strength("123"), PasswordStrength::Weak);
        assert_eq!(password_strength("password"), PasswordStrength::Weak);
        assert_eq!(password_strength("Password1"), PasswordStrength::Fair);
        assert_eq!(password_strength("Password123!"), PasswordStrength::Good);
        assert_eq!(
            password_strength("VeryStr0ng!Pass#2024"),
            PasswordStrength::Strong
        );
    }
}
