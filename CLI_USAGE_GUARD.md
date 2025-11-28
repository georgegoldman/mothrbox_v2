# MothrBox CLI - Complete Usage Guide

## 🔐 Three Encryption Methods Available

### 1. AES-256-GCM (Default - Best for Most Users)
- ✅ Hardware-accelerated (Intel/AMD)
- ✅ Military-grade security
- ✅ Best for: Compliance, large files, general use

### 2. ChaCha20-Poly1305 (Mobile-Optimized)
- ✅ Faster on ARM processors
- ✅ Mobile and IoT friendly
- ✅ Best for: Phones, Raspberry Pi, no hardware acceleration

### 3. ECC (Public Key Cryptography)
- ✅ No password sharing needed
- ✅ NIST P-256 curve
- ✅ Best for: Multi-recipient sharing, enterprise

---

## 📋 System Commands

```bash
# Start MothrBox
./mothrbox start

# Check status
./mothrbox status

# View logs
./mothrbox logs

# Stop MothrBox
./mothrbox stop

# Restart
./mothrbox restart

# Rebuild (after code changes)
./mothrbox rebuild

# Clean everything
./mothrbox clean
```

---

## 🔒 AES-256-GCM Commands

### Encrypt and Upload
```bash
./mothrbox encrypt <file> <password>

# Example:
./mothrbox encrypt confidential.pdf "MySecurePassword123"
# Output: Blob ID: abc123xyz...
```

### Download and Decrypt
```bash
./mothrbox decrypt <blob-id> <output-file> <password>

# Example:
./mothrbox decrypt abc123xyz... recovered.pdf "MySecurePassword123"
# Output: Saved to: data/recovered.pdf
```

### Complete Workflow
```bash
# 1. Start system
./mothrbox start

# 2. Encrypt and upload
echo "Top secret data" > secret.txt
./mothrbox encrypt secret.txt "Password123"
# Save the Blob ID!

# 3. Download and decrypt (later)
./mothrbox decrypt <blob-id> recovered.txt "Password123"

# 4. Verify
cat data/recovered.txt
```

---

## ⚡ ChaCha20-Poly1305 Commands

### Encrypt and Upload
```bash
./mothrbox chacha-encrypt <file> <password>

# Example:
./mothrbox chacha-encrypt video.mp4 "MobilePass456"
# Output: Blob ID: def456uvw...
```

### Download and Decrypt
```bash
./mothrbox chacha-decrypt <blob-id> <output-file> <password>

# Example:
./mothrbox chacha-decrypt def456uvw... recovered.mp4 "MobilePass456"
# Output: Saved to: data/recovered.mp4
```

### Use Case: Mobile/IoT
```bash
# Perfect for ARM devices (Raspberry Pi, Android, iOS)
./mothrbox start

# Encrypt for mobile access
./mothrbox chacha-encrypt sensor_data.json "IoTDevice2024"

# Later, decrypt on mobile device
./mothrbox chacha-decrypt <blob-id> data.json "IoTDevice2024"
```

---

## 🔑 ECC (Public Key) Commands

### Generate Key Pair
```bash
./mothrbox keygen

# Creates:
# - data/private.key (keep secret!)
# - data/public.key (share freely)
```

### Encrypt with Public Key (No Password!)
```bash
./mothrbox ecc-encrypt <file> <recipient-public-key>

# Example:
./mothrbox ecc-encrypt contract.pdf data/bob_public.key
# Output: Blob ID: ghi789rst...
```

### Decrypt with Private Key
```bash
./mothrbox ecc-decrypt <blob-id> <output-file> <private-key>

# Example:
./mothrbox ecc-decrypt ghi789rst... contract.pdf data/private.key
# Output: Saved to: data/contract.pdf
```

### Multi-Recipient Sharing Workflow
```bash
# 1. Start system
./mothrbox start

# 2. Generate keys for each party
./mothrbox keygen  # Alice's keys
mv data/private.key data/alice_private.key
mv data/public.key data/alice_public.key

./mothrbox keygen  # Bob's keys
mv data/private.key data/bob_private.key
mv data/public.key data/bob_public.key

./mothrbox keygen  # Carol's keys
mv data/private.key data/carol_private.key
mv data/public.key data/carol_public.key

# 3. Alice encrypts for Bob (using Bob's public key)
./mothrbox ecc-encrypt report.pdf data/bob_public.key
# Blob ID: bob_data123...

# 4. Alice encrypts same file for Carol
./mothrbox ecc-encrypt report.pdf data/carol_public.key
# Blob ID: carol_data456...

# 5. Share Blob IDs publicly - only Bob and Carol can decrypt!

# 6. Bob decrypts with his private key
./mothrbox ecc-decrypt bob_data123... report.pdf data/bob_private.key

# 7. Carol decrypts with her private key
./mothrbox ecc-decrypt carol_data456... report.pdf data/carol_private.key
```

