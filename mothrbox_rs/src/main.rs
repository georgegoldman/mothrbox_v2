use clap::{Parser, Subcommand};
use std::path::Path;

mod encryption;
mod walrus;

use encryption::{aes, chacha, ecc};
use walrus::{encrypt_and_upload_aes, download_and_decrypt_aes, WalrusCli};

#[derive(Parser)]
#[command(name = "mothrbox")]
#[command(about = "MothrBox - Encrypted Decentralized Storage", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// AES-256-GCM encryption operations
    Aes {
        #[command(subcommand)]
        action: AesCommands,
    },
    /// ChaCha20-Poly1305 encryption operations
    Chacha {
        #[command(subcommand)]
        action: ChachaCommands,
    },
    /// ECC (Elliptic Curve Cryptography) operations
    Ecc {
        #[command(subcommand)]
        action: EccCommands,
    },
    /// Walrus decentralized storage operations
    Walrus {
        #[command(subcommand)]
        action: WalrusCommands,
    },
}

#[derive(Subcommand)]
enum AesCommands {
    /// Encrypt a file with AES-256-GCM
    Encrypt {
        /// Input file path
        input: String,
        /// Output file path
        output: String,
        /// Encryption password
        password: String,
    },
    /// Decrypt a file with AES-256-GCM
    Decrypt {
        /// Input file path
        input: String,
        /// Output file path
        output: String,
        /// Decryption password
        password: String,
    },
}

#[derive(Subcommand)]
enum ChachaCommands {
    /// Encrypt a file with ChaCha20-Poly1305
    Encrypt {
        input: String,
        output: String,
        password: String,
    },
    /// Decrypt a file with ChaCha20-Poly1305
    Decrypt {
        input: String,
        output: String,
        password: String,
    },
}

#[derive(Subcommand)]
enum EccCommands {
    /// Generate ECC key pair
    Keygen,
    /// Encrypt a file with ECC public key
    Encrypt {
        input: String,
        output: String,
        /// Public key file path
        public_key: String,
    },
    /// Decrypt a file with ECC private key
    Decrypt {
        input: String,
        output: String,
        /// Private key file path
        private_key: String,
    },
}

#[derive(Subcommand)]
enum WalrusCommands {
    /// Upload file to Walrus (raw)
    Upload {
        /// File to upload
        file: String,
    },
    /// Download file from Walrus (raw)
    Download {
        /// Blob ID
        blob_id: String,
        /// Output file path
        output: String,
    },
    /// Encrypt with AES and upload to Walrus
    UploadAes {
        /// File to encrypt and upload
        file: String,
        /// Encryption password
        password: String,
    },
    /// Download from Walrus and decrypt with AES
    DownloadAes {
        /// Blob ID
        blob_id: String,
        /// Output file path
        output: String,
        /// Decryption password
        password: String,
    },
    /// Encrypt with ChaCha20 and upload to Walrus
    UploadChacha {
        file: String,
        password: String,
    },
    /// Download from Walrus and decrypt with ChaCha20
    DownloadChacha {
        blob_id: String,
        output: String,
        password: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Aes { action } => handle_aes(action),
        Commands::Chacha { action } => handle_chacha(action),
        Commands::Ecc { action } => handle_ecc(action),
        Commands::Walrus { action } => handle_walrus(action),
    };

    match result {
        Ok(msg) => {
            if !msg.is_empty() {
                println!("✅ {}", msg);
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_aes(action: AesCommands) -> Result<String, String> {
    match action {
        AesCommands::Encrypt { input, output, password } => {
            aes::encrypt_file(&input, &output, &password)?;
            Ok(format!("Encrypted: {} -> {}", input, output))
        }
        AesCommands::Decrypt { input, output, password } => {
            aes::decrypt_file(&input, &output, &password)?;
            Ok(format!("Decrypted: {} -> {}", input, output))
        }
    }
}

fn handle_chacha(action: ChachaCommands) -> Result<String, String> {
    match action {
        ChachaCommands::Encrypt { input, output, password } => {
            chacha::encrypt_file(&input, &output, &password)?;
            Ok(format!("Encrypted: {} -> {}", input, output))
        }
        ChachaCommands::Decrypt { input, output, password } => {
            chacha::decrypt_file(&input, &output, &password)?;
            Ok(format!("Decrypted: {} -> {}", input, output))
        }
    }
}

fn handle_ecc(action: EccCommands) -> Result<String, String> {
    match action {
        EccCommands::Keygen => {
            ecc::generate_keypair("private.key", "public.key")?;
            Ok("Generated: private.key, public.key".to_string())
        }
        EccCommands::Encrypt { input, output, public_key } => {
            ecc::encrypt_file(&input, &output, &public_key)?;
            Ok(format!("Encrypted: {} -> {}", input, output))
        }
        EccCommands::Decrypt { input, output, private_key } => {
            ecc::decrypt_file(&input, &output, &private_key)?;
            Ok(format!("Decrypted: {} -> {}", input, output))
        }
    }
}

fn handle_walrus(action: WalrusCommands) -> Result<String, String> {
    let cli = WalrusCli::new();
    
    match action {
        WalrusCommands::Upload { file } => {
            let blob_id = cli.upload(&file)?;
            println!("📦 Blob ID: {}", blob_id);
            Ok(String::new())
        }
        WalrusCommands::Download { blob_id, output } => {
            cli.download(&blob_id, &output)?;
            Ok(format!("Downloaded: {}", output))
        }
        WalrusCommands::UploadAes { file, password } => {
            let blob_id = encrypt_and_upload_aes(&file, &password)?;
            println!("📦 Encrypted Blob ID: {}", blob_id);
            Ok(String::new())
        }
        WalrusCommands::DownloadAes { blob_id, output, password } => {
            download_and_decrypt_aes(&blob_id, &output, &password)?;
            Ok(format!("Decrypted: {}", output))
        }
        WalrusCommands::UploadChacha { file, password } => {
            // Encrypt with ChaCha20
            let encrypted_path = format!("{}.enc", file);
            chacha::encrypt_file(&file, &encrypted_path, &password)?;
            
            // Upload to Walrus
            let blob_id = cli.upload(&encrypted_path)?;
            
            // Clean up
            let _ = std::fs::remove_file(&encrypted_path);
            
            println!("📦 Encrypted Blob ID: {}", blob_id);
            Ok(String::new())
        }
        WalrusCommands::DownloadChacha { blob_id, output, password } => {
            // Download from Walrus
            let encrypted_path = format!("{}.enc", output);
            cli.download(&blob_id, &encrypted_path)?;
            
            // Decrypt with ChaCha20
            chacha::decrypt_file(&encrypted_path, &output, &password)?;
            
            // Clean up
            let _ = std::fs::remove_file(&encrypted_path);
            
            Ok(format!("Decrypted: {}", output))
        }
    }
}