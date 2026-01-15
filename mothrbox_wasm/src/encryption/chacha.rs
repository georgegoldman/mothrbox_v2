use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

pub fn encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes".into());
    }

    let cipher = XChaCha20Poly1305::new_from_slice(key).map_err(|e| e.to_string())?;

    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng); // 24-byte nonce
    let ciphertext = cipher.encrypt(&nonce, data).map_err(|e| e.to_string())?;

    // Prepend nonce to ciphertext
    let mut result = nonce.to_vec();
    result.extend(ciphertext);
    Ok(result)
}
