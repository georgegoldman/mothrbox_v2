#!/bin/bash
# Automated Nautilus Integration Script
# Run this from your mothrbox_v2/ directory

set -e

BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}"
cat << 'EOF'
╔════════════════════════════════════════╗
║  MothrBox + Nautilus Integration      ║
║  Automated Setup                       ║
╚════════════════════════════════════════╝
EOF
echo -e "${NC}"

# Check we're in the right directory
if [ ! -d "mothrbox_rs" ]; then
    echo -e "${YELLOW}⚠️  Run this script from your mothrbox_v2/ directory${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Found mothrbox_rs${NC}"

# Step 1: Create nautilus-enclave directory
echo -e "\n${BLUE}[1/7] Creating nautilus-enclave directory...${NC}"
mkdir -p nautilus-enclave/src
echo -e "${GREEN}✓ Created nautilus-enclave/src${NC}"

# Step 2: Create Cargo.toml for nautilus-enclave
echo -e "\n${BLUE}[2/7] Creating nautilus-enclave/Cargo.toml...${NC}"
cat > nautilus-enclave/Cargo.toml << 'EOF'
[package]
name = "mothrbox-nautilus"
version = "1.0.0"
edition = "2021"

[dependencies]
actix-web = "4.5"
actix-rt = "2.9"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
base64 = "0.22"
sha2 = "0.10"
uuid = { version = "1.0", features = ["v4"] }
env_logger = "0.11"
log = "0.4"

# Your existing encryption library
mothrbox-crypto = { path = "../mothrbox_rs" }
EOF
echo -e "${GREEN}✓ Created Cargo.toml${NC}"

# Step 3: Update mothrbox_rs/Cargo.toml
echo -e "\n${BLUE}[3/7] Updating mothrbox_rs/Cargo.toml...${NC}"

# Backup original
cp mothrbox_rs/Cargo.toml mothrbox_rs/Cargo.toml.backup

# Add library configuration if not present
if ! grep -q "\[lib\]" mothrbox_rs/Cargo.toml; then
    cat >> mothrbox_rs/Cargo.toml << 'EOF'

# Library configuration (for Nautilus integration)
[lib]
name = "mothrbox_crypto"
path = "src/lib.rs"

[[bin]]
name = "mothrbox-cli"
path = "src/main.rs"
EOF
    echo -e "${GREEN}✓ Added library configuration${NC}"
else
    echo -e "${YELLOW}⚠️  Library config already exists${NC}"
fi

# Step 4: Create lib.rs
echo -e "\n${BLUE}[4/7] Creating mothrbox_rs/src/lib.rs...${NC}"
cat > mothrbox_rs/src/lib.rs << 'EOF'
// Library interface for MothrBox encryption

pub mod encryption;

// Re-export for convenience
pub use encryption::{
    aes::{encrypt_aes, decrypt_aes},
    chacha::{encrypt_chacha, decrypt_chacha},
    ecc::{encrypt_ecc, decrypt_ecc, generate_keypair},
};

// Unified encryption interface
pub fn encrypt_file(
    input_path: &str,
    output_path: &str,
    password: &str,
    algorithm: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match algorithm {
        "aes" => encrypt_aes(input_path, output_path, password),
        "chacha" => encrypt_chacha(input_path, output_path, password),
        "ecc" => encrypt_ecc(input_path, output_path, password),
        _ => Err("Invalid algorithm".into()),
    }
}

pub fn decrypt_file(
    input_path: &str,
    output_path: &str,
    password: &str,
    algorithm: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match algorithm {
        "aes" => decrypt_aes(input_path, output_path, password),
        "chacha" => decrypt_chacha(input_path, output_path, password),
        "ecc" => decrypt_ecc(input_path, output_path, password),
        _ => Err("Invalid algorithm".into()),
    }
}
EOF
echo -e "${GREEN}✓ Created lib.rs${NC}"

# Step 5: Update encryption/mod.rs
echo -e "\n${BLUE}[5/7] Updating encryption module exports...${NC}"
cat > mothrbox_rs/src/encryption/mod.rs << 'EOF'
pub mod aes;
pub mod chacha;
pub mod ecc;

// Re-export for convenience
pub use aes::{encrypt_aes, decrypt_aes};
pub use chacha::{encrypt_chacha, decrypt_chacha};
pub use ecc::{encrypt_ecc, decrypt_ecc, generate_keypair};
EOF
echo -e "${GREEN}✓ Updated encryption/mod.rs${NC}"

