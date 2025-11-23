# 🚀 MothrBox Setup - Prerequisites & Installation

## 📋 Prerequisites

### 1. Docker Installation

**Ubuntu/Debian:**
```bash
# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Start Docker
sudo systemctl start docker
sudo systemctl enable docker

# Add user to docker group (to avoid sudo)
sudo usermod -aG docker $USER
newgrp docker

# Test Docker
docker run hello-world
```

**macOS:**
```bash
# Install Docker Desktop
# Download from: https://docs.docker.com/desktop/install/mac-install/
# Or with Homebrew:
brew install --cask docker
```

**Windows:**
- Download Docker Desktop: https://docs.docker.com/desktop/install/windows-install/
- Install WSL2 if needed

### 2. Sui Wallet Setup

You need a Sui wallet with a private key for Walrus storage.

**Option A: Use Sui CLI (Recommended)**
```bash
# Install Sui CLI
cargo install --locked --git https://github.com/MystenLabs/sui.git --branch testnet sui

# Generate new address
sui client new-address ed25519

# Output will show:
# Created new keypair and saved it to keystore.
# Private key (base64): ...
# Public key: ...
# Address: 0x...

# Export your private key
sui keytool export --key-identity <address>
# This gives you the private key in format: suiprivkey1...
```

**Option B: Use Existing Sui Wallet**
```bash
# If you have Sui Wallet extension
# 1. Open Sui Wallet
# 2. Go to Settings → Export Private Key
# 3. Copy the private key (starts with suiprivkey1...)
```

**Option C: Get Test Wallet**
```bash
# Use the example key for testing (NOT for production!)
# This is provided in .env.example
SUI_SECRET_KEY=suiprivkey1qpvv2g070y4ewm3fam6yp7y8gq3w3y4c4a
```

### 3. Get Test SUI Tokens (Optional)

If you're using testnet and need tokens:

```bash
# Request testnet tokens
curl --location --request POST \
  'https://faucet.testnet.sui.io/gas' \
  --header 'Content-Type: application/json' \
  --data-raw '{
    "FixedAmountRequest": {
      "recipient": "<YOUR_SUI_ADDRESS>"
    }
  }'
```

Or use the Sui Testnet Faucet:
- https://docs.sui.io/guides/developer/getting-started/get-coins

## 📁 Project Setup

### Step 1: Prepare Your Project

```bash
cd ~/mothrbox_v2

# Your structure should be:
# mothrbox_v2/
# ├── mothrbox_rs/
# ├── mothrbox_ts/
# └── (files to be added)
```

### Step 2: Add Docker Files

```bash
# Copy these files to your project:

# 1. Dockerfiles
cp /path/to/Dockerfile.rust mothrbox_rs/Dockerfile
cp /path/to/Dockerfile.deno mothrbox_ts/Dockerfile

# 2. Docker Compose
cp /path/to/docker-compose-full.yml docker-compose.yml

# 3. Helper Script
cp /path/to/mothrbox.sh .
chmod +x mothrbox.sh

# 4. .env.example
cp /path/to/.env.example mothrbox_ts/.env.example
```

### Step 3: Configure Environment Variables

```bash
cd ~/mothrbox_v2

# Create .env file from example
cp mothrbox_ts/.env.example mothrbox_ts/.env

# Edit with your Sui private key
nano mothrbox_ts/.env
```

**Edit the .env file:**
```bash
# Replace with YOUR Sui private key
SUI_SECRET_KEY=suiprivkey1qpvv2g070y4ewm3fam6yp7y8gq3w3y4c4a
SUI_NETWORK=testnet
PORT=8000
```

**⚠️ IMPORTANT:**
- Never commit `.env` to git
- Keep your private key secret
- Use different keys for testnet and mainnet

### Step 4: Add .env to .gitignore

```bash
cd ~/mothrbox_v2

# Create/update .gitignore
cat >> .gitignore << 'EOF'
# Environment variables
mothrbox_ts/.env
.env

# Docker volumes
data/

# Keys
*.key
private.key
public.key

# Temporary files
*.enc
*.ecc
.temp_*
EOF
```

