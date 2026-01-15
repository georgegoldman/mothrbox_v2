use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

pub fn encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes".into());
    }

    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| e.to_string())?;

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 12-byte nonce
    let ciphertext = cipher.encrypt(&nonce, data).map_err(|e| e.to_string())?;

    // Prepend nonce
    let mut result = nonce.to_vec();
    result.extend(ciphertext);
    Ok(result)
}
