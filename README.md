# MothrBox 🦋🔒

**Unified Encrypted Decentralized Storage System**

MothrBox combines military-grade encryption with Walrus Protocol's decentralized storage in a unified, easy-to-use system.

---

## Introduction

MothrBox is a unified system with two integrated components that work seamlessly together:

```
┌─────────────────────────────────────────────────────┐
│                    MOTHRBOX                         │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌──────────────────┐ bash   ┌──────────────────┐   │
│  │   Rust Engine    │ ◄──►   │ Deno Walrus      │   │
│  │  (Encryption)    │        │   Client         │   │
│  │  mothrbox_rs     │        │  mothrbox_ts     │   │
│  └──────────────────┘        └──────────────────┘   │
│          │                            │             │
│     File I/O                         RPC            │
│                                                     │
└─────────────────────────────────────────────────────┘
         │                          │
    Your Files (data/)      Walrus Protocol
```

### How It Works

**Your Data Flow:**
```
Your File → Encrypt (Rust) → bash call → Deno → Walrus SDK → Walrus Network
                                                                      ↓
Recovered ← Decrypt (Rust) ← JSON stdout ← Deno ← Walrus SDK ← Blob ID
```

### Component Roles

**Rust Engine (mothrbox_rs)**:
- Encryption/decryption layer
- AES-256-GCM, ChaCha20-Poly1305, ECC
- Handles all cryptographic operations
- Communicates with Deno via bash subprocess
- Never stores unencrypted data

**Deno Walrus Client (mothrbox_ts)**:
- Walrus Protocol integration
- Called by Rust via bash commands
- Handles uploads/downloads to decentralized storage
- Uses Walrus SDK and Sui blockchain
- Returns results via stdout/JSON

### Key Features

- **🔐 End-to-end encryption** - Files encrypted before leaving your machine
- **🌐 Decentralized storage** - Distributed across Walrus Protocol nodes
- **🛡️ Multiple algorithms** - AES-256-GCM, ChaCha20-Poly1305, ECC
- **🔄 Unified system** - Single Docker image, Rust + Deno integration
- **⚡ Bash IPC** - Simple subprocess communication (no HTTP overhead)
- **📦 Docker-based** - No dependency installation required

---

## Installation

### Prerequisites

1. **Docker & Docker Compose** (v20.10+)
   - Linux/macOS: https://docs.docker.com/get-docker/
   - Windows: Use WSL2 + Docker Desktop

2. **Sui Wallet Private Key** (testnet recommended)
   - Get testnet SUI: https://faucet.testnet.sui.io/
   - Key format: `suiprivkey1...`
   - Export from Sui wallet: Settings → Export Private Key

### Installation Steps

```bash
# 1. Clone repository
git clone <repository-url>
cd mothrbox_v2

# 2. Make main script executable
chmod +x mothrbox

# 3. Configure Sui wallet
cd mothrbox_ts
cp .env.example .env

# Edit .env file
nano .env
# Add your values:
# SUI_SECRET_KEY=suiprivkey1...
# SUI_NETWORK=testnet

cd ..

# 4. Build the system
./mothrbox rebuild

# 5. Start MothrBox
./mothrbox start

# Expected output:
# ℹ Starting MothrBox system...
# ℹ Starting Walrus server...
# ℹ Waiting for server to be ready...
# ✅ MothrBox server is running!
# ✅ Server URL: http://localhost:8000

# 6. Verify installation
./mothrbox status
./mothrbox test
```

**Configuration File (`.env`):**
```bash
# Required
SUI_SECRET_KEY=suiprivkey1abc123...    # Your Sui wallet private key
SUI_NETWORK=testnet                     # testnet or mainnet

# Optional (defaults provided)
WALRUS_PUBLISHER=https://publisher.walrus-testnet.walrus.space
WALRUS_AGGREGATOR=https://aggregator.walrus-testnet.walrus.space
```

---

## ⚡ Quickstart

### System Management

```bash
# Start MothrBox Docker container
./mothrbox start

# Check Docker container status
./mothrbox status
# Output:
# ✅ Container: Running
# ✅ Ready to encrypt/decrypt

# Stop MothrBox
./mothrbox stop

# Restart MothrBox
./mothrbox restart

# View logs
./mothrbox logs
```