---

## 🎯 Real-World Examples

### Example 1: Law Firm Secure File Sharing (AES)
```bash
./mothrbox start

# Lawyer encrypts contract
./mothrbox encrypt client_contract.pdf "ClientPass2024"
# Blob ID: legal789...

# Share via two channels:
# - Email: Blob ID (legal789...)
# - Phone: Password (ClientPass2024)

# Client downloads and decrypts
./mothrbox decrypt legal789... contract.pdf "ClientPass2024"
```

### Example 2: Mobile Backup (ChaCha20)
```bash
./mothrbox start

# Backup photos from phone storage
./mothrbox chacha-encrypt vacation_photos.zip "BackupPass2024"
# Faster on mobile ARM processors!

# Restore on new device
./mothrbox chacha-decrypt <blob-id> photos.zip "BackupPass2024"
```

### Example 3: Corporate Document Sharing (ECC)
```bash
# Generate company key pairs
./mothrbox keygen  # CEO keys
./mothrbox keygen  # CFO keys
./mothrbox keygen  # Board members keys

# Encrypt quarterly report for CEO
./mothrbox ecc-encrypt q4_report.pdf data/ceo_public.key

# Encrypt same report for CFO
./mothrbox ecc-encrypt q4_report.pdf data/cfo_public.key

# No password sharing needed!
# Each recipient uses their own private key to decrypt
```

### Example 4: Whistleblower Protection (AES)
```bash
./mothrbox start

# Encrypt sensitive documents immediately
./mothrbox encrypt leaked_docs.zip "Whistleblower2024"

# Delete local copy (use shred for security)
shred -vfz -n 10 leaked_docs.zip

# Later: Retrieve when needed
./mothrbox decrypt <blob-id> docs.zip "Whistleblower2024"
```

### Example 5: Automated Backup Script
```bash
#!/bin/bash

# backup.sh - Daily automated backup

./mothrbox start

DATE=$(date +%Y%m%d)
PASSWORD="AutoBackup2024"
BACKUP_FILE="backup_${DATE}.tar.gz"

# Create backup
tar czf "$BACKUP_FILE" ~/important_files/

# Choose encryption method based on your needs:

# Option A: AES (default, fastest with hardware)
RESULT=$(./mothrbox encrypt "$BACKUP_FILE" "$PASSWORD")

# Option B: ChaCha20 (mobile/ARM-friendly)
# RESULT=$(./mothrbox chacha-encrypt "$BACKUP_FILE" "$PASSWORD")

# Option C: ECC (no password needed)
# RESULT=$(./mothrbox ecc-encrypt "$BACKUP_FILE" data/backup_public.key)

# Extract Blob ID
BLOB_ID=$(echo "$RESULT" | grep -oP 'Blob ID: \K\S+')

# Log
echo "${DATE},${BLOB_ID}" >> backup_log.csv

# Cleanup
rm "$BACKUP_FILE"

echo "✅ Backup complete: $BLOB_ID"
```

---

## 🛠️ Advanced: Direct CLI Access

### Access Raw Rust CLI
```bash
./mothrbox cli <command> [args...]

# Examples:
./mothrbox cli aes encrypt /data/file.txt /data/file.enc "password"
./mothrbox cli chacha decrypt /data/file.enc /data/file.txt "password"
./mothrbox cli ecc keygen /data/priv.key /data/pub.key
```

### Manual Workflow (Without Auto-Upload)
```bash
./mothrbox start

# 1. Encrypt only (no upload)
./mothrbox cli aes encrypt /app/data/doc.txt /app/data/doc.enc "pass123"

# 2. Upload manually
docker exec mothrbox_system bash -c "
  deno run -A --env-file=mothrbox_ts/.env \
  mothrbox_ts/src/walrus-cli.ts upload /app/data/doc.enc
"

# 3. Download manually
docker exec mothrbox_system bash -c "
  deno run -A --env-file=mothrbox_ts/.env \
  mothrbox_ts/src/walrus-cli.ts download <blob-id> /app/data/downloaded.enc
"

# 4. Decrypt only
./mothrbox cli aes decrypt /app/data/downloaded.enc /app/data/recovered.txt "pass123"
```

---

## 📊 Comparison Table

