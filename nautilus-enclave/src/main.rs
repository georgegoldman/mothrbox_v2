// nautilus-enclave/src/main.rs
// MothrBox Nautilus Enclave Server - Custom for your function signatures

use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use serde::{Deserialize, Serialize};
use std::fs;
use base64::{Engine as _, engine::general_purpose};
use sha2::{Sha256, Digest};
use std::process::Command;
use mothrbox_crypto;

#[derive(Deserialize)]
struct EncryptRequest {
    file_data: String,  // Base64 encoded
    password: String,
    algorithm: String,  // "aes", "chacha", or "ecc"
    filename: String,
    #[serde(default)]
    public_key: Option<String>,  // For ECC (base64)
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
    #[serde(default)]
    private_key: Option<String>,  // For ECC (base64)
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
    log::info!("🔐 Encrypting: {} with {}", req.filename, req.algorithm);

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

    // Encrypt based on algorithm
    let encrypt_result = match req.algorithm.as_str() {
        "aes" => {
            mothrbox_crypto::aes::encrypt_file(&input_path, &output_path, &req.password)
        },
        "chacha" => {
            mothrbox_crypto::chacha::encrypt_file(&input_path, &output_path, &req.password)
        },
        "ecc" => {
            // For ECC, we need a public key file
            if let Some(public_key_b64) = &req.public_key {
                let public_key_data = match general_purpose::STANDARD.decode(public_key_b64) {
                    Ok(data) => data,
                    Err(e) => return HttpResponse::BadRequest().json(EncryptResponse {
                        success: false, blob_id: None, file_hash: None,
                        attestation_document: None, 
                        error: Some(format!("Invalid public key base64: {}", e)),
                    }),
                };
                
                let pubkey_path = format!("{}/public.key", temp_dir);
                if let Err(e) = fs::write(&pubkey_path, public_key_data) {
                    return HttpResponse::InternalServerError().json(EncryptResponse {
                        success: false, blob_id: None, file_hash: None,
                        attestation_document: None, error: Some(format!("Key write error: {}", e)),
                    });
                }
                
                mothrbox_crypto::ecc::encrypt_file(&input_path, &output_path, &pubkey_path)
            } else {
                return HttpResponse::BadRequest().json(EncryptResponse {
                    success: false, blob_id: None, file_hash: None,
                    attestation_document: None, 
                    error: Some("ECC requires public_key field".to_string()),
                });
            }
        },
        _ => {
            return HttpResponse::BadRequest().json(EncryptResponse {
                success: false, blob_id: None, file_hash: None,
                attestation_document: None, 
                error: Some("Invalid algorithm (use 'aes', 'chacha', or 'ecc')".to_string()),
            });
        }
    };

    if let Err(e) = encrypt_result {
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
    log::info!("🔓 Decrypting: {} with {}", req.blob_id, req.algorithm);

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

    let decrypt_result = match req.algorithm.as_str() {
        "aes" => {
            mothrbox_crypto::aes::decrypt_file(&encrypted_path, &decrypted_path, &req.password)
        },
        "chacha" => {
            mothrbox_crypto::chacha::decrypt_file(&encrypted_path, &decrypted_path, &req.password)
        },
        "ecc" => {
            if let Some(private_key_b64) = &req.private_key {
                let private_key_data = match general_purpose::STANDARD.decode(private_key_b64) {
                    Ok(data) => data,
                    Err(e) => return HttpResponse::BadRequest().json(DecryptResponse {
                        success: false, file_data: None, attestation_document: None,
                        error: Some(format!("Invalid private key base64: {}", e)),
                    }),
                };
                
                let privkey_path = format!("{}/private.key", temp_dir);
                if let Err(e) = fs::write(&privkey_path, private_key_data) {
                    return HttpResponse::InternalServerError().json(DecryptResponse {
                        success: false, file_data: None, attestation_document: None,
                        error: Some(format!("Key write error: {}", e)),
                    });
                }
                
                mothrbox_crypto::ecc::decrypt_file(&encrypted_path, &decrypted_path, &privkey_path)
            } else {
                return HttpResponse::BadRequest().json(DecryptResponse {
                    success: false, file_data: None, attestation_document: None,
                    error: Some("ECC requires private_key field".to_string()),
                });
            }
        },
        _ => {
            return HttpResponse::BadRequest().json(DecryptResponse {
                success: false, file_data: None, attestation_document: None,
                error: Some("Invalid algorithm (use 'aes', 'chacha', or 'ecc')".to_string()),
            });
        }
    };

    if let Err(e) = decrypt_result {
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
        "algorithms": ["aes", "chacha", "ecc"],
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    log::info!("🚀 Starting MothrBox Nautilus Enclave Server");
    log::info!("🔐 Algorithms: AES-256-GCM, ChaCha20-Poly1305, ECC P-256");
    log::info!("⛓️  Storage: Walrus Protocol");
    log::info!("🌐 Listening on http://127.0.0.1:8080");

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