### Basic Workflow

```bash
# 1. Ensure system is running
./mothrbox start

# 2. Create a test file
echo "My secret data!" > secret.txt

# 3. Encrypt and upload to Walrus
./mothrbox encrypt secret.txt "MyPassword123"

# Output:
# ╔════════════════════════════════════════════════════════╗
# ║                     MOTHRBOX                           ║
# ║     Encrypted Decentralized Storage System             ║
# ╚════════════════════════════════════════════════════════╝
# 
# ℹ Encrypting and uploading...
# 📤 Uploading to Walrus...
# 📦 Encrypted Blob ID: DcNxScTcvltoCYZwLPVC45QNFpddxjL8FmueI7I7-Ho

# 4. Save the Blob ID! Share it separately from password

# 5. Download and decrypt
./mothrbox decrypt DcNxScTcvltoCYZwLPVC45QNFpddxjL8FmueI7I7-Ho recovered.txt "MyPassword123"

# Output:
# ℹ Downloading and decrypting...
# 📥 Downloading from Walrus...
# ✅ Downloaded 15 bytes
# ✅ Decrypted successfully
# ✅ Saved to: data/recovered.txt

# 6. Verify
cat data/recovered.txt
# Output: My secret data!
```

### Available Commands

```bash
# System Commands
./mothrbox start              # Start MothrBox system
./mothrbox stop               # Stop MothrBox system
./mothrbox restart            # Restart MothrBox system
./mothrbox status             # Check system status
./mothrbox logs               # View system logs
./mothrbox test               # Run system tests
./mothrbox rebuild            # Rebuild Docker images

# Encryption Commands
./mothrbox encrypt <file> <password>           # Encrypt and upload (AES-256)
./mothrbox decrypt <blob-id> <output> <password>  # Download and decrypt
./mothrbox keygen             # Generate ECC key pair

# Advanced Commands
./mothrbox cli <command>      # Run any CLI command directly
./mothrbox clean              # Clean up everything
```

---

## Storage Modes

MothrBox supports three encryption modes for different use cases.

### 1. AES-256-GCM (Default) ✅

**Best for:** General purpose, compliance requirements, maximum compatibility

```bash
# Simple encrypt and upload
./mothrbox encrypt confidential.pdf "SecurePassword2024"
# Output: 📦 Encrypted Blob ID: abc123...

# Download and decrypt
./mothrbox decrypt abc123... recovered.pdf "SecurePassword2024"
```

**Features:**
- Industry-standard encryption (used by Signal, WhatsApp, 1Password)
- Hardware-accelerated on Intel/AMD processors (AES-NI)
- Authenticated encryption (prevents tampering)
- PBKDF2 key derivation with 600,000 iterations

**Security Details:**
- **Key size:** 256 bits
- **Salt:** Random 16 bytes per file
- **Nonce (IV):** Random 12 bytes per file
- **Authentication:** GCM mode (Galois Counter Mode)
- **Output format:** `[salt][nonce][ciphertext+tag]`

**When to use:**
- Default choice for most users
- Compliance requirements (HIPAA, GDPR, etc.)
- Large files (hardware acceleration)
- Maximum security

---

### 2. ChaCha20-Poly1305

**Best for:** Mobile devices, ARM processors, systems without AES hardware

```bash
# Start system
./mothrbox start

# Encrypt with ChaCha20 (via CLI)
./mothrbox cli chacha encrypt /data/video.mp4 /data/video.enc "MobilePass123"

# Upload to Walrus
./mothrbox cli walrus upload /data/video.enc
# Output: {"blobId":"xyz789..."}

# Download from Walrus
./mothrbox cli walrus download xyz789... /data/video_downloaded.enc

# Decrypt
./mothrbox cli chacha decrypt /data/video_downloaded.enc /data/video_recovered.mp4 "MobilePass123"
```

**Features:**
- Faster than AES on devices without hardware acceleration
- Constant-time implementation (resistant to timing attacks)
- Authenticated encryption (Poly1305 MAC)
- Mobile-friendly and IoT-optimized

