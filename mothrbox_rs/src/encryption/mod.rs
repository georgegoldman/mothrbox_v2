// src/encryption/mod.rs
pub mod ecc;
pub mod aes;
pub mod chacha;

// Re-export main structs for easier access
pub use ecc::ECCEncryption;
pub use aes::AESEncryption;
pub use chacha::ChaChaEncryption;