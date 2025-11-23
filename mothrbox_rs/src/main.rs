mod encryption;

use encryption::{ECCEncryption, AESEncryption, ChaChaEncryption};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        std::process::exit(0);
    }
    
    let command = args[1].to_lowercase();
    
    match command.as_str() {
        "help" | "--help" | "-h" => print_help(),
        "ecc" => handle_ecc(&args[2..]),
        "aes" => handle_aes(&args[2..]),
        "chacha" => handle_chacha(&args[2..]),
        "walrus" => handle_walrus(&args[2..]),
        "version" | "--version" | "-v" => print_version(),
        _ => {
            eprintln!("❌ Unknown command: {}", command);
            eprintln!("Run 'cryptool help' for usage information");
            std::process::exit(1);
        }
    }
}

fn print_version() {
    println!("CrypTool v1.0.0");
    println!("Multi-algorithm encryption tool with Walrus storage");
    println!("Algorithms: ECC (P-256), AES-256-GCM, ChaCha20-Poly1305");
}

fn print_help() {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║                   CRYPTOOL v1.0                        ║");
    println!("║      Multi-Algorithm Encryption + Walrus Storage       ║");
    println!("╚════════════════════════════════════════════════════════╝\n");
    
    println!("USAGE:");
    println!("  cryptool <algorithm> <action> [options]\n");
    
    println!("ALGORITHMS:");
    println!("  ecc      Elliptic Curve Cryptography (Public-Key)");
    println!("  aes      AES-256-GCM (Symmetric, Password-based)");
    println!("  chacha   ChaCha20-Poly1305 (Symmetric, Password-based)");
    println!("  walrus   Walrus storage integration\n");
    
    println!("LOCAL ENCRYPTION:\n");
    
    println!("  ECC Commands:");
    println!("    keygen                           Generate new key pair");
    println!("    encrypt <input> <output> <pub>   Encrypt file with public key");
    println!("    decrypt <input> <output> <priv>  Decrypt file with private key\n");
    
    println!("  AES/ChaCha Commands:");
    println!("    encrypt <input> <output> <pass>  Encrypt file with password");
    println!("    decrypt <input> <output> <pass>  Decrypt file with password\n");
    
    println!("WALRUS INTEGRATION:\n");
    
    println!("  Upload & Store:");
    println!("    walrus upload-aes <file> <pass> <url>        Encrypt with AES & upload");
    println!("    walrus upload-chacha <file> <pass> <url>     Encrypt with ChaCha & upload");
    println!("    walrus upload-ecc <file> <pubkey> <url>      Encrypt with ECC & upload\n");
    
    println!("  Download & Decrypt:");
    println!("    walrus download-aes <blobId> <out> <pass> <url>    Download & decrypt AES");
    println!("    walrus download-chacha <blobId> <out> <pass> <url> Download & decrypt ChaCha");
    println!("    walrus download-ecc <blobId> <out> <priv> <url>    Download & decrypt ECC\n");
    
    println!("EXAMPLES:\n");
    
    println!("  Local Encryption:");
    println!("  cryptool aes encrypt doc.pdf doc.enc \"MyPass123\"\n");
    
    println!("  Encrypt & Upload to Walrus:");
    println!("  cryptool walrus upload-aes secret.pdf \"MyPass123\" http://localhost:8000");
    println!("  → Returns: {{\"blobId\": \"xyz...\"}}\n");
    
    println!("  Download & Decrypt from Walrus:");
    println!("  cryptool walrus download-aes xyz... decrypted.pdf \"MyPass123\" http://localhost:8000\n");
    
    println!("  Full Workflow:");
    println!("  1. cryptool ecc keygen");
    println!("  2. cryptool walrus upload-ecc doc.pdf public.key http://localhost:8000");
    println!("  3. Share blobId with recipient");
    println!("  4. cryptool walrus download-ecc <blobId> doc.pdf private.key http://localhost:8000\n");
    
    println!("NOTE: Make sure your Deno Walrus server is running on the specified URL");
}