**Security Details:**
- **Key size:** 256 bits
- **Salt:** Random 16 bytes per file
- **Nonce:** Random 12 bytes per file
- **Authentication:** Poly1305 MAC
- **Output format:** `[salt][nonce][ciphertext+tag]`

**When to use:**
- Mobile devices (ARM processors)
- Raspberry Pi, IoT devices
- Performance-critical applications
- No AES hardware acceleration available

---

### 3. ECC (Elliptic Curve Cryptography)

**Best for:** Sharing encrypted data without sharing passwords, multi-recipient scenarios

```bash
# Generate key pair
./mothrbox keygen
# Creates: data/private.key, data/public.key

# Encrypt with recipient's public key (no password!)
./mothrbox cli ecc encrypt /data/secret_doc.txt /data/secret_doc.enc /data/recipient_public.key

# Upload to Walrus
./mothrbox cli walrus upload /data/secret_doc.enc
# Output: {"blobId":"ecc456..."}

# Share blob ID publicly - only recipient with private key can decrypt

# Download from Walrus
./mothrbox cli walrus download ecc456... /data/downloaded.enc

# Recipient decrypts with their private key
./mothrbox cli ecc decrypt /data/downloaded.enc /data/decrypted.txt /data/private.key
```

**Features:**
- Public key cryptography (no shared password needed)
- NIST P-256 elliptic curve (secp256r1)
- Ephemeral ECDH (perfect forward secrecy)
- Hybrid encryption (ECC + AES-256-GCM)

**Security Details:**
- **Curve:** NIST P-256 (approved by NSA for TOP SECRET)
- **Key exchange:** Ephemeral Elliptic Curve Diffie-Hellman
- **Key derivation:** HKDF (HMAC-based KDF)
- **Content encryption:** AES-256-GCM
- **Output format:** `[ephemeral_public_key][nonce][ciphertext+tag]`

**When to use:**
- Secure data sharing without password exchange
- Multi-recipient scenarios (encrypt once per recipient)
- Long-term storage with key rotation
- Enterprise key management systems

---

### Comparison Table

| Feature | AES-256-GCM | ChaCha20-Poly1305 | ECC |
|---------|-------------|-------------------|-----|
| **Speed** | Very Fast (HW) | Fast (SW) | Moderate |
| **Security** | Military-grade | Military-grade | Military-grade |
| **Password-based** | ✅ Yes | ✅ Yes | ❌ Key-based |
| **Hardware accel** | ✅ x86/x64 | ❌ No | ❌ No |
| **Mobile-friendly** | Good | ✅ Excellent | Good |
| **Key sharing** | Requires password | Requires password | ✅ Public key only |
| **Use case** | General purpose | Performance | Sharing |
| **Overhead** | +28 bytes | +44 bytes | +65 bytes |

---

## 💡 Use Cases

### 1. Secure Document Sharing

**Scenario:** Law firm needs to share confidential contracts with clients

```bash
# Ensure system is running
./mothrbox start

# Lawyer encrypts contract
./mothrbox encrypt client_contract_2024.pdf "ClientPass!2024"
# Blob ID: legal789...

# Share with client:
# - Email blob ID: legal789...
# - Phone call password: ClientPass!2024

# Client downloads and decrypts
./mothrbox decrypt legal789... contract.pdf "ClientPass!2024"
```

**Why MothrBox:**
- ✅ No central server to hack
- ✅ End-to-end encrypted
- ✅ Blob ID can be shared via insecure channels
- ✅ Password transmitted separately
- ✅ Decentralized storage (no single point of failure)

---

### 2. Personal Cloud Backup

**Scenario:** Backup important documents with encryption before cloud storage

```bash
# Start system
./mothrbox start

# Create automated backup script
cat > backup.sh << 'EOF'
#!/bin/bash
DATE=$(date +%Y%m%d)
PASSWORD="MyBackupPass2024"

# Backup each important file
for file in ~/Documents/*.pdf ~/Photos/*.jpg; do
    if [ -f "$file" ]; then
        echo "Backing up: $file"
        ./mothrbox encrypt "$file" "$PASSWORD"
    fi
done

# Log completion
echo "Backup completed: $DATE" >> backup.log
EOF

chmod +x backup.sh
./backup.sh
```

