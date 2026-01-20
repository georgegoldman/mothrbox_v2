use mothrbox_engine::{
    ecc,
    encryption::{
        aes, chacha,
        ecc::{MothrboxEccPublicKey, ToEncodedPoint},
    },
};
use wasm_bindgen::prelude::*;

// define this struct or js
#[wasm_bindgen]
pub struct KeyPairResult {
    // We use Vec<u8> because wasm-bindgen turns this into a JS Uint8Array automatically
    private: Vec<u8>,
    public: Vec<u8>,
}

// Add getters so JavaScript can access the fields
#[wasm_bindgen]
impl KeyPairResult {
    #[wasm_bindgen(getter)]
    pub fn private_key(&self) -> Vec<u8> {
        self.private.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.public.clone()
    }
}

#[wasm_bindgen]
pub fn aes_encrypt(plaintext: &[u8], password: &str) -> Result<Vec<u8>, String> {
    match aes::AESEncryption::encrypt(plaintext, password) {
        Ok(ciphertext) => Ok(ciphertext),
        Err(err) => Err(err.to_string()),
    }
}

#[wasm_bindgen]
pub fn aes_decrypt(ciphertext: &[u8], password: &str) -> Result<Vec<u8>, String> {
    match aes::AESEncryption::decrypt(ciphertext, password) {
        Ok(plaintext) => Ok(plaintext),
        Err(err) => Err(err.to_string()),
    }
}

#[wasm_bindgen]
pub fn chacha_encrypt(plaintext: &[u8], password: &str) -> Result<Vec<u8>, String> {
    match chacha::ChaChaEncryption::encrypt(plaintext, password) {
        Ok(plaintext) => Ok(plaintext),
        Err(err) => Err(err.to_string()),
    }
}

#[wasm_bindgen]
pub fn chacha_decrypt(ciphertext: &[u8], password: &str) -> Result<Vec<u8>, String> {
    match chacha::ChaChaEncryption::decrypt(ciphertext, password) {
        Ok(plaintext) => Ok(plaintext),
        Err(err) => Err(err.to_string()),
    }
}

#[wasm_bindgen]
pub fn ecc_generate_key() -> KeyPairResult {
    // Call your original function
    let (priv_array, pub_key_struct) = ecc::ECCEncryption::generate_keypair();

    // A. Convert [u8; 32] private key to Vec<u8>
    let private_vec = priv_array.to_vec();

    // B. Convert the P256 Public Key struct to bytes (SEC1 encoding)
    // false = compressed format (33 bytes), true = uncompressed (65 bytes)
    let public_vec = pub_key_struct.to_encoded_point(false).as_bytes().to_vec();

    // Return the wrapper struct
    KeyPairResult {
        private: private_vec,
        public: public_vec,
    }
}

#[wasm_bindgen]
pub fn ecc_encrypt(
    plaintext: &[u8],
    recipient_public_key_bytes: &[u8],
    sender_private_key_bytes: &[u8],
) -> Result<Vec<u8>, JsError> {
    // 1. Convert JS bytes back into a Rust Public Key
    let recipient_key = MothrboxEccPublicKey::from_sec1_bytes(recipient_public_key_bytes)
        .map_err(|_| JsError::new("Invalid recipient public key format"))?;

    // 2. Validate Private Key Length (Must be 32 bytes)
    let my_secret_array: [u8; 32] = sender_private_key_bytes
        .try_into()
        .map_err(|_| JsError::new("Private key must be exactly 32 bytes"))?;

    // 3. Call your Rust logic (the function we fixed earlier)
    let encrypted_data =
        ecc::ECCEncryption::encrypt_authenticated(plaintext, &recipient_key, &my_secret_array)
            .map_err(|e| JsError::new(&e))?; // Convert String error to JS Error

    // 4. Return bytes (automatically becomes Uint8Array in JS)
    Ok(encrypted_data)
}

#[wasm_bindgen]
pub fn ecc_decrypt(encrypted_data: &[u8], my_private_key_bytes: &[u8]) -> Result<Vec<u8>, JsError> {
    // 1. Validate Private Key Length
    let my_secret_array: [u8; 32] = my_private_key_bytes
        .try_into()
        .map_err(|_| JsError::new("Private key must be exactly 32 bytes"))?;

    // 2. Call the new authenticated decrypt function
    let decrypted_data =
        ecc::ECCEncryption::decrypt_authenticated(encrypted_data, &my_secret_array)
            .map_err(|e| JsError::new(&e))?;
    Ok(decrypted_data)
}