fn handle_walrus(args: &[String]) {
    if args.is_empty() {
        eprintln!("❌ Missing Walrus action");
        eprintln!("Use: upload-aes, upload-chacha, upload-ecc, download-aes, download-chacha, download-ecc");
        std::process::exit(1);
    }
    
    let action = args[0].to_lowercase();
    
    match action.as_str() {
        "upload-aes" => {
            if args.len() < 4 {
                eprintln!("❌ Usage: cryptool walrus upload-aes <file> <password> <walrus_url>");
                std::process::exit(1);
            }
            walrus_upload_aes(&args[1], &args[2], &args[3]);
        }
        "upload-chacha" => {
            if args.len() < 4 {
                eprintln!("❌ Usage: cryptool walrus upload-chacha <file> <password> <walrus_url>");
                std::process::exit(1);
            }
            walrus_upload_chacha(&args[1], &args[2], &args[3]);
        }
        "upload-ecc" => {
            if args.len() < 4 {
                eprintln!("❌ Usage: cryptool walrus upload-ecc <file> <public_key> <walrus_url>");
                std::process::exit(1);
            }
            walrus_upload_ecc(&args[1], &args[2], &args[3]);
        }
        "download-aes" => {
            if args.len() < 5 {
                eprintln!("❌ Usage: cryptool walrus download-aes <blobId> <output> <password> <walrus_url>");
                std::process::exit(1);
            }
            walrus_download_aes(&args[1], &args[2], &args[3], &args[4]);
        }
        "download-chacha" => {
            if args.len() < 5 {
                eprintln!("❌ Usage: cryptool walrus download-chacha <blobId> <output> <password> <walrus_url>");
                std::process::exit(1);
            }
            walrus_download_chacha(&args[1], &args[2], &args[3], &args[4]);
        }
        "download-ecc" => {
            if args.len() < 5 {
                eprintln!("❌ Usage: cryptool walrus download-ecc <blobId> <output> <private_key> <walrus_url>");
                std::process::exit(1);
            }
            walrus_download_ecc(&args[1], &args[2], &args[3], &args[4]);
        }
        _ => {
            eprintln!("❌ Unknown Walrus action: {}", action);
            std::process::exit(1);
        }
    }
}

// Walrus Upload Functions
fn walrus_upload_aes(file: &str, password: &str, walrus_url: &str) {
    println!("🔒 Encrypting with AES-256-GCM...");
    
    if !Path::new(file).exists() {
        eprintln!("❌ File not found: {}", file);
        std::process::exit(1);
    }
    
    // Read and encrypt file
    let plaintext = fs::read(file).expect("Failed to read file");
    println!("   📄 File: {} ({} bytes)", file, plaintext.len());
    
    let encrypted = AESEncryption::encrypt(&plaintext, password)
        .expect("Encryption failed");
    println!("   ✓ Encrypted: {} bytes", encrypted.len());
    
    // Upload to Walrus
    println!("\n📤 Uploading to Walrus...");
    upload_to_walrus(&encrypted, file, walrus_url);
}

fn walrus_upload_chacha(file: &str, password: &str, walrus_url: &str) {
    println!("🔒 Encrypting with ChaCha20-Poly1305...");
    
    if !Path::new(file).exists() {
        eprintln!("❌ File not found: {}", file);
        std::process::exit(1);
    }
    
    let plaintext = fs::read(file).expect("Failed to read file");
    println!("   📄 File: {} ({} bytes)", file, plaintext.len());
    
    let encrypted = ChaChaEncryption::encrypt(&plaintext, password)
        .expect("Encryption failed");
    println!("   ✓ Encrypted: {} bytes", encrypted.len());
    
    println!("\n📤 Uploading to Walrus...");
    upload_to_walrus(&encrypted, file, walrus_url);
}

fn walrus_upload_ecc(file: &str, pubkey_file: &str, walrus_url: &str) {
    println!("🔒 Encrypting with ECC...");
    
    if !Path::new(file).exists() {
        eprintln!("❌ File not found: {}", file);
        std::process::exit(1);
    }
    
    // Load public key
    let pubkey_bytes = fs::read(pubkey_file)
        .expect("Failed to read public key");
    let public_key = p256::PublicKey::from_sec1_bytes(&pubkey_bytes)
        .expect("Invalid public key");
    
    let plaintext = fs::read(file).expect("Failed to read file");
    println!("   📄 File: {} ({} bytes)", file, plaintext.len());
    
    let encrypted = ECCEncryption::encrypt(&plaintext, &public_key)
        .expect("Encryption failed");
    println!("   ✓ Encrypted: {} bytes", encrypted.len());
    
    println!("\n📤 Uploading to Walrus...");
    upload_to_walrus(&encrypted, file, walrus_url);
}

// Walrus Download Functions
fn walrus_download_aes(blob_id: &str, output: &str, password: &str, walrus_url: &str) {
    println!("📥 Downloading from Walrus...");
    let encrypted = download_from_walrus(blob_id, walrus_url);
    println!("   ✓ Downloaded: {} bytes", encrypted.len());
    
    println!("\n🔓 Decrypting with AES-256-GCM...");
    let decrypted = AESEncryption::decrypt(&encrypted, password)
        .expect("Decryption failed - wrong password?");
    
    fs::write(output, &decrypted).expect("Failed to save file");
    println!("   ✓ Decrypted: {} bytes", decrypted.len());
    println!("   📄 Saved to: {}", output);
    println!("\n✅ Download and decryption complete!");
}

