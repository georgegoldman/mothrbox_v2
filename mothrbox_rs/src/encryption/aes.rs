use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key,
};
use argon2::{
    Argon2, 
    password_hash::SaltString,
};
use rand::RngCore;
use std::fs;

/// AES-256-GCM Encryption with Argon2 Key Derivation
pub struct AESEncryption;

impl AESEncryption {
    /// Encrypt data using AES-256-GCM
    /// Uses Argon2 to derive key from password
    pub fn encrypt(plaintext: &[u8], password: &str) -> Result<Vec<u8>, String> {
        // 1. Generate random salt for Argon2
        let salt = SaltString::generate(&mut OsRng);
        
        // 2. Derive 256-bit key from password using Argon2
        let argon2 = Argon2::default();
        let mut key_bytes = [0u8; 32];
        
        argon2.hash_password_into(
            password.as_bytes(),
            salt.as_str().as_bytes(),
            &mut key_bytes
        ).map_err(|e| format!("Key derivation failed: {}", e))?;
        
        // 3. Create AES-256-GCM cipher
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        // 4. Generate random 96-bit nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // 5. Encrypt the plaintext
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption failed: {}", e))?;
        
        // 6. Format: [salt_len(1)][salt][nonce(12)][ciphertext+tag]
        let salt_bytes = salt.as_str().as_bytes();
        let mut result = Vec::new();
        result.push(salt_bytes.len() as u8);
        result.extend_from_slice(salt_bytes);
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }
    
    /// Decrypt data using AES-256-GCM
    pub fn decrypt(encrypted_data: &[u8], password: &str) -> Result<Vec<u8>, String> {
        if encrypted_data.len() < 30 {
            return Err("Encrypted data too short".to_string());
        }
        
        let mut offset = 0;
        
        // 1. Extract salt
        let salt_len = encrypted_data[0] as usize;
        offset += 1;
        
        if encrypted_data.len() < offset + salt_len {
            return Err("Invalid salt length".to_string());
        }
        
        let salt_bytes = &encrypted_data[offset..offset + salt_len];
        offset += salt_len;
        
        // 2. Extract nonce (12 bytes)
        if encrypted_data.len() < offset + 12 {
            return Err("Invalid nonce".to_string());
        }
        let nonce_bytes = &encrypted_data[offset..offset + 12];
        let nonce = Nonce::from_slice(nonce_bytes);
        offset += 12;
        
        // 3. Extract ciphertext
        let ciphertext = &encrypted_data[offset..];
        
        // 4. Derive key from password using same salt
        let argon2 = Argon2::default();
        let mut key_bytes = [0u8; 32];
        
        argon2.hash_password_into(
            password.as_bytes(),
            salt_bytes,
            &mut key_bytes
        ).map_err(|e| format!("Key derivation failed: {}", e))?;
        
        // 5. Create cipher and decrypt
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed (wrong password or corrupted data): {}", e))?;
        
        Ok(plaintext)
    }
    
    /// Encrypt a file
    pub fn encrypt_file(input_path: &str, output_path: &str, password: &str) -> std::io::Result<()> {
        let plaintext = fs::read(input_path)?;
        let encrypted = Self::encrypt(&plaintext, password)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(output_path, encrypted)?;
        Ok(())
    }
    
    /// Decrypt a file
    pub fn decrypt_file(input_path: &str, output_path: &str, password: &str) -> std::io::Result<()> {
        let encrypted = fs::read(input_path)?;
        let plaintext = Self::decrypt(&encrypted, password)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(output_path, plaintext)?;
        Ok(())
    }
    
    /// Generate a random key (for advanced users who want to manage keys themselves)
    pub fn generate_random_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        key
    }
    
    /// Encrypt with raw key (no password derivation)
    pub fn encrypt_with_key(plaintext: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
        let cipher_key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(cipher_key);
        
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption failed: {}", e))?;
        
        // Format: [nonce(12)][ciphertext+tag]
        let mut result = Vec::new();
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }
    
    /// Decrypt with raw key (no password derivation)
    pub fn decrypt_with_key(encrypted_data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
        if encrypted_data.len() < 28 { // 12 nonce + 16 tag minimum
            return Err("Encrypted data too short".to_string());
        }
        
        let nonce = Nonce::from_slice(&encrypted_data[0..12]);
        let ciphertext = &encrypted_data[12..];
        
        let cipher_key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(cipher_key);
        
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        
        Ok(plaintext)
    }
}