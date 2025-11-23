use p256::{
    ecdh::EphemeralSecret,
    PublicKey, SecretKey,
    elliptic_curve::sec1::ToEncodedPoint,
};
use sha2::{Sha256, Digest};
use hkdf::Hkdf;
use aes::Aes256;
use ctr::cipher::{KeyIvInit, StreamCipher};
use hmac::{Hmac, Mac};
use rand::RngCore;
use aes::cipher::generic_array::GenericArray;

type HmacSha256 = Hmac<Sha256>;
type Aes256Ctr = ctr::Ctr128BE<Aes256>;

/// ECIES (Elliptic Curve Integrated Encryption Scheme) Implementation
/// Uses P-256 curve with AES-256-CTR and HMAC-SHA256
pub struct ECCEncryption;

impl ECCEncryption {
    /// Encrypt data using ECIES
    pub fn encrypt(plaintext: &[u8], recipient_public_key: &PublicKey) -> Result<Vec<u8>, String> {
        // 1. Generate ephemeral key pair
        let ephemeral_secret = EphemeralSecret::random(&mut rand::thread_rng());
        let ephemeral_public = ephemeral_secret.public_key();
        
        // 2. Perform ECDH to get shared secret
        let shared_secret = ephemeral_secret.diffie_hellman(recipient_public_key);
        
        // 3. Derive encryption and MAC keys using HKDF
        let hk = Hkdf::<Sha256>::new(None, shared_secret.raw_secret_bytes());
        let mut key_material = [0u8; 64]; // 32 bytes for AES + 32 bytes for HMAC
        hk.expand(b"ecies-encryption", &mut key_material)
            .map_err(|e| format!("HKDF error: {}", e))?;
        
        let enc_key = &key_material[0..32];
        let mac_key = &key_material[32..64];
        
        // 4. Generate random IV for AES-CTR
        let mut iv = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut iv);
        
        // 5. Encrypt with AES-256-CTR
        let mut ciphertext = plaintext.to_vec();
        let mut cipher = Aes256Ctr::new(
            GenericArray::from_slice(enc_key),
            GenericArray::from_slice(&iv)
        );
        cipher.apply_keystream(&mut ciphertext);
        
        // 6. Create HMAC over ephemeral public key + IV + ciphertext
        let ephemeral_public_bytes = ephemeral_public.to_encoded_point(false);
        let mut mac = HmacSha256::new_from_slice(mac_key)
            .map_err(|e| format!("HMAC error: {}", e))?;
        
        mac.update(ephemeral_public_bytes.as_bytes());
        mac.update(&iv);
        mac.update(&ciphertext);
        let tag = mac.finalize().into_bytes();
        
        // 7. Format: [ephemeral_public_key_len(2)][ephemeral_public_key][iv(16)][ciphertext][mac(32)]
        let mut result = Vec::new();
        let pub_key_bytes = ephemeral_public_bytes.as_bytes();
        result.extend_from_slice(&(pub_key_bytes.len() as u16).to_be_bytes());
        result.extend_from_slice(pub_key_bytes);
        result.extend_from_slice(&iv);
        result.extend_from_slice(&ciphertext);
        result.extend_from_slice(&tag);
        
        Ok(result)
    }
    
    /// Decrypt data using ECIES
    pub fn decrypt(encrypted_data: &[u8], private_key: &[u8; 32]) -> Result<Vec<u8>, String> {
        if encrypted_data.len() < 50 {
            return Err("Encrypted data too short".to_string());
        }
        
        let mut offset = 0;
        
        // 1. Extract ephemeral public key length
        let pub_key_len = u16::from_be_bytes([encrypted_data[0], encrypted_data[1]]) as usize;
        offset += 2;
        
        // 2. Extract ephemeral public key
        if encrypted_data.len() < offset + pub_key_len {
            return Err("Invalid ephemeral public key".to_string());
        }
        let ephemeral_public_bytes = &encrypted_data[offset..offset + pub_key_len];
        let ephemeral_public = PublicKey::from_sec1_bytes(ephemeral_public_bytes)
            .map_err(|e| format!("Invalid public key: {}", e))?;
        offset += pub_key_len;
        
        // 3. Extract IV (16 bytes)
        let iv = &encrypted_data[offset..offset + 16];
        offset += 16;
        
        // 4. Extract MAC (last 32 bytes)
        if encrypted_data.len() < offset + 32 {
            return Err("Invalid MAC".to_string());
        }
        let mac_tag = &encrypted_data[encrypted_data.len() - 32..];
        let ciphertext = &encrypted_data[offset..encrypted_data.len() - 32];
        
        // 5. Recover private key and perform ECDH
        let secret_key = SecretKey::from_slice(private_key)
            .map_err(|e| format!("Invalid private key: {}", e))?;
        
        let shared_secret = p256::ecdh::diffie_hellman(
            secret_key.to_nonzero_scalar(),
            ephemeral_public.as_affine(),
        );
        
        // 6. Derive encryption and MAC keys using HKDF
        let hk = Hkdf::<Sha256>::new(None, shared_secret.raw_secret_bytes());
        let mut key_material = [0u8; 64];
        hk.expand(b"ecies-encryption", &mut key_material)
            .map_err(|e| format!("HKDF error: {}", e))?;
        
        let enc_key = &key_material[0..32];
        let mac_key = &key_material[32..64];
        
        // 7. Verify HMAC
        let mut mac = HmacSha256::new_from_slice(mac_key)
            .map_err(|e| format!("HMAC error: {}", e))?;
        mac.update(ephemeral_public_bytes);
        mac.update(iv);
        mac.update(ciphertext);
        
        mac.verify_slice(mac_tag)
            .map_err(|_| "MAC verification failed - data may be tampered")?;
        
        // 8. Decrypt with AES-256-CTR
        let mut plaintext = ciphertext.to_vec();
        let mut cipher = Aes256Ctr::new(
            GenericArray::from_slice(enc_key),
            GenericArray::from_slice(iv)
        );
        cipher.apply_keystream(&mut plaintext);
        
        Ok(plaintext)
    }
    
    /// Generate a new ECC key pair
    pub fn generate_keypair() -> ([u8; 32], PublicKey) {
        let secret = SecretKey::random(&mut rand::thread_rng());
        let public = secret.public_key();
        (*secret.to_bytes().as_ref(), public)
    }
    
    /// Save encrypted data to file
    pub fn save_to_file(data: &[u8], filename: &str) -> std::io::Result<()> {
        std::fs::write(filename, data)
    }
    
    /// Load encrypted data from file
    pub fn load_from_file(filename: &str) -> std::io::Result<Vec<u8>> {
        std::fs::read(filename)
    }
}