fn walrus_download_chacha(blob_id: &str, output: &str, password: &str, walrus_url: &str) {
    println!("📥 Downloading from Walrus...");
    let encrypted = download_from_walrus(blob_id, walrus_url);
    println!("   ✓ Downloaded: {} bytes", encrypted.len());
    
    println!("\n🔓 Decrypting with ChaCha20-Poly1305...");
    let decrypted = ChaChaEncryption::decrypt(&encrypted, password)
        .expect("Decryption failed - wrong password?");
    
    fs::write(output, &decrypted).expect("Failed to save file");
    println!("   ✓ Decrypted: {} bytes", decrypted.len());
    println!("   📄 Saved to: {}", output);
    println!("\n✅ Download and decryption complete!");
}

fn walrus_download_ecc(blob_id: &str, output: &str, privkey_file: &str, walrus_url: &str) {
    println!("📥 Downloading from Walrus...");
    let encrypted = download_from_walrus(blob_id, walrus_url);
    println!("   ✓ Downloaded: {} bytes", encrypted.len());
    
    // Load private key
    let privkey_bytes = fs::read(privkey_file)
        .expect("Failed to read private key");
    let private_key: [u8; 32] = privkey_bytes.try_into()
        .expect("Invalid private key");
    
    println!("\n🔓 Decrypting with ECC...");
    let decrypted = ECCEncryption::decrypt(&encrypted, &private_key)
        .expect("Decryption failed - wrong key?");
    
    fs::write(output, &decrypted).expect("Failed to save file");
    println!("   ✓ Decrypted: {} bytes", decrypted.len());
    println!("   📄 Saved to: {}", output);
    println!("\n✅ Download and decryption complete!");
}