| Feature | AES-256-GCM | ChaCha20-Poly1305 | ECC |
|---------|-------------|-------------------|-----|
| **Command** | `encrypt` | `chacha-encrypt` | `ecc-encrypt` |
| **Speed** | Very Fast (HW) | Fast (SW) | Moderate |
| **Security** | Military-grade | Military-grade | Military-grade |
| **Password** | ✅ Required | ✅ Required | ❌ Key-based |
| **Hardware Accel** | ✅ x86/x64 | ❌ No | ❌ No |
| **Mobile-Friendly** | Good | ✅ Excellent | Good |
| **Key Sharing** | Password | Password | ✅ Public key only |
| **Multi-Recipient** | ❌ Need separate encrypt | ❌ Need separate encrypt | ✅ Easy |
| **Use Case** | General purpose | Performance | Enterprise sharing |

---

## 🚨 Security Best Practices

### Password Management
- Use strong passwords (20+ chars, mixed case, numbers, symbols)
- Never reuse passwords across files
- Use a password manager (1Password, Bitwarden)
- For critical data, write password in a physical safe

### Key Management (ECC)
- **NEVER share private keys**
- Store private keys encrypted on disk
- Back up private keys securely (encrypted USB, hardware wallet)
- Share public keys freely - they can't decrypt anything

### General Security
- Always delete encrypted local files after upload
- Use `shred` instead of `rm` for sensitive files:
  ```bash
  shred -vfz -n 10 sensitive_file.txt
  ```
- Verify Blob IDs before sharing
- Store Blob IDs separately from passwords/keys

---

## 🐛 Troubleshooting

### Container Not Running
```bash
# Check status
./mothrbox status

# If not running:
./mothrbox start

# View logs
./mothrbox logs
```

### Upload Failed
```bash
# Check .env configuration
cat mothrbox_ts/.env

# Verify required fields:
# SUI_SECRET_KEY=suiprivkey1...
# SUI_NETWORK=testnet

# Check SUI balance (testnet)
# Get free SUI: https://faucet.testnet.sui.io/
```

### Decrypt Failed
```bash
# Verify you're using the correct:
# 1. Blob ID (exact match)
# 2. Password (case-sensitive)
# 3. Encryption method (aes vs chacha vs ecc)

# Try manual download first
docker exec mothrbox_system bash -c "
  deno run -A --env-file=mothrbox_ts/.env \
  mothrbox_ts/src/walrus-cli.ts download <blob-id> /app/data/test.enc
"

# Check if file exists
ls -lh data/test.enc
```

### Permission Errors
```bash
# Make script executable
chmod +x mothrbox

# Fix data directory permissions
sudo chown -R $USER:$USER data/
chmod -R 755 data/
```

---

## 📝 Quick Reference

```bash
# AES (default)
./mothrbox encrypt file.txt "pass"
./mothrbox decrypt <blob> out.txt "pass"

# ChaCha20 (mobile)
./mothrbox chacha-encrypt file.txt "pass"
./mothrbox chacha-decrypt <blob> out.txt "pass"

# ECC (public key)
./mothrbox keygen
./mothrbox ecc-encrypt file.txt recipient_pub.key
./mothrbox ecc-decrypt <blob> out.txt my_priv.key

# System
./mothrbox start
./mothrbox status
./mothrbox help
```

---

## 💡 Tips

1. **Choose the right algorithm:**
   - Desktop/Server → AES (hardware accelerated)
   - Mobile/IoT → ChaCha20 (software optimized)
   - Sharing → ECC (no password exchange)

2. **Batch operations:**
   ```bash
   for file in *.pdf; do
     ./mothrbox encrypt "$file" "BatchPass2024"
   done
   ```

3. **Test before production:**
   ```bash
   # Encrypt test file
   echo "test" > test.txt
   ./mothrbox encrypt test.txt "test123"
   
   # Verify you can decrypt
   ./mothrbox decrypt <blob> recovered.txt "test123"
   
   # Compare
   diff test.txt data/recovered.txt
   ```

4. **Store Blob IDs safely:**
   - Password manager notes field
   - Encrypted spreadsheet
   - Physical notebook for critical data

---

## 🎓 Learn More

- **Walrus Protocol:** https://walrus.site
- **Sui Blockchain:** https://sui.io
- **AES-256-GCM:** NIST standard, used by Signal, WhatsApp
- **ChaCha20:** Modern cipher by Daniel J. Bernstein
- **ECC P-256:** NSA Suite B approved for TOP SECRET

---

**Built for secure, unified, censorship-resistant storage** 🦋🔒