// lib.rs - Library interface for MothrBox encryption

pub mod encryption;

// Re-export for convenience
pub use encryption::{aes, chacha, ecc};

// Unified encryption interface
pub fn encrypt_file(
    input_path: &str,
    output_path: &str,
    password: &str,
    algorithm: &str,
) -> Result<(), String> {
    match algorithm {
        "aes" => aes::encrypt_file(input_path, output_path, password),
        "chacha" => chacha::encrypt_file(input_path, output_path, password),
        _ => Err("Invalid algorithm (use 'aes' or 'chacha')".into()),
    }
}

pub fn decrypt_file(
    input_path: &str,
    output_path: &str,
    password: &str,
    algorithm: &str,
) -> Result<(), String> {
    match algorithm {
        "aes" => aes::decrypt_file(input_path, output_path, password),
        "chacha" => chacha::decrypt_file(input_path, output_path, password),
        _ => Err("Invalid algorithm (use 'aes' or 'chacha')".into()),
    }
}

// ECC has different signature (uses key files instead of passwords)
pub fn encrypt_file_ecc(
    input_path: &str,
    output_path: &str,
    public_key_path: &str,
) -> Result<(), String> {
    ecc::encrypt_file(input_path, output_path, public_key_path)
}

pub fn decrypt_file_ecc(
    input_path: &str,
    output_path: &str,
    private_key_path: &str,
) -> Result<(), String> {
    ecc::decrypt_file(input_path, output_path, private_key_path)
}