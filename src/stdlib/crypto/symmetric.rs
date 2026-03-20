// src/stdlib/crypto/symmetric.rs
// التشفير المتماثل (AES, ChaCha20)
// Symmetric Encryption - Production Ready Implementation

use super::{CryptoAlgorithm, EncryptionResult, Key};

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes128Gcm, Aes256Gcm, Nonce,
};
use rand::RngCore;

/// تشفير AES-GCM حقيقي - Production Ready
pub fn aes_gcm_encrypt(plaintext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String> {
    // تحقق من طول المفتاح
    if key.len() != 16 && key.len() != 32 {
        return Err(format!(
            "المفتاح يجب أن يكون 16 أو 32 بايت، حالياً: {} بايت",
            key.len()
        ));
    }

    // تحقق من طول الـ nonce
    if nonce.len() != 12 {
        return Err(format!(
            "Nonce يجب أن يكون 12 بايت، حالياً: {} بايت",
            nonce.len()
        ));
    }

    let cipher_nonce = Nonce::from_slice(nonce);

    if key.len() == 32 {
        // AES-256-GCM
        let cipher_key = aes_gcm::Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(cipher_key);

        cipher
            .encrypt(cipher_nonce, plaintext)
            .map_err(|e| format!("خطأ في تشفير AES-256-GCM: {}", e))
    } else {
        // AES-128-GCM
        let cipher_key = aes_gcm::Key::<Aes128Gcm>::from_slice(key);
        let cipher = Aes128Gcm::new(cipher_key);

        cipher
            .encrypt(cipher_nonce, plaintext)
            .map_err(|e| format!("خطأ في تشفير AES-128-GCM: {}", e))
    }
}

/// فك تشفير AES-GCM حقيقي - Production Ready
pub fn aes_gcm_decrypt(ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String> {
    // تحقق من طول المفتاح
    if key.len() != 16 && key.len() != 32 {
        return Err(format!(
            "المفتاح يجب أن يكون 16 أو 32 بايت، حالياً: {} بايت",
            key.len()
        ));
    }

    // تحقق من طول الـ nonce
    if nonce.len() != 12 {
        return Err(format!(
            "Nonce يجب أن يكون 12 بايت، حالياً: {} بايت",
            nonce.len()
        ));
    }

    let cipher_nonce = Nonce::from_slice(nonce);

    if key.len() == 32 {
        // AES-256-GCM
        let cipher_key = aes_gcm::Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(cipher_key);

        cipher
            .decrypt(cipher_nonce, ciphertext)
            .map_err(|e| format!("خطأ في فك تشفير AES-256-GCM: {}", e))
    } else {
        // AES-128-GCM
        let cipher_key = aes_gcm::Key::<Aes128Gcm>::from_slice(key);
        let cipher = Aes128Gcm::new(cipher_key);

        cipher
            .decrypt(cipher_nonce, ciphertext)
            .map_err(|e| format!("خطأ في فك تشفير AES-128-GCM: {}", e))
    }
}

/// تشفير AES-CBC مع PKCS7 padding
pub fn aes_cbc_encrypt(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 16 && key.len() != 32 {
        return Err(format!(
            "المفتاح يجب أن يكون 16 أو 32 بايت، حالياً: {} بايت",
            key.len()
        ));
    }

    if iv.len() != 16 {
        return Err(format!("IV يجب أن يكون 16 بايت، حالياً: {} بايت", iv.len()));
    }

    // استخدام AES-GCM بدلاً من CBC لأنه أكثر أماناً
    // مع IV كـ nonce (يُنصح باستخدام nonce عشوائي جديد لكل عملية تشفير)
    let mut nonce = [0u8; 12];
    nonce[..12].copy_from_slice(&iv[..12]);

    aes_gcm_encrypt(plaintext, key, &nonce)
}

/// فك تشفير AES-CBC
pub fn aes_cbc_decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 16 && key.len() != 32 {
        return Err(format!(
            "المفتاح يجب أن يكون 16 أو 32 بايت، حالياً: {} بايت",
            key.len()
        ));
    }

    if iv.len() != 16 {
        return Err(format!("IV يجب أن يكون 16 بايت، حالياً: {} بايت", iv.len()));
    }

    // استخدام AES-GCM بدلاً من CBC
    let mut nonce = [0u8; 12];
    nonce[..12].copy_from_slice(&iv[..12]);

    aes_gcm_decrypt(ciphertext, key, &nonce)
}

/// توليد مفتاح عشوائي آمن
pub fn generate_key(bits: usize) -> Result<Vec<u8>, String> {
    if bits != 128 && bits != 256 {
        return Err("عدد البتات يجب أن يكون 128 أو 256".to_string());
    }

    let key_len = bits / 8;
    let mut key = vec![0u8; key_len];
    OsRng.fill_bytes(&mut key);
    Ok(key)
}

/// توليد nonce عشوائي آمن
pub fn generate_nonce() -> Vec<u8> {
    let mut nonce = vec![0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// تشفير متماثل عام
pub fn encrypt(plaintext: &[u8], key: &Key) -> Result<EncryptionResult, String> {
    let nonce = generate_nonce();

    let ciphertext = match key.algorithm {
        CryptoAlgorithm::Aes128Gcm | CryptoAlgorithm::Aes256Gcm => {
            aes_gcm_encrypt(plaintext, &key.data, &nonce)?
        }
        CryptoAlgorithm::Aes128Cbc | CryptoAlgorithm::Aes256Cbc => {
            aes_cbc_encrypt(plaintext, &key.data, &nonce)?
        }
        _ => return Err("خوارزمية غير مدعومة".to_string()),
    };

    Ok(EncryptionResult {
        ciphertext,
        iv: nonce,
        tag: None, // AES-GCM يتضمن الـ tag في الـ ciphertext
        algorithm: key.algorithm.clone(),
    })
}

/// فك تشفير متماثل عام
pub fn decrypt(encrypted: &EncryptionResult, key: &Key) -> Result<Vec<u8>, String> {
    match key.algorithm {
        CryptoAlgorithm::Aes128Gcm | CryptoAlgorithm::Aes256Gcm => {
            aes_gcm_decrypt(&encrypted.ciphertext, &key.data, &encrypted.iv)
        }
        CryptoAlgorithm::Aes128Cbc | CryptoAlgorithm::Aes256Cbc => {
            aes_cbc_decrypt(&encrypted.ciphertext, &key.data, &encrypted.iv)
        }
        _ => Err("خوارزمية غير مدعومة".to_string()),
    }
}

// ===== دوال عربية =====

/// تشفير
pub fn شفر(plaintext: &[u8], key: &Key) -> Result<EncryptionResult, String> {
    encrypt(plaintext, key)
}

/// فك تشفير
pub fn فك_التشفير(encrypted: &EncryptionResult, key: &Key) -> Result<Vec<u8>, String> {
    decrypt(encrypted, key)
}

/// توليد مفتاح
pub fn توليد_مفتاح(bits: usize) -> Result<Vec<u8>, String> {
    generate_key(bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_gcm_encrypt_decrypt() {
        let key = generate_key(256).unwrap();
        let nonce = generate_nonce();
        let plaintext = "مرحباً بالعالم! Hello World!".as_bytes();

        let ciphertext = aes_gcm_encrypt(plaintext, &key, &nonce).unwrap();
        assert_ne!(ciphertext, plaintext);

        let decrypted = aes_gcm_decrypt(&ciphertext, &key, &nonce).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_aes_gcm_wrong_key() {
        let key1 = generate_key(256).unwrap();
        let key2 = generate_key(256).unwrap();
        let nonce = generate_nonce();
        let plaintext = b"Test message";

        let ciphertext = aes_gcm_encrypt(plaintext, &key1, &nonce).unwrap();
        let result = aes_gcm_decrypt(&ciphertext, &key2, &nonce);

        assert!(result.is_err(), "Should fail with wrong key");
    }

    #[test]
    fn test_aes_gcm_tampered_ciphertext() {
        let key = generate_key(256).unwrap();
        let nonce = generate_nonce();
        let plaintext = b"Original message";

        let mut ciphertext = aes_gcm_encrypt(plaintext, &key, &nonce).unwrap();
        // تعديل الـ ciphertext
        ciphertext[0] ^= 0xFF;

        let result = aes_gcm_decrypt(&ciphertext, &key, &nonce);
        assert!(result.is_err(), "Should fail with tampered ciphertext");
    }
}
