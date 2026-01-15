use getrandom::getrandom;
use wasm_bindgen::prelude::*;
// use mothrbox::your_encryption_logic; // Import your existing logic if compatible

#[wasm_bindgen]
pub struct EncryptedResult {
    // We return a struct to JS so we can separate the nonce/salt if needed
    data: Vec<u8>,
    nonce: Vec<u8>,
}

#[wasm_bindgen]
impl EncryptedResult {
    // Getters for JS to access the fields
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
    pub fn get_nonce(&self) -> Vec<u8> {
        self.nonce.clone()
    }
}

#[wasm_bindgen]
pub fn encrypt_blob(file_data: &[u8], user_password: &str) -> Result<EncryptedResult, JsError> {
    // 1. Generate a secure random salt/nonce inside WASM
    let mut nonce = [0u8; 12];
    getrandom(&mut nonce).map_err(|e| JsError::new(&e.to_string()))?;

    // 2. Perform Encryption (Simulated Mothrbox Logic)
    // In reality, call your actual mothrbox::encrypt(file_data, password, nonce) here.
    // This logic happens inside the WASM "black box" memory.
    let encrypted_data = mock_encrypt(file_data, user_password, &nonce);

    Ok(EncryptedResult {
        data: encrypted_data,
        nonce: nonce.to_vec(),
    })
}

// Mock function - Replace with your actual Mothrbox encryption
fn mock_encrypt(data: &[u8], key: &str, nonce: &[u8]) -> Vec<u8> {
    // XOR cipher just for demo purposes
    data.iter().map(|b| b ^ 0xFF).collect()
}