// HTTP Client Functions
fn upload_to_walrus(data: &[u8], filename: &str, walrus_url: &str) {
    use std::process::Command;
    
    // Save encrypted data to temp file
    let temp_file = format!(".temp_encrypted_{}", filename);
    fs::write(&temp_file, data).expect("Failed to save temp file");
    
    // Use curl to upload
    let url = format!("{}/write", walrus_url);
    let output = Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg(&url)
        .arg("-F")
        .arg(format!("file=@{}", temp_file))
        .output()
        .expect("Failed to execute curl - make sure curl is installed");
    
    // Clean up temp file
    fs::remove_file(&temp_file).ok();
    
    if output.status.success() {
        let response = String::from_utf8_lossy(&output.stdout);
        println!("   ✓ Upload response: {}", response);
        println!("\n✅ Successfully uploaded to Walrus!");
        println!("💾 Save the blobId from the response above to download later");
    } else {
        eprintln!("❌ Upload failed: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
}

fn download_from_walrus(blob_id: &str, walrus_url: &str) -> Vec<u8> {
    use std::process::Command;
    
    let url = format!("{}/read/{}", walrus_url, blob_id);
    let output = Command::new("curl")
        .arg("-s")
        .arg(&url)
        .output()
        .expect("Failed to execute curl");
    
    if output.status.success() {
        output.stdout
    } else {
        eprintln!("❌ Download failed: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
}

// Keep existing ECC, AES, ChaCha handlers from previous cli_main.rs
// (Insert all the handle_ecc, handle_aes, handle_chacha functions here)
// ... [previous implementation] ...

fn handle_ecc(args: &[String]) {
    if args.is_empty() {
        eprintln!("❌ Missing action. Use: keygen, encrypt, or decrypt");
        std::process::exit(1);
    }
    
    let action = args[0].to_lowercase();
    
    match action.as_str() {
        "keygen" => ecc_keygen(),
        "encrypt" => {
            if args.len() < 4 {
                eprintln!("❌ Usage: cryptool ecc encrypt <input> <output> <public_key_file>");
                std::process::exit(1);
            }
            ecc_encrypt(&args[1], &args[2], &args[3]);
        }
        "decrypt" => {
            if args.len() < 4 {
                eprintln!("❌ Usage: cryptool ecc decrypt <input> <output> <private_key_file>");
                std::process::exit(1);
            }
            ecc_decrypt(&args[1], &args[2], &args[3]);
        }
        _ => {
            eprintln!("❌ Unknown ECC action: {}", action);
            std::process::exit(1);
        }
    }
}

fn handle_aes(args: &[String]) {
    if args.is_empty() {
        eprintln!("❌ Missing action. Use: encrypt or decrypt");
        std::process::exit(1);
    }
    
    let action = args[0].to_lowercase();
    
    match action.as_str() {
        "encrypt" => {
            if args.len() < 4 {
                eprintln!("❌ Usage: cryptool aes encrypt <input> <output> <password>");
                std::process::exit(1);
            }
            aes_encrypt(&args[1], &args[2], &args[3]);
        }
        "decrypt" => {
            if args.len() < 4 {
                eprintln!("❌ Usage: cryptool aes decrypt <input> <output> <password>");
                std::process::exit(1);
            }
            aes_decrypt(&args[1], &args[2], &args[3]);
        }
        _ => {
            eprintln!("❌ Unknown AES action: {}", action);
            std::process::exit(1);
        }
    }
}

fn handle_chacha(args: &[String]) {
    if args.is_empty() {
        eprintln!("❌ Missing action. Use: encrypt or decrypt");
        std::process::exit(1);
    }
    
    let action = args[0].to_lowercase();
    
    match action.as_str() {
        "encrypt" => {
            if args.len() < 4 {
                eprintln!("❌ Usage: cryptool chacha encrypt <input> <output> <password>");
                std::process::exit(1);
            }
            chacha_encrypt(&args[1], &args[2], &args[3]);
        }
        "decrypt" => {
            if args.len() < 4 {
                eprintln!("❌ Usage: cryptool chacha decrypt <input> <output> <password>");
                std::process::exit(1);
            }
            chacha_decrypt(&args[1], &args[2], &args[3]);
        }
        _ => {
            eprintln!("❌ Unknown ChaCha action: {}", action);
            std::process::exit(1);
        }
    }
}

// Local encryption functions (same as before)
fn ecc_keygen() {
    println!("🔑 Generating ECC key pair...");
    let (private_key, public_key) = ECCEncryption::generate_keypair();
    fs::write("private.key", &private_key).expect("Failed to save private key");
    fs::write("public.key", public_key.to_sec1_bytes().as_ref()).expect("Failed to save public key");
    println!("✅ Key pair generated!");
    println!("   📄 Private key: private.key");
    println!("   📄 Public key:  public.key");
}

fn ecc_encrypt(input: &str, output: &str, pubkey_file: &str) {
    println!("🔒 Encrypting with ECC...");
    let pubkey_bytes = fs::read(pubkey_file).expect("Failed to read public key");
    let public_key = p256::PublicKey::from_sec1_bytes(&pubkey_bytes).expect("Invalid public key");
    let plaintext = fs::read(input).expect("Failed to read input");
    let encrypted = ECCEncryption::encrypt(&plaintext, &public_key).expect("Encryption failed");
    fs::write(output, &encrypted).expect("Failed to save output");
    println!("✅ File encrypted: {}", output);
}

fn ecc_decrypt(input: &str, output: &str, privkey_file: &str) {
    println!("🔓 Decrypting with ECC...");
    let privkey_bytes = fs::read(privkey_file).expect("Failed to read private key");
    let private_key: [u8; 32] = privkey_bytes.try_into().expect("Invalid private key");
    let encrypted = fs::read(input).expect("Failed to read input");
    let decrypted = ECCEncryption::decrypt(&encrypted, &private_key).expect("Decryption failed");
    fs::write(output, &decrypted).expect("Failed to save output");
    println!("✅ File decrypted: {}", output);
}

fn aes_encrypt(input: &str, output: &str, password: &str) {
    println!("🔒 Encrypting with AES-256-GCM...");
    AESEncryption::encrypt_file(input, output, password).expect("Encryption failed");
    println!("✅ File encrypted: {}", output);
}

fn aes_decrypt(input: &str, output: &str, password: &str) {
    println!("🔓 Decrypting with AES-256-GCM...");
    AESEncryption::decrypt_file(input, output, password).expect("Decryption failed");
    println!("✅ File decrypted: {}", output);
}

fn chacha_encrypt(input: &str, output: &str, password: &str) {
    println!("🔒 Encrypting with ChaCha20-Poly1305...");
    ChaChaEncryption::encrypt_file(input, output, password).expect("Encryption failed");
    println!("✅ File encrypted: {}", output);
}

fn chacha_decrypt(input: &str, output: &str, password: &str) {
    println!("🔓 Decrypting with ChaCha20-Poly1305...");
    ChaChaEncryption::decrypt_file(input, output, password).expect("Decryption failed");
    println!("✅ File decrypted: {}", output);
}