use aes::cipher::generic_array::GenericArray;
use aes::Aes256;
use ctr::cipher::{KeyIvInit, StreamCipher};
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
pub use p256::elliptic_curve::sec1::ToEncodedPoint;
use p256::{ecdh::EphemeralSecret, PublicKey, SecretKey};
use rand::RngCore;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;
type Aes256Ctr = ctr::Ctr128BE<Aes256>;

pub type MothrboxEccPublicKey = PublicKey;

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
            GenericArray::from_slice(&iv),
        );
        cipher.apply_keystream(&mut ciphertext);

        // 6. Create HMAC over ephemeral public key + IV + ciphertext
        let ephemeral_public_bytes = ephemeral_public.to_encoded_point(false);
        let mut mac =
            HmacSha256::new_from_slice(mac_key).map_err(|e| format!("HMAC error: {}", e))?;

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
        let mut mac =
            HmacSha256::new_from_slice(mac_key).map_err(|e| format!("HMAC error: {}", e))?;
        mac.update(ephemeral_public_bytes);
        mac.update(iv);
        mac.update(ciphertext);

        mac.verify_slice(mac_tag)
            .map_err(|_| "MAC verification failed - data may be tampered")?;

        // 8. Decrypt with AES-256-CTR
        let mut plaintext = ciphertext.to_vec();
        let mut cipher = Aes256Ctr::new(
            GenericArray::from_slice(enc_key),
            GenericArray::from_slice(iv),
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

    pub fn encrypt_authenticated(
        plaintext: &[u8],
        recipient_public_key: &PublicKey,
        my_secret_bytes: &[u8; 32], // <--- Input your private key here
    ) -> Result<Vec<u8>, String> {
        // 1. Reconstruct YOUR SecretKey from the raw bytes
        let sender_secret = p256::SecretKey::from_slice(my_secret_bytes)
            .map_err(|_| "Invalid secret key bytes provided".to_string())?;

        // 2. Perform ECDH: (Your Private Key) + (Their Public Key)
        // We use the low-level diffie_hellman function for static keys
        let shared_secret = p256::elliptic_curve::ecdh::diffie_hellman(
            sender_secret.to_nonzero_scalar(),
            recipient_public_key.as_affine(),
        );

        // 3. Derive encryption keys (Same as before)
        let hk = Hkdf::<Sha256>::new(None, shared_secret.raw_secret_bytes());
        let mut key_material = [0u8; 64];
        hk.expand(b"ecies-encryption", &mut key_material)
            .map_err(|e| format!("HKDF error: {}", e))?;

        let enc_key = &key_material[0..32];
        let mac_key = &key_material[32..64];

        // 4. Generate random IV (Same as before)
        let mut iv = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut iv);

        // 5. Encrypt (Same as before)
        let mut ciphertext = plaintext.to_vec();
        let mut cipher = Aes256Ctr::new(
            GenericArray::from_slice(enc_key),
            GenericArray::from_slice(&iv),
        );
        cipher.apply_keystream(&mut ciphertext);

        // 6. PREPARE HEADER: Use YOUR Public Key
        // Crucial Step: We attach YOUR public key instead of a random one.
        // The recipient needs this to calculate the shared secret on their end.
        let sender_public_key = sender_secret.public_key();
        let sender_public_bytes = sender_public_key.to_encoded_point(false);

        // 7. MAC (Same as before, but over YOUR public key bytes)
        let mut mac =
            HmacSha256::new_from_slice(mac_key).map_err(|e| format!("HMAC error: {}", e))?;

        mac.update(sender_public_bytes.as_bytes());
        mac.update(&iv);
        mac.update(&ciphertext);
        let tag = mac.finalize().into_bytes();

        // 8. Pack Result
        let mut result = Vec::new();
        let pub_key_bytes = sender_public_bytes.as_bytes();
        result.extend_from_slice(&(pub_key_bytes.len() as u16).to_be_bytes());
        result.extend_from_slice(pub_key_bytes); // <--- This is now YOUR key
        result.extend_from_slice(&iv);
        result.extend_from_slice(&ciphertext);
        result.extend_from_slice(&tag);

        Ok(result)
    }

    pub fn decrypt_authenticated(
        encrypted_data: &[u8],
        my_secret_bytes: &[u8; 32],
    ) -> Result<Vec<u8>, String> {
        // 1. Reconstruct YOUR Secret Key
        let my_secret = SecretKey::from_slice(my_secret_bytes)
            .map_err(|_| "Invalid private key bytes".to_string())?;

        // --- PARSE THE PACKET ---
        // Minimal length check: 2 (len) + 33 (min pubkey) + 16 (iv) + 32 (tag) = 83 bytes
        if encrypted_data.len() < 83 {
            return Err("Message too short".to_string());
        }

        let mut offset = 0;

        // A. Read Sender Public Key Length (2 bytes)
        let pub_key_len =
            u16::from_be_bytes(encrypted_data[offset..offset + 2].try_into().unwrap()) as usize;
        offset += 2;

        // B. Read Sender Public Key
        if offset + pub_key_len > encrypted_data.len() {
            return Err("Malformed packet: invalid public key length".to_string());
        }
        let sender_pub_key_bytes = &encrypted_data[offset..offset + pub_key_len];
        let sender_public_key = PublicKey::from_sec1_bytes(sender_pub_key_bytes)
            .map_err(|_| "Invalid sender public key".to_string())?;
        offset += pub_key_len;

        // C. Read IV (16 bytes)
        let iv_bytes = &encrypted_data[offset..offset + 16];
        offset += 16;

        // D. Read Ciphertext (Variable length)
        // The tag is always at the end (32 bytes), so ciphertext is everything between IV and Tag
        let ciphertext_len = encrypted_data.len() - offset - 32;
        let ciphertext_bytes = &encrypted_data[offset..offset + ciphertext_len];
        offset += ciphertext_len;

        // E. Read Tag (32 bytes)
        let received_tag = &encrypted_data[offset..];

        // --- CRYPTO OPERATIONS ---

        // 2. Perform ECDH: (My Private) + (Sender Public)
        // This regenerates the exact same shared secret created during encryption
        let shared_secret = p256::elliptic_curve::ecdh::diffie_hellman(
            my_secret.to_nonzero_scalar(),
            sender_public_key.as_affine(),
        );

        // 3. Derive Keys (HKDF)
        let hk = Hkdf::<Sha256>::new(None, shared_secret.raw_secret_bytes());
        let mut key_material = [0u8; 64];
        hk.expand(b"ecies-encryption", &mut key_material)
            .map_err(|e| format!("HKDF error: {}", e))?;

        let enc_key = &key_material[0..32];
        let mac_key = &key_material[32..64];

        // 4. Verify MAC (Authentication)
        // We must re-calculate the MAC over [SenderPubKey + IV + Ciphertext] and compare
        let mut mac =
            HmacSha256::new_from_slice(mac_key).map_err(|e| format!("HMAC error: {}", e))?;

        mac.update(sender_pub_key_bytes); // The bytes we extracted from the packet
        mac.update(iv_bytes);
        mac.update(ciphertext_bytes);

        // Verify ensures constant-time comparison to prevent timing attacks
        mac.verify_slice(received_tag)
            .map_err(|_| "Decryption failed: Integrity check failed (MAC mismatch)".to_string())?;

        // 5. Decrypt (AES-CTR)
        // AES-CTR is symmetric: applying the keystream again decrypts it
        let mut plaintext = ciphertext_bytes.to_vec();
        let mut cipher = Aes256Ctr::new(
            GenericArray::from_slice(enc_key),
            GenericArray::from_slice(iv_bytes),
        );
        cipher.apply_keystream(&mut plaintext);

        Ok(plaintext)
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
use std::fs;

// File operation functions
pub fn generate_keypair(private_key_path: &str, public_key_path: &str) -> Result<(), String> {
    let (private_key, public_key) = ECCEncryption::generate_keypair();

    fs::write(private_key_path, &private_key)
        .map_err(|e| format!("Failed to write private key: {}", e))?;

    let public_key_bytes = public_key.to_encoded_point(false);
    fs::write(public_key_path, public_key_bytes.as_bytes())
        .map_err(|e| format!("Failed to write public key: {}", e))?;

    Ok(())
}

pub fn encrypt_file(
    input_path: &str,
    output_path: &str,
    public_key_path: &str,
) -> Result<(), String> {
    let plaintext =
        fs::read(input_path).map_err(|e| format!("Failed to read input file: {}", e))?;

    let public_key_bytes =
        fs::read(public_key_path).map_err(|e| format!("Failed to read public key: {}", e))?;

    let public_key = PublicKey::from_sec1_bytes(&public_key_bytes)
        .map_err(|e| format!("Invalid public key: {}", e))?;

    let ciphertext = ECCEncryption::encrypt(&plaintext, &public_key)?;

    fs::write(output_path, ciphertext)
        .map_err(|e| format!("Failed to write output file: {}", e))?;

    Ok(())
}

pub fn decrypt_file(
    input_path: &str,
    output_path: &str,
    private_key_path: &str,
) -> Result<(), String> {
    let ciphertext =
        fs::read(input_path).map_err(|e| format!("Failed to read encrypted file: {}", e))?;

    let private_key_bytes =
        fs::read(private_key_path).map_err(|e| format!("Failed to read private key: {}", e))?;

    if private_key_bytes.len() != 32 {
        return Err("Invalid private key length".to_string());
    }

    let mut private_key = [0u8; 32];
    private_key.copy_from_slice(&private_key_bytes);

    let plaintext = ECCEncryption::decrypt(&ciphertext, &private_key)?;

    fs::write(output_path, plaintext).map_err(|e| format!("Failed to write output file: {}", e))?;

    Ok(())
}