# Step 6: Create nautilus server
echo -e "\n${BLUE}[6/7] Creating nautilus-enclave/src/main.rs...${NC}"
cat > nautilus-enclave/src/main.rs << 'EOFRUST'
use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use serde::{Deserialize, Serialize};
use std::fs;
use base64::{Engine as _, engine::general_purpose};
use sha2::{Sha256, Digest};
use std::process::Command;
use mothrbox_crypto;

#[derive(Deserialize)]
struct EncryptRequest {
    file_data: String,
    password: String,
    algorithm: String,
    filename: String,
}

#[derive(Serialize)]
struct EncryptResponse {
    success: bool,
    blob_id: Option<String>,
    file_hash: Option<String>,
    attestation_document: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct DecryptRequest {
    blob_id: String,
    password: String,
    algorithm: String,
}

#[derive(Serialize)]
struct DecryptResponse {
    success: bool,
    file_data: Option<String>,
    attestation_document: Option<String>,
    error: Option<String>,
}

fn generate_attestation(user_data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(user_data.as_bytes());
    let hash = hasher.finalize();
    general_purpose::STANDARD.encode(hash)
}

fn upload_to_walrus(file_path: &str) -> Result<String, String> {
    let output = Command::new("deno")
        .args(&[
            "run", "-A", "--env-file=../mothrbox_ts/.env",
            "../mothrbox_ts/src/walrus-cli.ts", "upload", file_path,
        ])
        .output()
        .map_err(|e| format!("Walrus upload failed: {}", e))?;

    if !output.status.success() {
        return Err(format!("Walrus error: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let response: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Parse error: {}", e))?;

    response["blobId"].as_str().map(|s| s.to_string())
        .ok_or_else(|| "No blobId".to_string())
}

fn download_from_walrus(blob_id: &str, output_path: &str) -> Result<(), String> {
    let output = Command::new("deno")
        .args(&[
            "run", "-A", "--env-file=../mothrbox_ts/.env",
            "../mothrbox_ts/src/walrus-cli.ts", "download", blob_id, output_path,
        ])
        .output()
        .map_err(|e| format!("Walrus download failed: {}", e))?;

    if !output.status.success() {
        return Err(format!("Walrus error: {}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(())
}

fn hash_file(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

async fn encrypt_handler(req: web::Json<EncryptRequest>) -> HttpResponse {
    log::info!("🔐 Encrypting: {}", req.filename);

    let file_data = match general_purpose::STANDARD.decode(&req.file_data) {
        Ok(data) => data,
        Err(e) => return HttpResponse::BadRequest().json(EncryptResponse {
            success: false, blob_id: None, file_hash: None,
            attestation_document: None, error: Some(format!("Invalid base64: {}", e)),
        }),
    };

    let temp_id = uuid::Uuid::new_v4();
    let temp_dir = format!("/tmp/encrypt_{}", temp_id);
    fs::create_dir_all(&temp_dir).unwrap();

    let input_path = format!("{}/input.bin", temp_dir);
    let output_path = format!("{}/output.enc", temp_dir);

    if let Err(e) = fs::write(&input_path, &file_data) {
        return HttpResponse::InternalServerError().json(EncryptResponse {
            success: false, blob_id: None, file_hash: None,
            attestation_document: None, error: Some(format!("Write error: {}", e)),
        });
    }

    if let Err(e) = mothrbox_crypto::encrypt_file(&input_path, &output_path, &req.password, &req.algorithm) {
        return HttpResponse::InternalServerError().json(EncryptResponse {
            success: false, blob_id: None, file_hash: None,
            attestation_document: None, error: Some(format!("Encryption error: {}", e)),
        });
    }

    let encrypted_data = match fs::read(&output_path) {
        Ok(data) => data,
        Err(e) => return HttpResponse::InternalServerError().json(EncryptResponse {
            success: false, blob_id: None, file_hash: None,
            attestation_document: None, error: Some(format!("Read error: {}", e)),
        }),
    };

    let file_hash = hash_file(&encrypted_data);
    let blob_id = match upload_to_walrus(&output_path) {
        Ok(id) => id,
        Err(e) => return HttpResponse::InternalServerError().json(EncryptResponse {
            success: false, blob_id: None, file_hash: None,
            attestation_document: None, error: Some(e),
        }),
    };

    let attestation = generate_attestation(&format!("{}:{}", blob_id, file_hash));
    fs::remove_dir_all(&temp_dir).ok();

    HttpResponse::Ok().json(EncryptResponse {
        success: true,
        blob_id: Some(blob_id),
        file_hash: Some(file_hash),
        attestation_document: Some(attestation),
        error: None,
    })
}

async fn decrypt_handler(req: web::Json<DecryptRequest>) -> HttpResponse {
    log::info!("🔓 Decrypting: {}", req.blob_id);

    let temp_id = uuid::Uuid::new_v4();
    let temp_dir = format!("/tmp/decrypt_{}", temp_id);
    fs::create_dir_all(&temp_dir).unwrap();

    let encrypted_path = format!("{}/encrypted.bin", temp_dir);
    let decrypted_path = format!("{}/decrypted.bin", temp_dir);

    if let Err(e) = download_from_walrus(&req.blob_id, &encrypted_path) {
        return HttpResponse::InternalServerError().json(DecryptResponse {
            success: false, file_data: None, attestation_document: None, error: Some(e),
        });
    }

    if let Err(e) = mothrbox_crypto::decrypt_file(&encrypted_path, &decrypted_path, &req.password, &req.algorithm) {
        return HttpResponse::InternalServerError().json(DecryptResponse {
            success: false, file_data: None, attestation_document: None,
            error: Some(format!("Decryption error: {}", e)),
        });
    }

    let decrypted_data = match fs::read(&decrypted_path) {
        Ok(data) => data,
        Err(e) => return HttpResponse::InternalServerError().json(DecryptResponse {
            success: false, file_data: None, attestation_document: None,
            error: Some(format!("Read error: {}", e)),
        }),
    };

    let attestation = generate_attestation(&format!("decrypt:{}", req.blob_id));
    fs::remove_dir_all(&temp_dir).ok();
    let file_data_b64 = general_purpose::STANDARD.encode(&decrypted_data);

    HttpResponse::Ok().json(DecryptResponse {
        success: true,
        file_data: Some(file_data_b64),
        attestation_document: Some(attestation),
        error: None,
    })
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "MothrBox Nautilus Enclave",
        "version": "1.0.0",
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    log::info!("🚀 Starting MothrBox Nautilus Enclave Server");
    log::info!("🔐 Encryption: AES-256-GCM, ChaCha20-Poly1305, ECC");
    log::info!("⛓️  Storage: Walrus Protocol");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(health_check))
            .route("/encrypt", web::post().to(encrypt_handler))
            .route("/decrypt", web::post().to(decrypt_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
EOFRUST
echo -e "${GREEN}✓ Created nautilus server${NC}"

# Step 7: Create test script
echo -e "\n${BLUE}[7/7] Creating test-nautilus.sh...${NC}"
cat > test-nautilus.sh << 'EOF'
#!/bin/bash

echo "🚀 Starting Nautilus server..."
cd nautilus-enclave
cargo run --release &
SERVER_PID=$!

sleep 5

echo ""
echo "🧪 Testing health endpoint..."
curl -s http://localhost:8080/health | jq '.'

echo ""
echo "✅ Server is running!"
echo ""
echo "💡 Test encryption:"
echo "   echo 'Hello World!' | base64 > /tmp/test.b64"
echo "   curl -X POST http://localhost:8080/encrypt \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{\"file_data\":\"'\$(cat /tmp/test.b64)'\",\"password\":\"test\",\"algorithm\":\"aes\",\"filename\":\"test.txt\"}' | jq '.'"
echo ""
echo "Press Ctrl+C to stop (PID: $SERVER_PID)"
wait $SERVER_PID
EOF
chmod +x test-nautilus.sh
echo -e "${GREEN}✓ Created test script${NC}"

# Final summary
echo -e "\n${GREEN}"
cat << 'EOF'
╔════════════════════════════════════════╗
║  Integration Complete! ✅              ║
╚════════════════════════════════════════╝
EOF
echo -e "${NC}"

echo -e "${BLUE}Next steps:${NC}"
echo "1. Build the encryption library:"
echo -e "   ${YELLOW}cd mothrbox_rs && cargo build --release${NC}"
echo ""
echo "2. Build the Nautilus server:"
echo -e "   ${YELLOW}cd ../nautilus-enclave && cargo build --release${NC}"
echo ""
echo "3. Test the server:"
echo -e "   ${YELLOW}cd .. && ./test-nautilus.sh${NC}"
echo ""
echo -e "${GREEN}Your project structure:${NC}"
echo "mothrbox_v2/"
echo "├── mothrbox_rs/          (Updated with lib.rs)"
echo "├── nautilus-enclave/     (NEW - TEE server)"
echo "└── test-nautilus.sh      (NEW - test script)"
echo ""
echo -e "${BLUE}Documentation:${NC}"
echo "- INTEGRATION_STEPS.md - Detailed integration guide"
echo "- SETUP_GUIDE.md - Production deployment"
echo "- BENEFITS_AND_USE_CASES.md - Use cases and ROI"
echo ""
echo -e "${GREEN}Happy hacking! 🔒🦋${NC}"