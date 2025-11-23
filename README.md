# 🦋 MothrBox - Unified System Guide

## 🎯 Overview

MothrBox is a **unified encrypted decentralized storage system** with two components that work as one:

```
┌─────────────────────────────────────────────────────┐
│                    MOTHRBOX                         │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌──────────────────┐      ┌──────────────────┐   │
│  │  mothrbox-server │ ◄──► │  mothrbox-cli    │   │
│  │  (Walrus Backend)│      │  (Encryption)    │   │
│  └──────────────────┘      └──────────────────┘   │
│          │                          │              │
│     Port 8000                  File Access         │
│                                                     │
└─────────────────────────────────────────────────────┘
         │                          │
    Walrus Storage            Your Files (data/)
```

### Component Roles:

**mothrbox-server** (Required):
- Walrus storage backend
- Must be running for system to work
- Handles uploads/downloads to Walrus

**mothrbox-cli** (On-demand):
- Encryption/decryption layer
- Only runs when you execute commands
- Requires server to be healthy

### The Unified Approach:

✅ CLI **depends on** server (won't start if server is down)
✅ Health checks ensure server is ready
✅ Single command interface: `./mothrbox`
✅ One network, one system name

## 🚀 Quick Start

### Step 1: Setup Files

```bash
cd ~/mothrbox_v2

# 1. Replace docker-compose.yml
cp /path/to/docker-compose-mothrbox.yml docker-compose.yml

# 2. Replace main script
cp /path/to/mothrbox ./mothrbox
chmod +x mothrbox

# 3. Setup .env
cd mothrbox_ts
cp .env.example .env
nano .env  # Add your SUI_SECRET_KEY
```

### Step 2: Start MothrBox

```bash
cd ~/mothrbox_v2

# Start the system
./mothrbox start

# Output:
# ℹ Starting MothrBox system...
# ℹ Starting Walrus server...
# ℹ Waiting for server to be ready...
# ✅ MothrBox server is running!
# ✅ Server URL: http://localhost:8000
```

### Step 3: Use MothrBox

```bash
# Check status
./mothrbox status

# Run tests
./mothrbox test

# Encrypt and upload
./mothrbox encrypt secret.pdf MyPassword123

# Decrypt and download
./mothrbox decrypt <blobId> secret.pdf MyPassword123
```

## 📖 Command Reference

### System Management

```bash
# Start MothrBox (server + CLI ready)
./mothrbox start

# Stop MothrBox
./mothrbox stop

# Restart MothrBox
./mothrbox restart

# Check system status
./mothrbox status

# View logs
./mothrbox logs

# Test system
./mothrbox test
```

### Encryption Commands

```bash
# Encrypt and upload to Walrus (uses AES-256-GCM)
./mothrbox encrypt <file> <password>
# Example:
./mothrbox encrypt document.pdf MyPass123

# Decrypt and download from Walrus
./mothrbox decrypt <blobId> <o> <password>
# Example:
./mothrbox decrypt abc123... document.pdf MyPass123

# Generate ECC key pair
./mothrbox keygen
```

### Advanced Commands

```bash
# Run any CLI command
./mothrbox cli <command>
# Examples:
./mothrbox cli help
./mothrbox cli aes encrypt /data/file.txt /data/file.enc "pass"
./mothrbox cli chacha encrypt /data/video.mp4 /data/video.enc "pass"

# Rebuild system
./mothrbox rebuild

# Clean up everything
./mothrbox clean
```

## 🎯 Usage Examples

### Example 1: Quick File Encryption

```bash
# Start system
./mothrbox start

# Encrypt a file
./mothrbox encrypt ~/Documents/secret.txt SecurePass123

# Output will show blobId, save it!
# {"blobId":"abc123xyz..."}

# Later, decrypt it
./mothrbox decrypt abc123xyz secret.txt SecurePass123

# File is saved to: data/secret.txt
```

### Example 2: Using Different Algorithms

```bash
# Start system
./mothrbox start

# Use AES (default via encrypt command)
./mothrbox encrypt file.pdf Pass123

# Use ChaCha20 (via CLI)
./mothrbox cli walrus upload-chacha /data/file.pdf Pass123 http://mothrbox-server:8000

# Use ECC (public key)
./mothrbox keygen
./mothrbox cli walrus upload-ecc /data/file.pdf /data/public.key http://mothrbox-server:8000
```

### Example 3: System Health Check

```bash
# Check if system is working
./mothrbox status

# Output:
# MothrBox System Status:
# ✅ Server: Running (http://localhost:8000)
# 
# NAME              STATUS    PORTS
# mothrbox-server   Up        0.0.0.0:8000->8000/tcp
```

### Example 4: Full Workflow

```bash
# 1. Start system
./mothrbox start

# 2. Run tests to verify
./mothrbox test

# 3. Encrypt your file
echo "My secrets" > myfile.txt
./mothrbox encrypt myfile.txt MyPassword

# 4. Save the blobId (from output)
# 5. Delete local file
rm myfile.txt

# 6. Later, decrypt from Walrus
./mothrbox decrypt <blobId> myfile.txt MyPassword

# 7. Check the file
cat data/myfile.txt
```

## 🔧 How Dependency Works

### Server Health Check:

```yaml
mothrbox-server:
  healthcheck:
    test: ["CMD", "curl", "-f", "http://localhost:8000/"]
    interval: 10s
    timeout: 5s
    retries: 5
    start_period: 10s
```

### CLI Dependency:

```yaml
mothrbox-cli:
  depends_on:
    mothrbox-server:
      condition: service_healthy  # ← Waits for server to be healthy
```

This ensures:
✅ Server starts first
✅ Server is fully ready (health check passes)
✅ CLI only starts after server is healthy
✅ CLI commands will work (server is guaranteed to be up)

## 🐛 Troubleshooting

### Issue: "MothrBox is not running"

```bash
# Start it
./mothrbox start

# Check status
./mothrbox status

# View logs if failed
./mothrbox logs
```

### Issue: "Server failed to start"

```bash
# Check .env file exists
ls -la mothrbox_ts/.env

# View detailed logs
docker-compose logs mothrbox-server

# Rebuild
./mothrbox rebuild
./mothrbox start
```

### Issue: CLI command fails

```bash
# Ensure server is running
./mothrbox status

# Restart system
./mothrbox restart

# Run test
./mothrbox test
```

### Issue: Health check failing

```bash
# Check server logs
./mothrbox logs

# Test server manually
curl http://localhost:8000/

# Rebuild if needed
./mothrbox rebuild
```

## 📊 System Architecture

```
┌─────────────────────────────────────────────────────────┐
│  User Commands via ./mothrbox                           │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┴────────────┐
        │                         │
        ▼                         ▼
┌──────────────┐          ┌──────────────┐
│ mothrbox-cli │◄────────►│mothrbox-server│
│ (Encryption) │  depends │ (Walrus API)  │
└──────────────┘   on     └───────┬───────┘
        │                         │
        │                         │
        ▼                         ▼
  Local Files               Walrus Network
  (./data/)                 (Decentralized)
```

### Communication Flow:

1. **Start**: User runs `./mothrbox start`
2. **Health Check**: System waits for server to be healthy
3. **Ready**: CLI commands can now use server
4. **Encrypt**: CLI encrypts locally → uploads via server
5. **Decrypt**: CLI downloads via server → decrypts locally

## 🎉 Benefits of Unified System

✅ **Single Entry Point**: One command for everything
✅ **Dependency Management**: CLI won't run if server is down
✅ **Health Checks**: Ensures system is ready before use
✅ **Simple Commands**: `./mothrbox encrypt` instead of long docker commands
✅ **Error Prevention**: Can't run CLI commands when server is unavailable
✅ **Unified Naming**: Everything is "mothrbox"

## 📝 Configuration Files

```
mothrbox_v2/
├── docker-compose.yml      # Defines mothrbox-server + mothrbox-cli
├── mothrbox                # Unified command script
├── data/                   # Your files
├── mothrbox_ts/
│   ├── .env               # Your Sui key
│   └── Dockerfile
└── mothrbox_rs/
    └── Dockerfile
```

## 🚀 You're Ready!

MothrBox now operates as a **single unified system**:

```bash
# One command to rule them all
./mothrbox start
./mothrbox encrypt myfile.pdf password
./mothrbox status
```

Your encryption tool and Walrus backend work together seamlessly! 🦋🔒