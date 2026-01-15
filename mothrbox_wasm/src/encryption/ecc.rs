use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead}; // Using AES for the data layer
use hkdf::Hkdf;
use p256::elliptic_curve::sec1::ToEncodedPoint;
use p256::{PublicKey, SecretKey, ecdh::EphemeralSecret};
use rand_core::OsRng;
use sha2::Sha256;

// Encrypt data TO a public key (User provides pubkey_hex)
pub fn encrypt_to_public_key(data: &[u8], recipient_pub_hex: &str) -> Result<Vec<u8>, String> {
    // 1. Parse Recipient Public Key
    let pub_bytes = hex::decode(recipient_pub_hex).map_err(|_| "Invalid Hex")?;
    let recipient_pk = PublicKey::from_sec1_bytes(&pub_bytes).map_err(|_| "Invalid PubKey")?;

    // 2. Generate Ephemeral Keypair
    let ephemeral_secret = EphemeralSecret::random(&mut OsRng);
    let ephemeral_public = ephemeral_secret.public_key();

    // 3. Perform ECDH to get shared secret
    let shared_secret = ephemeral_secret.diffie_hellman(&recipient_pk);

    // 4. Derive Symmetric Key (KDF)
    let shared_bytes = shared_secret.raw_secret_bytes();
    let hkdf = Hkdf::<Sha256>::new(None, &shared_bytes);
    let mut derived_key = [0u8; 32];
    hkdf.expand(b"mothrbox-ecies", &mut derived_key)
        .map_err(|_| "KDF failed")?;

    // 5. Encrypt Data with AES-GCM using derived key
    let cipher = Aes256Gcm::new(&derived_key.into());
    let nonce = aes_gcm::Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, data).map_err(|e| e.to_string())?;

    // 6. Return: [Ephemeral PubKey (65b) | Nonce (12b) | Ciphertext]
    let mut output = ephemeral_public.to_encoded_point(false).as_bytes().to_vec();
    output.extend_from_slice(&nonce);
    output.extend(ciphertext);

    Ok(output)
}