**Why MothrBox:**
- ✅ Encrypted before upload
- ✅ Decentralized (can't lose all data if one node fails)
- ✅ Password-protected
- ✅ Automated workflow
- ✅ No monthly subscription fees

---

### 3. Journalist Source Protection

**Scenario:** Investigative journalist protecting whistleblower documents

```bash
./mothrbox start

# Encrypt sensitive documents immediately
./mothrbox encrypt leaked_documents.zip "Whistleblower!Pass2024"
# Blob ID: protect123...

# Delete local copy immediately
shred -vfz -n 10 leaked_documents.zip

# Store blob ID in encrypted notes app

# Later: Retrieve when needed for story
./mothrbox decrypt protect123... working_copy.zip "Whistleblower!Pass2024"
```

**Why MothrBox:**
- ✅ Censorship-resistant (decentralized storage)
- ✅ No central server to subpoena
- ✅ Encrypted at rest
- ✅ Distributed across multiple nodes
- ✅ Blockchain audit trail (Sui)

---

### 4. Medical Records (HIPAA Compliance)

**Scenario:** Healthcare provider storing patient records securely

```bash
./mothrbox start

# Encrypt patient data
./mothrbox encrypt patient_records_Q4_2024.csv "MedicalDB!SecurePass"
# Blob ID: hipaa456...

# Access control via password distribution
# Audit trail via Sui blockchain
# Encrypted at rest (HIPAA requirement met)

# Access later for authorized personnel only
./mothrbox decrypt hipaa456... patient_records.csv "MedicalDB!SecurePass"
```

**Why MothrBox:**
- ✅ HIPAA-compliant encryption (AES-256)
- ✅ Encrypted before transmission
- ✅ Decentralized storage (redundancy)
- ✅ Blockchain audit trail
- ✅ Access control via passwords

---

### 5. Developer SSH Key Backup

**Scenario:** Securely backup SSH keys to recover on new machines

```bash
./mothrbox start

# Create encrypted backup
cd ~/.ssh
tar czf /tmp/ssh_backup.tar.gz id_rsa id_rsa.pub config known_hosts

# Encrypt and upload
cd ~/mothrbox_v2
./mothrbox encrypt /tmp/ssh_backup.tar.gz "SSHBackup!2024"
# Blob ID: ssh789...
# Store blob ID in password manager!

# Clean up
shred -vfz -n 10 /tmp/ssh_backup.tar.gz

# --- On new machine ---
# Restore SSH keys
./mothrbox decrypt ssh789... ssh_restore.tar.gz "SSHBackup!2024"
tar xzf data/ssh_restore.tar.gz -C ~/.ssh/
chmod 600 ~/.ssh/id_rsa
```

**Why MothrBox:**
- ✅ Never store private keys unencrypted
- ✅ Decentralized backup
- ✅ Restore from any machine
- ✅ Password-protected

---

### 6. Academic Research Data Archival

**Scenario:** Publish research paper with verifiable dataset

```bash
./mothrbox start

# Encrypt research dataset
./mothrbox encrypt climate_study_2024.tar.gz "ResearchData2024"
# Blob ID: research123...

# Include blob ID in published paper
# Other researchers can verify data integrity
# Data remains encrypted but accessible

# Peer reviewer decrypts (password shared via email)
./mothrbox decrypt research123... dataset.tar.gz "ResearchData2024"
```

**Why MothrBox:**
- ✅ Verifiable storage (Sui blockchain)
- ✅ Decentralized availability
- ✅ Encrypted for privacy
- ✅ Content-addressed (blob ID)
- ✅ Long-term archival

---

### 7. Multi-Party Secure Exchange (ECC)

**Scenario:** Share encrypted data with multiple parties without password exchange

```bash
./mothrbox start

# Generate key pairs for each party
./mothrbox keygen  # Alice's keys → data/alice_private.key, data/alice_public.key
./mothrbox keygen  # Bob's keys → data/bob_private.key, data/bob_public.key
./mothrbox keygen  # Carol's keys → data/carol_private.key, data/carol_public.key

# Alice encrypts data for Bob (using Bob's public key)
./mothrbox cli ecc encrypt /data/confidential_report.pdf /data/for_bob.enc /data/bob_public.key
./mothrbox cli walrus upload /data/for_bob.enc
# Blob ID: bob_data123...

# Alice encrypts same data for Carol (using Carol's public key)
./mothrbox cli ecc encrypt /data/confidential_report.pdf /data/for_carol.enc /data/carol_public.key
./mothrbox cli walrus upload /data/for_carol.enc
# Blob ID: carol_data456...

# Share blob IDs publicly - only Bob and Carol can decrypt with their private keys

# Bob downloads and decrypts with his private key
./mothrbox cli walrus download bob_data123... /data/bob_encrypted.enc
./mothrbox cli ecc decrypt /data/bob_encrypted.enc /data/report.pdf /data/bob_private.key

# Carol downloads and decrypts with her private key
./mothrbox cli walrus download carol_data456... /data/carol_encrypted.enc
./mothrbox cli ecc decrypt /data/carol_encrypted.enc /data/report.pdf /data/carol_private.key
```

**Why MothrBox:**
- ✅ No shared password needed
- ✅ Each party has unique key
- ✅ Can revoke access (don't share new encryptions)
- ✅ Public key cryptography
- ✅ Enterprise-grade key management

---

### 8. Disaster Recovery Planning

**Scenario:** Company backing up critical infrastructure configs

```bash
./mothrbox start

# Encrypt critical configs
./mothrbox encrypt /etc/nginx/nginx.conf "InfraPass2024"
./mothrbox encrypt /etc/kubernetes/admin.conf "InfraPass2024"
./mothrbox encrypt docker-compose.yml "InfraPass2024"

# Store blob IDs in secure documentation
# Configs are now safely stored in decentralized storage

# During disaster recovery:
./mothrbox decrypt config1... nginx.conf "InfraPass2024"
./mothrbox decrypt config2... admin.conf "InfraPass2024"
./mothrbox decrypt config3... docker-compose.yml "InfraPass2024"

# Restore infrastructure from recovered configs
```

**Why MothrBox:**
- ✅ Off-site backup (decentralized)
- ✅ Encrypted configurations
- ✅ Quick recovery
- ✅ No infrastructure dependencies
- ✅ Works even if primary datacenter is down

---

## System Architecture

### Mothrbox System Architecture

```
┌─────────────────────────────────────────────────────────┐
│  User Interface (./mothrbox command)                    │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┴────────────┐
        │                         │
        ▼                         ▼
┌──────────────┐ bash     ┌──────────────┐
│ Rust Engine  │subprocess│  Deno Client │
│ (Encryption) │ ◄──────► │ (Walrus SDK) │
│              │  JSON    │              │
│AES/ECC/ChaCha│  stdout  │  - Walrus    │
└──────┬───────┘          └──────┬───────┘
       │                         │
       │                         │
       ▼                         ▼
  Local Files            Walrus Protocol
  (./data/)              (Decentralized)
                               │
                               ▼
                        Sui Blockchain
                         (Metadata)
```


### Communication Mechanism

**Bash Subprocess:**
```rust
// Rust spawns Deno subprocess
let output = Command::new("deno")
    .args(&["run", "-A", "--env-file=.env", 
            "mothrbox_ts/src/walrus-cli.ts", 
            "upload", file_path])
    .output()?;

// Parse JSON response from stdout
let response: WalrusResponse = serde_json::from_str(&stdout)?;
```

**This ensures:**
- ✅ Rust controls the encryption flow
- ✅ Deno handles Walrus Protocol communication
- ✅ Simple IPC via stdin/stdout
- ✅ JSON for structured data exchange
- ✅ Single Docker image (both components together)

---

## Advanced Usage

### Batch Operations

```bash
./mothrbox start

# Encrypt multiple files
for file in Documents/*.pdf; do
    echo "Encrypting: $file"
    ./mothrbox encrypt "$file" "BatchPass2024"
done
```

### Custom Automation Script

```bash
#!/bin/bash
# Daily backup automation

./mothrbox start

DATE=$(date +%Y%m%d)
BACKUP_FILE="backup_${DATE}.tar.gz"
PASSWORD="AutoBackup2024"

# Create backup
tar czf "$BACKUP_FILE" ~/important_files/

# Encrypt and upload
RESULT=$(./mothrbox encrypt "$BACKUP_FILE" "$PASSWORD")
BLOB_ID=$(echo "$RESULT" | grep -oP 'Blob ID: \K\S+')

# Log
echo "${DATE},${BLOB_ID}" >> backup_log.csv

# Cleanup
rm "$BACKUP_FILE"

echo "Backup complete: $BLOB_ID"
```

### Integration with CI/CD

```bash
# In your CI pipeline
- name: Backup artifacts
  run: |
    ./mothrbox start
    ./mothrbox encrypt build_artifacts.tar.gz "${{ secrets.BACKUP_PASSWORD }}"
```

---

## Troubleshooting

### System Won't Start

```bash
# Check Docker
docker --version
docker compose version

# View logs
./mothrbox logs

# Rebuild system
./mothrbox rebuild
./mothrbox start
```

### Commands Fail

```bash
# Verify container is running
./mothrbox status

# Check Docker logs
docker compose logs

# Restart system
./mothrbox restart
```

### Configuration Issues

```bash
# Check .env file
cat mothrbox_ts/.env

# Verify required fields:
# - SUI_SECRET_KEY=suiprivkey1...
# - SUI_NETWORK=testnet

# Recreate if needed
cd mothrbox_ts
cp .env.example .env
nano .env
```

### Out of SUI Balance

```bash
# Testnet: Get free SUI
# Visit: https://faucet.testnet.sui.io/
# Enter your wallet address

# Check balance
sui client balance
```

---

## FAQ

**Q: How does Rust communicate with Deno?**
A: Via bash subprocess. Rust spawns Deno, passes commands via args, and receives JSON responses via stdout.

**Q: Do I need to keep the system running all the time?**
A: No! Start when you need it (`./mothrbox start`), stop when done (`./mothrbox stop`). Everything runs in a single Docker container.

**Q: Can I run commands without starting Docker?**
A: No, the system runs inside Docker. Always run `./mothrbox start` first.

**Q: How secure is MothrBox?**
A: Uses AES-256-GCM (military-grade), PBKDF2 with 600K iterations, same standards as Signal and WhatsApp.

**Q: Where is my data stored?**
A: Encrypted data is distributed across Walrus Protocol nodes (decentralized network). Metadata on Sui blockchain.

**Q: Can I lose my data?**
A: Walrus uses erasure coding - data survives node failures. Default storage: 3 epochs (~30 days).

**Q: What if I forget my password?**
A: **Cannot be recovered.** Use a password manager. Consider writing critical passwords in a safe.

**Q: How much does it cost?**
A: Testnet is free. Mainnet requires SUI tokens for storage.


---

## Technical Specifications

**Architecture:**
- Single Docker image
- Rust engine + Deno client integrated
- Bash subprocess IPC (stdin/stdout/JSON)
- Volume-mounted data directory

**Encryption Algorithms:**
- AES-256-GCM (Galois Counter Mode)
- ChaCha20-Poly1305
- ECC P-256 (secp256r1)

**Key Derivation:**
- PBKDF2-HMAC-SHA256
- 600,000 iterations
- 16-byte random salt per file

**Storage:**
- Walrus Protocol (erasure coding)
- Default: 3 epochs (~30 days)
- Deletable blobs

**Blockchain:**
- Sui Network
- Metadata storage
- Proof of storage

**Languages & Runtime:**
- Rust 1.83 (encryption engine)
- TypeScript/Deno (Walrus client)
- Docker & Docker Compose

---

## Contributing

This is a hackathon project. For improvements or issues, contact the team.

---

## License

[Specify your license]

---

## Acknowledgments

- **Walrus Protocol** - Decentralized storage infrastructure
- **Sui Foundation** - Blockchain and smart contract platform
- **Rust Community** - Cryptography libraries
- **Deno Team** - TypeScript runtime

---

**Built for secure, unified, censorship-resistant storage** 🦋🔒