### Step 5: Create Data Directory

```bash
mkdir -p data
```

## 🚀 Building and Running

### Automated Setup (Recommended)

```bash
cd ~/mothrbox_v2

# Run the helper script
./mothrbox.sh

# Choose option 1: Build and start services
# The script will check for .env and guide you if missing
```

### Manual Setup

```bash
cd ~/mothrbox_v2

# Build images
docker-compose build

# Start services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f
```

## ✅ Verify Installation

### Test 1: Check Services Running

```bash
# Check containers
docker-compose ps

# Should show:
# walrus-server    Up    0.0.0.0:8000->8000/tcp
# cryptool         Up
```

### Test 2: Test Walrus Server

```bash
# Test endpoint
curl http://localhost:8000/

# Should return: "Hello Hono!" or similar
```

### Test 3: Run Full System Test

```bash
# Use helper script
./mothrbox.sh

# Choose option 4: Test the system

# This will:
# 1. Create test file
# 2. Encrypt and upload to Walrus
# 3. Download and decrypt
# 4. Verify integrity
```

### Test 4: Manual Encryption Test

```bash
# Create test file
echo "Test message" > data/test.txt

# Encrypt with AES
docker-compose run --rm cryptool aes encrypt /data/test.txt /data/test.enc "password123"

# Decrypt
docker-compose run --rm cryptool aes decrypt /data/test.enc /data/test2.txt "password123"

# Verify
diff data/test.txt data/test2.txt && echo "✅ Works!"
```

## 🔧 Troubleshooting

### Issue 1: ".env file not found"

```bash
# Create from example
cp mothrbox_ts/.env.example mothrbox_ts/.env

# Edit with your key
nano mothrbox_ts/.env
```

### Issue 2: "Invalid SUI_SECRET_KEY"

```bash
# Check format - should start with: suiprivkey1
# Get new key:
sui client new-address ed25519
```

### Issue 3: "Docker daemon not running"

```bash
# Start Docker
sudo systemctl start docker

# Check status
sudo systemctl status docker
```

### Issue 4: "Permission denied" on Docker

```bash
# Add user to docker group
sudo usermod -aG docker $USER
newgrp docker

# Or use sudo
sudo docker-compose up -d
```

### Issue 5: "Port 8000 already in use"

```bash
# Find process using port 8000
sudo lsof -i :8000

# Kill it
sudo kill <PID>

# Or change port in docker-compose.yml
# ports:
#   - "8001:8000"
```

## 📂 Final Project Structure

```
mothrbox_v2/
├── docker-compose.yml          ✅ Main orchestration
├── mothrbox.sh                 ✅ Helper script
├── .gitignore                  ✅ Ignore sensitive files
├── data/                       ✅ Your files (mounted volume)
├── mothrbox_rs/
│   ├── Dockerfile              ✅ Rust CLI container
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── encryption/
└── mothrbox_ts/
    ├── Dockerfile              ✅ Deno server container
    ├── .env.example            ✅ Template
    ├── .env                    ✅ Your config (DO NOT COMMIT!)
    ├── deno.json
    └── src/
        ├── main.ts
        └── walrus-client.ts
```

## 🎯 Quick Start Commands

```bash
# Start everything
docker-compose up -d

# Stop everything
docker-compose down

# View logs
docker-compose logs -f walrus-server

# Run encryption command
docker-compose run --rm cryptool help

# Interactive helper
./mothrbox.sh
```

## 🔐 Security Checklist

- [ ] Docker installed and running
- [ ] Sui wallet created
- [ ] Private key added to `.env`
- [ ] `.env` added to `.gitignore`
- [ ] Services start successfully
- [ ] Encryption test passes
- [ ] Walrus upload/download works

## 🎉 You're Ready!

Once all prerequisites are met:

```bash
cd ~/mothrbox_v2
./mothrbox.sh
# Choose option 1, then option 4 to test
```

Your encrypted Walrus storage system is ready to use! 🚀🔒