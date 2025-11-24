use std::process::Command;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct WalrusUploadResponse {
    #[serde(rename = "blobId")]
    pub blob_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalrusDownloadResponse {
    pub success: Option<bool>,
    pub output: Option<String>,
    pub size: Option<usize>,
    pub error: Option<String>,
}

/// Call Deno Walrus CLI directly (no HTTP server needed)
pub struct WalrusCli {
    deno_path: String,
    cli_script: String,
}

impl WalrusCli {
    pub fn new() -> Self {
        Self {
            deno_path: "deno".to_string(),
            // When we set current_dir to "mothrbox_ts", this is relative to that
            cli_script: "src/walrus-cli.ts".to_string(),
        }
    }

    pub fn with_paths(deno_path: String, cli_script: String) -> Self {
        Self { deno_path, cli_script }
    }

    /// Read environment variables (prefer real env, fallback to .env file)
    fn read_env_vars() -> Result<(String, String), String> {
        // 1) Prefer real environment variables (good for Docker)
        let env_sui_key = std::env::var("SUI_SECRET_KEY").ok();
        let env_sui_network = std::env::var("SUI_NETWORK").ok();

        if let Some(sui_key) = env_sui_key {
            if sui_key.is_empty() {
                return Err("SUI_SECRET_KEY is set but empty".to_string());
            }
            let sui_network = env_sui_network.unwrap_or_else(|| "testnet".to_string());
            return Ok((sui_key, sui_network));
        }

        // 2) Fallback to .env file (local dev)
        let env_path = "mothrbox_ts/.env";
        let env_content = fs::read_to_string(env_path)
            .map_err(|e| format!("Failed to read .env file at {}: {}", env_path, e))?;
        
        let mut sui_key = String::new();
        let mut sui_network = String::from("testnet");
        
        for line in env_content.lines() {
            let line = line.trim();
            if line.starts_with("SUI_SECRET_KEY=") {
                sui_key = line.strip_prefix("SUI_SECRET_KEY=").unwrap_or("").to_string();
            } else if line.starts_with("SUI_NETWORK=") {
                sui_network = line.strip_prefix("SUI_NETWORK=").unwrap_or("testnet").to_string();
            }
        }
        
        if sui_key.is_empty() {
            return Err("SUI_SECRET_KEY not found in .env file or environment".to_string());
        }

        Ok((sui_key, sui_network))
    }

    /// Convert container-absolute path (/app/...) to repo-relative ("data/..."),
    /// so that "../data/..." from cwd "mothrbox_ts" resolves to "/app/data/...".
    fn normalize_path_for_deno(file_path: &str) -> String {
        // strip leading "/app/" if present
        let stripped = if let Some(rest) = file_path.strip_prefix("/app/") {
            rest
        } else if let Some(rest) = file_path.strip_prefix('/') {
            // generic case: remove leading slash
            rest
        } else {
            file_path
        };

        // If it already starts with "data/", just keep it
        if stripped.starts_with("data/") {
            stripped.to_string()
        } else if stripped.starts_with("app/data/") {
            // handle "app/data/..." -> "data/..."
            stripped.trim_start_matches("app/").to_string()
        } else {
            stripped.to_string()
        }
    }

    /// Upload file to Walrus storage
    pub fn upload(&self, file_path: &str) -> Result<String, String> {
        println!("📤 Uploading {} to Walrus...", file_path);

        let (sui_key, sui_network) = Self::read_env_vars()?;

        // Example: "/app/data/secret.pdf.enc" -> "data/secret.pdf.enc"
        let rel = Self::normalize_path_for_deno(file_path);
        // Deno side expects "../" + rel
        let deno_arg = format!("../{}", rel);

        let output = Command::new(&self.deno_path)
            .current_dir("mothrbox_ts")
            .env("SUI_SECRET_KEY", sui_key)
            .env("SUI_NETWORK", sui_network)
            .arg("run")
            .arg("--allow-net")
            .arg("--allow-read")
            .arg("--allow-env")
            .arg("--allow-write")
            .arg("--allow-sys")
            .arg(&self.cli_script)
            .arg("upload")
            .arg(&deno_arg)
            .output()
            .map_err(|e| format!("Failed to execute Deno CLI: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Walrus CLI error: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        let response: WalrusUploadResponse = serde_json::from_str(&stdout)
            .map_err(|e| format!("Failed to parse response: {}. Output: {}", e, stdout))?;

        if let Some(error) = response.error {
            return Err(format!("Walrus upload failed: {}", error));
        }

        response
            .blob_id
            .ok_or_else(|| "No blob ID in response".to_string())
    }

    /// Download file from Walrus storage
    pub fn download(&self, blob_id: &str, output_path: &str) -> Result<(), String> {
        println!("📥 Downloading {} from Walrus...", blob_id);

        let (sui_key, sui_network) = Self::read_env_vars()?;

        let rel = Self::normalize_path_for_deno(output_path);
        let deno_arg = format!("../{}", rel);

        let output = Command::new(&self.deno_path)
            .current_dir("mothrbox_ts")
            .env("SUI_SECRET_KEY", sui_key)
            .env("SUI_NETWORK", sui_network)
            .arg("run")
            .arg("--allow-net")
            .arg("--allow-read")
            .arg("--allow-env")
            .arg("--allow-write")
            .arg("--allow-sys")
            .arg(&self.cli_script)
            .arg("download")
            .arg(blob_id)
            .arg(&deno_arg)
            .output()
            .map_err(|e| format!("Failed to execute Deno CLI: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Walrus CLI error: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        let response: WalrusDownloadResponse = serde_json::from_str(&stdout)
            .map_err(|e| format!("Failed to parse response: {}. Output: {}", e, stdout))?;

        if let Some(error) = response.error {
            return Err(format!("Walrus download failed: {}", error));
        }

        if response.success.unwrap_or(false) {
            println!("✅ Downloaded {} bytes", response.size.unwrap_or(0));
            Ok(())
        } else {
            Err("Download failed".to_string())
        }
    }
}

/// Upload encrypted file to Walrus
pub fn upload_encrypted_to_walrus(encrypted_path: &str) -> Result<String, String> {
    let cli = WalrusCli::new();
    cli.upload(encrypted_path)
}

/// Download and save from Walrus
pub fn download_from_walrus(blob_id: &str, output_path: &str) -> Result<(), String> {
    let cli = WalrusCli::new();
    cli.download(blob_id, output_path)
}

/// Full workflow: Encrypt file and upload to Walrus
pub fn encrypt_and_upload_aes(
    input_path: &str,
    password: &str,
) -> Result<String, String> {
    use crate::encryption::aes::encrypt_file;
    
    // Encrypt file
    let encrypted_path = format!("{}.enc", input_path);
    encrypt_file(input_path, &encrypted_path, password)?;
    
    // Upload to Walrus
    let blob_id = upload_encrypted_to_walrus(&encrypted_path)?;
    
    // Clean up encrypted file
    let _ = fs::remove_file(&encrypted_path);
    
    Ok(blob_id)
}

/// Full workflow: Download from Walrus and decrypt
pub fn download_and_decrypt_aes(
    blob_id: &str,
    output_path: &str,
    password: &str,
) -> Result<(), String> {
    use crate::encryption::aes::decrypt_file;

    // Normalize output path for inside-container layout.
    // The bash wrapper passes "/data/<file>".
    // Inside the container, the volume is mounted at "/app/data",
    // so we'll actually read/write "/app/data/<file>".
    let mut internal_output = output_path.to_string();

    if internal_output.starts_with("/data/") {
        // Map "/data/foo.pdf" -> "/app/data/foo.pdf"
        internal_output = format!("/app{}", internal_output);
    }

    // Encrypted file lives next to the final output
    let encrypted_path = format!("{}.enc", &internal_output);

    // 1. Download encrypted blob to encrypted_path
    download_from_walrus(blob_id, &encrypted_path)?;

    // 2. Decrypt into internal_output
    decrypt_file(&encrypted_path, &internal_output, password)?;

    // 3. Clean up encrypted temp file
    let _ = std::fs::remove_file(&encrypted_path);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walrus_cli_paths() {
        let cli = WalrusCli::new();
        assert_eq!(cli.deno_path, "deno");
    }

    #[test]
    fn test_normalize_path_for_deno() {
        assert_eq!(
            WalrusCli::normalize_path_for_deno("/app/data/test.enc"),
            "data/test.enc".to_string()
        );
        assert_eq!(
            WalrusCli::normalize_path_for_deno("data/test.enc"),
            "data/test.enc".to_string()
        );
        assert_eq!(
            WalrusCli::normalize_path_for_deno("/data/test.enc"),
            "data/test.enc".to_string()
        );
    }
}
